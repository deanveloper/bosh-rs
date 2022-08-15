use crate::game::Vector2D;
use crate::rider::{Bone, BoneType, Entity, Joint};

/// Returns Some((p1, p2)) for the bone-bounded locations, or None if the bone should break.
/// Noteworthy that only bones of type Mount are breakable.
pub fn next_bone_locations(bone: &Bone, entity: &Entity) -> Option<(Vector2D, Vector2D)> {
    let p1 = entity.point_at(bone.p1);
    let p2 = entity.point_at(bone.p2);

    let length = p2.location.distance_squared(p1.location).sqrt();

    match bone.bone_type {
        BoneType::Normal => Some(bone_resolve(
            p1.location,
            p2.location,
            get_diff(bone.resting_length, length),
        )),
        BoneType::Repel { length_factor } => {
            if length >= bone.resting_length * length_factor {
                Some((p1.location, p2.location))
            } else {
                Some(bone_resolve(
                    p1.location,
                    p2.location,
                    get_diff(bone.resting_length * length_factor, length),
                ))
            }
        }
        BoneType::Mount { endurance } => {
            let diff = get_diff(bone.resting_length, length);
            if diff > endurance * bone.resting_length * 0.5 {
                None
            } else {
                Some(bone_resolve(p1.location, p2.location, diff))
            }
        }
    }
}

pub fn joint_should_break(joint: &Joint, entity: &Entity) -> bool {
    let p1 = entity.point_at(joint.pair1.0);
    let p2 = entity.point_at(joint.pair1.1);
    let q1 = entity.point_at(joint.pair2.0);
    let q2 = entity.point_at(joint.pair2.1);

    (p2.location - p1.location).cross_product_length(q2.location - q1.location) < 0.0
}

fn bone_resolve(p1: Vector2D, p2: Vector2D, diff: f64) -> (Vector2D, Vector2D) {
    let delta = (p1 - p2) * diff;

    (p1 - delta, delta + p2)
}

fn get_diff(resting_length: f64, current_length: f64) -> f64 {
    if current_length == 0.0 {
        0.0
    } else {
        ((current_length - resting_length) / current_length) * 0.5
    }
}
