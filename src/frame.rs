use crate::rider::entities::Entity;
use crate::track::Track;
use std::rc::Rc;

/// Frame represents a frozen instance of riders on a track.
#[derive(Clone)]
pub struct Frame {
    pub riders: Vec<Entity>,
    pub track: Rc<Track>,
}
