#![feature(proc_macro_hygiene, decl_macro)]

use std::fs::File;

#[macro_use] extern crate rocket;
use rocket::{get, routes};
use rocket::response::content;

#[get("/version/latest/area/sweden/product/comp/<year>/<month>/<day>/<filename>")]
fn retrieve(year: String, month: String, day: String, filename: String) -> Option<content::Plain<File>> {
    let file = format!("{year}/{month}/{day}/{filename}", year = year, month = month, day = day, filename = filename);
    println!("{}", file);
    File::open(&file).map(|f| content::Plain(f)).ok()
}

#[catch(404)]
fn not_found(req: &rocket::Request) -> content::Html<String> {
    content::Html(format!("<p>File {} not found.</p>", req.uri()))
}

fn main() {
    let r = rocket::ignite()
        .mount("/api", routes![retrieve])
        .register(catchers![not_found])
        .launch();

    println!("Wops: {}", r)
}
