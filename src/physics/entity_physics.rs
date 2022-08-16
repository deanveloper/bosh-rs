use crate::game::Track;
use crate::game::Vector2D;
use crate::physics::bone_physics::{joint_should_break, next_bone_locations};
use crate::physics::line_physics::apply_gravity_wells;
use crate::rider::{Entity, EntityPoint};

pub type PhysicsEntity = Entity;

impl PhysicsEntity {
    /// Pushes the points of `self` in accordance to gravity well logic.
    pub fn apply_gravity_wells(&mut self, track: &Track) {
        self.mutate_points(|p| apply_gravity_wells(p, track))
    }

    /// Applies bone physics to a list of bones. Moves self because
    /// a BoshSled may break, causing `self` to become unusable.
    pub fn apply_bones(mut self) -> UpdateBonesResult {
        let mut broken = false;
        for bone in self.bones.clone() {
            if let Some((next_p1, next_p2)) = next_bone_locations(&bone, &self) {
                self.point_at_mut(bone.p1).location = next_p1;
                self.point_at_mut(bone.p2).location = next_p2;
            } else {
                broken = true
            }
        }

        if broken {
            let (bosh, sled) = self.split();
            UpdateBonesResult::Broken(bosh, sled)
        } else {
            UpdateBonesResult::Same(self)
        }
    }

    /// Performs the logic of stepping the points of the rider to the next frame.
    /// Does not actually do any physics besides applying gravity.
    pub fn next_points(&mut self, gravity: Vector2D) {
        self.mutate_points(|p| {
            let new_velocity = (p.location - p.previous_location) + gravity;

            *p = EntityPoint {
                previous_location: p.location,
                location: p.location + new_velocity,
                friction: p.friction,
            };
        })
    }

    /// applies joint logic
    /// does nothing on non-boshsleds
    pub fn apply_all_joints(self) -> UpdateBonesResult {
        if self.joints.iter().any(|j| joint_should_break(j, &self)) {
            let (bosh, sled) = self.split();
            UpdateBonesResult::Broken(bosh, sled)
        } else {
            UpdateBonesResult::Same(self)
        }
    }

    /// Applies all physics steps to the rider in the correct order.
    /// Moves `self` because it may become unusable after the sled breaks.
    pub fn apply_all_physics_ez(self, track: &Track) -> UpdateBonesResult {
        self.apply_all_physics(track, Vector2D(0.0, 0.175), 6)
    }

    /// Applies all physics steps to the rider in the correct order.
    /// Moves `self` because it may become unusable after the sled breaks.
    pub fn apply_all_physics(
        mut self,
        track: &Track,
        gravity: Vector2D,
        iterations: u64,
    ) -> UpdateBonesResult {
        self.next_points(gravity);

        let mut result = UpdateBonesResult::Same(self);

        for _ in 0..iterations {
            result = match result {
                UpdateBonesResult::Same(same) => same.apply_bones(),
                UpdateBonesResult::Broken(bosh, sled) => {
                    let bosh = bosh.apply_bones().unwrap_same();
                    let sled = sled.apply_bones().unwrap_same();

                    UpdateBonesResult::Broken(bosh, sled)
                }
            };
            match &mut result {
                UpdateBonesResult::Same(same) => {
                    same.apply_gravity_wells(track);
                }
                UpdateBonesResult::Broken(bosh, sled) => {
                    bosh.apply_gravity_wells(track);
                    sled.apply_gravity_wells(track);
                }
            }
        }

        match result {
            UpdateBonesResult::Same(same) => same.apply_all_joints(),
            UpdateBonesResult::Broken(_, _) => result,
        }
    }
}

pub enum UpdateBonesResult {
    Same(PhysicsEntity),
    Broken(PhysicsEntity, PhysicsEntity),
}

impl UpdateBonesResult {
    pub fn unwrap_same(self) -> PhysicsEntity {
        if let UpdateBonesResult::Same(entity) = self {
            entity
        } else {
            panic!("unwrap_same called on UpdateBonesResult::Broken")
        }
    }
}
