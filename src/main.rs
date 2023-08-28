mod server {
    pub mod about; // Assuming about.rs is in the 'server' subdirectory
}

use server::about;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str { // IMMUTABLE
    "Hello, world!"
}

#[get("/world")]              
fn world() -> String {  // MUTABLE
    String::from("hello, world!")
}

#[launch]   
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, world]).mount(about::BASE, about::routes())
}