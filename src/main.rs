fn main() {
    let instances = wgpu::Instance::new(wgpu::InstanceDescriptor{
        backends:wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });
}
