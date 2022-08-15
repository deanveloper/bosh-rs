use crate::physics::entity_physics::UpdateBonesResult;
use crate::rider::Entity;
use crate::Track;

/// Runs the entire physics engine on a frame to get the next frame.
pub fn frame_after(riders: &[Entity], track: &Track) -> Vec<Entity> {
    riders
        .iter()
        .flat_map(|entity| match entity.clone().apply_all_physics_ez(track) {
            UpdateBonesResult::Same(bosh_sled) => vec![bosh_sled],
            UpdateBonesResult::Broken(bosh, sled) => {
                vec![bosh, sled]
            }
        })
        .collect()
}
