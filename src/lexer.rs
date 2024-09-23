use std::iter::{self, from_fn};

#[derive(Debug, Clone)]
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

const KEYWORDS: [(Token, &str); 3] = [
    (Token::IntKeyword, "int"),
    (Token::ReturnKeyWord, "return"),
    (Token::VoidKeyWord, "void"),
];

pub fn tokenizer(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            ch if ch.is_whitespace() => continue,
            '(' => tokens.push(Token::OpenParenthesis),
            ')' => tokens.push(Token::CloseParenthesis),
            '{' => tokens.push(Token::OpenBrace),
            '}' => tokens.push(Token::CloseBrace),
            ';' => tokens.push(Token::Semicolon),
            '0'..='9' => {
                let n: i32 = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())))
                    .collect::<String>()
                    .parse()
                    .unwrap();

                tokens.push(Token::Constant(n));
            }
            '/' => {
                if iter.next_if(|s| s.eq(&'/')).is_some() {
                    // Single line comment (//)
                    iter.by_ref().find(|&c| c == '\n'); // Skip until end of the line
                } else if iter.next_if(|s| s.eq(&'*')).is_some() {
                    // Multiline comment (/* */)
                    while let Some(ch) = iter.next() {
                        if ch == '*' {
                            if iter.next_if(|s| s.eq(&'/')).is_some() {
                                break; // End of the multiline comment
                            }
                        }
                    }
                } else {
                    // If it's not part of a comment, you can handle it as an error or another token
                    panic!("Unexpected '/' character");
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
                println!("Identifier: {}", &n);
                let token = KEYWORDS
                    .iter()
                    .find(|(_, s)| s.eq(&n))
                    .map_or(Token::Identifier(n), |(t, _)| t.clone());
                tokens.push(token);
            }
            ch => {
                panic!("unrecognized char: {}", ch);
            }
        }
    }

    tokens
}
