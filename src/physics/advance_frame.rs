use crate::frame::Frame;
use crate::physics::rider_physics::{PhysicsEntity, UpdateBonesResult};
use crate::rider::entities::Entity;

/// Runs the entire physics engine on a frame to get the next frame.
pub fn frame_after<'t>(frame: &Frame<'t>) -> Frame<'t> {
    let new_riders = frame
        .riders
        .iter()
        .flat_map(|r| match r.clone() {
            Entity::BoshSled(bosh_sled) => match bosh_sled.apply_all_physics(frame.track) {
                UpdateBonesResult::Same(bosh_sled) => vec![Entity::BoshSled(bosh_sled)],
                UpdateBonesResult::Broken(bosh, sled) => {
                    vec![Entity::Bosh(bosh), Entity::Sled(sled)]
                }
            },
            Entity::Bosh(bosh) => vec![Entity::Bosh(
                bosh.apply_all_physics(frame.track).unwrap_same(),
            )],
            Entity::Sled(sled) => vec![Entity::Sled(
                sled.apply_all_physics(frame.track).unwrap_same(),
            )],
        })
        .collect();

    Frame {
        riders: new_riders,
        track: frame.track,
    }
}
