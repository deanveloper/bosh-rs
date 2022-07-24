use crate::rider::entities::PointIndex;

pub trait Bone {}

/// A standard bone is one which simply holds two points together.
#[derive(Clone, Debug, PartialEq)]
pub struct StandardBone {
    pub p1: PointIndex,
    pub p2: PointIndex,

    pub resting_length: f64,
}

/// A Mounter is a bone which holds bosh onto his sled.
#[derive(Clone, Debug, PartialEq)]
pub struct MounterBone {
    pub p1: PointIndex,
    pub p2: PointIndex,
    pub endurance: f64,

    pub resting_length: f64,
}

/// Repel is a bone which makes sure two points don't get too close to each other.
#[derive(Clone, Debug, PartialEq)]
pub struct RepelBone {
    pub p1: PointIndex,
    pub p2: PointIndex,
    pub length_factor: f64,

    pub resting_length: f64,
}

impl Bone for StandardBone {}
impl Bone for MounterBone {}
impl Bone for RepelBone {}

/// A joint is a bone which tries to hold a 90 degree angle between its two bones.
/// Or I think that's what a joint is. I'm still not quite sure, I'll find out later.
pub struct Joint<T: Bone>(pub T, pub T);
