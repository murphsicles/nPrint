use super::{verify_macro, VerifyError, verify_script};
use super::{MacroDef, MacroElem};
use nprint_core::bsv_script;
use sv::script::op_codes::OP_PICK;
use sv::script::op_codes::OP_DUP;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_macro() {
        let def = MacroDef { name: "xswap".to_string(), param_count: 1, template: vec![MacroElem::Param(0), MacroElem::Op(OP_PICK)] };  // Simplified
        let inputs = vec![vec![1], vec![2], vec![3]];
        let script = bsv_script! { OP_DUP };
        assert!(verify_macro(&def, &[3]).is_ok());
        assert!(verify_script(&script, inputs).unwrap());
    }
}
