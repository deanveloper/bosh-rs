pub mod grid;
mod raw_store;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::game::Line;
    use crate::game::Vector2D;
    use crate::linestore::grid::Grid;

    #[test]
    fn infinite_slope_line() {
        let line = &Line::builder().point(0.0, 0.0).point(0.0, 100.0).build();

        let grid = Grid::new(&vec![*line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0), 1);
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, 50.0), 1);
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, -10.0), 1);
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(30.0, 110.0), 1);
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, 150.0), 1);
        assert_eq!(nearby, Vec::<&Line>::new());

        let nearby = grid.lines_near(Vector2D(0.0, -30.0), 1);
        assert_eq!(nearby, Vec::<&Line>::new());
    }

    #[test]
    fn zero_slope_line() {
        let line = &Line::builder().point(0.0, 0.0).point(100.0, 0.0).build();

        let grid = Grid::new(&vec![*line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0), 1);
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(50.0, 0.0), 1);
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(-10.0, 0.0), 1);
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(110.0, 30.0), 1);
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(150.0, 0.0), 1);
        assert_eq!(nearby, Vec::<&Line>::new());

        let nearby = grid.lines_near(Vector2D(-30.0, 0.0), 1);
        assert_eq!(nearby, Vec::<&Line>::new());
    }

    #[test]
    fn positive_slope_line() {
        let line = &Line::builder()
            .point(45.124, 98.348952734)
            .point(435.47457, 348.3489237)
            .build();

        let grid = Grid::new(&vec![*line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0), 1);
        assert_eq!(nearby, Vec::<&Line>::new());

        let nearby = grid.lines_near(Vector2D(100.0, 125.0), 1);
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(400.0, 320.0), 1);
        assert_eq!(nearby, vec![line]);
    }

    #[test]
    fn negative_slope_line() {
        let line = &Line::builder()
            .point(45.124, 348.3489237)
            .point(435.47457, 98.348952734)
            .build();

        let grid = Grid::new(&vec![*line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0), 1);
        assert_eq!(nearby, Vec::<&Line>::new());

        let nearby = grid.lines_near(Vector2D(200.0, 205.0), 1);
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(400.0, 100.0), 1);
        assert_eq!(nearby, vec![line]);
    }

    #[test]
    fn has_negative_x() {
        let line = &Line::builder()
            .point(-100.0, 0.0)
            .point(-10.0, 50.0)
            .build();

        let grid = Grid::new(&vec![*line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0), 1);
        assert_eq!(nearby, Vec::<&Line>::new());

        let nearby = grid.lines_near(Vector2D(-100.0, 5.0), 1);
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(-50.0, 25.0), 1);
        assert_eq!(nearby, vec![line]);
    }

    #[test]
    fn has_negative_y() {
        let line = &Line::builder()
            .point(0.0, -100.0)
            .point(150.0, 50.0)
            .build();

        let grid = Grid::new(&vec![*line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0), 1);
        assert_eq!(nearby, Vec::<&Line>::new());

        let nearby = grid.lines_near(Vector2D(25.0, -40.0), 1);
        assert_eq!(nearby, vec![line]);
    }

    #[test]
    fn has_negative_y_and_negative_slope() {
        let line = &Line::builder()
            .point(0.0, -100.0)
            .point(50.0, -250.0)
            .build();

        let grid = Grid::new(&vec![*line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0), 1);
        assert_eq!(nearby, Vec::<&Line>::new());

        let nearby = grid.lines_near(Vector2D(25.0, -175.0), 1);
        assert_eq!(nearby, vec![line]);
    }

    #[test]
    fn multiple_lines() {
        let line1 = Line::builder().point(0.0, 0.0).point(100.0, 0.0).build();
        let line2 = Line::builder().point(1.0, 0.0).point(100.0, 0.0).build();
        let line3 = Line::builder().point(2.0, 0.0).point(100.0, 0.0).build();
        let far_line = Line::builder()
            .point(0.0, 1000.0)
            .point(100.0, 1000.0)
            .build();
        let grid = Grid::new(&vec![line1, line2, line3, far_line]);

        let lines = grid.lines_near(Vector2D(50.0, 0.0), 1);
        assert_eq!(
            HashSet::from_iter(lines),
            HashSet::from([&line1, &line2, &line3])
        );
    }

    #[test]
    fn multiple_lines_with_remove() {
        let line1 = Line::builder().point(0.0, 0.0).point(100.0, 0.0).build();
        let line2 = Line::builder().point(1.0, 0.0).point(100.0, 0.0).build();
        let line3 = Line::builder().point(2.0, 0.0).point(100.0, 0.0).build();
        let far_line = Line::builder()
            .point(0.0, 1000.0)
            .point(100.0, 1000.0)
            .build();

        let mut grid = Grid::new(&vec![line1, line2, line3, far_line]);

        grid.remove_line(&line2);

        let lines = grid.lines_near(Vector2D(50.0, 0.0), 1);
        eprintln!("{:#?}", grid);
        assert_eq!(HashSet::from_iter(lines), HashSet::from([&line1, &line3]));
    }

    #[test]
    fn multiple_lines_with_remove_last() {
        let line1 = Line::builder().point(0.0, 0.0).point(100.0, 0.0).build();
        let line2 = Line::builder().point(1.0, 0.0).point(100.0, 0.0).build();
        let line3 = Line::builder().point(2.0, 0.0).point(100.0, 0.0).build();
        let far_line = Line::builder()
            .point(0.0, 1000.0)
            .point(100.0, 1000.0)
            .build();

        let mut grid = Grid::new(&vec![line1, line2, line3, far_line]);

        grid.remove_line(&far_line);

        let lines = grid.lines_near(Vector2D(50.0, 0.0), 1);
        assert_eq!(
            HashSet::from_iter(lines),
            HashSet::from([&line1, &line2, &line3])
        );
    }

    #[test]
    fn all_lines_duplicates() {
        let line1 = Line::builder().point(0.0, 0.0).point(100.0, 0.0).build();
        let line2 = Line::builder().point(1.0, 0.0).point(100.0, 0.0).build();
        let line3 = Line::builder().point(2.0, 0.0).point(100.0, 0.0).build();
        let line4 = Line::builder().point(2.0, 0.0).point(100.0, 0.0).build();
        let far_line = Line::builder()
            .point(0.0, 1000.0)
            .point(100.0, 1000.0)
            .build();

        let grid = Grid::new(&vec![line1, line2, line3, line4, far_line]);

        let lines = grid.all_lines();
        assert_eq!(lines, &vec![line1, line2, line3, line4, far_line]);
    }
}
