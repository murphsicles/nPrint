use nprint_core::Stack;
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

pub fn verify_script(_script: &[u8], inputs: Vec<Vec<u8>>) -> Result<bool, VerifyError> {
    let mut stack = Stack::default();
    for input in inputs {
        stack.push(input);
    }
    stack.execute(&[]).map_err(|_| VerifyError::InvalidOp)?;
    Ok(!stack.main.is_empty() && !stack.main.last().unwrap().is_empty())
}
