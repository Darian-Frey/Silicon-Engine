# 🚀 Silicon Engine: 10-Step Development Roadmap

This roadmap outlines the path from initial architectural setup to a market-ready "Developer Edition." It prioritizes building a robust C++ core and a high-performance Rust frontend.

---

### **Phase 1: Foundation & Bridge (Weeks 1-2)**

1.  **Repository Setup & FFI Handshake**
    * Initialize the Rust crate (`cargo init`) and C++ project.
    * Implement the `engine_init` and `engine_load` FFI functions.
    * **Goal:** Successfully load a raw binary file from Rust and print its size in the terminal via C++ logic.

2.  **The System Factory & Sega Module**
    * Build the `AbstractSystem` interface and `SystemFactory` in C++.
    * Implement the **Sega Mega Drive/Genesis** parser as the "Reference Module."
    * **Goal:** Auto-detect a `.bin` ROM and display the internal Game Title in the Rust UI header.

---

### **Phase 2: Visualizing the Silicon (Weeks 3-4)**

3.  **Shannon Entropy Scraper**
    * Implement the sliding-window Shannon Entropy algorithm in the C++ core.
    * Expose the entropy buffer to Rust via a shared memory pointer for zero-copy performance.
    * **Goal:** Generate a raw stream of "randomness" data for any loaded ROM file.

4.  **The Entropy Ribbon Widget**
    * Develop the custom `egui` painter widget for the left sidebar.
    * Map entropy values to the "Cyberpunk" palette (Blue=Code, Green=Tiles, Red=Compressed).
    * **Goal:** A vertical, interactive heatmap that reflects the ROM's internal data structure.

---

### **Phase 3: Deep Forensics (Weeks 5-6)**

5.  **Virtualized Hex Grid**
    * Implement the high-speed "Render Only Visible Rows" logic in Rust.
    * Add "Neon Orange" highlighting for selection and "Cyan" for address offsets.
    * **Goal:** Smooth, 60 FPS scrolling through 32MB+ ROMs without UI lag.

6.  **The Live Tile Scraper**
    * Build the 2D rendering engine in the right sidebar.
    * Implement 4bpp (Sega) and 2bpp (Gameboy) bit-planar decoding logic in C++.
    * **Goal:** Selecting a green block in the Entropy Ribbon instantly renders sprites in the visualizer.

---

### **Phase 4: Asset Extraction & VFS (Weeks 7-8)**

7.  **The Virtual File System (VFS)**
    * Implement the "Carving Engine" to identify Header and Code boundaries.
    * Build the Rust "Tab Switcher" to swap between the Ribbon and the File Tree.
    * **Goal:** Browse a ROM's internal components as if they were standard files.

8.  **Smart Asset Extraction (Pro Feature)**
    * Develop the "Snap-to-Boundary" logic (32-byte alignment for Sega tiles).
    * Implement the "Right-Click -> Export to PNG" functionality.
    * **Goal:** Save a perfectly carved sprite sheet from a raw ROM directly to the desktop.

---

### **Phase 5: Expansion & Launch (Weeks 9-10)**

9.  **Advanced Modules (GBA & PS1)**
    * Add the **Nintendo GBA** module (Header + Logo validation).
    * Implement basic **ISO-9660** parsing for the PlayStation 1 module.
    * **Goal:** Prove the "Universal" nature of the engine by supporting disc-based and cartridge systems.

10. **Product Finalization & SDK Packaging**
    * Enable **Link-Time Optimization (LTO)** and strip debug symbols for the Release build.
    * Document the `AbstractSystem` API for the **Developer Edition** source bundle.
    * **Goal:** Produce the three final binaries (Lite, Pro, Dev) ready for distribution.