#include <stdio.h>
#include <stdlib.h>

typedef struct {
  int x, y;
} point;

point add(point a, point b)
{
  point result = { a.x + b.x, a.y + b.y };
  return result;
}

int main(int argc, char* argv[])
{
  point a = { 1, 2 };
  point* b = malloc(sizeof(point));
  b->x = 4;
  b->y = 3;
  point c = add(a, *b);
  printf("{.x = %d, .y = %d}\n", c.x, c.y);
  return 0;
}
