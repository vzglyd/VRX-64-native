//! Rendering module for slides and transitions.

pub(crate) mod shader_contract;
pub mod slide;
pub mod transition;

pub use slide::{
    DynamicMeshBuffers, MeshBuffers, ScreenBindGroup, ScreenSlideRenderer, ScreenUniforms,
    SlidePipelines, SlideRenderer, SlideTexture, WorldBindGroup, WorldSlideRenderer, WorldUniforms,
    LoadedSlide, LoadedScreenSlide, LoadedWorldSlide, create_loaded_slide_renderer,
    create_slide_renderer, load_wasm_slide, load_wasm_slide_from_bytes,
};

pub use transition::{
    ActiveTransition, TransitionKind, TransitionRenderer, TransitionState, TransitionUniforms,
};
