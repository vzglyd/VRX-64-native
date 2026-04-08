//! Rendering module for slides and transitions.

pub(crate) mod shader_contract;
pub mod overlay;
pub mod slide;
pub mod transition;

pub use overlay::OverlayRenderer;

pub use slide::{
    DynamicMeshBuffers, LoadedScreenSlide, LoadedSlide, LoadedWorldSlide, MeshBuffers,
    ScreenBindGroup, ScreenSlideRenderer, ScreenUniforms, SlidePipelines, SlideRenderer,
    SlideTexture, WorldBindGroup, WorldSlideRenderer, WorldUniforms, create_loaded_slide_renderer,
    create_slide_renderer, load_wasm_slide, load_wasm_slide_from_bytes,
};

pub use transition::{
    ActiveTransition, TransitionKind, TransitionRenderer, TransitionState, TransitionUniforms,
};
