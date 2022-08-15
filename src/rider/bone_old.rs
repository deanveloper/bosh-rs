// use crate::rider::entities::PointIndex;
//
// pub trait Bone {
//     fn points(&self) -> (PointIndex, PointIndex);
// }
//
// /// A standard bone is one which simply holds two points together.
// #[derive(Clone, Copy, Debug, PartialEq)]
// pub struct StandardBone {
//     pub p1: PointIndex,
//     pub p2: PointIndex,
//
//     pub resting_length: f64,
// }
//
// /// A Mounter is a bone which holds bosh onto his sled.
// #[derive(Clone, Copy, Debug, PartialEq)]
// pub struct MounterBone {
//     pub p1: PointIndex,
//     pub p2: PointIndex,
//     pub endurance: f64,
//
//     pub resting_length: f64,
// }
//
// /// Repel is a bone which makes sure two points don't get too close to each other.
// #[derive(Clone, Copy, Debug, PartialEq)]
// pub struct RepelBone {
//     pub p1: PointIndex,
//     pub p2: PointIndex,
//     pub length_factor: f64,
//
//     pub resting_length: f64,
// }
//
// impl Bone for StandardBone {
//     fn points(&self) -> (PointIndex, PointIndex) {
//         (self.p1, self.p2)
//     }
// }
// impl Bone for MounterBone {
//     fn points(&self) -> (PointIndex, PointIndex) {
//         (self.p1, self.p2)
//     }
// }
// impl Bone for RepelBone {
//     fn points(&self) -> (PointIndex, PointIndex) {
//         (self.p1, self.p2)
//     }
// }
//
// /// A joint breaks if its cross product is negative. Joints don't actually affect the position
// /// of entities, they only exist to break if needed.
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Joint {
//     pub pair1: (PointIndex, PointIndex),
//     pub pair2: (PointIndex, PointIndex),
// }
