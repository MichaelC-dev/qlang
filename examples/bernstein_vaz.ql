const SIZE = 3;
bits secret = 0b101;

function f(x: bits[SIZE]) -> bits[1] {
    x * secret
}
oracle U_f(x: qubits[SIZE], y: qubits[1]) loads f;

circuit bernstein_vazirani {
    register:
        qubits x = "0" * SIZE;
        qubits y = "1";

    apply:
        H(x);
        H(y);
        U_f(x, y);
        H(x);
        measure(x);
}

bernstein_vazirani.printCircuit(spaces=2);
const SHOTS = 2;
bernstein_vazirani.measure(shots=SHOTS);