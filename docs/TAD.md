# 🏗️ Technical Architecture Document: Silicon Engine

**Project:** Silicon Engine  
**Version:** 1.0  
**Architectural Pattern:** Hybrid Rust-Frontend / C++-Core (Clean Bridge)

---

## 1. System Overview
Silicon Engine is divided into three distinct layers to balance high-performance binary processing with a modern, safe user interface:
1.  **The Application Layer (Rust/egui):** Manages the high-performance UI loop, user input, state management, and top-level orchestration.
2.  **The FFI Bridge (C-Interface):** A simplified, flat communication layer that passes pointers, handles, and primitives between Rust and C++.
3.  **The Forensic Core (C++):** The "Engine Room" where heavy binary manipulation, entropy calculation, and platform-specific hardware parsing occur.

---

## 2. The Core Architecture (C++)

### 2.1. The System Factory Pattern
To ensure modularity and scalability for the **Developer Edition**, the engine utilizes a Factory pattern for ROM identification and parsing.

* **`AbstractSystem` (Base Class):** Defines the interface for all console modules (e.g., `identify()`, `get_header_info()`, `get_memory_map()`).
* **`SystemFactory`:** A registry class. When a ROM is loaded, the Factory iterates through all registered system modules. The first system to return `true` on `identify()` becomes the active handler.
* **Implementation Modules:** (e.g., `SegaGenesisSystem.cpp`, `GameboySystem.cpp`) are independent units that can be added or removed without modifying the core factory logic.

### 2.2. Virtual File System (VFS)
Since cartridges lack a traditional file system, the core implements a **Logical VFS**:
* **Carving Engine:** Identifies data boundaries based on system-specific headers and entropy transitions.
* **VFS Nodes:** Represents logical blocks (e.g., "Vector Table," "Main Code," "Tile Bank 0") as browseable files in the UI.

### 2.3. Memory Mapping (Large File Handling)
To handle CD-ROM based systems (PS1/Dreamcast) without massive RAM overhead:
* The engine utilizes **Memory-Mapped Files** (`mmap` on POSIX / `CreateFileMapping` on Windows). 
* This allows the operating system to manage the buffer, enabling Silicon Engine to point to any byte in a 700MB ISO instantly with zero loading latency.

---

## 3. The Communication Layer (FFI)

The FFI Bridge remains strictly C-compatible (`extern "C"`) to ensure zero-overhead calls from Rust.

| Function | Purpose | Data Type |
| :--- | :--- | :--- |
| `engine_init()` | Instantiate the C++ Core | Returns `*mut c_void` (Opaque Handle) |
| `engine_load_rom()` | Trigger Factory Identification | Input: `*const c_char` (Path) |
| `get_entropy_map()` | Retrieve 1D Heatmap data | Returns: `*const f32` (Buffer) |
| `get_vfs_json()` | Get virtual file structure | Returns: `*const c_char` (JSON String) |

---

## 4. The UI Engine (Rust)

### 4.1. Immediate Mode UI (egui)
The UI is built on the `egui` framework to ensure the interface remains fluid while scrolling through millions of hex lines.
* **Virtualized Scroller:** A custom implementation ensures that only the visible 30–50 rows of Hex data are ever calculated or rendered at once, keeping CPU usage near zero during idle.

### 4.2. Custom Painter Widgets
* **Entropy Ribbon:** A custom widget using `egui::Painter` to draw a vertical texture representing data randomness. Color values are fetched directly from the C++ Core's Shannon Entropy buffer.
* **Tile Scraper:** A dynamic `Texture` buffer. As the user moves the cursor, the C++ Core feeds raw pixel data (based on active BPP settings) into this texture for real-time visualization.

---

## 5. Forensic Logic (The "Pro" Algorithm)

The **Asset Scraper** utilizes a "Heuristic Pipeline":
1.  **Signaling:** C++ scans current offsets for known binary headers or magic numbers.
2.  **Entropy Analysis:** If a selection has "Medium Entropy" ($3.0 < H < 5.0$), it is flagged as potential Graphics data.
3.  **Visual Proofing:** The engine attempts a "Speculative Render" in 2bpp, 4bpp, and 8bpp formats.

---

## 6. Build & Deployment
* **Compiler:** `cargo` (Rust) + `cc` crate (managing the C++ toolchain).
* **Static Linking:** The C++ core is compiled into a static library (`.lib` or `.a`) and linked into the Rust binary to produce a single, portable executable.
* **LTO:** Link-Time Optimization is enabled for release builds to optimize the cross-language function calls.

---

## 7. Security
* **Rust Guardrails:** All FFI calls are wrapped in safe Rust wrappers to prevent null-pointer dereferences from the C++ side from crashing the UI.
* **Boundary Checks:** The C++ core performs strict bounds-checking on the memory-mapped buffer to prevent buffer overflows during data carving.