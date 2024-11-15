mod policy;
pub use policy::*;
mod statement;
pub use statement::*;
mod effect;
pub use effect::*;
mod resource;
pub use resource::*;
pub mod aws;
pub mod traits;
mod policy_collection;
mod engine;

pub use policy_collection::*;
pub use matches_macro::Matches;
pub use engine::*;

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
