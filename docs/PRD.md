# 📑 Product Requirements Document: Silicon Engine

**Project Name:** Silicon Engine  
**Version:** 1.0  
**Status:** Planning / Architecture Phase  
**Target Platforms:** Windows (Primary), Linux, macOS  
**Core Technologies:** Rust (`egui`), C++ (Forensic Core), FFI (Bridge)

---

## 1. Executive Summary
**Silicon Engine** is a universal, forensic-grade ROM analysis and data-mapping workbench. Utilizing a hybrid architecture of high-performance C++ and memory-safe Rust, it provides deep binary insights into retro game cartridges. Unlike standard hex editors, Silicon Engine employs **Entropy Visualization** and **Heuristic Pattern Recognition** to navigate internal structures across multiple console generations (8-bit to 32-bit).

---

## 2. Product Goals & Objectives
* **Visual Forensic Mapping:** Transform flat binary data into a navigable "Data Landscape" via the Entropy Ribbon.
* **Automated Asset Carving:** Detect and extract graphics (tiles), palettes, and code blocks without manual offset hunting.
* **System Agnosticism:** Provide a modular "System Factory" that supports Sega, Nintendo, and Sony hardware through a unified interface.
* **Performance at Scale:** Maintain 60 FPS UI responsiveness even when memory-mapping large CD-ROM images (700MB+).
* **Commercial Viability:** Create a clear feature-tier separation for Lite, Pro, and Developer editions.

---

## 3. User Personas
| Persona | Goal | Primary Feature Used |
| :--- | :--- | :--- |
| **The Preservationist** | Verify 1:1 clean dumps and analyze "bad" dumps. | Checksum & Header Validation. |
| **The Fan Translator** | Locate text strings and font tiles for localization. | Asset Scraper & VFS Tree. |
| **The ROM Hacker** | Extract and modify sprites, code, or level data. | Asset Carving & Hex Editing. |
| **The Tool Developer** | Build custom tools for rare or proprietary hardware. | C++ System Factory (Developer Edition). |

---

## 4. Functional Requirements

### 4.1. Core Engine (C++ Backend)
* **FR-101: System Identification:** Auto-detect console type via header "Magic Numbers" (e.g., `TMR SEGA`).
* **FR-102: Endianness Normalization:** Automatically handle Big-Endian (68000) vs. Little-Endian (Z80/ARM) data.
* **FR-103: Entropy Scraper:** Implement a sliding-window Shannon Entropy algorithm to identify data types.
* **FR-104: Virtual File System (VFS):** Map logical regions (Headers, Code, Graphics) into a browseable file tree.

### 4.2. User Interface (Rust/egui)
* **FR-201: Entropy Ribbon:** A vertical heatmap in the left panel representing the entire ROM address space.
* **FR-202: Virtualized Hex Grid:** A high-speed editor rendering only visible lines to save CPU/GPU cycles.
* **FR-203: Live Tile Scraper:** A 2D render window in the right panel interpreting hex as tiles (2bpp, 4bpp, 8bpp).
* **FR-204: Multi-Tab Side Panel:** Toggle between the Entropy Ribbon and the VFS File Viewer.

### 4.3. Forensic & Extraction (Pro Features)
* **FR-301: Smart Boundary Snap:** Automatically align selections to system-specific tile boundaries (e.g., 32-byte).
* **FR-302: Palette Hunter:** Automated scanning for valid color ramps and 16/256 color arrays.
* **FR-303: Asset Extraction:** Export carved data directly to `.png`, `.pal`, or `.bin`.

---

## 5. Non-Functional Requirements
* **NFR-01: Portability:** The application must be a single, portable executable with statically linked dependencies.
* **NFR-02: Efficiency:** Utilize memory-mapping to handle large files with a RAM footprint < 200MB.
* **NFR-03: Security:** Wrap all C++ FFI calls in safe Rust abstractions to prevent memory-related crashes.

---

## 6. Tiered Feature Matrix
| Feature | Lite ($15) | Pro ($45) | Dev ($199) |
| :--- | :---: | :---: | :---: |
| System Identification | ✅ | ✅ | ✅ |
| Hex & Entropy Viewing | ✅ | ✅ | ✅ |
| Asset Extraction (PNG) | ❌ | ✅ | ✅ |
| Manual Checksum Fixing | ❌ | ✅ | ✅ |
| Source Code Access | ❌ | ❌ | ✅ |
| SDK / System Plugin API | ❌ | ❌ | ✅ |

---

## 7. Success Metrics
* **Time to Identification:** System type identified in < 100ms upon file drop.
* **UI Performance:** Zero input lag on the Entropy Ribbon during vertical scrolling.
* **Accuracy:** > 95% detection rate for standard retail ROM headers.