use std::sync::Arc;


use winit::{application::ApplicationHandler, window::Window};

use crate::gpu::{camera::Camera, engine::Engine, render::Render, resources::{RenderUniforms, UV, Vertex, VertexCreatorUniforms}, transitioner::Transitioner, utils::create_depth_texture_view, vertex_creator::VertexCreator};



pub struct AppState {

    pub engine: Engine,
    pub vertex_creator: VertexCreator,
    pub render: Render,
    pub transitioner: Transitioner,
    pub depth_view: wgpu::TextureView,
    pub transition_percentage: f32,
    pub vertex_creator_uniforms: VertexCreatorUniforms,
    pub render_uniforms:  RenderUniforms,
    pub camera: Camera,
    pub is_dragging: bool,
    pub is_panning: bool,


}

impl AppState {

    pub async fn new(
        window: Arc<Window>,
        mesh1: &[Vertex],
        mesh2: &[Vertex],
        uv1: &[UV],
        uv2: &[UV],
        tex1: &[u8],
        tex2: &[u8],
    ) ->  Self {

        let engine = Engine::new(window).await;

        let vertex_creator = VertexCreator::new(&engine.device);
        let render = Render::new(&engine.device, &engine.config);

        let depth_texture_view = create_depth_texture_view(&engine.device, &engine.config);

        


        Self {

        }

    }

}

pub struct App {




}


impl ApplicationHandler for App {

}