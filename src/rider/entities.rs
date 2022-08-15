use std::collections::HashMap;

use crate::game::Vector2D;
use crate::rider::bone::{Bone, BoneType};
use crate::rider::point::{EntityPoint, PointIndex};
use crate::rider::Joint;

#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    pub points: HashMap<PointIndex, EntityPoint>,

    pub bones: Vec<Bone>,
    pub joints: Vec<Joint>,
}

impl Entity {
    /// Get a PhysicsPoint at a PointIndex
    pub fn point_at(&self, index: PointIndex) -> &EntityPoint {
        self.points
            .get(&index)
            .unwrap_or_else(|| panic!("invalid index {index:?}"))
    }

    /// Get a mutable reference to the PhysicsPoint at a point index
    pub fn point_at_mut(&mut self, index: PointIndex) -> &mut EntityPoint {
        self.points
            .get_mut(&index)
            .unwrap_or_else(|| panic!("invalid index {index:?}"))
    }

    pub fn default_boshsled() -> Entity {
        let points = boshsled::default_points();
        let bones = boshsled::default_bones(&points);
        let joints = boshsled::default_joints();
        Entity {
            points,
            bones,
            joints,
        }
    }

    pub fn default_bosh() -> Entity {
        let points = bosh::default_points();
        let bones = bosh::default_bones(&points);
        Entity {
            points,
            bones,
            joints: Default::default(),
        }
    }

    pub fn default_sled() -> Entity {
        let points = sled::default_points();
        let bones = sled::default_bones(&points);
        Entity {
            points,
            bones,
            joints: Default::default(),
        }
    }
}

pub mod boshsled {
    use super::*;

    pub const DEFAULT_SLED_MOUNT_BONES: usize = 3;
    pub const DEFAULT_BOSH_MOUNT_BONES: usize = 5;
    pub const DEFAULT_BONE_COUNT: usize = bosh::DEFAULT_BONE_COUNT
        + sled::DEFAULT_BONE_COUNT
        + DEFAULT_SLED_MOUNT_BONES
        + DEFAULT_BOSH_MOUNT_BONES;

    pub fn default_points() -> HashMap<PointIndex, EntityPoint> {
        sled::default_points()
            .into_iter()
            .chain(bosh::default_points())
            .collect()
    }

    pub fn default_bones(points: &HashMap<PointIndex, EntityPoint>) -> Vec<Bone> {
        vec![
            sled::default_bones(points),
            default_sled_mounter_bones(points),
            bosh::default_bones(points),
            default_bosh_mounter_bones(points),
        ]
        .concat()
    }

    // TODO - precompute resting lengths of bones
    pub fn default_sled_mounter_bones(points: &HashMap<PointIndex, EntityPoint>) -> Vec<Bone> {
        make_bones(
            vec![
                (
                    PointIndex::SledPeg,
                    PointIndex::BoshButt,
                    BoneType::Mount { endurance: 0.057 },
                ),
                (
                    PointIndex::SledTail,
                    PointIndex::BoshButt,
                    BoneType::Mount { endurance: 0.057 },
                ),
                (
                    PointIndex::SledNose,
                    PointIndex::BoshButt,
                    BoneType::Mount { endurance: 0.057 },
                ),
            ],
            points,
        )
    }

    pub fn default_joints() -> Vec<Joint> {
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

    // TODO - precompute resting lengths of bones
    fn default_bosh_mounter_bones(points: &HashMap<PointIndex, EntityPoint>) -> Vec<Bone> {
        make_bones(
            vec![
                (
                    PointIndex::BoshShoulder,
                    PointIndex::SledPeg,
                    BoneType::Mount { endurance: 0.057 },
                ),
                (
                    PointIndex::SledRope,
                    PointIndex::BoshLeftHand,
                    BoneType::Mount { endurance: 0.057 },
                ),
                (
                    PointIndex::SledRope,
                    PointIndex::BoshRightHand,
                    BoneType::Mount { endurance: 0.057 },
                ),
                (
                    PointIndex::BoshLeftFoot,
                    PointIndex::SledNose,
                    BoneType::Mount { endurance: 0.057 },
                ),
                (
                    PointIndex::BoshRightFoot,
                    PointIndex::SledNose,
                    BoneType::Mount { endurance: 0.057 },
                ),
            ],
            points,
        )
    }
}

pub mod bosh {
    use super::*;

    pub const DEFAULT_BONE_COUNT: usize = 8;

