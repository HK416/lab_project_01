use std::fmt;
use std::mem;
use bytemuck::{Pod, Zeroable};
use glam::Vec4Swizzles;

use crate::resource::ShaderResource;



/// #### 한국어 </br>
/// 게임 월드에 존재하는 물체의 trait 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a trait of an object that exists in the game world. </br>
/// 
pub trait GameObject : fmt::Debug {
    #[inline]
    fn get_translation(&self) -> glam::Vec3 {
        self.world_transform_ref().w_axis.xyz()
    }

    #[inline]
    fn set_translation(&mut self, translation: glam::Vec3) {
        self.world_transform_mut().w_axis.x = translation.x;
        self.world_transform_mut().w_axis.y = translation.y;
        self.world_transform_mut().w_axis.z = translation.z;
    }

    #[inline]
    fn translate_local(&mut self, distance: glam::Vec3) {
        let mat = self.world_transform_ref();
        let right = mat.x_axis.xyz().normalize();
        let up = mat.y_axis.xyz().normalize();
        let look = mat.z_axis.xyz().normalize();
        self.translate_world(right * distance.x + up * distance.y + look * distance.z)
    }

    #[inline]
    fn translate_world(&mut self, distance: glam::Vec3) {
        self.world_transform_mut().w_axis.x += distance.x;
        self.world_transform_mut().w_axis.y += distance.y;
        self.world_transform_mut().w_axis.z += distance.z;
    }

    #[inline]
    fn get_rotation(&self) -> glam::Quat {
        glam::Quat::from_mat4(self.world_transform_ref()).normalize()
    }

    #[inline]
    fn get_right(&self) -> glam::Vec3 {
        self.world_transform_ref().x_axis.xyz().normalize()
    }

    #[inline]
    fn get_up(&self) -> glam::Vec3 {
        self.world_transform_ref().y_axis.xyz().normalize()
    }

    #[inline]
    fn get_look(&self) -> glam::Vec3 {
        self.world_transform_ref().z_axis.xyz().normalize()
    }

    #[inline]
    fn set_rotation(&mut self, rotation: glam::Quat) {
        let rot = glam::Mat3::from_quat(rotation.normalize());
        let mat = self.world_transform_mut();
        mat.x_axis.x = rot.x_axis.x;
        mat.x_axis.y = rot.x_axis.y;
        mat.x_axis.z = rot.x_axis.z;

        mat.y_axis.x = rot.y_axis.x;
        mat.y_axis.y = rot.y_axis.y;
        mat.y_axis.z = rot.y_axis.z;

        mat.z_axis.x = rot.z_axis.x;
        mat.z_axis.y = rot.z_axis.y;
        mat.z_axis.z = rot.z_axis.z;
    }

    #[inline]
    fn rotate(&mut self, rotation: glam::Quat) {
        let rot = glam::Mat4::from_quat(rotation.normalize());
        *self.world_transform_mut() = self.world_transform_ref().mul_mat4(&rot);
    }

    fn world_transform_ref(&self) -> &glam::Mat4;

    fn world_transform_mut(&mut self) -> &mut glam::Mat4;
}

/// #### 한국어 </br>
/// 표준 오브젝트를 생성하는 빌더입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a builder that creates standard objects. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StdObjectBuilder {
    pub color: glam::Vec3, 
    pub rotation: glam::Quat, 
    pub translation: glam::Vec3, 
}

impl Default for StdObjectBuilder {
    #[inline]
    fn default() -> Self {
        Self { 
            color: glam::Vec3::ONE, 
            rotation: glam::Quat::IDENTITY, 
            translation: glam::Vec3::ZERO 
        }
    }
}

