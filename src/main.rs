#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Carlos"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
