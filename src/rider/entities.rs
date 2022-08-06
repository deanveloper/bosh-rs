use std::collections::HashMap;
use std::hash::Hash;

use crate::physics::line_physics::PhysicsPoint;
use crate::rider::bone::{Joint, MounterBone, RepelBone, StandardBone};
use crate::vector::Vector2D;

#[derive(Clone, Debug, PartialEq)]
pub enum Entity {
    Bosh(Bosh),
    Sled(Sled),
    BoshSled(BoshSled),
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
    pub fn is_bosh(&self) -> bool {
        &PointIndex::BoshLeftFoot <= self && self <= &PointIndex::BoshButt
    }
}

#[derive(Clone, Debug)]
pub struct Bosh {
    pub points: HashMap<PointIndex, PhysicsPoint>,

    pub standard_bones: Vec<StandardBone>,
    pub repel_bones: Vec<RepelBone>,
}

#[derive(Clone, Debug)]
pub struct Sled {
    pub points: HashMap<PointIndex, PhysicsPoint>,

    pub standard_bones: Vec<StandardBone>,
}

#[derive(Clone, Debug)]
pub struct BoshSled {
    pub bosh: Bosh,
    pub sled: Sled,

    pub bosh_mounter_bones: Vec<MounterBone>,
    pub sled_mounter_bones: Vec<MounterBone>,
    pub joints: Vec<Joint>,
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
            standard_bones: vec![],
            repel_bones: vec![],
        };

        bosh.standard_bones = make_standard_bones(
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

impl BoshSled {
    pub fn new(bosh: Bosh, sled: Sled) -> BoshSled {
        let mut points = bosh.points.clone();
        points.extend(sled.points.clone().into_iter());

        BoshSled {
            bosh,
            sled,
            bosh_mounter_bones: BoshSled::default_bosh_mounter_bones(&points),
            sled_mounter_bones: BoshSled::default_sled_mounter_bones(&points),
            joints: BoshSled::default_joints(),
        }
    }

    fn default_joints() -> Vec<Joint> {
        vec![
            Joint {
                pair1: (PointIndex::BoshShoulder, PointIndex::BoshButt),
                pair2: (PointIndex::SledRope, PointIndex::SledPeg),
            },
            Joint {
                pair1: (PointIndex::SledPeg, PointIndex::SledTail),
                pair2: (PointIndex::SledRope, PointIndex::SledPeg),
            },
        ]
    }

    fn default_sled_mounter_bones(points: &HashMap<PointIndex, PhysicsPoint>) -> Vec<MounterBone> {
        make_mounter_bones(
            vec![
                (PointIndex::SledPeg, PointIndex::BoshButt, 0.057),
                (PointIndex::SledTail, PointIndex::BoshButt, 0.057),
                (PointIndex::SledNose, PointIndex::BoshButt, 0.057),
            ],
            points,
        )
    }

    fn default_bosh_mounter_bones(points: &HashMap<PointIndex, PhysicsPoint>) -> Vec<MounterBone> {
        make_mounter_bones(
            vec![
                (PointIndex::BoshShoulder, PointIndex::SledPeg, 0.057),
                (PointIndex::SledRope, PointIndex::BoshLeftHand, 0.057),
                (PointIndex::SledRope, PointIndex::BoshRightHand, 0.057),
                (PointIndex::BoshLeftFoot, PointIndex::SledNose, 0.057),
                (PointIndex::BoshRightFoot, PointIndex::SledNose, 0.057),
            ],
            points,
        )
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
            standard_bones: vec![],
        };

        sled.standard_bones = make_standard_bones(
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
        let bosh: Bosh = Default::default();
        let sled: Sled = Default::default();

        let mut points = bosh.points.clone();
        points.extend(sled.points.clone().into_iter());

        BoshSled {
            bosh,
            sled,
            bosh_mounter_bones: BoshSled::default_bosh_mounter_bones(&points),
            sled_mounter_bones: BoshSled::default_sled_mounter_bones(&points),
            joints: BoshSled::default_joints(),
        }
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
        velocity: Default::default(),
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

fn make_mounter_bones(
    bones: Vec<(PointIndex, PointIndex, f64)>,
    point_map: &HashMap<PointIndex, PhysicsPoint>,
) -> Vec<MounterBone> {
    bones
        .iter()
        .map(|(p1, p2, endurance)| MounterBone {
            p1: *p1,
            p2: *p2,
            endurance: *endurance,
            resting_length: length_between(p1, p2, point_map),
        })
        .collect()
}
