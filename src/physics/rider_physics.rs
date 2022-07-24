use crate::physics::line_physics::PhysicsPoint;
use crate::rider::bone::{Bone, RepelBone, StandardBone};
use crate::rider::entities::{Entity, PointIndex};
use crate::vector::Vector2D;
use std::collections::HashMap;

pub fn apply_gravity(_rider: &Entity) {}

pub fn update_bones(rider: &mut Entity) {
    match rider {
        Entity::Bosh(bosh) => {
            let standard_bones = bosh.bones.clone();
            next_bone_locs(&standard_bones, &mut bosh.points, next_standardbone_locs);

            let repel_bones = bosh.repel_bones.clone();
            next_bone_locs(&repel_bones, &mut bosh.points, next_repelbone_locs);
        }
        Entity::Sled(sled) => {
            let standard_bones = sled.bones.clone();
            next_bone_locs(&standard_bones, &mut sled.points, next_standardbone_locs);
        }
        Entity::BoshSled(bosh_sled) => {
            // just recursively call on the bosh and the sled
            update_bones(&mut Entity::Bosh(bosh_sled.clone().bosh));
            update_bones(&mut Entity::Sled(bosh_sled.clone().sled));
        }
    }
}

/// Generic wrapper to easily use next_repelbone_locs/next_standardbone_locs
fn next_bone_locs<T, F>(
    bones: &Vec<T>,
    points: &mut HashMap<PointIndex, PhysicsPoint>,
    next_locs: F,
) where
    T: Bone,
    F: Fn(&T, &HashMap<PointIndex, PhysicsPoint>) -> (Vector2D, Vector2D),
{
    for bone in bones {
        let (i1, i2) = bone.points();
        let (p1, p2) = next_locs(bone, points);

        if let Some(p) = points.get_mut(&i1) {
            p.location = p1
        }
        if let Some(p) = points.get_mut(&i2) {
            p.location = p2
        }
    }
}

pub fn next_repelbone_locs(
    bone: &RepelBone,
    point_map: &HashMap<PointIndex, PhysicsPoint>,
) -> (Vector2D, Vector2D) {
    let p1 = point_map.get(&bone.p1).expect("no p1 found");
    let p2 = point_map.get(&bone.p2).expect("no p2 found");

    let diff = p2.location - p1.location;
    let length = diff.length_squared().sqrt();

    if length >= bone.resting_length * bone.length_factor {
        return (p1.location, p2.location);
    }

    stick_resolve(
        p1.location,
        p2.location,
        get_diff(bone.resting_length * bone.length_factor * 0.5, length),
    )
}

pub fn next_standardbone_locs(
    bone: &StandardBone,
    point_map: &HashMap<PointIndex, PhysicsPoint>,
) -> (Vector2D, Vector2D) {
    let p1 = point_map.get(&bone.p1).expect("no p1 found");
    let p2 = point_map.get(&bone.p2).expect("no p2 found");

    let diff = p2.location - p1.location;
    let length = diff.length_squared().sqrt();

    stick_resolve(
        p1.location,
        p2.location,
        get_diff(bone.resting_length, length),
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
