//! WASM runtime for slide instantiation.
//!
//! Handles wasmtime engine, store, and host function exports.

use wasmtime::{Engine, Module, Config};

/// WASM runtime managing the wasmtime engine.
pub struct WasmRuntime {
    pub engine: Engine,
}

impl WasmRuntime {
    /// Creates a new WASM runtime.
    pub fn new() -> Result<Self, String> {
        let mut config = Config::new();
        config.wasm_reference_types(true);
        config.epoch_interruption(true);

        let engine = Engine::new(&config)
            .map_err(|e| format!("Failed to create wasmtime engine: {}", e))?;

        Ok(Self { engine })
    }

    /// Compiles a WASM module from bytes.
    pub fn compile(&self, wasm_bytes: &[u8]) -> Result<Module, String> {
        Module::from_binary(&self.engine, wasm_bytes)
            .map_err(|e| format!("Failed to compile WASM module: {}", e))
    }
}

impl Default for WasmRuntime {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
