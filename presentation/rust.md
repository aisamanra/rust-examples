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
