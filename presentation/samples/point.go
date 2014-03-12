package main

import "fmt"

type Point struct { X, Y int }

func (a Point) add(b Point) Point {
	return Point{ a.X + b.X, a.Y + b.Y }
}

func main() {
	a := Point{1, 2}
	b := new(Point)
	b.X, b.Y = 4, 3
	fmt.Println(a.add(*b))
}
