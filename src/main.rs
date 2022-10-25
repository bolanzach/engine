mod camera_controller;

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
        vec3(-600.0, 600.0, 600.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    );
    let mut camera_controller = camera_controller::CameraController::new(
        *camera.target(),
        0.01 * camera.target().distance(*camera.position()),
        5.0 * camera.target().distance(*camera.position()),
    );


    let graphics_context = window.gl();
    let mut gui = three_d::GUI::new(&graphics_context);

    let mut loaded = three_d_asset::io::load_async(&[
        "assets/PenguinBaseMesh.obj",
        "assets/StonePlatform_A.obj",
        "assets/StonePlatforms_Base-Diffuse.png",
    ])
    .await
    .unwrap();

    let penguin_raw: CpuModel = loaded.deserialize("PenguinBaseMesh.obj").unwrap();

    let scale = Mat4::from_scale(10.0);
    let dist = 300.0;

    let mut models = Vec::new();

    for i in 0..8 {
        let mut penguin = 
            Model::<PhysicalMaterial>::new(&graphics_context, &penguin_raw).unwrap();
        let angle: f32 = i as f32 * 2.0 * std::f32::consts::PI / 8.0;
        let rotation = Mat4::from_angle_y(radians(0.8 * std::f32::consts::PI - angle));
        let translation = Mat4::from_translation(vec3(
            angle.cos() * dist,
            (1.2 * std::f32::consts::PI - angle).cos() * 21.0 - 33.0,
            angle.sin() * dist,
        ));

        penguin.iter_mut().for_each(|m| {
            m.set_transformation(translation * scale * rotation);
            m.material.render_states.cull = Cull::Back;
        });
        models.push(penguin);
    }

    let stone_floor_raw: CpuModel = loaded.deserialize("StonePlatform_A.obj").unwrap();
    let mut stone_floor = Model::<PhysicalMaterial>::new(
        &graphics_context, 
        &stone_floor_raw
    )
    .unwrap();
    let angle: f32 = 2.0 * std::f32::consts::PI / 8.0;
    let rotation = Mat4::from_angle_y(radians(0.8 * std::f32::consts::PI - angle));
    let translation = Mat4::from_translation(vec3(
        angle.cos() * dist,
        (1.2 * std::f32::consts::PI - angle).cos() * 21.0 - 33.0,
        angle.sin() * dist,
    ));

    stone_floor.iter_mut().for_each(|m| {
        m.set_transformation(translation * scale * rotation);
        m.material.render_states.cull = Cull::Back;
    });
    models.push(stone_floor);
    
    let ambient_light = AmbientLight::new(&graphics_context, 0.4, Color::WHITE);
    let mut directional_light = DirectionalLight::new(
        &graphics_context, 
        10.0, 
        Color::new_opaque(255, 0, 0), 
        &vec3(0.0, -1.0, -1.0)
    );

    directional_light.generate_shadow_map(1024, models.iter().flat_map(|m| m.into_iter()));

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

        camera_controller.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
            .write(|| {
                for object in models
                    .iter()
                    .flatten()
                    .filter(|o| camera.in_frustum(&o.aabb()))
                {
                    object.render(&camera, &[&ambient_light, &directional_light]);
                }
                gui.render();
            });

        // Returns default frame output to end the frame
        FrameOutput::default()
    });

}
