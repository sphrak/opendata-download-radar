#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate image;
#[macro_use]
extern crate rocket;

use std::fs::create_dir_all;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use rand::Rng;

use image::DynamicImage;
use image::GenericImageView;
use image::ImageBuffer;
use image::load_from_memory_with_format;
use image::RGB;
use image::Rgba;
use image::save_buffer;
use reqwest::Response;
use rocket::{get, routes};
use rocket::http::ContentType;
use rocket::response::content;

const API_URL: &str = "https://opendata-download-radar.smhi.se/api";
const SUB_DIRECTORY: &str = "version/latest/area/sweden/product/comp";
const TIFF_EXTENSION: &str = "tif";

fn mkdir(path: &str) -> std::io::Result<()> {
    create_dir_all(&path)?;
    Ok(())
}

fn get_rand_col(alpha: u8) -> Rgba<u8> {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(1, 255);
    let g = rng.gen_range(1, 255);
    let b = rng.gen_range(1, 255);
    //println!("Color: R: {}, G: {}, B: {}, A: {}", r, g, b, alpha);
    Rgba([r, g, b, alpha])
}

fn gain(rgb: [u8; 3], alpha: u8) -> Rgba<u8> {
    match rgb {
        [0, 0, 0] => Rgba([0,0,0, 0]),
        [54, 54, 54] => get_rand_col(alpha),
        [55, 55, 55] => get_rand_col(alpha),
        [56, 56, 56] => get_rand_col(alpha),
        [57, 57, 57] => get_rand_col(alpha),
        [58, 58, 58] => get_rand_col(alpha),
        [59, 59, 59] => get_rand_col(alpha),
        [60, 60, 60] => get_rand_col(alpha),
        [61, 61, 61] => get_rand_col(alpha),
        [62, 62, 62] => get_rand_col(alpha),
        [63, 63, 63] => get_rand_col(alpha),
        [64, 64, 64] => get_rand_col(alpha),
        [65, 65, 65] => get_rand_col(alpha),
        [66, 66, 66] => get_rand_col(alpha),
        [67, 67, 67] => get_rand_col(alpha),
        [68, 68, 68] => get_rand_col(alpha),
        [69, 69, 69] => get_rand_col(alpha),
        [70, 70, 70] => get_rand_col(alpha),
        [71, 71, 71] => get_rand_col(alpha),
        [72, 72, 72] => get_rand_col(alpha),
        [73, 73, 73] => get_rand_col(alpha),
        [74, 74, 74] => get_rand_col(alpha),
        [75, 75, 75] => get_rand_col(alpha),
        [76, 76, 76] => get_rand_col(alpha),
        [87, 87, 87] => get_rand_col(alpha),
        [97, 97, 97] => get_rand_col(alpha),
        [108, 108, 108] => get_rand_col(alpha),
        [118, 118, 118] => get_rand_col(alpha),
        [129, 129, 129] => get_rand_col(alpha),
        [139, 139, 139] => get_rand_col(alpha),
        [150, 150, 150] => get_rand_col(alpha),
        [160, 160, 160] => get_rand_col(alpha),
        [171, 171, 171] => get_rand_col(alpha),
        [181, 181, 181] => get_rand_col(alpha),
        [192, 192, 192] => get_rand_col(alpha),
        [0, 50, 255] => get_rand_col(alpha),
        [0, 70, 255] => get_rand_col(alpha),
        [0, 90, 255] => get_rand_col(alpha),
        [0, 110, 255] => get_rand_col(alpha),
        [0, 130, 255] => get_rand_col(alpha),
        [0, 150, 255] => get_rand_col(alpha),
        [0, 170, 255] => get_rand_col(alpha),
        [0, 128, 0] => get_rand_col(alpha),
        [0, 138, 0] => get_rand_col(alpha),
        [0, 148, 0] => get_rand_col(alpha),
        [0, 158, 0] => get_rand_col(alpha),
        [0, 163, 0] => get_rand_col(alpha),
        [0, 168, 0] => get_rand_col(alpha),
        [0, 173, 0] => get_rand_col(alpha),
        [0, 178, 0] => get_rand_col(alpha),
        [10, 208, 10] => get_rand_col(alpha),
        [10, 218, 10] => get_rand_col(alpha),
        [10, 228, 10] => get_rand_col(alpha),
        [10, 238, 10] => get_rand_col(alpha),
        [10, 248, 10] => get_rand_col(alpha),
        [255, 255, 15] => get_rand_col(alpha),
        [255, 246, 15] => get_rand_col(alpha),
        [255, 238, 15] => get_rand_col(alpha),
        [255, 229, 15] => get_rand_col(alpha),
        [255, 220, 15] => get_rand_col(alpha),
        [255, 200, 0] => get_rand_col(alpha),
        [255, 180, 0] => get_rand_col(alpha),
        [255, 160, 0] => get_rand_col(alpha),
        [255, 140, 0] => get_rand_col(alpha),
        [255, 120, 0] => get_rand_col(alpha),
        [255, 35, 35] => get_rand_col(alpha),
        [255, 15, 15] => get_rand_col(alpha),
        [255, 0, 0] => get_rand_col(alpha),
        [235, 0, 0] => get_rand_col(alpha),
        [215, 0, 0] => get_rand_col(alpha),
        [195, 0, 0] => get_rand_col(alpha),
        [175, 0, 0] => get_rand_col(alpha),
        [155, 0, 0] => get_rand_col(alpha),
        [135, 0, 0] => get_rand_col(alpha),
        [115, 0, 0] => get_rand_col(alpha),
        [175, 0, 175] => get_rand_col(alpha),
        [184, 0, 184] => get_rand_col(alpha),
        [193, 0, 193] => get_rand_col(alpha),
        [202, 0, 202] => get_rand_col(alpha),
        [211, 0, 211] => get_rand_col(alpha),
        [219, 0, 219] => get_rand_col(alpha),
        [228, 0, 228] => get_rand_col(alpha),
        [237, 0, 237] => get_rand_col(alpha),
        [246, 0, 246] => get_rand_col(alpha),
        [255, 0, 255] => get_rand_col(alpha),
        [0, 255, 255] => get_rand_col(alpha),
        [13, 255, 255] => get_rand_col(alpha),
        [26, 255, 255] => get_rand_col(alpha),
        [39, 255, 255] => get_rand_col(alpha),
        [51, 255, 255] => get_rand_col(alpha),
        [64, 255, 255] => get_rand_col(alpha),
        [77, 255, 255] => get_rand_col(alpha),
        [90, 255, 255] => get_rand_col(alpha),
        [102, 255, 255] => get_rand_col(alpha),
        [115, 255, 255] => get_rand_col(alpha),
        [128, 255, 255] => get_rand_col(alpha),
        [141, 255, 255] => get_rand_col(alpha),
        [154, 255, 255] => get_rand_col(alpha),
        [166, 255, 255] => get_rand_col(alpha),
        [179, 255, 255] => get_rand_col(alpha),
        [192, 255, 255] => get_rand_col(alpha),
        [255, 255, 255] => Rgba([255, 255, 255, 16]),
        _ => get_rand_col(alpha),
    }
}

