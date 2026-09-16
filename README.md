# Kestrel 🐦

A sleek, fast, and lightweight systems utility written in Rust. **Kestrel** combines a beautiful, structured directory lister (supporting both customizable tables and structured JSON outputs) with a real-time, lightweight CPU and RAM terminal monitor. 

This project was built to explore Rust systems programming, terminal formatting, cross-platform terminal control, and serialization pipelines.

Kestrel is now expanding into a fully custom terminal emulator — 
rendering text pixel by pixel with no GPU or GUI framework.

---

## 🚀 Features

- **🎨 High-Aesthetic Directory Lister (`kes ls`)**: Generates colorful, structured directory lists using true-color styling and rounded border grids.
- **⚡ Fast JSON Export (`kes ls --json`)**: Instantly serializes directory contents into machine-readable JSON formats for automation pipelines.
- **📊 Real-Time Resource Monitor (`kes monitor`)**: Pulls system hardware state to output active CPU usage percentages and memory metrics dynamically.
- **🛡️ Operating System Agnostic**: Programmed defensively using safe filesystem abstractions to ensure predictable behavior across Windows and Unix platforms.

---

## 🛠️ Built With

*   [**clap**](https://crates.io/crates/clap) (v4) - Declarative, type-safe command-line argument parsing and help-menu generation.
*   [**tabled**](https://crates.io/crates/tabled) - Beautiful, highly customizable console tables.
*   [**serde** & **serde_json**](https://crates.io/crates/serde) - High-performance, zero-copy serialization engine.
*   [**sysinfo**](https://crates.io/crates/sysinfo) - Cross-platform hardware diagnostics and resource polling.
*   [**colored**](https://crates.io/crates/colored) & [**crossterm**](https://crates.io/crates/crossterm) - Rich terminal colors, text formatting, and cross-platform terminal screen manipulation.
*   [**winit**](https://crates.io/crates/winit) - Cross-platform window creation.
*   [**softbuffer**](https://crates.io/crates/softbuffer) - CPU-side pixel buffer rendering.
*   [**fontdue**](https://crates.io/crates/fontdue) - Pure Rust font rasterizer.
---

## 📦 Installation & Getting Started

### Prerequisites
Make sure you have the Rust compiler and Cargo installed on your system.

### Build from Source
1. Clone this repository:
   ```bash
   git clone https://github.com/YOUR_USERNAME/kestrel.git
   cd kestrel
   ```
2. Build the release binary:
   ```bash
   cargo build --release
   ```
3. Run the lister:
   ```bash
   cargo run -- ls
   ```
4. Run the live resource monitor:
   ```bash
   cargo run -- monitor
   ```

---

## 📖 Roadmap & Future Extensions

Kestrel is designed with an extensible, modular architecture. Upcoming features include:
- [x] Custom window renderer (winit + softbuffer)
- [x] Software font rasterization (fontdue, no GPU)
- [x] Text rendering with baseline correction
- [ ] Keyboard input + blinking cursor
- [ ] Line buffer + scrollback
- [ ] Shell backend (spawn process, pipe stdout)
- [ ] Full interactive terminal emulator
