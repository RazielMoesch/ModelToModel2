use std::sync::Arc;
use wgpu::CurrentSurfaceTexture;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop,
    window::Window,
};

use crate::{
    gpu::{
        camera::Camera,
        engine::Engine,
        render::Render,
        resources::{RenderUniforms, UV, Vertex, VertexCreatorUniforms},
        transitioner::Transitioner,
        utils::create_depth_texture_view,
        vertex_creator::VertexCreator,
    },
    utils::create_texture,
};

pub struct AppState {
    pub engine: Engine,
    pub vertex_creator: VertexCreator,
    pub render: Render,
    pub transitioner: Transitioner,
    pub depth_view: wgpu::TextureView,
    pub transition_percentage: f32,
    pub vertex_creator_uniforms: VertexCreatorUniforms,
    pub render_uniforms: RenderUniforms,
    pub camera: Camera,
    pub is_dragging: bool,
    pub is_panning: bool,
}

impl AppState {
    pub async fn new(
        window: Arc<Window>,
        indices: &[u32],
        mesh1: &[Vertex],
        mesh2: &[Vertex],
        uv1: &[UV],
        uv2: &[UV],
        tex1: &[u8],
        tex2: &[u8],
        tex1_size: (u32, u32),
        tex2_size: (u32, u32),
    ) -> Self {
        let engine = Engine::new(window).await;
        let vertex_creator = VertexCreator::new(&engine.device);
        let render = Render::new(&engine.device, &engine.config);
        let depth_texture_view = create_depth_texture_view(&engine.device, &engine.config);

        let sampler = engine.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::MirrorRepeat,
            address_mode_v: wgpu::AddressMode::MirrorRepeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let transitioner = Transitioner::new(
            &engine.device,
            &vertex_creator,
            &render,
            indices,
            mesh1,
            mesh2,
            uv1,
            uv2,
            &create_texture(&engine.device, &engine.queue, tex1, tex1_size.0, tex1_size.1),
            &create_texture(&engine.device, &engine.queue, tex2, tex2_size.0, tex2_size.1),
            &sampler,
        );

        let vertex_creator_uniforms = VertexCreatorUniforms::new(
            transitioner.num_vertices,
            mesh1.len() as u32,
            mesh2.len() as u32,
        );

        let size = PhysicalSize::new(engine.config.width, engine.config.height);
        let camera = Camera::new(size);
        let render_uniforms = RenderUniforms::new(camera.matrix().to_cols_array_2d(), [1.0, 1.0, 1.0], 0.0);

        Self {
            engine,
            vertex_creator,
            render,
            transitioner,
            depth_view: depth_texture_view,
            transition_percentage: 0.0,
            vertex_creator_uniforms,
            render_uniforms,
            camera,
            is_dragging: false,
            is_panning: false,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.engine.resize(size);
        self.camera.resize(size);
        self.depth_view = create_depth_texture_view(&self.engine.device, &self.engine.config);
    }

    pub fn update_transition_percentage(&mut self, new: f32) {
        self.transition_percentage = new.clamp(0.0, 1.0);
        self.render_uniforms.transition_percentage = self.transition_percentage;
        self.transitioner.update_render_uniforms(&self.engine.queue, &self.render_uniforms);
    }
}

pub struct App {
    pub window: Option<Arc<Window>>,
    pub state: Option<AppState>,
    indices: Vec<u32>,
    mesh1: Vec<Vertex>,
    mesh2: Vec<Vertex>,
    uv1: Vec<UV>,
    uv2: Vec<UV>,
    tex1: Vec<u8>,
    tex2: Vec<u8>,
    tex1_size: (u32, u32),
    tex2_size: (u32, u32),
}

impl App {
    pub fn new(
        indices: &[u32],
        mesh1: &[Vertex],
        mesh2: &[Vertex],
        uv1: &[UV],
        uv2: &[UV],
        tex1: &[u8],
        tex2: &[u8],
        tex1_size: (u32, u32),
        tex2_size: (u32, u32),
    ) -> Self {
        Self {
            window: None,
            state: None,
            indices: indices.to_vec(),
            mesh1: mesh1.to_vec(),
            mesh2: mesh2.to_vec(),
            uv1: uv1.to_vec(),
            uv2: uv2.to_vec(),
            tex1: tex1.to_vec(),
            tex2: tex2.to_vec(),
            tex1_size,
            tex2_size,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes().with_title("Transitioner Demo");
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

            let state = pollster::block_on(AppState::new(
                window.clone(),
                &self.indices,
                &self.mesh1,
                &self.mesh2,
                &self.uv1,
                &self.uv2,
                &self.tex1,
                &self.tex2,
                self.tex1_size,
                self.tex2_size,
            ));

            self.window = Some(window);
            self.state = Some(state);

            if let Some(state) = self.state.as_mut() {
                let mut encoder = state.engine.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                state.transitioner.update_create_vertices_uniforms(&state.engine.queue, &state.vertex_creator_uniforms);
                state.transitioner.record_create_vertices(&state.vertex_creator, &mut encoder);
                state.engine.queue.submit(std::iter::once(encoder.finish()));
            }

            self.window.as_ref().unwrap().request_redraw();
        }
    }

