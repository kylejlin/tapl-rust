use super::super::term::named;
use crate::file_position::{FilePositionRange, Position};
use named::Term as NamedTerm;

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
            Term::Abs(a) => a.position(),
            Term::Callable(c) => c.position(),
            Term::CallableAbs(ca) => {
                let (c, a) = &**ca;
                FilePositionRange {
                    start: c.position().start,
                    end: a.position().end,
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
            Arg::Var(var) => var.position(),
            Arg::Parenthesized(term) => term.position(),
        }
    }
}

impl Into<NamedTerm> for Term {
    fn into(self) -> NamedTerm {
        let position = self.position();
        match self {
            Term::Abs(abs) => (*abs).into(),
            Term::Callable(callable) => (*callable).into(),
            Term::CallableAbs(ca) => {
                let (callable, abs) = *ca;
                named::App {
                    position,
                    callee: callable.into(),
                    arg: abs.into(),
                }
                .into()
            }
        }
    }
}

impl Into<named::Abs> for Abs {
    fn into(self) -> named::Abs {
        named::Abs {
            position: self.position(),
            param: self.param.into(),
            body: self.body.into(),
        }
    }
}

impl Into<NamedTerm> for Abs {
    fn into(self) -> NamedTerm {
        let named_abs: named::Abs = self.into();
        named_abs.into()
    }
}

impl Into<named::Var> for Var {
    fn into(self) -> named::Var {
        named::Var {
            position: self.position(),
            name: self.name,
        }
    }
}

impl Into<NamedTerm> for Var {
    fn into(self) -> NamedTerm {
        let named_var: named::Var = self.into();
        named_var.into()
    }
}

impl Into<NamedTerm> for Callable {
    fn into(self) -> NamedTerm {
        if self.right.is_empty() {
            self.left.into()
        } else {
            let mut app: NamedTerm = self.left.into();
            for arg in self.right {
                app = named::App {
                    position: FilePositionRange {
                        start: app.position().start,
                        end: arg.position().end,
                    },
                    callee: app,
                    arg: arg.into(),
                }
                .into();
            }
            app
        }
    }
}

impl Into<NamedTerm> for Arg {
    fn into(self) -> NamedTerm {
        match self {
            Arg::Var(var) => var.into(),
            Arg::Parenthesized(inner) => (*inner).into(),
        }
    }
}
