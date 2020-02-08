extern crate dotenv;
extern crate talks_game_backend_rust;

use dotenv::dotenv;
use talks_game_backend_rust::start_server;
use talks_game_backend_rust::db::get_users;
use talks_game_backend_rust::auth::generate_hash;

fn main() {
    dotenv().ok();
    println!("{}", generate_hash("mypassword"));
    get_users();
    start_server();
}
