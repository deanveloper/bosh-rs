use crate::rider::PointIndex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bone {
    pub p1: PointIndex,
    pub p2: PointIndex,
    pub resting_length: f64,

    pub bone_type: BoneType,
}

impl Bone {
    pub fn is_bosh_bone(&self) -> bool {
        self.p1.is_bosh() && self.p2.is_bosh()
    }
    pub fn is_sled_bone(&self) -> bool {
        !self.p1.is_bosh() && !self.p2.is_bosh()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoneType {
    Normal,
    Mount { endurance: f64 },
    Repel { length_factor: f64 },
}

/// A joint breaks if its cross product is negative. Joints don't actually affect the position
/// of entities, they only exist to break if needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Joint {
    pub pair1: (PointIndex, PointIndex),
    pub pair2: (PointIndex, PointIndex),
}
