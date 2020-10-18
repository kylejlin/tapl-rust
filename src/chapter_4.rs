#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>),
}

pub mod term_builder {
    use super::*;

    pub fn if_(t1: Term, t2: Term, t3: Term) -> Term {
        Term::If(Box::new(t1), Box::new(t2), Box::new(t3))
    }

    pub fn succ(t1: Term) -> Term {
        Term::Succ(Box::new(t1))
    }

    pub fn pred(t1: Term) -> Term {
        Term::Pred(Box::new(t1))
    }
    pub fn is_zero(t1: Term) -> Term {
        Term::IsZero(Box::new(t1))
    }
}

pub fn is_numeric_val(t: &Term) -> bool {
    match t {
        Term::Zero => true,
        Term::Succ(t1) => is_numeric_val(t1),
        _ => false,
    }
}

pub fn is_val(t: &Term) -> bool {
    match t {
        Term::True | Term::False => true,
        _ => is_numeric_val(t),
    }
}

pub fn eval1(t: Term) -> Option<Term> {
    match t {
        Term::If(guard, then_term, _) if *guard == Term::True => Some(*then_term.clone()),
        Term::If(guard, _, else_term) if *guard == Term::False => Some(*else_term.clone()),
        Term::If(guard, then_term, else_term) => eval1(*guard).map(|evaluated_guard| {
            Term::If(
                Box::new(evaluated_guard),
                then_term.clone(),
                else_term.clone(),
            )
        }),

        Term::Succ(n) => eval1(*n).map(|evaluated_n| term_builder::succ(evaluated_n)),
        Term::Pred(n) => {
            if let Term::Succ(n2) = *n {
                if is_numeric_val(&*n2) {
                    Some(*n2.clone())
                } else {
                    None
                }
            } else if Term::Zero == *n {
                Some(Term::Zero)
            } else {
                eval1(*n).map(|evaluated_n| term_builder::pred(evaluated_n))
            }
        }
        Term::IsZero(n) => {
            if is_val(&*n) {
                if *n == Term::Zero {
                    Some(Term::True)
                } else {
                    Some(Term::False)
                }
            } else {
                eval1(*n).map(|evaluated_n| term_builder::is_zero(evaluated_n))
            }
        }

        _ => None,
    }
}

pub fn eval(mut t: Term) -> Option<Term> {
    while !is_val(&t) {
        t = if let Some(evaluated_t) = eval1(t) {
            evaluated_t
        } else {
            return None;
        };
    }
    Some(t)
}
