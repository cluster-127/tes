//! TES Visualization - Pure Field-Based
//!
//! NO particle tracking. NO object iteration.
//! Only the Grid exists. Shapes are invisible - only their trace remains.
//!
//! Run with: cargo run --bin tes-viz --features viz

use rand::Rng;
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use tes::{IsotopeGrid, ServiceColor};

// Frame rate limiting (60 FPS)
const TARGET_FRAME_TIME: Duration = Duration::from_millis(16);

// Grid dimensions - this is the ONLY data structure
const GRID_WIDTH: usize = 256;
const GRID_HEIGHT: usize = 256;
const DECAY_RATE: u32 = 2;
const HABITABILITY_THRESHOLD: u32 = 500;

// Contribution parameters
const CONTRIBUTIONS_PER_TICK: usize = 150;

/// Service hotspot - just parameters, NO shape tracking
struct Hotspot {
    color: ServiceColor,
    center_x: usize,
    center_y: usize,
    radius: f32,
    intensity: u32,
}

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("TES - Pure Field Visualization")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 800))
            .build(&event_loop)
            .unwrap(),
    );

    // wgpu setup
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let surface = instance.create_surface(window.clone()).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap();

    let size = window.inner_size();
    let mut config = surface
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();
    config.present_mode = wgpu::PresentMode::AutoVsync;
    surface.configure(&device, &config);

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("TES Shader"),
        source: wgpu::ShaderSource::Wgsl(SHADER.into()),
    });

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Field Texture"),
        size: wgpu::Extent3d {
            width: GRID_WIDTH as u32,
            height: GRID_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    // THE ONLY DATA STRUCTURE: The Grid (Field)
    let grid = IsotopeGrid::new(
        GRID_WIDTH,
        GRID_HEIGHT,
        DECAY_RATE,
        HABITABILITY_THRESHOLD,
        HABITABILITY_THRESHOLD / 2,
    );

    // Hotspots define WHERE contributions happen, but NO objects are tracked
    let hotspots = [
        Hotspot {
            color: ServiceColor::from_name("Auth"),
            center_x: 70,
            center_y: 70,
            radius: 45.0,
            intensity: 20,
        },
        Hotspot {
            color: ServiceColor::from_name("Payment"),
            center_x: 128,
            center_y: 128,
            radius: 55.0,
            intensity: 25,
        },
        Hotspot {
            color: ServiceColor::from_name("Worker"),
            center_x: 190,
            center_y: 190,
            radius: 40.0,
            intensity: 18,
        },
    ];

    let mut rng = rand::thread_rng();
    let mut tick = 0u64;
    let mut last_frame = Instant::now();

    // Pixel buffer - reused every frame
    let mut pixels = vec![0u8; GRID_WIDTH * GRID_HEIGHT * 4];

    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    elwt.exit();
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(new_size),
                    ..
                } => {
                    if new_size.width > 0 && new_size.height > 0 {
                        config.width = new_size.width;
                        config.height = new_size.height;
                        surface.configure(&device, &config);
                    }
                }
                Event::AboutToWait => {
                    tick += 1;

                    // === PHASE 1: Direct Field Contribution ===
                    for hotspot in &hotspots {
                        for _ in 0..CONTRIBUTIONS_PER_TICK / hotspots.len() {
                            let angle: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
                            let dist: f32 = rng.gen_range(0.0..hotspot.radius);
                            let x = (hotspot.center_x as f32 + angle.cos() * dist) as usize;
                            let y = (hotspot.center_y as f32 + angle.sin() * dist) as usize;

                            if x < GRID_WIDTH && y < GRID_HEIGHT {
                                if grid.is_habitable(x, y, HABITABILITY_THRESHOLD) {
                                    grid.contribute(x, y, hotspot.intensity, hotspot.color);
                                }
                            }
                        }
                    }

                    // === PHASE 2: Diffusion (Energy Conserving) ===
                    grid.diffuse();

                    // === PHASE 3: Global Decay ===
                    grid.apply_decay();

                    // === PHASE 4: Render Field ===
                    // CPU only sends raw data, GPU does the glow
                    let mut saturated_count = 0usize;
                    for y in 0..GRID_HEIGHT {
                        for x in 0..GRID_WIDTH {
                            let (r, g, b) = grid.rgb(x, y);
                            let density = grid.density(x, y);
                            let idx = (y * GRID_WIDTH + x) * 4;

                            if density >= HABITABILITY_THRESHOLD {
                                pixels[idx] = 255;
                                pixels[idx + 1] = 255;
                                pixels[idx + 2] = 255;
                                saturated_count += 1;
                            } else {
                                // Map 0-THRESHOLD to 0-255 for full danger ramp visibility
                                let scale =
                                    |v: u32| ((v * 255) / HABITABILITY_THRESHOLD).min(255) as u8;
                                pixels[idx] = scale(r);
                                pixels[idx + 1] = scale(g);
                                pixels[idx + 2] = scale(b);
                            }
                            pixels[idx + 3] = 255;
                        }
                    }

                    // Single texture upload
                    queue.write_texture(
                        wgpu::ImageCopyTexture {
                            texture: &texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                        },
                        &pixels,
                        wgpu::ImageDataLayout {
                            offset: 0,
                            bytes_per_row: Some(GRID_WIDTH as u32 * 4),
                            rows_per_image: Some(GRID_HEIGHT as u32),
                        },
                        wgpu::Extent3d {
                            width: GRID_WIDTH as u32,
                            height: GRID_HEIGHT as u32,
                            depth_or_array_layers: 1,
                        },
                    );

                    // Render
                    let output = surface.get_current_texture().unwrap();
                    let view = output
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder =
                        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });

                    {
                        let mut render_pass =
                            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: Some("Render Pass"),
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: &view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                        store: wgpu::StoreOp::Store,
                                    },
                                })],
                                depth_stencil_attachment: None,
                                timestamp_writes: None,
                                occlusion_query_set: None,
                            });

                        render_pass.set_pipeline(&pipeline);
                        render_pass.set_bind_group(0, &bind_group, &[]);
                        render_pass.draw(0..6, 0..1);
                    }

                    queue.submit(std::iter::once(encoder.finish()));
                    output.present();

                    // Telemetry - throttled
                    if tick % 30 == 0 {
                        let saturated_pct =
                            (saturated_count as f32 / (GRID_WIDTH * GRID_HEIGHT) as f32) * 100.0;
                        window.set_title(&format!(
                            "TES | Tick: {} | Saturated: {:.1}%",
                            tick, saturated_pct
                        ));
                    }

                    // Frame rate limiting
                    let elapsed = last_frame.elapsed();
                    if elapsed < TARGET_FRAME_TIME {
                        std::thread::sleep(TARGET_FRAME_TIME - elapsed);
                    }
                    last_frame = Instant::now();

                    window.request_redraw();
                }
                _ => {}
            }
        })
        .unwrap();
}

// GPU-powered shader with gamma glow
const SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0),
    );
    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, 0.0),
    );

    var output: VertexOutput;
    output.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    output.uv = uvs[vertex_index];
    return output;
}

@group(0) @binding(0) var field_texture: texture_2d<f32>;
@group(0) @binding(1) var field_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let raw = textureSample(field_texture, field_sampler, in.uv);

    // GPU-powered gamma glow
    // pow(x, 0.6) brightens low values, * 3.0 amplifies
    let r = pow(raw.r, 0.6) * 3.0;
    let g = pow(raw.g, 0.6) * 3.0;
    let b = pow(raw.b, 0.6) * 3.0;

    return vec4<f32>(r, g, b, 1.0);
}
"#;
