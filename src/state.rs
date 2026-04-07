/* Libs */
use anyhow::Result;
use egui_wgpu::ScreenDescriptor;
use egui_winit::EventResponse;
use std::sync::Arc;
use wgpu::{
    BackendOptions, Backends, CommandEncoder, CommandEncoderDescriptor, CompositeAlphaMode,
    CurrentSurfaceTexture, Device, DeviceDescriptor, Dx12BackendOptions, Dx12SwapchainKind,
    ExperimentalFeatures, Features, Instance, InstanceDescriptor, Limits, LoadOp, Operations,
    PowerPreference, Queue, RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions,
    StoreOp, Surface, SurfaceConfiguration, TextureUsages, TextureView, TextureViewDescriptor,
    Trace,
};
use winit::{event::WindowEvent, window::Window};

/* Locals */
use crate::{constants, ui::Ui};

pub struct State {
    // WGPU Core Components
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    device: Device,
    queue: Queue,

    // Winit Window
    window: Arc<Window>,

    // Optional EGUI State
    ui: Option<Ui>,

    // State Flags
    is_surface_configured: bool,
}

impl State {
    pub async fn init(window: Arc<Window>) -> Result<Self> {
        let size = window.inner_size();

        let backends = if cfg!(target_os = "windows") {
            Backends::DX12
        } else {
            Backends::PRIMARY
        };

        let instance = Instance::new(InstanceDescriptor {
            backends,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: BackendOptions {
                dx12: Dx12BackendOptions {
                    presentation_system: Dx12SwapchainKind::DxgiFromVisual,
                    ..Default::default()
                },
                ..Default::default()
            },
            display: Default::default(),
        });

        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::None,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                experimental_features: ExperimentalFeatures::disabled(),
                required_limits: Limits::default(),
                memory_hints: Default::default(),
                trace: Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let alpha_mode = [
            CompositeAlphaMode::PreMultiplied,
            CompositeAlphaMode::PostMultiplied,
            CompositeAlphaMode::Inherit,
        ]
        .into_iter()
        .find(|mode| surface_caps.alpha_modes.contains(mode))
        .unwrap_or(surface_caps.alpha_modes[0]);

        if matches!(alpha_mode, CompositeAlphaMode::Opaque) {
            log::warn!(
                "Surface transparency is not supported by this adapter/configuration; alpha modes: {:?}",
                surface_caps.alpha_modes
            );
        }

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode,
            view_formats: Vec::new(),
            desired_maximum_frame_latency: 2,
        };

        let ui = Ui::new(
            &device,
            config.format,
            None,
            constants::gpu::MSAA_SAMPLES,
            constants::gpu::DITHERING,
            constants::gpu::PREDICTABLE_TEXTURE_FILTERING,
            &window,
        );

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            ui: Some(ui),
        })
    }

    pub fn handle_egui_event(&mut self, event: &WindowEvent) -> EventResponse {
        if let Some(ui) = &mut self.ui {
            ui.handle_event(&self.window, event)
        } else {
            EventResponse::default()
        }
    }

    pub fn render(&mut self) -> Result<()> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }

        let output = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                self.surface.configure(&self.device, &self.config);
                surface_texture
            }
            CurrentSurfaceTexture::Occluded => return Ok(()),
            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Validation => {
                // Skip the frame
                log::trace!(
                    "Surface is not ready for rendering: {:?}",
                    self.surface.get_current_texture()
                );
                return Ok(());
            }
            CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            CurrentSurfaceTexture::Lost => {
                anyhow::bail!("Lost device");
            }
        };

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.render_background(&mut encoder, &view);
        self.render_egui(&mut encoder, &view);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn render_egui(&mut self, encoder: &mut CommandEncoder, view: &TextureView) {
        if let Some(ui) = &mut self.ui {
            let screen_descriptor = ScreenDescriptor {
                size_in_pixels: [self.config.width, self.config.height],
                pixels_per_point: ui.context.pixels_per_point(),
            };

            ui.draw(
                &self.device,
                &self.queue,
                encoder,
                &self.window,
                view,
                &screen_descriptor,
                |ui| {
                    let ctx = ui.ctx();

                    egui::Window::new("Test").show(ctx, |ui| {
                        ui.label("Hello, EGUI!");
                        if ui.button("Click me").clicked() {
                            log::info!("Button clicked!");
                        }
                        if ui.button("Quit").clicked() {
                            log::info!("Quit button clicked, exiting...");
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                },
            );
        }
    }

    fn render_background(&self, encoder: &mut CommandEncoder, window_surface_view: &TextureView) {
        let _rpass = encoder.begin_render_pass(&RenderPassDescriptor {
            color_attachments: &[Some(RenderPassColorAttachment {
                depth_slice: None,
                view: window_surface_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(constants::gpu::CLEAR_COLOR),
                    store: StoreOp::Store,
                },
            })],
            label: Some("Background Render Pass"),
            timestamp_writes: None,
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn should_close(&self) -> bool {
        self.ui
            .as_ref()
            .map(|ui| ui.should_close())
            .unwrap_or(false)
    }

    pub fn update(&mut self) {
        // Update code goes here
        _ = self;
    }

    pub fn window(&self) -> &Window {
        self.window.as_ref()
    }
}
