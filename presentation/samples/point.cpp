#include "stdio.h"
#include "stdlib.h"
#include <iostream>

struct point {
  int x, y;
  point(int _x, int _y) {
    x = _x; y = _y;
  }
  point add(point other) {
    return point(x + other.x, y + other.y);
  }
};

int main(int argc, char* argv[]) {
  point a(1, 2);
  point* b = new point(4, 3);
  point c = a.add(*b);
  std::cout << "{.x = " << c.x;
  std::cout << ", .y = " << c.y << "}" << std::endl;
  delete b;
}
