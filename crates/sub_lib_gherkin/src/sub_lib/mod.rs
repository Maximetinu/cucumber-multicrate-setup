use cucumber::*;
use sub_lib::Cat;

pub mod steps;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct AnimalWorld {
    pub(crate) cat: Cat,
}
