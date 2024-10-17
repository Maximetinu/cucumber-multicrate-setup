use sub_lib::Cat;

use cucumber::*;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TestContext {
    pub(crate) cat: Cat,
}

// This runs before everything else, so you can setup things here.
#[tokio::main]
async fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)

    let _cat = Cat::default();

    TestContext::cucumber()
        .run_and_exit("tests/features/")
        .await;
}

mod steps;
