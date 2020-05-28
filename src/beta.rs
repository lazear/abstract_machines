#[macro_use]
use super::*;

fn walk<F: Fn(&mut Term, usize)>(term: &mut Term, cutoff: usize, f: &F) {
    match term {
        Term::Var(_) => {
            f(term, cutoff);
        }
        Term::Abs(t2) => walk(t2, cutoff + 1, f),
        Term::App(t1, t2) => {
            walk(t1, cutoff, f);
            walk(t2, cutoff, f);
        }
    }
}

fn shift(t: &mut Term, s: isize) {
    walk(t, 0, &|f, c| {
        if let Term::Var(n) = f {
            if *n >= c {
                *n = (*n as isize + s) as usize;
            }
        }
    })
}

fn subst(t: &mut Term, mut s: Term) {
    walk(t, 0, &|f, c| {
        if let Term::Var(n) = f {
            if *n == c {
                let mut sub = s.clone();
                shift(&mut sub, c as isize);
                *f = sub;
            }
        }
    })
}

/// Substitute term `s` into term `t`
fn subst_top(t: &mut Term, mut s: Term) {
    shift(&mut s, 1);
    subst(t, s);
    shift(t, -1);
}

fn eval1(tm: Term) -> Result<Term, Term> {
    match tm {
        Term::App(e1, e2) => {
            if !e2.nf() {
                Ok(Term::App(e1, Box::new(eval1(*e2)?)))
            } else if !e1.nf() {
                Ok(Term::App(Box::new(eval1(*e1)?), e2))
            } else {
                match *e1 {
                    Term::Abs(mut e) => {
                        subst_top(&mut e, *e2);
                        Ok(*e)
                    }
                    _ => Err(Term::App(e1, e2)),
                }
            }
        }
        _ => Err(tm),
    }
}

pub fn eval(tm: Term) -> Term {
    let mut t = tm;
    loop {
        match eval1(t) {
            Ok(e) => t = e,
            Err(e) => return e,
        }
    }
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
    fn eval_subst1() {
        let t = app!(abs!(app!(app!(var!(1), var!(0)), var!(2))), abs!(var!(0)));
        let res = app!(app!(var!(0), abs!(var!(0))), var!(1));
        assert_eq!(eval1(t), Ok(res));
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
        let a = app!(app!(app!(church::test(), church::tru()), var!(5)), var!(7));
        let b = app!(app!(app!(church::test(), church::fls()), var!(5)), var!(7));
        let c = app!(app!(church::and(), church::fls()), church::tru());
        let d = app!(app!(church::and(), church::tru()), church::tru());
        assert_eq!(eval(a), var!(5));
        assert_eq!(eval(b), var!(7));
        assert_eq!(eval(app!(app!(church::tru(), var!(1)), var!(2))), var!(1));
        assert_eq!(eval(app!(app!(church::fls(), var!(1)), var!(2))), var!(2));
        assert_eq!(eval(c), church::fls());
        assert_eq!(eval(d), church::tru());
    }
}
