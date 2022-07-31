pub mod frame;
pub mod line;
pub mod physics;
pub mod rider;
pub mod track;
pub mod vector;

mod grid;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
