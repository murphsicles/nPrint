#[test]
fn test_verify_macro() {
    let def = MacroDef { name: "xswap".to_string(), param_count: 1, template: vec![MacroElem::Param(0), MacroElem::Op(OP_PICK)] };  // Simplified
    assert!(verify_macro(&def, &[3]).is_ok());
}

#[test]
fn test_verify_script() {
    let script = bsv_script! { OP_DUP };
    let inputs = vec![vec![1]];
    assert!(verify_script(&script, inputs).unwrap());
}
