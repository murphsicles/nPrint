#![no_std]  // Lightweight, embeddable core
#![feature(const_trait_impl)]  // For future const generics
#![doc = include_str!("../README.md")]

extern crate alloc;  // For Vec in no_std

use alloc::vec::Vec;
use nom::IResult;  // For parsing
use sv::script::{Opcode, OP_DUP, OP_SWAP, OP_PICK, OP_ROLL, OP_DROP};  // Assume sv::script exports these; adjust if needed (e.g., sv::script::opcodes::OP_DUP)

/// Custom declarative macro for building BSV scripts as Vec<u8>.
/// Usage: bsv_script! { OP_DUP, 2, OP_SWAP } -> pushes opcode bytes and literals.
/// Supports Opcode enums and i32 literals (pushed as minimal data).
#[macro_export]
macro_rules! bsv_script {
    ($($token:expr),*) => {{
        let mut script = Vec::new();
        $(
            match $token {
                t if let Some(op) = t as Option<Opcode> => script.extend_from_slice(&op.to_bytes()),
                n: i32 => {
                    // Minimal push: for small n, use OP_0 to OP_16; else PUSHDATA
                    if n == 0 {
                        script.push(OP_0 as u8);
                    } else if n >= 1 && n <= 16 {
                        script.push((OP_1 as u8 + (n as u8 - 1)));
                    } else {
                        let bytes = n.to_le_bytes().to_vec();
                        script.push(bytes.len() as u8);  // Simplify; use PUSHDATA1 for now
                        script.extend_from_slice(&bytes);
                    }
                },
                _ => compile_error!("Unsupported token in bsv_script!"),
            }
        )*
        script
    }};
}

/// Enum representing BSV Script opcodes (wrapper if sv doesn't export directly).
/// For now, assume sv::script::Opcode; extend for BSV-specific like OP_MUL.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BsvOpcode {
    Dup,    // OP_DUP
    Swap,   // OP_SWAP
    Pick,   // OP_PICK
    Roll,   // OP_ROLL
    Drop,   // OP_DROP
    // BSV-specific: Mul, Lshift, etc. Add as sv supports.
    PushData(Vec<u8>),
}

impl BsvOpcode {
    /// Converts to bytecode (use sv's if available).
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            BsvOpcode::Dup => vec![OP_DUP as u8],
            BsvOpcode::Swap => vec![OP_SWAP as u8],
            BsvOpcode::Pick => vec![OP_PICK as u8],
            BsvOpcode::Roll => vec![OP_ROLL as u8],
            BsvOpcode::Drop => vec![OP_DROP as u8],
            BsvOpcode::PushData(data) => {
                let mut bytes = Vec::with_capacity(data.len() + 1);
                bytes.push(0x4c);  // OP_PUSHDATA1
                bytes.push(data.len() as u8);
                bytes.extend_from_slice(data);
                bytes
            },
        }
    }
}

// Update Stack, MacroDef, expand_macro to use BsvOpcode.

// For xswap! using new macro:
#[macro_export]
macro_rules! xswap {
    ($n:expr) => {
        bsv_script! {
            $n - 1,  // <n-1>
            OP_PICK,
            $n - 1,  // <n-1>
            OP_ROLL,
            OP_SWAP,
            OP_DROP
        }
    };
}

// Rest of the code (Stack, MacroDef, expand_macro, parse_script, tests) remains similar, but use BsvOpcode.
// In apply, use sv::script evaluator if needed for full simulation.

#[cfg(test)]
mod tests {
    // ... Use proptest as before, but with bsv_script! outputs.
}
