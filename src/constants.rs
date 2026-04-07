pub mod egui {
    use egui::CornerRadius;
    use egui::epaint::Shadow;

    pub const BORDER_RADIUS: CornerRadius = CornerRadius::same(2);
    pub const SHADOW: Shadow = Shadow::NONE;
}

pub mod gpu {
    pub const MSAA_SAMPLES: u32 = 1;
    pub const DITHERING: bool = true;
    pub const PREDICTABLE_TEXTURE_FILTERING: bool = false;
    pub const CLEAR_COLOR: wgpu::Color = wgpu::Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
}
