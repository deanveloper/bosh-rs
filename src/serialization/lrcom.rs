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
    r#type: LRComLineType,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    flipped: bool,
}

#[derive(Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LRComLineType {
    Normal = 0,
    Accelerate = 1,
    Scenery = 2,
}

impl<V: Borrow<LRComVec2>> From<V> for Vector2D {
    fn from(vector: V) -> Vector2D {
        let vector = vector.borrow();
        Vector2D(vector.x, vector.y)
    }
}

impl<V: Borrow<Vector2D>> From<V> for LRComVec2 {
    fn from(vector: V) -> LRComVec2 {
        let vector = vector.borrow();
        LRComVec2 {
            x: vector.0,
            y: vector.1,
        }
    }
}

impl<LT: Borrow<LineType>> From<LT> for LRComLineType {
    fn from(line_type: LT) -> LRComLineType {
        match line_type.borrow() {
            LineType::Normal => LRComLineType::Normal,
            LineType::Accelerate { .. } => LRComLineType::Accelerate,
            LineType::Scenery => LRComLineType::Scenery,
        }
    }
}

impl<LT: Borrow<LRComLineType>> From<LT> for LineType {
    fn from(line_type: LT) -> LineType {
        match line_type.borrow() {
            LRComLineType::Normal => LineType::Normal,
            LRComLineType::Accelerate => LineType::Accelerate { amount: 1 },
            LRComLineType::Scenery => LineType::Scenery,
        }
    }
}

impl<L: Borrow<Line>> From<L> for LRComLine {
    fn from(line: L) -> LRComLine {
        let line = line.borrow();
        LRComLine {
            id: NEXT_LINE_ID.fetch_add(1, Ordering::Relaxed),
            r#type: line.borrow().line_type.into(),
            x1: line.ends.0 .0,
            y1: line.ends.0 .1,
            x2: line.ends.1 .0,
            y2: line.ends.1 .1,
            flipped: line.flipped,
        }
    }
}

impl From<&LRComLine> for Line {
    fn from(line: &LRComLine) -> Line {
        Line {
            flipped: line.flipped,
            line_type: (&line.r#type).into(),
            ends: (Vector2D(line.x1, line.y1), Vector2D(line.x2, line.y2)),
        }
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

impl<E: Borrow<LRComEntity>> From<E> for Entity {
    fn from(entity: E) -> Entity {
        let entity = entity.borrow();

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
            .map(|entity| entity.into())
            .collect::<Result<Vec<LRComEntity>>>()?;

        let serdey_lines: Vec<LRComLine> = track.all_lines().iter().map(|l| l.into()).collect();

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
