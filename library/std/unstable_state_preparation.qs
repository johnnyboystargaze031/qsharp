// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

namespace Microsoft.Quantum.Unstable.StatePreparation {
    open Microsoft.Quantum.Convert;
    open Microsoft.Quantum.Diagnostics;
    open Microsoft.Quantum.Arrays;
    open Microsoft.Quantum.Math;

    /// # Summary
    /// Given a set of coefficients and a little-endian encoded quantum register,
    /// prepares a state on that register described by the given coefficients.
    ///
    /// # Description
    /// This operation prepares an arbitrary quantum
    /// state |𝜓⟩ with coefficients 𝑎ⱼ from
    /// the n-qubit computational basis state |0...0⟩.
    ///
    /// The action of U on the all-zeros state is given by
    /// $$
    /// \begin{align}
    ///     U \ket{0\cdots 0} = \ket{\psi} = \frac{\sum_{j=0}^{2^n-1}\alpha_j \ket{j}}{\sqrt{\sum_{j=0}^{2^n-1}|\alpha_j|^2}}.
    /// \end{align}
    /// $$
    ///
    /// # Input
    /// ## coefficients
    /// Array of up to 2ⁿ real coefficients. The j-th coefficient
    /// indexes the number state |j⟩ encoded in little-endian format.
    ///
    /// ## qubits
    /// Qubit register encoding number states in little-endian format. This is
    /// expected to be initialized in the computational basis state |0...0⟩.
    ///
    /// # Remarks
    /// `coefficients` will be normalized and padded with
    /// elements 𝑎ⱼ = 0.0 if fewer than 2ⁿ are specified.
    ///
    /// # Example
    /// The following snippet prepares the quantum state |𝜓⟩=√(1/8)|0⟩+√(7/8)|2⟩
    /// in the qubit register `qubitsLE`.
    /// ```qsharp
    /// let amplitudes = [Sqrt(0.125), 0.0, Sqrt(0.875), 0.0];
    /// use qubits = Qubit[2];
    /// let qubitsLE = LittleEndian(qubits);
    /// PreparePureStateD(amplitudes, qubitsLE);
    /// ```
    ///
    /// # References
    /// - Synthesis of Quantum Logic Circuits
    ///   Vivek V. Shende, Stephen S. Bullock, Igor L. Markov
    ///   https://arxiv.org/abs/quant-ph/0406176
    ///
    /// # See Also
    /// - Microsoft.Quantum.Unstable.StatePreparation.ApproximatelyPreparePureStateCP
    operation PreparePureStateD(coefficients : Double[], qubits : Qubit[]) : Unit is Adj + Ctl {
        let coefficientsAsComplexPolar = Mapped(a -> ComplexAsComplexPolar(Complex(a, 0.0)), coefficients);
        ApproximatelyPreparePureStateCP(0.0, coefficientsAsComplexPolar, qubits);
    }

