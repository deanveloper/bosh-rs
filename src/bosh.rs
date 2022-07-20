use crate::physics::line_physics::PhysicsPoint;
use crate::vector::Vector2D;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Bone(pub Vector2D, pub Vector2D);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bosh {
    pub right_foot: PhysicsPoint,
    pub left_foot: PhysicsPoint,
    pub left_hand: PhysicsPoint,
    pub right_hand: PhysicsPoint,
    pub shoulder: PhysicsPoint,
    pub butt: PhysicsPoint,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sled {
    pub peg: PhysicsPoint,
    pub nose: PhysicsPoint,
    pub tail: PhysicsPoint,
    pub rope: PhysicsPoint,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BoshSled {
    pub bosh: Bosh,
    pub sled: Sled,
}

impl Bosh {
    fn new() -> Bosh {
        Bosh {
            right_foot: PhysicsPoint {
                previous_location: Vector2D(10.0, 5.0),
                location: Vector2D(10.0, 5.0),
                friction: 0.0,
            },
            left_foot: PhysicsPoint {
                previous_location: Vector2D(10.0, 5.0),
                location: Vector2D(10.0, 5.0),
                friction: 0.0,
            },
            left_hand: PhysicsPoint {
                previous_location: Vector2D(11.5, -5.0),
                location: Vector2D(11.5, -5.0),
                friction: 0.1,
            },
            right_hand: PhysicsPoint {
                previous_location: Vector2D(11.5, -5.0),
                location: Vector2D(11.5, -5.0),
                friction: 0.1,
            },
            shoulder: PhysicsPoint {
                previous_location: Vector2D(5.0, -5.5),
                location: Vector2D(5.0, -5.5),
                friction: 0.8,
            },
            butt: PhysicsPoint {
                previous_location: Vector2D(5.0, 0.0),
                location: Vector2D(5.0, 0.0),
                friction: 0.8,
            },
        }
    }
}

impl Sled {
    fn new() -> Sled {
        Sled {
            peg: PhysicsPoint {
                previous_location: PhysicsPoint(0.0, 0.0),
                location: PhysicsPoint(0.0, 0.0),
                friction: 0.8,
            },
            nose: PhysicsPoint {
                previous_location: PhysicsPoint(15.0, 5.0),
                location: PhysicsPoint(15.0, 5.0),
                friction: 0.0,
            },
            tail: PhysicsPoint {
                previous_location: PhysicsPoint(0.0, 5.0),
                location: PhysicsPoint(0.0, 5.0),
                friction: 0.0,
            },
            rope: PhysicsPoint {
                previous_location: PhysicsPoint(17.5, 0.0),
                location: PhysicsPoint(17.5, 0.0),
                friction: 0.0,
            },
        }
    }
}

impl BoshSled {
    fn new() -> BoshSled {
        BoshSled {
            bosh: Bosh::new(),
            sled: Sled::new(),
        }
    }
}