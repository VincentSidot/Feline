/* Libs */
use egui::{Align, Button, Color32, Label, Layout, RichText, ScrollArea, TextEdit, Ui, vec2};

/* Locals */
use crate::{
    model::{ChatAuthor, ChatMessage},
    theme::{self, Palette},
};

pub const CHAT_WIDTH: f32 = 430.0;
pub const CHAT_CONTENT_WIDTH: f32 = 382.0;
const MESSAGE_VIEW_HEIGHT: f32 = 444.0;

pub fn status_bar(ui: &mut Ui, message_count: usize) {
    theme::header_frame().show(ui, |ui| {
        ui.set_width(CHAT_CONTENT_WIDTH);
        ui.horizontal(|ui| {
            ui.label(RichText::new("●").color(Palette::CYAN).size(10.0));
            status_pill(ui, "local demo");
            status_pill(ui, &format!("{message_count} messages"));
            ui.label(
                RichText::new("model backend not connected")
                    .color(Palette::MUTED)
                    .small(),
            );
        });
    });
}

pub fn message_list(ui: &mut Ui, messages: &[ChatMessage]) {
    ui.allocate_ui_with_layout(
        vec2(CHAT_CONTENT_WIDTH, MESSAGE_VIEW_HEIGHT),
        *ui.layout(),
        |ui| {
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .max_height(MESSAGE_VIEW_HEIGHT)
                .show(ui, |ui| {
                    ui.set_width(CHAT_CONTENT_WIDTH);
                    for message in messages {
                        message_bubble(ui, message);
                        ui.add_space(10.0);
                    }
                });
        },
    );
}

pub fn composer(ui: &mut Ui, draft: &mut String) -> bool {
    let mut send = false;

    theme::composer_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("…").color(Palette::MUTED).size(18.0));

            let edit_width = CHAT_CONTENT_WIDTH - 96.0;
            let response = ui.add_sized(
                vec2(edit_width, 34.0),
                TextEdit::singleline(draft)
                    .hint_text("Message Feline")
                    .desired_width(edit_width),
            );

            let enter_pressed =
                response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
            let button_pressed = ui
                .add_sized(
                    vec2(58.0, 34.0),
                    Button::new(RichText::new("Send").strong().color(Palette::BACKGROUND))
                        .fill(Palette::CYAN),
                )
                .clicked();

            send = enter_pressed || button_pressed;
            if send {
                response.request_focus();
            }
        });
    });

    send
}

fn message_bubble(ui: &mut Ui, message: &ChatMessage) {
    let is_user = message.author == ChatAuthor::User;
    let available_width = CHAT_CONTENT_WIDTH;
    let bubble_width = if is_user {
        (available_width * 0.66).clamp(180.0, 270.0)
    } else {
        (available_width * 0.78).clamp(220.0, 318.0)
    };
    let body_width = (bubble_width - 24.0).max(120.0);

    ui.horizontal(|ui| {
        if is_user {
            ui.add_space((available_width - bubble_width).max(0.0));
        }

        theme::message_frame(is_user).show(ui, |ui| {
            ui.set_width(bubble_width);
            ui.set_max_width(bubble_width);
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                ui.set_width(body_width);
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                ui.label(
                    RichText::new(if is_user { "You" } else { "Feline" })
                        .small()
                        .strong()
                        .color(if is_user {
                            Palette::VIOLET
                        } else {
                            Palette::CYAN
                        }),
                );
                ui.add(
                    Label::new(RichText::new(message.text.as_str()).color(Palette::TEXT))
                        .wrap()
                        .halign(Align::Min),
                );
            });
        });
    });
}

fn status_pill(ui: &mut Ui, text: &str) {
    ui.add(
        Button::new(RichText::new(text).small().color(Palette::MUTED))
            .fill(Color32::from_rgba_premultiplied(20, 30, 48, 180)),
    );
}
