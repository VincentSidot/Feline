mod utils;

/* Libs */
use anyhow::Result;
use feline_render::{App, ApplicationRenderRet, WinitApplication, egui};

#[derive(Default)]
struct MyApp {}

impl App for MyApp {
    fn render(&mut self, ctx: &egui::Context) -> ApplicationRenderRet {
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
