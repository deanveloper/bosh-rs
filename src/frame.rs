use crate::bosh::{Bosh, BoshSled, Sled};
use crate::track::Track;
use std::rc::Rc;

/// Rider represents a rider in the game.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Rider {
    Together(BoshSled),
    Separate(Bosh, Sled),
}

/// Frame represents a frozen instance of riders on a track.
#[derive(Clone)]
pub struct Frame<'a> {
    pub riders: Vec<Rider>,
    pub track: Rc<Track<'a>>,
}
