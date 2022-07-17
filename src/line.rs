use crate::track::Point;

#[derive(Copy, Clone)]
pub enum LineType {
    Normal,
    Accelerate { amount: u64 },
    Scenery,
}

#[derive(Copy, Clone)]
pub struct Line {
    pub line_type: LineType,
    pub points: (Point, Point),
}

impl Line {
    fn hitbox(&self) {}
}
