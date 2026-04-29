



use crate::gpu::{render::Render, vertex_creator::VertexCreator};
use crate::gpu::resources::{RenderUniforms, UV, Vertex, VertexCreatorUniforms};
use crate::gpu::utils::{buffer_binding_entry, create_bind_group, create_buffer, create_buffer_init, sampler_binding_entry, texture_binding_entry};




pub struct CreateVerticesBuffers {
    pub v1: wgpu::Buffer,
    pub v2: wgpu::Buffer,
    pub uv1: wgpu::Buffer,
    pub uv2: wgpu::Buffer,

    pub v1_out: wgpu::Buffer,
    pub v2_out: wgpu::Buffer,
    pub uv1_out: wgpu::Buffer,
    pub uv2_out: wgpu::Buffer,

    pub uniforms: wgpu::Buffer,
}

pub struct RenderBuffers {
    pub v1: wgpu::Buffer,
    pub v2: wgpu::Buffer,
    pub uv2: wgpu::Buffer,
    pub uv1: wgpu::Buffer,
    pub uniforms: wgpu::Buffer,
    pub index: wgpu::Buffer,
}

pub struct BindGroups3 {
    bg0:  wgpu::BindGroup,
    bg1: wgpu::BindGroup,
    bg2: wgpu::BindGroup,
}


pub struct Transitioner {


    pub create_vertices_buffers: CreateVerticesBuffers,
    pub render_buffers: RenderBuffers,

    pub create_vertices_bind_groups: BindGroups3,
    pub render_bind_groups: BindGroups3,

    pub num_vertices:  u32,
    pub num_indices: u32
}


impl Transitioner {

    pub fn new(
        device: &wgpu::Device,
        vertex_creator: &VertexCreator,
        render: &Render,
        indices: &[u32],
        mesh_1: &[Vertex],
        mesh_2: &[Vertex],
        uv_1: &[UV],
        uv_2: &[UV],
        tex1_view: &wgpu::TextureView,
        tex2_view: &wgpu::TextureView,
        sampler: &wgpu::Sampler
    ) -> Self {

        let num_vertices: u32;
        let v_size = std::mem::size_of::<Vertex>();

        if mesh_1.len() > mesh_2.len() {

            num_vertices = mesh_1.len() as u32;

        }

        else {
            num_vertices = mesh_2.len() as u32;
        }

        let v_total_size = (v_size as u32 * num_vertices) as u64;
        let uv_total_size = ( num_vertices * std::mem::size_of::<UV>() as u32) as u64;
        

        let storage = wgpu::BufferUsages::STORAGE;
        let storage_copysrc = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST; 
        let uniform = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;
        let index = wgpu::BufferUsages::INDEX;

        let create_vertices_buffers = CreateVerticesBuffers {
            
            v1: create_buffer_init(device, bytemuck::cast_slice(mesh_1), storage),
            v2: create_buffer_init(device, bytemuck::cast_slice(mesh_2), storage),
            uv1: create_buffer_init(device, bytemuck::cast_slice(uv_1), storage),
            uv2: create_buffer_init(device, bytemuck::cast_slice(uv_2), storage),
            v1_out: create_buffer(device, v_total_size, storage_copysrc),
            v2_out: create_buffer(device, v_total_size, storage_copysrc),
            uv1_out: create_buffer(device, uv_total_size, storage_copysrc),
            uv2_out: create_buffer(device, uv_total_size, storage_copysrc),
            uniforms:  create_buffer(device, 16, uniform)

        };

        let create_vertices_bind_groups = BindGroups3 {
            bg0: create_bind_group(device, &vertex_creator.layout_0, 
                &[
                    buffer_binding_entry(&create_vertices_buffers.v1, 0),
                    buffer_binding_entry(&create_vertices_buffers.v2, 1),
                    buffer_binding_entry(&create_vertices_buffers.uv1, 2),
                    buffer_binding_entry(&create_vertices_buffers.uv2, 3)
                ]
            ),
            bg1: create_bind_group(device, &vertex_creator.layout_1, 
            &[
                buffer_binding_entry(&create_vertices_buffers.v1_out, 0),
                buffer_binding_entry(&create_vertices_buffers.v2_out, 1),
                buffer_binding_entry(&create_vertices_buffers.uv1_out, 2),
                buffer_binding_entry(&create_vertices_buffers.uv2_out, 3),
            ]),
            bg2:  create_bind_group(device, &vertex_creator.layout_2, 
            &[
                buffer_binding_entry(&create_vertices_buffers.uniforms, 0)
            ])
        };

        let render_buffers = RenderBuffers {
            v1: create_buffer(device, v_total_size, storage_copysrc),
            v2: create_buffer(device, v_total_size, storage_copysrc),
            uv1: create_buffer(device, uv_total_size, storage_copysrc),
            uv2: create_buffer(device, uv_total_size, storage_copysrc),
            uniforms: create_buffer(device, 80, uniform),
            index: create_buffer_init(device, bytemuck::cast_slice(indices), index)
        };

        let render_bind_groups = BindGroups3 {
            bg0: create_bind_group(device, &render.layout_0, 
            &[
                buffer_binding_entry(&render_buffers.v1, 0),
                buffer_binding_entry(&render_buffers.v2, 1),
                buffer_binding_entry(&render_buffers.uv1, 2),
                buffer_binding_entry(&render_buffers.uv2, 3),
            ]),
            bg1: create_bind_group(device, &render.layout_1, 
            &[
                texture_binding_entry(tex1_view, 0),
                texture_binding_entry(tex2_view, 1),
                sampler_binding_entry(sampler, 2)
            ]),
            bg2: create_bind_group(device, &render.layout_2, 
            &[
                buffer_binding_entry(&render_buffers.uniforms, 0)
            ])
        };


        Self {
            create_vertices_bind_groups,
            create_vertices_buffers,
            render_bind_groups,
            render_buffers,
            num_vertices,
            num_indices: indices.len() as u32
        }



    }

    pub fn record_create_vertices(
        &self,
        vertex_creator: &VertexCreator,
        encoder:  &mut  wgpu::CommandEncoder,
    ) {


        vertex_creator.record(
            encoder,
            &self.create_vertices_bind_groups.bg0,
            &self.create_vertices_bind_groups.bg1,
            &self.create_vertices_bind_groups.bg2,
            self.num_vertices,
        );

    }

    pub fn record_render(
        &self,
        render: &Render,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
    ) {

        render.record(
            encoder,
            view,
            depth_texture_view,
            &self.render_bind_groups.bg0,
            &self.render_bind_groups.bg1,
            &self.render_bind_groups.bg2,
            &self.render_buffers.index,
            self.num_indices

        )

    }

    pub fn update_render_uniforms(&mut self, queue: &wgpu::Queue, new_uniforms: &RenderUniforms) {
        queue.write_buffer(&self.render_buffers.uniforms, 0, bytemuck::cast_slice(&[new_uniforms.clone()]));
    }

    pub fn update_create_vertices_uniforms(&mut self, queue: &wgpu::Queue, new_uniforms: &VertexCreatorUniforms) {
        queue.write_buffer(&self.create_vertices_buffers.uniforms, 0, bytemuck::cast_slice(&[new_uniforms.clone()]));
    }
}