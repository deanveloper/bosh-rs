pub mod advance_frame;
pub mod line_physics;
pub mod rider_physics;

#[cfg(test)]
mod tests {
    use crate::physics::rider_physics::update_bones;
    use crate::rider::entities::{Bosh, Entity};

    #[test]
    fn rider_physics() {
        let original_bosh = Entity::Bosh(Bosh::default());
        let mut new_bosh = original_bosh.clone();
        update_bones(&mut new_bosh);

        assert_eq!(
            original_bosh, new_bosh,
            "should be equal: {:?}, {:?}",
            original_bosh, new_bosh,
        );
    }
}
