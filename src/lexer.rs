use miette::{Diagnostic, NamedSource, SourceSpan};
use std::{
    fmt::Display,
    iter::{self, from_fn},
};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Constant(i32),
    IntKeyword,
    VoidKeyWord,
    ReturnKeyWord,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(id) => f.write_str(id),
            Token::Constant(i) => f.write_str(&i.to_string()),
            Token::IntKeyword => f.write_str("int"),
            Token::VoidKeyWord => f.write_str("void"),
            Token::ReturnKeyWord => f.write_str("return"),
            Token::OpenParenthesis => f.write_str("{"),
            Token::CloseParenthesis => f.write_str("}"),
            Token::OpenBrace => f.write_str("("),
            Token::CloseBrace => f.write_str(")"),
            Token::Semicolon => f.write_str(";"),
        }
    }
}

const KEYWORDS: [(Token, &str); 3] = [
    (Token::IntKeyword, "int"),
    (Token::ReturnKeyWord, "return"),
    (Token::VoidKeyWord, "void"),
];

#[derive(Error, Debug, Diagnostic, Clone)]
#[error("Failed to parse the code")]
#[diagnostic(code(error::on::base))]
pub struct LexerError {
    #[source_code]
    src: NamedSource<String>,
    #[label = "This is the highlight"]
    span: SourceSpan,
    pub error: LexerErrorType,
}

#[derive(Debug, Clone, Copy)]
pub enum LexerErrorType {
    // #[error("[Line {line_nr}:{nr_in_line}] Invalid digit '{ch}' in decimal constant")]
    InvalidCharInDigitalConstant,
    // #[error("[Line {line_nr}:{nr_in_line}] Unrecognized char '{ch}'")]
    UnexpectedChar,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileToken {
    pub token: Token,
    pub line: usize,
    pub start_char_in_line: usize,
}

pub fn tokenizer(path: std::path::PathBuf) -> Result<Vec<FileToken>, LexerError> {
    let file = std::fs::read_to_string(path.clone()).expect("Could not read the file");
    let mut errors = Vec::<LexerError>::new();
    let mut tokens: Vec<FileToken> = Vec::new();
    let mut iter = file.chars().peekable();
    let mut line_nr = 1;
    let mut nr_in_line = 0;

    while let Some(ch) = iter.next() {
        nr_in_line += 1;
        if ch.eq(&'\n') {
            nr_in_line = 0;
            line_nr += 1;
        }
        match ch {
            ch if ch.is_whitespace() => continue,
            '(' => tokens.push(FileToken {
                token: Token::OpenParenthesis,
                line: line_nr,
                start_char_in_line: nr_in_line,
            }),
            ')' => tokens.push(FileToken {
                token: Token::CloseParenthesis,
                line: line_nr,
                start_char_in_line: nr_in_line,
            }),
            '{' => tokens.push(FileToken {
                token: Token::OpenBrace,
                line: line_nr,
                start_char_in_line: nr_in_line,
            }),
            '}' => tokens.push(FileToken {
                token: Token::CloseBrace,
                line: line_nr,
                start_char_in_line: nr_in_line,
            }),
            ';' => tokens.push(FileToken {
                token: Token::Semicolon,
                line: line_nr,
                start_char_in_line: nr_in_line,
            }),
            '0'..='9' => {
                let value = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())))
                    .collect::<String>();
                let n: i32 = value.parse().unwrap();

                tokens.push(FileToken {
                    line: line_nr,
                    start_char_in_line: nr_in_line,
                    token: Token::Constant(n),
                });
                nr_in_line += value.len();
                if let Some(next_ch) = iter.peek() {
                    if next_ch.is_alphabetic() {
                        errors.push(LexerError {
                            src: NamedSource::new(path.to_str().unwrap(), file.clone()),
                            span: (line_nr, nr_in_line).into(),
                            error: LexerErrorType::InvalidCharInDigitalConstant,
                        });
                        iter.next();
                        nr_in_line += 1;
                    }
                }
            }
            '/' => {
                if iter.next_if(|s| s.eq(&'/')).is_some() {
                    // Single line comment (//)
                    iter.by_ref().find(|&c| c == '\n'); // Skip until end of the line
                    nr_in_line = 0;
                    line_nr += 1;
                } else if iter.next_if(|s| s.eq(&'*')).is_some() {
                    nr_in_line += 1;
                    // Multiline comment (/* */)
                    while let Some(ch) = iter.next() {
                        nr_in_line += 1;
                        if ch.eq(&'\n') {
                            nr_in_line = 0;
                            line_nr += 1;
                        }
                        if ch == '*' && iter.next_if(|s| s.eq(&'/')).is_some() {
                            nr_in_line += 1;
                            break; // End of the multiline comment
                        }
                    }
                } else {
                    errors.push(LexerError {
                        src: NamedSource::new(path.to_str().unwrap(), file.clone()),
                        span: (line_nr, nr_in_line).into(),
                        error: LexerErrorType::UnexpectedChar,
                    });
                }
            }
            ch if ch.is_ascii_alphabetic() => {
                let n: String = iter::once(ch)
                    .chain(from_fn(|| {
                        iter.by_ref().next_if(|s| s.is_ascii_alphanumeric())
                    }))
                    .collect::<String>()
                    .parse()
                    .unwrap();
                let length = n.len();
                let token = KEYWORDS
                    .iter()
                    .find(|(_, s)| s.eq(&n))
                    .map_or(Token::Identifier(n), |(t, _)| t.clone());
                tokens.push(FileToken {
                    line: line_nr,
                    start_char_in_line: nr_in_line,
                    token,
                });
                nr_in_line += length;
            }
            _ => {
                errors.push(LexerError {
                    src: NamedSource::new(path.to_str().unwrap(), file.clone()),
                    span: (line_nr, nr_in_line).into(),
                    error: LexerErrorType::UnexpectedChar,
                });
            }
        }
    }

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors.first().unwrap().clone())
    }
}
