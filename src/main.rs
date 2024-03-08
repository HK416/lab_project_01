mod camera;
mod mesh;
mod object;
mod pipeline;
mod resource;
mod timer;
mod utils;

use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering as MemOrdering};
use crossbeam_queue::SegQueue;
use winit::{
    keyboard::{KeyCode, PhysicalKey},
    event::{Event, WindowEvent}, 
    window::{Window, WindowBuilder}, 
    event_loop::{EventLoop, ControlFlow},
};

use camera::PerspectiveCameraBuilder;
use mesh::{ModelMesh, CubeMesh, PlaneMesh};
use object::StdObjectBuilder;
use resource::ShaderResource;

/// #### 한국어 </br>
/// 현재 애플리케이션이 실행 중인 경우 `true`값을 가집니다. </br>
/// 
/// #### English (Translation) </br>
/// Has the value `true` if the application is currently running. </br>
/// 
static IS_RUNNING: AtomicBool = AtomicBool::new(true);

/// #### 한국어 </br>
/// 렌더링 루프로 보내는 창 이벤트 대기열 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the window event queue that is sent to the rendering loop. </br>
/// 
static EVENT_QUEUE: SegQueue<Event<()>> = SegQueue::new();



fn render_loop(
    window: Arc<Window>, 
    instance: Arc<wgpu::Instance>, 
    surface: Arc<wgpu::Surface>, 
    _adapter: Arc<wgpu::Adapter>, 
    device: Arc<wgpu::Device>, 
    queue: Arc<wgpu::Queue>
) {
    // (한국어) 카메라 바인드 그룹 레이아웃을 생성합니다.
    // (English Translation) Create a camera bind group layout.
    let camera_bind_group_layout = device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Uniform(Camera))"), 
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0, 
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT, 
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None 
                    }, 
                    count: None, 
                }, 
            ], 
        }, 
    );

    // (한국어) 게임 카메라를 생성합니다. 
    // (English Translation) Create a game camera.
    let mut camera = PerspectiveCameraBuilder::new()
        .set_width(window.inner_size().width as f32)
        .set_height(window.inner_size().height as f32)
        .set_translation((0.0, 5.5, 5.0).into())
        .set_rotation(glam::Quat::from_rotation_x(-45.0f32.to_radians()))
        .build(&camera_bind_group_layout, &device, &queue);

    // (한국어) 오브젝트 바인드 그룹 레이아웃을 생성합니다.
    // (English Translation) Create a object bind group layout. 
    let object_bind_group_layout = device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Uniform(Object))"), 
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0, 
                    visibility: wgpu::ShaderStages::VERTEX, 
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None 
                    }, 
                    count: None, 
                }, 
            ],
        }, 
    );

    // (한국어) 평면 메쉬를 생성합니다.
    // (English Translation) Create a plane mesh.
    let plane_mesh = PlaneMesh::new(10.0, 10.0, &device, &queue);

    // (한국어) 큐브 메쉬들을 생성합니다.
    // (English Translation) Creates cube meshes. 
    let cube_mesh_0 = CubeMesh::new(1.0, 1.0, 1.0, &device, &queue);

    // (한국어) 오브젝트들을 생성합니다.
    // (English Translation) Creates objects. 
    let plane = StdObjectBuilder::new()
        .set_color((0.5, 0.5, 0.5).into())
        .set_translation((0.0, 0.0, 0.0).into())
        .build(&object_bind_group_layout, &device, &queue);

    let mut cubes = Vec::new();
    let red_cube = StdObjectBuilder::new()
        .set_color((1.0, 0.2, 0.2).into())
        .set_translation((0.0, 0.5, 0.0).into())
        .build(&object_bind_group_layout, &device, &queue);
    cubes.push(red_cube);

    let green_cube = StdObjectBuilder::new()
        .set_color((0.2, 1.0, 0.2).into())
        .set_translation((1.0, 1.25, 1.0).into())
        .set_rotation(glam::Quat::from_axis_angle(
            glam::Vec3::new(1.0, 1.0, 1.0).normalize(), 
            60.0f32.to_radians()
        ))
        .build(&object_bind_group_layout, &device, &queue);
    cubes.push(green_cube);

    let blue_cube = StdObjectBuilder::new()
        .set_color((0.2, 0.2, 1.0).into())
        .set_translation((-1.0, 0.75, -0.8).into())
        .set_rotation(glam::Quat::from_axis_angle(
            glam::Vec3::new(-1.0, 1.0, 0.0).normalize(), 
            38.0f32.to_radians()
        ))
        .build(&object_bind_group_layout, &device, &queue);
    cubes.push(blue_cube);

    // (한국어) 색상 그래픽스 파이프라인을 생성합니다.
    // (English Translation) Create a color graphics pipeline.
    let bind_group_layouts = &[&camera_bind_group_layout, &object_bind_group_layout];
    let color_pipeline = pipeline::create_colored_pipeline(&device, bind_group_layouts);

    // (한국어) 스왑체인 및 프레임 버퍼를 설정합니다.
    // (English Translation) Sets the swapchain and frame buffer. 
    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT, 
        format: wgpu::TextureFormat::Bgra8Unorm, 
        width: window.inner_size().width, 
        height: window.inner_size().height, 
        present_mode: wgpu::PresentMode::AutoVsync, 
        desired_maximum_frame_latency: 2, 
        alpha_mode: wgpu::CompositeAlphaMode::Auto, 
        view_formats: vec![], 
    };
    surface.configure(&device, &config);
    
    // (한국어) 깊이-스텐실 텍스처 뷰를 생성합니다.
    // (English Translation) Create the depth-stencil texture view.
    let mut depth_stencil_view = device.create_texture(
        &wgpu::TextureDescriptor {
            label: Some("DepthStencilBuffer"), 
            size: wgpu::Extent3d {
                width: window.inner_size().width, 
                height: window.inner_size().height, 
                depth_or_array_layers: 1, 
            },
            format: wgpu::TextureFormat::Depth32Float, 
            dimension: wgpu::TextureDimension::D2, 
            mip_level_count: 1, 
            sample_count: 1, 
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING, 
            view_formats: &[],
        },
    )
    .create_view(&wgpu::TextureViewDescriptor { 
        ..Default::default()
    });


    // (한국어) 렌더링 루프를 실행합니다.
    // (English Translation) Run the rendering loop.
    log::info!("Run Rendering loop.");
    let mut timer = timer::GameTimer::<50>::new();
    while IS_RUNNING.load(MemOrdering::Acquire) {
        // (한국어) 타이머를 갱신합니다.
        // (English Translation) Updates the timer. 
        timer.tick();

        // (한국어) 창 이벤트를 처리합니다.
        // (English Translation) Handles window events. 
        while let Some(event) = EVENT_QUEUE.pop() {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(size) => {
                        if size.width > 0 && size.height > 0 {
                            // (한국어) 모든 작업이 끝날 때 까지 기다립니다.
                            // (English Translation) Wait until all operations are completed.
                            instance.poll_all(true);

                            // (한국어) 스왑체인 및 프레임 버퍼를 재설정합니다.
                            // (English Translation) Reset swapchain and frame buffer. 
                            config.width = size.width;
                            config.height = size.height;
                            surface.configure(&device, &config);

                            // (한국어) 깊이-스텐실 텍스처 뷰를 재생성합니다.
                            // (English Translation) Recreate the depth-stencil texture view. 
                            depth_stencil_view = device.create_texture(
                                &wgpu::TextureDescriptor {
                                    label: Some("DepthStencilBuffer"), 
                                    size: wgpu::Extent3d {
                                        width: size.width, 
                                        height: size.height, 
                                        depth_or_array_layers: 1, 
                                    },
                                    format: wgpu::TextureFormat::Depth32Float, 
                                    dimension: wgpu::TextureDimension::D2, 
                                    mip_level_count: 1, 
                                    sample_count: 1, 
                                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING, 
                                    view_formats: &[],
                                },
                            )
                            .create_view(&wgpu::TextureViewDescriptor { 
                                ..Default::default()
                            });
                        }
                    },
                    _ => { /*--- empty ---*/ }
                },
                _ => { /*--- empty ---*/ }
            }
        }

        
        // (한국어) 오브젝트들을 그립니다.
        // (English Translation) Draws the objects.
        window.pre_present_notify();
        
        // (한국어) 이전 작업이 끝날 때 까지 기다립니다.
        // (English Translation) Wait until the previous operation is finished.
        device.poll(wgpu::Maintain::Wait);

        // (한국어) 다음 프레임을 가져옵니다.
        // (English Translation) Get the next frame.
        let frame = surface.get_current_texture().unwrap();

        // (한국어) 렌더 타겟의 텍스처 뷰를 생성합니다.
        // (English Translation) Creates a texture view of render target.
        let render_target_view = frame.texture.create_view(&wgpu::TextureViewDescriptor { 
            ..Default::default()
        });

        // (한국어) 커맨드 버퍼를 생성합니다.
        // (English Translation) Creates a command buffer. 
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut rpass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("RenderPass(Test)"), 
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment {
                            view: &render_target_view, 
                            resolve_target: None, 
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), 
                                store: wgpu::StoreOp::Store, 
                            }, 
                        }),
                    ],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &depth_stencil_view, 
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0), 
                            store: wgpu::StoreOp::Store, 
                        }), 
                        stencil_ops: None, 
                    }), 
                    timestamp_writes: None, 
                    occlusion_query_set: None, 
                },
            );

            rpass.set_pipeline(&color_pipeline);
            rpass.set_bind_group(0, camera.bind_group_ref(), &[]);

            plane_mesh.bind(&mut rpass);
            rpass.set_bind_group(1, plane.bind_group_ref(), &[]);
            plane_mesh.draw(&mut rpass);

            cube_mesh_0.bind(&mut rpass);
            for object in cubes.iter() {
                rpass.set_bind_group(1, object.bind_group_ref(), &[]);
                cube_mesh_0.draw(&mut rpass);
            }
        }

        // (한국어) 명령 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
        // (English Translation) Submit command buffer to the queue and output to the framebuffer. 
        queue.submit(Some(encoder.finish()));
        frame.present();
    }

    log::info!("Finish Rendering loop.");
}

