use miette::{Diagnostic, NamedSource, SourceOffset, SourceSpan};
use std::{
    fmt::Display,
    iter::{self, from_fn},
    path::PathBuf,
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
    Tilde,
    Hyphen,
    Decrement,
    Plus,
    Asteriks,
    Slash,
    PercentSign,
    Not,
    And,
    Or,
    EqualTo,
    NotEqualTo,
    LessThan,
    GreaterThan,
    LessThanEqualTo,
    GreaterThanEqualTo,
}

impl Token {
    fn text_length(&self) -> usize {
        match self {
            Token::Identifier(s) => s.len(),
            Token::Constant(i) => i.to_string().len(),
            Token::IntKeyword => 3,
            Token::VoidKeyWord => 4,
            Token::ReturnKeyWord => 6,
            Token::Decrement
            | Token::And
            | Token::Or
            | Token::NotEqualTo
            | Self::LessThanEqualTo
            | Token::GreaterThanEqualTo => 2,
            _ => 1,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(id) => f.write_str(id),
            Token::Constant(i) => f.write_str(&i.to_string()),
            Token::IntKeyword => f.write_str("int"),
            Token::VoidKeyWord => f.write_str("void"),
            Token::ReturnKeyWord => f.write_str("return"),
            Token::OpenParenthesis => f.write_str("("),
            Token::CloseParenthesis => f.write_str(")"),
            Token::OpenBrace => f.write_str("{"),
            Token::CloseBrace => f.write_str("}"),
            Token::Semicolon => f.write_str(";"),
            Token::Hyphen => f.write_str("-"),
            Token::Decrement => f.write_str("--"),
            Token::Tilde => f.write_str("~"),
            Token::Plus => f.write_str("+"),
            Token::Asteriks => f.write_str("*"),
            Token::Slash => f.write_str("/"),
            Token::PercentSign => f.write_str("%"),
            Token::And => f.write_str("&&"),
            Token::Not => f.write_str("!"),
            Token::Or => f.write_str("||"),
            Token::EqualTo => f.write_str("=="),
            Token::NotEqualTo => f.write_str("!="),
            Token::LessThan => f.write_str("<"),
            Token::GreaterThan => f.write_str(">"),
            Token::GreaterThanEqualTo => f.write_str(">="),
            Token::LessThanEqualTo => f.write_str("<="),
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
    pub src: NamedSource<String>,
    #[label = "{error}"]
    pub span: SourceSpan,
    pub error: LexerErrorType,
}

#[derive(Debug, Clone, Copy, Error)]
pub enum LexerErrorType {
    #[error("Invalid digit in decimal constant")]
    InvalidCharInDigitalConstant,
    #[error("Unrecognized char")]
    UnexpectedChar,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileToken {
    pub token: Token,
    pub line: usize,
    pub start_char_in_line: usize,
}

impl FileToken {
    pub fn source_span(&self, source: impl AsRef<str>) -> SourceSpan {
        SourceSpan::new(
            SourceOffset::from_location(source, self.line, self.start_char_in_line),
            self.token.text_length(),
        )
    }
}

pub struct Lexer {
    pub path: PathBuf,
    pub content: String,
    pub tokens: Vec<FileToken>,
    line_nr: usize,
    nr_in_line: usize,
}

impl Lexer {
    pub fn from_content(content: String) -> Self {
        Self {
            path: "main.c".into(),
            content,
            tokens: Vec::new(),
            line_nr: 1,
            nr_in_line: 0,
        }
    }
    pub fn from_path(path: PathBuf) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path.clone())?;
        Ok(Self {
            path,
            content,
            tokens: Vec::new(),
            line_nr: 1,
            nr_in_line: 0,
        })
    }
    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(FileToken {
            token,
            line: self.line_nr,
            start_char_in_line: self.nr_in_line,
        })
    }

