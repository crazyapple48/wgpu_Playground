fn main() {
    // enable wgpu logger
    env_logger::init();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    println!("-- GPUs found on this machine --");
    for adapter in instance.enumerate_adapters(wgpu::Backends::all()) {
        let info = adapter.get_info();

        println!("Name:\t\t{}", info.name);
        println!("Backend:\t\t{:?}", info.backend);
        println!("Type:\t\t{:?}", info.device_type);
        println!("Driver:\t\t{} {}", info.driver, info.driver_info);

        let limits = adapter.limits();

        println!("Max Texture Size:\t\t{}", limits.max_texture_dimension_2d);
        println!("Max Buffer Size:\t\t{}", limits.max_buffer_size);
        println!("Max bind groups:\t\t{}", limits.max_bind_groups);
        println!("Max vertex attribs:\t\t{}", limits.max_vertex_attributes);
        println!("Max compute group:\t\t{}", limits.max_compute_workgroup_size_x);

        println!();
    }
}