#[get("/version/latest/area/sweden/product/comp/<year>/<month>/<day>/<filename>")]
fn retrieve(year: String, month: String, day: String, filename: String) -> Option<content::Content<File>> {
    let start = Instant::now();
    println!("Start Elapsed: {:?}", start.elapsed());

    let file: String = format!("{year}/{month}/{day}/{filename}", year = year, month = month, day = day, filename = filename);
    let path: String = format!("{year}/{month}/{day}", year = year, month = month, day = day);
    let url: String = format!("{}/{}/{}.{}", API_URL, SUB_DIRECTORY, file, TIFF_EXTENSION);

    mkdir(&path);

    let filename_path: &Path = Path::new(&filename);

    let path_buf: PathBuf = match filename_path.file_stem() {
        Some(stem) => {

            let new_file: String = format!("{}/{}.png", &path, stem.to_str().unwrap());
            let new_path: &Path = Path::new(&new_file);

            if new_path.exists() {
                println!("File exists: {:?}", new_path);
                let buf: PathBuf = new_path.to_owned();
                buf
            } else {
                println!("File not found!");
                let mut response: Response = reqwest::get(&url).unwrap();

                let mut buffer: Vec<u8> = Vec::new();

                response.read_to_end(&mut buffer).unwrap();

                let img: DynamicImage = load_from_memory_with_format(&buffer, image::ImageFormat::TIFF).unwrap();
                let (width, height) = img.dimensions();

                let mut new_colorized_image = ImageBuffer::new(width, height);

                for (x, y, pixel) in img.pixels() {
                    let [r, g, b, alpha] = pixel.data;
                    let new_pixel: Rgba<u8> = gain([r, g, b], alpha);
                    new_colorized_image.put_pixel(x, y, new_pixel)
                }
                new_colorized_image.save(new_path).unwrap();

                let pathbuf: PathBuf = new_path.to_owned();
                pathbuf
            }
        }
        None => {
            panic!("Error");
        }
    };

    let f = File::open(&path_buf).map(|f| content::Content(ContentType::PNG, f)).ok();
    let duration = start.elapsed();
    println!("Elapsed: {:?}", start.elapsed());
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