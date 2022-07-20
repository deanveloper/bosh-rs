use crate::physics::line_physics::PhysicsPoint;
use std::cell::RefCell;
use std::rc::Rc;

/// Represents a link between two [`PhysicsPoints`].
#[derive(Clone, Debug, PartialEq)]
pub enum Bone {
    /// A standard bone is one which simply holds two points together.
    Standard {
        p1: Rc<RefCell<PhysicsPoint>>,
        p2: Rc<RefCell<PhysicsPoint>>,
    },

    /// A Mounter is a bone which holds bosh onto his sled.
    Mounter {
        p1: Rc<RefCell<PhysicsPoint>>,
        p2: Rc<RefCell<PhysicsPoint>>,
        endurance: f64,
    },

    /// Repel is a bone which makes sure two points don't get too close to each other.
    Repel {
        p1: Rc<RefCell<PhysicsPoint>>,
        p2: Rc<RefCell<PhysicsPoint>>,
        factor: f64,
    },
}

/// A joint is a bone which tries to hold a 90 degree angle between its two bones.
/// Or I think that's what a joint is. I'm still not quite sure, I'll find out later.
pub struct Joint(Bone, Bone);
