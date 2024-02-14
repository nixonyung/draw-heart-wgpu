mod assets;
mod vertex;

use assets::VERTICES;
use vertex::Vertex;
use wgpu::util::DeviceExt;

const INITIAL_WINDOW_SIZE: winit::dpi::PhysicalSize<u32> = winit::dpi::PhysicalSize::new(800, 800);

const BACKGROUND_COLOR: wgpu::Color = wgpu::Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

fn main() {
    env_logger::init();

    // create window
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(crate::INITIAL_WINDOW_SIZE)
        .with_theme(Some(winit::window::Theme::Dark))
        .build(&event_loop)
        .unwrap();

    // create surface
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..Default::default()
    });
    let surface = instance.create_surface(&window).unwrap();
    let adapter = pollster::block_on(async {
        instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
    })
    .unwrap();
    let mut surface_config = {
        let mut config = surface
            .get_default_config(
                &adapter,
                window.inner_size().width,
                window.inner_size().height,
            )
            .unwrap();
        let capabilities = surface.get_capabilities(&adapter);
        config.format = *capabilities
            .formats
            .iter()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(&capabilities.formats[0]);
        config
    };

    // create device and resources
    let (device, queue) = pollster::block_on(async {
        adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
    })
    .unwrap();
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        usage: wgpu::BufferUsages::VERTEX,
        contents: bytemuck::cast_slice(crate::VERTICES),
    });
    let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/main.wgsl"));
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor::default())),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[crate::Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList, // every three vertices will correspond to one triangle
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back), // discard triangles facing back
            ..Default::default()
        },
        depth_stencil: None, // 1.
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,                         // use all samples
            alpha_to_coverage_enabled: false, // TODO: anti-aliasing
        },
        multiview: None,
    });

    surface.configure(&device, &surface_config);
    event_loop
        .run(|event, elwt| {
            let mut resize = |new_size: winit::dpi::PhysicalSize<u32>| {
                surface_config.width = new_size.width;
                surface_config.height = new_size.height;
                surface.configure(&device, &surface_config);
            };

            let render = || -> Result<(), wgpu::SurfaceError> {
                let output = surface.get_current_texture()?;
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[
                            // This is what @location(0) in the fragment shader targets
                            Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(crate::BACKGROUND_COLOR),
                                    store: wgpu::StoreOp::Store,
                                },
                            }),
                        ],
                        depth_stencil_attachment: None,
                        occlusion_query_set: None,
                        timestamp_writes: None,
                    });

                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.draw(0..(crate::VERTICES.len() as u32), 0..1);
                }
                queue.submit([encoder.finish()]);
                output.present();
                Ok(())
            };

            match event {
                winit::event::Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    match event {
                        winit::event::WindowEvent::CloseRequested => elwt.exit(),
                        winit::event::WindowEvent::Resized(physical_size) => {
                            resize(*physical_size);
                        }
                        winit::event::WindowEvent::RedrawRequested => {
                            match render() {
                                Ok(_) => {}
                                // Reconfigure the surface if lost
                                Err(wgpu::SurfaceError::Lost) => resize(window.inner_size()),
                                // The system is out of memory, we should probably quit
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                // All other errors (Outdated, Timeout) should be resolved by the next frame
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        // handle pressed Escape
                        winit::event::WindowEvent::KeyboardInput {
                            event:
                                winit::event::KeyEvent {
                                    physical_key:
                                        winit::keyboard::PhysicalKey::Code(
                                            winit::keyboard::KeyCode::Escape,
                                        ),
                                    ..
                                },
                            ..
                        } => elwt.exit(),
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .unwrap();
}
