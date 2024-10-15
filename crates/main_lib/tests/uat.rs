use sub_lib::uat::AnimalWorld;
use cucumber::World;

// This runs before everything else, so you can setup things here.
#[tokio::main]
async fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)

    AnimalWorld::cucumber().run_and_exit("tests/features/").await;
}