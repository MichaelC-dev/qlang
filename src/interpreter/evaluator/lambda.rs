use crate::interpreter::evaluator::ast;
use crate::interpreter::evaluator::{Evaluator};
use crate::interpreter::evaluator::environment::Bits;
use crate::interpreter::evaluator::environment::{Environment, EvaluatorType};

use qlang::engine::register_error::RegisterError;
use qlang::engine::black_box::Lambda;

use std::collections::HashMap;
use std::sync::Arc;


impl Evaluator {
    /// Given a function and its specifications, produce a
    /// `Lambda` that can be loaded into oracle operators.
    pub fn function_to_lambda(
        &self,
        func: &ast::Expr,
        params: &[String],
        param_bit_lengths: &[usize]
    ) -> Lambda {
        let func = func.clone();
        let params: Vec<String> = params.to_vec();
        let param_bit_lengths: Vec<usize> = param_bit_lengths.to_vec();
        let parent_env: Environment = self.environment.clone();

        Arc::new(move |args: Vec<usize>| -> Result<usize, RegisterError> {
            if args.len() != params.len() {
                return Err(RegisterError::RunTimeFailure(Some(format!(
                    "expected {} arguments, got {}",
                    params.len(),
                    args.len()
                ))));
            }

            let mut lambda_env: Environment = Environment {
                working_env: HashMap::new(),
                parent: Some(Box::new(parent_env.clone()))
            };

            for i in 0..params.len() {
                let bits: Bits = Bits {
                    literal: args[i],
                    length: param_bit_lengths[i]
                };
                lambda_env.working_env.insert(params[i].clone(), EvaluatorType::Bits(bits));
            }

            let mut lambda_eval: Evaluator = Evaluator::new();
            lambda_eval.environment = lambda_env;
            match lambda_eval.eval_expr(&func) {
                Ok(EvaluatorType::Bits(bits)) => Ok(bits.literal),
                Ok(_) => Err(RegisterError::RunTimeFailure(Some(
                    "function did not return a bits value".to_string()
                ))),
                Err(err) => Err(RegisterError::RunTimeFailure(Some(err.to_string())))
            }
        })
    }
}