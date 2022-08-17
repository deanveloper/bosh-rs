//! Module for deserializing BoshTF, the Bosh Track Format

use std::borrow::Borrow;
use std::collections::HashMap;

use crate::rider::{Entity, PointIndex};
use crate::{Line, Track, Vector2D};
use anyhow::{anyhow, Context, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoshTFTrack {
    pub entites: Vec<BoshTFEntity>,
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BoshTFEntityType {
    Bosh,
    Sled,
    BoshSled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoshTFEntity {
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<BoshTFEntityType>,
    #[serde(default)]
    pub points: HashMap<String, (f64, f64)>,
}

impl<E: From<Error>> From<&BoshTFTrack> for Result<Track, E> {
    fn from(track: &BoshTFTrack) -> Result<Track, E> {
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

impl From<&Track> for BoshTFTrack {
    fn from(track: &Track) -> Self {
        BoshTFTrack {
            lines: track.all_lines().clone(),
            entites: track
                .entity_positions_at(0)
                .iter()
                .map(|entity| entity.into())
                .collect(),
        }
    }
}

impl From<&Entity> for BoshTFEntity {
    fn from(entity: &Entity) -> BoshTFEntity {
        let points = &entity.points;

        let points_serialized: HashMap<String, (f64, f64)> = points
            .iter()
            .map(|(idx, point)| {
                let loc = point.location;

                (idx.into(), (loc.0, loc.1))
            })
            .collect();

        BoshTFEntity {
            entity_type: entity_type_from_entity(entity),
            points: points_serialized,
        }
    }
}

impl<E: From<Error>> From<&BoshTFEntity> for Result<Entity, E> {
    fn from(entity: &BoshTFEntity) -> Result<Entity, E> {
        match entity.entity_type.context("entity type not provided")? {
            BoshTFEntityType::Bosh => {
                let mut bosh = Entity::default_bosh();
                for point in &entity.points {
                    bosh.point_at_mut(point.0.as_str().try_into()?).location =
                        Vector2D(point.1 .0, point.1 .1);
                }
                Ok(bosh)
            }
            BoshTFEntityType::Sled => {
                let mut sled = Entity::default_sled();
                for point in &entity.points {
                    sled.point_at_mut(point.0.as_str().try_into()?).location =
                        Vector2D(point.1 .0, point.1 .1);
                }
                Ok(sled)
            }
            BoshTFEntityType::BoshSled => {
                let mut bosh_sled = Entity::default_boshsled();

                for point in &entity.points {
                    bosh_sled
                        .point_at_mut(point.0.as_str().try_into()?)
                        .location = Vector2D(point.1 .0, point.1 .1);
                }
                Ok(bosh_sled)
            }
        }
    }
}

fn entity_type_from_entity(entity: &Entity) -> Option<BoshTFEntityType> {
    if entity.is_bosh_sled() {
        Some(BoshTFEntityType::BoshSled)
    } else if entity.is_bosh() {
        Some(BoshTFEntityType::Bosh)
    } else if entity.is_sled() {
        Some(BoshTFEntityType::Sled)
    } else {
        None
    }
}

impl TryFrom<&str> for PointIndex {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
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
}

impl From<&PointIndex> for String {
    fn from(idx: &PointIndex) -> String {
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
}
