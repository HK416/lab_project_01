/// #### 한국어 </br>
/// 쉐이더 리소스가 사용하는 trait 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the trait used by the shader resource. </br>
/// 
pub trait ShaderResource {
    fn update_resource(&self, queue: &wgpu::Queue);
}