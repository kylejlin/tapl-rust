use super::parse;
use comparable_result::CmpResult;
use unpositioned as unp;

mod unpositioned {
    use crate::chapter_7::term::named::Term as NamedTerm;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Term {
        Var(Var),
        Abs(Box<Abs>),
        App(Box<App>),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Var {
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Abs {
        pub param: Var,
        pub body: Term,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct App {
        pub callee: Term,
        pub arg: Term,
    }

    pub fn var(name: &str) -> Term {
        Term::Var(Var {
            name: name.to_string(),
        })
    }

    pub fn abs(param: &str, body: Term) -> Term {
        Term::Abs(Box::new(Abs {
            param: Var {
                name: param.to_string(),
            },
            body,
        }))
    }

    pub fn app(callee: Term, arg: Term) -> Term {
        Term::App(Box::new(App { callee, arg }))
    }

    impl PartialEq<Term> for NamedTerm {
        fn eq(&self, other: &Term) -> bool {
            true
        }
    }

    impl PartialEq<NamedTerm> for Term {
        fn eq(&self, other: &NamedTerm) -> bool {
            true
        }
    }
}

mod comparable_result {
    #[derive(Clone, Debug)]
    pub enum CmpResult<T, E> {
        Ok(T),
        Err(E),
    }

    impl<T, E> CmpResult<T, E> {
        pub fn from_result(r: Result<T, E>) -> CmpResult<T, E> {
            match r {
                Ok(t) => CmpResult::Ok(t),
                Err(e) => CmpResult::Err(e),
            }
        }

        pub fn into_result(self) -> Result<T, E> {
            match self {
                CmpResult::Ok(t) => Ok(t),
                CmpResult::Err(e) => Err(e),
            }
        }
    }

    impl<T, E, T2, E2> PartialEq<CmpResult<T2, E2>> for CmpResult<T, E>
    where
        T: PartialEq<T2>,
        E: PartialEq<E2>,
    {
        fn eq(&self, rhs: &CmpResult<T2, E2>) -> bool {
            use CmpResult::{Err, Ok};

            match (self, rhs) {
                (Ok(t), Ok(t2)) => t == t2,
                (Err(e), Err(e2)) => e == e2,
                _ => false,
            }
        }
    }

    impl<T, E> Eq for CmpResult<T, E>
    where
        T: Eq,
        E: Eq,
    {
    }
}

#[test]
fn identity() {
    use super::ParseErr;

    let actual = CmpResult::from_result(parse(r"\x.x"));
    let expected: CmpResult<_, ParseErr> = CmpResult::Ok(unp::abs("x", unp::var("x")));
    let j = actual.into_result().unwrap();
    let k = unp::var("hi");
    let b = CmpResult::<_, ()>::Ok(k) == CmpResult::Ok(j);
}
