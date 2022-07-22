use crate::rider::bone::{Bone, StandardBone};
use crate::rider::entities::Entity;
use crate::vector::Vector2D;

pub fn update_bones(_rider: Entity) -> Entity {
    todo!()
}

pub fn next_standardbone_locs<T>(bone: &StandardBone) -> (Vector2D, Vector2D) {
    let (p1, p2) = bone.points();
    let diff = p2.location - p1.location;
    let length = diff.length_squared().sqrt();

    stick_resolve(
        p1.location,
        p2.location,
        get_diff(bone.standard_length(), length),
    )
}

fn stick_resolve(p1: Vector2D, p2: Vector2D, diff: f64) -> (Vector2D, Vector2D) {
    let delta = (p1 - p2) * diff;

    (p1 - delta, delta + p2)
}

fn get_diff(standard_length: f64, current_length: f64) -> f64 {
    if current_length == 0.0 {
        0.0
    } else {
        (current_length - standard_length) / current_length
    }
}