    /// # Summary
    /// Given a set of coefficients and a little-endian encoded quantum register,
    /// prepares a state on that register described by the given coefficients,
    /// up to a given approximation tolerance.
    ///
    /// # Description
    /// This operation prepares an arbitrary quantum
    /// state |𝜓⟩ with complex coefficients rⱼ·𝒆^(𝒊·tⱼ) from
    /// the n-qubit computational basis state |0...0⟩.
    /// In particular, the action of this operation can be simulated by the
    /// a unitary transformation U which acts on the all-zeros state as
    ///
    /// $$
    /// \begin{align}
    ///     U\ket{0...0}
    ///         & = \ket{\psi} \\\\
    ///         & = \frac{
    ///                 \sum_{j=0}^{2^n-1} r_j e^{i t_j} \ket{j}
    ///             }{
    ///                 \sqrt{\sum_{j=0}^{2^n-1} |r_j|^2}
    ///             }.
    /// \end{align}
    /// $$
    ///
    /// # Input
    /// ## tolerance
    /// The approximation tolerance to be used when preparing the given state.
    ///
    /// ## coefficients
    /// Array of up to 2ⁿ complex coefficients represented by their
    /// absolute value and phase (rⱼ, tⱼ). The j-th coefficient
    /// indexes the number state |j⟩ encoded in little-endian format.
    ///
    /// ## qubits
    /// Qubit register encoding number states in little-endian format. This is
    /// expected to be initialized in the computational basis state
    /// |0...0⟩.
    ///
    /// # Remarks
    /// Negative input coefficients rⱼ < 0 will be treated as though
    /// positive with value |rⱼ|. `coefficients` will be padded with
    /// elements (rⱼ, tⱼ) = (0.0, 0.0) if fewer than 2ⁿ are
    /// specified.
    ///
    /// # References
    /// - Synthesis of Quantum Logic Circuits
    ///   Vivek V. Shende, Stephen S. Bullock, Igor L. Markov
    ///   https://arxiv.org/abs/quant-ph/0406176
    operation ApproximatelyPreparePureStateCP(
        tolerance : Double,
        coefficients : ComplexPolar[],
        qubits : Qubit[]
    ) : Unit is Adj + Ctl {
        let nQubits = Length(qubits);
        // pad coefficients at tail length to a power of 2.
        let coefficientsPadded = Padded(-2 ^ nQubits, ComplexPolar(0.0, 0.0), coefficients);
        let idxTarget = 0;
        // Determine what controls to apply
        let rngControl = nQubits > 1 ? (1 .. (nQubits - 1)) | (1..0);
        Adjoint ApproximatelyUnprepareArbitraryState(
            tolerance, coefficientsPadded, rngControl, idxTarget, qubits
        );
    }

    /// # Summary
    /// Implementation step of arbitrary state preparation procedure.
    internal operation ApproximatelyUnprepareArbitraryState(
        tolerance : Double,
        coefficients : ComplexPolar[],
        rngControl : Range,
        idxTarget : Int,
        register: Qubit[]
    ) : Unit is Adj + Ctl {
        // For each 2D block, compute disentangling single-qubit rotation parameters
        let (disentanglingY, disentanglingZ, newCoefficients) = StatePreparationSBMComputeCoefficients(coefficients);
        if (AnyOutsideToleranceD(tolerance, disentanglingZ)) {
            ApproximatelyMultiplexPauli(tolerance, disentanglingZ, PauliZ, register[rngControl], register[idxTarget]);

        }
        if (AnyOutsideToleranceD(tolerance, disentanglingY)) {
            ApproximatelyMultiplexPauli(tolerance, disentanglingY, PauliY, register[rngControl], register[idxTarget]);
        }
        // target is now in |0> state up to the phase given by arg of newCoefficients.

        // Continue recursion while there are control qubits.
        if (IsRangeEmpty(rngControl)) {
            let (abs, arg) = newCoefficients[0]!;
            if (AbsD(arg) > tolerance) {
                Exp([PauliI], -1.0 * arg, [register[idxTarget]]);
            }
        } elif (AnyOutsideToleranceCP(tolerance, newCoefficients)) {
            let newControl = (RangeStart(rngControl) + 1)..RangeStep(rngControl)..RangeEnd(rngControl);
            let newTarget = RangeStart(rngControl);
            ApproximatelyUnprepareArbitraryState(tolerance, newCoefficients, newControl, newTarget, register);
        }
    }

