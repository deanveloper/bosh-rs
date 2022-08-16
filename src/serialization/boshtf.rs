use std::collections::HashMap;

use crate::rider::{Entity, PointIndex};
use crate::{Line, Track, Vector2D};
use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdeyTrack {
    pub entites: Vec<SerdeyEntity>,
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SerdeyEntityType {
    Bosh,
    Sled,
    BoshSled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdeyEntity {
    #[serde(alias = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<SerdeyEntityType>,
    #[serde(default)]
    pub points: HashMap<String, (f64, f64)>,
}

impl<E: From<anyhow::Error>> From<&SerdeyTrack> for Result<Track, E> {
    fn from(track: &SerdeyTrack) -> Result<Track, E> {
        Ok(Track::new(
            track
                .entites
                .iter()
                .map(|se| se.into())
                .collect::<Result<Vec<Entity>, E>>()?,
            &track.lines,
        ))
    }
}

impl From<&Track> for SerdeyTrack {
    fn from(track: &Track) -> Self {
        SerdeyTrack {
            lines: track.all_lines().clone(),
            entites: track
                .entity_positions_at(0)
                .iter()
                .map(|entity| entity.into())
                .collect(),
        }
    }
}

impl From<&Entity> for SerdeyEntity {
    fn from(entity: &Entity) -> SerdeyEntity {
        let points = &entity.points;

        let points_serialized: HashMap<String, (f64, f64)> = points
            .iter()
            .map(|(idx, point)| {
                let loc = point.location;

                (point_index_to_string(idx), (loc.0, loc.1))
            })
            .collect();

        SerdeyEntity {
            entity_type: entity_type_from_entity(entity),
            points: points_serialized,
        }
    }
}

impl<E: From<anyhow::Error>> From<&SerdeyEntity> for Result<Entity, E> {
    fn from(entity: &SerdeyEntity) -> Result<Entity, E> {
        match entity.entity_type.context("entity type not provided")? {
            SerdeyEntityType::Bosh => {
                let mut bosh = Entity::default_bosh();
                for point in &entity.points {
                    bosh.point_at_mut(string_to_point_index(point.0)?).location =
                        Vector2D(point.1 .0, point.1 .1);
                }
                Ok(bosh)
            }
            SerdeyEntityType::Sled => {
                let mut sled = Entity::default_sled();
                for point in &entity.points {
                    sled.point_at_mut(string_to_point_index(point.0)?).location =
                        Vector2D(point.1 .0, point.1 .1);
                }
                Ok(sled)
            }
            SerdeyEntityType::BoshSled => {
                let mut bosh_sled = Entity::default_boshsled();

                for point in &entity.points {
                    bosh_sled
                        .point_at_mut(string_to_point_index(point.0)?)
                        .location = Vector2D(point.1 .0, point.1 .1);
                }
                Ok(bosh_sled)
            }
        }
    }
}

fn entity_type_from_entity(entity: &Entity) -> Option<SerdeyEntityType> {
    if entity.is_bosh_sled() {
        Some(SerdeyEntityType::BoshSled)
    } else if entity.is_bosh() {
        Some(SerdeyEntityType::Bosh)
    } else if entity.is_sled() {
        Some(SerdeyEntityType::Sled)
    } else {
        None
    }
}

fn string_to_point_index(s: &str) -> Result<PointIndex, anyhow::Error> {
    match s {
        "BoshLeftFoot" => Ok(PointIndex::BoshLeftFoot),
        "BoshRightFoot" => Ok(PointIndex::BoshRightFoot),
        "BoshLeftHand" => Ok(PointIndex::BoshLeftHand),
        "BoshRightHand" => Ok(PointIndex::BoshRightHand),
        "BoshShoulder" => Ok(PointIndex::BoshShoulder),
        "BoshButt" => Ok(PointIndex::BoshButt),
        "SledPeg" => Ok(PointIndex::SledPeg),
        "SledTail" => Ok(PointIndex::SledTail),
        "SledNose" => Ok(PointIndex::SledNose),
        "SledRope" => Ok(PointIndex::SledRope),
        _ => Err(anyhow!("")),
    }
}

fn point_index_to_string(idx: &PointIndex) -> String {
    match idx {
        PointIndex::BoshLeftFoot => "BoshLeftFoot",
        PointIndex::BoshRightFoot => "BoshRightFoot",
        PointIndex::BoshLeftHand => "BoshLeftHand",
        PointIndex::BoshRightHand => "BoshRightHand",
        PointIndex::BoshShoulder => "BoshShoulder",
        PointIndex::BoshButt => "BoshButt",
        PointIndex::SledPeg => "SledPeg",
        PointIndex::SledTail => "SledTail",
        PointIndex::SledNose => "SledNose",
        PointIndex::SledRope => "SledRope",
    }
    .to_owned()
}
