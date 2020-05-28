use super::*;
use std::rc::Rc;

#[derive(Clone, Default)]
struct Env {
    parent: Option<Rc<Env>>,
    clo: Option<Closure>,
}

impl Env {
    fn lookup(&self, idx: usize) -> Option<&Closure> {
        if idx == 0 {
            return self.clo.as_ref();
        }
        self.parent.as_ref()?.lookup(idx - 1)
    }

    fn extend(parent: Rc<Env>, clo: Closure) -> Rc<Env> {
        Rc::new(Env {
            parent: Some(parent.clone()),
            clo: Some(clo),
        })
    }
}

#[derive(Clone)]
struct Closure {
    term: Term,
    env: Rc<Env>,
}

enum Continuation {
    Return,
    Arg(Term, Rc<Env>, Box<Continuation>),
    Call(Term, Rc<Env>, Box<Continuation>),
}

struct Machine {
    control: Term,
    env: Rc<Env>,
    kont: Continuation,
}

impl Machine {
    fn step(self) -> Machine {
        use Continuation::*;
        use Term::*;
        match (self.control, self.env, self.kont) {
            (Var(idx), e, k) => match e.lookup(idx).cloned() {
                Some(clo) => Machine {
                    control: clo.term,
                    env: e,
                    kont: k,
                },
                None => panic!("invalid binding!"),
            },
            (App(e1, e2), e, k) => Machine {
                control: *e1,
                env: e.clone(),
                kont: Arg(*e2, e, Box::new(k)),
            },
            (Abs(body), env, Arg(a, e, k)) => Machine {
                control: a,
                env: e,
                kont: Call(Abs(body), env, k),
            },
            (Abs(body), env, Call(Abs(body_), e_, k)) => Machine {
                control: *body_,
                env: Env::extend(
                    e_,
                    Closure {
                        term: Abs(body),
                        env: env,
                    },
                ),
                kont: *k,
            },
            (c, e, Return) => Machine {
                control: c,
                env: e,
                kont: Return,
            },
            _ => panic!("Invalid machine state"),
        }
    }
}

fn eval(term: Term) -> Term {
    let mut m = Machine {
        control: term,
        env: Rc::new(Env::default()),
        kont: Continuation::Return,
    };
    loop {
        match (&m.control, &m.kont) {
            (Term::Abs(_), Continuation::Return) => break,
            _ => m = m.step(),
        }
    }
    m.control
}

#[cfg(test)]
mod test {
    use super::*;

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
        ($ex:expr, $ex2:expr) => { Term::App(Box::new($ex), Box::new($ex2))};
        ($ex1:expr, $($ex:expr),+) => {
            vec![$(var!($ex)),+].into_iter().fold(var!($ex1), |tm, n| app!(tm, n))
        }
    }

    #[test]
    fn eval_id() {
        let id = abs!(var!(0));
        let a = app!(id.clone(), id.clone());
        let ev = eval(a);
        assert_eq!(ev, id);
    }

    #[test]
    fn numerals() {
        let c0 = abs!(abs!(var!(0)));

        let succ = abs!(abs!(abs!(app!(
            app!(var!(2), var!(1)),
            app!(var!(1), var!(0))
        ))));

        let plus = abs!(abs!(abs!(abs!(app!(
            app!(var!(3), var!(1)),
            app!(app!(var!(2), var!(1)), var!(0))
        )))));

        let c1 = app!(succ.clone(), c0.clone());
        let c2 = app!(succ.clone(), c1.clone());
        let c3 = app!(succ.clone(), c2.clone());

        assert_eq!(
            eval(app!(succ.clone(), c3)),
            eval(app!(
                app!(app!(app!(plus.clone(), c2.clone()), c2.clone()), succ),
                c0
            ))
        );
    }

    #[test]
    fn eval_church() {
        let a = app!(
            app!(app!(church::test(), church::tru()), church::fls()),
            church::tru()
        );
        let b = app!(
            app!(app!(church::test(), church::fls()), church::fls()),
            church::tru()
        );
        let c = app!(app!(church::and(), church::fls()), church::tru());
        let d = app!(app!(church::and(), church::tru()), church::tru());
        assert_eq!(eval(a), church::fls());
        assert_eq!(eval(b), church::tru());
        assert_eq!(eval(c), church::fls());
        assert_eq!(eval(d), church::tru());
    }
}
