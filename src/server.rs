use ambient_api::{
    components::core::{
        physics::{cube_collider, sphere_collider},
        primitives::cube,
        transform::translation,
    },
    concepts::make_sphere,
    entity::add_component,
    prelude::*,
};
use itertools::Itertools;

#[main]
pub fn main() {
    for (x, y) in (0..10).cartesian_product(0..10) {
        Entity::new()
            .with_merge(make_sphere())
            .with(sphere_collider(), 0.5)
            .with(translation(), uvec3(x, y, 0).as_vec3())
            .with(color(), vec4(0.1, 0.1, 0.1, 1.))
            .spawn();
    }
    messages::Click::subscribe(|_, ev| {
        if let Some(hit) = physics::raycast_first(ev.origin, ev.dir) {
            add_component(hit.entity, color(), ev.color);
        }
    });
}
