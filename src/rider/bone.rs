use std::hash::{Hash, Hasher};

use crate::physics::line_physics::PhysicsPoint;
use crate::rider::entities::{Entity, PointIndex};

/// Represents a link between two [`PhysicsPoint`]s.
pub trait Bone {
    fn standard_length(&self) -> f64;
    fn points(&self) -> (PhysicsPoint, PhysicsPoint);
}

/// A standard bone is one which simply holds two points together.
#[derive(Clone, Debug, PartialEq)]
pub struct StandardBone {
    pub entity: Entity,
    pub p1: PointIndex,
    pub p2: PointIndex,

    initial_len: f64,
}

/// A Mounter is a bone which holds bosh onto his sled.
#[derive(Clone, Debug, PartialEq)]
pub struct MounterBone {
    pub entity: Entity,
    pub p1: PointIndex,
    pub p2: PointIndex,
    pub endurance: f64,

    initial_len: f64,
}

/// Repel is a bone which makes sure two points don't get too close to each other.
#[derive(Clone, Debug, PartialEq)]
pub struct RepelBone {
    pub entity: Entity,
    pub p1: PointIndex,
    pub p2: PointIndex,
    pub length_factor: f64,

    initial_len: f64,
}

/// A joint is a bone which tries to hold a 90 degree angle between its two bones.
/// Or I think that's what a joint is. I'm still not quite sure, I'll find out later.
pub struct Joint<T: Bone>(pub T, pub T);

impl StandardBone {
    pub fn new(entity: Entity, i1: PointIndex, i2: PointIndex) -> StandardBone {
        let p1 = entity.point_at(i1).expect("StandardBone::new - i1 is None");
        let p2 = entity.point_at(i2).expect("StandardBone::new - i2 is None");

        StandardBone {
            entity: entity.clone(),
            p1: i1,
            p2: i2,
            initial_len: (p1.location - p2.location).length_squared().sqrt(),
        }
    }
}

impl MounterBone {
    pub fn new(entity: Entity, i1: PointIndex, i2: PointIndex, endurance: f64) -> MounterBone {
        let p1 = entity.point_at(i1).expect("StandardBone::new - i1 is None");
        let p2 = entity.point_at(i2).expect("StandardBone::new - i2 is None");

        MounterBone {
            entity: entity.clone(),
            p1: i1,
            p2: i2,
            endurance,
            initial_len: (p1.location - p2.location).length_squared().sqrt(),
        }
    }
}

impl RepelBone {
    pub fn new(entity: Entity, i1: PointIndex, i2: PointIndex, length_factor: f64) -> RepelBone {
        let p1 = entity.point_at(i1).expect("StandardBone::new - i1 is None");
        let p2 = entity.point_at(i2).expect("StandardBone::new - i2 is None");

        RepelBone {
            entity: entity.clone(),
            p1: i1,
            p2: i2,
            length_factor,
            initial_len: (p1.location - p2.location).length_squared().sqrt(),
        }
    }
}

impl Bone for StandardBone {
    fn standard_length(&self) -> f64 {
        self.initial_len
    }
    fn points(&self) -> (PhysicsPoint, PhysicsPoint) {
        (
            self.entity.point_at(self.p1).expect("failed unwrap p1"),
            self.entity.point_at(self.p2).expect("failed unwrap p2"),
        )
    }
}

impl Bone for MounterBone {
    fn standard_length(&self) -> f64 {
        self.initial_len
    }
    fn points(&self) -> (PhysicsPoint, PhysicsPoint) {
        (
            self.entity.point_at(self.p1).expect("failed unwrap p1"),
            self.entity.point_at(self.p2).expect("failed unwrap p2"),
        )
    }
}

impl Bone for RepelBone {
    fn standard_length(&self) -> f64 {
        self.initial_len
    }
    fn points(&self) -> (PhysicsPoint, PhysicsPoint) {
        (
            self.entity.point_at(self.p1).expect("failed unwrap p1"),
            self.entity.point_at(self.p2).expect("failed unwrap p2"),
        )
    }
}

/// an unordered tuple of points
#[derive(Copy, Clone)]
pub struct BoneMapKey(PointIndex, PointIndex);

impl Hash for BoneMapKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Ord::min(self.0, self.1).hash(state);
        Ord::max(self.0, self.1).hash(state);
    }
}

impl PartialEq for BoneMapKey {
    fn eq(&self, other: &Self) -> bool {
        let s_min = Ord::min(self.0, self.1);
        let s_max = Ord::max(self.0, self.1);
        let o_min = Ord::min(other.0, other.1);
        let o_max = Ord::max(other.0, other.1);

        s_min == o_min && s_max == o_max
    }
}

impl Eq for BoneMapKey {}
