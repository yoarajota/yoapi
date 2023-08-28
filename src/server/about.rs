use rocket::{http::uri::Origin, Route};


pub const BASE: Origin<'static> = uri!("/about");

#[get("/<user>")]
fn about(user: &str) -> String {
    String::from(user)
}

#[get("/me")]
fn about_me() -> String {
    String::from("yoarajota")
}

pub fn routes() -> Vec<Route> {
    routes![about, about_me]
}