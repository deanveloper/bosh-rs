use crate::physics::line_physics::PhysicsPoint;
use crate::rider::bone::{Bone, RepelBone, StandardBone};
use crate::rider::entities::{Bosh, BoshSled, PointIndex, Sled};
use crate::vector::Vector2D;
use std::collections::HashMap;

pub trait PhysicsEntity
where
    Self: Sized,
{
    fn apply_bones(self) -> UpdateBonesResult<Self>;
    fn apply_gravity(self, accel: Vector2D) -> Self;
}

pub enum UpdateBonesResult<T: PhysicsEntity> {
    Same(T),
    Broken(Bosh, Sled),
}

impl<T: PhysicsEntity> UpdateBonesResult<T> {
    pub fn unwrap_same(self) -> T {
        if let UpdateBonesResult::Same(t) = self {
            t
        } else {
            panic!("unwrap_same called on UpdateBonesResult::Broken")
        }
    }
}

impl PhysicsEntity for Bosh {
    fn apply_bones(mut self) -> UpdateBonesResult<Self> {
        for bone in &self.bones {
            apply_bone(bone, &mut self.points, next_standardbone_locs);
        }

        for bone in &self.repel_bones {
            apply_bone(bone, &mut self.points, next_repelbone_locs);
        }

        UpdateBonesResult::Same(self)
    }

    fn apply_gravity(mut self, accel: Vector2D) -> Self {
        for p in self.points.values_mut() {
            p.velocity += accel;
        }

        self
    }
}
impl PhysicsEntity for Sled {
    fn apply_bones(mut self) -> UpdateBonesResult<Self> {
        for bone in &self.bones {
            apply_bone(bone, &mut self.points, next_standardbone_locs);
        }

        UpdateBonesResult::Same(self)
    }

    fn apply_gravity(mut self, accel: Vector2D) -> Self {
        for p in self.points.values_mut() {
            p.velocity += accel;
        }

        self
    }
}
impl PhysicsEntity for BoshSled {
    #![allow(unreachable_code, unused_variables)]
    fn apply_bones(self) -> UpdateBonesResult<Self> {
        // just recursively call on the bosh and the sled
        let bosh = self.bosh.apply_bones().unwrap_same();
        let sled = self.sled.apply_bones().unwrap_same();

        todo!("check mounter bones");

        if todo!("if sled is broken") {
            UpdateBonesResult::Broken(bosh, sled)
        } else {
            UpdateBonesResult::Same(BoshSled {
                bosh,
                sled,
                mounter_bones: self.mounter_bones,
            })
        }
    }

    fn apply_gravity(self, accel: Vector2D) -> Self {
        let bosh = self.bosh.apply_gravity(accel);
        let sled = self.sled.apply_gravity(accel);

        BoshSled {
            bosh,
            sled,
            mounter_bones: self.mounter_bones,
        }
    }
}

/// Generic wrapper to easily use next_repelbone_locs/next_standardbone_locs
fn apply_bone<T, F>(bone: &T, points: &mut HashMap<PointIndex, PhysicsPoint>, next_locs: F)
where
    T: Bone,
    F: Fn(&T, &HashMap<PointIndex, PhysicsPoint>) -> (Vector2D, Vector2D),
{
    let (i1, i2) = bone.points();
    let (p1, p2) = next_locs(bone, points);

    if let Some(p) = points.get_mut(&i1) {
        p.location = p1
    }
    if let Some(p) = points.get_mut(&i2) {
        p.location = p2
    }
}

pub fn next_repelbone_locs(
    bone: &RepelBone,
    point_map: &HashMap<PointIndex, PhysicsPoint>,
) -> (Vector2D, Vector2D) {
    let p1 = point_map.get(&bone.p1).expect("no p1 found");
    let p2 = point_map.get(&bone.p2).expect("no p2 found");

    let diff = p2.location - p1.location;
    let length = diff.length_squared().sqrt();

    if length >= bone.resting_length * bone.length_factor {
        return (p1.location, p2.location);
    }

    stick_resolve(
        p1.location,
        p2.location,
        get_diff(bone.resting_length * bone.length_factor * 0.5, length),
    )
}

pub fn next_standardbone_locs(
    bone: &StandardBone,
    point_map: &HashMap<PointIndex, PhysicsPoint>,
) -> (Vector2D, Vector2D) {
    let p1 = point_map.get(&bone.p1).expect("no p1 found");
    let p2 = point_map.get(&bone.p2).expect("no p2 found");

    let diff = p2.location - p1.location;
    let length = diff.length_squared().sqrt();

    stick_resolve(
        p1.location,
        p2.location,
        get_diff(bone.resting_length, length),
    )
}

fn stick_resolve(p1: Vector2D, p2: Vector2D, diff: f64) -> (Vector2D, Vector2D) {
    let delta = (p1 - p2) * diff;

    (p1 - delta, delta + p2)
}

fn get_diff(standard_length: f64, current_length: f64) -> f64 {
    if current_length == 0.0 {
        0.0
    } else {
        (current_length - standard_length) / current_length
    }
}
