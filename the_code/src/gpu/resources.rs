

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Zeroable, Pod, Clone, Copy)]
pub struct Vertex {

    pub pos: [ f32; 3 ],
    pub _pad1: f32,
    pub normal: [ f32; 3 ],
    _pad2: f32

}

impl Vertex {

    pub fn new( pos: [f32; 3], normal: [f32; 3] ) -> Self {

        Self {
            pos: pos,
            _pad1: 0.0,
            normal: normal,
            _pad2: 0.0
            
        }

    }

}


#[repr(C)]
#[derive(Zeroable, Pod, Clone, Copy)]
pub struct UV {

    uv: [f32; 2],
    _pad: [f32; 2]

}

impl UV {

    pub fn new(uv: [f32; 2]) -> Self {

        Self {
            uv,
            _pad: [0.0, 0.0]
        }

    }

}

#[repr(C)]
#[derive(Zeroable, Pod, Clone, Copy)]
pub struct VertexCreatorUniforms {

    num_vertices: u32,
    len_1: u32,
    len_2: u32,
    _pad1: u32,

}

impl VertexCreatorUniforms {

    pub fn new( num_vertices: u32, len_1: u32, len_2: u32 ) -> Self {

        Self {

            num_vertices,
            len_1,
            len_2,
            _pad1: 0

        }

    }

}


#[repr(C)]
#[derive(Zeroable, Pod, Clone, Copy)]
pub struct RenderUniforms {
    mvp: [ [f32; 4]; 4 ],
    light_source: [f32; 3],
    transition_percentage: f32
}

impl RenderUniforms {

    pub fn new( mvp: [[f32;4]; 4], light_source: [f32; 3], transition_percentage: f32 ) -> Self {

        Self {
            mvp,
            light_source,
            transition_percentage
        }

    }

}


