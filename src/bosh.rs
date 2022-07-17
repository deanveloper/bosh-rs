use crate::track::Point;

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

impl Default for Bosh {
    fn default() -> Self {
        Bosh {
            right_foot: Point::default(),
            left_foot: Point::default(),
            left_arm: Point::default(),
            right_arm: Point::default(),
            shoulder: Point::default(),
            butt: Point::default(),
        }
    }
}

impl Default for Sled {
    fn default() -> Self {
        Sled {
            peg: Point::default(),
            nose: Point::default(),
            tail: Point::default(),
            rope: Point::default(),
        }
    }
}

impl Default for BoshSled {
    fn default() -> BoshSled {
        return BoshSled {
            bosh: Bosh::default(),
            sled: Sled::default(),
        };
    }
}
