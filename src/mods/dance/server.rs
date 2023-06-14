use ambient_api::{components::core::transform::translation, entity::set_component, prelude::*};

#[main]
pub fn main() {
    let start_time = time();
    query(translation()).each_frame(move |entities| {
        let t = (time() - start_time).as_secs_f32();
        for (entity, pos) in entities {
            let t = pos.x * pos.y * 0.1 + t;
            set_component(entity, translation(), vec3(pos.x, pos.y, t.sin()));
        }
    });
}
