fn main() {
    // enable wgpu logger
    env_logger::init();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
}