    /// # Summary
    /// Applies a Pauli rotation conditioned on an array of qubits, truncating
    /// small rotation angles according to a given tolerance.
    ///
    /// # Description
    /// This applies a multiply controlled unitary operation that performs
    /// rotations by angle $\theta_j$ about single-qubit Pauli operator $P$
    /// when controlled by the $n$-qubit number state $\ket{j}$.
    /// In particular, the action of this operation is represented by the
    /// unitary
    ///
    /// $$
    /// \begin{align}
    ///     U = \sum^{2^n - 1}_{j=0} \ket{j}\bra{j} \otimes e^{i P \theta_j}.
    /// \end{align}
    /// ##
    ///
    /// # Input
    /// ## tolerance
    /// A tolerance below which small coefficients are truncated.
    ///
    /// ## coefficients
    /// Array of up to $2^n$ coefficients $\theta_j$. The $j$th coefficient
    /// indexes the number state $\ket{j}$ encoded in little-endian format.
    ///
    /// ## pauli
    /// Pauli operator $P$ that determines axis of rotation.
    ///
    /// ## control
    /// $n$-qubit control register that encodes number states $\ket{j}$ in
    /// little-endian format.
    ///
    /// ## target
    /// Single qubit register that is rotated by $e^{i P \theta_j}$.
    ///
    /// # Remarks
    /// `coefficients` will be padded with elements $\theta_j = 0.0$ if
    /// fewer than $2^n$ are specified.
    internal operation ApproximatelyMultiplexPauli(
        tolerance : Double,
        coefficients : Double[],
        pauli : Pauli,
        control : Qubit[],
        target : Qubit) : Unit is Adj + Ctl {
        if pauli == PauliZ {
            ApproximatelyMultiplexZ(tolerance, coefficients, control, target);
        } elif pauli == PauliX {
            within {
                H(target);
            } apply {
                ApproximatelyMultiplexPauli(tolerance, coefficients, PauliZ, control, target);
            }
        } elif pauli == PauliY {
            within {
                Adjoint S(target);
            } apply {
                ApproximatelyMultiplexPauli(tolerance, coefficients, PauliX, control, target);
            }
        } elif pauli == PauliI {
            ApproximatelyApplyDiagonalUnitary(tolerance, coefficients, control);
        } else {
            fail $"MultiplexPauli failed. Invalid pauli {pauli}.";
        }
    }

    /// # Summary
    /// Implementation step of arbitrary state preparation procedure.
    internal function StatePreparationSBMComputeCoefficients(coefficients : ComplexPolar[]) : (Double[], Double[], ComplexPolar[]) {
        mutable disentanglingZ = [0.0, size = Length(coefficients) / 2];
        mutable disentanglingY = [0.0, size = Length(coefficients) / 2];
        mutable newCoefficients = [ComplexPolar(0.0, 0.0), size = Length(coefficients) / 2];

        for idxCoeff in 0 .. 2 .. Length(coefficients) - 1 {
            let (rt, phi, theta) = BlochSphereCoordinates(coefficients[idxCoeff], coefficients[idxCoeff + 1]);
            set disentanglingZ w/= idxCoeff / 2 <- 0.5 * phi;
            set disentanglingY w/= idxCoeff / 2 <- 0.5 * theta;
            set newCoefficients w/= idxCoeff / 2 <- rt;
        }

        return (disentanglingY, disentanglingZ, newCoefficients);
    }

    /// # Summary
    /// Computes the Bloch sphere coordinates for a single-qubit state.
    ///
    /// Given two complex numbers $a0, a1$ that represent the qubit state, computes coordinates
    /// on the Bloch sphere such that
    /// $a0 \ket{0} + a1 \ket{1} = r e^{it}(e^{-i \phi /2}\cos{(\theta/2)}\ket{0}+e^{i \phi /2}\sin{(\theta/2)}\ket{1})$.
    ///
    /// # Input
    /// ## a0
    /// Complex coefficient of state $\ket{0}$.
    /// ## a1
    /// Complex coefficient of state $\ket{1}$.
    ///
    /// # Output
    /// A tuple containing `(ComplexPolar(r, t), phi, theta)`.
    internal function BlochSphereCoordinates (a0 : ComplexPolar, a1 : ComplexPolar) : (ComplexPolar, Double, Double) {
        let abs0 = AbsComplexPolar(a0);
        let abs1 = AbsComplexPolar(a1);
        let arg0 = ArgComplexPolar(a0);
        let arg1 = ArgComplexPolar(a1);
        let r = Sqrt(abs0 * abs0 + abs1 * abs1);
        let t = 0.5 * (arg0 + arg1);
        let phi = arg1 - arg0;
        let theta = 2.0 * ArcTan2(abs1, abs0);
        return (ComplexPolar(r, t), phi, theta);
    }

