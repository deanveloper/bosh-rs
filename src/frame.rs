use crate::rider::Entity;
use crate::track::Track;

/// Frame represents a frozen instance of riders on a track.
#[derive(Clone)]
pub struct Frame<'t> {
    pub riders: Vec<Entity>,
    pub track: &'t Track,
}
