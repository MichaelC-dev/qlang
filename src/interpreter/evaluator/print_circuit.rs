use qlang::engine::{gate::Gate, operator::Operator};
use std::collections::HashMap;
use crate::interpreter::evaluator::environment::Circuit;
use crate::interpreter::evaluator::RuntimeError;

impl Circuit {
    pub fn print_circuit(&self, spaces: usize) -> Result<(), RuntimeError> {
        let mut wires: Vec<String> = Vec::new();
        for c in self.init.chars().into_iter() {
            wires.push(format!("|{}> ", c));
        }

        for op in self.ops.iter() {
            // yield non-identity symbols to print.
            let symbols: HashMap<usize, String> = symbols_from_op(
                &op,
                self.init.len()
            )?;
            let mut i: usize = 0;
            for wire in wires.iter_mut() {
                let curr: &str = match symbols.get(&i) {
                    Some(g) => { g },
                    None => { "--" }
                };
                wire.push_str(curr);
                wire.push_str(&"-".repeat(spaces));
                i += 1;
            }
        }

        for wire in wires.iter() {
            println!("{}", wire);
            println!("");
        }

        return Ok(());
    }
}


/// Given an operator, return a HashMap that demonstrates how an
/// instruction is to be rendered. For example, given a CNOT occuring at
/// `vec![0, 2]`, return `{0: "*", 2: "X"}`
fn symbols_from_op(
    op: &Operator,
    circuit_size: usize
) -> Result<HashMap<usize, String>, RuntimeError> {
    let mut table: HashMap<usize, String> = HashMap::new();

    // produce symbols
    let symbols: Vec<&str> = match op.gate() {
        Gate::Hadamard => vec!["H"],
        Gate::RotateX => vec!["X"],
        Gate::RotateY => vec!["Y"],
        Gate::RotateZ => vec!["Z"],
        Gate::ShiftS => vec!["S"],
        Gate::ShiftT => vec!["T"],

        Gate::CNot => vec!["*", "X"],
        Gate::CZ => vec!["*", "Z"],
        Gate::Swap => vec!["Sw", "Sw"],
        Gate::BlackBox(b) => {
            let x_s: Vec<&str> = vec!["Bx"; b.input_size()];
            let y_s: Vec<&str> = vec!["By"; b.output_size()];
            [x_s, y_s].concat()
        },

        Gate::Toffoli => vec!["*", "*", "X"],
        Gate::CSwap => vec!["*", "*", "X"],

        Gate::Measure(n) => vec!["M"; *n],
    };

    // pad symbols that aren't length two (so the wires are evenly placed)
    let mut symbols: Vec<String> = symbols.iter()
        .map(|s| s.to_string()).collect();
    for symbol in symbols.iter_mut() {
        if symbol.len() == 1 {
            symbol.push_str("-");
        }
    }

    // bind symbols to indices
    if op.applies().len() != symbols.len() {
        return Err(RuntimeError::Fatal); // todo
    }
    let mut i: usize = 0;
    for idx in op.applies().iter() {
        if *idx >= circuit_size {
            return Err(RuntimeError::Fatal); // todo
        }
        table.insert(*idx, symbols[i].to_string());
        i += 1;
    }

    return Ok(table);
}