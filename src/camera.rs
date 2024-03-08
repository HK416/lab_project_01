use std::mem;
use bytemuck::{Pod, Zeroable};

use crate::{object::GameObject, resource::ShaderResource};



/// #### 한국어 </br>
/// 게임 월드에 존재하는 카메라의 trait 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a trait of the camera that exists in the game world. </br>
/// 
pub trait GameCameraObject : GameObject {
    fn view_transform(&self) -> glam::Mat4;
    fn projection_transform(&self) -> glam::Mat4;
}

/// #### 한국어 </br>
/// 원근 투영 카메라를 생성하는 빌더 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a builder that creates a perspective projection camera. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerspectiveCameraBuilder {
    pub translation: glam::Vec3, 
    pub rotation: glam::Quat, 
    pub fov_y_radian: f32, 
    pub width: f32, 
    pub height: f32, 
    pub near_z: f32, 
    pub far_z: f32, 
}

impl Default for PerspectiveCameraBuilder {
    #[inline]
    fn default() -> Self {
        Self { 
            translation: glam::Vec3::ZERO, 
            rotation: glam::Quat::IDENTITY, 
            fov_y_radian: 60.0f32.to_radians(), 
            width: 800.0, 
            height: 600.0, 
            near_z: 0.001, 
            far_z: 1000.0 
        }
    }
}

#[allow(dead_code)]
impl PerspectiveCameraBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn set_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    #[inline]
    pub fn set_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    #[inline]
    pub fn set_translation(mut self, translation: glam::Vec3) -> Self {
        self.translation = translation;
        self
    }
    
    #[inline]
    pub fn translate_world(mut self, distance: glam::Vec3) -> Self {
        self.translation += distance;
        self
    }

    #[inline]
    pub fn translate_local(self, distance: glam::Vec3) -> Self {
        let rot = glam::Mat3::from_quat(self.rotation.normalize());
        let right = rot.x_axis.normalize();
        let up = rot.y_axis.normalize();
        let look = rot.z_axis.normalize();
        self.translate_world(right * distance.x + up * distance.y + look * distance.z)
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

    pub fn build(
        self, 
        bind_group_layout: &wgpu::BindGroupLayout, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) -> PerspectiveCamera {
        let uniform_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Uniform(PerspectiveCamera)"), 
                mapped_at_creation: false, 
                size: mem::size_of::<CameraUniformLayout>() as wgpu::BufferAddress, 
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, 
            }, 
        );

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("BindGroup(PerspectiveCamera)"), 
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

        let camera = PerspectiveCamera {
            transform: glam::Mat4::from_rotation_translation(
                self.rotation.normalize(), 
                self.translation
            ), 
            fov_y_radian: self.fov_y_radian, 
            aspect_ratio: self.width / self.height, 
            near_z: self.near_z, 
            far_z: self.far_z, 
            uniform_buffer, 
            uniform_bind_group: bind_group, 
        };
        camera.update_resource(queue);

        return camera;
    }
}

/// #### 한국어 </br>
/// 게임 월드에 존재하는 원근 투영 카메라입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a perspective projection camera that exists in the game world. </br>
/// 
#[derive(Debug)]
pub struct PerspectiveCamera {
    transform: glam::Mat4, 
    fov_y_radian: f32, 
    aspect_ratio: f32, 
    near_z: f32, 
    far_z: f32, 
    uniform_buffer: wgpu::Buffer, 
    pub uniform_bind_group: wgpu::BindGroup, 
}

impl GameObject for PerspectiveCamera {
    #[inline]
    fn world_transform_ref(&self) -> &glam::Mat4 {
        &self.transform
    }

    #[inline]
    fn world_transform_mut(&mut self) -> &mut glam::Mat4 {
        &mut self.transform
    }
}

impl GameCameraObject for PerspectiveCamera {
    fn view_transform(&self) -> glam::Mat4 {
        let right = self.get_right();
        let up = self.get_up();
        let look = self.get_look();
        let position = self.get_translation();
        return glam::mat4(
            glam::vec4(right.x, up.x, look.x, 0.0), 
            glam::vec4(right.y, up.y, look.y, 0.0), 
            glam::vec4(right.z, up.z, look.z, 0.0), 
            glam::vec4(-position.dot(right), -position.dot(up), -position.dot(look), 1.0)
        );
    }

    #[inline]
    fn projection_transform(&self) -> glam::Mat4 {
        glam::Mat4::perspective_rh(self.fov_y_radian, self.aspect_ratio, self.near_z, self.far_z)
    }
}

impl ShaderResource for PerspectiveCamera {
    #[inline]
    fn update_resource(&self, queue: &wgpu::Queue) {
        let data = CameraUniformLayout {
            view: self.view_transform(), 
            projection: self.projection_transform(), 
            position: (self.get_translation(), 0.0).into(), 
        };
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&data));
    }
}

/// #### 한국어 </br>
/// 쉐이더에서 사용하는 카메라 유니폼 데이터의 레아아웃 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the layout of the camera uniform data used in the shader. </br>
/// 
#[repr(C, align(16))]
#[derive(Pod, Zeroable)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraUniformLayout {
    pub view: glam::Mat4, 
    pub projection: glam::Mat4, 
    pub position: glam::Vec4, 
}

impl Default for CameraUniformLayout {
    #[inline]
    fn default() -> Self {
        Self { 
            view: glam::Mat4::IDENTITY, 
            projection: glam::Mat4::IDENTITY, 
            position: glam::Vec4::ZERO, 
        }
    }
}
