use crate::chapter_7::parser::{
    err::{
        ExpectedToken, ParseErr as PositionedParseErr, TokenOrEof as PositionedTokenOrEof,
        UnexpectedTokenOrEofErr as PositionedUnexpectedTokenOrEofErr,
    },
    lexer::{Token, TokenizationErr},
};
use crate::chapter_7::term::named::{self, Term as NamedTerm};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Var),
    Abs(Box<Abs>),
    App(Box<App>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Var {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abs {
    pub param: Var,
    pub body: Term,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub callee: Term,
    pub arg: Term,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErr {
    Tokenization(TokenizationErr),
    UnexpectedTokenOrEof(UnexpectedTokenOrEofErr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnexpectedTokenOrEofErr {
    pub expected: Vec<ExpectedToken>,
    pub actual: TokenOrEof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenOrEof {
    Token(Token),
    Eof,
}

impl From<PositionedParseErr> for ParseErr {
    fn from(p: PositionedParseErr) -> ParseErr {
        match p {
            PositionedParseErr::Tokenization(e) => ParseErr::Tokenization(e),
            PositionedParseErr::UnexpectedTokenOrEof(e) => ParseErr::UnexpectedTokenOrEof(e.into()),
        }
    }
}

impl From<PositionedUnexpectedTokenOrEofErr> for UnexpectedTokenOrEofErr {
    fn from(p: PositionedUnexpectedTokenOrEofErr) -> UnexpectedTokenOrEofErr {
        UnexpectedTokenOrEofErr {
            expected: p.expected,
            actual: p.actual.into(),
        }
    }
}

impl From<PositionedTokenOrEof> for TokenOrEof {
    fn from(p: PositionedTokenOrEof) -> TokenOrEof {
        match p {
            PositionedTokenOrEof::Token(positioned) => TokenOrEof::Token(positioned.token),
            PositionedTokenOrEof::Eof => TokenOrEof::Eof,
        }
    }
}

pub fn var(name: &str) -> Term {
    Term::Var(Var {
        name: name.to_string(),
    })
}

pub fn abs(param: &str, body: Term) -> Term {
    Term::Abs(Box::new(Abs {
        param: Var {
            name: param.to_string(),
        },
        body,
    }))
}

pub fn app(callee: Term, arg: Term) -> Term {
    Term::App(Box::new(App { callee, arg }))
}

impl From<NamedTerm> for Term {
    fn from(n: NamedTerm) -> Self {
        match n {
            NamedTerm::Var(var) => Term::Var(var.into()),
            NamedTerm::Abs(abs) => Term::Abs(Box::new(Abs::from(*abs))),
            NamedTerm::App(app) => Term::App(Box::new(App::from(*app))),
        }
    }
}

impl From<named::Var> for Var {
    fn from(n: named::Var) -> Self {
        Var { name: n.name }
    }
}

impl From<named::Abs> for Abs {
    fn from(n: named::Abs) -> Self {
        Abs {
            param: n.param.into(),
            body: n.body.into(),
        }
    }
}

impl From<named::App> for App {
    fn from(n: named::App) -> Self {
        App {
            callee: n.callee.into(),
            arg: n.arg.into(),
        }
    }
}

pub trait IntoUnpositioned<T> {
    fn into_unpositioned(self) -> T;
}

impl IntoUnpositioned<Term> for NamedTerm {
    fn into_unpositioned(self) -> Term {
        self.into()
    }
}

impl IntoUnpositioned<ParseErr> for PositionedParseErr {
    fn into_unpositioned(self) -> ParseErr {
        self.into()
    }
}

impl<T, E, T2, E2> IntoUnpositioned<Result<T2, E2>> for Result<T, E>
where
    T: IntoUnpositioned<T2>,
    E: IntoUnpositioned<E2>,
{
    fn into_unpositioned(self) -> Result<T2, E2> {
        match self {
            Ok(t) => Ok(t.into_unpositioned()),
            Err(e) => Err(e.into_unpositioned()),
        }
    }
}
