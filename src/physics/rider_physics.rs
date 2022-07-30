use crate::physics::bone_physics::PhysicsBone;
use crate::physics::line_physics::PhysicsPoint;
use crate::rider::entities::{Bosh, BoshSled, PointIndex, Sled};
use crate::vector::Vector2D;

pub trait PhysicsEntity
where
    Self: Sized,
{
    fn point_at(&self, index: PointIndex) -> PhysicsPoint;
    fn point_at_mut(&mut self, index: PointIndex) -> &mut PhysicsPoint;
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
    fn point_at(&self, index: PointIndex) -> PhysicsPoint {
        self.points
            .get(&index)
            .copied()
            .unwrap_or_else(|| panic!("invalid index {index:?}"))
    }

    fn point_at_mut(&mut self, index: PointIndex) -> &mut PhysicsPoint {
        self.points
            .get_mut(&index)
            .unwrap_or_else(|| panic!("invalid index {index:?}"))
    }

    fn apply_bones(self) -> UpdateBonesResult<Self> {
        let mut entity = self;
        for bone in &entity.bones.clone() {
            if let Some((p1, p2)) = bone.next_locations(&entity) {
                entity.point_at_mut(bone.p1).location = p1;
                entity.point_at_mut(bone.p2).location = p2;
            }
        }

        for bone in &entity.repel_bones.clone() {
            if let Some((p1, p2)) = bone.next_locations(&entity) {
                entity.point_at_mut(bone.p1).location = p1;
                entity.point_at_mut(bone.p2).location = p2;
            }
        }

        UpdateBonesResult::Same(entity)
    }

    fn apply_gravity(mut self, accel: Vector2D) -> Self {
        for p in self.points.values_mut() {
            p.velocity += accel;
        }

        self
    }
}
impl PhysicsEntity for Sled {
    fn point_at(&self, index: PointIndex) -> PhysicsPoint {
        self.points
            .get(&index)
            .copied()
            .unwrap_or_else(|| panic!("invalid index {index:?}"))
    }

    fn point_at_mut(&mut self, index: PointIndex) -> &mut PhysicsPoint {
        self.points
            .get_mut(&index)
            .unwrap_or_else(|| panic!("invalid index {index:?}"))
    }

    fn apply_bones(self) -> UpdateBonesResult<Self> {
        let mut entity = self;
        for bone in &entity.bones.clone() {
            if let Some((p1, p2)) = bone.next_locations(&entity) {
                entity.point_at_mut(bone.p1).location = p1;
                entity.point_at_mut(bone.p2).location = p2;
            }
        }

        UpdateBonesResult::Same(entity)
    }

    fn apply_gravity(mut self, accel: Vector2D) -> Self {
        for p in self.points.values_mut() {
            p.velocity += accel;
        }

        self
    }
}
impl PhysicsEntity for BoshSled {
    fn point_at(&self, index: PointIndex) -> PhysicsPoint {
        if index.is_bosh() {
            self.bosh.point_at(index)
        } else {
            self.sled.point_at(index)
        }
    }

    fn point_at_mut(&mut self, index: PointIndex) -> &mut PhysicsPoint {
        if index.is_bosh() {
            self.bosh.point_at_mut(index)
        } else {
            self.sled.point_at_mut(index)
        }
    }

    fn apply_bones(self) -> UpdateBonesResult<Self> {
        let mut entity = self;

        entity.bosh = entity.bosh.apply_bones().unwrap_same();
        entity.sled = entity.sled.apply_bones().unwrap_same();

        let mut broken = false;
        for bone in entity.mounter_bones.clone().iter() {
            if let Some((p1, p2)) = bone.next_locations(&entity) {
                entity.point_at_mut(bone.p1).location = p1;
                entity.point_at_mut(bone.p2).location = p2;
            } else {
                broken = true;
            }
        }

        if broken {
            UpdateBonesResult::Broken(entity.bosh, entity.sled)
        } else {
            UpdateBonesResult::Same(BoshSled {
                bosh: entity.bosh,
                sled: entity.sled,
                mounter_bones: entity.mounter_bones,
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
