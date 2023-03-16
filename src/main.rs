use std::borrow::Cow;
use wgpu::util::DeviceExt;

async fn run() {

    let numbers = vec![[0.03682002, 0.22507057, 0.3644007],[0.2,2.0,20.0],[0.3,3.0,30.0],[0.4,4.0,40.0],[0.5,5.0,50.0]];
    let ret = execute_gpu(&numbers).await.unwrap();
    println!("ret: {:?}", ret);
}

async fn execute_gpu(numbers: &Vec<[f32; 3]>) -> Option<Vec<[f32; 3]>> {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await?;
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .unwrap();
    //let info = adapter.get_info();

    execute_gpu_inner(&device, &queue, numbers).await
}

async fn execute_gpu_inner(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    numbers: &Vec<[f32; 3]>,
) -> Option<Vec<[f32; 3]>> {
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });
    // *3 is because 3 values per array, this will need changed if alpha is used.
    let slice_size = numbers.len() * 3 * std::mem::size_of::<u32>();
    let size = slice_size as wgpu::BufferAddress;

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(numbers),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &cs_module,
        entry_point: "main",
    });

    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("start gpu");
        cpass.dispatch_workgroups(numbers.len() as u32, 1, 1); 
    }

    encoder.copy_buffer_to_buffer(&storage_buffer, 0, &staging_buffer, 0, size);
    queue.submit(Some(encoder.finish()));
    let buffer_slice = staging_buffer.slice(..);
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    device.poll(wgpu::Maintain::Wait);

    if let Some(Ok(())) = receiver.receive().await {
        let data = buffer_slice.get_mapped_range();
        let result = bytemuck::cast_slice(&data).to_vec();
        drop(data);
        staging_buffer.unmap(); 
        Some(result)
    } else {
        panic!("failed to run compute on gpu!")
    }
}

fn main() {
    env_logger::init();
    pollster::block_on(run());
}

