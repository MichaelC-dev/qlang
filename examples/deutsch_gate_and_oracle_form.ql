function f_not(x: bits[1]) -> bits[1] { x ^ 0b1}
oracle U_f(x: qubits[1], y: qubits[1]) loads f_not;

# Circuit form #
circuit deutsch_no_bb {
    register:
        qubits x = "0";
        qubits y = "0";
    
    apply:
        X(y);
        H(x, y);
        # oracle #
        CNOT(x, y);
        X(y);
        # measure #
        H(x);
        measure(x);
}

# oracle form #
circuit deutsch_with_bb {
    register:
        qubits x = "0";
        qubits y = "0";

    apply:
        X(y);
        H(x, y);
        # oracle #
        U_f(x, y);
        # measure #
        H(x);
        measure(x);
}

deutsch_no_bb.printCircuit(spaces=2);
deutsch_with_bb.printCircuit(spaces=2);

deutsch_no_bb.measure(shots=3);
deutsch_with_bb.measure(shots=3);