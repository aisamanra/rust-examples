import core.memory;
import std.stdio;

struct Point {
  int x, y;
  Point add(Point other) {
    return Point(this.x + other.x, this.y + other.y);
  }
}

void main() {
  Point a = Point(1, 2);
  Point* b = cast(Point*)GC.malloc(Point.sizeof);
  b.x = 4;
  b.y = 3;
  writeln(a.add(*b));
  GC.free(b);
}
