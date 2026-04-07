/* Libs */
use egui::{Color32, CornerRadius, Pos2, ProgressBar, Rect, RichText, Sense, Stroke, Ui, Vec2};

/* Locals */
use crate::theme::{self, Palette};

pub fn card(ui: &mut Ui, title: &str, add_contents: impl FnOnce(&mut Ui)) {
    theme::card_frame().show(ui, |ui| {
        ui.label(RichText::new(title).strong().color(Palette::WHITE));
        ui.add_space(6.0);
        add_contents(ui);
    });
}

pub fn pill(ui: &mut Ui, text: &str, color: Color32) {
    ui.add(theme::pill_button(text, color));
}

pub fn themed_button(ui: &mut Ui, text: &str, color: Color32) {
    ui.add(theme::accent_button(text, color));
}

pub fn metric(ui: &mut Ui, label: &str, value: f32, color: Color32) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).strong());
        ui.label(
            RichText::new(format!("{:.0}%", value * 100.0))
                .color(color)
                .monospace(),
        );
    });
    ui.add(
        ProgressBar::new(value)
            .desired_width(finite_width(ui, 220.0))
            .fill(color)
            .corner_radius(CornerRadius::same(255)),
    );
}

pub fn capability(ui: &mut Ui, title: &str, text: &str, color: Color32) {
    ui.label(RichText::new(title).strong().color(color));
    ui.label(RichText::new(text).color(Palette::MUTED));
    ui.label(RichText::new("active").small().color(Palette::GREEN));
}

pub fn mini_chart(ui: &mut Ui, energy: f32, latency: f32, density: f32) {
    let desired_size = Vec2::new(finite_width(ui, 260.0), 120.0);
    let (rect, _) = ui.allocate_exact_size(desired_size, Sense::hover());
    let painter = ui.painter_at(rect);

    painter.rect_filled(rect, CornerRadius::same(16), Color32::from_rgb(11, 16, 27));

    for i in 0..6 {
        let t = i as f32 / 5.0;
        let x = egui::lerp(rect.left()..=rect.right(), t);
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 16)),
        );
    }

    draw_wave(&painter, rect.shrink(12.0), energy, Palette::CYAN, 0.0);
    draw_wave(&painter, rect.shrink(12.0), latency, Palette::VIOLET, 1.8);
    draw_wave(&painter, rect.shrink(12.0), density, Palette::GREEN, 3.3);
}

pub struct ElectronCloudState {
    yaw: f32,
    pitch: f32,
    roll: f32,
}

impl Default for ElectronCloudState {
    fn default() -> Self {
        Self {
            yaw: 0.45,
            pitch: -0.32,
            roll: 0.0,
        }
    }
}

pub fn electron_cloud(ui: &mut Ui, state: &mut ElectronCloudState, excitation: f32) {
    let size = Vec2::new(finite_width(ui, 280.0).min(360.0), 230.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::drag());
    let painter = ui.painter_at(rect);

    if response.dragged() {
        let delta = ui.input(|input| input.pointer.delta());
        state.yaw += delta.x * 0.012;
        state.pitch = (state.pitch - delta.y * 0.012).clamp(-1.2, 1.2);
    }

    let time = ui.input(|input| input.time) as f32;
    let hover = if response.hovered() { 1.0 } else { 0.0 };
    let pulse = 0.5 + 0.5 * (time * 2.4).sin();
    let glow = (0.45 + excitation * 0.45 + hover * 0.35).clamp(0.0, 1.25);
    let spin = time * (0.45 + excitation * 0.7);
    state.roll = spin * 0.22;

    painter.rect_filled(rect, CornerRadius::same(18), Color32::from_rgb(6, 10, 18));
    paint_background_grid(&painter, rect, glow);

    let center = rect.center();
    let scale = rect.width().min(rect.height()) * 0.36;
    let camera = Camera {
        center,
        scale,
        yaw: state.yaw + spin * 0.18,
        pitch: state.pitch,
        roll: state.roll,
    };

    paint_cloud_particles(&painter, rect, &camera, time, glow);
    paint_orbital(
        &painter,
        &camera,
        Orbital::new(1.0, 0.42, 0.0, 0.0),
        Palette::CYAN,
        glow,
    );
    paint_orbital(
        &painter,
        &camera,
        Orbital::new(0.9, 0.34, 1.22, 0.58),
        Palette::VIOLET,
        glow,
    );
    paint_orbital(
        &painter,
        &camera,
        Orbital::new(0.78, 0.52, -1.08, 1.25),
        Palette::GREEN,
        glow,
    );
    paint_electrons(&painter, &camera, time, pulse, glow);
    paint_nucleus(&painter, center, scale, pulse, glow);

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
    }
    ui.ctx().request_repaint();
}

