extern crate core;

mod game;
mod linestore;
pub mod physics;
pub mod rider;
pub mod serialization;

pub use game::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
