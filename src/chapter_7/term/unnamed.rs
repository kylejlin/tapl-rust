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
    pub index: usize,
    pub context_length: usize,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Abs {
    pub position: FilePositionRange,
    pub parameter_name: String,
    pub body: Term,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct App {
    pub position: FilePositionRange,
    pub callee: Term,
    pub argument: Term,
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
                    argument: shift_with_cutoff(app.argument, amount, cutoff),
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
                argument: app.argument.subst(replacee, replacer),
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

    pub fn position(&self) -> &FilePositionRange {
        match self {
            Term::Var(var) => &var.position,
            Term::Abs(abs) => &abs.position,
            Term::App(app) => &app.position,
        }
    }
}

impl Abs {
    pub fn apply(self, argument: &Term) -> Term {
        self.body.subst(0, &argument.clone().shift(1)).shift(-1)
    }
}
