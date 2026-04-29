use the_code::{app::App, utils::load_glb};

fn main() {
    let model1 = load_glb("src/assets/model1.glb");
    let model2 = load_glb("src/assets/model2.glb");

    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    // Compare index counts, not vertex counts
    let (indices, mesh1, mesh2, uv1, uv2, tex1, tex2, tex1_size, tex2_size) =
        if model1.2.len() >= model2.2.len() {
            (&model1.2, &model1.0, &model2.0, &model1.1, &model2.1,
             &model1.3, &model2.3, (model1.4, model1.5), (model2.4, model2.5))
        } else {
            (&model2.2, &model2.0, &model1.0, &model2.1, &model1.1,
             &model2.3, &model1.3, (model2.4, model2.5), (model1.4, model1.5))
        };

    let mut app = App::new(
        indices, mesh1, mesh2, uv1, uv2,
        tex1, tex2, tex1_size, tex2_size,
    );

    event_loop.run_app(&mut app).unwrap();
}