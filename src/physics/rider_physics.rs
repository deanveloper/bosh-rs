use crate::rider::bone::{Bone, RepelBone, StandardBone};
use crate::rider::entities::Entity;
use crate::vector::Vector2D;

pub fn update_bones(rider: &Entity) {
    match rider {
        Entity::Bosh(bosh) => {
            let bones = bosh.borrow().bones.clone();
            for bone in bones {
                let (p1, p2) = next_standardbone_locs(bone.as_ref());

                let points = &mut bosh.borrow_mut().points;
                points.get_mut(&bone.p1).map(|p| p.location = p1);
                points.get_mut(&bone.p2).map(|p| p.location = p2);
            }
            let repel_bones = bosh.borrow().repel_bones.clone();
            for repel in repel_bones {
                let (p1, p2) = next_repelbone_locs(repel.as_ref());

                let points = &mut bosh.borrow_mut().points;
                points.get_mut(&repel.p1).map(|p| p.location = p1);
                points.get_mut(&repel.p2).map(|p| p.location = p2);
            }
        }
        Entity::Sled(sled) => {
            let bones = sled.borrow().bones.clone();
            for bone in bones {
                let (p1, p2) = next_standardbone_locs(bone.as_ref());

                let points = &mut sled.borrow_mut().points;
                points.get_mut(&bone.p1).map(|p| p.location = p1);
                points.get_mut(&bone.p2).map(|p| p.location = p2);
            }
        }
        Entity::BoshSled(bosh_sled) => {
            update_bones(&Entity::Bosh(bosh_sled.borrow().clone().bosh));
            update_bones(&Entity::Sled(bosh_sled.borrow().clone().sled));
        }
    }
}

pub fn next_repelbone_locs(bone: &RepelBone) -> (Vector2D, Vector2D) {
    let (p1, p2) = bone.points();
    let diff = p2.location - p1.location;
    let length = diff.length_squared().sqrt();

    if length >= bone.standard_length() {
        return (p1.location, p2.location);
    }

    stick_resolve(
        p1.location,
        p2.location,
        get_diff(bone.standard_length() * 0.5, length),
    )
}

pub fn next_standardbone_locs(bone: &StandardBone) -> (Vector2D, Vector2D) {
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
