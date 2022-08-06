use crate::physics::bone_physics::{joint_should_break, PhysicsBone};
use crate::physics::line_physics;
use crate::physics::line_physics::PhysicsPoint;
use crate::rider::{Bosh, BoshSled, Entity, PointIndex, Sled};
use crate::track::Track;
use crate::vector::Vector2D;

pub trait PhysicsEntity
where
    Self: Sized,
{
    /// Turns a PhysicsEntity into an Entity
    fn to_entity(self) -> Entity;

    /// Utility function for applying a mapping to all points of the entity
    fn mutate_points<F: FnMut(&mut PhysicsPoint)>(&mut self, mapper: F);

    /// Get a PhysicsPoint at a PointIndex
    fn point_at(&self, index: PointIndex) -> &PhysicsPoint;

    /// Get a mutable reference to the PhysicsPoint at a point index
    fn point_at_mut(&mut self, index: PointIndex) -> &mut PhysicsPoint;

    /// Pushes the points of `self` in accordance to gravity well logic.
    fn apply_gravity_wells(&mut self, track: &Track) {
        self.mutate_points(|p| *p = line_physics::update_position(*p, track))
    }

    /// Applies bone and joint physics to a list of bones. Moves self because
    /// a BoshSled may break, causing `self` to become unusable.
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

    /// Applies bone and joint physics to all of the rider's bones. Moves self because
    /// a BoshSled may break, causing `self` to become unusable.
    fn apply_all_bones(self) -> UpdateBonesResult<Self>;

    /// Performs the logic of stepping the points of the rider to the next frame.
    /// Does not actually do any physics besides applying gravity.
    fn next_points(&mut self, gravity: Vector2D) {
        self.mutate_points(|p| {
            let new_velocity = (p.location - p.previous_location) + gravity;

            *p = PhysicsPoint {
                previous_location: p.location,
                location: p.location + new_velocity,
                friction: 0.0,
            };
        })
    }

    /// Applies all physics steps to the rider in the correct order.
    /// Moves `self` because it may become unusable after the sled breaks.
    fn apply_all_physics(self, track: &Track) -> UpdateBonesResult<Self> {
        self.apply_all_physics_custom_gravity(track, Vector2D(0.0, 0.175))
    }

    /// Applies all physics steps to the rider in the correct order.
    /// Moves `self` because it may become unusable after the sled breaks.
    fn apply_all_physics_custom_gravity(
        self,
        track: &Track,
        gravity: Vector2D,
    ) -> UpdateBonesResult<Self> {
        let mut result = self.apply_all_bones();

        match &mut result {
            UpdateBonesResult::Same(t) => {
                t.apply_gravity_wells(track);

                t.next_points(gravity);
            }
            UpdateBonesResult::Broken(bosh, sled) => {
                bosh.apply_gravity_wells(track);
                sled.apply_gravity_wells(track);

                bosh.next_points(gravity);
                sled.next_points(gravity);
            }
        }

        result
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
    fn to_entity(self) -> Entity {
        Entity::Bosh(self)
    }

    fn mutate_points<F: FnMut(&mut PhysicsPoint)>(&mut self, mapper: F) {
        self.points.values_mut().for_each(mapper);
    }

    fn point_at(&self, index: PointIndex) -> &PhysicsPoint {
        self.points
            .get(&index)
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
    fn to_entity(self) -> Entity {
        Entity::Sled(self)
    }

    fn mutate_points<F: FnMut(&mut PhysicsPoint)>(&mut self, mapper: F) {
        self.points.values_mut().for_each(mapper);
    }

    fn point_at(&self, index: PointIndex) -> &PhysicsPoint {
        self.points
            .get(&index)
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
    fn to_entity(self) -> Entity {
        Entity::BoshSled(self)
    }

    fn mutate_points<F: FnMut(&mut PhysicsPoint)>(&mut self, mut mapper: F) {
        self.bosh.mutate_points(|p| mapper(p));
        self.sled.mutate_points(|p| mapper(p));
    }

    fn point_at(&self, index: PointIndex) -> &PhysicsPoint {
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
            UpdateBonesResult::Same(entity)
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

        if this.joints.iter().any(|j| joint_should_break(j, &this)) {
            broken = true;
        }

        if broken {
            UpdateBonesResult::Broken(this.bosh, this.sled)
        } else {
            UpdateBonesResult::Same(this)
        }
    }
}
