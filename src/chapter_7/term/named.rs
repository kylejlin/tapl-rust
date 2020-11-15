use super::unnamed;
use crate::file_position::{FilePosition, FilePositionRange, Position};
use unnamed::{Context, Term as UnnamedTerm};

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

impl Term {
    pub fn unpositioned_from_unnamed(un: UnnamedTerm, ctx: &Context) -> Term {
        match un {
            UnnamedTerm::Var(var) => Var::unpositioned_from_unnamed(var, ctx).into(),
            UnnamedTerm::Abs(abs) => Abs::unpositioned_from_unnamed(*abs, ctx).into(),
            UnnamedTerm::App(app) => App::unpositioned_from_unnamed(*app, ctx).into(),
        }
    }
}

impl Var {
    pub fn unpositioned_from_unnamed(un: unnamed::Var, ctx: &Context) -> Var {
        let name = if let Some(name) = ctx.name(un.index) {
            name.to_string()
        } else {
            format!("${}", un.index)
        };
        Var {
            name,
            position: dummy_position(),
        }
    }
}

impl Abs {
    pub fn unpositioned_from_unnamed(un: unnamed::Abs, ctx: &Context) -> Abs {
        let (body_ctx, used_param_name) = extend_ctx(ctx, un.param_name);

        fn extend_ctx(ctx: &Context, ideal_name: String) -> (Context, String) {
            let mut used_name = ideal_name;
            while let Some(_) = ctx.index(&used_name) {
                used_name.push('\'');
            }
            (ctx + used_name.clone(), used_name)
        }

        Abs {
            position: dummy_position(),
            param: Var {
                position: dummy_position(),
                name: used_param_name,
            },
            body: Term::unpositioned_from_unnamed(un.body, &body_ctx),
        }
    }
}

impl App {
    pub fn unpositioned_from_unnamed(un: unnamed::App, ctx: &Context) -> App {
        App {
            position: dummy_position(),
            callee: Term::unpositioned_from_unnamed(un.callee, ctx),
            arg: Term::unpositioned_from_unnamed(un.arg, ctx),
        }
    }
}

fn dummy_position() -> FilePositionRange {
    FilePositionRange {
        start: FilePosition {
            index: 0,
            column: 0,
            line: 0,
        },
        end: FilePosition {
            index: 0,
            column: 0,
            line: 0,
        },
    }
}
