pub mod grid;
mod raw_store;

#[cfg(test)]
mod tests {
    use crate::game::{Line, LineType};
    use crate::linestore::grid::Grid;
    use crate::game::Vector2D;
    use std::collections::HashSet;

    fn make_line(v1: Vector2D, v2: Vector2D) -> Line {
        Line {
            flipped: false,
            line_type: LineType::Normal,
            ends: (v1, v2),
        }
    }

    #[test]
    fn infinite_slope_line() {
        let line = make_line(Vector2D(0.0, 0.0), Vector2D(0.0, 100.0));

        let grid = Grid::new(&vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0));
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, 50.0));
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, -10.0));
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(30.0, 110.0));
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, 150.0));
        assert_eq!(nearby, vec![]);

        let nearby = grid.lines_near(Vector2D(0.0, -30.0));
        assert_eq!(nearby, vec![]);
    }

    #[test]
    fn zero_slope_line() {
        let line = make_line(Vector2D(0.0, 0.0), Vector2D(100.0, 0.0));

        let grid = Grid::new(&vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0));
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(50.0, 0.0));
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(-10.0, 0.0));
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(110.0, 30.0));
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(150.0, 0.0));
        assert_eq!(nearby, vec![]);

        let nearby = grid.lines_near(Vector2D(-30.0, 0.0));
        assert_eq!(nearby, vec![]);
    }

    #[test]
    fn positive_slope_line() {
        let line = make_line(
            Vector2D(45.124, 98.348952734),
            Vector2D(435.47457, 348.3489237),
        );

        let grid = Grid::new(&vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0));
        assert_eq!(nearby, vec![]);

        let nearby = grid.lines_near(Vector2D(100.0, 125.0));
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(400.0, 320.0));
        assert_eq!(nearby, vec![line]);
    }

    #[test]
    fn negative_slope_line() {
        let line = make_line(
            Vector2D(45.124, 348.3489237),
            Vector2D(435.47457, 98.348952734),
        );

        let grid = Grid::new(&vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0));
        assert_eq!(nearby, vec![]);

        let nearby = grid.lines_near(Vector2D(200.0, 205.0));
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(400.0, 100.0));
        assert_eq!(nearby, vec![line]);
    }

    #[test]
    fn has_negative_x() {
        let line = make_line(Vector2D(-100.0, 0.0), Vector2D(-10.0, 50.0));

        let grid = Grid::new(&vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0));
        assert_eq!(nearby, vec![]);

        let nearby = grid.lines_near(Vector2D(-100.0, 5.0));
        assert_eq!(nearby, vec![line]);

        let nearby = grid.lines_near(Vector2D(-50.0, 25.0));
        assert_eq!(nearby, vec![line]);
    }

    #[test]
    fn has_negative_y() {
        let line = make_line(Vector2D(0.0, -100.0), Vector2D(150.0, 50.0));

        let grid = Grid::new(&vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0));
        assert_eq!(nearby, vec![]);

        let nearby = grid.lines_near(Vector2D(25.0, -40.0));
        assert_eq!(nearby, vec![line]);
    }

    #[test]
    fn has_negative_y_and_negative_slope() {
        let line = make_line(Vector2D(0.0, -100.0), Vector2D(50.0, -250.0));

        let grid = Grid::new(&vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0));
        assert_eq!(nearby, vec![]);

        let nearby = grid.lines_near(Vector2D(25.0, -175.0));
        assert_eq!(nearby, vec![line]);
    }

    #[test]
    fn multiple_lines() {
        let line1 = make_line(Vector2D(0.0, 0.0), Vector2D(100.0, 0.0));
        let line2 = make_line(Vector2D(1.0, 0.0), Vector2D(100.0, 0.0));
        let line3 = make_line(Vector2D(2.0, 0.0), Vector2D(100.0, 0.0));
        let far_line = make_line(Vector2D(0.0, 1000.0), Vector2D(100.0, 1000.0));
        let grid = Grid::new(&vec![line1, line2, line3, far_line]);

        let lines = grid.lines_near(Vector2D(50.0, 0.0));
        assert_eq!(
            HashSet::from_iter(lines),
            HashSet::from([line1, line2, line3])
        );
    }

    #[test]
    fn multiple_lines_with_remove() {
        let line1 = make_line(Vector2D(0.0, 0.0), Vector2D(100.0, 0.0));
        let line2 = make_line(Vector2D(1.0, 0.0), Vector2D(100.0, 0.0));
        let line3 = make_line(Vector2D(2.0, 0.0), Vector2D(100.0, 0.0));
        let far_line = make_line(Vector2D(0.0, 1000.0), Vector2D(100.0, 1000.0));

        let mut grid = Grid::new(&vec![line1, line2, line3, far_line]);

        grid.remove_line(line2);

        let lines = grid.lines_near(Vector2D(50.0, 0.0));
        eprintln!("{:#?}", grid);
        assert_eq!(HashSet::from_iter(lines), HashSet::from([line1, line3]));
    }

    #[test]
    fn multiple_lines_with_remove_last() {
        let line1 = make_line(Vector2D(0.0, 0.0), Vector2D(100.0, 0.0));
        let line2 = make_line(Vector2D(1.0, 0.0), Vector2D(100.0, 0.0));
        let line3 = make_line(Vector2D(2.0, 0.0), Vector2D(100.0, 0.0));
        let far_line = make_line(Vector2D(0.0, 1000.0), Vector2D(100.0, 1000.0));

        let mut grid = Grid::new(&vec![line1, line2, line3, far_line]);

        grid.remove_line(far_line);

        let lines = grid.lines_near(Vector2D(50.0, 0.0));
        assert_eq!(
            HashSet::from_iter(lines),
            HashSet::from([line1, line2, line3])
        );
    }

    #[test]
    fn all_lines_duplicates() {
        let line1 = make_line(Vector2D(0.0, 0.0), Vector2D(100.0, 0.0));
        let line2 = make_line(Vector2D(1.0, 0.0), Vector2D(100.0, 0.0));
        let line3 = make_line(Vector2D(2.0, 0.0), Vector2D(100.0, 0.0));
        let line4 = make_line(Vector2D(2.0, 0.0), Vector2D(100.0, 0.0));
        let far_line = make_line(Vector2D(0.0, 1000.0), Vector2D(100.0, 1000.0));

        let grid = Grid::new(&vec![line1, line2, line3, line4, far_line]);

        let lines = grid.all_lines();
        assert_eq!(lines, &vec![line1, line2, line3, line4, far_line]);
    }
}
