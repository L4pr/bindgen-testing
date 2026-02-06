#include "analytics.h"
#include <math.h>
#include <stdio.h>
#include <stdlib.h>

float calculate_distance(DataPoint p) {
  printf("Point Label: %s\n", p.label);
  return sqrt(p.x * p.x + p.y * p.y);
}

DataPoint* create_heap_point(float x, float y, const char* label) {
  DataPoint* p = (DataPoint*)malloc(sizeof(DataPoint));
  p->x = x;
  p->y = y;
  p->label = label;
  return p;
}

void free_heap_point(DataPoint* p) {
  if (p) free(p);
}