use ambient_api::{
    components::core::{
        physics::{cube_collider, sphere_collider, visualize_collider},
        primitives::cube,
    },
    concepts::make_sphere,
    entity::{add_components, remove_components},
    prelude::*,
};

#[main]
pub fn main() {
    spawn_query(cube()).bind(|entities| {
        for (entity, _) in entities {
            remove_components(entity, &[&cube(), &cube_collider()]);
            add_components(
                entity,
                make_sphere().with(sphere_collider(), 0.5), // .with_default(visualize_collider()),
            );
        }
    });
}
