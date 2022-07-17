use crate::vector::Vector2D;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct MovingPoint {
    pub location: Vector2D,
    pub velocity: Vector2D,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Bone(pub Vector2D, pub Vector2D);

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Bosh {
    pub right_foot: MovingPoint,
    pub left_foot: MovingPoint,
    pub left_arm: MovingPoint,
    pub right_arm: MovingPoint,
    pub shoulder: MovingPoint,
    pub butt: MovingPoint,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Sled {
    pub peg: MovingPoint,
    pub nose: MovingPoint,
    pub tail: MovingPoint,
    pub rope: MovingPoint,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct BoshSled {
    pub bosh: Bosh,
    pub sled: Sled,
}
