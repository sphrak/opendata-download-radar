#![feature(proc_macro_hygiene, decl_macro)]

use std::fs::File;
use std::io::Read;
use std::fs::create_dir_all;
use std::io::copy;

//  Get a url
//  Check if its in cache, on disk if it is we serve that
//  otherwise we fetch from SMHI
//  colorize it via libtiff and convert to png
//  return a png file


#[macro_use] extern crate rocket;
use rocket::{get, routes};
use rocket::response::content;
use rocket::http::ContentType;

const API_URL: &str = "https://opendata-download-radar.smhi.se/api";
const SUB_DIRECTORY: &str = "version/latest/area/sweden/product/comp";

fn mkdir(path: String) -> std::io::Result<()> {
    create_dir_all(&path)?;
    Ok(())
}

#[get("/version/latest/area/sweden/product/comp/<year>/<month>/<day>/<filename>")]
fn retrieve(year: String, month: String, day: String, filename: String) -> Option<content::Content<File>> {
    let file: String = format!("{year}/{month}/{day}/{filename}", year = year, month = month, day = day, filename = filename);
    let path: String = format!("{year}/{month}/{day}", year = year, month = month, day = day);
    println!("{}", file);

    let url: String = format!("{}/{}/{}", API_URL, SUB_DIRECTORY, file);
    println!("GET {}", url);

    /**
     *  Create directory if we know it does not exist.
     */
    mkdir(path);

    //let res = client.get(req).send();
    let mut resp = reqwest::get(&url).expect("Request failed.");
    // file will have same filename, move into path ${path}
    let mut dest = File::create(&file).expect("Failed writing file");


    copy(&mut resp, &mut dest);

    // we return this
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
