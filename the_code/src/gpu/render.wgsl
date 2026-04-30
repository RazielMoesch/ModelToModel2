


struct Vertex { // 32

    pos: vec3<f32>, // 12
    _pad1: f32, // 4
    normal: vec3<f32>, // 12
    _pad2: f32, // 4

};

struct UV { //16

    uv: vec2<f32>, // 8
    _pad1: vec2<f32> // 8

};

struct Uniforms { // 80

    mvp: mat4x4<f32>, // 64
    light_source: vec3<f32>, // 12
    transition_percentage: f32 // 4
    

};

struct VertexOutput {

    @builtin(position) pos: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv_1: vec2<f32>,
    @location(2) uv_2: vec2<f32>,

};

// in from "create_vertices.wgsl"
@group(0) @binding(0) var<storage, read> vertices_1: array<Vertex>;
@group(0) @binding(1) var<storage, read> vertices_2: array<Vertex>;
@group(0) @binding(2) var<storage, read> uv_1: array<UV>;
@group(0) @binding(3) var<storage, read> uv_2: array<UV>;

// texture info
@group(1) @binding(0) var texture_1: texture_2d<f32>;
@group(1) @binding(1) var texture_2: texture_2d<f32>;
@group(1) @binding(2) var t_sampler: sampler;

// uniforms
@group(2) @binding(0) var<uniform> uniforms: Uniforms;


@vertex
fn vs(@builtin(vertex_index) idx: u32) -> VertexOutput {

    var out: VertexOutput;

    let v1 = vertices_1[idx];
    let v2 = vertices_2[idx];
    let u1 = uv_1[idx];
    let u2 = uv_2[idx];

    let pos = mix( v1.pos, v2.pos, uniforms.transition_percentage );

    out.pos = uniforms.mvp * vec4<f32>(pos, 1.0);
    out.normal = mix( v1.normal, v2.normal, uniforms.transition_percentage );
    out.uv_1 = u1.uv;
    out.uv_2 = u2.uv;

    return out;

}

@fragment
fn fs( in: VertexOutput ) -> @location(0) vec4<f32> {

    let c1 = textureSample(texture_1, t_sampler, in.uv_1);
    let c2 = textureSample(texture_2, t_sampler, in.uv_2);
    let color = mix(c1, c2, uniforms.transition_percentage);


    let normal = normalize(in.normal);


    let brightness = clamp(dot(normal, vec3<f32>(0.0, 10.0, 10.0)), 0.1, 1.0);


    return vec4<f32>(color.xyz * brightness, color.a);
}
