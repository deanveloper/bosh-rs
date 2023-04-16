pub mod advance_frame;
pub mod bone_physics;
pub mod entity_physics;
pub mod line_physics;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::game::Line;
    use crate::game::Track;
    use crate::game::Vector2D;
    use crate::physics::line_physics::apply_gravity_wells;
    use crate::rider::{Bone, BoneType, Entity, EntityPoint, PointIndex};
    use crate::LineType;

    fn _avg_position(entity: &Entity) -> Vector2D {
        let bosh_sum: Vector2D = entity.points.values().map(|p| p.location).sum();
        bosh_sum / entity.points.len() as f64
    }

    fn avg_velocity(entity: &Entity) -> Vector2D {
        let bosh_sum: Vector2D = entity
            .points
            .values()
            .map(|p| (p.location - p.previous_location))
            .sum();
        bosh_sum / entity.points.len() as f64
    }

    #[test]
    fn update_bones_contract() {
        let bosh = Entity {
            points: HashMap::from([
                (
                    PointIndex::BoshShoulder,
                    EntityPoint {
                        previous_location: Default::default(),
                        location: Vector2D(10.0, 0.0),
                        momentum: Default::default(),
                        friction: 0.0,
                    },
                ),
                (
                    PointIndex::BoshButt,
                    EntityPoint {
                        previous_location: Default::default(),
                        location: Vector2D(20.0, 0.0),
                        momentum: Default::default(),
                        friction: 0.0,
                    },
                ),
            ]),
            bones: vec![Bone {
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
                        momentum: Default::default(),
                        friction: 0.0,
                    },
                ),
                (
                    PointIndex::BoshButt,
                    EntityPoint {
                        previous_location: Default::default(),
                        location: Vector2D(17.5, 0.0),
                        momentum: Default::default(),
                        friction: 0.0,
                    },
                )
            ])
        )
    }

    #[test]
    fn update_bones_expand() {
        let bosh = Entity {
            points: HashMap::from([
                (
                    PointIndex::BoshShoulder,
                    EntityPoint {
                        previous_location: Default::default(),
                        location: Vector2D(10.0, 0.0),
                        momentum: Default::default(),
                        friction: 0.0,
                    },
                ),
                (
                    PointIndex::BoshButt,
                    EntityPoint {
                        previous_location: Default::default(),
                        location: Vector2D(13.0, 0.0),
                        momentum: Default::default(),
                        friction: 0.0,
                    },
                ),
            ]),
            bones: vec![Bone {
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
                        momentum: Default::default(),
                        friction: 0.0,
                    },
                ),
                (
                    PointIndex::BoshButt,
                    EntityPoint {
                        previous_location: Default::default(),
                        location: Vector2D(14.0, 0.0),
                        momentum: Default::default(),
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
            momentum: Vector2D(0.0, 0.2),
            friction: 0.0,
        };
        let line = Line::builder().point(0.0, 25.0).point(100.0, 25.0).build();

        apply_gravity_wells(&mut point, &Track::new(vec![], vec![line]));

        assert_eq!(point.location, Vector2D(10.23, 25.0))
    }

    #[test]
    fn rider_physics_bosh_sled_at_rest() {
        let original_bosh_sled = Entity::default_boshsled();
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
        let bosh_sled = Entity::default_boshsled();

        let track = Track::new(vec![bosh_sled], vec![]);

        let entities = track.entity_positions_at(100);
        assert_eq!(1, entities.len(), "bosh broke!");

        let x_velocity = avg_velocity(entities.first().unwrap()).0;

        assert!(x_velocity < 0.400001, "x velocity was > 0.4");
        assert!(x_velocity > 0.399999, "x velocity was < 0.4");
    }

    #[test]
    fn rider_physics_bosh_with_line() {
        let bosh_sled = Entity::default_boshsled();
        let track = Track::new(
            vec![bosh_sled],
            vec![Line::builder().point(0.0, 5.0).point(30.0, 20.0).build()],
        );

        let entities = track.entity_positions_at(100);
        assert_eq!(1, entities.len(), "bosh broke!");

        let x_velocity = avg_velocity(entities.first().unwrap()).0;

        assert!(x_velocity > 1.0, "should have significant x velocity");
    }

    #[test]
    fn rider_slingshot() {
        let entity = Entity::default_boshsled();
        let track = Track::new(
            vec![entity],
            vec![Line::builder()
                .point(1.2112666897140032, -3.0419052379903606)
                .point(2.7302375219426875, -2.021219126142812)
                .build()],
        );

        let entities = track.entity_positions_at(10);
        let entity = entities.get(0).unwrap();

        // comparison is to linerider.com's physics
        assert_eq!(
            entity.point_at(PointIndex::SledTail),
            &EntityPoint {
                previous_location: Vector2D(11.862884095165988, -10.089233973375801),
                location: Vector2D(14.127750467596647, -10.151751900395013),
                momentum: Vector2D(2.2295236487517673, -0.3484422284398246),
                friction: 0.0,
            }
        );
    }

    #[test]
    fn rider_accel() {
        let entity = Entity::default_boshsled();
        let track = Track::new(
            vec![entity],
            vec![Line::builder()
                .line_type(LineType::Accelerate { amount: 1 })
                .point(-5.0, 1.0)
                .point(10.0, 1.0)
                .build()],
        );

        let entities = track.entity_positions_at(10);
        let entity = entities.get(0).unwrap();

        // comparison is to linerider.com's physics
        assert_eq!(
            entity.point_at(PointIndex::SledTail),
            &EntityPoint {
                previous_location: Vector2D(8.546038495647734, -17.059432508544703),
                location: Vector2D(10.437748868700394, -17.70589979578289),
                momentum: Vector2D(1.8303232225405575, -0.8753245061408925),
                friction: 0.0,
            }
        );
    }
}
