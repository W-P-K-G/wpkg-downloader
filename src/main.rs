
mod utils;
mod versions;
mod updater;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    updater::update("").await.expect("O kurwa zjebało się");
}