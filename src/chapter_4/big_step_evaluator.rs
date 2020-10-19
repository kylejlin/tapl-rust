#[cfg(test)]
mod test;

use super::{evaluator::is_numeric_val, term_builder, Term};

pub fn eval(t: Term) -> Option<Term> {
    match t {
        Term::True | Term::False | Term::Zero => Some(t),
        Term::If(guard, then, else_) => {
            eval(*guard).and_then(|evaluated_guard| match evaluated_guard {
                Term::True => eval(*then),
                Term::False => eval(*else_),
                _ => None,
            })
        }
        Term::Succ(n) => eval(*n)
            .filter(is_numeric_val)
            .map(|evaluated_n| term_builder::succ(evaluated_n)),
        Term::Pred(n) => eval(*n).and_then(|evaluated_n| {
            if let Term::Zero = evaluated_n {
                Some(Term::Zero)
            } else if let Term::Succ(n1) = evaluated_n {
                Some(*n1)
            } else {
                None
            }
        }),
        Term::IsZero(n) => eval(*n).and_then(|evaluated_n| {
            if let Term::Zero = evaluated_n {
                Some(Term::True)
            } else if is_numeric_val(&evaluated_n) {
                Some(Term::False)
            } else {
                None
            }
        }),
    }
}
