#![no_std]
#![doc = include_str!("../../README.md")]

extern crate alloc;

use alloc::{vec, vec::Vec, string::String, format};
use nom::IResult;
#[allow(unused_imports)]
use sv::script::op_codes::{OP_DUP, OP_SWAP, OP_PICK, OP_ROLL, OP_DROP, OP_HASH160, OP_CAT, OP_1, OP_FALSE};
use sv::script::stack::encode_num;

const OP_SHA256: u8 = 0xa8;
const OP_EQUAL: u8 = 0x87;
const OP_2: u8 = 0x52;
const OP_3: u8 = 0x53;
const OP_ADD: u8 = 0x93;
const OP_5: u8 = 0x55;

/// Custom macro for BSV scripts as Vec<u8>.
/// Supports u8 opcodes and i32 literals (minimal push).
#[macro_export]
macro_rules! bsv_script {
    ($($token:expr),*) => {{
        let mut script = Vec::new();
        $(
            match $token {
                op if op >= 0 && op <= 255 => {
                    script.push(op as u8);
                }
                n => {
                    if n == 0 {
                        script.push(OP_FALSE);
                    } else if n >= 1 && n <= 16 {
                        script.push(OP_1 + (n as u8 - 1));
                    } else {
                        match encode_num(n as i64) {
                            Ok(bytes) => {
                                script.push(bytes.len() as u8);
                                script.extend_from_slice(&bytes);
                            }
                            Err(e) => panic!("Failed to encode number: {}", e),
                        }
                    }
                }
            }
        )*
        script
    }};
}

/// Stack model: Simulates main and alt stacks as 2PDA.
#[derive(Clone, Debug, Default)]
pub struct Stack {
    main: Vec<Vec<u8>>,
    alt: Vec<Vec<u8>>,
}

impl Stack {
    pub fn push(&mut self, value: Vec<u8>) { self.main.push(value); }
    pub fn pop(&mut self) -> Vec<u8> { self.main.pop().expect("Stack underflow") }

    /// Symbolic execution for verification.
    pub fn execute(&mut self, script: &[u8]) -> Result<(), String> {
        let mut i = 0;
        while i < script.len() {
            let op = script[i];
            i += 1;
            match op {
                OP_DUP => {
                    let top = self.main.last().cloned().ok_or("Dup underflow")?;
                    self.push(top);
                }
                OP_SWAP => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a);
                    self.push(b);
                }
                OP_PICK => {
                    let n = self.pop()[0] as usize;
                    let item = self.main[self.main.len() - 1 - n].clone();
                    self.push(item);
                }
                OP_ROLL => {
                    let n = self.pop()[0] as usize;
                    let item = self.main.remove(self.main.len() - 1 - n);
                    self.push(item);
                }
                OP_DROP => { let _ = self.pop(); }
                _ => return Err(format!("Unsupported op: {}", op)),
            }
        }
        Ok(())
    }
}

/// Macro element: Opcode or parameter.
#[derive(Clone, Debug)]
pub enum MacroElem {
    Op(u8),
    Param(usize),
}

/// Macro definition: (name, params, template) per article.
#[derive(Clone, Debug)]
pub struct MacroDef {
    pub name: String,
    pub param_count: usize,
    pub template: Vec<MacroElem>,
}

/// Expand macro hygienically.
pub fn expand_macro(def: &MacroDef, args: &[i32]) -> Vec<u8> {
    if args.len() != def.param_count { panic!("Arg mismatch"); }
    let mut expanded = Vec::new();
    for elem in &def.template {
        match elem {
            MacroElem::Op(op) => expanded.push(*op),
            MacroElem::Param(idx) => {
                let n = args[*idx];
                if n >= 0 && n <= 16 {
                    expanded.push(OP_1 - 1 + n as u8);
                } else {
                    let bytes = n.to_le_bytes().to_vec();
                    expanded.push(bytes.len() as u8);
                    expanded.extend(bytes);
                }
            }
        }
    }
    expanded
}

/// OP_XSWAP_n: Swaps top with nth item.
/// Expands to [<n-1>, OP_PICK, <n-1>, OP_ROLL, OP_SWAP, OP_DROP].
#[macro_export]
macro_rules! xswap {
    ($n:expr) => {
        bsv_script! { $n - 1, OP_PICK, $n - 1, OP_ROLL, OP_SWAP, OP_DROP }
    };
}

/// OP_XDROP_n: Drops nth item.
/// Expands to [<n-1>, OP_ROLL, OP_DROP].
#[macro_export]
macro_rules! xdrop {
    ($n:expr) => {
        bsv_script! { $n - 1, OP_ROLL, OP_DROP }
    };
}

/// OP_XROT_n: Rotates nth item to top.
/// Expands to [<n>, OP_ROLL].
#[macro_export]
macro_rules! xrot {
    ($n:expr) => {
        bsv_script! { $n, OP_ROLL }
    };
}

/// OP_HASHCAT: Duplicates top, hashes one, concatenates.
/// Expands to [OP_DUP, OP_HASH160, OP_CAT].
#[macro_export]
macro_rules! hashcat {
    () => {
        bsv_script! { OP_DUP, OP_HASH160, OP_CAT }
    };
}

/// LOOP[n]{body}: Unrolls body n times statically.
/// Example: loop_unroll!(3, { OP_DUP }) -> [OP_DUP, OP_DUP, OP_DUP].
#[macro_export]
macro_rules! loop_unroll {
    ($($count:expr, { $($body:tt)* })+) => {{
        let mut script = Vec::new();
        $(
            for _ in 0..$count {
                script.extend(bsv_script! { $($body)* });
            }
        )+
        script
    }};
}

/// Parser stub for script to elements.
pub fn parse_script(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    Ok((input, vec![]))
}
