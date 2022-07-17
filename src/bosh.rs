use crate::vector::Vector2D;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct BoshPoint {
    pub location: Vector2D,
    pub velocity: Vector2D,
    pub friction: f64,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Bone(pub Vector2D, pub Vector2D);

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Bosh {
    pub right_foot: BoshPoint,
    pub left_foot: BoshPoint,
    pub left_arm: BoshPoint,
    pub right_arm: BoshPoint,
    pub shoulder: BoshPoint,
    pub butt: BoshPoint,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Sled {
    pub peg: BoshPoint,
    pub nose: BoshPoint,
    pub tail: BoshPoint,
    pub rope: BoshPoint,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct BoshSled {
    pub bosh: Bosh,
    pub sled: Sled,
}
