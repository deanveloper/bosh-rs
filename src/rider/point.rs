use crate::Vector2D;

#[derive(Hash, Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
#[repr(usize)]
pub enum PointIndex {
    BoshLeftFoot = 0,
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct EntityPoint {
    pub previous_location: Vector2D,
    pub location: Vector2D,
    pub friction: f64,
}
