use std::fmt;
use std::mem;

use crate::object::ObjectVertexLayout;


/// #### 한국어 </br>
/// 3d 모델 메쉬의 trait 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a trait of 3D model mesh. </br>
///  
pub trait ModelMesh : fmt::Debug {
    fn bind<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>);
    fn draw<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>);
}

/// #### 한국어 </br>
/// 3D 큐브 모델의 메쉬입니다. </br>
/// 
/// #### English (Translation) </br>
/// A mesh of a 3D cube model. </br>
/// 
#[derive(Debug)]
pub struct CubeMesh {
    num_indices: u32, 
    index_buffer: wgpu::Buffer, 
    vertex_buffer: wgpu::Buffer, 
}

impl CubeMesh {
    pub fn new(
        x: f32, y: f32, z: f32, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) -> Self {
        assert!(x > 0.0 && y > 0.0 && z > 0.0);
        
        let hx = 0.5 * x;
        let hy = 0.5 * y;
        let hz = 0.5 * z;
        let mut vertices = Vec::new();
        vertices.push(ObjectVertexLayout { position: (-hx, -hy, hz).into(), normal: ( 0.0,  0.0,  1.0).into() });
        vertices.push(ObjectVertexLayout { position: ( hx, -hy,  hz).into(), normal: ( 0.0,  0.0,  1.0).into() });
        vertices.push(ObjectVertexLayout { position: ( hx,  hy,  hz).into(), normal: ( 0.0,  0.0,  1.0).into() });
        vertices.push(ObjectVertexLayout { position: (-hx,  hy,  hz).into(), normal: ( 0.0,  0.0,  1.0).into() });
        
        vertices.push(ObjectVertexLayout { position: (-hx,  hy, -hz).into(), normal: ( 0.0,  0.0, -1.0).into() });
        vertices.push(ObjectVertexLayout { position: ( hx,  hy, -hz).into(), normal: ( 0.0,  0.0, -1.0).into() });
        vertices.push(ObjectVertexLayout { position: ( hx, -hy, -hz).into(), normal: ( 0.0,  0.0, -1.0).into() });
        vertices.push(ObjectVertexLayout { position: (-hx, -hy, -hz).into(), normal: ( 0.0,  0.0, -1.0).into() });

        vertices.push(ObjectVertexLayout { position: ( hx, -hy, -hz).into(), normal: ( 1.0,  0.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: ( hx,  hy, -hz).into(), normal: ( 1.0,  0.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: ( hx,  hy,  hz).into(), normal: ( 1.0,  0.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: ( hx, -hy,  hz).into(), normal: ( 1.0,  0.0,  0.0).into() });
        
        vertices.push(ObjectVertexLayout { position: (-hx, -hy,  hz).into(), normal: (-1.0,  0.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: (-hx,  hy,  hz).into(), normal: (-1.0,  0.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: (-hx,  hy, -hz).into(), normal: (-1.0,  0.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: (-hx, -hy, -hz).into(), normal: (-1.0,  0.0,  0.0).into() });

        vertices.push(ObjectVertexLayout { position: ( hx,  hy, -hz).into(), normal: ( 0.0,  1.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: (-hx,  hy, -hz).into(), normal: ( 0.0,  1.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: (-hx,  hy,  hz).into(), normal: ( 0.0,  1.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: ( hx,  hy,  hz).into(), normal: ( 0.0,  1.0,  0.0).into() });

        vertices.push(ObjectVertexLayout { position: ( hx, -hy,  hz).into(), normal: ( 0.0, -1.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: (-hx, -hy,  hz).into(), normal: ( 0.0, -1.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: (-hx, -hy, -hz).into(), normal: ( 0.0, -1.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: ( hx, -hy, -hz).into(), normal: ( 0.0, -1.0,  0.0).into() });

        let vertex_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Vertex(Cube)"), 
                mapped_at_creation: false, 
                size: (mem::size_of::<ObjectVertexLayout>() * vertices.len()) as wgpu::BufferAddress, 
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, 
            }, 
        );
        queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&vertices));

        let indices: [u16; 36] = [
            0, 1, 2, 2, 3, 0,
            4, 5, 6, 6, 7, 4, 
            8, 9, 10, 10, 11, 8, 
            12, 13, 14, 14, 15, 12, 
            16, 17, 18, 18, 19, 16, 
            20, 21, 22, 22, 23, 20, 
        ];

        let index_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Index(Cube)"), 
                mapped_at_creation: false, 
                size: mem::size_of_val(&indices) as wgpu::BufferAddress, 
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST, 
            }, 
        );
        queue.write_buffer(&index_buffer, 0, bytemuck::cast_slice(&indices));
    
        Self { 
            num_indices: indices.len() as u32, 
            index_buffer, 
            vertex_buffer 
        }
    }
}

impl ModelMesh for CubeMesh {
    #[inline]
    fn bind<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    }

    #[inline]
    fn draw<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        rpass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}

/// #### 한국어 </br>
/// 3D 평면 모델의 메쉬 입니다. </br>
/// 
/// #### English (Translation) </br>
/// A mesh of a 3D plane model. </br>
/// 
#[derive(Debug)]
pub struct PlaneMesh {
    num_vertices: u32, 
    vertex_buffer: wgpu::Buffer, 
}

impl PlaneMesh {
    pub fn new(
        w: f32, h: f32, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) -> Self {
        assert!(w > 0.0 && h > 0.0);

        let hw = 0.5 * w;
        let hh = 0.5 * h;
        let mut vertices = Vec::new();
        vertices.push(ObjectVertexLayout { position: (-hw,  0.0, -hh).into(), normal: ( 0.0,  1.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: (-hw,  0.0,  hh).into(), normal: ( 0.0,  1.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: ( hw,  0.0, -hh).into(), normal: ( 0.0,  1.0,  0.0).into() });

        vertices.push(ObjectVertexLayout { position: ( hw,  0.0, -hh).into(), normal: ( 0.0,  1.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: (-hw,  0.0,  hh).into(), normal: ( 0.0,  1.0,  0.0).into() });
        vertices.push(ObjectVertexLayout { position: ( hw,  0.0,  hh).into(), normal: ( 0.0,  1.0,  0.0).into() });

        let vertex_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Vertex(Plane)"), 
                mapped_at_creation: false, 
                size: (mem::size_of::<ObjectVertexLayout>() * vertices.len()) as wgpu::BufferAddress, 
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, 
            }, 
        );
        queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&vertices));

        Self { 
            num_vertices: vertices.len() as u32, 
            vertex_buffer 
        }
    }
}

impl ModelMesh for PlaneMesh {
    fn bind<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    }

    fn draw<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        rpass.draw(0..self.num_vertices, 0..1);
    }
}