    /// # Summary
    /// Applies an array of complex phases to numeric basis states of a register
    /// of qubits, truncating small rotation angles according to a given
    /// tolerance.
    ///
    /// # Description
    /// This operation implements a diagonal unitary that applies a complex phase
    /// $e^{i \theta_j}$ on the $n$-qubit number state $\ket{j}$.
    /// In particular, this operation can be represented by the unitary
    ///
    /// $$
    /// \begin{align}
    ///     U = \sum^{2^n-1}_{j=0}e^{i\theta_j}\ket{j}\bra{j}.
    /// \end{align}
    /// $$
    ///
    /// # Input
    /// ## tolerance
    /// A tolerance below which small coefficients are truncated.
    ///
    /// ## coefficients
    /// Array of up to $2^n$ coefficients $\theta_j$. The $j$th coefficient
    /// indexes the number state $\ket{j}$ encoded in little-endian format.
    ///
    /// ## qubits
    /// $n$-qubit control register that encodes number states $\ket{j}$ in
    /// little-endian format.
    ///
    /// # Remarks
    /// `coefficients` will be padded with elements $\theta_j = 0.0$ if
    /// fewer than $2^n$ are specified.
    ///
    /// # References
    /// - Synthesis of Quantum Logic Circuits
    ///   Vivek V. Shende, Stephen S. Bullock, Igor L. Markov
    ///   https://arxiv.org/abs/quant-ph/0406176
    internal operation ApproximatelyApplyDiagonalUnitary(tolerance : Double, coefficients : Double[], qubits : Qubit[])
    : Unit is Adj + Ctl {
        if IsEmpty(qubits) {
            fail "operation ApplyDiagonalUnitary -- Number of qubits must be greater than 0.";
        }

        // pad coefficients length at tail to a power of 2.
        let coefficientsPadded = Padded(-2 ^ Length(qubits), 0.0, coefficients);

        // Compute new coefficients.
        let (coefficients0, coefficients1) = MultiplexZCoefficients(coefficientsPadded);
        ApproximatelyMultiplexZ(tolerance,coefficients1, Most(qubits), Tail(qubits));

        if Length(coefficientsPadded) == 2 {
            // Termination case
            if AbsD(coefficients0[0]) > tolerance {
                Exp([PauliI], 1.0 * coefficients0[0], qubits);
            }
        } else {
            ApproximatelyApplyDiagonalUnitary(tolerance, coefficients0, Most(qubits));
        }
    }

