% The Rust Programming Language
% G.D. Ritter
% May 2015

# The Rust Programming Language

![](imgs/rust-logo.png)

A new systems programming language being developed by Mozilla Research, with
an emphasis on correctness while still allowing for very low-level programing
by emphasizing _zero-cost abstractions_.

# Low-Level Programming

# Low-Level Programming

\begin{center}
\includegraphics[width=.9\textwidth]{imgs/kanye-water-bottle-01.png}
\end{center}

# Low-Level Programming

\begin{center}
\includegraphics[width=.9\textwidth]{imgs/kanye-water-bottle-02.png}
\end{center}

# Systems Programming Languages

> System software is computer software designed to operate and control the
> computer hardware and to provide a platform for running application
> software, and includes such things as operating systems, utility software,
> device drivers, compilers, and linkers.
>
> —Wikipedia

> "Systems programs" means "programs where the constant factors are important".
>
> —Comment by `neelk` on Lambda the Ultimate

# Systems Programming Languages

## Example Program

~~~~{.haskell}
data Point = { x, y : Int }

addPoint : Point -> Point -> Point
addPoint l r = { x = l.x + r.x, y = l.y + r.y }

main : ()
main = { let a = { x = 1, y = 2 }
       ; let b = malloc { x = 4, y = 3}
       ; print (addPoint a (deref b))
       ; free(b)
       }
~~~~

# Systems Programming Languages

## C

~~~~{.c}
typedef struct { int x, y; } point;

point add(point a, point b) {
  point result = { a.x + b.x, a.y + b.y };
  return result;
}

void main(int argc, char* argv[]) {
  point a = { 1, 2 };
  point* b = malloc(sizeof(point));
  b->x = 4; b->y = 3;
  point c = add(a, *b);
  printf("{.x = %d, .y = %d}\n", c.x, c.y);
  free(b);
}
~~~~

# Systems Programming Languages

## C++

~~~~{.cpp}
struct point {
  int x, y;
  point(int _x, int _y) { x = _x; y = _y; }
  point add(point other) {
    return point(x + other.x, y + other.y);
  }
};
int main(int argc, char* argv[]) {
  point a(1, 2);
  point* b = new point(4, 3);
  point c = a.add(*b);
  cout << "{ .x = " << c.x;
  cout << ", .y = " << c.y << " }" << endl;
  delete b;
}
~~~~

# Systems Programming Languages

## Go

~~~~{.go}
type Point struct { X, Y int }

func (a Point) add(b Point) Point {
    return Point{ a.X + b.X, a.Y + b.Y }
}

func main() {
    a := Point{1, 2}
    b := new(Point)
    b.X, b.Y = 4, 3
    fmt.Println(a.add(*b))
    // No free, because Go is garbage-collected
}
~~~~

# Systems Programming Languages

## D

~~~~{.d}
struct Point {
  int x, y;
  Point add(Point other) {
    return Point(this.x + other.x, this.y + other.y);
  }
}

void main() {
  Point a = Point(1, 2);
  Point* b = cast(Point*)GC.malloc(Point.sizeof);
  b.x = 4; b.y = 3;
  writeln(a.add(*b));
  GC.free(b);
}
~~~~

# Systems Programming Languages

## Nim

~~~~
type Point = tuple[x: int, y: int]

proc add(a: Point, b: Point): Point =
  (x: a.x + b.x, y: a.y + b.y)

var a : Point
var b : ptr Point

a = (x: 1, y: 2)
b = cast[ptr Point](alloc(sizeof(Point)))
b.x = 4
b.y = 3
echo(add(a, b[]))
dealloc(b)
~~~~

# Systems Programming Languages

## Rust

~~~~{.rust}
struct Point { x: isize, y: isize }

impl Point {
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x,
                y: self.y + other.y }
    }
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = Box::new(Point { x: 4, y: 3 });
    println!("{:?}", a.add(*b));
}
~~~~

# What Makes Rust Interesting

