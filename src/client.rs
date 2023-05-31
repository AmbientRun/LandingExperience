use ambient_api::{
    components::core::camera::aspect_ratio_from_window,
    concepts::make_perspective_infinite_reverse_camera, prelude::*,
};

#[main]
pub fn main() {
    println!("----------- CLIENT STARTED");
    let camera = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 15.)
        .with(lookat_target(), vec3(5., 5., 0.))
        .with(name(), "Main camera".to_string())
        .spawn();

    ambient_api::messages::WindowMouseInput::subscribe(move |ev| {
        if !ev.pressed {
            return;
        }
        let input = input::get();
        let ray = camera::screen_position_to_world_ray(camera, input.mouse_position);
        println!("click: {:?}", ray);

        messages::Click {
            origin: ray.origin,
            dir: ray.dir,
        }
        .send_server_unreliable();
    });
    println!("----------- CLIENT STARTED done");
}
