use crate::file_position::{FilePosition, FilePositionRange};
use matchers::{Match, MATCHERS};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Lambda,
    Dot,
    LParen,
    RParen,
}

impl Token {
    pub fn is_ident(&self) -> bool {
        if let Token::Ident(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_lambda(&self) -> bool {
        *self == Token::Lambda
    }

    pub fn is_dot(&self) -> bool {
        *self == Token::Dot
    }

    pub fn is_l_paren(&self) -> bool {
        *self == Token::LParen
    }

    pub fn is_r_paren(&self) -> bool {
        *self == Token::RParen
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositionedToken {
    token: Token,
    position: FilePositionRange,
}

impl PositionedToken {
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn position(&self) -> FilePositionRange {
        self.position
    }
}

mod matchers {
    use super::*;

    pub static MATCHERS: [fn(&str) -> Option<Match>; 5] = [
        match_ident,
        match_lambda,
        match_dot,
        match_lparen,
        match_rparen,
    ];

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Match {
        token: Token,
        len: usize,
    }

    impl Match {
        pub fn token(&self) -> &Token {
            &self.token
        }

        pub fn len(&self) -> usize {
            self.len
        }
    }

    fn match_ident(s: &str) -> Option<Match> {
        let mut name = "".to_string();
        for c in s.chars() {
            let valid = c.is_ascii_alphabetic()
                || c == '_'
                || (!name.is_empty() && (c == '\'' || c.is_digit(10)));
            if valid {
                name.push(c)
            } else {
                break;
            }
        }
        if name.is_empty() {
            None
        } else {
            let len = name.len();
            Some(Match {
                token: Token::Ident(name),
                len,
            })
        }
    }

    fn match_lambda(s: &str) -> Option<Match> {
        if s.starts_with("\\") {
            Some(Match {
                token: Token::Lambda,
                len: 1,
            })
        } else {
            None
        }
    }

    fn match_dot(s: &str) -> Option<Match> {
        if s.starts_with(".") {
            Some(Match {
                token: Token::Dot,
                len: 1,
            })
        } else {
            None
        }
    }

    fn match_lparen(s: &str) -> Option<Match> {
        if s.starts_with("(") {
            Some(Match {
                token: Token::LParen,
                len: 1,
            })
        } else {
            None
        }
    }

    fn match_rparen(s: &str) -> Option<Match> {
        if s.starts_with(")") {
            Some(Match {
                token: Token::RParen,
                len: 1,
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenizationErr(String);

impl TokenizationErr {
    pub fn from_string(s: String) -> TokenizationErr {
        TokenizationErr(s)
    }

    pub fn str(&self) -> &str {
        &self.0
    }
}

pub fn tokenize(mut src: &str) -> Result<Vec<PositionedToken>, TokenizationErr> {
    let mut out = Vec::new();
    let mut tracker = PositionTracker::new("");

    while !src.is_empty() {
        if src.starts_with(char::is_whitespace) {
            tracker.update(&src[0..1]);
            src = &src[1..];
        } else if let Some(match_) = get_leading_token_match(&src) {
            let start = tracker.current_position();
            tracker.update(&src[0..match_.len()]);
            let end = tracker.current_position();

            out.push(PositionedToken {
                token: match_.token().clone(),
                position: FilePositionRange { start, end },
            });
            src = &src[match_.len()..];
        } else {
            return Err(TokenizationErr::from_string(src.to_string()));
        }
    }

    Ok(out)
}

fn get_leading_token_match(src: &str) -> Option<Match> {
    for matcher in &MATCHERS {
        if let Some(m) = matcher(src) {
            return Some(m);
        }
    }

    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PositionTracker {
    position: FilePosition,
}

impl PositionTracker {
    fn new(s: &str) -> PositionTracker {
        let mut tracker = PositionTracker {
            position: FilePosition {
                index: 0,
                line: 1,
                column: 0,
            },
        };
        tracker.update(s);
        tracker
    }

    fn update(&mut self, s: &str) {
        let PositionTracker { position } = self;

        for c in s.chars() {
            position.index += 1;
            if c == '\n' {
                position.line += 1;
                position.column = 0;
            } else {
                position.column += 1;
            }
        }
    }

    fn current_position(&self) -> FilePosition {
        self.position
    }
}
