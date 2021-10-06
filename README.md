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
characers.

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

If you want a literal backslash, that too must be escaped:

```text
this is a backslash: \\
```

produces:

```text
this is a backslash: \
```

Note that the escaping backslashes are removed from the output.

Inputs can contain multiple expansion lists, and these can be nested.  For
example:

```text
{hello,goodbye} {world,my {friends,colleagues}}
```

produces:

```text
hello world
goodbye world
hello my friends
hello my colleagues
goodbye my friends
goodbye my colleagues
```

## Example

```rust
use brace_expand::brace_expand;

fn main() {
    let output = brace_expand("this {is,is not} a pipe"); 

    assert_eq!(output, vec!["this is a pipe", "this is not a pipe"]);
}
```
