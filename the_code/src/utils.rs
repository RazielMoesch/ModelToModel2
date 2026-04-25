use glam::{Mat4, Vec3};
use crate::gpu::resources::{UV, Vertex};

pub fn load_glb(path: &str) -> (Vec<Vertex>, Vec<UV>) {
    let (doc, buffers, ..) = gltf::import(path).expect("Failed to Load GLB");
    let mut all_vertices = Vec::new();
    let mut all_uvs = Vec::new();

    for scene in doc.scenes() {
        for node in scene.nodes() {
            extract_node_recursive(
                &node,
                Mat4::IDENTITY,
                &buffers,
                &mut all_vertices,
                &mut all_uvs,
            );
        }
    }

    (all_vertices, all_uvs)
}

fn extract_node_recursive(
    node: &gltf::Node,
    parent_transform: Mat4,
    buffers: &[gltf::buffer::Data],
    all_vertices: &mut Vec<Vertex>,
    all_uvs: &mut Vec<UV>,
) {
    let local_transform = Mat4::from_cols_array_2d(&node.transform().matrix());
    let world_transform = parent_transform * local_transform;

    if let Some(mesh) = node.mesh() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            let pos_iter = reader.read_positions().expect("No Positions Found");
            let norm_iter = reader.read_normals().expect("No Normals Found");
            let uv_iter = reader.read_tex_coords(0)
                .expect("No UVs Found")
                .into_f32();

            for ((p, n), u) in pos_iter.zip(norm_iter).zip(uv_iter) {
                let world_pos = world_transform.transform_point3(Vec3::from(p));
                let world_norm = world_transform.transform_vector3(Vec3::from(n)).normalize();

                all_vertices.push(Vertex::new(world_pos.into(), world_norm.into()));
                all_uvs.push(UV::new(u));
            }
        }
    }

    for child in node.children() {
        extract_node_recursive(&child, world_transform, buffers, all_vertices, all_uvs);
    }
}