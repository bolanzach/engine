use three_d::*;

pub struct CameraController {
    control: CameraControl,
}

impl CameraController {
    pub fn new(target: Vec3, min: f32, max: f32) -> Self {
        Self {
            control: CameraControl {
                left_drag_horizontal: CameraAction::Left { speed: 0.5 },
                left_drag_vertical: CameraAction::Up { speed: 0.5 },
                right_drag_vertical: CameraAction::OrbitUp { target, speed: 0.5 },
                right_drag_horizontal: CameraAction::OrbitLeft { target, speed: 0.5 },
                scroll_vertical: CameraAction::Zoom {
                    min,
                    max,
                    speed: 0.1,
                    target,
                },
                ..Default::default()
            }
        }
    }

    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        // let idk = self.control.scroll_vertical;
        // idk.
        if let CameraAction::Zoom {
            speed,
            target,
            min,
            max,
        } = &mut self.control.scroll_vertical
        {
            let x = target.distance(*camera.position());
            *speed = 0.5 * smoothstep(*min, *max, x) + 0.001;
        }

        self.control.handle_events(camera, events)
    }
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).max(0.0).min(1.0);
    t * t * (3.0 - 2.0 * t)
}