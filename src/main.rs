#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate image;
#[macro_use]
extern crate rocket;

use std::fs::create_dir_all;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::{Duration, Instant};

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

fn mkdir(path: &str) -> std::io::Result<()> {
    create_dir_all(&path)?;
    Ok(())
}

//  (((54 + 54) + 54) * 0.4) + (-30) gets the dBZ gain

fn gain(rgb: u8) -> Rgba([u8]) {
    // calculate GAIN dBZ values
    // gain = 0.4
    // offset = -30.0
    // dBZ = pixel_value * gain * offset
    let arr: [[u8; 3]; 100] = [
        [54, 54, 54],
        [55, 55, 55],
        [56, 56, 56],
        [57, 57, 57],
        [58, 58, 58],
        [59, 59, 59],
        [60, 60, 60],
        [61, 61, 61],
        [62, 62, 62],
        [63, 63, 63],
        [64, 64, 64],
        [65, 65, 65],
        [66, 66, 66],
        [67, 67, 67],
        [68, 68, 68],
        [69, 69, 69],
        [70, 70, 70],
        [71, 71, 71],
        [72, 72, 72],
        [73, 73, 73],
        [74, 74, 74],
        [75, 75, 75],
        [76, 76, 76],
        [87, 87, 87],
        [97, 97, 97],
        [108, 108, 108],
        [118, 118, 118],
        [129, 129, 129],
        [139, 139, 139],
        [150, 150, 150],
        [160, 160, 160],
        [171, 171, 171],
        [181, 181, 181],
        [192, 192, 192],
        [0, 50, 255],
        [0, 70, 255],
        [0, 90, 255],
        [0, 110, 255],
        [0, 130, 255],
        [0, 150, 255],
        [0, 170, 255],
        [0, 128, 0],
        [0, 138, 0],
        [0, 148, 0],
        [0, 158, 0],
        [0, 163, 0],
        [0, 168, 0],
        [0, 173, 0],
        [0, 178, 0],
        [10, 208, 10],
        [10, 218, 10],
        [10, 228, 10],
        [10, 238, 10],
        [10, 248, 10],
        [255, 255, 15],
        [255, 246, 15],
        [255, 238, 15],
        [255, 229, 15],
        [255, 220, 15],
        [255, 200, 0],
        [255, 180, 0],
        [255, 160, 0],
        [255, 140, 0],
        [255, 120, 0],
        [255, 35, 35],
        [255, 15, 15],
        [255, 0, 0],
        [235, 0, 0],
        [215, 0, 0],
        [195, 0, 0],
        [175, 0, 0],
        [155, 0, 0],
        [135, 0, 0],
        [115, 0, 0],
        [175, 0, 175],
        [184, 0, 184],
        [193, 0, 193],
        [202, 0, 202],
        [211, 0, 211],
        [219, 0, 219],
        [228, 0, 228],
        [237, 0, 237],
        [246, 0, 246],
        [255, 0, 255],
        [0, 255, 255],
        [13, 255, 255],
        [26, 255, 255],
        [39, 255, 255],
        [51, 255, 255],
        [64, 255, 255],
        [77, 255, 255],
        [90, 255, 255],
        [102, 255, 255],
        [115, 255, 255],
        [128, 255, 255],
        [141, 255, 255],
        [154, 255, 255],
        [166, 255, 255],
        [179, 255, 255],
        [192, 255, 255],
    ];

    for x in &arr[..] {
        println!(
            "x,y: RGB: {:?}, R: {:?}, G: {:?}, B: {:?},",
            x, x[0], x[1], x[2]
        )
    }

    if [[r][g][b]] == array[r][g][b] {
        println!("Lol: {:?}", array);
    }
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

    // TODO let blah = match etc and reeturn the file boi
    match filename_path.file_stem() {
        Some(stem) => {
            let new_file: String = format!("{}/{}.png", path, stem.to_str().unwrap());
            let new_path: &Path = Path::new(&new_file);

            let mut buffer: Vec<u8> = Vec::new();

            response.read_to_end(&mut buffer).unwrap();

            let mut img: DynamicImage = load_from_memory_with_format(&buffer, image::ImageFormat::TIFF).unwrap();

            // get dimensions
            let (width, height) = img.dimensions();

            // save the buffer as .png file
            println!("Saving: {:?}", new_path);


            println!("Height: {}", height);
            println!("Weight: {}", width);

            let mut new_colorized_image = ImageBuffer::new(width, height);

            for (x, y, pixel) in img.pixels() {
                //println!("R: {:?}, G: {:?}, B: {:?}, A: {:?}", pixel.data[0], pixel.data[1], pixel.data[2], pixel.data[3]);
                //pixel.data
                let [r, g, b, a] = pixel.data;



                // get new translate(rgb, channel) color
                // get back [r,g,b,a]
                // TODO just summarize all rgb values.. match against
                // our new color scheme. maybe a color ini file?

                let new_pixel = image::Rgba([r, g, b, a]);


                new_colorized_image.put_pixel(x, y, new_pixel)
            }

            new_colorized_image.save("nybild.png").unwrap();

            // THIS IS HOW WE SHOULD SAVE BUT NE
            img.save(new_path).unwrap();
        }
        None => {
            panic!("Error");
        }
    }

    let new_path = Path::new("2019/04/03/radar_1904030005.png");

    println!("Elapsed: {:?}", start.elapsed());
    let f = File::open(&new_path).map(|f| content::Content(ContentType::PNG, f)).ok();
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