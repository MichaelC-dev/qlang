use std::collections::HashMap;
use qlang::engine::circuit;
use qlang::engine::operator::Operator;
use crate::interpreter::parser::ast_types as ast;

#[derive(Clone)]
pub enum EvaluatorType {
    Const(usize),
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
    pub circuit: circuit::Circuit
}

impl Circuit {
    pub fn new(init: String, ops: Vec<Operator>) -> Self {
        Self { circuit: circuit::Circuit::new(init, ops) }
    }
}