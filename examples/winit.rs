use danmakw::Renderer;
use std::sync::Arc;
use wgpu::{
    CompositeAlphaMode, DeviceDescriptor, Instance, InstanceDescriptor, PresentMode,
    RequestAdapterOptions, SurfaceConfiguration, TextureUsages, TextureViewDescriptor,
};
use winit::{
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::EventLoop,
    window::Window,
};

mod utils;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop
        .run_app(&mut Application { window_state: None })
        .unwrap();
}

struct WindowState {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: SurfaceConfiguration,
    renderer: Renderer,
    window: Arc<Window>,
}

impl WindowState {
    async fn new(window: Arc<Window>) -> Self {
        let physical_size = window.inner_size();
        let scale_factor = window.scale_factor();

        let instance = Instance::new(&InstanceDescriptor::default());

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: PresentMode::Mailbox,
            alpha_mode: CompositeAlphaMode::PreMultiplied,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        let mut renderer = danmakw::Renderer::new(&device, &queue, surface_format, scale_factor);

        let danmakus = utils::parse_bilibili_xml(include_str!("test.xml")).unwrap();

        renderer.init(danmakus);

        Self {
            device,
            queue,
            surface,
            surface_config,
            renderer,
            window,
        }
    }
}

struct Application {
    window_state: Option<WindowState>,
}

impl winit::application::ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window_state.is_some() {
            return;
        }

        let (width, height) = (800, 600);
        let window_attributes = Window::default_attributes()
            .with_inner_size(LogicalSize::new(width as f64, height as f64))
            .with_title("Danmakw Renderer");
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.window_state = Some(pollster::block_on(WindowState::new(window)));

        if let Some(state) = &mut self.window_state {
            state.renderer.resize(
                &state.queue,
                state.surface_config.width,
                state.surface_config.height,
            );
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = &mut self.window_state else {
            return;
        };

        let WindowState {
            window,
            device,
            queue,
            surface,
            surface_config,
            renderer,
            ..
        } = state;

        match event {
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    surface_config.width = size.width;
                    surface_config.height = size.height;
                    surface.configure(device, surface_config);
                    renderer.resize(queue, size.width, size.height);
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                renderer.update();

                match surface.get_current_texture() {
                    Ok(frame) => {
                        let view = frame.texture.create_view(&TextureViewDescriptor::default());
                        if let Err(e) = renderer.render(
                            device,
                            queue,
                            &view,
                            surface_config.width,
                            surface_config.height,
                        ) {
                            eprintln!("Surface error: {:?}", e);
                            match e {
                                wgpu::SurfaceError::Lost => {
                                    surface.configure(device, surface_config);
                                    renderer.resize(
                                        queue,
                                        surface_config.width,
                                        surface_config.height,
                                    );
                                    window.request_redraw();
                                }
                                wgpu::SurfaceError::OutOfMemory => event_loop.exit(),
                                _ => {}
                            }
                        } else {
                            frame.present();
                            window.request_redraw();
                        }
                    }
                    Err(wgpu::SurfaceError::Lost) => {
                        surface.configure(device, surface_config);
                        renderer.resize(queue, surface_config.width, surface_config.height);
                        window.request_redraw();
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    Err(e) => eprintln!("Error acquiring frame: {:?}", e),
                }
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}
