mod theme;
mod widgets;

/* Libs */
use egui::{ComboBox, Grid, RichText, ScrollArea, Slider, TextEdit, TextStyle, Ui, Window, vec2};

/* Locals */
use feline_render::AppExt;
use theme::Palette;
use widgets::{
    ElectronCloudState, capability, card, electron_cloud, metric, mini_chart, pill, themed_button,
};

pub struct FelineUi {
    open: bool,
    energy: f32,
    latency: f32,
    density: f32,
    glow: bool,
    diagnostics: bool,
    selected_tab: DemoTab,
    command: String,
    electron_cloud: ElectronCloudState,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DemoTab {
    Overview,
    Controls,
    Console,
}

impl Default for FelineUi {
    fn default() -> Self {
        Self {
            open: true,
            energy: 0.72,
            latency: 0.28,
            density: 0.64,
            glow: true,
            diagnostics: true,
            selected_tab: DemoTab::Overview,
            command: "scan --overlay --theme feline".to_owned(),
            electron_cloud: ElectronCloudState::default(),
        }
    }
}

impl FelineUi {
    fn render_content(&mut self, ui: &mut Ui) {
        self.hero(ui);
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            self.tab(ui, DemoTab::Overview, "Overview");
            self.tab(ui, DemoTab::Controls, "Controls");
            self.tab(ui, DemoTab::Console, "Console");
            ui.label(
                RichText::new("transparent overlay UI")
                    .color(Palette::MUTED)
                    .small(),
            );
        });

        ui.add_space(8.0);

        match self.selected_tab {
            DemoTab::Overview => self.overview(ui),
            DemoTab::Controls => self.controls(ui),
            DemoTab::Console => self.console(ui),
        }
    }

    fn hero(&self, ui: &mut Ui) {
        theme::hero_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Feline Control Surface")
                            .heading()
                            .strong()
                            .color(Palette::WHITE),
                    );
                    ui.label(
                        RichText::new("glass panels, styled widgets, live layout, custom painting")
                            .color(Palette::MUTED),
                    );
                });

                pill(ui, "GPU READY", Palette::GREEN);
                pill(ui, "EGUI THEMED", Palette::CYAN);
            });
        });
    }

    fn overview(&mut self, ui: &mut Ui) {
        ui.columns(2, |columns| {
            card(&mut columns[0], "Signal", |ui| {
                metric(ui, "Energy", self.energy, Palette::CYAN);
                metric(ui, "Latency", self.latency, Palette::VIOLET);
                metric(ui, "Density", self.density, Palette::GREEN);
            });

            card(&mut columns[1], "Electron Cloud", |ui| {
                electron_cloud(ui, &mut self.electron_cloud, self.energy);
                ui.label(
                    RichText::new("Drag to rotate. Hover to excite the orbital glow.")
                        .color(Palette::MUTED),
                );
            });
        });

        ui.add_space(10.0);

        card(ui, "Render Pulse", |ui| {
            mini_chart(ui, self.energy, self.latency, self.density);
            ui.add_space(8.0);
            ui.label(
                RichText::new("Custom painter output inside a normal egui card.")
                    .color(Palette::MUTED),
            );
        });

        ui.add_space(10.0);

        card(ui, "Capability Preview", |ui| {
            Grid::new("capabilities")
                .num_columns(3)
                .spacing(vec2(16.0, 8.0))
                .striped(true)
                .show(ui, |ui| {
                    capability(ui, "Window chrome", "built-in close control", Palette::CYAN);
                    ui.end_row();
                    capability(ui, "Cards", "nested frames and strokes", Palette::VIOLET);
                    ui.end_row();
                    capability(ui, "State", "live sliders and toggles", Palette::GREEN);
                    ui.end_row();
                    capability(
                        ui,
                        "Canvas",
                        "manual painting in allocated rects",
                        Palette::AMBER,
                    );
                    ui.end_row();
                });
        });
    }

    fn controls(&mut self, ui: &mut Ui) {
        ui.columns(2, |columns| {
            card(&mut columns[0], "Theme Controls", |ui| {
                ui.add(
                    Slider::new(&mut self.energy, 0.0..=1.0)
                        .text("energy")
                        .trailing_fill(true),
                );
                ui.add(
                    Slider::new(&mut self.latency, 0.0..=1.0)
                        .text("latency")
                        .trailing_fill(true),
                );
                ui.add(
                    Slider::new(&mut self.density, 0.0..=1.0)
                        .text("density")
                        .trailing_fill(true),
                );
                ui.separator();
                ui.checkbox(&mut self.glow, "enable glow accents");
                ui.checkbox(&mut self.diagnostics, "show diagnostics");
            });

            card(&mut columns[1], "Action Stack", |ui| {
                ui.horizontal_wrapped(|ui| {
                    themed_button(ui, "Prime", Palette::CYAN);
                    themed_button(ui, "Stabilize", Palette::VIOLET);
                    themed_button(ui, "Deploy", Palette::GREEN);
                });

                ComboBox::from_label("profile")
                    .selected_text(if self.glow {
                        "neon glass"
                    } else {
                        "quiet glass"
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.glow, true, "neon glass");
                        ui.selectable_value(&mut self.glow, false, "quiet glass");
                    });

                if self.diagnostics {
                    ui.separator();
                    ui.monospace("diagnostics: frame graph stable, pointer hit-test active");
                }
            });
        });
    }

    fn console(&mut self, ui: &mut Ui) {
        card(ui, "Command Console", |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(">").color(Palette::CYAN).monospace());
                let width = widgets::finite_width(ui, 260.0);
                ui.add(
                    TextEdit::singleline(&mut self.command)
                        .desired_width(width)
                        .font(TextStyle::Monospace),
                );
            });

            ui.add_space(8.0);

            ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                for (status, message, color) in [
                    (
                        "ok",
                        "theme style applied through egui::Style and Visuals",
                        Palette::GREEN,
                    ),
                    (
                        "ok",
                        "widgets inherit custom inactive/hovered/active states",
                        Palette::GREEN,
                    ),
                    (
                        "ui",
                        "manual painter chart rendered inside a card frame",
                        Palette::CYAN,
                    ),
                    (
                        "sys",
                        "window close decoration is owned by egui::Window::open",
                        Palette::VIOLET,
                    ),
                    (
                        "warn",
                        "this is still immediate-mode UI, so state belongs here",
                        Palette::AMBER,
                    ),
                ] {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("[{status}]"))
                                .monospace()
                                .color(color),
                        );
                        ui.label(RichText::new(message).monospace().color(Palette::MUTED));
                    });
                }
            });
        });
    }

    fn tab(&mut self, ui: &mut Ui, tab: DemoTab, label: &str) {
        let selected = self.selected_tab == tab;

        if ui.add(theme::tab_button(label, selected)).clicked() {
            self.selected_tab = tab;
        }
    }
}

impl AppExt for FelineUi {
    fn render(&mut self, ctx: &egui::Context) -> feline_render::ApplicationRenderRet {
        theme::apply(ctx);
        ctx.request_repaint();

        let mut open = self.open;
        let response = Window::new("Feline UI")
            .open(&mut open)
            .collapsible(true)
            .resizable(true)
            .default_width(760.0)
            .frame(theme::window_frame())
            .show(ctx, |ui| self.render_content(ui));

        self.open = open;
        response.map(|r| r.response)
    }

    fn should_close(&self) -> bool {
        !self.open
    }
}