    fn device_event(&mut self, _: &event_loop::ActiveEventLoop, _: winit::event::DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            if let Some(state) = self.state.as_mut() {
                if state.is_dragging || state.is_panning {
                    if state.is_dragging {
                        state.camera.rotate(delta.0 as f32, delta.1 as f32);
                    } else {
                        state.camera.pan(delta.0 as f32, delta.1 as f32);
                    }
                    self.window.as_ref().unwrap().request_redraw();
                }
            }
        }
    }

    fn window_event(&mut self, event_loop: &event_loop::ActiveEventLoop, _: winit::window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::MouseInput { state, button, .. } => match button {
                MouseButton::Left => {
                    if let Some(app_state) = self.state.as_mut() {
                        app_state.is_dragging = state == ElementState::Pressed;
                    }
                }
                MouseButton::Right => {
                    if let Some(app_state) = self.state.as_mut() {
                        app_state.is_panning = state == ElementState::Pressed;
                    }
                }
                _ => {}
            },
            WindowEvent::MouseWheel { delta, .. } => {
                if let Some(state) = self.state.as_mut() {
                    let scroll = match delta {
                        MouseScrollDelta::LineDelta(_, y) => y,
                        MouseScrollDelta::PixelDelta(pos) => pos.y as f32 * 0.1,
                    };
                    state.camera.zoom(scroll);
                    self.window.as_ref().unwrap().request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                if key_event.state == ElementState::Pressed {
                    if let Some(state) = self.state.as_mut() {
                        let step = 0.05;
                        match key_event.logical_key {
                            winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowRight) => {
                                state.update_transition_percentage(state.transition_percentage + step);
                                self.window.as_ref().unwrap().request_redraw();
                            }
                            winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowLeft) => {
                                state.update_transition_percentage(state.transition_percentage - step);
                                self.window.as_ref().unwrap().request_redraw();
                            }
                            _ => {}
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = self.state.as_mut() {
                    let output = match state.engine.surface.get_current_texture() {
                        CurrentSurfaceTexture::Success(texture) => texture,
                        _ => return,
                    };
                    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder = state.engine.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                    
                    state.transitioner.record_create_vertices(&state.vertex_creator, &mut encoder);

                    let v_total_size = state.transitioner.create_vertices_buffers.v1_out.size();
                    let uv_total_size = state.transitioner.create_vertices_buffers.uv1_out.size();

                    encoder.copy_buffer_to_buffer(&state.transitioner.create_vertices_buffers.v1_out, 0, &state.transitioner.render_buffers.v1, 0, v_total_size);
                    encoder.copy_buffer_to_buffer(&state.transitioner.create_vertices_buffers.v2_out, 0, &state.transitioner.render_buffers.v2, 0, v_total_size);
                    encoder.copy_buffer_to_buffer(&state.transitioner.create_vertices_buffers.uv1_out, 0, &state.transitioner.render_buffers.uv1, 0, uv_total_size);
                    encoder.copy_buffer_to_buffer(&state.transitioner.create_vertices_buffers.uv2_out, 0, &state.transitioner.render_buffers.uv2, 0, uv_total_size);

                    state.camera.update();
                    state.render_uniforms.mvp = state.camera.matrix().to_cols_array_2d();

                    state.transitioner.update_render_uniforms(&state.engine.queue, &state.render_uniforms);
                    state.transitioner.record_render(&state.render, &mut encoder, &view, &state.depth_view);
                    
                    state.engine.queue.submit(std::iter::once(encoder.finish()));
                    output.present();
                }
            }
            WindowEvent::Resized(new_size) => {
                if let Some(state) = self.state.as_mut() {
                    state.resize(new_size);
                }
            }
            _ => {}
        }
    }
}