> It's like C++ grew up, went to grad school, started dating Haskell, and
> is sharing an office with Erlang...
>
> —Michael Sullivan

# What Makes Rust Interesting

## Ownership

\begin{center}
\includegraphics[width=.9\textwidth]{imgs/dawkins-owned.png}
\end{center}

## Ownership

~~~~{.rust}
#[derive(Debug)]
struct MyNum { num: i32 }

fn main() {
  let x = MyNum { num: 2 };
  println!("x = {:?}", x);
}
~~~~

# Brief Aside
## Traits

~~~~{.rust}
trait ToString {
  fn to_string(&self) -> String;
}

impl ToString for () {
  fn to_string(&self) -> String {
    "unit".to_owned()
  }
}
~~~~

# Brief Aside
## Traits

~~~~{.rust}
fn print_excitedly<T: ToString>(t: T) {
  println!("{}!!!", t.to_string());
}

fn main() {
  print_excitedly( () );
}
~~~~

# Brief Aside from the Brief Aside
## Polymorphism

~~~~{.rust}
fn make_pair<A, B>(a: A, b: B) -> (A, B) {
  (a, b)
}

fn not_eq<A: Eq>(left: A, right: A) -> bool {
  left != right
}
~~~~

# Brief Aside from the Brief Aside
## Polymorphism

~~~~{.rust}
fn print_eq<A: Eq + ToString>(left: A, right: A) {
  if left == right {
    println!("{} and {} are equal",
             left.to_string(),
             right.to_string());
  } else {
    println!("{} and {} are different",
             left.to_string(),
             right.to_string());
  }
}
~~~~

# Brief Aside
## Traits

~~~~{.rust}
/* this is /slightly/ different in the stdlib */
trait PartialEq<Rhs> {
  fn eq(&self, other: &Rhs) -> bool;
  fn ne(&self, other: &Rhs) -> bool;
}

/* no more methods, but more laws */
trait Eq: PartialEq<Self> { }
~~~~

# Brief Aside
## Traits

~~~~{.rust}
struct MyNum { num: i32 }

impl PartialEq<MyNum> for MyNum {
  fn eq(&self, other: &MyNum) -> bool {
    self.num == other.num
  }
}

impl Eq for MyNum { }
~~~~

# Brief Aside
## Traits

~~~~{.rust}
/* or just this */
#[derive(PartialEq,Eq)]
struct MyNum { num: i32 }
~~~~

# What Makes Rust Interesting

## Ownership

~~~~{.rust}
#[derive(Debug)]
struct MyNum { num: i32 }

fn main() {
  let x = MyNum { num: 2 };
  let y = x;
  println!("x = {:?}", x);

}
~~~~

# What Makes Rust Interesting

## Ownership

~~~~{.rust}
#[derive(Debug)]
struct MyNum { num: i32 }

fn main() {
  let x = MyNum { num: 2 };
  let y = x; /* <- value moves here */
  println!("x = {:?}", x);

}
~~~~

# What Makes Rust Interesting

## Ownership

~~~~{.rust}
#[derive(Debug)]
struct MyNum { num: i32 }

fn main() {
  let x = MyNum { num: 2 };
  let y = x;
  println!("x = {:?}", x);
  /* so this does not compile */
}
~~~~

# What Makes Rust Interesting

## Ownership --- Explicit Cloning

~~~~{.rust}
#[derive(Debug, Clone)]
struct MyNum { num: i32 }

fn main() {
  let x = MyNum { num: 2 };
  let y = x.clone();
  println!("x = {:?}", x);
  /* but this does! */
}
~~~~

# What Makes Rust Interesting

## Ownership --- Implicit Copying

~~~~{.rust}
#[derive(Debug, Clone, Copy)]
struct MyNum { num: i32 }

fn main() {
  let x = MyNum { num: 2 };
  let y = x;
  println!("x = {:?}", x);
  /* as does this! */
}
~~~~

# What Makes Rust Interesting

