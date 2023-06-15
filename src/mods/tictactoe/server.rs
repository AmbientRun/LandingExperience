use std::collections::HashMap;

use ambient_api::{
    components::core::transform::translation,
    entity::{add_component, remove_component, set_component},
    prelude::*,
};
use itertools::Itertools;

#[main]
pub fn main() {
    change_query(color())
        .track_change(color())
        .bind(move |entities| {
            eval_board();
        });
}
fn eval_board() {
    let cells = query((translation(), color()))
        .build()
        .evaluate()
        .into_iter()
        .map(|(id, (pos, col))| (pos.as_uvec3().xy(), (id, col)))
        .collect::<HashMap<_, _>>();
    for (_, (id, _)) in &cells {
        remove_component(*id, outline());
    }
    for (&pos, &(_, color)) in cells.iter().filter(|x| x.1 .1 != vec4(0.1, 0.1, 0.1, 1.)) {
        for d in [UVec2::X, UVec2::Y, UVec2::ONE] {
            let cells = (0..5)
                .filter_map(|i| cells.get(&(pos + d * i)))
                .filter(|x| x.1 == color);
            if cells.clone().count() == 5 {
                for (id, _) in cells {
                    add_component(*id, outline(), Vec4::ONE);
                }
            }
        }
    }
}
