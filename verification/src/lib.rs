use nprint_core::{expand_macro, MacroDef, Stack};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("Stack underflow")]
    Underflow,
    #[error("Invalid opcode")]
    InvalidOp,
    #[error("Verification failed")]
    Failed,
}

pub fn verify_macro(def: &MacroDef, args: &[i32], inputs: Vec<Vec<u8>>) -> Result<(), VerifyError> {
    let script = expand_macro(def, args);
    let mut stack = Stack::default();
    for input in inputs {
        stack.push(input);
    }
    stack.execute(&script).map_err(|_| VerifyError::InvalidOp)?;
    Ok(())
}

pub fn verify_script(script: &[u8], inputs: Vec<Vec<u8>>) -> Result<bool, VerifyError> {
    let mut stack = Stack::default();
    for input in inputs {
        stack.push(input);
    }
    stack.execute(script).map_err(|_| VerifyError::InvalidOp)?;
    Ok(!stack.main.is_empty() && !stack.main.last().unwrap().is_empty())
}
