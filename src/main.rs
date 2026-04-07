/* Models */
mod app;
mod constants;
mod platform;
mod state;
mod ui;
mod utils;
mod window;

/* Libs */
use anyhow::Result;

/* Locals */
use window::WinitApplication;

use crate::app::App;

#[derive(Default)]
struct MyApp {}

impl App for MyApp {
    fn render(&mut self, ctx: &egui::Context) -> app::ApplicationRenderRet {
        egui::Window::new("Test")
            .show(ctx, |ui| {
                ui.label("Hello, EGUI!");
                if ui.button("Click me").clicked() {
                    log::info!("Button clicked!");
                }
                if ui.button("Quit").clicked() {
                    log::info!("Quit button clicked, exiting...");
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            })
            .map(|r| r.response)
    }
}

fn main() -> Result<()> {
    utils::logger::init();
    let mut app = WinitApplication::default();
    app.register_default::<MyApp>();
    app.run()
}
