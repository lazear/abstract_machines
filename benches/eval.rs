#[macro_use]
extern crate abstract_machines;
use abstract_machines::*;

use criterion::*;

fn numerals<F: Fn(Term) -> Term>(eval: F) {
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

fn church_encodings_cek(c: &mut Criterion) {
    c.bench_function("cek", |b| b.iter(|| black_box(numerals(cek::eval))));
}

fn church_encodings_beta(c: &mut Criterion) {
    c.bench_function("beta", |b| b.iter(|| black_box(numerals(beta::eval))));
}

criterion_group!(benches, church_encodings_cek, church_encodings_beta);
criterion_main!(benches);
