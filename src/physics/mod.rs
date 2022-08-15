pub mod advance_frame;
pub mod bone_physics;
pub mod entity_physics;
pub mod line_physics;

#[cfg(test)]
mod tests {
    use crate::game::Track;
    use crate::game::Vector2D;
    use crate::game::{Line, LineType};
    use crate::physics::line_physics::apply_gravity_wells;
    use crate::rider::{BoneStruct, BoneType, EntityPoint, EntityStruct, PointIndex};
    use std::collections::HashMap;

    fn _avg_position(entity: &EntityStruct) -> Vector2D {
        let bosh_sum: Vector2D = entity.points.values().map(|p| p.location).sum();
        bosh_sum / entity.points.len() as f64
    }

    fn avg_velocity(entity: &EntityStruct) -> Vector2D {
        let bosh_sum: Vector2D = entity
            .points
            .values()
            .map(|p| (p.location - p.previous_location))
            .sum();
        bosh_sum / entity.points.len() as f64
    }

    #[test]
    fn update_bones_contract() {
        let bosh = EntityStruct {
            points: HashMap::from([
                (
                    PointIndex::BoshShoulder,
                    EntityPoint {
                        previous_location: Default::default(),
                        location: Vector2D(10.0, 0.0),
                        friction: 0.0,
                    },
                ),
                (
                    PointIndex::BoshButt,
                    EntityPoint {
                        previous_location: Default::default(),
                        location: Vector2D(20.0, 0.0),
                        friction: 0.0,
                    },
                ),
            ]),
            bones: vec![BoneStruct {
                p1: PointIndex::BoshShoulder,
                p2: PointIndex::BoshButt,
                resting_length: 5.0,
                bone_type: BoneType::Normal,
            }],
            joints: vec![],
        };

        let bosh = bosh.apply_bones().unwrap_same();

        assert_eq!(
            bosh.points,
            HashMap::from([
                (
                    PointIndex::BoshShoulder,
                    EntityPoint {
                        previous_location: Default::default(),
                        location: Vector2D(12.5, 0.0),
                        friction: 0.0,
                    },
                ),
                (
                    PointIndex::BoshButt,
                    EntityPoint {
                        previous_location: Default::default(),
                        location: Vector2D(17.5, 0.0),
                        friction: 0.0,
                    },
                )
            ])
        )
    }

    #[test]
    fn update_bones_expand() {
        let bosh = EntityStruct {
            points: HashMap::from([
                (
                    PointIndex::BoshShoulder,
                    EntityPoint {
                        previous_location: Default::default(),
                        location: Vector2D(10.0, 0.0),
                        friction: 0.0,
                    },
                ),
                (
                    PointIndex::BoshButt,
                    EntityPoint {
                        previous_location: Default::default(),
                        location: Vector2D(13.0, 0.0),
                        friction: 0.0,
                    },
                ),
            ]),
            bones: vec![BoneStruct {
                p1: PointIndex::BoshShoulder,
                p2: PointIndex::BoshButt,
                resting_length: 5.0,
                bone_type: BoneType::Normal,
            }],
            joints: vec![],
        };

        let bosh = bosh.apply_bones().unwrap_same();

        assert_eq!(
            bosh.points,
            HashMap::from([
                (
                    PointIndex::BoshShoulder,
                    EntityPoint {
                        previous_location: Default::default(),
                        location: Vector2D(9.0, 0.0),
                        friction: 0.0,
                    },
                ),
                (
                    PointIndex::BoshButt,
                    EntityPoint {
                        previous_location: Default::default(),
                        location: Vector2D(14.0, 0.0),
                        friction: 0.0,
                    },
                )
            ])
        )
    }

    #[test]
    fn update_gravity_wells_flat() {
        let mut point = EntityPoint {
            previous_location: Vector2D(10.23, 30.0),
            location: Vector2D(10.23, 30.2345345),
            friction: 0.0,
        };
        let line = Line {
            flipped: false,
            line_type: LineType::Normal,
            ends: (Vector2D(0.0, 25.0), Vector2D(100.0, 25.0)),
        };

        apply_gravity_wells(&mut point, &Track::new(&[], &vec![line]));

        assert_eq!(point.location, Vector2D(10.23, 25.0))
    }

    #[test]
    fn rider_physics_bosh_sled_at_rest() {
        let original_bosh_sled = EntityStruct::default_boshsled();
        let new_bosh_sled = original_bosh_sled.clone();
        let new_bosh_sled = new_bosh_sled.apply_bones().unwrap_same();

        assert_eq!(
            original_bosh_sled, new_bosh_sled,
            "should be equal: {:?}, {:?}",
            original_bosh_sled, new_bosh_sled,
        );
    }

    #[test]
    fn rider_physics_bosh_falling() {
        let mut bosh_sled = EntityStruct::default_boshsled();
        bosh_sled.mutate_points(|p| p.previous_location -= Vector2D(0.4, 0.0));

        let track = Track::new(&vec![bosh_sled], &vec![]);

        let entities = track.entity_positions_at(100);
        assert_eq!(1, entities.len(), "bosh broke!");

        let x_velocity = avg_velocity(entities.first().unwrap()).0;

        assert!(x_velocity < 0.400001, "x velocity was > 0.4");
        assert!(x_velocity > 0.399999, "x velocity was < 0.4");
    }

    #[test]
    fn rider_physics_bosh_with_line() {
        let mut bosh_sled = EntityStruct::default_boshsled();
        bosh_sled.mutate_points(|p| p.previous_location -= Vector2D(0.4, 0.0));
        let track = Track::new(
            &vec![bosh_sled],
            &vec![Line {
                flipped: false,
                line_type: LineType::Normal,
                ends: (Vector2D(0.0, 5.0), Vector2D(30.0, 20.0)),
            }],
        );

        let entities = track.entity_positions_at(100);
        assert_eq!(1, entities.len(), "bosh broke!");

        let x_velocity = avg_velocity(entities.first().unwrap()).0;

        assert!(x_velocity > 1.0, "should have significant x velocity");
    }
}
