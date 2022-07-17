use crate::track::Point;

#[derive(Default, Copy, Clone)]
pub struct MovingPoint {
    pub location: Point,
    pub prev_location: Point,
}

#[derive(Default, Copy, Clone)]
pub struct Bone(pub Point, pub Point);

#[derive(Default, Copy, Clone)]
pub struct Bosh {
    pub right_foot: MovingPoint,
    pub left_foot: MovingPoint,
    pub left_arm: MovingPoint,
    pub right_arm: MovingPoint,
    pub shoulder: MovingPoint,
    pub butt: MovingPoint,
}

#[derive(Default, Copy, Clone)]
pub struct Sled {
    pub peg: MovingPoint,
    pub nose: MovingPoint,
    pub tail: MovingPoint,
    pub rope: MovingPoint,
}

#[derive(Default, Copy, Clone)]
pub struct BoshSled {
    pub bosh: Bosh,
    pub sled: Sled,
}
