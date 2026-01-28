# Chalante

**The Efficient Approach to AI.** A high-performance terminal chat interface powered by **Rust** and **llama.cpp**. Chalante eliminates browser bloat, giving your RAM back to the models that matter.

## Quick Start

### 1. Prerequisites

Ensure you have the following installed on your system:

* **Rust** (cargo, rustc)
* **CMake** (required to build the inference engine)
* **Git**

### 2. Build & Run

Clone the repository and launch the application:

```bash
git clone https://github.com/your-username/chalante
cd chalante
cargo run

```

---

## Setup Instructions (Inside the App)

Once the app is running, follow these steps in the **Config Page** to initialize your local AI:

1. **Install Engine**: Type `install engine` + `[ENTER]`. This clones and builds `llama.cpp` using CMake.
2. **Install Models**: Type `install models` + `[ENTER]`. This runs the internal script to fetch supported models (Phi2, Qwen, Danube).
3. **Start Server**: Type `start server` + `[ENTER]`. This initializes the local inference server on port `11343`.
4. **Load Model**: Type `load model` + `[ENTER]` to move the model into your VRAM/RAM.
5. **Go to Chat**: Type `go chat` + `[ENTER]` to start the conversation.

---

## ⌨️ Controls

| Key | Action |
| --- | --- |
| `[ENTER]` | Execute command / Send message |
| `[ESC]` | Exit application |
| `[UP/DOWN]` | Navigate model list (Chat Screen) |
| `BACKSPACE` | Delete text |

---

## Features

* **Zero Browser Overload**: Native terminal UI using `ratatui`.
* **Automated Tooling**: Built-in engine compilation and model management.
* **Local-First**: Complete privacy and performance by running 100% on your hardware.

**Version:** 0.1.0 (Stable)

**Date:** 28/01/2026

---