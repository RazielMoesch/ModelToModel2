use crate::gpu::utils::{storage_buffer, uniform_buffer};






pub struct VertexCreator {

    pub layout_0: wgpu::BindGroupLayout,
    pub layout_1: wgpu::BindGroupLayout,
    pub layout_2: wgpu::BindGroupLayout,
    pub pipeline: wgpu::ComputePipeline,

}


impl VertexCreator {

    pub fn new( device: &wgpu::Device ) -> Self {

        let compute_only = wgpu::ShaderStages::COMPUTE;

        let layout_0 = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {

                label: Some("Vertex Creator Bind Group Layout 0"),
                entries: &[

                    storage_buffer(0, compute_only, true),
                    storage_buffer(1, compute_only, true),
                    storage_buffer(2, compute_only, true),
                    storage_buffer(3, compute_only, true)

                ]

            }
        );

        let layout_1 = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Vertex Creator Bind Group Layout 1"),
                entries: &[
                    storage_buffer(0, compute_only, false),
                    storage_buffer(1, compute_only, false),
                    storage_buffer(2, compute_only, false),
                    storage_buffer(3, compute_only, false),

                ]
            }
        );

        let layout_2 = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Vertex Creator Bind Group Layout 2"),
                entries: &[
                    uniform_buffer(0, compute_only)
                ]
            }
        );

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Vertex Creator Pipeline Layout"),
                bind_group_layouts: &[
                    Some(&layout_0),
                    Some(&layout_1),
                    Some(&layout_2),
                ],
                immediate_size: 0
            }
        );

        let module = device.create_shader_module(wgpu::include_wgsl!("create_vertices.wgsl"));

        let pipeline = device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: Some("Vertex Creator Pipeline Layout"),
                layout: Some(&pipeline_layout),
                module: &module,
                entry_point: Some("main"),
                cache: None,
                compilation_options: wgpu::PipelineCompilationOptions::default()
            }
        );


        Self {
            layout_0,
            layout_1,
            layout_2,
            pipeline
        }

    }


    pub fn record (
        &self,
        encoder: &mut wgpu::CommandEncoder,
        bg_0: &wgpu::BindGroup,
        bg_1: &wgpu::BindGroup,
        bg_2: &wgpu::BindGroup,
        num_vertices: u32,
        
    ) {


        let mut pass = encoder.begin_compute_pass(
            &wgpu::ComputePassDescriptor {
                label: Some("Vertex Creator Pipeline"),
                ..Default::default()
            }
        );

        let workgroups: u32 = (num_vertices + 63) / 64;

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, bg_0, &[]);
        pass.set_bind_group(1, bg_1, &[]);
        pass.set_bind_group(2, bg_2, &[]);
        pass.dispatch_workgroups(workgroups, 1, 1);

    }
 
}