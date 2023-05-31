use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        physics::cube_collider,
        primitives::{cube, quad},
        transform::{lookat_target, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    entity::add_component,
    prelude::*,
    rand,
};

#[main]
pub fn main() {
    for y in 0..10 {
        for x in 0..10 {
            Entity::new()
                .with_default(cube())
                .with(cube_collider(), Vec3::ONE)
                .with(translation(), vec3(x as f32, y as f32, 0.))
                .spawn();
        }
    }
    messages::Click::subscribe(|_, ev| {
        println!("server click: {:?}", ev.dir);
        if let Some(hit) = physics::raycast_first(ev.origin, ev.dir) {
            println!("server hit: {:?}", hit.entity);
            add_component(hit.entity, color(), rand::random::<Vec3>().extend(1.0));
        }
    });
}
