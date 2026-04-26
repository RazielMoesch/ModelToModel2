use crate::gpu::resources::{UV, Vertex};

pub fn load_glb(path: &str) -> (Vec<Vertex>, Vec<UV>, Vec<u32>, Vec<u8>) {
    let (doc, buffers, images) = gltf::import(path).expect("Failed to Load GLB");

    let mesh = doc.meshes().next().expect("No mesh found");
    let primitive = mesh.primitives().next().expect("No primitive found");
    
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

    let pos_iter = reader.read_positions().expect("No Positions Found");
    let norm_iter = reader.read_normals().expect("No Normals Found");
    let uv_iter = reader.read_tex_coords(0)
        .expect("No UVs Found")
        .into_f32();

    let mut vertices = Vec::new();
    let mut uvs = Vec::new();

    for ((p, n), u) in pos_iter.zip(norm_iter).zip(uv_iter) {
        vertices.push(Vertex::new(p, n));
        uvs.push(UV::new(u));
    }

    let indices = reader
        .read_indices()
        .map(|indices| indices.into_u32().collect())
        .expect("No Indices Found");

    let texture_data = images.get(0)
        .expect("No texture found")
        .pixels
        .clone();

    (vertices, uvs, indices, texture_data)
}