# VZGLYD Native Host

Native (Linux/Raspberry Pi) host implementation for the VZGLYD display engine.

## Overview

This crate integrates the platform-agnostic `vzglyd-kernel` with:
- **winit** for windowing and event handling
- **wgpu** for GPU rendering
- **wasmtime** for WASM slide instantiation
- **std::fs** for asset loading

## Building

```bash
cargo build --release
```

## Running

```bash
# Run without slides (shows colored background)
cargo run

# Run with a slides directory
cargo run -- --slides-dir slides/

# Run with verbose output
cargo run -- --slides-dir slides/ -v
```

## Slides Folder

The `slides/` folder contains WebAssembly slide files that are loaded at runtime.
Each slide should be a `.wasm` file compiled for the VZGLYD ABI.

### Folder Structure

```
slides/
├── courtyard/
│   └── slide.wasm      # Courtyard visualization slide
└── beach_dog/
    └── slide.wasm      # Beach dog visualization slide
```

Slides are discovered recursively in subdirectories.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Native Host                            │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────────┐   │
│  │ winit       │  │ wasmtime     │  │ std::fs          │   │
│  │ event loop  │  │ WASM loader  │  │ asset loading    │   │
│  └─────────────┘  └──────────────┘  └──────────────────┘   │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────────┐   │
│  │ wgpu        │  │ NativeHost   │  │ RenderCommand    │   │
│  │ device/queue│  │ : Host       │  │ → wgpu execution │   │
│  └─────────────┘  └──────────────┘  └──────────────────┘   │
└────────────────────────────┬────────────────────────────────┘
                             │ implements Host trait
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                  VZGLYD Kernel                              │
│  - Engine state machine                                     │
│  - Slide scheduling                                         │
│  - Transition logic                                         │
│  - RenderCommand generation                                 │
└─────────────────────────────────────────────────────────────┘
```

## License

MIT OR Apache-2.0
