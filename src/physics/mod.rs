pub mod advance_frame;
pub mod bone_physics;
pub mod entity_physics;
pub mod line_physics;

#[cfg(test)]
mod tests {
    use crate::game::Track;
    use crate::game::Vector2D;
    use crate::game::{Line, LineType};
    use crate::physics::entity_physics::PhysicsEntity;
    use crate::physics::line_physics::PhysicsPoint;
    use crate::rider::{Bosh, BoshSled, Entity, PointIndex, StandardBone};
    use std::collections::HashMap;

    fn avg_position(bosh_sled: &BoshSled) -> Vector2D {
        let bosh_sum: Vector2D = bosh_sled.bosh.points.values().map(|p| p.location).sum();
        bosh_sum / bosh_sled.bosh.points.len() as f64
    }

    fn avg_velocity(bosh_sled: &BoshSled) -> Vector2D {
        let bosh_sum: Vector2D = bosh_sled
            .bosh
            .points
            .values()
            .map(|p| (p.location - p.previous_location))
            .sum();
        bosh_sum / bosh_sled.bosh.points.len() as f64
    }

    #[test]
    fn update_bones_contract() {
        let bosh = Bosh {
            points: HashMap::from([
                (
                    PointIndex::BoshShoulder,
                    PhysicsPoint {
                        previous_location: Default::default(),
                        location: Vector2D(10.0, 0.0),
                        friction: 0.0,
                    },
                ),
                (
                    PointIndex::BoshButt,
                    PhysicsPoint {
                        previous_location: Default::default(),
                        location: Vector2D(20.0, 0.0),
                        friction: 0.0,
                    },
                ),
            ]),
            standard_bones: vec![StandardBone {
                p1: PointIndex::BoshShoulder,
                p2: PointIndex::BoshButt,
                resting_length: 5.0,
            }],
            repel_bones: vec![],
        };

        let bosh = bosh.apply_all_bones().unwrap_same();

        assert_eq!(
            bosh.points,
            HashMap::from([
                (
                    PointIndex::BoshShoulder,
                    PhysicsPoint {
                        previous_location: Default::default(),
                        location: Vector2D(12.5, 0.0),
                        friction: 0.0,
                    },
                ),
                (
                    PointIndex::BoshButt,
                    PhysicsPoint {
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
        let bosh = Bosh {
            points: HashMap::from([
                (
                    PointIndex::BoshShoulder,
                    PhysicsPoint {
                        previous_location: Default::default(),
                        location: Vector2D(10.0, 0.0),
                        friction: 0.0,
                    },
                ),
                (
                    PointIndex::BoshButt,
                    PhysicsPoint {
                        previous_location: Default::default(),
                        location: Vector2D(13.0, 0.0),
                        friction: 0.0,
                    },
                ),
            ]),
            standard_bones: vec![StandardBone {
                p1: PointIndex::BoshShoulder,
                p2: PointIndex::BoshButt,
                resting_length: 5.0,
            }],
            repel_bones: vec![],
        };

        let bosh = bosh.apply_all_bones().unwrap_same();

        assert_eq!(
            bosh.points,
            HashMap::from([
                (
                    PointIndex::BoshShoulder,
                    PhysicsPoint {
                        previous_location: Default::default(),
                        location: Vector2D(9.0, 0.0),
                        friction: 0.0,
                    },
                ),
                (
                    PointIndex::BoshButt,
                    PhysicsPoint {
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
        let mut point = PhysicsPoint {
            previous_location: Vector2D(10.23, 30.0),
            location: Vector2D(10.23, 30.2345345),
            friction: 0.0,
        };
        let line = Line {
            flipped: false,
            line_type: LineType::Normal,
            ends: (Vector2D(0.0, 25.0), Vector2D(100.0, 25.0)),
        };

        point.apply_gravity_wells(&Track::new(&[], &vec![line]));

        assert_eq!(point.location, Vector2D(10.23, 25.0))
    }

    #[test]
    fn rider_physics_bosh_sled_at_rest() {
        let original_bosh_sled = BoshSled::default();
        let new_bosh_sled = original_bosh_sled.clone();
        let new_bosh_sled = new_bosh_sled.apply_all_bones().unwrap_same();

        assert_eq!(
            original_bosh_sled, new_bosh_sled,
            "should be equal: {:?}, {:?}",
            original_bosh_sled, new_bosh_sled,
        );
    }

    #[test]
    fn rider_physics_bosh_falling() {
        let mut bosh_sled = BoshSled::default();
        let track = Track::new(&vec![Entity::BoshSled(Default::default())], &vec![]);

        bosh_sled.mutate_points(|p| p.previous_location -= Vector2D(0.4, 0.0));

        for _ in 0..100 {
            bosh_sled = bosh_sled.apply_all_physics_ez(&track).unwrap_same();
        }
    }

    #[test]
    fn rider_physics_bosh_with_line() {
        let mut bosh_sled = BoshSled::default();
        let track = Track::new(
            &vec![Entity::BoshSled(Default::default())],
            &vec![Line {
                flipped: false,
                line_type: LineType::Normal,
                ends: (Vector2D(0.0, 5.0), Vector2D(30.0, 20.0)),
            }],
        );

        bosh_sled.mutate_points(|p| p.previous_location -= Vector2D(0.4, 0.0));

        for _ in 0..100 {
            bosh_sled = bosh_sled.apply_all_physics_ez(&track).unwrap_same();
        }

        eprintln!("{:?}", avg_position(&bosh_sled));
        eprintln!("{:?}", avg_velocity(&bosh_sled));
    }
}
