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

pub fn eval(term: Term) -> Term {
    eval_count(term).0
}

pub fn eval_count(mut term: Term) -> (Term, usize) {
    let mut i = 0;
    while let Some(evaluated) = eval1(term.clone()) {
        i += 1;
        term = evaluated;
    }
    (term, i)
}
