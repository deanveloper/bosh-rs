pub mod advance_frame;
pub mod bone_physics;
pub mod line_physics;
pub mod rider_physics;

#[cfg(test)]
mod tests {
    use crate::game::Track;
    use crate::game::Vector2D;
    use crate::game::{Line, LineType};
    use crate::physics::line_physics::PhysicsPoint;
    use crate::physics::rider_physics::PhysicsEntity;
    use crate::rider::{BoshSled, Entity};

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
        let mut falling_bosh_sled = BoshSled::default();
        let empty_track = Track::new(&vec![Entity::BoshSled(Default::default())], &vec![]);
        let _track = Track::new(
            &vec![Entity::BoshSled(Default::default())],
            &vec![Line {
                flipped: false,
                line_type: LineType::Normal,
                ends: (Vector2D(0.0, 5.0), Vector2D(30.0, 5.0)),
            }],
        );

        falling_bosh_sled.mutate_points(|p| p.previous_location -= Vector2D(0.4, 0.0));

        for i in 0..1000 {
            {
                let avg = avg_position(&falling_bosh_sled);
                eprintln!("falling bosh sled at {i}");
                eprintln!("{} {} {}", avg.0, avg.1, avg_velocity(&falling_bosh_sled));
            }
            falling_bosh_sled = falling_bosh_sled
                .apply_all_physics(&empty_track)
                .unwrap_same();
        }

        eprintln!(
            "======== falling bosh\n{:#?}",
            avg_position(&falling_bosh_sled)
        );
    }
}
