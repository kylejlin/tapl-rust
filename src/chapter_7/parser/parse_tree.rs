use super::super::term::named::Term as NamedTerm;
use crate::file_position::{FilePositionRange, Position};

#[derive(Clone, Debug)]
pub enum Term {
    Abs(Box<Abs>),
    Callable(Box<Callable>),
    CallableAbs(Box<(Callable, Abs)>),
}

#[derive(Clone, Debug)]
pub struct Abs {
    pub param: Var,
    pub body: Term,
    pub position: FilePositionRange,
}

#[derive(Clone, Debug)]
pub struct Var {
    pub name: String,
    pub position: FilePositionRange,
}

#[derive(Clone, Debug)]
pub struct Callable {
    pub left: Arg,
    pub right: Vec<Arg>,
}

#[derive(Clone, Debug)]
pub enum Arg {
    Var(Var),
    Parenthesized(Box<Term>),
}

impl Position for &Term {
    fn position(self) -> FilePositionRange {
        match self {
            Term::Abs(a) => a.position,
            Term::Callable(c) => c.position(),
            Term::CallableAbs(ca) => {
                let (c, a) = **ca;
                FilePositionRange {
                    start: c.position().start,
                    end: a.position.end,
                }
            }
        }
    }
}

impl From<Abs> for Term {
    fn from(abs: Abs) -> Term {
        Term::Abs(Box::new(abs))
    }
}

impl From<Callable> for Term {
    fn from(callable: Callable) -> Term {
        Term::Callable(Box::new(callable))
    }
}

impl From<(Callable, Abs)> for Term {
    fn from(tuple: (Callable, Abs)) -> Term {
        Term::CallableAbs(Box::new(tuple))
    }
}

impl Position for &Abs {
    fn position(self) -> FilePositionRange {
        self.position
    }
}

impl Position for &Var {
    fn position(self) -> FilePositionRange {
        self.position
    }
}

impl Position for &Callable {
    fn position(self) -> FilePositionRange {
        let Callable { left, right } = self;
        let start = left.position().start;
        let end = if right.is_empty() {
            left.position().end
        } else {
            right[right.len() - 1].position().end
        };
        FilePositionRange { start, end }
    }
}

impl Position for &Arg {
    fn position(self) -> FilePositionRange {
        match self {
            Arg::Var(var) => var.position,
            Arg::Parenthesized(term) => term.position(),
        }
    }
}

impl Into<NamedTerm> for Term {
    fn into(self) -> NamedTerm {}
}
