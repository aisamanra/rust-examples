struct Point {
    x: int,
    y: int
}

impl Point {
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x,
                y: self.y + other.y }
    }
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = ~Point { x: 4, y: 3 };
    println!("{:?}", a.add(*b));
}
