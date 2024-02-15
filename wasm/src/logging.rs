// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::cell::RefCell;

use wasm_bindgen::prelude::*;

use js_sys::Function;
use log::LevelFilter;

#[wasm_bindgen]
extern "C" {
    type Error;

    #[wasm_bindgen(constructor)]
    fn new() -> Error;

    #[wasm_bindgen(structural, method, getter)]
    fn stack(error: &Error) -> String;

    #[wasm_bindgen(static_method_of = Error, getter = stackTraceLimit)]
    fn get_stack_trace_limit() -> Option<u32>;

    #[wasm_bindgen(static_method_of = Error, setter = stackTraceLimit)]
    fn set_stack_trace_limit(val: u32);
}

static MY_LOGGER: MyLogger = MyLogger;

// We're in Wasm, so only one thread anyway, but needs to be thread_local to avoid errors without Sync trait on RefCell
thread_local! {
    // Will hold a reference to the JS logging function that was passed in
    static LOG_JS_FN: RefCell<Option<Function>> = RefCell::new(None);
}

struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        // We only get here if logging is enabled, and thus there is a function to call, so a
        // call to the JavaScript side is definitely going to happen here. Hence the relative
        // perf cost of unwrapping the thread_local RefCell is probably negligible.
        LOG_JS_FN.with(|f| {
            let fnborrow = f.borrow();
            if let Some(js_fn) = fnborrow.as_ref() {
                let msg = format!("{}", record.args());
                let level = record.level() as i32;
                let target = record.target();
                // Ignore any errors calling the JavaScript provided handler
                let _ = js_fn.call3(
                    &JsValue::NULL, // JavaScript 'this' value
                    &JsValue::from(level),
                    &JsValue::from(target),
                    &JsValue::from(msg),
                );
            }
        });
    }

    fn flush(&self) {}
}

pub fn hook(info: &std::panic::PanicInfo) {
    // Code similar to https://github.com/rustwasm/console_error_panic_hook/blob/master/src/lib.rs#L97
    // for capturing the JS stack as well as the panic info
    let mut msg = info.to_string();
    msg.push_str("\n\nStack:\n\n");
    let e = Error::new();
    let stack = e.stack();
    msg.push_str(&stack);
    msg.push_str("\n\n");

    let err_text = format!("Wasm panic occurred: {}", msg);
    log::error!(target: "wasm", "{}", &err_text);
}

#[wasm_bindgen(js_name=initLogging)]
pub fn init_logging(callback: JsValue, level: i32) -> Result<(), JsError> {
    if !callback.is_function() {
        return Err(JsError::new("Invalid callback"));
    }

    if !(0..=5).contains(&level) {
        return Err(JsError::new("Invalid logging level"));
    }

    let thefn: Function = callback.dyn_into().unwrap(); // Already checked it was a function
    LOG_JS_FN.with(|f| {
        *f.borrow_mut() = Option::Some(thefn);
    });

    // The below will return an error if it was already set
    log::set_logger(&MY_LOGGER).map_err(|e| {
        // The stack trace default of 10 frames gets taken up by the
        // logging and panic handling code itself. Temporarily increase the limit.
        let old_stack_trace_limit = Error::get_stack_trace_limit().unwrap_or(10);
        Error::set_stack_trace_limit(old_stack_trace_limit.max(20));
        // JsError constructor will capture a stack trace
        let err = JsError::new(&e.to_string());
        Error::set_stack_trace_limit(old_stack_trace_limit);
        err
    })?;
    std::panic::set_hook(Box::new(hook));

    set_log_level(level);
    Ok(())
}

#[wasm_bindgen(js_name=setLogLevel)]
pub fn set_log_level(level: i32) {
    // NOTE: Could also accept a string here too for user-friendliness
    log::set_max_level(match level {
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
    log::info!("Log level set to {}", level);
}
