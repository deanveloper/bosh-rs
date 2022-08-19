//! Module for deserializing BoshTF, the Bosh Track Format

use serde::{Deserialize, Serialize};

use crate::rider::{Entity, PointIndex};
use crate::{Line, Track, Vector2D};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BoshTFEntity {
    #[serde(rename = "default")]
    Default { starting_position: Vector2D },
    #[serde(rename = "custom")]
    Custom(Entity),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoshTFTrack {
    pub entites: Vec<BoshTFEntity>,
    pub lines: Vec<Line>,
}

impl From<&BoshTFTrack> for Track {
    fn from(track: &BoshTFTrack) -> Track {
        Track::new(
            track.entites.iter().map(Entity::from).collect(),
            track.lines.clone(),
        )
    }
}

impl From<&Track> for BoshTFTrack {
    fn from(track: &Track) -> Self {
        BoshTFTrack {
            lines: track.all_lines().clone(),
            entites: track
                .entity_positions_at(0)
                .iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<&Entity> for BoshTFEntity {
    fn from(entity: &Entity) -> Self {
        let peg = entity.points.get(&PointIndex::SledPeg);
        if matches!(peg, None) {
            return BoshTFEntity::Custom(entity.clone());
        }
        let peg = peg.unwrap();
        let peg_location = peg.location;
        let peg_velocity = peg.previous_location - peg.location;

        let default_boshsled: Entity = Entity::default_boshsled();
        let mut mapped_entity = entity.clone();
        for p in mapped_entity.points.values_mut() {
            let p_velocity = p.previous_location - p.location;
            if p_velocity != peg_velocity {
                return BoshTFEntity::Custom(entity.clone());
            }

            p.location -= peg_location;
            p.previous_location -= peg_location;
        }

        if default_boshsled != mapped_entity {
            BoshTFEntity::Custom(entity.clone())
        } else {
            BoshTFEntity::Default {
                starting_position: peg_location,
            }
        }
    }
}

impl From<&BoshTFEntity> for Entity {
    fn from(entity: &BoshTFEntity) -> Self {
        match entity {
            BoshTFEntity::Default { starting_position } => {
                let mut bosh_sled = Entity::default_boshsled();
                bosh_sled.mutate_points(|point| {
                    point.previous_location += *starting_position;
                    point.location += *starting_position;
                });

                bosh_sled
            }
            BoshTFEntity::Custom(custom) => custom.clone(),
        }
    }
}
