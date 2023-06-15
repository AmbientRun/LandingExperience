use ambient_api::{
    components::core::{physics::cube_collider, primitives::cube, transform::translation},
    entity::add_component,
    prelude::*,
};
use itertools::Itertools;

#[main]
pub fn main() {
    for (x, y) in (0..10).cartesian_product(0..10) {
        Entity::new()
            .with_default(cube())
            .with(cube_collider(), Vec3::ONE)
            .with(translation(), uvec3(x, y, 0).as_vec3() * 1.1)
            .spawn();
    }
    messages::Click::subscribe(|_, ev| {
        if let Some(hit) = physics::raycast_first(ev.origin, ev.dir) {
            add_component(hit.entity, color(), ev.color);
        }
    });
}
