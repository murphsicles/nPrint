use nprint_core::bsv_script;
use nprint_core::{OP_2, OP_3, OP_ADD, OP_5, OP_EQUAL};

fn main() {
    let script = bsv_script! { OP_2, OP_3, OP_ADD, OP_5, OP_EQUAL };
    println!("{:?}", script);
}
