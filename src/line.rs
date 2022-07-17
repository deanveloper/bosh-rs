use crate::track::Point;

pub enum LineType {
    Normal,
    Accelerate { amount: u64 },
    Scenery,
}

pub struct Line {
    pub line_type: LineType,
    pub points: (Point, Point),
}