## Ownership --- Destructors

~~~~{.rust}
#[derive(Debug)]
struct MyNum { num: i32 }

impl Drop for MyNum {
  fn drop(&mut self) {
    println!("dropping: {:?}", self)
  }
}

fn main() {
  let x = MyNum { num: 2 };
  println!("x = {:?}", x);
}
~~~~

# What Makes Rust Interesting

## Ownership --- Special Clones

~~~~{.rust}
#[derive(Debug)]
struct MyNum { num: i32 }

impl Clone for MyNum {
  fn clone(&self) -> Self {
    println!("Cloning a MyNum...");
    MyNum { num: self.num }
  }
}

fn main() {
  let x = MyNum { num: 2 };
  let y = x.clone();
  println!("x = {:?}", y);
}
~~~~

# What Makes Rust Interesting

## References

\begin{center}
\includegraphics[width=.9\textwidth]{imgs/dril-owned.png}
\end{center}

# What Makes Rust Interesting

## References

~~~~{.rust}
#[derive(Debug)]
struct MyNum { num: i32 }

fn some_func(_: MyNum) {
  println!("yeah, whatevs");
}

fn main() {
  let x = MyNum { num: 2 };
  some_func(x);
  println("{:?}", x);

}
~~~~

# What Makes Rust Interesting

## References

~~~~{.rust}
#[derive(Debug)]
struct MyNum { num: i32 }

fn some_func(_: MyNum) {
  println!("yeah, whatevs");
}

fn main() {
  let x = MyNum { num: 2 };
  some_func(x);
  println("{:?}", x);
  /* ERROR: use of moved value */
}
~~~~

# What Makes Rust Interesting

## References

~~~~{.rust}
#[derive(Debug)]
struct MyNum { num: i32 }

fn some_func(_: MyNum) -> MyNum {
  println!("yeah, whatevs");
}

fn main() {
  let x = MyNum { num: 2 };
  let y = some_func(x.clone());
  println("{:?}", y);
  /* works---but so tedious! */
}
~~~~

# What Makes Rust Interesting

## References

~~~~{.rust}
#[derive(Debug,Clone)]
struct MyNum { num: i32 }

fn some_func(_: MyNum) {
  println!("yeah, whatevs");
}

fn main() {
  let x = MyNum { num: 2 };
  some_func(x.clone());
  println("{:?}", x);
  /* works---but not what we want */
}
~~~~

# What Makes Rust Interesting

## References

~~~~{.rust}
#[derive(Debug,Clone)]
struct MyNum { num: i32 }

fn some_func(_: &MyNum)  {
  println!("yeah, whatevs");
}

fn main() {
  let x = MyNum { num: 2 };
  some_func(&x);
  println("{:?}", x);
  /* works! */
}
~~~~

# Pointers and Memory

# Pointers and Memory

## "Owned" Pointers

~~~~{.rust}
fn main() {
  let x: Box<[i32]> = Box::new([1,2,3]);
  /* x in scope */
  {
    let y: Box<[i32]> = Box::new([4,5,6]);
    /* x, y in scope */
  }
  /* x in scope */
}
~~~~

# Pointers and Memory

## "Owned" Pointers

~~~~{.rust}
fn main() {
  let x: Box<[i32]> =    // malloc |----+
    Box::new([1,2,3]);   //             |
  /* ... */              //             |
  {                      //             |
    let y: Box<[i32]> =  // malloc |-+  |
      Box::new([4,5,6]); //          |  |
    /* ... */            //          |  |
  }                      // free <---+  |
  /* ... */              //             |
}                        // free <------+
~~~~

# Pointers and Memory

## "Owned" Pointers

~~~~{.rust}
fn f0() -> Box<[i32]> {
  return Box::new([1,2,3]); // returning ownership
}
fn f1() -> Box<[int]> {
  let a = Box::new([1,2,3]);
  let b = a;
  return a; // error: use of moved value: `a`
}
fn f2() -> Box::new([int]) {
  let a = Box::new([1,2,3]);
  let b = a.clone();
  return a; // fine now; `a` and `b` both valid
}
~~~~

