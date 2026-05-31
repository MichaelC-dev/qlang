use std::vec;

use num_complex::Complex64;
use qlang::engine::{circuit::Circuit, gate::Gate, operator::Operator, qubit_register::QubitRegister};

#[test]
fn gate_from_string_parses_known_gates() {
    match Gate::from_string(&"H".to_string(), 1) {
        Some(Gate::Hadamard) => (),
        other => panic!("expected Hadamard, got {:?}", other),
    }

    match Gate::from_string(&"CNOT".to_string(), 2) {
        Some(Gate::CNot) => (),
        other => panic!("expected CNot, got {:?}", other),
    }

    match Gate::from_string(&"SWAP".to_string(), 2) {
        Some(Gate::Swap) => (),
        other => panic!("expected CNot, got {:?}", other),
    }
}

#[test]
fn operator_new_requires_matching_arity() {
    let op = Operator::new(Gate::Hadamard, vec![0]);
    assert!(op.is_some(), "expected valid operator for arity 1");

    let invalid = Operator::new(Gate::CNot, vec![0]);
    assert!(invalid.is_none(), "expected invalid operator for mismatched arity");
}

#[test]
fn qubit_register_can_be_created_and_applied() {
    let mut register = QubitRegister::from_pattern(&"0".to_string()).expect("create 1-qubit register");
    let op = Operator::new(Gate::Hadamard, vec![0]).expect("create hadamard operator");

    let result = register.apply(&op).expect("apply hadamard");
    assert!(result.is_empty(), "hadamard should not return measured qubits");
    assert_eq!(register.get_n(), 1);
}

#[test]
fn qubit_register_from_pattern_rejects_invalid_characters() {
    let invalid_pattern = "0A".to_string();
    assert!(QubitRegister::from_pattern(&invalid_pattern).is_err());
}

#[test]
fn qubit_register_from_pattern_accepts_valid_pattern() {
    let pattern = "+-".to_string();
    let register = QubitRegister::from_pattern(&pattern).expect("create register from pattern");
    assert_eq!(register.get_n(), 2);
    assert!(register.get_qubit(0).is_some());
    assert!(register.get_qubit(1).is_some());
}

#[test]
fn pauli_x_flips_zero_to_one() {
    let mut register = QubitRegister::from_pattern(&"0".to_string()).expect("create 1-qubit register");
    let op = Operator::new(Gate::RotateX, vec![0]).expect("create x operator");

    register.apply(&op).expect("apply x");
    assert_eq!(register.state, vec![Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)]);
}

#[test]
fn cnot_flips_target_when_control_is_one() {
    let mut register = QubitRegister::from_pattern(&"10".to_string()).expect("create 2-qubit register");
    let op = Operator::new(Gate::CNot, vec![0, 1]).expect("create cnot operator");

    register.apply(&op).expect("apply cnot");
    assert_eq!(register.state, vec![
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(1.0, 0.0)
    ]);
}

#[test]
fn deterministic_three_gate_circuit_produces_expected_state() {
    let mut register = QubitRegister::from_pattern(&"00".to_string()).expect("create 2-qubit register");

    let x0 = Operator::new(Gate::RotateX, vec![0]).expect("create x operator");
    let cnot = Operator::new(Gate::CNot, vec![0, 1]).expect("create cnot operator");
    let x1 = Operator::new(Gate::RotateX, vec![1]).expect("create x operator");

    register.apply(&x0).expect("apply x on qubit 0");
    register.apply(&cnot).expect("apply cnot");
    register.apply(&x1).expect("apply x on qubit 1");

    assert_eq!(register.state, vec![
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 0.0)
    ]);
}

#[test]
fn swap_gate_exchanges_two_qubits() {
    let mut register = QubitRegister::from_pattern(&"10".to_string()).expect("create 2-qubit register");
    let op = Operator::new(Gate::Swap, vec![0, 1]).expect("create swap operator");

    register.apply(&op).expect("apply swap");
    assert_eq!(register.state, vec![
        Complex64::new(0.0, 0.0),
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0)
    ]);
}

#[test]
fn deutsch_f_not_oracle_composition() {
    let pattern: String = "00".to_string();
    let ops: Vec<Operator> = vec![ 
        Operator::new(Gate::RotateX, vec![1]).expect("create X"),
        Operator::new(Gate::Hadamard, vec![0]).expect("create upper Hadamard"),
        Operator::new(Gate::Hadamard, vec![1]).expect("create lower Hadamard"),
        // f_not
        Operator::new(Gate::CNot, vec![0,1]).expect("create CNOT"),
        Operator::new(Gate::RotateX, vec![1]).expect("create f_not X"),
        // measure
        Operator::new(Gate::Hadamard, vec![0]).expect("create measure Hadamard"),
        Operator::new(Gate::Measure(1), vec![0]).expect("create measure")
    ];
    
    let circuit: Circuit = Circuit::new(pattern, ops);
    let result: Vec<usize> = circuit.execute(false).expect("run circuit");
    assert_eq!(result, vec![1]);
}
