mod lexer;

pub mod err {
    pub use super::lexer::TokenizationErr;

    use super::lexer::PositionedToken;

    pub enum ParseErr {
        TokenizationErr(TokenizationErr),
        UnexpectedTokenErr(UnexpectedTokenErr),
        UnexpectedEndOfInputErr(UnexpectedEndOfInputError),
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct UnexpectedTokenErr {
        pub expected: &'static [ExpectedToken],
        pub actual: PositionedToken,
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

    #[derive(Clone, Debug)]
    pub struct UnexpectedEndOfInputError {
        pub expected: &'static [ExpectedToken],
    }
}

use super::term::named;
use crate::file_position::FilePositionRange;
use err::*;
use lexer::{tokenize, PositionedToken, Token};
use named::Term as NamedTerm;

pub fn parse(src: &str) -> Result<NamedTerm, ParseErr> {
    match tokenize(src) {
        Err(err) => Err(ParseErr::TokenizationErr(err)),
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

    pub fn parse(self) -> Result<NamedTerm, ParseErr> {
        let term = self.consume_term();
        if self.is_exhausted() {
            term
        } else {
            Err(ParseErr::UnexpectedTokenErr(UnexpectedTokenErr {
                expected: &[ExpectedToken::Eof],
                actual: self.tokens[0],
            }))
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
            Err(self.expected_tokens_err(&[
                ExpectedToken::Lambda,
                ExpectedToken::LParen,
                ExpectedToken::Ident,
            ]))
        }
    }

    fn consume_opt_abs(&mut self) -> Option<Result<NamedTerm, ParseErr>> {
        if let Some(lambda) = self.consume_token_if(Token::is_lambda) {
            Some(self.consume_var().and_then(|param| {
                self.consume_token(ExpectedToken::Dot).and_then(|_| {
                    let body = self.consume_term();
                    constructors::build_abs(lambda, param, body)
                })
            }))
        } else {
            None
        }
    }

    fn consume_var(&mut self) -> Result<named::Var, ParseErr> {
        if self.is_exhausted() {
            Err(self.expected_tokens_err(&[ExpectedToken::Ident]))
        } else if let PositionedToken::Var {} = self.tokens[0] {
        } else {
            Err(self.expected_tokens_err(&[ExpectedToken::Ident]))
        }
    }

    fn consume_opt_callable(&mut self) -> Option<Result<NamedTerm, ParseErr>> {}

    fn consume_token(&mut self, expected: ExpectedToken) -> Result<PositionedToken, ParseErr> {
        let pred = expected.matcher();
        if let Some(token) = self.consume_token_if(pred) {
            Ok(token)
        } else {
            Err(self.expected_tokens_err(&[expected]))
        }
    }

    fn consume_token_if<F>(&mut self, pred: F) -> Option<PositionedToken>
    where
        F: FnOnce(&Token) -> bool,
    {
        if self.is_exhausted() {
            None
        } else {
            let ptoken = self.tokens[0];
            if pred(&ptoken.token()) {
                Some(ptoken)
            } else {
                None
            }
        }
    }

    fn is_exhausted(&self) -> bool {
        self.tokens.is_empty()
    }

    fn expected_tokens_err(&self, expected: &'static [ExpectedToken]) -> ParseErr {
        if self.is_exhausted() {
            ParseErr::UnexpectedEndOfInputErr(UnexpectedEndOfInputError { expected })
        } else {
            ParseErr::UnexpectedTokenErr(UnexpectedTokenErr {
                expected,
                actual: self.tokens[0],
            })
        }
    }
}

mod constructors {
    use super::*;

    pub fn build_abs(lambda: NamedTerm, param: named::Var, body: NamedTerm) -> NamedTerm {
        NamedTerm::Abs(Box::new(named::Abs {
            param,
            body,
            position: FilePositionRange {
                start: lambda.position().start,
                end: body.position().end,
            },
        }))
    }

    pub fn build_app(callee: NamedTerm, argument: NamedTerm) -> NamedTerm {
        NamedTerm::App(Box::new(named::App {
            callee,
            argument,
            position: FilePositionRange {
                start: callee.position().start,
                end: argument.position().end,
            },
        }))
    }
}
