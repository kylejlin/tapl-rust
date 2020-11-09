use super::term::unnamed::{App, Term};

pub fn eval1(term: Term) -> Option<Term> {
    if let Term::App(app) = term {
        let App {
            callee,
            arg,
            position,
        } = *app;

        if callee.is_app() {
            eval1(callee).map(|evaluated_callee| {
                Term::App(Box::new(App {
                    callee: evaluated_callee,
                    arg,
                    position,
                }))
            })
        } else if callee.is_val() && arg.is_app() {
            eval1(arg).map(|evaluated_argument| {
                Term::App(Box::new(App {
                    callee,
                    arg: evaluated_argument,
                    position,
                }))
            })
        } else if arg.is_val() {
            if let Term::Abs(callee) = callee {
                Some(callee.apply(&arg))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
