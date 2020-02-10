extern crate dotenv;
extern crate talks_game_backend_rust;

use dotenv::dotenv;
use talks_game_backend_rust::start_server;

fn main() {
    dotenv().ok();
    start_server();
}
