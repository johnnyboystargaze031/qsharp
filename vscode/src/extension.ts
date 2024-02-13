// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

import {
  ILanguageService,
  getLanguageService,
  getLibrarySourceContent,
  loadWasmModule,
  log,
  qsharpLibraryUriScheme,
} from "qsharp-lang";
import * as vscode from "vscode";
import {
  isQsharpDocument,
  isQsharpNotebookCell,
  qsharpLanguageId,
} from "./common.js";
import { createCompletionItemProvider } from "./completion";
import { activateDebugger } from "./debugger/activate";
import { getTarget } from "./config";
import { createDefinitionProvider } from "./definition";
import { startCheckingQSharp } from "./diagnostics";
import { createHoverProvider } from "./hover";
import {
  registerCreateNotebookCommand,
  registerQSharpNotebookCellUpdateHandlers,
  registerQSharpNotebookHandlers,
} from "./notebook.js";
import {
  EventType,
  QsharpDocumentType,
  initTelemetry,
  sendTelemetryEvent,
} from "./telemetry.js";
import { initAzureWorkspaces } from "./azure/commands.js";
import { initCodegen } from "./qirGeneration.js";
import { createSignatureHelpProvider } from "./signature.js";
import { createRenameProvider } from "./rename.js";
import { registerWebViewCommands } from "./webviewPanel.js";
import { createReferenceProvider } from "./references.js";
import { activateTargetProfileStatusBarItem } from "./statusbar.js";
import { initFileSystem } from "./memfs.js";
import { getManifest, readFile, listDir } from "./projectSystem.js";
import {
  Logging,
  initLogForwarder,
  initOutputWindowLogger,
} from "./logging.js";
import { createCodeLensProvider } from "./codeLens.js";

export async function activate(
  context: vscode.ExtensionContext,
): Promise<ExtensionApi> {
  const api: ExtensionApi = {};

  if (context.extensionMode === vscode.ExtensionMode.Test) {
    // Don't log to the output window in tests, forward to a listener instead
    api.logging = initLogForwarder();
  } else {
    // Direct logging to the output window
    initOutputWindowLogger();
  }

  log.info("Q# extension activating.");
  initTelemetry(context);

  checkForOldQdk();

  context.subscriptions.push(
    vscode.workspace.registerTextDocumentContentProvider(
      qsharpLibraryUriScheme,
      new QsTextDocumentContentProvider(),
    ),
  );

  context.subscriptions.push(...activateTargetProfileStatusBarItem());

  context.subscriptions.push(
    ...(await activateLanguageService(context.extensionUri)),
  );

  context.subscriptions.push(...registerQSharpNotebookHandlers());

  initAzureWorkspaces(context);
  initCodegen(context);
  activateDebugger(context);
  registerCreateNotebookCommand(context);
  registerWebViewCommands(context);
  initFileSystem(context);

  log.info("Q# extension activated.");

  return api;
}

export interface ExtensionApi {
  // Only available in test mode. Allows listening to extension log events.
  logging?: Logging;
}

function registerDocumentUpdateHandlers(languageService: ILanguageService) {
  vscode.workspace.textDocuments.forEach((document) => {
    updateIfQsharpDocument(document);
  });

  const subscriptions = [];
  subscriptions.push(
    vscode.workspace.onDidOpenTextDocument((document) => {
      const documentType = isQsharpNotebookCell(document)
        ? QsharpDocumentType.JupyterCell
        : isQsharpDocument(document)
        ? QsharpDocumentType.Qsharp
        : QsharpDocumentType.Other;
      if (documentType !== QsharpDocumentType.Other) {
        sendTelemetryEvent(
          EventType.OpenedDocument,
          { documentType },
          { linesOfCode: document.lineCount },
        );
      }
      updateIfQsharpDocument(document);
    }),
  );

  subscriptions.push(
    vscode.workspace.onDidChangeTextDocument((evt) => {
      updateIfQsharpDocument(evt.document);
    }),
  );

  subscriptions.push(
    vscode.workspace.onDidCloseTextDocument((document) => {
      if (isQsharpDocument(document) && !isQsharpNotebookCell(document)) {
        languageService.closeDocument(document.uri.toString());
      }
    }),
  );

  function updateIfQsharpDocument(document: vscode.TextDocument) {
    if (isQsharpDocument(document) && !isQsharpNotebookCell(document)) {
      // Regular (not notebook) Q# document.
      languageService.updateDocument(
        document.uri.toString(),
        document.version,
        document.getText(),
      );
    }
  }

  return subscriptions;
}

