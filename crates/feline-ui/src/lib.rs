/* Libs */
use egui::{ViewportCommand, Window};

/* Locals */
use feline_render::AppExt;

#[derive(Default)]
pub struct FelineUi {}

impl AppExt for FelineUi {
    fn render(&mut self, ctx: &egui::Context) -> feline_render::ApplicationRenderRet {
        Window::new("Feline UI")
            .show(ctx, |ui| {
                ui.label("Hello, Feline UI!");

                if ui.button("Close").clicked() {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            })
            .map(|r| r.response)
    }
}
