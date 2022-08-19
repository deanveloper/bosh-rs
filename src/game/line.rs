use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::game::vector::Vector2D;

const ANTIGRAVITY_WELL_RATIO: f64 = 0.25;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum LineType {
    Normal,
    Accelerate { amount: u64 },
    Scenery,
}

impl Default for LineType {
    fn default() -> Self {
        LineType::Normal
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Default)]
pub struct LinePoint {
    pub location: Vector2D,
    #[serde(skip_serializing_if = "is_false", default)]
    pub extended: bool,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Default)]
pub struct Line {
    pub ends: (LinePoint, LinePoint),
    #[serde(rename = "lineType")]
    pub line_type: LineType,
    pub flipped: bool,
}

#[derive(Default)]
pub struct LineBuilder {
    first_location_init: bool,
    second_location_init: bool,
    line: Line,
}

impl LineBuilder {
    pub fn line_type(mut self, line_type: LineType) -> LineBuilder {
        self.line.line_type = line_type;
        self
    }
    pub fn flipped(mut self, flipped: bool) -> LineBuilder {
        self.line.flipped = flipped;
        self
    }
    pub fn point(mut self, p1: f64, p2: f64) -> LineBuilder {
        if !self.first_location_init {
            self.line.ends.0.location = Vector2D(p1, p2);
            self.first_location_init = true;
        } else {
            self.line.ends.1.location = Vector2D(p1, p2);
            self.second_location_init = true;
        }

        self
    }
    pub fn point_vec(mut self, point: Vector2D) -> LineBuilder {
        if !self.first_location_init {
            self.line.ends.0.location = point;
        } else {
            self.line.ends.1.location = point;
        }

        self
    }
    pub fn extended(mut self, extended: bool) -> LineBuilder {
        if !self.first_location_init {
            panic!("extended should be called after the point is located");
        } else if !self.second_location_init {
            self.line.ends.0.extended = extended;
        } else {
            self.line.ends.1.extended = extended;
        }

        self
    }
    pub fn build(self) -> Line {
        self.line
    }
}

impl Line {
    pub fn builder() -> LineBuilder {
        Default::default()
    }

    pub fn as_vector2d(&self) -> Vector2D {
        self.ends.1.location - self.ends.0.location
    }

    pub fn length_squared(&self) -> f64 {
        self.ends.0.location.distance_squared(self.ends.1.location)
    }

    /// Returns the perpendicular unit vector for this line, facing "upwards" (the direction
    /// in which it applies force).
    pub fn perpendicular(&self) -> Vector2D {
        if self.flipped {
            self.as_vector2d().rotate90_right().normalize()
        } else {
            self.as_vector2d().rotate90_left().normalize()
        }
    }

    pub fn hitbox_extensions(&self) -> (f64, f64) {
        let clamped_len = (self.length_squared().sqrt() * ANTIGRAVITY_WELL_RATIO).clamp(0.0, 10.0);
        let mut extensions = (0.0, 0.0);

        if self.ends.0.extended {
            extensions.0 = clamped_len;
        }
        if self.ends.1.extended {
            extensions.1 = clamped_len;
        }

        extensions
    }
}

fn is_false(b: &bool) -> bool {
    !*b
}
