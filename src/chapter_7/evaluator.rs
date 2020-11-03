use super::term::unnamed::{App, Term};

pub fn eval1(term: Term) -> Option<Term> {
    if let Term::App(app) = term {
        let App {
            callee,
            argument,
            position,
        } = *app;

        if callee.is_app() {
            eval1(callee).map(|evaluated_callee| {
                Term::App(Box::new(App {
                    callee: evaluated_callee,
                    argument,
                    position,
                }))
            })
        } else if callee.is_val() && argument.is_app() {
            eval1(argument).map(|evaluated_argument| {
                Term::App(Box::new(App {
                    callee,
                    argument: evaluated_argument,
                    position,
                }))
            })
        } else if argument.is_val() {
            if let Term::Abs(callee) = callee {
                Some(callee.apply(&argument))
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
