use std::str::eq_slice;

#[deriving(Eq,Clone)]
enum Term {
    Num(int),
    Var(~str),
    Lam(~str, ~Term),
    App(~Term, ~Term),
    Let(~str, ~Term, ~Term),
}

#[deriving(Eq,Clone)]
enum Val {
    VNum(int),
    VLam(~str, ~Term, ~Env),
}

#[deriving(Eq,Clone)]
enum Env {
    Empty,
    Binding(~str, ~Val, ~Env),
}

fn lookup(s: &str, e: &Env) -> ~Val {
    match *e {
        Empty => { fail!(format!("Couldn't find {} in environment", s)) }
        Binding(ref n, ref v, ref p) => {
            if eq_slice(*n, s) {
                v.clone()
            } else {
                lookup(s, *p)
            }
        }
    }
}

fn eval(t: &Term, e: &Env) -> ~Val {
    match t {
        &Num(num) => { ~VNum(num) }
        &Var(ref str) => { lookup(*str, e) }
        &Lam(ref s, ref b) => { ~VLam(s.clone(), b.clone(), ~e.clone()) }
        &App(ref f, ref x) => {
            match (*eval(*f, e)) {
                VLam(ref arg, ref body, ref env) => {
                     let newEnv = Binding(arg.clone(),
                                          eval(*x, e),
                                          env.clone());
                     eval(*body, &newEnv)
                  }
                _ => fail!()
            }
        }
        &Let(ref s, ref t, ref b) => {
             let newEnv = Binding(s.clone(),
                                  eval(*t, e),
                                  ~e.clone());
             eval(*b, &newEnv)
        }
    }
}

fn main() {
    // (λx.λy.x)(5)(6)
    let s1 = ~App(~App(~Lam(~"x",~Lam(~"y", ~Var(~"x"))),~Num(5)),~Num(8));
    // let f = (λx.λy.x)(2) in f 4
    let s2 = ~Let( ~"f",
                     ~App(~Lam(~"x",~Lam(~"y", ~Var(~"x"))),~Num(2)),
                     ~App(~Var(~"f"),~Num(4))
                     );
    let e = Empty;
    println!("s1: {:?}", eval(s1, &e));
    println!("s2: {:?}", eval(s2, &e));
}
