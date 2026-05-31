use std::collections::HashMap;
use crate::engine::gate::Gate;
use crate::engine::qubit_register::QubitRegister;
use crate::engine::operator::Operator;
use crate::engine::register_error::RegisterError;


// ----- CONSTRUCTORS -----
#[derive(Debug, Clone)]
pub struct Circuit {
    init: String,
    ops: Vec<Operator>    
}

impl Circuit {
    pub fn new(init: String, ops: Vec<Operator>) -> Self {
        Self { init, ops }
    }
}


// ----- API CALLABLE METHODS
impl Circuit {
    /// executes the circuit implemented by `self`. A return value
    /// of `Ok(v)` implies safe execution, and `v` stores the last measurement
    /// captured in the system. If `v` is empty, it means that the circuit did not
    /// end with a measurement.
    /// 
    /// enabling `verbose` will print the final distribution to `stdout`.
    pub fn execute(&self, verbose: bool) -> Result<Vec<usize>, RegisterError> {
        let mut register: QubitRegister = QubitRegister::from_pattern(&self.init)?;
        let mut measurement: Vec<usize> = Vec::new();
        for op in &self.ops {
            measurement = register.apply(op)?;
        }
        if verbose {
            println!("Yielded distribution of {}", register.to_string());
        }
        Ok(measurement)
    }
}

impl Circuit {
    pub fn print_circuit(&self, spaces: usize) -> Result<(), RegisterError> {
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


// ----- HELPERS -----
/// Given an operator, return a HashMap that demonstrates how an
/// instruction is to be rendered. For example, given a CNOT occuring at
/// `vec![0, 2]`, return `{0: "*", 2: "X"}`
fn symbols_from_op(
    op: &Operator,
    circuit_size: usize
) -> Result<HashMap<usize, String>, RegisterError> {
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
        println!("{} vs {}", op.applies().len(), symbols.len());
        return Err(RegisterError::RenderFailed);
    }

    let mut i: usize = 0;
    for idx in op.applies().iter() {
        if *idx >= circuit_size {
            return Err(RegisterError::RenderFailed);
        }
        table.insert(*idx, symbols[i].to_string());
        i += 1;
    }

    return Ok(table);
}