#[allow(dead_code)]
impl StdObjectBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn set_color(mut self, color: glam::Vec3) -> Self {
        self.color = color;
        self
    }

    #[inline]
    pub fn set_translation(mut self, translation: glam::Vec3) -> Self {
        self.translation = translation;
        self
    }

    #[inline]
    pub fn translate_local(self, distance: glam::Vec3) -> Self {
        let mat = glam::Mat3::from_quat(self.rotation.normalize());
        let right = mat.x_axis.normalize();
        let up = mat.y_axis.normalize();
        let look = mat.z_axis.normalize();
        self.translate_world(right * distance.x + up * distance.y + look * distance.z)
    }

    #[inline]
    pub fn translate_world(mut self, distance: glam::Vec3) -> Self {
        self.translation += distance;
        self
    }

    #[inline]
    pub fn set_rotation(mut self, rotation: glam::Quat) -> Self {
        self.rotation = rotation.normalize();
        self
    }

    #[inline]
    pub fn rotate(mut self, rotation: glam::Quat) -> Self {
        self.rotation = self.rotation.mul_quat(rotation.normalize());
        self
    }

    #[inline]
    pub fn set_look_at_point(mut self, point: glam::Vec3) -> Self {
        let neg_eye_direction = (self.translation - point).normalize();
        let up = glam::Mat3::from_quat(self.rotation.normalize()).y_axis.normalize();
        let mat = glam::Mat4::look_to_lh(self.translation, neg_eye_direction, up);
        self.rotation = glam::Quat::from_mat4(&mat);
        self
    }

    pub fn build(
        self, 
        bind_group_layout: &wgpu::BindGroupLayout, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) -> StdObject {
        let uniform_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Uniform(Object)"), 
                mapped_at_creation: false, 
                size: mem::size_of::<ObjectUniformLayout>() as wgpu::BufferAddress, 
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, 
            }, 
        );

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("BindGroup(Uniform(Object))"), 
                layout: bind_group_layout, 
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0, 
                        resource: wgpu::BindingResource::Buffer(
                            uniform_buffer.as_entire_buffer_binding()
                        ),
                    }, 
                ], 
            }, 
        );

        let object = StdObject { 
            color: self.color, 
            transform: glam::Mat4::from_rotation_translation(
                self.rotation, 
                self.translation
            ), 
            uniform_buffer, 
            bind_group 
        };
        object.update_resource(queue);

        return object;
    }
}

/// #### 한국어 </br>
/// 게임 월드에 존재하는 표준 오브젝트 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a standard object that exists in the game world. </br>
/// 
#[derive(Debug)]
pub struct StdObject {
    pub color: glam::Vec3, 
    pub transform: glam::Mat4, 
    pub uniform_buffer: wgpu::Buffer, 
    pub bind_group: wgpu::BindGroup, 
}

impl GameObject for StdObject {
    #[inline]
    fn world_transform_ref(&self) -> &glam::Mat4 {
        &self.transform
    }

    #[inline]
    fn world_transform_mut(&mut self) -> &mut glam::Mat4 {
        &mut self.transform
    }
}

impl ShaderResource for StdObject {
    #[inline]
    fn bind_group_ref(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    #[inline]
    fn update_resource(&self, queue: &wgpu::Queue) {
        let data = ObjectUniformLayout {
            world: self.world_transform_ref().clone(), 
            color: (self.color, 1.0).into(), 
        };
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&data));
    }
}

/// #### 한국어 </br>
/// 쉐이더에서 사용하는 큐브 오브젝트 유니폼 데이터의 레아아웃 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the layout of the cube object uniform data used in the shader. </br>
/// 
#[repr(C, align(16))]
#[derive(Pod, Zeroable)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectUniformLayout {
    pub world: glam::Mat4, 
    pub color: glam::Vec4, 
}

impl Default for ObjectUniformLayout {
    #[inline]
    fn default() -> Self {
        Self { 
            world: glam::Mat4::IDENTITY, 
            color: glam::Vec4::ONE, 
        }
    }
}

/// #### 한국어 </br>
/// 쉐이더에서 사용하는 큐브 오브젝트 버텍스 입력 데이터의 레이아웃 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the layout of the cube object vertex input data used in the shader. </br>
/// 
#[repr(C)]
#[derive(Pod, Zeroable)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectVertexLayout {
    pub position: glam::Vec3, 
    pub normal: glam::Vec3, 
}

impl Default for ObjectVertexLayout {
    #[inline]
    fn default() -> Self {
        Self {
            position: glam::Vec3::ZERO, 
            normal: glam::Vec3::ZERO, 
        }
    }
}
