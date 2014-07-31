use std::task;

#[deriving(Eq,PartialEq,Clone,Show,Send)]
enum Term {
    Num(int),
    Var(String),
    Lam(String, Box<Term>),
    App(Box<Term>, Box<Term>),
    Let(String, Box<Term>, Box<Term>),
}

// The following are wrappers over λ-terms to simplify writing
// allocations. It really does help, as you can see in main.
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

// A value is either a number or a closure, which has to have
// its environment around. We'll have to clone the environment
// into the closure to make sure that it stays around even if
// the closure is returned from the environment where it was used.
#[deriving(Eq,PartialEq,Clone,Show,Send)]
enum Val {
    VNum(int),
    VLam(String, Box<Term>, Box<Env>),
}

// I could also use a pair of a map and a parent pointer, but
// this is a little more TAPL-ish. Plus, we generally always
// bind a single variable at a time.
#[deriving(Eq,PartialEq,Clone,Show,Send)]
enum Env {
    Empty,
    Binding(String, Box<Val>, Box<Env>),
}

// We're going to just fail out of the thread if we can't find a binding.
// We can always wrap this in another thread if we want to get a value.
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

// The actual evaluator: this does some heap allocation, in particular, some
// copying of environments and allocating the result, for which it returns
// ownership.
fn lcalc_eval(t: &Term, e: &Env) -> Box<Val> {
    match t {
        &Num(num) => {
            box VNum(num)
        }
        &Var(ref str) => {
            lookup(str, e)
        }
        &Lam(ref s, ref b) => {
            box VLam(s.clone(), b.clone(), box e.clone())
        }
        &App(box ref f, box ref x) => {
            match *lcalc_eval(f, e) {
                VLam(ref arg, box ref body, box ref env) => {
                     let newEnv = Binding(arg.clone(),
                                          lcalc_eval(x, e),
                                          box env.clone());
                     lcalc_eval(body, &newEnv)
                  }
                _ => fail!("Tried to apply a non-function!")
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

// This copies the arguments and evaluates it in another thread, returning
// None if the evaluation fails.
fn lcalc_eval_opt(t: &Term, e: &Env) -> Option<Box<Val>> {
    let (tx, rx) = channel();
    let new_term = t.clone();
    let new_env = e.clone();
    let result = task::try(proc() {
        tx.send(lcalc_eval(&new_term, &new_env));
    });
    match result {
        Ok(_) => Some(rx.recv()),
        Err(_) => None,
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
    // (λx.y)(5), which will obviously fail
    let s3 = app(lam("x", var("y")), num(5));
    // (2)(3), which will also obviously fail
    let s4 = app(num(2), num(3));
    let e = Empty;
    println!("s1: {:}", lcalc_eval_opt(&*s1, &e));
    println!("s2: {:}", lcalc_eval_opt(&*s2, &e));
    println!("s3: {:}", lcalc_eval_opt(&*s3, &e));
    println!("s4: {:}", lcalc_eval_opt(&*s4, &e));
}
