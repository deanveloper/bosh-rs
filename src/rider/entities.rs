use std::collections::HashMap;
use std::hash::Hash;

use crate::physics::line_physics::PhysicsPoint;
use crate::rider::bone::{MounterBone, RepelBone, StandardBone};
use crate::vector::Vector2D;

#[derive(Clone, Debug, PartialEq)]
pub enum Entity {
    Bosh(Bosh),
    Sled(Sled),
    BoshSled(BoshSled),
}

impl Entity {
    pub fn point_at(&self, index: PointIndex) -> Option<PhysicsPoint> {
        match self {
            Entity::Bosh(bosh) => bosh.points.get(&index).copied(),
            Entity::Sled(sled) => sled.points.get(&index).copied(),
            Entity::BoshSled(bosh_sled) => {
                if index.is_bosh() {
                    bosh_sled.bosh.points.get(&index).copied()
                } else {
                    bosh_sled.sled.points.get(&index).copied()
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

    pub bones: Vec<StandardBone>,
    pub repel_bones: Vec<RepelBone>,
}

#[derive(Clone, Debug)]
pub struct Sled {
    pub points: HashMap<PointIndex, PhysicsPoint>,

    pub bones: Vec<StandardBone>,
}

#[derive(Clone, Debug)]
pub struct BoshSled {
    pub bosh: Bosh,
    pub sled: Sled,

    pub mounter_bones: Vec<MounterBone>,
}

impl Default for Bosh {
    fn default() -> Bosh {
        let left_foot = make_physics_point(Vector2D(10.0, 5.0), 0.0);
        let right_foot = make_physics_point(Vector2D(10.0, 5.0), 0.0);
        let left_hand = make_physics_point(Vector2D(11.5, -5.0), 0.1);
        let right_hand = make_physics_point(Vector2D(11.5, -5.0), 0.1);
        let shoulder = make_physics_point(Vector2D(5.0, -5.5), 0.8);
        let butt = make_physics_point(Vector2D(5.0, 0.0), 0.8);

        let mut bosh = Bosh {
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
        };

        bosh.bones = make_standard_bones(
            vec![
                (PointIndex::BoshShoulder, PointIndex::BoshButt),
                (PointIndex::BoshShoulder, PointIndex::BoshLeftHand),
                (PointIndex::BoshShoulder, PointIndex::BoshRightHand),
                (PointIndex::BoshButt, PointIndex::BoshLeftFoot),
                (PointIndex::BoshButt, PointIndex::BoshRightFoot),
                (PointIndex::BoshShoulder, PointIndex::BoshRightHand),
            ],
            &bosh.points,
        );
        bosh.repel_bones = make_repel_bones(
            vec![
                (PointIndex::BoshShoulder, PointIndex::BoshLeftFoot, 0.5),
                (PointIndex::BoshShoulder, PointIndex::BoshRightFoot, 0.5),
            ],
            &bosh.points,
        );

        bosh
    }
}

impl Default for Sled {
    fn default() -> Sled {
        let peg = make_physics_point(Vector2D(0.0, 0.0), 0.8);
        let nose = make_physics_point(Vector2D(15.0, 5.0), 0.0);
        let tail = make_physics_point(Vector2D(0.0, 5.0), 0.0);
        let rope = make_physics_point(Vector2D(17.5, 0.0), 0.0);
        let mut sled = Sled {
            points: HashMap::from([
                (PointIndex::SledPeg, peg),
                (PointIndex::SledNose, nose),
                (PointIndex::SledTail, tail),
                (PointIndex::SledRope, rope),
            ]),
            bones: vec![],
        };

        sled.bones = make_standard_bones(
            vec![
                (PointIndex::SledPeg, PointIndex::SledTail),
                (PointIndex::SledTail, PointIndex::SledNose),
                (PointIndex::SledNose, PointIndex::SledRope),
                (PointIndex::SledRope, PointIndex::SledPeg),
                (PointIndex::SledPeg, PointIndex::SledNose),
                (PointIndex::SledRope, PointIndex::SledTail),
            ],
            &sled.points,
        );

        sled
    }
}

impl Default for BoshSled {
    fn default() -> BoshSled {
        let bosh = Default::default();
        let sled = Default::default();

        BoshSled {
            bosh,
            sled,
            mounter_bones: vec![],
        };
        todo!("mounter bones")
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
fn length_between(
    p1: &PointIndex,
    p2: &PointIndex,
    point_map: &HashMap<PointIndex, PhysicsPoint>,
) -> f64 {
    (point_map.get(p2).expect("").location - point_map.get(p1).expect("").location)
        .length_squared()
        .sqrt()
}

fn make_repel_bones(
    bones: Vec<(PointIndex, PointIndex, f64)>,
    point_map: &HashMap<PointIndex, PhysicsPoint>,
) -> Vec<RepelBone> {
    bones
        .iter()
        .map(|(p1, p2, length_factor)| RepelBone {
            p1: *p1,
            p2: *p2,
            length_factor: *length_factor,
            resting_length: length_between(p1, p2, point_map),
        })
        .collect()
}

fn make_standard_bones(
    bones: Vec<(PointIndex, PointIndex)>,
    point_map: &HashMap<PointIndex, PhysicsPoint>,
) -> Vec<StandardBone> {
    bones
        .iter()
        .map(|(p1, p2)| StandardBone {
            p1: *p1,
            p2: *p2,
            resting_length: length_between(p1, p2, point_map),
        })
        .collect()
}
