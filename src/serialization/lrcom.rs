//! Module for deserializing linerider.com tracks, aka ".track.json"

use crate::rider::{Entity, PointIndex};
use crate::{Line, LineType, Track, Vector2D};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::borrow::Borrow;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_LINE_ID: AtomicU64 = AtomicU64::new(0);

type Result<T> = anyhow::Result<T>;

#[derive(Clone, Serialize, Deserialize)]
pub struct LRComTrack {
    label: String,
    creator: String,
    description: String,
    duration: u64,
    version: String,
    audio: Option<()>,
    #[serde(rename = "startPosition")]
    start_position: LRComVec2,
    riders: Vec<LRComEntity>,
    lines: Vec<LRComLine>,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct LRComEntity {
    #[serde(rename = "startPosition")]
    start_position: LRComVec2,
    #[serde(rename = "startVelocity")]
    start_velocity: LRComVec2,
    remountable: bool,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct LRComVec2 {
    x: f64,
    y: f64,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct LRComLine {
    id: u64,
    #[serde(rename = "type")]
    line_type: LRComLineType,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    flipped: bool,
    #[serde(rename = "leftExtended")]
    left_extended: bool,
    #[serde(rename = "leftExtended")]
    right_extended: bool,
}

#[derive(Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LRComLineType {
    Normal = 0,
    Accelerate = 1,
    Scenery = 2,
}

impl From<LRComVec2> for Vector2D {
    fn from(vector: LRComVec2) -> Vector2D {
        let vector = vector.borrow();
        Vector2D(vector.x, vector.y)
    }
}

impl From<Vector2D> for LRComVec2 {
    fn from(vector: Vector2D) -> LRComVec2 {
        let vector = vector.borrow();
        LRComVec2 {
            x: vector.0,
            y: vector.1,
        }
    }
}

impl From<LineType> for LRComLineType {
    fn from(line_type: LineType) -> LRComLineType {
        match line_type.borrow() {
            LineType::Normal => LRComLineType::Normal,
            LineType::Accelerate { .. } => LRComLineType::Accelerate,
            LineType::Scenery => LRComLineType::Scenery,
        }
    }
}

impl From<LRComLineType> for LineType {
    fn from(line_type: LRComLineType) -> LineType {
        match line_type.borrow() {
            LRComLineType::Normal => LineType::Normal,
            LRComLineType::Accelerate => LineType::Accelerate { amount: 1 },
            LRComLineType::Scenery => LineType::Scenery,
        }
    }
}

impl From<Line> for LRComLine {
    fn from(line: Line) -> LRComLine {
        LRComLine {
            id: NEXT_LINE_ID.fetch_add(1, Ordering::Relaxed),
            line_type: line.borrow().line_type.into(),
            x1: line.ends.0.location.0,
            y1: line.ends.0.location.1,
            x2: line.ends.1.location.0,
            y2: line.ends.1.location.1,
            flipped: line.flipped,
            left_extended: line.ends.0.extended,
            right_extended: line.ends.1.extended,
        }
    }
}

impl From<&LRComLine> for Line {
    fn from(line: &LRComLine) -> Line {
        Line::builder()
            .point(line.x1, line.y1)
            .extended(line.left_extended)
            .point(line.x2, line.y2)
            .extended(line.right_extended)
            .flipped(line.flipped)
            .line_type(line.line_type.into())
            .build()
    }
}

impl From<&Entity> for Result<LRComEntity> {
    fn from(entity: &Entity) -> Result<LRComEntity> {
        let loc = entity
            .points
            .get(&PointIndex::SledPeg)
            .context("must be a boshsled to serialize to track.json")?;
        Ok(LRComEntity {
            start_position: loc.location.into(),
            start_velocity: (loc.previous_location - loc.location).into(),
            remountable: false,
        })
    }
}

impl From<Entity> for Result<LRComEntity> {
    fn from(entity: Entity) -> Result<LRComEntity> {
        Self::from(&entity)
    }
}

impl From<&LRComEntity> for Entity {
    fn from(entity: &LRComEntity) -> Entity {
        let mut bosh_sled = Entity::default_boshsled();
        bosh_sled.mutate_points(|p| p.location += entity.start_position.into());
        bosh_sled.mutate_points(|p| p.previous_location -= entity.start_velocity.into());

        bosh_sled
    }
}

impl From<Track> for Result<LRComTrack> {
    fn from(track: Track) -> Self {
        Self::from(&track)
    }
}

impl From<&Track> for Result<LRComTrack> {
    fn from(track: &Track) -> Result<LRComTrack> {
        let serdey_entities: Vec<LRComEntity> = track
            .entity_positions_at(0)
            .iter()
            .map(Into::into)
            .collect::<Result<Vec<LRComEntity>>>()?;

        let serdey_lines: Vec<LRComLine> = track
            .all_lines()
            .iter()
            .copied()
            .map(|l| l.into())
            .collect();

        Ok(LRComTrack {
            label: "A Bosh Track".to_string(),
            creator: "".to_string(),
            description: "".to_string(),
            duration: 0,
            version: "".to_string(),
            audio: None,
            start_position: serdey_entities
                .first()
                .map(|e| e.start_position)
                .unwrap_or_else(|| LRComVec2 { x: 0.0, y: 0.0 }),
            riders: serdey_entities,
            lines: serdey_lines,
        })
    }
}
