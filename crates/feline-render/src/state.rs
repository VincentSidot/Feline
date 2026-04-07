/* Libs */
use anyhow::{Result, bail};
use egui::{Rect, Ui as EguiUi};
use egui_wgpu::ScreenDescriptor;
use egui_winit::EventResponse;
use std::{collections::HashMap, sync::Arc};
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
use crate::{
    app::{AppExt, ApplicationId},
    constants,
    platform::OverlayWindowPlatformExt,
    ui::Ui,
};

const HITTEST_MARGIN: f32 = 2.0;

pub struct State {
    // WGPU Core Components
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    device: Device,
    queue: Queue,

    // Winit Window
    window: Arc<Window>,

    // EGUI Render State
    ui: Ui,

    // Applications bank
    bank: ApplicationBank,

    // State Flags
    is_surface_configured: bool,
    cursor_hittest_enabled: bool,
    cursor_hittest_available: bool,
    egui_interactive_rect: Vec<Option<Rect>>,
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
            ui,
            cursor_hittest_enabled: true,
            cursor_hittest_available: true,
            egui_interactive_rect: Default::default(),
            bank: Default::default(),
        })
    }

    pub fn register(&mut self, app: Box<dyn AppExt>) -> Result<ApplicationId> {
        self.bank.register(app)
    }

    pub fn handle_egui_event(&mut self, event: &WindowEvent) -> EventResponse {
        self.ui.handle_event(&self.window, event)
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
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: self.ui.context.pixels_per_point(),
        };

        self.ui.draw(
            &self.device,
            &self.queue,
            encoder,
            &self.window,
            view,
            &screen_descriptor,
            |ui| {
                self.bank.render(ui, &mut self.egui_interactive_rect);
            },
        );
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
        self.ui.should_close()
    }

    pub fn update(&mut self) {
        self.update_cursor_hittest();
        self.bank.garbage_collect();
    }

    pub fn window(&self) -> &Window {
        self.window.as_ref()
    }

    fn update_cursor_hittest(&mut self) {
        if !self.cursor_hittest_available {
            return;
        }

        if self.ui_is_using_pointer() {
            self.set_cursor_hittest(true);
            return;
        }

        let Some(cursor_position) = self.window.cursor_position_in_window() else {
            return;
        };

        let hit = self
            .egui_interactive_rect
            .iter()
            .flatten()
            .any(|rect| rect.expand(HITTEST_MARGIN).contains(cursor_position));

        self.set_cursor_hittest(hit);
    }

    fn ui_is_using_pointer(&self) -> bool {
        self.ui.is_using_pointer()
    }

    fn set_cursor_hittest(&mut self, enabled: bool) {
        if self.cursor_hittest_enabled == enabled {
            return;
        }

        match self.window.set_cursor_hittest(enabled) {
            Ok(()) => {
                self.cursor_hittest_enabled = enabled;
            }
            Err(err) => {
                self.cursor_hittest_available = false;
                log::warn!("Cursor hit testing is unavailable: {err}");
            }
        }
    }
}

struct ApplicationEntry {
    id: ApplicationId,
    opaque: Box<dyn AppExt>,
}

#[derive(Default)]
struct ApplicationBank {
    entries: Vec<ApplicationEntry>,
    index_by_id: HashMap<ApplicationId, usize>,
    free_ids: Vec<ApplicationId>,
    to_free_ids: Vec<ApplicationId>,
}

impl ApplicationBank {
    fn render(&mut self, ui: &mut EguiUi, rects: &mut Vec<Option<Rect>>) {
        let ctx = ui.ctx();

        // Ensure rect has capacity for all apps
        rects.resize(self.entries.len(), None);

        for (id, entry) in &mut self.entries.iter_mut().enumerate() {
            let response = entry.opaque.render(ctx);

            rects[id] = response.map(|response| response.rect);

            if entry.opaque.should_close() {
                self.to_free_ids.push(entry.id);
            }
        }
    }

    fn garbage_collect(&mut self) {
        let ids = std::mem::take(&mut self.to_free_ids);

        for id in ids {
            if let Err(err) = self.unregister(id) {
                // TODO [MEDIUM]: Handle errors better and propagate them up.
                log::error!("Failed to unregister app with id {id}: {err}");
            }
        }
    }

    fn fetch_id(&mut self) -> ApplicationId {
        if let Some(id) = self.free_ids.pop() {
            return id;
        }

        // Else if no free ids, it means there is no hole in app vecs, so we can
        // just use the next id which is the current length of the apps vec
        self.entries.len() as ApplicationId
    }

    fn register(&mut self, mut app: Box<dyn AppExt>) -> Result<ApplicationId> {
        app.init()?;
        let id = self.fetch_id();

        let entry = ApplicationEntry { id, opaque: app };

        self.index_by_id.insert(id, self.entries.len());
        self.entries.push(entry);

        Ok(id)
    }

    fn unregister(&mut self, id: ApplicationId) -> Result<()> {
        let Some(index) = self.index_by_id.remove(&id) else {
            bail!("No application with id {id}");
        };

        let index_to_fix = self.entries.len() - 1;
        let mut app = self.entries.swap_remove(index);

        // Post fix the app if we didn't just remove the last one
        if index != index_to_fix {
            let moved_app_id = self.entries[index].id;
            self.index_by_id.insert(moved_app_id, index);
        }

        self.free_ids.push(id);

        app.opaque.deinit()?;

        Ok(())
    }
}
