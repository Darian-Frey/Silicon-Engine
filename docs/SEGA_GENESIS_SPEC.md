# 🎮 System Specification: Sega Genesis / Mega Drive

**Component ID:** SYS-SEGA-GEN  
**Status:** Baseline Specification for Silicon Engine  
**Target Hardware:** Motorola 68000 (Main), Zilog Z80 (Audio)

---

## 1. Identification (Magic Numbers)
The Sega Genesis has a highly standardized header located in the first bank of the ROM.

| Offset | Size | Value (Hex/ASCII) | Description |
| :--- | :--- | :--- | :--- |
| `$0100` | 16 bytes | `SEGA MEGA DRIVE ` or `SEGA GENESIS    ` | Hardware System ID |
| `$0120` | 48 bytes | String (ASCII) | Domestic Title (Japan/General) |
| `$0150` | 48 bytes | String (ASCII) | International Title (Export) |
| `$0180` | 14 bytes | `GM XXXXXXXX-XX` | Product Type & Serial Number |
| `$018E` | 2 bytes | Checksum (Word) | Internal ROM Checksum |

---

## 2. Memory Map & Endianness
* **Endianness:** **Big-Endian**. The Motorola 68000 reads the high byte first.
* **Standard ROM Limit:** $000000 to $3FFFFF (4MB). 
    * *Note: Games larger than 4MB (like Super Street Fighter II) utilize a Bank-Switching mapper.*

| Address Range | Description |
| :--- | :--- |
| `$000000 - $3FFFFF` | ROM Space (Cartridge) |
| `$FF0000 - $FFFFFF` | Work RAM (64KB) |
| `$A00000 - $A0FFFF` | Z80 Address Space |
| `$C00000 - $C0001F` | VDP (Video Display Processor) Registers |

---

## 3. Graphics Architecture (Tiles)
Genesis graphics are stored as linear tiles that the VDP interprets.

* **Format:** 4 bits per pixel (4bpp), Planar.
* **Tile Size:** 8x8 pixels.
* **Data Size:** Each tile is exactly 32 bytes (`8 pixels * 8 pixels * 0.5 bytes`).
* **Palettes:** 4 palettes of 16 colors each (64 colors total). 
    * Colors are stored as **9-bit BGR** (0-7 for each channel), usually packed into a 16-bit Word (`0000 BBB0 GGG0 RRR0`).

---

## 4. Forensic Signatures for "Silicon Engine"

### 4.1. The Entropy Profile
* **Code Sections:** High Entropy ($H > 4.5$). Usually located near the start of the ROM (after the header).
* **Tile Banks:** Medium-High Entropy ($3.5 < H < 4.5$). Often look like "vertical bands" in the Hex view due to the 32-byte tile alignment.
* **PCM Audio:** Extremely High Entropy (looks like white noise). Common in games with digitized speech (e.g., "SEGA!" scream).

### 4.2. Carving Rules (The "Pro" Logic)
* **Alignment Snap:** All extractions for the Genesis module should snap to **2-byte (Word)** boundaries for code and **32-byte** boundaries for graphics.
* **Empty Space:** Unused ROM space is typically padded with `$FF` or `$00`.

---

## 5. Metadata Extraction (VFS Mapping)
The following "Virtual Files" should be automatically generated in the Silicon Engine UI upon detection:

1.  `[HEADER] System_Info.txt` : Derived from offsets `$0100 - $01AF`.
2.  `[VECTORS] Exception_Table.bin` : The first 256 bytes (contains Stack Pointer and PC reset).
3.  `[REGION] Compatibility.md` : Extracted from offset `$01F0` (J=Japan, U=USA, E=Europe).

---

## 6. Known "Traps" for Developers
* **Interleaved ROMs:** Some old `.smd` dumps are interleaved (split into odd/even bytes). Silicon Engine should detect the `64 00` (Word-swapped `00 64`) pattern and offer to de-interleave to standard `.bin` format.
* **SRAM Info:** Offset `$01B0` contains the SRAM start/end addresses. This is critical for identifying where save-game data is mapped.