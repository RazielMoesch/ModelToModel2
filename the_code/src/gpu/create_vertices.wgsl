struct Vertex {
    pos: vec3<f32>,
    _pad1: f32,
    normal: vec3<f32>,
    _pad2: f32,
};

struct UV {
    uv: vec2<f32>,
    _pad1: vec2<f32>
};

struct Uniforms {
    num_vertices: u32,
    len_1: u32,
    len_2: u32,
    _pad1: u32
};

// In
@group(0) @binding(0) var<storage, read> vertices_1: array<Vertex>;
@group(0) @binding(1) var<storage, read> vertices_2: array<Vertex>;
@group(0) @binding(2) var<storage, read> uv_1: array<UV>;
@group(0) @binding(3) var<storage, read> uv_2: array<UV>;

// Out
@group(1) @binding(0) var<storage, read_write> vertices_1_out: array<Vertex>;
@group(1) @binding(1) var<storage, read_write> vertices_2_out: array<Vertex>;
@group(1) @binding(2) var<storage, read_write> uv_1_out: array<UV>;
@group(1) @binding(3) var<storage, read_write> uv_2_out: array<UV>;

// Uniforms
@group(2) @binding(0) var<uniform> uniforms: Uniforms;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {

    let gid = id.x;

    if (gid >= uniforms.num_vertices) { return; }

    let num_triangles_out = uniforms.num_vertices / 3u;
    let triangle_idx = gid / 3u;
    let vertex_in_triangle = gid % 3u;

    if (uniforms.len_1 >= uniforms.len_2) {

        let num_triangles_2 = uniforms.len_2 / 3u;
        let src_triangle_2 = min(
            u32(f32(triangle_idx) * f32(num_triangles_2) / f32(num_triangles_out)),
            num_triangles_2 - 1u
        );

        vertices_1_out[gid] = vertices_1[gid];
        uv_1_out[gid] = uv_1[gid];

        vertices_2_out[gid] = vertices_2[src_triangle_2 * 3u + vertex_in_triangle];
        uv_2_out[gid] = uv_2[src_triangle_2 * 3u + vertex_in_triangle];

    } else {

        let num_triangles_1 = uniforms.len_1 / 3u;
        let src_triangle_1 = min(
            u32(f32(triangle_idx) * f32(num_triangles_1) / f32(num_triangles_out)),
            num_triangles_1 - 1u
        );

        vertices_1_out[gid] = vertices_1[src_triangle_1 * 3u + vertex_in_triangle];
        uv_1_out[gid] = uv_1[src_triangle_1 * 3u + vertex_in_triangle];

        vertices_2_out[gid] = vertices_2[gid];
        uv_2_out[gid] = uv_2[gid];

    }
}