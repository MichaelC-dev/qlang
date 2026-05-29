use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::evaluator::environment::{Circuit, EvaluatorType};
use crate::interpreter::evaluator::runtime_error::RuntimeError;
use crate::interpreter::parser::ast_types as ast;

impl Evaluator {
    pub fn eval_method_call(&mut self, call: &ast::MethodCall) -> Result<(), RuntimeError> {
        let circuit_name = &call.name;
        let c: Circuit = match self.environment.working_env.get(circuit_name) {
            Some(EvaluatorType::Circuit(c)) => c.clone(),
            Some(_) => { return Err(RuntimeError::TypeMismatch); }
            None => { return Err(RuntimeError::VarNotFound(circuit_name.clone())); }
        };

        match call.call.as_str() {
            "distribution" => {
                let shots: usize = self.get_arg(&call.args, "shots")?.unwrap_or(1);
                println!("Executing circuit '{}' {} time/s.", circuit_name, shots);
                for i in 0..shots {
                    print!("SHOT {} OF {}: ", i+1, shots);
                    c.circuit.execute(true)?;
                }
                println!("");
            },

            "measure" => {
                let shots: usize = self.get_arg(&call.args, "shots")?.unwrap_or(1);
                println!("Measuring circuit '{}' {} time/s.", circuit_name, shots);
                for i in 0..shots {
                    print!("SHOT {} OF {}: ", i+1, shots);
                    let result: Vec<usize> = c.circuit.execute(false)?;
                    println!("Got measurement {:?}", result);
                }
                println!("");
            },

            // 2 spaces means we suffix each gate with two dashes
            "printCircuit" => {
                let spaces: usize = self.get_arg(&call.args, "spaces")?
                    .unwrap_or(1);
                println!("Circuit '{}': ", circuit_name);
                c.circuit.print_circuit(spaces)?;
            }

            _ => {
                let name: String = call.call.to_string();
                return Err(RuntimeError::MethodUndefined(name));
            }
        }

        return  Ok(());
    }
}


// ----- HELPERS -----
impl Evaluator {
    fn get_arg(
        &mut self,
        args: &Vec<ast::MethodArg>,
        arg_name: &str
    )-> Result<Option<usize>, RuntimeError> {
        for arg in args {
            let n: usize = self.expect_const(&arg.value)?;
            if arg.name.as_str() == arg_name { return Ok(Some(n)); }
        }
        return Ok(None);
    }
}