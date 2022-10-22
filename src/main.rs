use std::env;

use three_d::*;

#[tokio::main]
async fn main() {
    let window = Window::new(WindowSettings {
        title: "ZACH".to_string(),
        ..Default::default()
    })
    .unwrap();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(-300.0, 250.0, 200.0),
        vec3(0.0, 100.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    );

    //let control = OrbitControl::new();
    let path = env::current_dir();
    println!("The current directory is {}", path.unwrap().display());

    let context = window.gl();
    let mut gui = three_d::GUI::new(&context);

    let mut loaded = three_d_asset::io::load_async(&[
        "assets/PenguinBaseMesh.obj",
    ])
    .await
    .unwrap();

    // whats a cpu_model??
    let cpu_model: CpuModel = loaded.deserialize("PenguinBaseMesh.obj").unwrap();

    let scale = Mat4::from_scale(10.0);
    let angle: f32 = 0.0;
    let rotation = Mat4::from_angle_y(radians(0.8 * std::f32::consts::PI - angle));
    let dist = 300.0;
    let translation = Mat4::from_translation(vec3(
        angle.cos() * dist,
        (1.2 * std::f32::consts::PI - angle).cos() * 21.0 - 33.0,
        angle.sin() * dist,
    ));

    let mut models = Vec::new();

    let mut penguin = Model::<PhysicalMaterial>::new(&context, &cpu_model).unwrap();
    penguin.iter_mut().for_each(|m| {
        m.set_transformation(translation * scale * rotation);
        m.material.render_states.cull = Cull::Back;
    });
    models.push(penguin);
    
    let ambient_light = AmbientLight::new(&context, 0.4, Color::WHITE);

    window.render_loop(move |mut frame_input: FrameInput| {
        let mut panel_width = 0.0;

        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {},
        );

        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
            .write(|| {
                for object in models
                    .iter()
                    .flatten()
                    .filter(|o| camera.in_frustum(&o.aabb()))
                {
                    object.render(&camera, &[&ambient_light]);
                }
                gui.render();
            });

        // Returns default frame output to end the frame
        FrameOutput::default()
    });

}
