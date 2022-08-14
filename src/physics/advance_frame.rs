use crate::physics::entity_physics::{PhysicsEntity, UpdateBonesResult};
use crate::rider::Entity;
use crate::Track;

/// Runs the entire physics engine on a frame to get the next frame.
pub fn frame_after(riders: &[Entity], track: &Track) -> Vec<Entity> {
    riders
        .iter()
        .flat_map(|r| match r.clone() {
            Entity::BoshSled(bosh_sled) => match bosh_sled.apply_all_physics(track) {
                UpdateBonesResult::Same(bosh_sled) => vec![Entity::BoshSled(bosh_sled)],
                UpdateBonesResult::Broken(bosh, sled) => {
                    vec![Entity::Bosh(bosh), Entity::Sled(sled)]
                }
            },
            Entity::Bosh(bosh) => vec![Entity::Bosh(bosh.apply_all_physics(track).unwrap_same())],
            Entity::Sled(sled) => vec![Entity::Sled(sled.apply_all_physics(track).unwrap_same())],
        })
        .collect()
}
