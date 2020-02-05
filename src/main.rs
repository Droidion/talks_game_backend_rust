extern crate talks_game_backend_rust;
use talks_game_backend_rust::start_server;
use talks_game_backend_rust::auth::generate_hash;

fn main() {
    println!("{}", generate_hash("mypassword"));
    start_server();
}
