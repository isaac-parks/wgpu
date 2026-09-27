use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ ActiveEventLoop, EventLoop },
    keyboard::{ KeyCode, PhysicalKey },
    window::Window,
};

#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub struct State {
    window: Arc<Window>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        Ok(Self { window })
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {}

    pub fn render(&mut self) {
        self.window.request_redraw();
    }
}

pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "canvas";

            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let html_canvas_element = canvas.get_element_by_id(CANVAS_ID).unwrap_throw();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.state = Some(pollster::block_on(State::new(window)).unwrap());
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(
                        proxy
                            .send_event(State::new(window).await.expect("Unable to create canvas"))
                            .is_ok()
                    )
                });
            }
        }
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => {
                return;
            }
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => state.render(),
            WindowEvent::KeyboardInput {
                event: KeyEvent { physical_key: PhysicalKey::Code(code), state: key_state, .. },
                ..
            } =>
                match (code, key_state.is_pressed()) {
                    (KeyCode::Escape, true) => event_loop.exit(),
                    _ => {}
                }
            _ => {}
        }
    }
}
