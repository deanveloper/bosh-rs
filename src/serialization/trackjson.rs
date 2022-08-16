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
pub struct SerdeyTrack {
    label: String,
    creator: String,
    description: String,
    duration: u64,
    version: String,
    audio: Option<()>,
    #[serde(alias = "startPosition")]
    start_position: SerdeyVec2,
    riders: Vec<SerdeyEntity>,
    lines: Vec<SerdeyLine>,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct SerdeyEntity {
    #[serde(alias = "startPosition")]
    start_position: SerdeyVec2,
    #[serde(alias = "startVelocity")]
    start_velocity: SerdeyVec2,
    remountable: bool,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct SerdeyVec2 {
    x: f64,
    y: f64,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct SerdeyLine {
    id: u64,
    r#type: SerdeyLineType,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    flipped: bool,
}

#[derive(Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SerdeyLineType {
    Normal = 0,
    Accelerate = 1,
    Scenery = 2,
}

impl<V: Borrow<SerdeyVec2>> From<V> for Vector2D {
    fn from(vector: V) -> Vector2D {
        let vector = vector.borrow();
        Vector2D(vector.x, vector.y)
    }
}

impl<V: Borrow<Vector2D>> From<V> for SerdeyVec2 {
    fn from(vector: V) -> SerdeyVec2 {
        let vector = vector.borrow();
        SerdeyVec2 {
            x: vector.0,
            y: vector.1,
        }
    }
}

impl<LT: Borrow<LineType>> From<LT> for SerdeyLineType {
    fn from(line_type: LT) -> SerdeyLineType {
        match line_type.borrow() {
            LineType::Normal => SerdeyLineType::Normal,
            LineType::Accelerate { .. } => SerdeyLineType::Accelerate,
            LineType::Scenery => SerdeyLineType::Scenery,
        }
    }
}

impl<LT: Borrow<SerdeyLineType>> From<LT> for LineType {
    fn from(line_type: LT) -> LineType {
        match line_type.borrow() {
            SerdeyLineType::Normal => LineType::Normal,
            SerdeyLineType::Accelerate => LineType::Accelerate { amount: 1 },
            SerdeyLineType::Scenery => LineType::Scenery,
        }
    }
}

impl<L: Borrow<Line>> From<L> for SerdeyLine {
    fn from(line: L) -> SerdeyLine {
        let line = line.borrow();
        SerdeyLine {
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

impl From<&SerdeyLine> for Line {
    fn from(line: &SerdeyLine) -> Line {
        Line {
            flipped: line.flipped,
            line_type: (&line.r#type).into(),
            ends: (Vector2D(line.x1, line.y1), Vector2D(line.x2, line.y2)),
        }
    }
}

impl From<&Entity> for Result<SerdeyEntity> {
    fn from(entity: &Entity) -> Result<SerdeyEntity> {
        let loc = entity
            .points
            .get(&PointIndex::SledPeg)
            .context("must be a boshsled to serialize to track.json")?;
        Ok(SerdeyEntity {
            start_position: loc.location.into(),
            start_velocity: (loc.previous_location - loc.location).into(),
            remountable: false,
        })
    }
}

impl From<Entity> for Result<SerdeyEntity> {
    fn from(entity: Entity) -> Result<SerdeyEntity> {
        Self::from(&entity)
    }
}

impl<E: Borrow<SerdeyEntity>> From<E> for Entity {
    fn from(entity: E) -> Entity {
        let entity = entity.borrow();

        let mut bosh_sled = Entity::default_boshsled();
        bosh_sled.mutate_points(|p| p.location += entity.start_position.into());
        bosh_sled.mutate_points(|p| p.previous_location -= entity.start_velocity.into());

        bosh_sled
    }
}

impl From<Track> for Result<SerdeyTrack> {
    fn from(track: Track) -> Self {
        Self::from(&track)
    }
}

impl From<&Track> for Result<SerdeyTrack> {
    fn from(track: &Track) -> Result<SerdeyTrack> {
        let serdey_entities: Vec<SerdeyEntity> = track
            .entity_positions_at(0)
            .iter()
            .map(|entity| entity.into())
            .collect::<Result<Vec<SerdeyEntity>>>()?;

        let serdey_lines: Vec<SerdeyLine> = track.all_lines().iter().map(|l| l.into()).collect();

        Ok(SerdeyTrack {
            label: "A Bosh Track".to_string(),
            creator: "".to_string(),
            description: "".to_string(),
            duration: 0,
            version: "".to_string(),
            audio: None,
            start_position: serdey_entities
                .first()
                .map(|e| e.start_position)
                .unwrap_or_else(|| SerdeyVec2 { x: 0.0, y: 0.0 }),
            riders: serdey_entities,
            lines: serdey_lines,
        })
    }
}