    /// # Summary
    /// Applies a Pauli Z rotation conditioned on an array of qubits, truncating
    /// small rotation angles according to a given tolerance.
    ///
    /// # Description
    /// This applies the multiply controlled unitary operation that performs
    /// rotations by angle $\theta_j$ about single-qubit Pauli operator $Z$
    /// when controlled by the $n$-qubit number state $\ket{j}$.
    /// In particular, this operation can be represented by the unitary
    ///
    /// $$
    /// \begin{align}
    ///     U = \sum^{2^n-1}_{j=0} \ket{j}\bra{j} \otimes e^{i Z \theta_j}.
    /// \end{align}
    /// $$
    ///
    /// # Input
    /// ## tolerance
    /// A tolerance below which small coefficients are truncated.
    ///
    /// ## coefficients
    /// Array of up to $2^n$ coefficients $\theta_j$. The $j$th coefficient
    /// indexes the number state $\ket{j}$ encoded in little-endian format.
    ///
    /// ## control
    /// $n$-qubit control register that encodes number states $\ket{j}$ in
    /// little-endian format.
    ///
    /// ## target
    /// Single qubit register that is rotated by $e^{i P \theta_j}$.
    ///
    /// # Remarks
    /// `coefficients` will be padded with elements $\theta_j = 0.0$ if
    /// fewer than $2^n$ are specified.
    ///
    /// # References
    /// - Synthesis of Quantum Logic Circuits
    ///   Vivek V. Shende, Stephen S. Bullock, Igor L. Markov
    ///   https://arxiv.org/abs/quant-ph/0406176
    internal operation ApproximatelyMultiplexZ(
        tolerance : Double,
        coefficients : Double[],
        control : Qubit[],
        target : Qubit) : Unit is Adj + Ctl {

        body (...) {
            // pad coefficients length at tail to a power of 2.
            let coefficientsPadded = Padded(-2 ^ Length(control), 0.0, coefficients);

            if Length(coefficientsPadded) == 1 {
                // Termination case
                if AbsD(coefficientsPadded[0]) > tolerance {
                    Exp([PauliZ], coefficientsPadded[0], [target]);
                }
            } else {
                // Compute new coefficients.
                let (coefficients0, coefficients1) = MultiplexZCoefficients(coefficientsPadded);
                ApproximatelyMultiplexZ(tolerance, coefficients0, Most(control), target);
                if AnyOutsideToleranceD(tolerance, coefficients1) {
                    within {
                        CNOT(Tail(control), target);
                    } apply {
                        ApproximatelyMultiplexZ(tolerance, coefficients1, Most(control), target);
                    }
                }
            }
        }

        controlled (controlRegister, ...) {
            // pad coefficients length to a power of 2.
            let coefficientsPadded = Padded(2 ^ (Length(control) + 1), 0.0, Padded(-2 ^ Length(control), 0.0, coefficients));
            let (coefficients0, coefficients1) = MultiplexZCoefficients(coefficientsPadded);
            ApproximatelyMultiplexZ(tolerance,coefficients0, control, target);
            if AnyOutsideToleranceD(tolerance,coefficients1) {
                within {
                    Controlled X(controlRegister, target);
                } apply {
                    ApproximatelyMultiplexZ(tolerance,coefficients1, control, target);
                }
            }
        }
    }

    /// # Summary
    /// Implementation step of multiply-controlled Z rotations.
    internal function MultiplexZCoefficients(coefficients : Double[]) : (Double[], Double[]) {
        let newCoefficientsLength = Length(coefficients) / 2;
        mutable coefficients0 = [0.0, size = newCoefficientsLength];
        mutable coefficients1 = [0.0, size = newCoefficientsLength];

        for idxCoeff in 0 .. newCoefficientsLength - 1 {
            set coefficients0 w/= idxCoeff <- 0.5 * (coefficients[idxCoeff] + coefficients[idxCoeff + newCoefficientsLength]);
            set coefficients1 w/= idxCoeff <- 0.5 * (coefficients[idxCoeff] - coefficients[idxCoeff + newCoefficientsLength]);
        }

        return (coefficients0, coefficients1);
    }

    internal function AnyOutsideToleranceD(tolerance : Double, coefficients : Double[]) : Bool {
        for coefficient in coefficients {
            // NOTE: This function is not used in a recursion termination condition
            // only to determine if the multiplex step needs to be applied.
            // For tolerance 0.0 it is always applied due to >= comparison.
            if AbsD(coefficient) >= tolerance {
                return true;
            }
        }
        return false;
    }

    internal function AnyOutsideToleranceCP(tolerance : Double, coefficients : ComplexPolar[]) : Bool {
        for coefficient in coefficients {
            if AbsComplexPolar(coefficient) > tolerance {
                return true;
            }
        }
        return false;
    }

}

