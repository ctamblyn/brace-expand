use super::*;

#[test]
fn simple_expansion_in_middle_of_string_works() {
    let output = brace_expand("a{b,c}d");

    assert_eq!(output, vec!["abd", "acd"]);
}

#[test]
fn nested_expansion_works() {
    let output = brace_expand("{a,b}c{e,f{g,h}}");

    assert_eq!(output, vec!["ace", "acfg", "acfh", "bce", "bcfg", "bcfh"]);
}

#[test]
fn empty_terms_work() {
    let output = brace_expand("a{,b,,c,}d");

    assert_eq!(output, vec!["ad", "abd", "ad", "acd", "ad"]);
}

#[test]
fn escaping_commas_works() {
    let output = brace_expand("{a\\,,b\\,}c");

    assert_eq!(output, vec!["a,c", "b,c"]);
}

#[test]
fn escaping_braces_works() {
    let output = brace_expand("{\\{a,b\\},c}d");

    assert_eq!(output, vec!["{ad", "b}d", "cd"]);
}

#[test]
fn escaping_backslashes_works() {
    let output = brace_expand("{\\\\{a,b\\\\},c}d");

    assert_eq!(output, vec!["\\ad", "\\b\\d", "cd"]);
}

#[test]
fn trival_expansion_works() {
    let output = brace_expand("a");

    assert_eq!(output, vec!["a"]);
}

#[test]
fn trival_expansion_with_escaped_backslash_works() {
    let output = brace_expand("a\\\\");

    assert_eq!(output, vec!["a\\"]);
}
