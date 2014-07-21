#![feature(macro_rules)]

#[deriving(Eq,PartialEq,Clone,Show)]
enum Term {
    Num(int),
    Var(String),
    Lam(String, Box<Term>),
    App(Box<Term>, Box<Term>),
    Let(String, Box<Term>, Box<Term>),
}

fn num(n: int) -> Box<Term> {
    box Num(n)
}

fn var(s: &str) -> Box<Term> {
    box Var(s.to_string())
}

fn lam(x: &str, n: Box<Term>) -> Box<Term> {
    box Lam(x.to_string(), n)
}

fn app(x: Box<Term>, y: Box<Term>) -> Box<Term> {
    box App(x, y)
}

fn let_(x: &str, y: Box<Term>, z: Box<Term>) -> Box<Term> {
    box Let(x.to_string(), y, z)
}

#[deriving(Eq,PartialEq,Clone,Show)]
enum Val {
    VNum(int),
    VLam(String, Box<Term>, Box<Env>),
}

#[deriving(Eq,PartialEq,Clone,Show)]
enum Env {
    Empty,
    Binding(String, Box<Val>, Box<Env>),
}

fn lookup(s: &String, e: &Env) -> Box<Val> {
    match *e {
        Empty => { fail!(format!("Couldn't find {} in environment", s)) }
        Binding(ref n, ref v, ref p) => {
            if n == s {
                v.clone()
            } else {
                lookup(s, &**p)
            }
        }
    }
}

fn lcalc_eval(t: &Term, e: &Env) -> Box<Val> {
    match t {
        &Num(num) => { box VNum(num) }
        &Var(ref str) => { lookup(str, e) }
        &Lam(ref s, ref b) => { box VLam(s.clone(), b.clone(), box e.clone()) }
        &App(box ref f, box ref x) => {
            match *lcalc_eval(f, e) {
                VLam(ref arg, box ref body, box ref env) => {
                     let newEnv = Binding(arg.clone(),
                                          lcalc_eval(x, e),
                                          box env.clone());
                     lcalc_eval(body, &newEnv)
                  }
                _ => fail!()
            }
        }
        &Let(ref s, box ref t, box ref b) => {
             let newEnv = Binding(s.clone(),
                                  lcalc_eval(t, e),
                                  box e.clone());
             lcalc_eval(b, &newEnv)
        }
    }
}

fn main() {
    // (λx.λy.x)(5)(6)
    let s1 = app(app(lam("x", lam("y", var("x"))),
                     num(5)),
                 num(6));
    // let f = (λx.λy.x)(2) in f 4
    let s2 = let_("f",
                  app(lam("x", lam("y", var("x"))),
                      num(2)),
                  app(var("f"),
                      num(4)));
    let e = Empty;
    println!("s1: {:}", lcalc_eval(&*s1, &e));
    println!("s2: {:}", lcalc_eval(&*s2, &e));
}
