use std::mem;
use bytemuck::{Pod, Zeroable};

use crate::object::GameObject;
use crate::resource::ShaderResource;



/// #### 한국어 </br>
/// 게임 월드에 존재하는 조명의 trait 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a trait of lighting that exists in the game world. </br>
/// 
pub trait LightObject : GameObject {
    fn texture_view_ref(&self) -> &wgpu::TextureView;
    fn get_projection_matrix(&self) -> glam::Mat4;
    fn get_view_matrix(&self) -> glam::Mat4;
}

/// #### 한국어 </br>
/// 전역 조명을 생성하는 빌더입니다. </br>
/// 
/// #### English (Translation) </br>
/// A builder that creates global lighting. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalLightBuilder {
    pub shadow_map_width: u32, 
    pub shadow_map_height: u32, 
    pub translation: glam::Vec3, 
    pub rotation: glam::Quat, 
    pub light_color: glam::Vec3, 
}

#[allow(dead_code)]
impl GlobalLightBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
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

    #[inline]
    pub fn set_shadow_map_width(mut self, shadow_map_width: u32) -> Self {
        self.shadow_map_width = shadow_map_width;
        self
    }

    #[inline]
    pub fn set_shadow_map_height(mut self, shadow_map_height: u32) -> Self {
        self.shadow_map_height = shadow_map_height;
        self
    }

    #[inline]
    pub fn set_light_color(mut self, light_color: glam::Vec3) -> Self {
        self.light_color = light_color;
        self
    }

    pub fn build(
        self, 
        uniform_bind_group_layout: &wgpu::BindGroupLayout, 
        texture_bind_group_layout: &wgpu::BindGroupLayout, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) -> GlobalLight {
        let uniform_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Uniform(GlobalLight)"), 
                mapped_at_creation: false,  
                size: mem::size_of::<GlobalLightUniformLayout>() as wgpu::BufferAddress, 
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, 
            }, 
        );

        let uniform_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("BindGroup(Uniform(GlobalLight))"), 
                layout: uniform_bind_group_layout, 
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

        let shadow_texture_view = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("Texture(GlobalLight)"), 
                size: wgpu::Extent3d {
                    width: self.shadow_map_width, 
                    height: self.shadow_map_height, 
                    depth_or_array_layers: 1, 
                }, 
                dimension: wgpu::TextureDimension::D2, 
                format: wgpu::TextureFormat::Depth32Float, 
                mip_level_count: 1, 
                sample_count: 1, 
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING, 
                view_formats: &[]
            }
        )
        .create_view(&wgpu::TextureViewDescriptor {
            ..Default::default()
        });

        let shadow_texture_sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some("Sampler(GlobalLight)"), 
                address_mode_u: wgpu::AddressMode::ClampToEdge, 
                address_mode_v: wgpu::AddressMode::ClampToEdge, 
                address_mode_w: wgpu::AddressMode::ClampToEdge, 
                mag_filter: wgpu::FilterMode::Linear, 
                min_filter: wgpu::FilterMode::Linear, 
                mipmap_filter: wgpu::FilterMode::Nearest, 
                compare: Some(wgpu::CompareFunction::LessEqual), 
                ..Default::default()
            }, 
        );

        let texture_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("BindGroup(TextureView(Shadow))"), 
                layout: texture_bind_group_layout, 
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0, 
                        resource: wgpu::BindingResource::TextureView(
                            &shadow_texture_view
                        ), 
                    }, 
                    wgpu::BindGroupEntry {
                        binding: 1, 
                        resource: wgpu::BindingResource::Sampler(
                            &shadow_texture_sampler
                        ), 
                    }, 
                ], 
            }, 
        );

        let global_light = GlobalLight {
            light_color: self.light_color, 
            shadow_map_width: self.shadow_map_width, 
            shadow_map_height: self.shadow_map_height, 
            shadow_texture_view, 
            transform: glam::Mat4::from_rotation_translation(
                self.rotation.normalize(), 
                self.translation
            ), 
            uniform_buffer, 
            uniform_bind_group, 
            texture_bind_group, 
        };
        global_light.update_resource(queue);

        return global_light;
    }
}

impl Default for GlobalLightBuilder {
    #[inline]
    fn default() -> Self {
        Self { 
            shadow_map_width: 1024, 
            shadow_map_height: 1024, 
            translation: glam::Vec3::ZERO, 
            rotation: glam::Quat::IDENTITY, 
            light_color: glam::Vec3::ONE 
        }
    }
}

/// #### 한국어 </br>
/// 게임 월드에 존재하는 전역 조명입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the global lighting that exists in the game world. </br>
/// 
#[derive(Debug)]
pub struct GlobalLight {
    light_color: glam::Vec3, 
    transform: glam::Mat4, 
    shadow_map_width: u32, 
    shadow_map_height: u32, 
    shadow_texture_view: wgpu::TextureView, 
    uniform_buffer: wgpu::Buffer, 
    pub uniform_bind_group: wgpu::BindGroup, 
    pub texture_bind_group: wgpu::BindGroup, 
}

impl GameObject for GlobalLight {
    #[inline]
    fn world_transform_ref(&self) -> &glam::Mat4 {
        &self.transform
    }

    #[inline]
    fn world_transform_mut(&mut self) -> &mut glam::Mat4 {
        &mut self.transform
    }
}

impl LightObject for GlobalLight {
    #[inline]
    fn texture_view_ref(&self) -> &wgpu::TextureView {
        &self.shadow_texture_view
    }

    fn get_projection_matrix(&self) -> glam::Mat4 {
        glam::Mat4::perspective_rh(
            90.0f32.to_radians(), 
            self.shadow_map_width as f32 / self.shadow_map_height as f32, 
            0.001, 
            1000.0
        )
    }

    fn get_view_matrix(&self) -> glam::Mat4 {
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
}

impl ShaderResource for GlobalLight {
    #[inline]
    fn update_resource(&self, queue: &wgpu::Queue) {
        let data = GlobalLightUniformLayout {
            proj_view: self.get_projection_matrix().mul_mat4(&self.get_view_matrix()), 
            direction: (self.get_look(), 0.0).into(), 
            light_color: (self.light_color, 1.0).into(), 
        };
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&data));
    }
}

/// #### 한국어 </br>
/// 쉐이더에서 사용하는 전역 조명 유니폼 데이터의 레이아웃 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the layout of the global lighting uniform data used in the shader. </br>
/// 
#[repr(C, align(16))]
#[derive(Pod, Zeroable)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalLightUniformLayout {
    pub proj_view: glam::Mat4, 
    pub direction: glam::Vec4, 
    pub light_color: glam::Vec4, 
}

impl Default for GlobalLightUniformLayout {
    #[inline]
    fn default() -> Self {
        Self { 
            proj_view: glam::Mat4::IDENTITY, 
            direction: glam::Vec4::ZERO, 
            light_color: glam::Vec4::ONE 
        }
    }
}
