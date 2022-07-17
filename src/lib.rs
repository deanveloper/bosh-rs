pub mod bosh;
pub mod frame;
pub mod line;
pub mod physics;
pub mod track;
pub mod vector;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
