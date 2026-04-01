//! Native application and event loop.
//!
//! Integrates the kernel with winit event loop and wgpu rendering.

use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop, ControlFlow};
use winit::window::{Window, WindowId};
use vzglyd_kernel::{Host, RenderCommand, LogLevel, Engine, EngineInput};

use crate::gpu::GpuContext;
use crate::wasm::WasmRuntime;

/// Native application state.
pub struct NativeApp {
    context: Option<GpuContext>,
    engine: Option<Engine>,  // Use Option to allow taking it out
    #[allow(dead_code)]
    wasm_runtime: WasmRuntime,
    window: Option<Arc<Window>>,
    last_frame: Option<Instant>,
    running: bool,
}

/// Temporary host wrapper that avoids borrow issues.
struct HostWrapper<'a> {
    app: &'a mut NativeApp,
}

impl<'a> Host for HostWrapper<'a> {
    fn request_data(&mut self, key: &str) -> Option<Vec<u8>> {
        std::fs::read(key).ok()
    }

    fn submit_render_commands(&mut self, _cmds: &[RenderCommand]) {
        // Stub - full implementation would execute wgpu commands
    }

    fn log(&mut self, level: LogLevel, msg: &str) {
        match level {
            LogLevel::Debug => log::debug!("{}", msg),
            LogLevel::Info => log::info!("{}", msg),
            LogLevel::Warn => log::warn!("{}", msg),
            LogLevel::Error => log::error!("{}", msg),
        }
    }

    fn now(&self) -> f32 {
        self.app.last_frame
            .map(|i| i.elapsed().as_secs_f32())
            .unwrap_or(0.0)
    }
}

impl NativeApp {
    /// Creates a new native application.
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            context: None,
            engine: Some(Engine::new()),
            wasm_runtime: WasmRuntime::new()?,
            window: None,
            last_frame: None,
            running: false,
        })
    }

    /// Runs the application.
    pub fn run() -> Result<(), String> {
        let event_loop = EventLoop::new()
            .map_err(|e| format!("Failed to create event loop: {}", e))?;

        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = Self::new()?;
        
        // Initialize engine with host wrapper
        if let Some(mut engine) = app.engine.take() {
            let mut host = HostWrapper { app: &mut app };
            engine.init(&mut host);
            app.engine = Some(engine);
        }

        event_loop.run_app(&mut app)
            .map_err(|e| format!("Event loop error: {}", e))?;

        Ok(())
    }
}

impl ApplicationHandler for NativeApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window_attrs = Window::default_attributes()
            .with_title("VZGLYD")
            .with_inner_size(winit::dpi::LogicalSize::new(640.0, 480.0));

        let window = event_loop.create_window(window_attrs)
            .expect("Failed to create window");

        let window = Arc::new(window);
        
        // Create GPU context
        let context = pollster::block_on(GpuContext::new(window.clone()))
            .expect("Failed to create GPU context");

        self.context = Some(context);
        self.window = Some(window);
        self.running = true;

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.running = false;
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(context) = &mut self.context {
                    context.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if !self.running {
                    return;
                }

                // Calculate delta time
                let now = Instant::now();
                let dt = if let Some(last) = self.last_frame {
                    now.duration_since(last).as_secs_f32()
                } else {
                    1.0 / 60.0
                };
                self.last_frame = Some(now);

                // Update engine using host wrapper to avoid borrow issues
                if let Some(mut engine) = self.engine.take() {
                    let mut host = HostWrapper { app: self };
                    let input = EngineInput {
                        dt,
                        events: vec![],
                    };
                    let _output = engine.update(&mut host, input);
                    self.engine = Some(engine);
                }

                // Request next frame
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}
