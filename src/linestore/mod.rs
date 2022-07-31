pub mod grid;
mod raw_store;

#[cfg(test)]
mod tests {
    use crate::line::{Line, LineType};
    use crate::linestore::grid::Grid;
    use crate::vector::Vector2D;
    use std::collections::HashSet;

    #[test]
    fn infinite_slope_line() {
        let line = Line {
            flipped: false,
            line_type: LineType::Normal,
            ends: (Vector2D(0.0, 0.0), Vector2D(0.0, 100.0)),
        };

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
        let line = Line {
            flipped: false,
            line_type: LineType::Normal,
            ends: (Vector2D(0.0, 0.0), Vector2D(100.0, 0.0)),
        };

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
        let line = Line {
            flipped: false,
            line_type: LineType::Normal,
            ends: (
                Vector2D(45.124, 98.348952734),
                Vector2D(435.47457, 348.3489237),
            ),
        };

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
        let line = Line {
            flipped: false,
            line_type: LineType::Normal,
            ends: (
                Vector2D(45.124, 348.3489237),
                Vector2D(435.47457, 98.348952734),
            ),
        };

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
        let line = Line {
            flipped: false,
            line_type: LineType::Normal,
            ends: (Vector2D(-100.0, 0.0), Vector2D(-10.0, 50.0)),
        };

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
        let line = Line {
            flipped: false,
            line_type: LineType::Normal,
            ends: (Vector2D(0.0, -100.0), Vector2D(150.0, 50.0)),
        };

        let grid = Grid::new(&vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0));
        assert_eq!(nearby, vec![]);

        let nearby = grid.lines_near(Vector2D(25.0, -40.0));
        assert_eq!(nearby, vec![line]);
    }

    #[test]
    fn has_negative_y_and_negative_slope() {
        let line = Line {
            flipped: false,
            line_type: LineType::Normal,
            ends: (Vector2D(0.0, -100.0), Vector2D(50.0, -250.0)),
        };

        let grid = Grid::new(&vec![line]);

        let nearby = grid.lines_near(Vector2D(0.0, 0.0));
        assert_eq!(nearby, vec![]);

        let nearby = grid.lines_near(Vector2D(25.0, -175.0));
        assert_eq!(nearby, vec![line]);
    }
}
