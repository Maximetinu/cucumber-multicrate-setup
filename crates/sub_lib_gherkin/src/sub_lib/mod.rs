use cucumber::*;

pub mod context;

use context::AnimalWorld;

// Steps are defined with `given`, `when` and `then` attributes.
#[given("a hungry cat")]
pub fn hungry_cat(world: &mut AnimalWorld) {
    world.cat.hungry = true;
}

#[when("I feed the cat")]
pub fn feed_cat(world: &mut AnimalWorld) {
    world.cat.feed();
}

#[then("the cat is not hungry")]
pub fn cat_is_fed(world: &mut AnimalWorld) {
    assert!(!world.cat.hungry);
}
