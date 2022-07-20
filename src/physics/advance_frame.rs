use crate::frame::Frame;

/// The entry point to the physics engine. This takes a frame, and moves
/// all of the riders according to the rest of the physics engine.
pub fn advance_frame<'a>(frame: &Frame) -> Frame {
    Frame {
        riders: frame.riders.clone(),
        track: frame.track.clone(),
    }
}
