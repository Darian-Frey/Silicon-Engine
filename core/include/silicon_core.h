#ifndef SILICON_CORE_H
#define SILICON_CORE_H

#include <cstdint>

extern "C" {
// Returns the version of the Forensic Core
int32_t get_core_version();

// A simple test to prove we can pass data
int32_t add_numbers(int32_t a, int32_t b);
}

#endif