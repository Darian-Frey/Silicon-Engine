# 🔪 Asset Carving Logic Specification: Silicon Engine

**Document ID:** SPEC-ACLS-01  
**Version:** 1.0  
**Focus:** Heuristic Data Identification & Surgical Extraction Algorithms

---

## 1. The Carving Philosophy
In a ROM environment without a file system, "files" do not exist—only contiguous blocks of data with varying purposes. The **Asset Carving Engine** uses statistical analysis (Entropy) and architectural "signatures" to define logical boundaries for these blocks, effectively "carving" them out of the flat binary.

---

## 2. Statistical Pre-Processing (The "Radar")
Before carving begins, the engine performs a **Sliding Window Shannon Entropy** scan to identify the "Texture" of the data.

* **Window Size (W):** 256 bytes (optimal for 8/16-bit tiles).
* **Step Size (S):** 16 bytes.
* **Target Calculation:** $H = -\sum_{i=0}^{n-1} p(x_i) \log_2 p(x_i)$

### 2.1. Data Type Probability Table
| Entropy Range (H) | Probable Content | Action |
| :--- | :--- | :--- |
| **0.0 - 1.0** | **Null/Padding** | Ignore or mark as "Free Space." |
| **1.1 - 3.8** | **Code / Logic** | Flag for Disassembly/Vector scanning. |
| **3.9 - 5.5** | **Structured Graphics** | Trigger **Tile Scraper** heuristic. |
| **5.6 - 7.2** | **Waveform Audio** | Trigger **PCM Wave** heuristic. |
| **7.3 - 8.0** | **Compressed/Encrypted** | Trigger **Header Search** for LZSS/Huffman. |

---

## 3. Tile Carving Heuristics (Graphics)
Once a "Green" (Medium-High Entropy) zone is identified, the engine attempts to align the data into viewable sprites.

### 3.1. Alignment & Snapping
The carver utilizes **System-Specific Alignment (SSA)** to prevent "ghosting" or skewed graphics.
* **Sega Genesis:** Snap to 32-byte boundaries (8x8 tiles @ 4bpp).
* **Gameboy:** Snap to 16-byte boundaries (8x8 tiles @ 2bpp).
* **SNES:** Snap to 32 or 64-byte boundaries (depending on bit-depth).

---

## 4. Palette Hunting (The "Rainbow" Algorithm)
Extraction is useless without color. The **Palette Hunter** scans for color data to apply to carved tiles.

1.  **Bit-Depth Filter:** Search for 16-word (4bpp) or 256-word (8bpp) arrays.
2.  **Luminance Gradient Check:** Valid palettes usually follow a mathematical "Ramp" (e.g., dark to light transitions).
3.  **Invalid Color Rejection:** If more than 10% of values fall outside the target console's hardware color-depth (e.g., 9-bit for Sega), the palette is discarded as noise.

---

## 5. Automated Extraction Pipeline (Pro/Dev Feature)

### 5.1. The "Smart-Export" Logic
When a user clicks "Extract Asset," the engine follows these steps:
1.  **Define Bounds:** Scan forward and backward from the cursor until entropy significantly shifts (> 15% delta).
2.  **Normalize:** Adjust the start/end offsets to match the **System Alignment**.
3.  **BPP Conversion:** Map the raw bit-planar data into a standard 8-bit RGBA buffer.
4.  **Output:** Stream the buffer to the Rust layer for PNG encoding.

### 5.2. Metadata Generation
Every carved asset generates an entry in the **Virtual File System (VFS)**:

- **Asset ID:** CARVE_001
- **Offset:** 0x0A4000
- **Length:** 0x4000
- **Type:** GRAPHICS_4BPP
- **Confidence:** 0.92

---

## 6. Pro-Edition: Binary Carving Tools
* **Manual Adjust:** Allow users to override the "Step Size" (e.g., forcing a 16-pixel wide strip for arcade sprites).
* **Bit-Flip Scavenger:** Automatically try "Byte Swapping" on the selection to see if it fixes skewed graphics (detecting endianness errors).