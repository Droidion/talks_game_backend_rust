

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn start() {
    println!("Starting the server...");
    rocket::ignite().mount("/", routes![index]).launch();
}
