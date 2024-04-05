fn main() {
    let instances = wgpu::Instance::new(wgpu::InstanceDescriptor{
        backends:wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    for adapter in instances.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adapter.get_info())
    }
}
