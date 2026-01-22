mod global;
mod level;
mod logger;
mod macros;

pub use global::{info, init};
pub use level::Level;
pub use logger::Logger;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
