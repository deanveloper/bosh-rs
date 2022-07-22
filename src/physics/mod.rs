pub mod advance_frame;
pub mod line_physics;
pub mod rider_physics;

#[cfg(test)]
mod tests {
    use crate::physics::rider_physics::update_bones;
    use crate::rider::entities::{Bosh, Entity};

    #[test]
    fn rider_physics() {
        let original_bosh = Entity::Bosh(Bosh::new());
        let new_bosh = original_bosh.clone();
        update_bones(&new_bosh);

        println!("lmao");
        assert_eq!(original_bosh, new_bosh, "message");
    }
}
