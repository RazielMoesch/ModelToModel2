

use wgpu::{FragmentState, VertexState};

use crate::gpu::utils::{sampler_entry, storage_buffer, texture_entry, uniform_buffer};





pub struct Render {

    pub layout_0: wgpu::BindGroupLayout,
    pub layout_1: wgpu::BindGroupLayout,
    pub layout_2: wgpu::BindGroupLayout,
    pub pipeline: wgpu::RenderPipeline

}

impl Render {

    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {

        let vertex_only = wgpu::ShaderStages::VERTEX;
        let fragment_only = wgpu::ShaderStages::FRAGMENT;
        let both = wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT;

        let layout_0 = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Render Bind Group Layout 0"),
                entries: &[
                    storage_buffer(0, vertex_only, true),
                    storage_buffer(1, vertex_only, true),
                    storage_buffer(2, vertex_only, true),
                    storage_buffer(3, vertex_only, true),
                ]   
            }
        );

        let layout_1 = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {

                label: Some("Render Bind Group Layout 1"),
                entries: &[
                    texture_entry(0, fragment_only),
                    texture_entry(1, fragment_only),
                    sampler_entry(2, fragment_only)
                ]

            }
        );

        let layout_2 = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Render Bind Group Layout 1"),
                entries: &[
                    uniform_buffer(0, both)
                ]
            }
        );

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    Some(&layout_0),
                    Some(&layout_1),
                    Some(&layout_2)
                ],
                immediate_size: 0
            }
        );

        let module = device.create_shader_module(wgpu::include_wgsl!("render.wgsl"));

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &module,
                    entry_point: Some("vs"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &[]
                },
                fragment: Some(
                    FragmentState {
                        module: &module,
                        entry_point: Some("fs"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        targets: &[
                            Some(
                                wgpu::ColorTargetState {
                                    format: config.format,
                                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                                    write_mask: wgpu::ColorWrites::ALL
                                }
                            )
                        ]
                    }
                ),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    cull_mode: Some(wgpu::Face::Back),
                    ..Default::default()
                },
                depth_stencil: Some(
                    wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: Some(true),
                        depth_compare: Some(wgpu::CompareFunction::LessEqual),
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default()
                    }
                ),
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None
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
        view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        bg_0: &wgpu::BindGroup,
        bg_1: &wgpu::BindGroup,
        bg_2: &wgpu::BindGroup,
        index_buffer: &wgpu::Buffer,
        num_indices: u32
    ) {


        {

            let mut pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {

                    label: Some("Render Pass"),
                    color_attachments: &[
                        Some(
                            wgpu::RenderPassColorAttachment {
                                view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                    store: wgpu::StoreOp::Store
                                },
                                depth_slice: None
                            }
                        )
                    ],
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachment {
                            view: depth_texture_view,
                            depth_ops: Some(
                                wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(1.0),
                                    store: wgpu::StoreOp::Store,
                                }
                            ),
                            stencil_ops: None
                        }
                    ),
                    ..Default::default()

                }
            );


            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, Some(bg_0), &[]);
            pass.set_bind_group(1, Some(bg_1), &[]);
            pass.set_bind_group(2, Some(bg_2), &[]);
            pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..num_indices, 0, 0..1);


        }


    }

}