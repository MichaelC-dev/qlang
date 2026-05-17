use std::fmt;
use crate::engine::black_box::BlackBox;

#[derive(Debug, Clone)]
pub enum Gate {
    Hadamard,
    RotateX,
    RotateY,
    RotateZ,
    ShiftT,
    ShiftS,
    CNot,
    Swap,
    CZ,
    Toffoli,
    CSwap,
    BlackBox(BlackBox),
    Measure(usize)
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Gate {
    pub fn from_string(gate_name: &String, m_arity: usize) -> Option<Gate> {
        match gate_name.as_str() {
            "H" => Some(Gate::Hadamard),
            "X" => Some(Gate::RotateX),
            "Y" => Some(Gate::RotateY),
            "Z" => Some(Gate::RotateZ),
            "T" => Some(Gate::ShiftT),
            "S" => Some(Gate::ShiftS),
            "CNOT" => Some(Gate::CNot),
            "SWAP" => Some(Gate::Swap),
            "CZ" => Some(Gate::CZ),
            "CCNOT" => Some(Gate::Toffoli),
            "CSWAP" => Some(Gate::CSwap),
            // we pass `m_arity` to this function because
            // measure's arity is not pre-determined.
            "measure" => Some(Gate::Measure(m_arity)),
            _ => None
        }
    }

    /// returns the number of qubits
    /// that a given gate applies to.
    pub fn arity(&self) -> usize {
        match self {
            Gate::Hadamard => 1,
            Gate::RotateX => 1,
            Gate::RotateY => 1,
            Gate::RotateZ => 1,
            Gate::ShiftT => 1,
            Gate::ShiftS => 1,
            Gate:: CNot => 2,
            Gate::Swap => 2,
            Gate::CZ => 2,
            Gate::Toffoli => 3,
            Gate::CSwap => 3,
            // user-implemented constructions
            // how many qubits the *gate* applies to
            Gate::BlackBox(bb) => bb.input_size() + bb.output_size(),
            Gate::Measure(tgts_count) => *tgts_count 
        }
    }
}