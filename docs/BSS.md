# 🔍 Binary Specification Sheet (BSS): Silicon Engine

**Document ID:** SPEC-BSS-CORE  
**Version:** 1.0  
**Focus:** Universal Binary Detection & Data Carving Logic

---

## 1. Universal Identification Layer
Before console-specific modules are triggered, the core must handle the "Physical" state of the binary.

| Feature | Logic | Purpose |
| :--- | :--- | :--- |
| **Endianness** | Scan for known word-swaps (e.g., `64 00` instead of `00 64`). | Normalize data for the Hex View. |
| **Magic Numbers** | Direct offset comparison against a global signature database. | Immediate System Identification. |
| **Padding Detection** | Statistical analysis of `$00` and `$FF` frequency. | Identifies "Dead Space" in the ROM. |

---

## 2. Entropy Analysis Specifications
Silicon Engine uses **Shannon Entropy** to visualize data density. The C++ Core calculates this in a sliding window.

* **Window Size:** 256 Bytes (Standard) / 1024 Bytes (Deep Scan).
* **Formula:** $H = -\sum p(x) \log_2 p(x)$
* **Color Mapping Table:**

| Entropy Value ($H$) | Visual Color | Probable Data Type |
| :--- | :--- | :--- |
| **0.0 - 1.5** | **Black / Grey** | Padding, empty space, or repeating zeroes. |
| **1.6 - 3.5** | **Cyan / Blue** | Structured Code, Jump Tables, or Text strings. |
| **3.6 - 5.5** | **Green / Lime** | Graphics (Tiles), Sprites, or Pattern data. |
| **5.6 - 8.0** | **Red / Magenta** | Compressed data, Encrypted blocks, or PCM Audio. |

---

## 3. Platform-Specific "Tells" (MVP Support)

### 3.1. Sega Mega Drive (Genesis)
* **Header Location:** `$0100`
* **Signature:** `SEGA` (4 bytes)
* **Tile Format:** 4bpp Planar, 8x8 pixels (32 bytes per tile).
* **Palette Signature:** Sequences of 16 Words where each Word fits the mask `0x0EEE`.

### 3.2. Nintendo Gameboy (GB/GBC)
* **Header Location:** `$0100` (Jump), `$0104` (Logo).
* **Signature:** Nintendo Logo Bitmap (48 bytes).
* **Tile Format:** 2bpp, 8x8 pixels (16 bytes per tile).
* **Cartridge Type:** Offset `$0147` (Determines the Memory Bank Controller).

### 3.3. Nintendo GBA
* **Header Location:** `$0004`
* **Signature:** Fixed Nintendo Logo sequence.
* **Identifier:** 4-character Game Code at `$00AC` (e.g., `AMZE` for Metroid).
* **Alignment:** Strict 4-byte (32-bit) word alignment.

---

## 4. Asset Carving & Extraction Logic
To move from raw bytes to a "Virtual File," the engine applies these rules:

### 4.1. Tile Scraper Snap
* **Sega:** Snap selection to **32-byte** increments.
* **Gameboy:** Snap selection to **16-byte** increments.
* **Standard:** Snap to **16-byte** (Paragraph) alignment if the system is unknown.

### 4.2. Palette Hunting Heuristics
1.  Scan for a sequence of 16 or 256 unique 16-bit values.
2.  Calculate "Color Distance." If values follow a logical "Ramp" (dark to light), flag as a **Potential Palette**.
3.  Cross-reference with the current viewport's graphics for a "Visual Match."

---

## 5. Metadata VFS Generation
The Binary Spec defines how the **Virtual File System** is populated:

* **ROOT/**
    * `SYSTEM_ID.TXT` (Raw Header info)
    * `VECTORS.BIN` (Interrupts and Boot Code)
    * `DATA_MAP.JSON` (Offset map of discovered entropy blocks)
    * **CARVED_ASSETS/**
        * `SCRAPED_TILES_01.PNG`
        * `SCRAPED_TILES_02.PNG`
        * `PALETTE_01.PAL`