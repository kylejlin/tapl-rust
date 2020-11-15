use super::named;
use crate::file_position::{FilePositionRange, Position};
use named::Term as NamedTerm;
use std::fmt;
use std::ops::Add;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Term {
    Var(Var),
    Abs(Box<Abs>),
    App(Box<App>),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Var {
    pub position: FilePositionRange,
    pub index: usize,
    pub context_length: usize,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Abs {
    pub position: FilePositionRange,
    pub param_name: String,
    pub body: Term,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct App {
    pub position: FilePositionRange,
    pub callee: Term,
    pub arg: Term,
}

#[derive(Clone, Debug)]
pub struct Context {
    names: Vec<String>,
}

impl Term {
    pub fn shift(self, amount: i32) -> Term {
        fn shift_with_cutoff(term: Term, amount: i32, cutoff: usize) -> Term {
            match term {
                Term::Var(var) => Term::Var(Var {
                    index: if var.index >= cutoff {
                        ((var.index as i32) + amount) as usize
                    } else {
                        var.index
                    },
                    context_length: ((var.context_length as i32) + amount) as usize,
                    ..var
                }),

                Term::Abs(abs) => Term::Abs(Box::new(Abs {
                    body: shift_with_cutoff(abs.body, amount, cutoff + 1),
                    ..*abs
                })),

                Term::App(app) => Term::App(Box::new(App {
                    callee: shift_with_cutoff(app.callee, amount, cutoff),
                    arg: shift_with_cutoff(app.arg, amount, cutoff),
                    ..*app
                })),
            }
        }

        shift_with_cutoff(self, amount, 0)
    }

    pub fn subst(self, replacee: usize, replacer: &Term) -> Term {
        match self {
            Term::Var(var) => {
                if var.index == replacee {
                    replacer.clone()
                } else {
                    Term::Var(var)
                }
            }

            Term::Abs(abs) => Term::Abs(Box::new(Abs {
                body: abs.body.subst(replacee + 1, &replacer.clone().shift(1)),
                ..*abs
            })),

            Term::App(app) => Term::App(Box::new(App {
                callee: app.callee.subst(replacee, replacer),
                arg: app.arg.subst(replacee, replacer),
                ..*app
            })),
        }
    }

    pub fn is_val(&self) -> bool {
        self.is_abs()
    }

    pub fn is_var(&self) -> bool {
        if let Term::Var(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_abs(&self) -> bool {
        if let Term::Abs(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_app(&self) -> bool {
        if let Term::App(_) = self {
            true
        } else {
            false
        }
    }
}

impl Abs {
    pub fn apply(self, argument: &Term) -> Term {
        self.body.subst(0, &argument.clone().shift(1)).shift(-1)
    }
}

impl From<Var> for Term {
    fn from(var: Var) -> Term {
        Term::Var(var)
    }
}

impl From<Abs> for Term {
    fn from(abs: Abs) -> Term {
        Term::Abs(Box::new(abs))
    }
}

impl From<App> for Term {
    fn from(app: App) -> Term {
        Term::App(Box::new(app))
    }
}

impl Position for &Term {
    fn position(self) -> FilePositionRange {
        match self {
            Term::Var(var) => var.position(),
            Term::Abs(abs) => abs.position(),
            Term::App(app) => app.position(),
        }
    }
}

impl Position for &Var {
    fn position(self) -> FilePositionRange {
        self.position
    }
}

impl Position for &Abs {
    fn position(self) -> FilePositionRange {
        self.position
    }
}

impl Position for &App {
    fn position(self) -> FilePositionRange {
        self.position
    }
}

impl Term {
    pub fn from_named(named: NamedTerm, ctx: &Context) -> Result<Term, CannotFindVarInCtxErr> {
        match named {
            NamedTerm::Var(var) => Var::from_named(var, ctx),
            NamedTerm::Abs(abs) => {
                let position = abs.position();
                let param_name = abs.param.name;
                let body_ctx = ctx.clone() + param_name.clone();
                match Term::from_named(abs.body, &body_ctx) {
                    Ok(body) => Ok(Abs {
                        position,
                        param_name,
                        body,
                    }
                    .into()),
                    Err(e) => Err(e),
                }
            }
            NamedTerm::App(app) => {
                let position = app.position;
                match Term::from_named(app.callee, ctx) {
                    Ok(callee) => match Term::from_named(app.arg, ctx) {
                        Ok(arg) => Ok(App {
                            position,
                            callee,
                            arg,
                        }
                        .into()),
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                }
            }
        }
    }
}

impl Var {
    pub fn from_named(var: named::Var, ctx: &Context) -> Result<Term, CannotFindVarInCtxErr> {
        match ctx.get(&var.name) {
            Some(index) => Ok(Term::Var(Var {
                position: var.position,
                index,
                context_length: ctx.len(),
            })),
            None => Err(CannotFindVarInCtxErr(var)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CannotFindVarInCtxErr(pub named::Var);

impl Context {
    pub fn from_strs(strs: &[&str]) -> Context {
        Context {
            names: strs.iter().map(ToString::to_string).collect(),
        }
    }

    pub fn from_strings(strings: &[String]) -> Context {
        Context {
            names: strings.iter().cloned().collect(),
        }
    }

    pub fn get(&self, target: &str) -> Option<usize> {
        for (i, name) in self.names.iter().rev().enumerate() {
            if name == target {
                return Some(i);
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }
}

impl Add<String> for Context {
    type Output = Context;

    fn add(mut self, name: String) -> Context {
        self.names.push(name);
        self
    }
}

impl Add<String> for &Context {
    type Output = Context;

    fn add(self, name: String) -> Context {
        let clone = self.clone();
        clone + name
    }
}

impl Add<&str> for Context {
    type Output = Context;

    fn add(self, name: &str) -> Context {
        self + name.to_string()
    }
}

impl Add<&str> for &Context {
    type Output = Context;

    fn add(self, name: &str) -> Context {
        self + name.to_string()
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(var) => write!(f, "{}", var.index),
            Term::Abs(abs) => write!(f, "(\\. {})", abs.body),
            Term::App(app) => write!(f, "({} {})", app.callee, app.arg),
        }
    }
}
