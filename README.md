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
# Run with a specific slide
cargo run -- --scene slides/flat

# Run with a slides directory
cargo run -- --slides-dir slides/
```

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
