

use crate::gpu::{render::Render, vertex_creator::VertexCreator};






struct CreateVerticesBuffers {
    v1: wgpu::Buffer,
    v2: wgpu::Buffer,
    uv1: wgpu::Buffer,
    uv2: wgpu::Buffer,

    v1_out: wgpu::Buffer,
    v2_out: wgpu::Buffer,
    uv1_out: wgpu::Buffer,
    uv2_out: wgpu::Buffer,

    uniforms: wgpu::Buffer,
}

struct RenderBuffers {
    v1: wgpu::Buffer,
    v2: wgpu::Buffer,
    uv1: wgpu::Buffer,
    uv2: wgpu::Buffer,
    uniforms: wgpu::Buffer,
    index: wgpu::Buffer,
}

struct BindGroups_3 {
    bg0:  wgpu::BindGroup,
    bg1: wgpu::BindGroup,
    bg2: wgpu::BindGroup,
}


pub struct Transitioner {

    create_vertices_buffers: CreateVerticesBuffers,
    render_buffers: RenderBuffers,

    create_vertices_bind_groups: BindGroups_3,
    render_bind_groups: BindGroups_3,


}


impl Transitioner {

    pub fn new(
        device: &wgpu::Device,
        vertex_creator: VertexCreator,
        render: Render,
        
    )

}