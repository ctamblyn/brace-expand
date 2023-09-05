//! This example demonstrates how to escape braces and commas.
//!
//! Prints the following:
//!
//! this is {braced}
//! this is [bracketed, nicely]

use brace_expand::*;

fn main() {
    let output = brace_expand("this is {\\{braced\\},[bracketed\\, nicely]}");

    for item in &output {
        println!("{}", item);
    }
}
