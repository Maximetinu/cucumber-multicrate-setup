use cucumber::*;

use crate::TestContext;

// Steps are defined with `given`, `when` and `then` attributes.
#[given("a hungry cat")]
fn hungry_cat(world: &mut TestContext) {
    world.cat.hungry = true;
}

#[when("I feed the cat")]
fn feed_cat(world: &mut TestContext) {
    world.cat.feed();
}

#[then("the cat is not hungry")]
fn cat_is_fed(world: &mut TestContext) {
    assert!(!world.cat.hungry);
}
