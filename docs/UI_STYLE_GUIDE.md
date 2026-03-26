# 🎨 UI/UX Style Guide: The "Cyberpunk" Bible

**Project:** Silicon Engine  
**Version:** 1.0  
**Design Philosophy:** Industrial Forensic Minimalism. The interface should feel like high-performance laboratory equipment used in a near-future setting—dark, data-dense, and glowing with purpose.

---

## 1. Color Palette (The "Neon-on-Carbon" Scheme)

To maintain the aesthetic from the mock-up, we use a deep neutral base with high-chroma accent colors to categorize data types.

| Element | Hex Code | Usage |
| :--- | :--- | :--- |
| **Primary Background** | `#0A0A0B` | Main window and panel backgrounds. |
| **Panel Surface** | `#141517` | Secondary containers (Hex view, Sidebar). |
| **Border / Stroke** | `#2D2F33` | Panel outlines and separator lines. |
| **Accent Primary** | `#FF4500` | **Safety Orange**: Highlights, selections, and primary buttons. |
| **Accent Secondary** | `#00FFFF` | **Cyan**: Address offsets and system status indicators. |
| **System Blue** | `#0055FF` | Sega-specific badges and "High Entropy" code markers. |
| **System Red** | `#E60012` | Nintendo-specific badges and "Compressed Data" alerts. |

---

## 2. Typography

The font choice is critical for the "Forensic" feel. We prioritize monospaced fonts to ensure hex data aligns perfectly.

* **Primary Data Font:** `JetBrains Mono` or `Fira Code`.
    * *Usage:* Hex grid, address offsets, and terminal output.
* **Interface Font:** `Inter` or `Roboto Condensed`.
    * *Usage:* Buttons, labels, and panel headers.
* **Header Badge Font:** `Orbitron` (Optional/Sparse usage).
    * *Usage:* "SILICON ENGINE" logo and "SYSTEM DETECTED" badges.

---

## 3. Component Layout & Spacing

Based on the mock-up, the layout follows a **Three-Column Laboratory Grid**:

### 3.1. The Header (Status Bar)
* **Left:** "SEGA MEGA DRIVE DETECTED" badge (Sleek, trapezoidal framing).
* **Center:** Silicon Engine Logo (Centered, glowing).
* **Right:** ROM Metadata (Name, Size, Region).

### 3.2. Left Panel: The Radar (Entropy Ribbon)
* Width: 10% of window width.
* A vertical, continuous heatmap.
* **UX Interaction:** Hovering over the ribbon displays a tooltip of the exact offset; clicking jumps the Hex Viewer to that location.

### 3.3. Center Panel: The Nerve Center (Hex Viewer)
* Width: 60% of window width.
* **Layout:** Offset column (Cyan text) | Hex columns (8x2 block) | ASCII representation.
* **Styling:** Zebra-striping every 4 lines with a 2% opacity difference for readability.

### 3.4. Right Panel: The Visualizer (Tile Scraper)
* Width: 30% of window width.
* **Layout:** A grid of reconstructed tiles.
* **Control Bar:** A footer with "BPP" sliders and "Palette" selection buttons.

---

## 4. Visual Effects & Shaders

To achieve the "Cyberpunk" look in Rust/egui, we apply the following:

* **Glow (Bloom):** Accent colors (Orange/Cyan) should have a soft 2px outer glow.
* **Glassmorphism:** Use slight transparency (`alpha: 0.95`) on panels with a 5px background blur.
* **Scanlines:** A very faint horizontal overlay (1px height, 10% opacity) across the Tile Scraper to mimic CRT hardware.
* **Beveled Edges:** Panels should not have rounded corners (radius: 0), but instead use 45-degree "clipped" corners for an industrial feel.

---

## 5. State-Based Theming

The UI "Adapts" to the hardware it identifies:

1.  **Sega Mode:** Accent secondary shifts to `#0055FF`. Panel headers show "M68000 ARCHITECTURE."
2.  **Nintendo Mode:** Accent secondary shifts to `#E60012`. Panel headers show "GBA / ARM7 ARCHITECTURE."
3.  **Error/Alert State:** All orange accents pulse at `1Hz` frequency until the error is dismissed.

---

## 6. Iconography Style

* **Weight:** Thin (2px stroke).
* **Style:** Linear, non-filled.
* **Interaction:** Icons should "light up" (increase brightness/glow) when the mouse hovers over them.