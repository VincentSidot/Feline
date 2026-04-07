/* Libs */
use egui::Window;

/* Locals */
use feline_render::AppExt;

pub struct FelineUi {
    open: bool,
}

impl Default for FelineUi {
    fn default() -> Self {
        Self { open: true }
    }
}

impl AppExt for FelineUi {
    fn render(&mut self, ctx: &egui::Context) -> feline_render::ApplicationRenderRet {
        let mut open = self.open;

        Window::new("Feline UI")
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label("Hello, Feline UI!");
            })
            .map(|r| {
                self.open = open;
                r.response
            })
    }

    fn should_close(&self) -> bool {
        !self.open
    }
}
