use std::fmt;

#[derive(Clone, Debug)]
pub enum CpsTerm {
    Var(String),
    Abs(String, Box<CpsTerm>),
    App(Box<CpsTerm>, Box<CpsTerm>),
}

#[derive(Clone, Debug)]
pub enum CTerm {
    Return,
    Var(String),
    Abs(String, Box<CTerm>),
    App(Box<CTerm>, Box<CTerm>),
    KVar(usize),
    KAbs(usize, Box<CTerm>),
    KApp(Box<CTerm>, Box<CTerm>),
}

impl fmt::Display for CTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CTerm::Return => write!(f, "return"),
            CTerm::Var(x) => write!(f, "{}", x),
            CTerm::KVar(x) => write!(f, "k{}", x),
            CTerm::Abs(s, t) => write!(f, "(lambda ({}) {})", s, t),
            CTerm::KAbs(i, t) => write!(f, "(lambda (k{}) {})", i, t),
            CTerm::App(e1, e2) => write!(f, "({} {})", e1, e2),
            CTerm::KApp(e1, e2) => write!(f, "({} {})", e1, e2),
        }
    }
}

macro_rules! cvar {
    ($ex:expr) => {
        CTerm::Var($ex)
    };
}

macro_rules! cabs {
    ($s:expr, $ex:expr) => {
        CTerm::Abs($s.into(), Box::new($ex))
    };
}

macro_rules! capp {
    ($ex:expr, $ex2:expr) => {
        CTerm::App(Box::new($ex), Box::new($ex2))
    };
}

macro_rules! kvar {
    ($ex:expr) => {
        CTerm::KVar($ex)
    };
}

macro_rules! kabs {
    ($idx:expr, $ex:expr) => {
        CTerm::KAbs($idx, Box::new($ex))
    };
}

macro_rules! kapp {
    ($ex:expr, $ex2:expr) => {
        CTerm::KApp(Box::new($ex), Box::new($ex2))
    };
}

#[macro_export]
macro_rules! cpsvar {
    ($ex:expr) => {
        CpsTerm::Var($ex.into())
    };
}

#[macro_export]

macro_rules! cpsabs {
    ($s:expr, $ex:expr) => {
        CpsTerm::Abs($s.into(), Box::new($ex))
    };
}

#[macro_export]

macro_rules! cpsapp {
    ($ex:expr, $ex2:expr) => {
        CpsTerm::App(Box::new($ex), Box::new($ex2))
    };
}

#[derive(Default)]
pub struct Transform {
    vars: usize,
}

impl Transform {
    fn freshk(&mut self) -> usize {
        let v = self.vars;
        self.vars += 1;
        v
    }

    pub fn cps_transform(&mut self, term: CpsTerm, kont: CTerm) -> CTerm {
        println!("{:?}, {:?}", term, kont);
        match term {
            CpsTerm::Var(i) => kapp!(kont, cvar!(i)),
            CpsTerm::Abs(s, body) => {
                let k = self.freshk();
                kapp!(
                    kont,
                    kabs!(k, cabs!(s, self.cps_transform(*body, kvar!(k))))
                )
            }
            CpsTerm::App(e1, e2) => {
                let k1 = self.freshk();
                let k2 = self.freshk();
                let e2_ = self.cps_transform(
                    *e2,
                    kabs!(
                        k1,
                        // kapp!(kapp!(kvar!(k1), kont), kvar!9)
                        kapp!(kapp!(kvar!(k1), kvar!(k2)), kont)
                    ),
                );
                self.cps_transform(*e1, kabs!(k2, e2_))
            }
        }
    }

    pub fn cps(&mut self, term: CpsTerm, kont: CTerm) -> CTerm {
        println!("{:?}, {:?}", term, kont);
        match term {
            CpsTerm::Var(i) => kapp!(kont, cvar!(i)),
            CpsTerm::Abs(s, body) => {
                let k = self.freshk();
                let z = self.freshk();

                // // kont [[body]] (\z. k z)
                // kapp!(
                //     kont,
                //     kabs!(k, cabs!(s, self.cps_transform(*body, kabs!(z, kapp!(kvar!(k), kvar!(z))))))
                // )

                kapp!(
                    kont,
                    kabs!(k, cabs!(s, self.cps_transform(*body, kvar!(k))))
                )
            }
            CpsTerm::App(e1, e2) => {
                let z1 = self.freshk();
                let z2 = self.freshk();

                let e2_ = self.cps_transform(
                    *e2,
                    kabs!(
                        z2,
                        // kapp!(kapp!(kvar!(z1), kont), kvar!9)
                        kapp!(kapp!(kvar!(z1), kvar!(z2)), kont)
                    ),
                );
                self.cps_transform(*e1, kabs!(z1, e2_))
            }
        }
    }
}
