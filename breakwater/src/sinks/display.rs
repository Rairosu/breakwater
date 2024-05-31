use crate::FrameBuffer;
use softbuffer::{Context, Surface};
use std::{num::NonZero, sync::Arc};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{self, EventLoop},
    raw_window_handle::{DisplayHandle, HasDisplayHandle},
    window::{Window, WindowAttributes},
};

pub struct DisplaySink {
    surface: Option<Surface<DisplayHandle<'static>, Arc<Window>>>,
    fb: Arc<FrameBuffer>,
}

impl DisplaySink {
    pub fn new(fb: Arc<FrameBuffer>) -> (EventLoop<()>, DisplaySink) {
        let event_loop = EventLoop::builder().build().unwrap();
        (event_loop, DisplaySink { surface: None, fb })
    }
    pub fn run(&mut self, event_loop: EventLoop<()>) {
        event_loop.run_app(self).unwrap();
    }
}

impl ApplicationHandler for DisplaySink {
    fn user_event(&mut self, event_loop: &event_loop::ActiveEventLoop, event: ()) {
        event_loop.exit();
    }

    fn window_event(
        &mut self,
        _event_loop: &event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(surface) = self.surface.as_mut() else {
            return;
        };

        match event {
            WindowEvent::Resized(_size) => {
                surface
                    .resize(
                        NonZero::new(self.fb.get_width() as u32).unwrap(),
                        NonZero::new(self.fb.get_height() as u32).unwrap(),
                    )
                    .unwrap();
                surface.window().request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let window = surface.window().clone();
                let mut buffer = surface.buffer_mut().unwrap();

                buffer.copy_from_slice(
                    &self
                        .fb
                        .get_buffer()
                        .iter()
                        .map(|pixel| (pixel << 8).swap_bytes())
                        .collect::<Vec<_>>(),
                );
                window.pre_present_notify();
                buffer.present().unwrap();
                window.request_redraw();
            }
            WindowEvent::CursorMoved { .. }
            | WindowEvent::CursorEntered { .. }
            | WindowEvent::CursorLeft { .. } => (),
            _ => {
                log::debug!("Window={:?}", event);
            }
        };
    }

    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default())
                .unwrap(),
        );
        self.surface = {
            let context =
                Context::new(unsafe { std::mem::transmute(event_loop.display_handle().unwrap()) })
                    .unwrap();
            Some(Surface::new(&context, window).unwrap())
        };
    }
}
