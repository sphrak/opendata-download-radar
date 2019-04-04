#![feature(proc_macro_hygiene, decl_macro)]
use std::fs::File;
use std::fs::create_dir_all;
use std::io::Read;
use std::path::Path;
use std::time::{Duration, Instant};

#[macro_use] extern crate image;
use image::RGB;
use image::load_from_memory_with_format;
use image::save_buffer;
use image::DynamicImage;
use image::GenericImageView;

#[macro_use] extern crate rocket;
use rocket::{get, routes};
use rocket::response::content;
use rocket::http::ContentType;

use reqwest::Response;

const API_URL: &str = "https://opendata-download-radar.smhi.se/api";
const SUB_DIRECTORY: &str = "version/latest/area/sweden/product/comp";

fn mkdir(path: &str) -> std::io::Result<()> {
    create_dir_all(&path)?;
    Ok(())
}

#[get("/version/latest/area/sweden/product/comp/<year>/<month>/<day>/<filename>")]
fn retrieve(year: String, month: String, day: String, filename: String) -> Option<content::Content<File>> {

    let start = Instant::now();
    println!("Start Elapsed: {:?}", start.elapsed());

    let file: String = format!("{year}/{month}/{day}/{filename}", year = year, month = month, day = day, filename = filename);
    let path: String = format!("{year}/{month}/{day}", year = year, month = month, day = day);
    let url: String = format!("{}/{}/{}", API_URL, SUB_DIRECTORY, file);

    /**
     *  Create directory if we know it does not exist.
     */
    println!("Elapsed: {:?}", start.elapsed());
    mkdir(&path);
    println!("Created directory: {:?}", start.elapsed());

    let full_path: &Path = Path::new(&file);

    let mut response: Response = reqwest::get(&url).unwrap();
    println!("After network request: {:?}", start.elapsed());
    let filename_path: &Path = Path::new(&filename);

    match filename_path.file_stem() {
        Some(stem) => {

            let new_file: String = format!("{}/{}.png", path, stem.to_str().unwrap());
            let new_path: &Path = Path::new(&new_file);

            let mut buffer: Vec<u8> = Vec::new();

            response.read_to_end(&mut buffer).unwrap();

            let img: DynamicImage = load_from_memory_with_format(&buffer, image::ImageFormat::TIFF).unwrap();

            // get dimensions
            let (width, height) = img.dimensions();

            // save the buffer as .png file
            println!("Saving: {:?}", new_path);
            img.save(new_path).unwrap();

            println!("Height: {}", height);
            println!("Weight: {}", width);
            for x in 0..width {
                for y in 0..height {

                }
            }
        }
        None => {
            panic!("Error");
        }
    }

    let new_path = Path::new("2019/04/03/radar_1904030005.png");

    println!("Elapsed: {:?}", start.elapsed());
    let f =  File::open(&new_path).map(|f| content::Content(ContentType::PNG, f)).ok();
    let duration = start.elapsed();
    println!("END Elapsed: {:?}", start.elapsed());
    f
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

    println!("Woops: {}", r)
}