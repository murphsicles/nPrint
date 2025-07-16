#[cfg(test)]
mod tests {
    use nprint_core::{xswap, loop_unroll, Stack, bsv_script};
    use proptest::prelude::*;
    use sv::script::op_codes::{OP_DUP, OP_SWAP, OP_ROLL};

    proptest! {
        #[test]
        fn test_xswap_preservation(n in 2..10i32) {
            let mut stack = Stack::default();
            for i in 0..=n { stack.push(i.to_le_bytes().to_vec()); }
            let script = xswap!(n);
            prop_assert!(stack.execute(&script).is_ok(), "Execution failed for n={} with script {:?}", n, script);
            prop_assert_eq!(stack.main.len() as i32, n + 1);
        }

        #[test]
        fn test_loop_unroll(count in 1..5usize) {
            let script = loop_unroll!(count, { OP_DUP, OP_SWAP });
            let mut stack = Stack::default();
            stack.push(vec![1]);
            stack.execute(&script).unwrap();
            prop_assert_eq!(stack.main.len(), count + 1);  // DUP adds one per iteration
        }
    }

    #[test]
    fn test_xswap_n2() {
        let mut stack = Stack::default();
        stack.push(vec![0]);
        stack.push(vec![1]);
        stack.push(vec![2]);
        let script = xswap!(2); // Should expand to [2, OP_ROLL]
        println!("Script for xswap!(2): {:?}", script);
        let result = stack.execute(&script);
        println!("Stack after execution: {:?}", stack.main);
        assert!(result.is_ok(), "Execution failed: {:?}", result);
        assert_eq!(stack.main.len(), 3);
        assert_eq!(stack.main, vec![vec![0], vec![2], vec![1]]);
    }
}
