use crate::Vector2D;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
pub enum PointIndex {
    BoshLeftFoot,
    BoshRightFoot,
    BoshLeftHand,
    BoshRightHand,
    BoshShoulder,
    BoshButt,

    SledPeg,
    SledTail,
    SledNose,
    SledRope,
}

impl PointIndex {
    pub fn is_bosh(&self) -> bool {
        &PointIndex::BoshLeftFoot <= self && self <= &PointIndex::BoshButt
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct EntityPoint {
    pub previous_location: Vector2D,
    pub location: Vector2D,
    pub friction: f64,
}
