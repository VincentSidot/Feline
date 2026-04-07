use anyhow::{Error, Result};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{WindowAttributes, WindowId, WindowLevel},
};

use crate::{app::AppExt, constants, platform::OverlayWindowPlatformExt, state::State};

const APP_NAME: &str = "Feline";

#[derive(Default)]
pub struct WinitApplication {
    state: Option<State>,
    error: Option<Error>,
    apps: Vec<Box<dyn AppExt>>,
}

impl WinitApplication {
    fn state_mut(&mut self) -> &mut State {
        self.state.as_mut().unwrap()
    }

    fn error(&mut self) -> Result<()> {
        if let Some(error) = self.error.take() {
            Err(error)
        } else {
            Ok(())
        }
    }

    pub fn run(mut self) -> Result<()> {
        let event_loop = EventLoop::new()?;

        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)?;

        self.error()?;

        Ok(())
    }

    pub fn register<A: AppExt + 'static>(&mut self, app: A) {
        self.apps.push(Box::new(app));
    }

    pub fn register_default<A: AppExt + Default + 'static>(&mut self) {
        let app = A::default();
        self.register(app);
    }
}

macro_rules! _try {
    ($self:expr, $exp:expr) => {
        match $exp {
            Ok(value) => value,
            Err(e) => {
                $self.error = Some(e.into());
                return;
            }
        }
    };
}

impl ApplicationHandler for WinitApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attr = WindowAttributes::default()
            .with_title(APP_NAME)
            .with_transparent(true)
            .with_decorations(constants::window::DECORATIONS)
            .with_resizable(false)
            .with_window_level(WindowLevel::AlwaysOnTop);

        let window = Arc::new(_try!(self, event_loop.create_window(attr)));
        window.configure_overlay_window();

        let monitor = _try!(
            self,
            window
                .current_monitor()
                .ok_or_else(|| anyhow::anyhow!("Failed to get current monitor for window"))
        );

        window.set_outer_position(monitor.position());
        if window.request_inner_size(monitor.size()).is_some() {
            self.error = Some(anyhow::anyhow!("Failed to set window size"));
            return;
        }

        let runner = _try!(self, tokio::runtime::Runtime::new());

        let mut state = _try!(self, runner.block_on(async { State::init(window).await }));

        for app in std::mem::take(&mut self.apps) {
            _try!(self, state.register(app));
        }

        self.state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(app) => app,
            None => {
                log::warn!("Received window event before app initialization");
                event_loop.exit();
                return;
            }
        };

        if state.should_close() {
            log::debug!("App requested close");
            event_loop.exit();
            return;
        }

        let response = state.handle_egui_event(&event);
        if response.repaint {
            state.window().request_redraw();
        }

        match event {
            WindowEvent::CloseRequested => {
                log::debug!("Window close requested");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                state.update();

                match state.render() {
                    Ok(_) => (),
                    Err(e) => {
                        self.error = Some(e);
                        event_loop.exit();
                    }
                }
            }

            WindowEvent::Resized(size) => {
                let app = self.state_mut();
                if app.window().id() == window_id {
                    app.resize(size.width, size.height);
                }
            }

            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &mut self.state {
            state.update();
            state.window().request_redraw();
        }
    }
}