fn main() {
    env_logger::init();
    log::info!("❖ Application Launching ❖");
    
    // (한국어) 창 시스템을 초기화 합니다.
    // (English Translation) Initializes the window system.
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_visible(true)
            .with_resizable(true)
            .with_title("Lab Project 00")
            .build(&event_loop)
            .unwrap()
    );

    // (한국어) 렌더링 시스템을 초기화 합니다.
    // (English Translation) Initialize the rendering system.
    let window_cloned = window.clone();
    let (instance, surface, adapter, device, queue) = utils::setup_rendering_system(window_cloned);

    // (한국어) 새로운 스레드에서 렌더링 루프를 실행합니다.
    // (English Translation) Runs the rendering loop in a new thread.
    let window_cloned = window.clone();
    let instance_cloned = instance.clone();
    let mut join = Some(thread::spawn(move || render_loop(
        window_cloned, 
        instance_cloned, 
        surface, 
        adapter, 
        device, 
        queue
    )));

    // (한국어) 윈도우 메시지 루프를 실행합니다.
    // (English Translation) Runs the window message loop.
    log::info!("Run Window message loop.");
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run(move |event, elwt| {
        // (한국어) 현재 렌더링 스레드가 실행 중인지 확인합니다.
        // (English Translation) Checks if the current rendering thread is running.
        if join.as_ref().is_some_and(|join| join.is_finished()) {
            // (한국어) 렌더링 스레드를 join 합니다.
            // (English Translation) Join the rendering thread.
            join.take().unwrap().join().unwrap();

            // (한국어) 애플리케이션을 종료합니다.
            // (English Translation) Quit the application.
            elwt.exit();
            return;
        }

        // (한국어) 윈도우 이벤트를 처리합니다.
        // (English Translation) Handles window events. 
        let event_cloned = event.clone();
        match event_cloned {
            Event::NewEvents(_) | Event::AboutToWait => {
                return;
            },
            Event::WindowEvent { window_id, event } 
            if window_id == window.id() => match event {
                WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                    IS_RUNNING.store(false, MemOrdering::Release);
                    elwt.exit();
                    return;
                },
                _ => { /* empty */ }
            },
            _ => { /* empty */ }
        }

        // (한국어) 창 이벤트를 이벤트 대기열에 추가합니다.
        // (English Translation) Add a window event to the event queue. 
        EVENT_QUEUE.push(event);
    }).unwrap();

    instance.poll_all(true);
    log::info!("❖ Application Terminate ❖");
}
