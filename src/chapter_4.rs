pub mod big_step_evaluator;
pub mod evaluator;
mod lexer;
pub mod parser;

pub use evaluator::eval;
pub use parser::parse;

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
