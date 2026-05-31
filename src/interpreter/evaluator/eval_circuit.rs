use qlang::engine::black_box::Lambda;
use qlang::engine::gate::Gate;
use qlang::engine::operator::Operator;

use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::evaluator::environment::{Circuit, EvaluatorType};
use crate::interpreter::evaluator::runtime_error::RuntimeError;
use crate::interpreter::parser::ast_types as ast;


// Although there is a case to use a HashMap over a Vec here,
// Vec's make it easy to preserve the ordering of variables. The
// tradeoff of O(n) searching through the vector is also negligible,
// seeing that the engine permits only 16 unique qubits in a given circuit.
type CircuitLookup = Vec<(String, usize)>; // (identifier, number_of_qubits)

impl Evaluator {
    pub fn eval_circuit_decl(&mut self, decl: &ast::CircuitDecl) -> Result<(), RuntimeError> {
        let (register_str, lookup) = self.construct_lookup(decl)?;

        // build engine ops
        let mut ops: Vec<Operator> = Vec::new();
        for instruction in decl.instructions.iter() {
            let mut tgts: Vec<usize> = Vec::new(); 
            for arg in &instruction.args {
                // identify target qubits from each qubit arg
                // being passed to a given gate.
                let result: Vec<usize> = resolve_targets(arg, &lookup)?;
                for r in result { tgts.push(r); }
            }

            // if the gate exists in the environment as an oracle,
            // then we don't need to determine how it is applied. 
            if let Some(EvaluatorType::Oracle(o)) =  self.environment.working_env.get(&instruction.name) {
                let (x, y): (usize, usize) = (o.input.0, o.input.1);
                let lambda: Lambda = self.function_to_lambda(&o.loads.func, &o.loads.input, &[x]);
                let op: Operator = Operator::new_u_f(lambda, x, y);
                ops.push(op);
                continue;
            }

            let gate: Gate = self.resolve_gate(&instruction.name, tgts.len())?;

            // If Gate arity is 1, then we want to apply the gate point-wise
            // to each target.
            if gate.arity() == 1 {
                let tmp_ops: Option<Vec<Operator>> = tgts.iter()
                    .map(|idx| Operator::new(gate.clone(), vec![*idx]))
                    .collect();
                if tmp_ops.is_none() { return Err(RuntimeError::Fatal); }
                let tmp_ops = tmp_ops.unwrap();
                for op in tmp_ops { ops.push(op); }
            }
            else { // TODO - there's probably a smarter way of representing this if/ else
                let op: Operator = match Operator::new(gate, tgts) {
                    Some(op) => op,
                    _ => { return Err(RuntimeError::OracleConstructionFailed); }
                };
                ops.push(op);
            }
        }

        let circuit_name: String = decl.name.clone();
        let circuit: Circuit = Circuit::new(register_str, ops);

        self.environment.working_env.insert(circuit_name, EvaluatorType::Circuit(circuit));

        return Ok(());
    }
}


// ----- HELPERS -----
impl Evaluator {
    fn resolve_gate(
        &self, 
        gate_name: &String,
        m_arity: usize
    ) -> Result<Gate, RuntimeError> {
        match Gate::from_string(gate_name, m_arity) {
            Some(g) => Ok(g),
            None => Err(RuntimeError::VarNotFound(gate_name.clone()))
        }
    }

    fn construct_lookup(
        &mut self,
        decl: &ast::CircuitDecl
    ) -> Result<(String, CircuitLookup), RuntimeError> {
        let mut register_str: String = String::new();
        let mut lookup: CircuitLookup = Vec::new();

        // Builds the entire register to be fed into the engine's QubitRegister,
        // along with the means to index into the register.
        //
        // For example, given the assignments `x := 00, y := ++`,
        // we would want to construct the register "00++" (we assume order);
        // but we also want to know that `x` begins at qubit 0, and `y` begins at 2.
        for reg in decl.registers.iter() {
            let m: usize = match &reg.multiplies {
                Some(n) => self.expect_const(&n)?,
                None => 1
            };
            let inits: String = reg.init.repeat(m);
            register_str += &inits;
            lookup.push((reg.name.clone(), inits.len()));
        }

        return Ok((register_str, lookup));
    }
}


/// given a `table` of identifiers, an identifier, and a (zero-indexed) 
/// pivot, return the index of `identifer[plus]`  in the circuits 
/// qubits register.
/// 
/// for example, given a `table` [("x", 2), ("y", 3)], and we wanted to find
/// `get_index(table, "y", 1)`, `3` should be returned, since `y[1]` is found at
/// index three. returns `None` if the identifier could not be resolved, or if
/// pivot exceeds the length of the resolved idenitifer.
fn get_index(table: &CircuitLookup, identifer: &String, plus: usize) -> Result<usize, RuntimeError> {
    let mut curr_idx: usize = 0;
    for (name, length) in table.iter() {
        if name != identifer {
            curr_idx += length;
            continue;
        }
        // match made, ensure that plus is valid
        return match plus < *length {
            true  => Ok(curr_idx + plus),
            false => Err(RuntimeError::OutOfBounds(identifer.clone(), plus))
        }
    }
    return Err(RuntimeError::VarNotFound(identifer.clone()));
}

/// Given a `CircuitRef`, identify the qubits
/// that the reference is attempting to target.
/// 
/// This method uses a `CircuitLookup to assist`.
/// For example, given the table `[("x", 3)]`, and
/// a circuit reference `x[1]`, it should return `vec![1]`
fn resolve_targets(
    arg: &ast::CircuitRef,
    lookup: &CircuitLookup
) -> Result<Vec<usize>, RuntimeError> {
    match arg.applies {
        ast::Applies::One(n) => {
            let idx: usize = get_index(lookup, &arg.name, n)?;
            return Ok(vec![idx]);
        }
        ast::Applies::All => {
            let idx: usize = get_index(&lookup, &arg.name, 0)?;
            let length: Option<usize> = lookup.iter()
                .find(|(a, _)| &arg.name == a)
                .map(|(_, n)| *n);
            let length: usize = match length {
                Some(n) => n,
                // unreacahable, as `idx` had been reached.
                None => { return Err(RuntimeError::Fatal); }
            };
            return Ok((idx..length+idx).collect());
        }
    }
}