# Pointers and Memory

## "Owned" Pointers

~~~~{.rust}
#[deriving(Clone)]
enum List<T> { Cons(T, ~List<T>), Nil }

fn f3() -> Box<List<int>> {
  let mut a = Box::new(Cons(1,
    Box::new(Cons(2, Box::new(Nil)))));
  /* a is mutable */
  let b = a;
  /* can no longer use a, b is immutable */
  let mut c = b.clone();
  /* can use both b and c */
  return b;
}
~~~~

# Pointers and Memory

## Dispreferred Style

~~~~{.rust}
type t8 = (u32,u32,u32,u32,u32,u32,u32,u32);

fn eight_nums() -> Box<t8> {
  Box::new((1,2,3,4,5,6,7,8))
}

fn main() {
  let t: Box<t8> = eight_nums();
  /* ... */
}
~~~~

# Pointers and Memory

## Preferred Style

~~~~{.rust}
type t8 = (u32,u32,u32,u32,u32,u32,u32,u32);

fn eight_nums() -> t8 {
  (1,2,3,4,5,6,7,8)
}

fn main() {
  let t: Box<t8> = Box::new(eight_nums());
  /* ... */
}
~~~~

# Pointers and Memory

## References

~~~~{.rust}
{
  let p = Point { x: 1.2, y: 3.4 };
  let q = & p;
  // both p and q usable
}
{
  let q = & Point { x: 1.2, y: 3.4 };
}
{
  let p = Point { x: 1.2, y: 3.4 };
  let r = & p.x;
}
~~~~

# Pointers and Memory

## References

~~~~{.rust}
fn eq(xl: ~List<int>, yl: ~List<int>) -> bool {
  /* elided */
}

fn main() {
  let l1 = ~Cons(1, ~Cons (2, ~Nil));
  let l2 = ~Cons(3, ~Cons (4, ~Nil));
  println!("{}", eq(l1, l2));
  println!("{:?}", l1);
}
~~~~

# Pointers and Memory

## References

~~~~{.rust}
fn eq(xl: ~List<int>, yl: ~List<int>) -> bool {
  /* elided */
}

fn main() {
  let l1 = ~Cons(1, ~Cons (2, ~Nil));
  let l2 = ~Cons(3, ~Cons (4, ~Nil));
  println!("{}", eq(l1, l2)); // ownership of l1 and l2
                              // moves to eq function
  println!("{:?}", l1); // error: use of moved value!
}
~~~~

# Pointers and Memory

## References

~~~~{.rust}
fn eq(xl: ~List<int>, yl: ~List<int>) -> bool {
  /* elided */
}

fn main() {
  let l1 = ~Cons(1, ~Cons (2, ~Nil));
  let l2 = ~Cons(3, ~Cons (4, ~Nil));
  println!("{}", eq(l1.clone(), l2.clone()));
  println!("{:?}", l1);
}
~~~~

# Pointers and Memory

## References

~~~~{.rust}
fn eq(xl: &List<int>, yl: &List<int>) -> bool {
  /* elided */
}

fn main() {
  let l1 = ~Cons(1, ~Cons (2, ~Nil));
  let l2 = ~Cons(3, ~Cons (4, ~Nil));
  println!("{}", eq(l1, l2));
  println!("{:?}", l1);
}
~~~~

# Pointers and Memory

## References

~~~~{.rust}
fn eq(xl: &List<int>, yl: &List<int>) -> bool {
  match (xl, yl) {
    (&Nil, &Nil) => true,
    (&Cons(x, ~ref xs), &Cons(y, ~ref ys))
      if x == y => eq(xs, ys),
    (_, _) => false
  }
}
~~~~

# Pointers and Memory

## References