pub fn finite_width(ui: &Ui, fallback: f32) -> f32 {
    let width = ui.available_width();
    if width.is_finite() && width > 1.0 {
        width
    } else {
        fallback
    }
}

#[derive(Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

struct Camera {
    center: Pos2,
    scale: f32,
    yaw: f32,
    pitch: f32,
    roll: f32,
}

impl Camera {
    fn project(&self, point: Vec3) -> (Pos2, f32) {
        let point = rotate_y(point, self.yaw);
        let point = rotate_x(point, self.pitch);
        let point = rotate_z(point, self.roll);
        let depth = (point.z + 1.6) / 3.2;
        let perspective = 1.0 / (1.0 + point.z * 0.24);

        (
            Pos2::new(
                self.center.x + point.x * self.scale * perspective,
                self.center.y + point.y * self.scale * perspective,
            ),
            depth.clamp(0.0, 1.0),
        )
    }
}

struct Orbital {
    radius: f32,
    squash: f32,
    tilt: f32,
    twist: f32,
}

impl Orbital {
    fn new(radius: f32, squash: f32, tilt: f32, twist: f32) -> Self {
        Self {
            radius,
            squash,
            tilt,
            twist,
        }
    }

    fn point(&self, t: f32) -> Vec3 {
        let angle = t * std::f32::consts::TAU;
        let point = Vec3::new(
            angle.cos() * self.radius,
            angle.sin() * self.radius * self.squash,
            0.0,
        );
        rotate_y(rotate_x(point, self.tilt), self.twist)
    }
}

fn paint_background_grid(painter: &egui::Painter, rect: Rect, glow: f32) {
    let stroke = Stroke::new(
        1.0,
        Color32::from_rgba_premultiplied(70, 170, 255, (14.0 + glow * 16.0) as u8),
    );

    for i in 0..7 {
        let t = i as f32 / 6.0;
        let x = egui::lerp(rect.left()..=rect.right(), t);
        let y = egui::lerp(rect.top()..=rect.bottom(), t);
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            stroke,
        );
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            stroke,
        );
    }
}

fn paint_cloud_particles(
    painter: &egui::Painter,
    rect: Rect,
    camera: &Camera,
    time: f32,
    glow: f32,
) {
    for i in 0..170 {
        let a = random_unit(i, 0) * std::f32::consts::TAU;
        let b = random_unit(i, 1) * std::f32::consts::TAU;
        let shell = random_unit(i, 2).sqrt();
        let drift = (time * 0.22 + random_unit(i, 3) * std::f32::consts::TAU).sin() * 0.045;
        let lobe = if i % 2 == 0 { 1.0 } else { -1.0 };
        let radius = 0.18 + shell * 0.82;
        let point = Vec3::new(
            a.cos() * radius * (0.82 + drift),
            b.sin() * radius * 0.54,
            lobe * a.sin().abs() * radius * 0.62 + drift,
        );

        let (pos, depth) = camera.project(point);
        if !rect.contains(pos) {
            continue;
        }

        let alpha = (18.0 + depth * 74.0 + glow * 20.0).clamp(0.0, 120.0) as u8;
        let radius = 0.7 + depth * 1.6 + glow * 0.35;
        painter.circle_filled(
            pos,
            radius,
            Color32::from_rgba_premultiplied(88, 210, 255, alpha),
        );
    }
}

