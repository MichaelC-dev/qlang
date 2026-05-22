circuit cz_eval {
    register:
        qubits x = "0";
        qubits y = "0";

    apply:
        H(x);
        CZ(x, y);
        H(x);
        measure(x,y);
}

cz_eval.measure(shots=10);