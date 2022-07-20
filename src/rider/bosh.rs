use crate::physics::line_physics::PhysicsPoint;
use crate::rider::bone::Bone;
use crate::vector::Vector2D;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Bosh {
    // points
    pub left_foot: Rc<RefCell<PhysicsPoint>>,
    pub right_foot: Rc<RefCell<PhysicsPoint>>,
    pub left_hand: Rc<RefCell<PhysicsPoint>>,
    pub right_hand: Rc<RefCell<PhysicsPoint>>,
    pub shoulder: Rc<RefCell<PhysicsPoint>>,
    pub butt: Rc<RefCell<PhysicsPoint>>,

    // bones
    pub bone_shoulder_to_butt: Bone,
    pub bone_shoulder_to_left_hand: Bone,
    pub bone_shoulder_to_right_hand: Bone,
    pub bone_shoulder_to_right_hand2: Bone,
    pub bone_butt_to_left_foot: Bone,
    pub bone_butt_to_right_foot: Bone,
    pub bone_repel_shoulder_and_left_foot: Bone,
    pub bone_repel_shoulder_and_right_foot: Bone,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sled {
    // points
    pub peg: Rc<RefCell<PhysicsPoint>>,
    pub nose: Rc<RefCell<PhysicsPoint>>,
    pub tail: Rc<RefCell<PhysicsPoint>>,
    pub rope: Rc<RefCell<PhysicsPoint>>,

    // bones
    pub bone_peg_to_tail: Bone,
    pub bone_tail_to_nose: Bone,
    pub bone_nose_to_rope: Bone,
    pub bone_rope_to_peg: Bone,
    pub bone_rope_to_tail: Bone,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoshSled {
    pub bosh: Bosh,
    pub sled: Sled,

    // bones
    pub bone_peg_to_butt: Bone,
    pub bone_tail_to_butt: Bone,
    pub bone_nose_to_butt: Bone,
    pub bone_shoulder_to_peg: Bone,
    pub bone_rope_to_left_hand: Bone,
    pub bone_rope_to_right_hand: Bone,
    pub bone_left_foot_to_nose: Bone,
    pub bone_right_foot_to_nose: Bone,
}

impl Bosh {
    pub fn new() -> Bosh {
        let left_foot = make_physics_point(Vector2D(10.0, 5.0), 0.0);
        let right_foot = make_physics_point(Vector2D(10.0, 5.0), 0.0);
        let left_hand = make_physics_point(Vector2D(11.5, -5.0), 0.1);
        let right_hand = make_physics_point(Vector2D(11.5, -5.0), 0.1);
        let shoulder = make_physics_point(Vector2D(5.0, -5.5), 0.8);
        let butt = make_physics_point(Vector2D(5.0, 0.0), 0.8);
        Bosh {
            left_foot: left_foot.clone(),
            right_foot: right_foot.clone(),
            left_hand: left_hand.clone(),
            right_hand: right_hand.clone(),
            shoulder: shoulder.clone(),
            butt: butt.clone(),
            bone_shoulder_to_butt: Bone::Standard {
                p1: shoulder.clone(),
                p2: butt.clone(),
            },
            bone_shoulder_to_left_hand: Bone::Standard {
                p1: shoulder.clone(),
                p2: left_hand.clone(),
            },
            bone_shoulder_to_right_hand: Bone::Standard {
                p1: shoulder.clone(),
                p2: right_hand.clone(),
            },
            bone_shoulder_to_right_hand2: Bone::Standard {
                p1: shoulder.clone(),
                p2: right_hand.clone(),
            },
            bone_butt_to_left_foot: Bone::Standard {
                p1: butt.clone(),
                p2: left_foot.clone(),
            },
            bone_butt_to_right_foot: Bone::Standard {
                p1: butt.clone(),
                p2: right_foot.clone(),
            },
            bone_repel_shoulder_and_left_foot: Bone::Repel {
                p1: shoulder.clone(),
                p2: left_foot.clone(),
                factor: 0.5,
            },
            bone_repel_shoulder_and_right_foot: Bone::Repel {
                p1: shoulder.clone(),
                p2: right_foot.clone(),
                factor: 0.5,
            },
        }
    }
}

impl Sled {
    pub fn new() -> Sled {
        let peg = make_physics_point(Vector2D(0.0, 0.0), 0.8);
        let nose = make_physics_point(Vector2D(15.0, 5.0), 0.0);
        let tail = make_physics_point(Vector2D(0.0, 5.0), 0.0);
        let rope = make_physics_point(Vector2D(17.5, 0.0), 0.0);
        Sled {
            peg: peg.clone(),
            nose: nose.clone(),
            tail: tail.clone(),
            rope: rope.clone(),
            bone_peg_to_tail: Bone::Standard {
                p1: peg.clone(),
                p2: tail.clone(),
            },
            bone_tail_to_nose: Bone::Standard {
                p1: tail.clone(),
                p2: nose.clone(),
            },
            bone_nose_to_rope: Bone::Standard {
                p1: nose.clone(),
                p2: rope.clone(),
            },
            bone_rope_to_peg: Bone::Standard {
                p1: rope.clone(),
                p2: peg.clone(),
            },
            bone_rope_to_tail: Bone::Standard {
                p1: rope.clone(),
                p2: tail.clone(),
            },
        }
    }
}

impl BoshSled {
    pub fn new() -> BoshSled {
        let bosh = Bosh::new();
        let sled = Sled::new();

        BoshSled {
            bosh: bosh.clone(),
            sled: sled.clone(),
            bone_peg_to_butt: Bone::Mounter {
                p1: sled.peg.clone(),
                p2: bosh.butt.clone(),
                endurance: 0.057,
            },
            bone_tail_to_butt: Bone::Mounter {
                p1: sled.tail.clone(),
                p2: bosh.butt.clone(),
                endurance: 0.057,
            },
            bone_nose_to_butt: Bone::Mounter {
                p1: sled.nose.clone(),
                p2: bosh.butt.clone(),
                endurance: 0.057,
            },
            bone_shoulder_to_peg: Bone::Mounter {
                p1: bosh.shoulder.clone(),
                p2: sled.peg.clone(),
                endurance: 0.057,
            },
            bone_rope_to_left_hand: Bone::Mounter {
                p1: sled.rope.clone(),
                p2: bosh.left_hand.clone(),
                endurance: 0.057,
            },
            bone_rope_to_right_hand: Bone::Mounter {
                p1: sled.rope.clone(),
                p2: bosh.right_hand.clone(),
                endurance: 0.057,
            },
            bone_left_foot_to_nose: Bone::Mounter {
                p1: bosh.left_foot.clone(),
                p2: sled.nose.clone(),
                endurance: 0.057,
            },
            bone_right_foot_to_nose: Bone::Mounter {
                p1: bosh.right_foot.clone(),
                p2: sled.nose.clone(),
                endurance: 0.057,
            },
        }
    }
}

fn make_physics_point(loc: Vector2D, friction: f64) -> Rc<RefCell<PhysicsPoint>> {
    Rc::new(RefCell::new(PhysicsPoint {
        previous_location: loc,
        location: loc,
        friction,
    }))
}
