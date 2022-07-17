use crate::bosh::{Bosh, BoshSled, Sled};
use crate::track::Track;

/// Rider represents a rider in the game.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Rider {
    Together(BoshSled),
    Separate(Bosh, Sled),
}

/// Frame represents a frozen instance of riders on a track.
///
/// * `'r` - the lifetime of the vector of riders
/// * `'t` - the lifetime of the track
#[derive(Clone)]
pub struct Frame {
    pub riders: Vec<Rider>,
    pub track: Track,
}
