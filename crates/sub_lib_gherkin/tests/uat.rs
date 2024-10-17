use cucumber::World;
use sub_lib::Cat;
use sub_lib_gherkin::sub_lib::AnimalWorld;

// This runs before everything else, so you can setup things here.
#[tokio::main]
async fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)

    let _cat = Cat::default();

    AnimalWorld::cucumber()
        .run_and_exit("tests/features/")
        .await;
}
