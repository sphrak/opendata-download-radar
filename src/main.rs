#![feature(proc_macro_hygiene, decl_macro)]

use std::fs::File;

//  Get a url
//  Check if its in cache, on disk if it is we serve that
//  otherwise we fetch from SMHI
//  colorize it via libtiff and convert to png
//  return a png file


#[macro_use] extern crate rocket;
use rocket::{get, routes};
use rocket::response::content;
use rocket::http::ContentType;

#[get("/version/latest/area/sweden/product/comp/<year>/<month>/<day>/<filename>")]
fn retrieve(year: String, month: String, day: String, filename: String) -> Option<content::Content<File>> {
    let file = format!("{year}/{month}/{day}/{filename}", year = year, month = month, day = day, filename = filename);
    println!("{}", file);
    File::open(&file).map(|f| content::Content(ContentType::PNG, f)).ok()
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
