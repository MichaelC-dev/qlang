pub mod runtime_error;
mod eval_expr;
mod eval_circuit;
mod eval_method;
mod environment;
mod print_circuit;

use std::collections::HashMap;
use crate::interpreter::evaluator::runtime_error::RuntimeError;
use crate::interpreter::evaluator::environment::{Environment, EvaluatorType,};
use crate::interpreter::evaluator::environment::{Function, Oracle}; 
use crate::interpreter::parser::ast_types as ast;


pub struct Evaluator {
    environment: Environment
}


/// EVALUATION ROUTINES 
impl Evaluator {
    /// consumes a `bits` or `const` assignment, and loads
    /// the respective variable into `self.environment`.
    fn eval_assignment(&mut self, assign: &ast::Assignment) -> Result<(), RuntimeError> {
        let var_name: String = assign.name.clone();

        // filter out non-bits, non-const values
        let result: EvaluatorType = match self.eval_expr(&assign.value)? {
            bits @ EvaluatorType::Bits(_) => bits,
            num @ EvaluatorType::Const(_) => num,
            _ => { return Err(RuntimeError::TypeMismatch); }
        };
        self.environment.working_env.insert(var_name, result);
        return Ok(()); 
    }


    fn eval_function_decl(&mut self, decl: &ast::FunctionDecl) -> Result<(), RuntimeError> {
        let func_name: String = decl.name.clone();

        let symbol_table: Vec<String> = decl.params.iter().
            map(|p| p.name.to_string())
            .collect();

        let func_literal: Function = Function {
            input: symbol_table,
            func: decl.body.clone()
        };
        let typed: EvaluatorType = EvaluatorType::Function(func_literal);
        self.environment.working_env.insert(func_name, typed);
        return Ok(());
    }


    fn eval_oracle_decl(&mut self, decl: &ast::OracleDecl) -> Result<(), RuntimeError> {
        let oracle_name:  String = decl.name.clone();
        let oracle_loads: String = decl.loads.clone();
        let (in_size, out_size) = self.sanitize_oracle(decl)?;

        // ensure that `oracle_loads` exists in the environment
        let f = match self.environment.working_env.get(&oracle_loads) {
            None => { return Err(RuntimeError::VarNotFound(oracle_loads)) },
            Some(EvaluatorType::Function(f)) => f,
            Some(_) => { return Err(RuntimeError::TypeMismatch) }
        };

        // oracle is safe to construct
        let oracle: Oracle = Oracle { input: (in_size, out_size), loads: f.clone() };
        self.environment.working_env.insert(oracle_name, EvaluatorType::Oracle(oracle));
        return Ok(());
    }


    fn eval_statement(&mut self, stmt: ast::Statement) -> Result<(), RuntimeError> {
        match stmt {
            ast::Statement::Expr(_) => { Ok(()) }, // vacuous expressions can be skipped.
            ast::Statement::Assignment(bits) => self.eval_assignment(&bits),
            ast::Statement::Function(decl) => self.eval_function_decl(&decl),
            ast::Statement::Oracle(decl) => self.eval_oracle_decl(&decl),
            ast::Statement::Circuit(decl) => self.eval_circuit_decl(&decl),
            ast::Statement::MethodCall(call) => self.eval_method_call(&call)
        }
    }
}



/// PUBLIC API METHODS
impl Evaluator {
    pub fn new() -> Self {
        let environment: Environment = Environment {
            working_env: HashMap::new(),
            parent: None
        };
        Self { environment }
    }

    pub fn eval(&mut self, program: &mut ast::Program) -> Result<(), RuntimeError> {
        for _ in 0..program.len() {
            let stmt: ast::Statement = program.remove(0);
            self.eval_statement(stmt)?;
        }
        return Ok(());
    }
}


// ----- HELPERS -----

impl Evaluator {
    /// Given an oracle declaration, ensure that it is correctly typed.
    /// 
    /// This includes ensuring that it contains exactly two params (both
    /// of which must be typed as `qubits`), and ensuring that the first
    /// param is greater-than or equal to the second.
    fn sanitize_oracle(&mut self, decl: &ast::OracleDecl) -> Result<(usize, usize), RuntimeError> {
        let length: usize = decl.params.len();
        if length != 2 {
            return Err(RuntimeError::IncorrectArgs(2, length));
        }

        let in_size: usize = match &decl.params[0].ty {
            ast::Type::Qubits(n) => self.expect_const(&n)?,
            _ => { return Err(RuntimeError::TypeMismatch); }
        };
        let out_size: usize = match &decl.params[1].ty {
            ast::Type::Qubits(n) => self.expect_const(&n)?,
            _ => { return Err(RuntimeError::TypeMismatch); }
        };

        match in_size >= out_size {
            true => Ok((in_size, out_size)),
            false => Err(RuntimeError::OracleConstructionFailed)
        }
    }


    pub fn expect_const(&mut self, expr: &ast::Expr) -> Result<usize, RuntimeError> {
        match self.eval_expr(expr)? {
            EvaluatorType::Const(n) => Ok(n),
            _ => Err(RuntimeError::Fatal)
        }
    }
}