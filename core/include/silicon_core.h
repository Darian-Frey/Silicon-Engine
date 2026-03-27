#ifndef SILICON_CORE_H
#define SILICON_CORE_H

#include <cstdint>

extern "C" {
int32_t get_core_version();
int32_t identify_system(const char *path);
int32_t get_rom_size();
void get_system_name(char *buffer);
void get_rom_title(char *buffer);

// NEW: Calculate entropy for the loaded ROM
// chunks: how many segments to divide the ROM into (e.g., 256 for the ribbon)
void calculate_entropy_map(float *output_buffer, int32_t chunks);
void read_rom_range(uint32_t offset, uint32_t length, uint8_t *output_buffer);
void decode_sega_tile(uint32_t offset, uint8_t *output_rgba);
}

#endif