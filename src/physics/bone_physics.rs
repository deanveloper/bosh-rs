use crate::physics::rider_physics::PhysicsEntity;
use crate::rider::bone::{Bone, MounterBone, RepelBone, StandardBone};
use crate::vector::Vector2D;

pub trait PhysicsBone: Bone {
    /// Returns Some((p1, p2)) for the bone-bounded locations, or None if the bone should break.
    /// Noteworthy that only MounterBones are breakable.
    fn next_locations<E: PhysicsEntity>(&self, entity: &E) -> Option<(Vector2D, Vector2D)>;
}

impl PhysicsBone for StandardBone {
    fn next_locations<E: PhysicsEntity>(&self, entity: &E) -> Option<(Vector2D, Vector2D)> {
        let p1 = entity.point_at(self.p1);
        let p2 = entity.point_at(self.p2);

        let diff = p2.location - p1.location;
        let length = diff.length_squared().sqrt();

        Some(stick_resolve(
            p1.location,
            p2.location,
            get_diff(self.resting_length, length),
        ))
    }
}

impl PhysicsBone for RepelBone {
    fn next_locations<E: PhysicsEntity>(&self, entity: &E) -> Option<(Vector2D, Vector2D)> {
        let p1 = entity.point_at(self.p1);
        let p2 = entity.point_at(self.p2);

        let diff = p2.location - p1.location;
        let length = diff.length_squared().sqrt();

        if length >= self.resting_length * self.length_factor {
            Some((p1.location, p2.location))
        } else {
            Some(stick_resolve(
                p1.location,
                p2.location,
                get_diff(self.resting_length * self.length_factor * 0.5, length),
            ))
        }
    }
}

impl PhysicsBone for MounterBone {
    fn next_locations<E: PhysicsEntity>(&self, entity: &E) -> Option<(Vector2D, Vector2D)> {
        let p1 = entity.point_at(self.p1).location;
        let p2 = entity.point_at(self.p2).location;

        let length = (p2 - p1).length_squared().sqrt();

        let diff = get_diff(self.resting_length, length);
        if diff > self.endurance {
            None
        } else {
            Some(stick_resolve(p1, p2, diff))
        }
    }
}

fn stick_resolve(p1: Vector2D, p2: Vector2D, diff: f64) -> (Vector2D, Vector2D) {
    let delta = (p1 - p2) * diff;

    (p1 - delta, delta + p2)
}

fn get_diff(resting_length: f64, current_length: f64) -> f64 {
    if current_length == 0.0 {
        0.0
    } else {
        (current_length - resting_length) / current_length
    }
}
