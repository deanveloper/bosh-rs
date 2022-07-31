pub mod advance_frame;
pub mod bone_physics;
pub mod line_physics;
pub mod rider_physics;

#[cfg(test)]
mod tests {
    use crate::physics::rider_physics::PhysicsEntity;
    use crate::rider::entities::BoshSled;

    #[test]
    fn rider_physics_bosh_sled_at_rest() {
        let original_bosh_sled = BoshSled::default();
        let new_bosh_sled = original_bosh_sled.clone();
        let new_bosh_sled = new_bosh_sled.apply_all_bones().unwrap_same();

        assert_eq!(
            original_bosh_sled, new_bosh_sled,
            "should be equal: {:?}, {:?}",
            original_bosh_sled, new_bosh_sled,
        );
    }
}