    pub fn default_points() -> HashMap<PointIndex, EntityPoint> {
        let left_foot = make_entity_point(Vector2D(10.0, 5.0), 0.0);
        let right_foot = make_entity_point(Vector2D(10.0, 5.0), 0.0);
        let left_hand = make_entity_point(Vector2D(11.5, -5.0), 0.1);
        let right_hand = make_entity_point(Vector2D(11.5, -5.0), 0.1);
        let shoulder = make_entity_point(Vector2D(5.0, -5.5), 0.8);
        let butt = make_entity_point(Vector2D(5.0, 0.0), 0.8);

        HashMap::from([
            (PointIndex::BoshLeftFoot, left_foot),
            (PointIndex::BoshRightFoot, right_foot),
            (PointIndex::BoshLeftHand, left_hand),
            (PointIndex::BoshRightHand, right_hand),
            (PointIndex::BoshShoulder, shoulder),
            (PointIndex::BoshButt, butt),
        ])
    }

    // TODO - precompute resting lengths of bones
    pub fn default_bones(points: &HashMap<PointIndex, EntityPoint>) -> Vec<Bone> {
        make_bones(
            vec![
                (
                    PointIndex::BoshShoulder,
                    PointIndex::BoshButt,
                    BoneType::Normal,
                ),
                (
                    PointIndex::BoshShoulder,
                    PointIndex::BoshLeftHand,
                    BoneType::Normal,
                ),
                (
                    PointIndex::BoshShoulder,
                    PointIndex::BoshRightHand,
                    BoneType::Normal,
                ),
                (
                    PointIndex::BoshButt,
                    PointIndex::BoshLeftFoot,
                    BoneType::Normal,
                ),
                (
                    PointIndex::BoshButt,
                    PointIndex::BoshRightFoot,
                    BoneType::Normal,
                ),
                (
                    PointIndex::BoshShoulder,
                    PointIndex::BoshRightHand,
                    BoneType::Normal,
                ),
                (
                    PointIndex::BoshShoulder,
                    PointIndex::BoshLeftFoot,
                    BoneType::Repel { length_factor: 0.5 },
                ),
                (
                    PointIndex::BoshShoulder,
                    PointIndex::BoshRightFoot,
                    BoneType::Repel { length_factor: 0.5 },
                ),
            ],
            points,
        )
    }
}

mod sled {
    use super::*;

    pub const DEFAULT_BONE_COUNT: usize = 6;

    pub fn default_points() -> HashMap<PointIndex, EntityPoint> {
        let peg = make_entity_point(Vector2D(0.0, 0.0), 0.8);
        let nose = make_entity_point(Vector2D(15.0, 5.0), 0.0);
        let tail = make_entity_point(Vector2D(0.0, 5.0), 0.0);
        let rope = make_entity_point(Vector2D(17.5, 0.0), 0.0);

        HashMap::from([
            (PointIndex::SledPeg, peg),
            (PointIndex::SledNose, nose),
            (PointIndex::SledTail, tail),
            (PointIndex::SledRope, rope),
        ])
    }

    pub fn default_bones(points: &HashMap<PointIndex, EntityPoint>) -> Vec<Bone> {
        make_bones(
            vec![
                (PointIndex::SledPeg, PointIndex::SledTail, BoneType::Normal),
                (PointIndex::SledTail, PointIndex::SledNose, BoneType::Normal),
                (PointIndex::SledNose, PointIndex::SledRope, BoneType::Normal),
                (PointIndex::SledRope, PointIndex::SledPeg, BoneType::Normal),
                (PointIndex::SledPeg, PointIndex::SledNose, BoneType::Normal),
                (PointIndex::SledRope, PointIndex::SledTail, BoneType::Normal),
            ],
            points,
        )
    }
}

// ==== PRIVATE UTIL FUNCTIONS ====

fn make_entity_point(loc: Vector2D, friction: f64) -> EntityPoint {
    EntityPoint {
        previous_location: loc,
        location: loc,
        friction,
    }
}

fn make_bones(
    bones: Vec<(PointIndex, PointIndex, BoneType)>,
    point_map: &HashMap<PointIndex, EntityPoint>,
) -> Vec<Bone> {
    bones
        .iter()
        .map(|(p1, p2, bone_type)| Bone {
            p1: *p1,
            p2: *p2,
            resting_length: length_between(p1, p2, point_map),
            bone_type: *bone_type,
        })
        .collect()
}
fn length_between(
    p1: &PointIndex,
    p2: &PointIndex,
    point_map: &HashMap<PointIndex, EntityPoint>,
) -> f64 {
    (point_map.get(p2).expect("").location - point_map.get(p1).expect("").location)
        .length_squared()
        .sqrt()
}