~~~~{.rust}
fn eq<T: Eq>(xl: &List<T>, yl: &List<T>) -> bool {
  match (xl, yl) {
    (&Nil, &Nil) => true,
    (&Cons(x, ~ref xs), &Cons(y, ~ref ys))
      if x == y => eq(xs, ys),
    (_, _) => false
  }
}
~~~~

# Pointers and Memory

## References and Lifetimes

~~~~{.rust}
{
  let     a = ~5;
  let mut p = &a;
  {
    let b = ~8;
    p     = &b;
  }
  println!("{}", **p)
}
~~~~


# Pointers and Memory

## References and Lifetimes

~~~~{.rust}
{
  let     a = ~5;     // malloc |---+
  let mut p = &a;     //            |
  {                   //            |
    let b = ~8;       // malloc |-+ |
    p     = &b;       //          | |
  }                   // free <---+ |
  println!("{}", **p) //            |
}                     // free <-----+
~~~~


# Pointers and Memory

## References and Lifetimes

~~~~{.rust}
{
  let     a = ~5;
  let mut p = &a;
  {
    let b = ~8;
    p     = &b; // error: borrowed value does
                // not live long enough
  }
  println!("{}", **p)
}
~~~~
# Pointers and Memory

## References, Pointers, Mutability

~~~~{.rust}
{
  let mut x = ~5;
  *x = *x + 1;
  {
    let y = &x;
    /* x is not mutable for the rest of this block */
  }
  /* x regains mutability */
}
~~~~

# Pointers and Memory

## References, Pointers, Mutability

~~~~{.rust}
enum IntList {
  Cons { head: int, tail: ~IntList },
  Nil,
}
{
  let mut lst = ~Cons { head: 5, tail: ~Nil };
  {
    let y = &(lst.head); // or &((*lst).head)
    lst = ~Nil;
    println!("{}", y);
  }
}
~~~~

# Pointers and Memory

## References, Pointers, Mutability

~~~~{.rust}
enum IntList {
  Cons { head: int, tail: ~IntList },
  Nil,
}
{
  let mut lst = ~Cons { head: 5, tail: ~Nil };
  {
    let y = &(lst.head);
    lst = ~Nil;
    println!("{}", y); // BAD
  }
}
~~~~

# Pointers and Memory

## Named Lifetimes

~~~~{.rust}
fn tail<T>(lst: &List<T>) -> &List<T> {
  match *lst {
    Nil              => &Nil,
    Cons(_, ~ref xs) => xs
  }
}
~~~~

# Pointers and Memory

## Named Lifetimes

~~~~{.rust}
fn tail<'s, T>(lst: &'s List<T>) -> &'s List<T> {
  match *lst {
    Nil              => &Nil,
    Cons(_, ~ref xs) => xs
  }
}
~~~~

# Pointers and Memory

## Reference Counting

~~~~{.rust}
use std::rc::Rc;
{
  let x = Rc::new([1,2,3]);
  let y = x.clone(); // two references, one vector
  assert!(x.ptr_eq(y));
  assert!(*y.borrow() == [1,2,3]);
}
~~~~

## Garbage Collection

~~~~{.rust}
use std::gc::Gc;
{
  let x = Gc::new([1,2,3]);
  // etc.
}
~~~~

# Pointers and Memory

## C Pointers

~~~~{.rust}
use std::ptr::RawPtr;

#[link(name="foo")]
extern {
  fn unsafe_get() -> *int;
}

fn safe_get() -> Option<int> {
  unsafe {
    let i = unsafe_get();
    i.to_option()
  }
}
~~~~

# Closures

# Closures

> [...] Lambdas are relegated to relative obscurity until Java makes them
> popular by not having them.
>
> —James Iry, "A Brief, Incomplete, and Mostly Wrong History of Programming
> Languages"

# Closures

## Functions

~~~~{.rust}
fn main() {
  let x = 5;
  fn inner(y: int) -> int {
    return x + y;
  }
  println!("{}", inner(1));
}
~~~~

# Closures

## Functions Do NOT Close Over Env

