mod beta_reduction;
mod cek;

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Var(usize),
    Abs(Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl Term {
    fn whnf(&self) -> bool {
        match self {
            Term::Abs(e) => e.whnf(),
            Term::App(e, _) => e.whnf(),
            _ => true,
        }
    }

    fn nf(&self) -> bool {
        match self {
            Term::App(_, _) => false,
            _ => true,
        }
    }
}

macro_rules! var {
    ($ex:expr) => {
        Term::Var($ex)
    };
}

macro_rules! abs {
    ($ex:expr) => {
        Term::Abs(Box::new($ex))
    };
}

macro_rules! app {
    ($ex:expr, $ex2:expr) => {
        Term::App(Box::new($ex), Box::new($ex2))
    };
}

pub mod church {
    use super::*;
    pub fn tru() -> Term {
        abs!(abs!(var!(1)))
    }
    pub fn fls() -> Term {
        abs!(abs!(var!(0)))
    }
    pub fn test() -> Term {
        abs!(abs!(abs!(app!(app!(var!(2), var!(1)), var!(0)))))
    }
    pub fn and() -> Term {
        abs!(abs!(app!(app!(var!(1), var!(0)), fls())))
    }
}
