use ambient_api::{
    components::core::{
        physics::{cube_collider, sphere_collider, visualize_collider},
        primitives::{cube, sphere},
    },
    concepts::make_sphere,
    entity::{add_components, remove_components},
    prelude::*,
};

#[main]
pub fn main() {
    // spawn_query(sphere()).bind(|entities| {
    //     for (entity, _) in entities {
    //         remove_components(entity, &[&sphere(), &sphere_collider()]);
    //         add_components(
    //             entity,
    //             Entity::new()
    //                 .with_default(cube())
    //                 .with(cube_collider(), Vec3::ONE),
    //         );
    //     }
    // });
}