~~~~{.rust}
fn main() {
  let x = 5;
  fn inner(y: int) -> int {
    return x + y; // error: can't capture dynamic env
  }
  println!("{}", inner(1));
}
~~~~

# Closures

## Stack Closure

~~~~{.rust}
fn main() {
  let x = 5;
  let inner = |y| x + y;
  println!("{}", inner(1));
}
~~~~

## Stack Closure with Type Annotations

~~~~{.rust}
fn main() {
  let x = 5;
  let inner = |y: int| -> int { x + y };
  println!("{}", inner(1));
}
~~~~

# Closures

## Stack Closures

~~~~{.rust}
fn my_map<A,B>(f: |&A|->B, l: &List<A>) -> List<B> {
    match *l {
        Nil => Nil,
        Cons(ref x, ~ref xs) =>
          Cons(f(x)), ~my_map(f, xs))
    }
}

fn main() {
    fn incr(x: &int) -> int { x + 1 }
    let l = ~Cons(1, ~Cons(2, ~Cons(3, ~Nil)));
    println!("{:?}", my_map(|x| x + 1, l));
    println!("{:?}", my_map(incr, l));
}
~~~~

# Closures

## Owned Closures

~~~~{.rust}
use std::task::spawn;

fn main() {
    let x = ~5;
    spawn(proc() {
        println!("{}", x);
    });
    // x is now owned by the proc above
}
~~~~

# Methods

## Methods on a Struct

~~~~{.rust}
use std::f64::{sqrt,pow};
struct Point { x: f64, y: f64 }
impl Point {
    fn magnitude(&self) -> f64 {
        sqrt(pow(self.x,2.0)+pow(self.y,2.0))
    }
    fn new((my_x, my_y): (f64, f64)) -> Point {
        Point { x: my_x, y: my_y }
    }
}
fn main() {
    let p = Point::new((2.0,4.0));
    println!("{}", p.magnitude());
}
~~~~

# Methods

## Methods on an Enum

~~~~{.rust}
impl<T> List<T> {
  fn is_empty(&self) -> bool {
    match self {
      &Nil        => true,
      &Cons(_, _) => false,
    }
  }
}
~~~~

# Traits

## Head of a List By Reference
~~~~{.rust}
fn head<'a, T>(lst: &'a List<T>) -> Option<&'a T> {
  match lst {
    &Nil             => None,
    &Cons(ref hd, _) => Some(hd)
  }
}
~~~~

# Traits

## Head of a List By Value
~~~~{.rust}
fn head<T>(lst: &List<T>) -> Option<T> {
  match lst {
    &Nil             => None,
    &Cons(ref hd, _) => Some(*hd)
  }
}
~~~~

# Traits

## Head of a List By Value
~~~~{.rust}
fn head<T>(lst: &List<T>) -> Option<T> {
  match lst {
    &Nil             => None,
    &Cons(ref hd, _) => Some(*hd)
    // cannot move out of dereference of & pointer
  }
}
~~~~

# Traits

## Head of a List By Value
~~~~{.rust}
fn head<T: Clone>(lst: &List<T>) -> Option<T> {
  match lst {
    &Nil             => None,
    &Cons(ref hd, _) => Some(hd.clone())
  }
}
~~~~

# Traits

## Declaring Traits
~~~~{.rust}
trait Printable {
    fn print(&self);
}

impl Printable for int {
    fn print(&self) { println!("{}", *self) }
}

impl Printable for bool {
    fn print(&self) { println!("{}", *self) }
}

fn main() {
    5.print(); true.print();
}
~~~~

# Traits

## Using Multiple Traits

~~~~{.rust}
fn print_head<T: Clone+Printable>(lst: &List<T>) {
    match lst {
        &Nil             => { println!("Nothing!") }
        &Cons(ref hd, _) => { hd.clone().print() }
    }
}
~~~~

# Traits

## Static Dispatch

