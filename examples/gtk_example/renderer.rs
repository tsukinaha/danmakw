use ash::vk;
use wgpu::hal;

use super::channel::RECEIVE_FRAME_CHANNEL;

pub struct Renderer {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub danmaku_renderer: danmakw::Renderer,
}

impl Renderer {
    pub async fn new() -> Self{
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            backend_options: wgpu::BackendOptions::from_env_or_default(),
            flags: wgpu::InstanceFlags::default(),
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = create_device_queue(
            &instance,
            &adapter,
            wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
        );

        let danmaku_renderer = danmakw::Renderer::new(
            &device,
            &queue,
            wgpu::TextureFormat::Rgba8Unorm,
            1.0,
        );

        Self {
            instance,
            adapter,
            device,
            queue,
            danmaku_renderer,
        }
    }

    pub fn init(&mut self, danmaku: Vec<danmakw::Danmaku>) {
        self.danmaku_renderer.init(danmaku);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.danmaku_renderer.resize(&self.queue, width, height);
    }

    pub async fn render(&mut self, width: u32, height: u32) {
        self.danmaku_renderer.update();
        let frame = self.danmaku_renderer.render_to_export_texture(
            &self.device,
            &self.instance,
            &self.queue,
            width,
            height
        ).unwrap();

        RECEIVE_FRAME_CHANNEL.tx
            .send_async(frame).await.unwrap();
    }
}

fn create_device_queue(
    instance: &wgpu::Instance,
    adapter: &wgpu::Adapter,
    required_features: wgpu::Features,
) -> (wgpu::Device, wgpu::Queue) {
    let instance = unsafe {
        if let Some(instance) = instance.as_hal::<hal::api::Vulkan>() {
            instance.shared_instance().raw_instance()
        } else {
            panic!("Failed to get vulakn hal instance");
        }
    };

    let mut open_device = None;
    let all_features = adapter.features() | required_features;
    unsafe {
        adapter.as_hal::<hal::api::Vulkan, _, _>(|adapter| {
            if let Some(adapter) = adapter {
                let raw = adapter.raw_physical_device();

                let mut enabled_extensions = adapter.required_device_extensions(all_features);
                enabled_extensions.push(vk::EXT_EXTERNAL_MEMORY_DMA_BUF_NAME);
                enabled_extensions.push(vk::KHR_EXTERNAL_MEMORY_FD_NAME);
                enabled_extensions.push(vk::KHR_EXTERNAL_MEMORY_NAME);
                enabled_extensions.push(vk::EXT_IMAGE_DRM_FORMAT_MODIFIER_NAME);

                let mut enabled_phd_features =
                    adapter.physical_device_features(&enabled_extensions, all_features);

                let queue_create_info = vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(0)
                    .queue_priorities(&[1.0]);
                let queue_family_infos = [queue_create_info];

                let str_pointers = enabled_extensions
                    .iter()
                    .map(|&s| s.as_ptr())
                    .collect::<Vec<_>>();

                let pre_info = vk::DeviceCreateInfo::default()
                    .queue_create_infos(&queue_family_infos)
                    .enabled_extension_names(&str_pointers);

                let device_create_info = enabled_phd_features.add_to_device_create(pre_info);

                let raw_device = instance
                    .create_device(raw, &device_create_info, None)
                    .expect("Failed to create device");

                open_device = Some(
                    adapter
                        .device_from_raw(
                            raw_device,
                            None,
                            &enabled_extensions,
                            required_features,
                            &wgpu::MemoryHints::Performance,
                            0,
                            0,
                        )
                        .expect("Failed to create adapter"),
                );
            }
        })
    };

    let (device, queue) = unsafe {
        adapter
            .create_device_from_hal(
                open_device.unwrap(),
                &wgpu::DeviceDescriptor {
                    required_features,
                    required_limits: wgpu::Limits::default().using_resolution(adapter.limits()),
                    label: None,
                    memory_hints: wgpu::MemoryHints::Performance,
                    trace: wgpu::Trace::Off,
                },
            )
            .expect("Failed to create device and queue from hal")
    };

    (device, queue)
}