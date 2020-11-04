mod lexer;

pub mod err {
    pub use super::lexer::TokenizationErr;

    use super::lexer::{PositionedToken, Token};

    #[derive(Clone, Debug)]
    pub enum ParseErr {
        Tokenization(TokenizationErr),
        UnexpectedTokenOrEof(UnexpectedTokenOrEofErr),
    }

    #[derive(Clone, Debug)]
    pub struct UnexpectedTokenOrEofErr {
        pub expected: Vec<ExpectedToken>,
        pub actual: TokenOrEof,
    }

    #[derive(Clone, Debug)]
    pub enum TokenOrEof {
        Token(PositionedToken),
        Eof,
    }

    impl TokenOrEof {
        pub fn token(self) -> Option<PositionedToken> {
            match self {
                TokenOrEof::Token(t) => Some(t),
                TokenOrEof::Eof => None,
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum ExpectedToken {
        Eof,
        Ident,
        Lambda,
        Dot,
        LParen,
        RParen,
    }

    impl ExpectedToken {
        pub(super) fn matcher(&self) -> fn(&Token) -> bool {
            fn return_false(_: &Token) -> bool {
                false
            }

            fn match_ident(t: &Token) -> bool {
                if let Token::Ident(_) = t {
                    true
                } else {
                    false
                }
            }

            fn match_lambda(t: &Token) -> bool {
                *t == Token::Lambda
            }

            fn match_dot(t: &Token) -> bool {
                *t == Token::Dot
            }

            fn match_lparen(t: &Token) -> bool {
                *t == Token::LParen
            }

            fn match_rparen(t: &Token) -> bool {
                *t == Token::RParen
            }

            match self {
                ExpectedToken::Eof => return_false,
                ExpectedToken::Ident => match_ident,
                ExpectedToken::Lambda => match_lambda,
                ExpectedToken::Dot => match_dot,
                ExpectedToken::LParen => match_lparen,
                ExpectedToken::RParen => match_rparen,
            }
        }
    }
}

use super::term::named;
use crate::file_position::FilePositionRange;
use err::*;
use lexer::{tokenize, PositionedToken, Token};
use named::Term as NamedTerm;

pub fn parse(src: &str) -> Result<NamedTerm, ParseErr> {
    match tokenize(src) {
        Err(err) => Err(ParseErr::Tokenization(err)),
        Ok(tokens) => TokenParser::from_tokens(&tokens).parse(),
    }
}

pub struct TokenParser<'a> {
    tokens: &'a [PositionedToken],
}

impl<'a> TokenParser<'a> {
    pub fn from_tokens(tokens: &[PositionedToken]) -> TokenParser {
        TokenParser { tokens }
    }

    pub fn parse(mut self) -> Result<NamedTerm, ParseErr> {
        match self.consume_term() {
            Ok(term) => {
                if self.is_exhausted() {
                    Ok(term)
                } else {
                    Err(self.expected_tokens_err(vec![ExpectedToken::Eof]))
                }
            }
            Err(e) => Err(e),
        }
    }

    fn consume_term(&mut self) -> Result<NamedTerm, ParseErr> {
        if let Some(abs_res) = self.consume_opt_abs() {
            abs_res
        } else if let Some(callable_res) = self.consume_opt_callable() {
            callable_res.and_then(|callable| {
                if let Some(abs_res) = self.consume_opt_abs() {
                    abs_res.map(|abs| constructors::build_app(callable, abs))
                } else {
                    Ok(callable)
                }
            })
        } else {
            Err(self.expected_tokens_err(vec![
                ExpectedToken::Lambda,
                ExpectedToken::LParen,
                ExpectedToken::Ident,
            ]))
        }
    }

    fn consume_opt_abs(&mut self) -> Option<Result<NamedTerm, ParseErr>> {
        if let Some(lambda) = self.consume_opt_token(ExpectedToken::Lambda) {
            Some(match self.consume_var() {
                Ok(param) => match self.consume_token(ExpectedToken::Dot) {
                    Ok(_) => match self.consume_term() {
                        Ok(body) => Ok(constructors::build_abs(&lambda, param, body)),
                        Err(err) => Err(err),
                    },
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            })
        } else {
            None
        }
    }

    fn consume_var(&mut self) -> Result<named::Var, ParseErr> {
        if let Some(var) = self.consume_opt_var() {
            Ok(var)
        } else {
            Err(self.expected_tokens_err(vec![ExpectedToken::Ident]))
        }
    }

    fn consume_opt_var(&mut self) -> Option<named::Var> {
        if self.is_exhausted() {
            None
        } else if let Token::Ident(name) = self.tokens[0].token() {
            let var = Some(named::Var {
                position: self.tokens[0].position(),
                name: name.clone(),
            });

            self.tokens = &self.tokens[1..];

            var
        } else {
            None
        }
    }

    fn consume_opt_callable(&mut self) -> Option<Result<NamedTerm, ParseErr>> {
        if let Some(var) = self.consume_opt_var() {
            Some(Ok(NamedTerm::Var(var)))
        } else if let Some(paren_res) = self.consume_opt_paren_exp() {
            Some(paren_res)
        } else {
            None
        }
    }

    fn consume_opt_paren_exp(&mut self) -> Option<Result<NamedTerm, ParseErr>> {
        self.consume_opt_token(ExpectedToken::LParen).map(|_| {
            self.consume_term()
                .and_then(|inner| self.consume_token(ExpectedToken::RParen).map(|_| inner))
        })
    }

    fn consume_token(&mut self, expected: ExpectedToken) -> Result<PositionedToken, ParseErr> {
        if let Some(token) = self.consume_opt_token(expected) {
            Ok(token)
        } else {
            Err(self.expected_tokens_err(vec![expected]))
        }
    }

    fn consume_opt_token(&mut self, expected: ExpectedToken) -> Option<PositionedToken> {
        if self.is_exhausted() {
            None
        } else {
            let ptoken = &self.tokens[0];
            if expected.matcher()(&ptoken.token()) {
                self.tokens = &self.tokens[1..];
                Some(ptoken.clone())
            } else {
                None
            }
        }
    }

    fn is_exhausted(&self) -> bool {
        self.tokens.is_empty()
    }

    fn expected_tokens_err(&self, expected: Vec<ExpectedToken>) -> ParseErr {
        ParseErr::UnexpectedTokenOrEof(UnexpectedTokenOrEofErr {
            expected,
            actual: if self.is_exhausted() {
                TokenOrEof::Eof
            } else {
                TokenOrEof::Token(self.tokens[0].clone())
            },
        })
    }
}

mod constructors {
    use super::*;

    pub fn build_abs(lambda: &PositionedToken, param: named::Var, body: NamedTerm) -> NamedTerm {
        let end = body.position().end;
        NamedTerm::Abs(Box::new(named::Abs {
            param,
            body,
            position: FilePositionRange {
                start: lambda.position().start,
                end,
            },
        }))
    }

    pub fn build_app(callee: NamedTerm, argument: NamedTerm) -> NamedTerm {
        let start = callee.position().start;
        let end = argument.position().end;
        NamedTerm::App(Box::new(named::App {
            callee,
            argument,
            position: FilePositionRange { start, end },
        }))
    }
}
