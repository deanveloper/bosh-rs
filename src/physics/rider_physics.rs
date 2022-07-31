use crate::physics::bone_physics::PhysicsBone;
use crate::physics::line_physics;
use crate::physics::line_physics::PhysicsPoint;
use crate::rider::entities::{Bosh, BoshSled, PointIndex, Sled};
use crate::track::Track;
use crate::vector::Vector2D;

pub trait PhysicsEntity
where
    Self: Sized,
{
    fn mutate_points<F: FnMut(&mut PhysicsPoint)>(&mut self, mapper: F);
    fn point_at(&self, index: PointIndex) -> PhysicsPoint;
    fn point_at_mut(&mut self, index: PointIndex) -> &mut PhysicsPoint;

    /// Moves self because BoshSleds may break and become unusable.
    fn apply_bones<B: PhysicsBone>(self, bones: &Vec<B>) -> UpdateBonesResult<Self> {
        let mut entity = self;
        for bone in bones {
            if let Some((p1, p2)) = bone.next_locations(&entity) {
                let (i1, i2) = bone.points();
                entity.point_at_mut(i1).location = p1;
                entity.point_at_mut(i2).location = p2;
            }
        }

        UpdateBonesResult::Same(entity)
    }

    /// Moves self because BoshSleds may break and become unusable.
    fn apply_all_bones(self) -> UpdateBonesResult<Self>;

    fn apply_gravity(&mut self, accel: Vector2D) {
        self.mutate_points(|p| p.velocity += accel);
    }

    fn apply_gravity_wells(&mut self, track: &Track) {
        self.mutate_points(|p| *p = line_physics::update_position(*p, track))
    }
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
    fn mutate_points<F: FnMut(&mut PhysicsPoint)>(&mut self, mapper: F) {
        self.points.values_mut().for_each(mapper);
    }

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

    fn apply_all_bones(self) -> UpdateBonesResult<Self> {
        let mut this = self;

        let standard_bones = &this.standard_bones.clone();
        this = this.apply_bones(standard_bones).unwrap_same();

        let repel_bones = &this.standard_bones.clone();
        this = this.apply_bones(repel_bones).unwrap_same();

        UpdateBonesResult::Same(this)
    }
}
impl PhysicsEntity for Sled {
    fn mutate_points<F: FnMut(&mut PhysicsPoint)>(&mut self, mapper: F) {
        self.points.values_mut().for_each(mapper);
    }

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

    fn apply_all_bones(self) -> UpdateBonesResult<Self> {
        let mut this = self;

        let standard_bones = &this.standard_bones.clone();
        this = this.apply_bones(standard_bones).unwrap_same();

        UpdateBonesResult::Same(this)
    }
}
impl PhysicsEntity for BoshSled {
    fn mutate_points<F: FnMut(&mut PhysicsPoint)>(&mut self, mapper: F) {
        let mut points = self.bosh.points.clone();
        points.extend(&self.sled.points);

        points.values_mut().for_each(mapper);
    }

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

    fn apply_bones<B: PhysicsBone>(self, bones: &Vec<B>) -> UpdateBonesResult<Self> {
        let mut entity = self;

        let mut broken = false;

        for bone in bones {
            if let Some((p1, p2)) = bone.next_locations(&entity) {
                let (i1, i2) = bone.points();
                entity.point_at_mut(i1).location = p1;
                entity.point_at_mut(i2).location = p2;
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
                sled_mounter_bones: entity.sled_mounter_bones,
                bosh_mounter_bones: entity.bosh_mounter_bones,
            })
        }
    }

    fn apply_all_bones(self) -> UpdateBonesResult<Self> {
        let mut this = self;

        let mut broken = false;

        let standard_bones = &this.sled.standard_bones.clone();
        this = this.apply_bones(standard_bones).unwrap_same();

        let mounter_bones = &this.sled_mounter_bones.clone();
        this = match this.apply_bones(mounter_bones) {
            UpdateBonesResult::Same(bosh_sled) => bosh_sled,
            UpdateBonesResult::Broken(bosh, sled) => {
                broken = true;
                BoshSled::new(bosh, sled)
            }
        };

        let standard_bones = &this.bosh.standard_bones.clone();
        this = this.apply_bones(standard_bones).unwrap_same();

        let mounter_bones = &this.bosh_mounter_bones.clone();
        this = match this.apply_bones(mounter_bones) {
            UpdateBonesResult::Same(bosh_sled) => bosh_sled,
            UpdateBonesResult::Broken(bosh, sled) => {
                broken = true;
                BoshSled::new(bosh, sled)
            }
        };

        let repel_bones = &this.bosh.repel_bones.clone();
        this = this.apply_bones(repel_bones).unwrap_same();

        if broken {
            UpdateBonesResult::Broken(this.bosh, this.sled)
        } else {
            UpdateBonesResult::Same(this)
        }
    }
}
