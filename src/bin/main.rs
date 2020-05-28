#[macro_use]
extern crate abstract_machines;

use abstract_machines::*;
use cps::*;

fn main() {
    let id1 = cpsabs!("x", cpsvar!("x"));
    let id2 = cpsabs!("y", cpsvar!("y"));
    let mut t = cps::Transform::default();
    // let mut c = t.cps(cpsapp!(id1.clone(), id2.clone()), CTerm::Return);
    let mut c = t.cps(id1, CTerm::Return);
    // dbg!(c);
    println!("{}", c);

    let c0 = cpsabs!("x", cpsabs!("y", cpsvar!("y")));

    let succ = cpsabs!(
        "x",
        cpsabs!(
            "y",
            cpsabs!(
                "z",
                cpsapp!(
                    cpsapp!(cpsvar!("x"), cpsvar!("y")),
                    cpsapp!(cpsvar!("y"), cpsvar!("z"))
                )
            )
        )
    );

    let plus = cpsabs!(
        "x",
        cpsabs!(
            "y",
            cpsabs!(
                "z",
                cpsabs!(
                    "c",
                    cpsapp!(
                        cpsapp!(cpsvar!("x"), cpsvar!("z")),
                        cpsapp!(cpsapp!(cpsvar!("y"), cpsvar!("z")), cpsvar!("c"))
                    )
                )
            )
        )
    );

    let c1 = cpsapp!(succ.clone(), c0.clone());
    let c2 = cpsapp!(succ.clone(), c1.clone());
    let c3 = cpsapp!(succ.clone(), c2.clone());

    c = t.cps(c3, CTerm::Return);
    println!("{}", c);

    // let mut e = cps::Eval::default();
    // for i in 0..5 {
    //     c = e.eval1(c);
    //     println!("{:?}", c);
    // }
}
