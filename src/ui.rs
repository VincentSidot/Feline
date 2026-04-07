/* Libs */
use egui::{Context, FullOutput, Ui as EUi, ViewportCommand, Visuals};
use egui_wgpu::{Renderer, RendererOptions, ScreenDescriptor};
use egui_winit::{EventResponse, State};
use wgpu::{
    CommandEncoder, Device, LoadOp, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, TextureFormat, TextureView,
};
use winit::{event::WindowEvent, window::Window};

/* Locals */
use crate::constants;

pub struct Ui {
    pub context: Context,
    renderer: Renderer,
    should_close: bool,
    state: State,
}

impl Ui {
    pub fn new(
        device: &Device,
        output_color_format: TextureFormat,
        depth_stencil_format: Option<TextureFormat>,
        msaa_samples: u32,
        dithering: bool,
        predictable_texture_filtering: bool,
        window: &Window,
    ) -> Self {
        let context = egui::Context::default();
        let viewport_id = context.viewport_id();

        let visuals = Visuals {
            window_corner_radius: constants::egui::BORDER_RADIUS,
            window_shadow: constants::egui::SHADOW,
            ..Default::default()
        };

        context.set_visuals(visuals);

        let state = State::new(context.clone(), viewport_id, window, None, None, None);

        let renderer = Renderer::new(
            device,
            output_color_format,
            RendererOptions {
                depth_stencil_format,
                msaa_samples,
                dithering,
                predictable_texture_filtering,
            },
        );

        Self {
            context,
            renderer,
            should_close: false,
            state,
        }
    }

    pub fn handle_event(&mut self, window: &Window, event: &WindowEvent) -> EventResponse {
        self.state.on_window_event(window, event)
    }

    pub fn draw(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        window: &Window,
        window_surface_view: &TextureView,
        screen_descriptor: &ScreenDescriptor,
        run_ui: impl FnMut(&mut EUi),
    ) {
        let raw_input = self.state.take_egui_input(window);
        let full_output = self.context.run_ui(raw_input, run_ui);

        self.compute_should_close(&full_output);

        self.state
            .handle_platform_output(window, full_output.platform_output);

        let tris = self
            .context
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        for (id, image_delta) in full_output.textures_delta.set {
            self.renderer
                .update_texture(device, queue, id, &image_delta);
        }

        self.renderer
            .update_buffers(device, queue, encoder, &tris, screen_descriptor);

        let mut rpass = encoder
            .begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[Some(RenderPassColorAttachment {
                    depth_slice: None,
                    view: window_surface_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Load,
                        store: StoreOp::Store,
                    },
                })],
                label: Some("EGUI Render Pass"),
                timestamp_writes: None,
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                multiview_mask: None,
            })
            .forget_lifetime();

        self.renderer.render(&mut rpass, &tris, screen_descriptor);
        drop(rpass);

        for id in full_output.textures_delta.free {
            self.renderer.free_texture(&id);
        }
    }

    fn compute_should_close(&mut self, output: &FullOutput) {
        if output.viewport_output.values().any(|output| {
            output
                .commands
                .iter()
                .any(|command| matches!(command, ViewportCommand::Close))
        }) {
            self.should_close = true;
        }
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn is_using_pointer(&self) -> bool {
        self.context.egui_is_using_pointer()
    }
}