    pub fn tokenize(&mut self) -> Result<Vec<FileToken>, LexerError> {
        let mut errors = Vec::<LexerError>::new();
        let content = self.content.clone();
        let mut iter = content.chars().peekable();
        self.line_nr = 1;
        self.nr_in_line = 0;

        while let Some(ch) = iter.next() {
            self.nr_in_line += 1;
            if ch.eq(&'\n') {
                self.nr_in_line = 0;
                self.line_nr += 1;
            }
            match ch {
                ch if ch.is_whitespace() => continue,
                '(' => self.add_token(Token::OpenParenthesis),
                ')' => self.add_token(Token::CloseParenthesis),
                '{' => self.add_token(Token::OpenBrace),
                '}' => self.add_token(Token::CloseBrace),
                ';' => self.add_token(Token::Semicolon),
                '~' => self.add_token(Token::Tilde),
                '+' => self.add_token(Token::Plus),
                '*' => self.add_token(Token::Asteriks),
                '%' => self.add_token(Token::PercentSign),
                '=' if iter.next_if_eq(&'=').is_some() => {
                    self.add_token(Token::EqualTo);
                    self.nr_in_line += 1;
                }
                '&' if iter.next_if_eq(&'&').is_some() => {
                    self.add_token(Token::And);
                    self.nr_in_line += 1;
                }
                '|' if iter.next_if_eq(&'|').is_some() => {
                    self.add_token(Token::Or);
                    self.nr_in_line += 1;
                }
                '-' => {
                    if iter.next_if_eq(&'-').is_some() {
                        self.add_token(Token::Decrement);
                        self.nr_in_line += 1;
                    } else {
                        self.add_token(Token::Hyphen);
                    }
                }
                '>' => {
                    if iter.next_if_eq(&'=').is_some() {
                        self.add_token(Token::GreaterThanEqualTo);
                        self.nr_in_line += 1;
                    } else {
                        self.add_token(Token::GreaterThan);
                    }
                }
                '<' => {
                    if iter.next_if_eq(&'=').is_some() {
                        self.add_token(Token::LessThanEqualTo);
                        self.nr_in_line += 1;
                    } else {
                        self.add_token(Token::LessThan);
                    }
                }
                '!' => {
                    if iter.next_if_eq(&'=').is_some() {
                        self.add_token(Token::NotEqualTo);
                        self.nr_in_line += 1;
                    } else {
                        self.add_token(Token::Not);
                    }
                }
                '0'..='9' => {
                    let value = iter::once(ch)
                        .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())))
                        .collect::<String>();
                    let n: i32 = value.parse().unwrap();

                    self.add_token(Token::Constant(n));
                    self.nr_in_line += value.len();
                    if let Some(next_ch) = iter.peek() {
                        if next_ch.is_alphabetic() {
                            errors.push(self.error(LexerErrorType::InvalidCharInDigitalConstant));
                            iter.next();
                            self.nr_in_line += 1;
                        }
                    }
                }
                // AT LEAST FOR NOW
                '#' => {
                    iter.by_ref().find(|&c| c == '\n'); // Skip until end of the line
                    self.nr_in_line = 0;
                    self.line_nr += 1;
                }
                '/' => {
                    if iter.next_if(|s| s.eq(&'/')).is_some() {
                        // Single line comment (//)
                        iter.by_ref().find(|&c| c == '\n'); // Skip until end of the line
                        self.nr_in_line = 0;
                        self.line_nr += 1;
                    } else if iter.next_if(|s| s.eq(&'*')).is_some() {
                        self.nr_in_line += 1;
                        // Multiline comment (/* */)
                        while let Some(ch) = iter.next() {
                            self.nr_in_line += 1;
                            if ch.eq(&'\n') {
                                self.nr_in_line = 0;
                                self.line_nr += 1;
                            }
                            if ch == '*' && iter.next_if(|s| s.eq(&'/')).is_some() {
                                self.nr_in_line += 1;
                                break; // End of the multiline comment
                            }
                        }
                    } else {
                        self.add_token(Token::Slash);
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
                    self.tokens.push(FileToken {
                        line: self.line_nr,
                        start_char_in_line: self.nr_in_line,
                        token,
                    });
                    self.nr_in_line += length;
                }
                _ => {
                    errors.push(self.error(LexerErrorType::UnexpectedChar));
                }
            }
        }

        if errors.is_empty() {
            Ok(self.tokens.clone())
        } else {
            Err(errors.first().unwrap().clone())
        }
    }

    pub fn error(&self, error: LexerErrorType) -> LexerError {
        LexerError {
            src: NamedSource::new(self.path.to_str().unwrap(), self.content.clone()),
            error,
            span: self.source_span(),
        }
    }

    pub fn source_span(&self) -> SourceSpan {
        SourceSpan::new(
            SourceOffset::from_location(&self.content, self.line_nr, self.nr_in_line),
            1,
        )
    }
}
