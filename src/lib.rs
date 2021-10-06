//! This library performs brace expansion of strings, as in shells like Bash etc.
//!
//! Given the input:
//!
//! ```text
//! {hello,goodbye,wonderful} world
//! ```
//!
//! this algorithm produces the following collection of strings:
//!
//! ```text
//! hello world
//! goodbye world
//! wonderful world
//! ```
//!
//! Note that unlike shell brace expansion, the result is a collection of separate
//! strings rather than a single string.  Also, whitespace characters are not
//! treated specially by the algorithm; they are on the same footing as printing
//! characers.
//!
//! Curly braces `{` and `}` are used to mark the start and end of an expansion
//! list, and commas separate the items in each list.  Literal curly braces and
//! commas must be escaped with single backslashes:
//!
//! ```text
//! this is {\{braced\},[bracketed\, nicely]}
//! ```
//!
//! produces:
//!
//! ```text
//! this is {braced}
//! this is [bracketed, nicely]
//! ```
//!
//! If you want a literal backslash, that too must be escaped:
//!
//! ```text
//! this is a backslash: \\
//! ```
//!
//! produces:
//!
//! ```text
//! this is a backslash: \
//! ```
//!
//! Note that the escaping backslashes are removed from the output.
//!
//! Inputs can contain multiple expansion lists, and these can be nested.  For
//! example:
//!
//! ```text
//! {hello,goodbye} {world,my {friends,colleagues}}
//! ```
//!
//! produces:
//!
//! ```text
//! hello world
//! goodbye world
//! hello my friends
//! hello my colleagues
//! goodbye my friends
//! goodbye my colleagues
//! ```
//!
//! # Example
//!
//! ```rust
//! # fn main() {
//! # use brace_expand::brace_expand;
//! let output = brace_expand("this {is,is not} a pipe");
//!
//! assert_eq!(output, vec!["this is a pipe", "this is not a pipe"]);
//! # }
//! ```

#![doc(html_root_url = "https://docs.rs/brace-expand/0.1.0")]

#[cfg(test)]
mod tests;

use std::str::Chars;

//------------------------------------------------------------------------------

// Iterator which converts a stream of characters into a stream of tokens.

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    OpenBrace,
    CloseBrace,
    Comma,
    Char(char),
}

struct TokenIter<'a> {
    stream: Chars<'a>,
}

impl<'a> TokenIter<'a> {
    fn new(buffer: &'a str) -> Self {
        Self {
            stream: buffer.chars(),
        }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream.next().and_then(|ch| match ch {
            '\\' => self.stream.next().map(Token::Char),
            '{' => Some(Token::OpenBrace),
            '}' => Some(Token::CloseBrace),
            ',' => Some(Token::Comma),
            _ => Some(Token::Char(ch)),
        })
    }
}

//------------------------------------------------------------------------------

fn convert_to_string(tokens: &[Token]) -> String {
    tokens
        .iter()
        .filter_map(|token| match token {
            Token::Char(ch) => Some(ch),
            _ => None,
        })
        .collect()
}

//------------------------------------------------------------------------------

enum Expansion {
    Partial(Vec<Vec<Token>>),
    Complete(String),
}

fn expand_one_level(to_expand: Vec<Token>) -> Expansion {
    let mut level = 0;
    let mut list_start_pos = 0;
    let mut list_end_pos = 0;
    let mut term_start_pos = 0;
    let mut terms = Vec::new();

    for (pos, token) in to_expand.iter().enumerate() {
        match token {
            Token::OpenBrace => {
                level += 1;
                if level == 1 {
                    list_start_pos = pos;
                    term_start_pos = pos + 1;
                }
            }

            Token::CloseBrace => {
                level -= 1;
                if level == 0 {
                    list_end_pos = pos + 1;
                    terms.push(&to_expand[term_start_pos..pos]);
                    break; // This is the last component.
                }
            }

            Token::Comma => {
                if level == 1 {
                    terms.push(&to_expand[term_start_pos..pos]);
                    term_start_pos = pos + 1;
                }
            }

            _ => (),
        }
    }

    if !terms.is_empty() {
        let prefix = &to_expand[..list_start_pos];
        let suffix = &to_expand[list_end_pos..];

        let results: Vec<Vec<Token>> = terms
            .iter()
            .map(|term| [prefix, term, suffix].concat())
            .collect();

        Expansion::Partial(results)
    } else {
        Expansion::Complete(convert_to_string(&to_expand))
    }
}

//------------------------------------------------------------------------------

/// Expand braces and return the set of results.
///
/// # Example
///
/// ```rust
/// # fn main() {
/// # use brace_expand::brace_expand;
/// let output = brace_expand("{hello,goodbye} {world,my {friends,colleagues}}");
///
/// assert_eq!(
///     output,
///     vec![
///         "hello world",
///         "hello my friends",
///         "hello my colleagues",
///         "goodbye world",
///         "goodbye my friends",
///         "goodbye my colleagues",
///     ]
/// );
/// # }
/// ```
pub fn brace_expand(input: &str) -> Vec<String> {
    let mut work_queue: Vec<Vec<_>> = vec![TokenIter::new(input).collect()];
    let mut results = Vec::new();

    while let Some(to_expand) = work_queue.pop() {
        match expand_one_level(to_expand) {
            Expansion::Partial(mut new_work) => work_queue.append(&mut new_work),
            Expansion::Complete(fully_expanded) => results.push(fully_expanded),
        }
    }

    // Reverse to get correct ordering of the expansions (Bash-like).
    results.reverse();

    results
}
