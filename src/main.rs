#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/version/latest/area/sweden/product/comp/<year>/<month>/<day>")]
fn api(year: String, month: String, day: String) -> String {
    format!("Current path: /version/latest/area/sweden/product/comp/{}/{}/{}", year, month, day)
}

fn main() {
    rocket::ignite().mount("/api", routes![api]).launch();
}