async function activateLanguageService(extensionUri: vscode.Uri) {
  const subscriptions: vscode.Disposable[] = [];

  const languageService = await loadLanguageService(extensionUri);

  // diagnostics
  subscriptions.push(...startCheckingQSharp(languageService));

  // synchronize document contents
  subscriptions.push(...registerDocumentUpdateHandlers(languageService));

  // synchronize notebook cell contents
  subscriptions.push(
    ...registerQSharpNotebookCellUpdateHandlers(languageService),
  );

  // synchronize configuration
  subscriptions.push(registerConfigurationChangeHandlers(languageService));

  // completions
  subscriptions.push(
    vscode.languages.registerCompletionItemProvider(
      qsharpLanguageId,
      createCompletionItemProvider(languageService),
      "@", // for attribute completion
    ),
  );

  // hover
  subscriptions.push(
    vscode.languages.registerHoverProvider(
      qsharpLanguageId,
      createHoverProvider(languageService),
    ),
  );

  // go to def
  subscriptions.push(
    vscode.languages.registerDefinitionProvider(
      qsharpLanguageId,
      createDefinitionProvider(languageService),
    ),
  );

  // find references
  subscriptions.push(
    vscode.languages.registerReferenceProvider(
      qsharpLanguageId,
      createReferenceProvider(languageService),
    ),
  );

  // signature help
  subscriptions.push(
    vscode.languages.registerSignatureHelpProvider(
      qsharpLanguageId,
      createSignatureHelpProvider(languageService),
      "(",
      ",",
    ),
  );

  // rename symbol
  subscriptions.push(
    vscode.languages.registerRenameProvider(
      qsharpLanguageId,
      createRenameProvider(languageService),
    ),
  );

  // code lens
  subscriptions.push(
    vscode.languages.registerCodeLensProvider(
      qsharpLanguageId,
      createCodeLensProvider(languageService),
    ),
  );

  // add the language service dispose handler as well
  subscriptions.push(languageService);

  return subscriptions;
}

async function updateLanguageServiceProfile(languageService: ILanguageService) {
  const targetProfile = getTarget();

  switch (targetProfile) {
    case "base":
    case "unrestricted":
      break;
    default:
      log.warn(`Invalid value for target profile: ${targetProfile}`);
  }
  log.debug("Target profile set to: " + targetProfile);

  languageService.updateConfiguration({
    targetProfile: targetProfile,
  });
}

async function loadLanguageService(baseUri: vscode.Uri) {
  const start = performance.now();
  const wasmUri = vscode.Uri.joinPath(baseUri, "./wasm/qsc_wasm_bg.wasm");
  const wasmBytes = await vscode.workspace.fs.readFile(wasmUri);
  await loadWasmModule(wasmBytes);
  const languageService = await getLanguageService(
    readFile,
    listDir,
    getManifest,
  );
  await updateLanguageServiceProfile(languageService);
  const end = performance.now();
  sendTelemetryEvent(
    EventType.LoadLanguageService,
    {},
    { timeToStartMs: end - start },
  );
  return languageService;
}

function registerConfigurationChangeHandlers(
  languageService: ILanguageService,
) {
  return vscode.workspace.onDidChangeConfiguration((event) => {
    if (event.affectsConfiguration("Q#.targetProfile")) {
      updateLanguageServiceProfile(languageService);
    }
  });
}

export class QsTextDocumentContentProvider
  implements vscode.TextDocumentContentProvider
{
  onDidChange?: vscode.Event<vscode.Uri> | undefined;
  provideTextDocumentContent(
    uri: vscode.Uri,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    token: vscode.CancellationToken,
  ): vscode.ProviderResult<string> {
    return getLibrarySourceContent(uri.path);
  }
}

function checkForOldQdk() {
  const oldQdkExtension = vscode.extensions.getExtension(
    "quantum.quantum-devkit-vscode",
  );

  const prereleaseQdkExtension = vscode.extensions.getExtension(
    "quantum.qsharp-lang-vscode-dev",
  );

  const releaseQdkExtension = vscode.extensions.getExtension(
    "quantum.qsharp-lang-vscode",
  );

  const previousQdkWarningMessage =
    'Extension "Microsoft Quantum Development Kit for Visual Studio" (`quantum.quantum-devkit-vscode`) found. We recommend uninstalling the prior QDK before using this release.';

  const bothReleaseAndPrereleaseWarningMessage =
    'Extension "Azure Quantum Development Kit (QDK)" has both release and pre-release versions installed. We recommend uninstalling one of these versions.';

  // we don't await the warnings below so we don't block extension initialization
  if (oldQdkExtension) {
    log.warn(previousQdkWarningMessage);
    vscode.window.showWarningMessage(previousQdkWarningMessage);
  }

  if (prereleaseQdkExtension && releaseQdkExtension) {
    log.warn(bothReleaseAndPrereleaseWarningMessage);
    vscode.window.showWarningMessage(bothReleaseAndPrereleaseWarningMessage);
  }
}
