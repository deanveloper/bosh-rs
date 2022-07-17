use crate::map::Point;

pub struct Link(pub Point, pub Point);

pub struct Bosh {
    pub right_foot: Point,
    pub left_foot: Point,
    pub left_arm: Point,
    pub right_arm: Point,
    pub shoulder: Point,
    pub butt: Point,
}

pub struct Sled {
    pub peg: Point,
    pub nose: Point,
    pub tail: Point,
    pub rope: Point,
}

pub struct BoshSled {
    pub bosh: Bosh,
    pub sled: Sled,
}
