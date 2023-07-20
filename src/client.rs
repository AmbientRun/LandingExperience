use ambient_api::{
    components::core::{
        app::{main_scene, name},
        camera::aspect_ratio_from_window,
        transform::{lookat_target, translation},
    },
    concepts::make_perspective_infinite_reverse_camera,
    prelude::*,
    rand,
};

#[main]
pub fn main() {
    let camera = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), vec3(0., 12., 10.))
        .with(lookat_target(), vec3(5., 5., -3.))
        .with(name(), "Main camera".to_string())
        .spawn();

    let color = rand::random::<Vec3>().extend(1.0);
    ambient_api::messages::WindowMouseInput::subscribe(move |ev| {
        if ev.pressed {
            let ray = camera::screen_position_to_world_ray(camera, input::get().mouse_position);
            messages::Click {
                origin: ray.origin,
                dir: ray.dir,
                color,
            }
            .send_server_unreliable();
        }
    });
}
