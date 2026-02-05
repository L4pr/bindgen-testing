#ifndef ANALYTICS_H
#define ANALYTICS_H

#include "common_types.h"

float calculate_distance(DataPoint p);

DataPoint* create_heap_point(float x, float y, const char* label);
void free_heap_point(DataPoint* p);

#endif