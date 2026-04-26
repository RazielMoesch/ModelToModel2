use wgpu::util::DeviceExt;





pub fn create_depth_texture_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {

    let size = wgpu::Extent3d {

        width: config.width,
        height: config.height,
        depth_or_array_layers: 1

    };

    let texture = device.create_texture(
        &wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[]
        }
    );

    texture.create_view(&wgpu::TextureViewDescriptor::default())


}

pub fn storage_buffer(binding: u32, visibility: wgpu::ShaderStages, read_only: bool ) -> wgpu::BindGroupLayoutEntry {

    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: read_only },
            has_dynamic_offset: false,
            min_binding_size: None
        },
        count: None
    }

}

pub fn uniform_buffer(binding: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {

    wgpu::BindGroupLayoutEntry {
                        binding: binding,
                        visibility: visibility,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset:  false,
                            min_binding_size: None
                        },
                        count: None
                    }

}


pub fn texture_entry(binding: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {

    wgpu::BindGroupLayoutEntry {

        binding: binding,
        visibility: visibility,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false
        },
        count: None
    }

}

pub fn sampler_entry(binding: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {

    wgpu::BindGroupLayoutEntry {
        binding: binding,
        visibility: visibility,
        ty: wgpu::BindingType::Sampler(
            wgpu::SamplerBindingType::Filtering,
        ),
        count: None
    }

}



pub fn create_buffer_init(device: &wgpu::Device, contents: &[u8], usage: wgpu::BufferUsages) -> wgpu::Buffer {

    device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label:  None,
            contents:  contents,
            usage: usage
        }
    )

}

pub fn create_buffer(device: &wgpu::Device, size: u64, usage: wgpu::BufferUsages) -> wgpu::Buffer {

    device.create_buffer(
        &wgpu::BufferDescriptor {
            label: None,
            size: size,
            usage: usage,
            mapped_at_creation: false
        }
    )

}



pub fn create_bind_group(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, entries: &[wgpu::BindGroupEntry]) -> wgpu::BindGroup {

    device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            label: None,
            layout: layout,
            entries: entries
        }
    )

}

pub fn buffer_binding_entry( buffer: &wgpu::Buffer , binding: u32 ) -> wgpu::BindGroupEntry {

    wgpu::BindGroupEntry { binding, resource: buffer.as_entire_binding() }
    

}

pub fn texture_binding_entry( view: &wgpu::TextureView , binding: u32 ) -> wgpu::BindGroupEntry {

    wgpu::BindGroupEntry { binding, resource: wgpu::BindingResource::TextureView(view) }
    

}

pub fn sampler_binding_entry ( sampler: &wgpu::Sampler, binding: u32) -> wgpu::BindGroupEntry {
    wgpu::BindGroupEntry { binding, resource: wgpu::BindingResource::Sampler(sampler) }
}

