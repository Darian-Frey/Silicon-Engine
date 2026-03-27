#include "silicon_core.h"

extern "C" {
int32_t get_core_version() {
  return 100; // Represents v1.0.0
}

int32_t add_numbers(int32_t a, int32_t b) { return a + b; }
}