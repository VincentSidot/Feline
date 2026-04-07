mod model;
mod theme;
mod widgets;

/* Libs */
use egui::Window;

/* Locals */
use feline_render::AppExt;
use model::{ChatMessage, canned_reply, demo_messages};

pub struct FelineUi {
    open: bool,
    messages: Vec<ChatMessage>,
    draft: String,
}

impl Default for FelineUi {
    fn default() -> Self {
        Self {
            open: true,
            messages: demo_messages(),
            draft: String::new(),
        }
    }
}

impl FelineUi {
    fn render_chat(&mut self, ui: &mut egui::Ui) {
        widgets::status_bar(ui, self.messages.len());
        ui.add_space(10.0);
        widgets::message_list(ui, &self.messages);
        ui.add_space(8.0);

        if widgets::composer(ui, &mut self.draft) {
            self.send_draft();
        }
    }

    fn send_draft(&mut self) {
        let text = self.draft.trim();
        if text.is_empty() {
            return;
        }

        self.messages.push(ChatMessage::user(text));
        self.messages
            .push(ChatMessage::assistant(canned_reply(text)));
        self.draft.clear();
    }
}

impl AppExt for FelineUi {
    fn render(&mut self, ctx: &egui::Context) -> feline_render::ApplicationRenderRet {
        theme::apply(ctx);

        let mut open = self.open;
        let response = Window::new("Feline UI")
            .open(&mut open)
            .collapsible(true)
            .resizable(true)
            .min_width(widgets::CHAT_WIDTH)
            .max_width(widgets::CHAT_WIDTH)
            .default_width(widgets::CHAT_WIDTH)
            .min_height(460.0)
            .default_height(620.0)
            .frame(theme::window_frame())
            .show(ctx, |ui| {
                ui.set_min_width(widgets::CHAT_CONTENT_WIDTH);
                ui.set_max_width(widgets::CHAT_CONTENT_WIDTH);
                self.render_chat(ui);
            });

        self.open = open;
        response.map(|r| r.response)
    }

    fn should_close(&self) -> bool {
        !self.open
    }
}
