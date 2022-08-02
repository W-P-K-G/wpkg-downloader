
mod utils;
mod versions;
mod updater;
mod globals;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    updater::update("").await.expect("O kurwa zjebało się");
}