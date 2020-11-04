use crate::file_position::{FilePositionRange, Position};

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
    pub arg: Term,
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
            Term::Var(var) => var.position,
            Term::Abs(abs) => abs.position,
            Term::App(app) => app.position,
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
