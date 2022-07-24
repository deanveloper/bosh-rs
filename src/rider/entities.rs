use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::physics::line_physics::PhysicsPoint;
use crate::rider::bone::{MounterBone, RepelBone, StandardBone};
use crate::vector::Vector2D;

#[derive(Clone, Debug, PartialEq)]
pub enum Entity {
    Bosh(Rc<RefCell<Bosh>>),
    Sled(Rc<RefCell<Sled>>),
    BoshSled(Rc<RefCell<BoshSled>>),
}

impl Entity {
    pub fn point_at(&self, index: PointIndex) -> Option<PhysicsPoint> {
        match self {
            Entity::Bosh(bosh) => {
                let bosh = bosh.borrow();
                bosh.points.get(&index).map(|p| *p)
            }
            Entity::Sled(sled) => {
                let sled = sled.borrow();
                sled.points.get(&index).map(|p| *p)
            }
            Entity::BoshSled(bosh_sled) => {
                let bosh_sled = bosh_sled.borrow();
                if index.is_bosh() {
                    bosh_sled.bosh.borrow().points.get(&index).map(|p| *p)
                } else {
                    bosh_sled.sled.borrow().points.get(&index).map(|p| *p)
                }
            }
        }
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
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
    fn is_bosh(&self) -> bool {
        &PointIndex::BoshLeftFoot <= self && self <= &PointIndex::BoshButt
    }
}

#[derive(Clone, Debug)]
pub struct Bosh {
    pub points: HashMap<PointIndex, PhysicsPoint>,

    pub bones: Vec<Box<StandardBone>>,
    pub repel_bones: Vec<Box<RepelBone>>,
}

#[derive(Clone, Debug)]
pub struct Sled {
    pub points: HashMap<PointIndex, PhysicsPoint>,

    pub bones: Vec<Box<StandardBone>>,
}

#[derive(Clone, Debug)]
pub struct BoshSled {
    pub bosh: Rc<RefCell<Bosh>>,
    pub sled: Rc<RefCell<Sled>>,

    pub mounter_bones: Vec<Box<MounterBone>>,
}

impl Bosh {
    pub fn new() -> Rc<RefCell<Bosh>> {
        let left_foot = make_physics_point(Vector2D(10.0, 5.0), 0.0);
        let right_foot = make_physics_point(Vector2D(10.0, 5.0), 0.0);
        let left_hand = make_physics_point(Vector2D(11.5, -5.0), 0.1);
        let right_hand = make_physics_point(Vector2D(11.5, -5.0), 0.1);
        let shoulder = make_physics_point(Vector2D(5.0, -5.5), 0.8);
        let butt = make_physics_point(Vector2D(5.0, 0.0), 0.8);

        let bosh = Rc::new(RefCell::new(Bosh {
            points: HashMap::from([
                (PointIndex::BoshLeftFoot, left_foot),
                (PointIndex::BoshRightFoot, right_foot),
                (PointIndex::BoshLeftHand, left_hand),
                (PointIndex::BoshRightHand, right_hand),
                (PointIndex::BoshShoulder, shoulder),
                (PointIndex::BoshButt, butt),
            ]),
            bones: vec![],
            repel_bones: vec![],
        }));

        bosh.borrow_mut().bones = make_standard_bones(
            Entity::Bosh(bosh.clone()),
            vec![
                (PointIndex::BoshShoulder, PointIndex::BoshButt),
                (PointIndex::BoshShoulder, PointIndex::BoshLeftHand),
                (PointIndex::BoshShoulder, PointIndex::BoshRightHand),
                (PointIndex::BoshButt, PointIndex::BoshLeftFoot),
                (PointIndex::BoshButt, PointIndex::BoshRightFoot),
                (PointIndex::BoshShoulder, PointIndex::BoshRightHand),
            ],
        );
        bosh.borrow_mut().repel_bones = make_repel_bones(
            Entity::Bosh(bosh.clone()),
            vec![
                (PointIndex::BoshShoulder, PointIndex::BoshLeftFoot, 0.5),
                (PointIndex::BoshShoulder, PointIndex::BoshRightFoot, 0.5),
            ],
        );

        bosh
    }
}

impl Sled {
    pub fn new() -> Rc<RefCell<Sled>> {
        let peg = make_physics_point(Vector2D(0.0, 0.0), 0.8);
        let nose = make_physics_point(Vector2D(15.0, 5.0), 0.0);
        let tail = make_physics_point(Vector2D(0.0, 5.0), 0.0);
        let rope = make_physics_point(Vector2D(17.5, 0.0), 0.0);
        let sled = Rc::new(RefCell::new(Sled {
            points: HashMap::from([
                (PointIndex::SledPeg, peg),
                (PointIndex::SledNose, nose),
                (PointIndex::SledTail, tail),
                (PointIndex::SledRope, rope),
            ]),
            bones: vec![],
        }));

        sled.borrow_mut().bones = make_standard_bones(
            Entity::Sled(sled.clone()),
            vec![
                (PointIndex::SledPeg, PointIndex::SledTail),
                (PointIndex::SledTail, PointIndex::SledNose),
                (PointIndex::SledNose, PointIndex::SledRope),
                (PointIndex::SledRope, PointIndex::SledPeg),
                (PointIndex::SledPeg, PointIndex::SledNose),
                (PointIndex::SledRope, PointIndex::SledTail),
            ],
        );

        sled
    }
}

impl BoshSled {
    pub fn new() -> Rc<RefCell<BoshSled>> {
        let bosh = Bosh::new();
        let sled = Sled::new();

        BoshSled {
            bosh: bosh.clone(),
            sled: sled.clone(),
            mounter_bones: vec![],
        };
        todo!()
    }
}

impl PartialEq for Bosh {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points
    }
}
impl Eq for Bosh {}

impl PartialEq for Sled {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points
    }
}
impl Eq for Sled {}

impl PartialEq for BoshSled {
    fn eq(&self, other: &Self) -> bool {
        self.bosh == other.bosh && self.sled == other.sled
    }
}
impl Eq for BoshSled {}

// ==== PRIVATE UTIL FUNCTIONS ====

fn make_physics_point(loc: Vector2D, friction: f64) -> PhysicsPoint {
    PhysicsPoint {
        previous_location: loc,
        location: loc,
        friction,
    }
}
fn length_between(entity: &Entity, p1: PointIndex, p2: PointIndex) -> f64 {
    (entity.point_at(p2).expect("").location - entity.point_at(p1).expect("").location)
        .length_squared()
        .sqrt()
}

fn make_repel_bones(
    entity: Entity,
    bones: Vec<(PointIndex, PointIndex, f64)>,
) -> Vec<Box<RepelBone>> {
    bones
        .iter()
        .map(|(p1, p2, length_factor)| {
            Box::new(RepelBone {
                p1: *p1,
                p2: *p2,
                length_factor: *length_factor,
                resting_length: length_between(&entity, *p1, *p2),
            })
        })
        .collect()
}

fn make_standard_bones(
    entity: Entity,
    bones: Vec<(PointIndex, PointIndex)>,
) -> Vec<Box<StandardBone>> {
    bones
        .iter()
        .map(|(p1, p2)| {
            Box::new(StandardBone {
                p1: *p1,
                p2: *p2,
                resting_length: length_between(&entity, *p1, *p2),
            })
        })
        .collect()
}
