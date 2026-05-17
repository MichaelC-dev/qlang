use std::collections::HashMap;
use qlang::engine::{operator::Operator, qubit_register::QubitRegister};
use crate::interpreter::parser::ast_types as ast;
use crate::interpreter::evaluator::runtime_error::RuntimeError;

#[derive(Clone)]
pub enum EvaluatorType {
    Bits(Bits),
    Function(Function),
    Oracle(Oracle),
    Circuit(Circuit)
}

#[derive(Clone)]
pub struct Environment {
    pub working_env: HashMap<String, EvaluatorType>,
    pub parent: Option<Box<Environment>>
}

impl Environment {
    pub fn resolve(&self, name: &String) -> Option<EvaluatorType> {
        match self.working_env.get(name) {
            Some(t) => { return Some(t.clone()); },
            None => { /* no-op */}
        };
        return match &self.parent {
            Some(e) => { e.resolve(name) },
            None => None
        };
    }
}


// ----- BITS DEFINITION -----
#[derive(Debug, Clone, Copy)]
pub struct Bits {
    pub literal: usize,
    pub length: usize
}

// ----- FUNCTION DEFINITION -----
#[derive(Debug, Clone)]
pub struct Function {
    /// the identifier for each argument.
    pub input: Vec<String>,
    pub func: ast::Expr
}

// ----- ORACLE DEFINITION -----
#[derive(Debug, Clone)]
pub struct Oracle {
    /// the cardinality of each `qubits` argument.
    /// 
    /// PRE: `(input.fst > 0) && (input.snd > 0)`
    pub input: (usize, usize),
    pub loads: Function
}

// ----- CIRCUIT DEFINITION -----
#[derive(Debug, Clone)]
pub struct Circuit {
    pub init: String,
    pub ops: Vec<Operator>
}

impl Circuit {
    /// executes the circuit implemented by `self`. A return value
    /// of `Ok(v)` implies safe execution, and `v` stores the last measurement
    /// captured in the system. If `v` is empty, it means that the circuit did not
    /// end with a measurement.
    /// 
    /// enabling `verbose` will print the final distribution to `stdout`.
    pub fn execute(&self, verbose: bool) -> Result<Vec<usize>, RuntimeError> {
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