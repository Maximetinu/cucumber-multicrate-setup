use cucumber::*;

use super::context::AnimalWorld;

// Steps are defined with `given`, `when` and `then` attributes.
#[given("a hungry cat")]
fn hungry_cat(world: &mut AnimalWorld) {
    world.cat.hungry = true;
}

#[when("I feed the cat")]
fn feed_cat(world: &mut AnimalWorld) {
    world.cat.feed();
}

#[then("the cat is not hungry")]
fn cat_is_fed(world: &mut AnimalWorld) {
    assert!(!world.cat.hungry);
}
