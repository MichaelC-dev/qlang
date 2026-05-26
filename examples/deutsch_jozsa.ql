const SIZE = 4;
bits s = 0b1101;

function f_balanced(x: bits[SIZE]) -> bits[1] { x * s }
function f_constant(x: bits[SIZE]) -> bits[1] { 0b1 }

oracle U_f(x: qubits[SIZE], y: qubits[1]) loads f_balanced;

circuit deutsch_jozsa {
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

deutsch_jozsa.measure(shots=3);