~~~~{.rust}
fn printAll<T: Printable>(vec: &[T]) {
    for p in vec.iter() { p.print() }
}
fn main() {
    printAll([1, 2, 3]);
}
~~~~

## Dynamic Dispatch

~~~~{.rust}
fn print_all(vec: &[~Printable]) {
    for p in vec.iter() { p.print() }
}
fn main() {
    print_all([~1 as ~Printable, ~true as ~Printable]);
}
~~~~

# Tasks and Communication

## Tasks

~~~~{.rust}
fn main() {
    spawn(proc() {
      println!("Hello from another task!");
    });
    println!("Hello from the parent task!");
}
~~~~

# Tasks and Communication

## Communication

~~~~{.rust}
fn main() {
    let (port, chan): (Port<int>, Chan<int>) = Chan::new();
    spawn(proc() {
        chan.send(some_computation());
    });
    some_other_computation();
    let result = port.recv();
}
~~~~

# Tasks and Communication

## Atomic Reference Counting

~~~~{.rust}
fn main() {
    let parent_copy = Arc::new(something_very_large());
    let (port, chan) = Chan::new();
    chan.send(parent_copy.clone());
    spawn(proc() {
        let task_copy = port.recv();
        task_copy.get().do_something();
    });
    parent_copy.get().do_something_else();
}
~~~~

# Tasks and Communication

## Failure

~~~~{.rust}
fn main() {
    let r : Result<int, ()> = try(proc() {
        if some_operation_succeeds() {
            return 5;
        } else {
            fail!("Hark! An error!");
        }
    });
    match r {
        Ok(i)  => println!("Got {}", i),
        Err(_) => println!("Hark!"),
    };
}
~~~~

# Crates and Modules

- A "crate" is a compilation unit; `rustc` produces a single crate
  if it is run (either a library or an executable.)
- A module is a grouping of definitions. Modules can be hierarchical
  and can be defined in a single file in `mod { ... }` blocks, or in
  separate files.

# Crates and Modules

## main.rs

~~~~{.rust}
mod mylist {
    pub enum List<T> { Cons(T, ~List<T>), Nil }
    pub fn from_vec<T>(mut vec : ~[T]) -> ~List<T> { ... }
    impl<T> List<T> {
        pub fn length(&self) -> int { ... }
    }
}

fn main() {
    let v = ~[1,2,3];
    let l = ::mylist::from_vec(v);
    /* ... */
}
~~~~

# Crates and Modules

## mylist.rs or mylist/mod.rs

~~~~{.rust}
mod mylist {
    pub enum List<T> { Cons(T, ~List<T>), Nil }
    pub fn from_vec<T>(mut vec : ~[T]) -> ~List<T> { ... }
    impl<T> List<T> {
        pub fn length(&self) -> int { ... }
    }
}
~~~~

## main.rs

~~~~{.rust}
mod mylist;
main() {
    let v = ~[1,2,3];
    let l = ::mylist::from_vec(v);
    /* ... */
}
~~~~

# Crates and Modules
## main.rs

~~~~{.rust}
use mylist::from_vec;
mod mylist;

main() {
    let v = ~[1,2,3];
    let l = from_vec(v);
    /* ... */
}
~~~~

# Crates and Modules
## Crate Metadata

~~~~{.rust}
#[crate_id = "mycrate#1.2"];
#[crate_type = "lib"];
~~~~

## Requesting Crate Metadata

~~~~{.rust}
extern crate mycrate "mycrate#1.2";
extern crate oldmycrate "mycrate#0.6";
~~~~

# The Future

# The Future

\begin{center}
\includegraphics[width=.9\textwidth]{imgs/flying-machines.jpg}
\end{center}

# The Future

## Possible Syntax Changes

- `~foo` might become `box foo`
- `~[T]` might become `Vec<T>`
- Operator overloading

## Possible Language Changes

- Speculations about inheritance, subtyping
- Stronger restrictions on `unsafe` code

## Standard Library Improvements

## Package Manager
