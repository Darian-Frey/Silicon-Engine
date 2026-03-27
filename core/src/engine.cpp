#include "silicon_core.h"
#include <algorithm>
#include <cmath>
#include <cstring>
#include <fstream>
#include <vector>

struct RomSession {
  std::string system_name = "Unknown";
  std::string rom_title = "None";
  std::vector<uint8_t> rom_data; // Store the ROM in memory for fast scanning
  int32_t system_id = 0;
} current_session;

// Helper to calculate Shannon Entropy for a block of data
float calculate_shannon(const uint8_t *data, size_t size) {
  if (size == 0)
    return 0.0f;
  float entropy = 0;
  size_t counts[256] = {0};

  for (size_t i = 0; i < size; ++i)
    counts[data[i]]++;

  for (int i = 0; i < 256; ++i) {
    if (counts[i] > 0) {
      float p = (float)counts[i] / size;
      entropy -= p * std::log2(p);
    }
  }
  return entropy / 8.0f; // Normalize to 0.0 - 1.0
}

extern "C" {
int32_t get_core_version() { return 100; } // v0.1.0

void get_system_name(char *buffer) {
  std::strcpy(buffer, current_session.system_name.c_str());
}

void get_rom_title(char *buffer) {
  std::strcpy(buffer, current_session.rom_title.c_str());
}

int32_t identify_system(const char *path) {
  std::ifstream file(path, std::ios::binary);
  if (!file)
    return -1;

  // Load the entire ROM into memory for the scraper
  current_session.rom_data.assign((std::istreambuf_iterator<char>(file)),
                                  std::istreambuf_iterator<char>());

  uint8_t *header = current_session.rom_data.data();

  if (current_session.rom_data.size() >= 0x0104 &&
      std::memcmp(&header[0x0100], "SEGA", 4) == 0) {
    current_session.system_id = 1;
    current_session.system_name = "Sega Mega Drive";
    char title[49];
    std::memcpy(title, &header[0x0150], 48);
    title[48] = '\0';
    current_session.rom_title = std::string(title);
    return 1;
  }
  return 0;
}

void calculate_entropy_map(float *output_buffer, int32_t chunks) {
  if (current_session.rom_data.empty() || chunks <= 0)
    return;

  size_t rom_size = current_session.rom_data.size();
  size_t chunk_size = rom_size / chunks;

  for (int i = 0; i < chunks; ++i) {
    size_t offset = i * chunk_size;
    output_buffer[i] =
        calculate_shannon(&current_session.rom_data[offset], chunk_size);
  }
}
}