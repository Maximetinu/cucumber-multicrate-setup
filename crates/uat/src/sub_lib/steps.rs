use cucumber::*;

use super::context::AnimalWorld;

// I shouldn't need to make these functions public, but I'm trying to make them public
// so cucumber doesn't skip them. Even so, they are skipped

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
