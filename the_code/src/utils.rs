use glam::Vec3;

use crate::gpu::resources::{UV, Vertex};

fn center_and_scale(vertices: Vec<Vertex>) -> Vec<Vertex> {
    if vertices.is_empty() { return vertices; }

    let mut min = Vec3::splat(f32::MAX);
    let mut max = Vec3::splat(f32::MIN);

    for v in &vertices {
        let p = Vec3::from(v.pos);
        min = min.min(p);
        max = max.max(p);
    }

    let center = (min + max) * 0.5;
    let extent = (max - min).max_element();
    let scale = if extent > 0.0 { 2.0 / extent } else { 1.0 };

    vertices.into_iter().map(|mut v| {
        let p = (Vec3::from(v.pos) - center) * scale;
        v.pos = p.to_array();
        v
    }).collect()
}

pub fn normalize(vertices: Vec<Vertex>) -> Vec<Vertex> {
    let mut max_dist: f32 = 0.0;
    for v in &vertices {
        let d = (v.pos[0]*v.pos[0] + v.pos[1]*v.pos[1] + v.pos[2]*v.pos[2]).sqrt();
        if d > max_dist { max_dist = d; }
    }
    
    let factor = if max_dist > 0.0 { 1.0 / max_dist } else { 1.0 };
    
    vertices.into_iter().map(|mut v| {
        v.pos[0] *= factor;
        v.pos[1] *= factor;
        v.pos[2] *= factor;
        v
    }).collect()
}

pub fn load_glb(path: &str) -> (Vec<Vertex>, Vec<UV>, Vec<u32>, Vec<u8>, u32, u32) {
    println!("[Loader] Loading file: {}", path);
    let (doc, buffers, images) = gltf::import(path).expect("Failed to load GLB");
    let mesh = doc.meshes().next().expect("No mesh found");
    let primitive = mesh.primitives().next().expect("No primitive found");
    let reader = primitive.reader(|buf| Some(&buffers[buf.index()]));

    let positions: Vec<[f32; 3]> = reader
        .read_positions()
        .expect("No positions found")
        .collect();

    let count = positions.len();

    let normals_src: Vec<[f32; 3]> = match reader.read_normals() {
        Some(n) => n.collect(),
        None => vec![[0.0, 0.0, 1.0]; count],
    };

    let uvs_src: Vec<[f32; 2]> = match reader.read_tex_coords(0) {
        Some(uvs) => uvs.into_f32().collect(),
        None => vec![[0.0, 0.0]; count],
    };

    let indices_raw: Vec<u32> = reader
        .read_indices()
        .expect("No indices found")
        .into_u32()
        .collect();

    let mut vertices = Vec::with_capacity(indices_raw.len());
    let mut uvs = Vec::with_capacity(indices_raw.len());

    for &i in &indices_raw {
        let idx = i as usize;
        vertices.push(Vertex::new(positions[idx], normals_src[idx]));
        uvs.push(UV::new(uvs_src[idx]));
    }

    let vertices = center_and_scale(vertices);

    let indices: Vec<u32> = (0..vertices.len() as u32).collect();

    let image = images.get(0).expect("No texture found in GLB");
    let width = image.width;
    let height = image.height;
    let pixels = match image.format {
        gltf::image::Format::R8G8B8 => {
            let mut rgba = Vec::with_capacity(image.pixels.len() / 3 * 4);
            for chunk in image.pixels.chunks_exact(3) {
                rgba.extend_from_slice(chunk);
                rgba.push(255);
            }
            rgba
        }
        gltf::image::Format::R8G8B8A8 => image.pixels.clone(),
        _ => image::load_from_memory(&image.pixels)
            .expect("Failed to decode texture")
            .to_rgba8()
            .into_raw(),
    };

    println!("[Loader] Finished loading {}. Vertices: {}, Indices: {}",
        path, vertices.len(), indices.len());
    (vertices, uvs, indices, pixels, width, height)
}

pub fn create_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    pixels: &[u8],
    width: u32,
    height: u32,
) -> wgpu::TextureView {
    let size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        pixels,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * width),
            rows_per_image: Some(height),
        },
        size,
    );

    texture.create_view(&wgpu::TextureViewDescriptor::default())
}