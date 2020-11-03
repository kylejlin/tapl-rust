use crate::file_position::FilePositionRange;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Term {
    Var(Var),
    Abs(Box<Abs>),
    App(Box<App>),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Var {
    pub position: FilePositionRange,
    pub name: String,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Abs {
    pub position: FilePositionRange,
    pub param: Var,
    pub body: Term,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct App {
    pub position: FilePositionRange,
    pub callee: Term,
    pub argument: Term,
}

impl Term {
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

    pub fn position(&self) -> &FilePositionRange {
        match self {
            Term::Var(var) => &var.position,
            Term::Abs(abs) => &abs.position,
            Term::App(app) => &app.position,
        }
    }
}
