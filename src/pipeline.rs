use std::mem;

use crate::object::ObjectVertexLayout;



/// #### 한국어 </br>
/// 색상 그래픽스 파이프라인을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a color graphics pipeline. </br>
/// 
pub fn create_colored_pipeline(
    device: &wgpu::Device, 
    bind_group_layouts: &[&wgpu::BindGroupLayout], 
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: Some("PipelineLayout(RenderPipeline(Colored))"), 
            bind_group_layouts, 
            push_constant_ranges: &[], 
        },
    );

    let vertex_shader = device.create_shader_module(
        wgpu::include_spirv!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/vertex.spv"))
    );
    let fragment_shader = device.create_shader_module(
        wgpu::include_spirv!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/fragment.spv"))
    );

    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("RenderPipeline(Colored)"), 
            layout: Some(&pipeline_layout), 
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, 
                front_face: wgpu::FrontFace::Ccw, 
                cull_mode: Some(wgpu::Face::Back), 
                polygon_mode: wgpu::PolygonMode::Fill, 
                ..Default::default()
            }, 
            vertex: wgpu::VertexState {
                module: &vertex_shader, 
                entry_point: "main", 
                buffers: &[
                    wgpu::VertexBufferLayout {
                        step_mode: wgpu::VertexStepMode::Vertex, 
                        array_stride: mem::size_of::<ObjectVertexLayout>() as wgpu::BufferAddress, 
                        attributes: &[
                            wgpu::VertexAttribute {
                                shader_location: 0, 
                                format: wgpu::VertexFormat::Float32x3, 
                                offset: bytemuck::offset_of!(ObjectVertexLayout, position) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 1, 
                                format: wgpu::VertexFormat::Float32x3, 
                                offset: bytemuck::offset_of!(ObjectVertexLayout, normal) as wgpu::BufferAddress, 
                            }, 
                        ], 
                    }, 
                ], 
            }, 
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float, 
                depth_write_enabled: true, 
                depth_compare: wgpu::CompareFunction::Less, 
                stencil: wgpu::StencilState::default(), 
                bias: wgpu::DepthBiasState::default()
            }), 
            multisample: wgpu::MultisampleState::default(), 
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader, 
                entry_point: "main", 
                targets: &[
                    Some(wgpu::ColorTargetState {
                        blend: None, 
                        format: wgpu::TextureFormat::Bgra8Unorm, 
                        write_mask: wgpu::ColorWrites::ALL, 
                    }), 
                ], 
            }), 
            multiview: None, 
        }, 
    )
}
