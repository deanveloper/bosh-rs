mod line;
mod track;
mod vector;

pub use line::*;
pub use track::*;
pub use vector::*;

#[cfg(test)]
mod test {
    use crate::Vector2D;

    #[test]
    fn test_distance() {
        let vec1 = Vector2D(0.0, 0.0);
        let vec2 = Vector2D(5.0, 0.0);

        assert_eq!(vec1.distance_squared(vec2), 25.0)
    }
}
