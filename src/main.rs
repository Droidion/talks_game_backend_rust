extern crate talks_game_backend_rust;
extern crate dotenv;

use talks_game_backend_rust::start_server;
use talks_game_backend_rust::auth::generate_hash;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    println!("{}", generate_hash("mypassword"));
    start_server();
}
