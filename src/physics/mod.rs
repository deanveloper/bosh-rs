pub mod advance_frame;
pub mod line_physics;
pub mod rider_physics;

#[cfg(test)]
mod tests {
    use crate::physics::rider_physics::PhysicsEntity;
    use crate::rider::entities::Bosh;

    #[test]
    fn rider_physics_bosh_at_rest() {
        let original_bosh = Bosh::default();
        let new_bosh = original_bosh.clone();
        let new_bosh = new_bosh.apply_bones().unwrap_same();

        assert_eq!(
            original_bosh, new_bosh,
            "should be equal: {:?}, {:?}",
            original_bosh, new_bosh,
        );
    }
}
