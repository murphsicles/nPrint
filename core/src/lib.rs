#![no_std]
#![doc = include_str!("../../README.md")]

extern crate alloc;

use alloc::{vec, vec::Vec, string::String, format};
use alloc::string::ToString;
use nom::IResult;
#[allow(unused_imports)]
use sv::script::op_codes::{OP_DUP, OP_SWAP, OP_PICK, OP_ROLL, OP_DROP, OP_HASH160, OP_CAT, OP_1, OP_FALSE, OP_PUSHDATA1, OP_PUSHDATA2, OP_PUSHDATA4, OP_16};

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
                        script.push(sv::script::op_codes::OP_FALSE);
                    } else if n >= 1 && n <= 16 {
                        script.push(sv::script::op_codes::OP_1 + (n as u8 - 1));
                    } else {
                        match sv::script::stack::encode_num(n as i64) {
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
    pub main: Vec<Vec<u8>>,
    pub alt: Vec<Vec<u8>>,
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
                    let item = self.main.get(self.main.len() - 1 - n).cloned().ok_or("Pick underflow")?;
                    self.push(item);
                }
                OP_ROLL => {
                    let n = self.pop()[0] as usize;
                    if n >= self.main.len() {
                        return Err("Roll underflow".to_string());
                    }
                    let item = self.main.remove(self.main.len() - 1 - n);
                    self.push(item);
                }
                OP_DROP => {
                    let _ = self.pop();
                }
                op if op >= OP_1 && op <= OP_16 => {
                    // Push small integers (1 to 16)
                    let value = (op - (OP_1 - 1)) as i64;
                    let bytes = sv::script::stack::encode_num(value).map_err(|e| format!("Failed to encode number: {}", e))?;
                    self.push(bytes);
                }
                op if op <= 75 => {
                    // Direct push of data (length <= 75)
                    if i + op as usize > script.len() {
                        return Err("Push data exceeds script length".to_string());
                    }
                    let data = script[i..i + op as usize].to_vec();
                    self.push(data);
                    i += op as usize;
                }
                OP_PUSHDATA1 => {
                    // Push data with 1-byte length
                    if i + 1 > script.len() {
                        return Err("PUSHDATA1 length byte missing".to_string());
                    }
                    let len = script[i] as usize;
                    i += 1;
                    if i + len > script.len() {
                        return Err("PUSHDATA1 data exceeds script length".to_string());
                    }
                    let data = script[i..i + len].to_vec();
                    self.push(data);
                    i += len;
                }
                OP_PUSHDATA2 => {
                    // Push data with 2-byte length
                    if i + 2 > script.len() {
                        return Err("PUSHDATA2 length bytes missing".to_string());
                    }
                    let len = u16::from_le_bytes([script[i], script[i + 1]]) as usize;
                    i += 2;
                    if i + len > script.len() {
                        return Err("PUSHDATA2 data exceeds script length".to_string());
                    }
                    let data = script[i..i + len].to_vec();
                    self.push(data);
                    i += len;
                }
                OP_PUSHDATA4 => {
                    // Push data with 4-byte length
                    if i + 4 > script.len() {
                        return Err("PUSHDATA4 length bytes missing".to_string());
                    }
                    let len = u32::from_le_bytes([script[i], script[i + 1], script[i + 2], script[i + 3]]) as usize;
                    i += 4;
                    if i + len > script.len() {
                        return Err("PUSHDATA4 data exceeds script length".to_string());
                    }
                    let data = script[i..i + len].to_vec();
                    self.push(data);
                    i += len;
                }
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
                    let bytes = sv::script::stack::encode_num(n as i64).unwrap();
                    expanded.push(bytes.len() as u8);
                    expanded.extend(bytes);
                }
            }
        }
    }
    expanded
}

/// OP_XSWAP_n: Swaps top with nth item.
/// Expands to [<n>, OP_ROLL].
#[macro_export]
macro_rules! xswap {
    ($n:expr) => {
        bsv_script! { $n, OP_ROLL }
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
