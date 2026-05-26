const CSIZE = 4;
bits secret = 0b0110;

function f(x: bits[CSIZE]) -> bits[1] {
    x * secret
}
oracle U_f(x: qubits[CSIZE], y: qubits[1]) loads f;

circuit bernstein_vazirani {
    register:
        qubits x = "0" * CSIZE;
        qubits y = "-";

    apply:
        H(x);
        U_f(x, y);
        H(x);
        measure(x);
}

const SHOTS = 5;
bernstein_vazirani.measure(shots=SHOTS);