mod lexer;
mod parse_tree;

#[cfg(test)]
mod test;

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
use parse_tree::*;

pub fn parse(src: &str) -> Result<NamedTerm, ParseErr> {
    match tokenize(src) {
        Err(err) => Err(ParseErr::Tokenization(err)),
        Ok(tokens) => TokenParser::from_tokens(&tokens).parse().map(Into::into),
    }
}

pub struct TokenParser<'a> {
    tokens: &'a [PositionedToken],
}

impl<'a> TokenParser<'a> {
    pub fn from_tokens(tokens: &[PositionedToken]) -> TokenParser {
        TokenParser { tokens }
    }

    pub fn parse(mut self) -> Result<Term, ParseErr> {
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

    fn consume_term(&mut self) -> Result<Term, ParseErr> {
        if let Some(abs_res) = self.consume_opt_abs() {
            abs_res.map(Into::into)
        } else if let Some(callable_res) = self.consume_opt_callable() {
            callable_res.and_then(|callable| {
                if let Some(abs_res) = self.consume_opt_abs() {
                    abs_res.map(|abs| (callable, abs).into())
                } else {
                    Ok(callable.into())
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

    fn consume_opt_abs(&mut self) -> Option<Result<Abs, ParseErr>> {
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

    fn consume_opt_callable(&mut self) -> Option<Result<Callable, ParseErr>> {
        if let Some(arg_res) = self.consume_opt_arg() {
            Some(match arg_res {
                Ok(left) => {
                    let mut right = vec![];
                    while let Some(arg_res) = self.consume_opt_arg() {
                        match arg_res {
                            Ok(arg) => right.push(arg),
                            Err(e) => {
                                return Some(Err(e));
                            }
                        }
                    }
                    Ok(Callable { left, right })
                }
                Err(e) => Err(e),
            })
        } else {
            None
        }
    }

    fn consume_opt_arg(&mut self) -> Option<Result<Arg, ParseErr>> {
        if let Some(var) = self.consume_opt_var() {
            Some(Ok(Arg::Var(var)))
        } else if let Some(paren_exp_res) = self.consume_opt_paren_exp() {
            Some(paren_exp_res.map(|inner| Arg::Parenthesized(Box::new(inner))))
        } else {
            None
        }
    }

    fn consume_var(&mut self) -> Result<Var, ParseErr> {
        if let Some(var) = self.consume_opt_var() {
            Ok(var)
        } else {
            Err(self.expected_tokens_err(vec![ExpectedToken::Ident]))
        }
    }

    fn consume_opt_var(&mut self) -> Option<Var> {
        if self.is_exhausted() {
            None
        } else if let Token::Ident(name) = &self.tokens[0].token {
            let var = Some(Var {
                position: self.tokens[0].position,
                name: name.clone(),
            });

            self.tokens = &self.tokens[1..];

            var
        } else {
            None
        }
    }

    fn consume_opt_paren_exp(&mut self) -> Option<Result<Term, ParseErr>> {
        if let Some(_) = self.consume_opt_token(ExpectedToken::LParen) {
            Some(match self.consume_term() {
                Ok(term) => match self.consume_token(ExpectedToken::RParen) {
                    Ok(_) => Ok(term),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            })
        } else {
            None
        }
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
            if expected.matcher()(&ptoken.token) {
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
    use crate::file_position::Position;

    pub fn build_abs(lambda: &PositionedToken, param: Var, body: Term) -> Abs {
        let end = body.position().end;
        Abs {
            param,
            body,
            position: FilePositionRange {
                start: lambda.position.start,
                end,
            },
        }
    }
}