fn paint_orbital(
    painter: &egui::Painter,
    camera: &Camera,
    orbital: Orbital,
    color: Color32,
    glow: f32,
) {
    let mut previous = None;
    for i in 0..=180 {
        let (point, depth) = camera.project(orbital.point(i as f32 / 180.0));
        if let Some(previous) = previous {
            let alpha = (54.0 + depth * 120.0 + glow * 42.0).clamp(0.0, 230.0) as u8;
            let stroke = Stroke::new(
                1.0 + depth * 1.6 + glow * 0.6,
                Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), alpha),
            );
            painter.line_segment([previous, point], stroke);
        }
        previous = Some(point);
    }
}

fn paint_electrons(painter: &egui::Painter, camera: &Camera, time: f32, pulse: f32, glow: f32) {
    let orbitals = [
        (Orbital::new(1.0, 0.42, 0.0, 0.0), Palette::CYAN, 0.0, 0.58),
        (
            Orbital::new(0.9, 0.34, 1.22, 0.58),
            Palette::VIOLET,
            0.33,
            -0.5,
        ),
        (
            Orbital::new(0.78, 0.52, -1.08, 1.25),
            Palette::GREEN,
            0.66,
            0.72,
        ),
    ];

    for (orbital, color, offset, speed) in orbitals {
        let t = (offset + time * speed * 0.18).fract();
        let (pos, depth) = camera.project(orbital.point(t));
        let radius = 3.0 + depth * 3.4 + pulse * 1.6;
        let alpha = (150.0 + depth * 80.0 + glow * 24.0).clamp(0.0, 255.0) as u8;

        painter.circle_filled(
            pos,
            radius * 2.0,
            Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 28),
        );
        painter.circle_filled(
            pos,
            radius,
            Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), alpha),
        );
        painter.circle_filled(pos, radius * 0.35, Palette::WHITE);
    }
}

fn paint_nucleus(painter: &egui::Painter, center: Pos2, scale: f32, pulse: f32, glow: f32) {
    let radius = scale * (0.11 + pulse * 0.012);

    painter.circle_filled(
        center,
        radius * (2.6 + glow * 0.5),
        Color32::from_rgba_premultiplied(63, 221, 255, 28),
    );
    painter.circle_filled(
        center,
        radius * 1.28,
        Color32::from_rgba_premultiplied(160, 112, 255, 70),
    );
    painter.circle_filled(center, radius, Color32::from_rgb(248, 252, 255));
    painter.circle_stroke(center, radius * 1.45, Stroke::new(1.0, Palette::CYAN));
}

fn rotate_x(point: Vec3, angle: f32) -> Vec3 {
    let (sin, cos) = angle.sin_cos();
    Vec3::new(
        point.x,
        point.y * cos - point.z * sin,
        point.y * sin + point.z * cos,
    )
}

fn rotate_y(point: Vec3, angle: f32) -> Vec3 {
    let (sin, cos) = angle.sin_cos();
    Vec3::new(
        point.x * cos + point.z * sin,
        point.y,
        -point.x * sin + point.z * cos,
    )
}

fn rotate_z(point: Vec3, angle: f32) -> Vec3 {
    let (sin, cos) = angle.sin_cos();
    Vec3::new(
        point.x * cos - point.y * sin,
        point.x * sin + point.y * cos,
        point.z,
    )
}

fn random_unit(index: usize, salt: usize) -> f32 {
    let seed = (index as f32 * 12.9898 + salt as f32 * 78.233).sin() * 43_758.547;
    seed.fract().abs()
}

fn draw_wave(painter: &egui::Painter, rect: Rect, amplitude: f32, color: Color32, phase: f32) {
    let mut previous = None;
    for index in 0..80 {
        let t = index as f32 / 79.0;
        let x = egui::lerp(rect.left()..=rect.right(), t);
        let wave = ((t * std::f32::consts::TAU * 2.0) + phase).sin();
        let y = rect.center().y - wave * amplitude * rect.height() * 0.35;
        let point = egui::pos2(x, y);

        if let Some(previous) = previous {
            painter.line_segment([previous, point], Stroke::new(2.0, color));
        }
        previous = Some(point);
    }
}
