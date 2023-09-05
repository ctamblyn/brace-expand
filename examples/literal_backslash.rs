//! This example demonstrates how to include a literal backslash in the output.
//!
//! Prints the following:
//!
//! this is a backslash: \

use brace_expand::*;

fn main() {
    let output = brace_expand("this is a backslash: \\\\");

    println!("{}", output[0]);
}
