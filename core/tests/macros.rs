#[cfg(test)]
mod tests {
    use crate::{xswap, loop_unroll, Stack};
    use proptest::prelude::*;
    use sv::script::op_codes::{OP_DUP, OP_SWAP};

    proptest! {
        #[test]
        fn test_xswap_preservation(n in 2..10i32) {
            let mut stack = Stack::default();
            for i in 0..=n { stack.push(i.to_le_bytes().to_vec()); }
            let script = xswap!(n);
            stack.execute(&script).unwrap();
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
}
