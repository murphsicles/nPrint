use nprint_core::{bsv_script, MacroDef, MacroElem, Stack};
use nprint_verification::{verify_macro, verify_script};
use sv::script::op_codes::{OP_DUP, OP_PICK};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_macro() {
        let def = MacroDef {
            name: "xswap".to_string(),
            param_count: 1,
            template: vec![MacroElem::Param(0), MacroElem::Op(OP_PICK)],
        }; // Simplified
        let inputs = vec![vec![1], vec![2], vec![3]];
        let script = bsv_script! { OP_DUP };
        let mut stack = Stack::default();
        for input in inputs.clone() {
            stack.push(input);
        }
        assert!(verify_macro(&def, &[3], inputs).is_ok());
        assert!(verify_script(&script, stack.main).unwrap());
    }
}
