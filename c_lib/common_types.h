#ifndef COMMON_TYPES_H
#define COMMON_TYPES_H

#define SYSTEM_LIMIT 100

typedef struct {
  float x;
  float y;
  const char* label;
} DataPoint;

typedef struct {
  int key;
  float value;
} ConfigEntry;

#endif