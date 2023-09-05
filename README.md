# Brace expansion crate for Rust

![Test results](https://github.com/ctamblyn/brace-expand/actions/workflows/rust.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/brace-expand)](https://crates.io/crates/brace-expand)
[![Documentation](https://docs.rs/brace-expand/badge.svg)](https://docs.rs/brace-expand)

This library performs brace expansion of strings, similar in spirit (though
different in several details) to that used in shells like Bash etc.

## What the algorithm does

Given the input:

```text
{hello,goodbye,wonderful} world
```

this algorithm produces the following collection of strings:

```text
hello world
goodbye world
wonderful world
```

Note that unlike shell brace expansion, the result is a collection of separate
strings rather than a single string.  Also, whitespace characters are not
treated specially by the algorithm; they are on the same footing as printing
characters.

Curly braces `{` and `}` are used to mark the start and end of an expansion
list, and commas separate the items in each list.  Literal curly braces and
commas must be escaped with single backslashes:

```text
this is {\{braced\},[bracketed\, nicely]}
```

produces:

```text
this is {braced}
this is [bracketed, nicely]
```

Note that in Rust source we must escape the backslashes within ordinary string
literals an additional time, giving the following:

```rust
let output = brace_expand("this is {\\{braced\\},[bracketed\\, nicely]}");

assert_eq!(output, vec!["this is {braced}", "this is [bracketed, nicely]"]);
```

If you want a literal backslash, that must be escaped by writing it as a double
backslash:

```text
this is a backslash: \\
```

produces:

```text
this is a backslash: \
```

In Rust source, this would look as follows:

```rust
let output = brace_expand("this is a backslash: \\\\");

println!("{}", output[0]);
```

which produces the following output:

```text
this is a backslash: \
```

# Example

The following code snippet illustrates how inputs can contain multiple lists,
and even be nested:

```rust
let output = brace_expand("{hello,goodbye} {world,my {friends,colleagues}}");

assert_eq!(
    output,
    vec![
        "hello world",
        "hello my friends",
        "hello my colleagues",
        "goodbye world",
        "goodbye my friends",
        "goodbye my colleagues",
    ]
);
```

## Minimum supported Rust version (MSRV) policy

`brace-expand`'s current minimum supported Rust version (MSRV) is **1.31.1**.

`brace-expand` is guaranteed to compile with that version.  It might also
compile with older versions, but that could change in a future patch release.

If the MSRV of `brace-expand` changes, that will be done in a _minor_ version
release (e.g. 1.0.x -> 1.1.0).
