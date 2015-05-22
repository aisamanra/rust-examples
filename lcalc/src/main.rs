use std::thread;

// This isn't a very Rust-ey lambda-calculus implementation. It's much
// more TAPL-ey, which I think makes it nice for pedagogical purposes
// when aimed at functional programmers, but you wouldn't actually
// use Rust like this in practice.

#[derive(Eq,PartialEq,Clone,Debug)]
enum Term {
    Num(i32),
    Var(String),
    Lam(String, Box<Term>),
    App(Box<Term>, Box<Term>),
    Let(String, Box<Term>, Box<Term>),
}

// The following are wrappers over λ-terms to simplify writing
// allocations. It really does help, as you can see in main.
fn num(n: i32) -> Box<Term> {
    Box::new(Term::Num(n))
}

fn var(s: &str) -> Box<Term> {
    Box::new(Term::Var(s.to_string()))
}

fn lam(x: &str, n: Box<Term>) -> Box<Term> {
    Box::new(Term::Lam(x.to_string(), n))
}

fn app(x: Box<Term>, y: Box<Term>) -> Box<Term> {
    Box::new(Term::App(x, y))
}

fn let_(x: &str, y: Box<Term>, z: Box<Term>) -> Box<Term> {
    Box::new(Term::Let(x.to_string(), y, z))
}

// A value is either a number or a closure, which has to have
// its environment around. We'll have to clone the environment
// into the closure to make sure that it stays around even if
// the closure is returned from the environment where it was used.
#[derive(Eq,PartialEq,Clone,Debug)]
enum Val {
    Num(i32),
    Lam(String, Box<Term>, Box<Env>),
}

fn vnum(n: i32) -> Box<Val> {
    Box::new(Val::Num(n))
}

fn vlam(x: String, b: Box<Term>, e: Box<Env>) -> Box<Val> {
    Box::new(Val::Lam(x, b, e))
}

// I could also use a pair of a map and a parent pointer, but
// this is a little more TAPL-ish. Plus, we generally always
// bind a single variable at a time.
#[derive(Eq,PartialEq,Clone,Debug)]
enum Env {
    Empty,
    Binding(String, Box<Val>, Box<Env>),
}

// We're going to just fail out of the thread if we can't find a binding.
// We can always wrap this in another thread if we want to get a value.
fn lookup(s: &String, e: &Env) -> Box<Val> {
    match *e {
        Env::Empty => { panic!(format!("Couldn't find {} in environment", s)) }
        Env::Binding(ref n, ref v, ref p) => {
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
        &Term::Num(num) => vnum(num),
        &Term::Var(ref str) => lookup(str, e),
        &Term::Lam(ref s, ref b) => {
            vlam(s.clone(), b.clone(), Box::new(e.clone()))
        }
        &Term::App(ref f, ref x) => {
            match *lcalc_eval(f, e) {
                Val::Lam(ref arg, ref body, ref env) => {
                    let new_env = Env::Binding(arg.clone(),
                                               lcalc_eval(x, e),
                                               Box::new(*env.clone()));
                     lcalc_eval(body, &new_env)
                  }
                _ => panic!("Tried to apply a non-function!")
            }
        }
        &Term::Let(ref s, ref t, ref b) => {
             let new_env =
             Env::Binding(s.clone(),
                          lcalc_eval(t, e),
                          Box::new(e.clone()));
             lcalc_eval(b, &new_env)
        }
    }
}

// This copies the arguments and evaluates it in another thread, returning
// None if the evaluation fails.
fn lcalc_eval_opt(t: &Term, e: &Env) -> Option<Box<Val>> {
    let new_term = t.clone();
    let new_env = e.clone();
    let guard = thread::spawn(move || {
        lcalc_eval(&new_term, &new_env)
    });
    match guard.join() {
        Ok(x) => Some(x),
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
    let e = Env::Empty;
    println!("s1: {:?}", lcalc_eval_opt(&*s1, &e));
    println!("s2: {:?}", lcalc_eval_opt(&*s2, &e));
    println!("s3: {:?}", lcalc_eval_opt(&*s3, &e));
    println!("s4: {:?}", lcalc_eval_opt(&*s4, &e));
}
