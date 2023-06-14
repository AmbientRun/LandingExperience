use ambient_api::{
    components::core::{physics::cube_collider, primitives::cube, transform::translation},
    entity::add_component,
    prelude::*,
};

#[main]
pub fn main() {
    for y in 0..10 {
        for x in 0..10 {
            Entity::new()
                .with_default(cube())
                .with(cube_collider(), Vec3::ONE)
                .with(translation(), vec3(x as f32 * 1.1, y as f32 * 1.1, 0.))
                .with(color(), vec4(0.1, 0.1, 0.1, 1.))
                .spawn();
        }
    }
    messages::Click::subscribe(|_, ev| {
        if let Some(hit) = physics::raycast_first(ev.origin, ev.dir) {
            add_component(hit.entity, color(), ev.color);
        }
    });
}
