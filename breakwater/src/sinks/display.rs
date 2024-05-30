use crate::FrameBuffer;
use std::sync::Arc;
use winit::{application::ApplicationHandler, event_loop::{self, EventLoop}, window::{Window, WindowAttributes}};

pub struct DisplaySink {
    fb: Arc<FrameBuffer>,
    state: DisplayApp,
}

impl DisplaySink {
    pub fn new(fb: Arc<FrameBuffer>) -> Self {
        let state = DisplayApp::default();
        DisplaySink {
            fb,
            state,
        }
    }

    pub async fn run(mut self) {
        let event_loop = EventLoop::builder().build().unwrap();
        event_loop.run_app(&mut self.state).unwrap();
    }
}

#[derive(Default)]
struct DisplayApp {
    window: Option<Window>
}

impl ApplicationHandler for DisplayApp {
    fn window_event(
            &mut self,
            event_loop: &event_loop::ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        let Some(window) = self.window.as_ref() else {
            return;
        };

        match event {
            _ => {
                log::debug!("Window={:?}", event);
            }
        };
    }

    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        self.window = Some(event_loop.create_window(WindowAttributes::default()).unwrap());
    }

}
