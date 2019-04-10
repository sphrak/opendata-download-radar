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

use image::DynamicImage;
use image::GenericImageView;
use image::ImageBuffer;
use image::load_from_memory_with_format;
use image::RGB;
use image::Rgba;
use image::save_buffer;
use rand::Rng;
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

fn get_rand_col_other(alpha: u8) -> Rgba<u8> {
    Rgba([20, 252, 0, alpha])
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
        [0, 0, 0] => Rgba([0, 0, 0, 0]),
        [15, 15, 15] => get_rand_col(alpha),
        [16, 16, 16] => get_rand_col(alpha),
        [17, 17, 17] => get_rand_col(alpha),
        [18, 18, 18] => get_rand_col(alpha),
        [19, 19, 19] => get_rand_col(alpha),
        [20, 20, 20] => get_rand_col(alpha),
        [21, 21, 21] => get_rand_col(alpha),
        [22, 22, 22] => get_rand_col(alpha),
        [23, 23, 23] => get_rand_col(alpha),
        [50, 50, 50] => get_rand_col(alpha),
        [51, 51, 51] => get_rand_col(alpha),
        [52, 52, 52] => get_rand_col(alpha),
        [53, 53, 53] => get_rand_col(alpha),
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
        [77, 77, 77] => get_rand_col(alpha),
        [78, 78, 78] => get_rand_col(alpha),
        [79, 79, 79] => get_rand_col(alpha),
        [80, 80, 80] => get_rand_col(alpha),
        [81, 81, 81] => get_rand_col(alpha),
        [82, 82, 82] => get_rand_col(alpha),
        [83, 83, 83] => get_rand_col(alpha),
        [84, 84, 84] => get_rand_col(alpha),
        [85, 85, 85] => get_rand_col(alpha),
        [86, 86, 86] => get_rand_col(alpha),
        [87, 87, 87] => get_rand_col(alpha),
        [88, 88, 88] => get_rand_col(alpha),
        [89, 89, 89] => get_rand_col(alpha),
        [90, 90, 90] => get_rand_col(alpha),
        [91, 91, 91] => get_rand_col(alpha),
        [92, 92, 92] => get_rand_col(alpha),
        [93, 93, 93] => get_rand_col(alpha),
        [94, 94, 94] => get_rand_col(alpha),
        [95, 95, 95] => get_rand_col(alpha),
        [96, 96, 96] => get_rand_col(alpha),
        [97, 97, 97] => get_rand_col(alpha),
        [98, 98, 98] => get_rand_col(alpha),
        [99, 99, 99] => get_rand_col(alpha),
        [100, 100, 100] => get_rand_col(alpha),
        [101, 101, 101] => get_rand_col(alpha),
        [102, 102, 102] => get_rand_col(alpha),
        [103, 103, 103] => get_rand_col(alpha),
        [104, 104, 104] => get_rand_col(alpha),
        [105, 105, 105] => get_rand_col(alpha),
        [106, 106, 106] => get_rand_col(alpha),
        [107, 107, 107] => get_rand_col(alpha),
        [108, 108, 108] => get_rand_col(alpha),
        [109, 109, 109] => get_rand_col(alpha),
        [110, 110, 110] => get_rand_col(alpha),
        [111, 111, 111] => get_rand_col(alpha),
        [112, 112, 112] => get_rand_col(alpha),
        [113, 113, 113] => get_rand_col(alpha),
        [114, 114, 114] => get_rand_col(alpha),
        [115, 115, 115] => get_rand_col(alpha),
        [116, 116, 116] => get_rand_col(alpha),
        [117, 117, 117] => get_rand_col(alpha),
        [118, 118, 118] => get_rand_col(alpha),
        [119, 119, 119] => get_rand_col(alpha),
        [120, 120, 120] => get_rand_col(alpha),
        [121, 121, 121] => get_rand_col(alpha),
        [122, 122, 122] => get_rand_col(alpha),
        [123, 123, 123] => get_rand_col(alpha),
        [124, 124, 124] => get_rand_col(alpha),
        [125, 125, 125] => get_rand_col(alpha),
        [126, 126, 126] => get_rand_col(alpha),
        [127, 127, 127] => get_rand_col(alpha),
        [128, 128, 128] => get_rand_col(alpha),
        [129, 129, 129] => get_rand_col(alpha),
        [130, 130, 130] => get_rand_col(alpha),
        [131, 131, 131] => get_rand_col(alpha),
        [132, 132, 132] => get_rand_col(alpha),
        [133, 133, 133] => get_rand_col(alpha),
        [134, 134, 134] => get_rand_col(alpha),
        [135, 135, 135] => get_rand_col(alpha),
        [136, 136, 136] => get_rand_col(alpha),
        [137, 137, 137] => get_rand_col(alpha),
        [138, 138, 138] => get_rand_col(alpha),
        [139, 139, 139] => get_rand_col(alpha),
        [140, 140, 140] => get_rand_col(alpha),
        [141, 141, 141] => get_rand_col(alpha),
        [142, 142, 142] => get_rand_col(alpha),
        [143, 143, 143] => get_rand_col(alpha),
        [144, 144, 144] => get_rand_col(alpha),
        [145, 145, 145] => get_rand_col(alpha),
        [146, 146, 146] => get_rand_col(alpha),
        [147, 147, 147] => get_rand_col(alpha),
        [148, 148, 148] => get_rand_col(alpha),
        [149, 149, 149] => get_rand_col(alpha),
        [150, 150, 150] => get_rand_col(alpha),
        [151, 151, 151] => get_rand_col(alpha),
        [152, 152, 152] => get_rand_col(alpha),
        [153, 153, 153] => get_rand_col(alpha),
        [154, 154, 154] => get_rand_col(alpha),
        [155, 155, 155] => get_rand_col(alpha),
        [156, 156, 156] => get_rand_col(alpha),
        [157, 157, 157] => get_rand_col(alpha),
        [158, 158, 158] => get_rand_col(alpha),
        [159, 159, 159] => get_rand_col(alpha),
        [160, 160, 160] => get_rand_col(alpha),
        [161, 161, 161] => get_rand_col(alpha),
        [162, 162, 162] => get_rand_col(alpha),
        [163, 163, 163] => get_rand_col(alpha),
        [164, 164, 164] => get_rand_col(alpha),
        [165, 165, 165] => get_rand_col(alpha),
        [166, 166, 166] => get_rand_col(alpha),
        [167, 167, 167] => get_rand_col(alpha),
        [168, 168, 168] => get_rand_col(alpha),
        [169, 169, 169] => get_rand_col(alpha),
        [170, 170, 170] => get_rand_col(alpha),
        [171, 171, 171] => get_rand_col(alpha),
        [172, 172, 172] => get_rand_col(alpha),
        [173, 173, 173] => get_rand_col(alpha),
        [174, 174, 174] => get_rand_col(alpha),
        [175, 175, 175] => get_rand_col(alpha),
        [176, 176, 176] => get_rand_col(alpha),
        [177, 177, 177] => get_rand_col(alpha),
        [178, 178, 178] => get_rand_col(alpha),
        [179, 179, 179] => get_rand_col(alpha),
        [180, 180, 180] => get_rand_col(alpha),
        [181, 181, 181] => get_rand_col(alpha),
        [182, 182, 182] => get_rand_col(alpha),
        [183, 183, 183] => get_rand_col(alpha),
        [184, 184, 184] => get_rand_col(alpha),
        [185, 185, 185] => get_rand_col(alpha),
        [186, 186, 186] => get_rand_col(alpha),
        [187, 187, 187] => get_rand_col(alpha),
        [188, 188, 188] => get_rand_col(alpha),
        [189, 189, 189] => get_rand_col(alpha),
        [190, 190, 190] => get_rand_col(alpha),
        [191, 191, 191] => get_rand_col(alpha),
        [192, 192, 192] => get_rand_col(alpha),
        [193, 193, 193] => get_rand_col(alpha),
        [194, 194, 194] => get_rand_col(alpha),
        [195, 195, 195] => get_rand_col(alpha),
        [196, 196, 196] => get_rand_col(alpha),
        [197, 197, 197] => get_rand_col(alpha),
        [198, 198, 198] => get_rand_col(alpha),
        [199, 199, 199] => get_rand_col(alpha),
        [200, 200, 200] => get_rand_col(alpha),
        [201, 201, 201] => get_rand_col(alpha),
        [202, 202, 202] => get_rand_col(alpha),
        [203, 203, 203] => get_rand_col(alpha),
        [204, 204, 204] => get_rand_col(alpha),
        [205, 205, 205] => get_rand_col(alpha),
        [206, 206, 206] => get_rand_col(alpha),
        [207, 207, 207] => get_rand_col(alpha),
        [208, 208, 208] => get_rand_col(alpha),
        [209, 209, 209] => get_rand_col(alpha),
        [210, 210, 210] => get_rand_col(alpha),
        [211, 211, 211] => get_rand_col(alpha),
        [212, 212, 212] => get_rand_col(alpha),
        [213, 213, 213] => get_rand_col(alpha),
        [214, 214, 214] => get_rand_col(alpha),
        [215, 215, 215] => get_rand_col(alpha),
        [216, 216, 216] => get_rand_col(alpha),
        [217, 217, 217] => get_rand_col(alpha),
        [218, 218, 218] => get_rand_col(alpha),
        [219, 219, 219] => get_rand_col(alpha),
        [220, 220, 220] => get_rand_col(alpha),
        [221, 221, 221] => get_rand_col(alpha),
        [222, 222, 222] => get_rand_col(alpha),
        [223, 223, 223] => get_rand_col(alpha),
        [224, 224, 224] => get_rand_col(alpha),
        [225, 225, 225] => get_rand_col(alpha),
        [226, 226, 226] => get_rand_col(alpha),
        [227, 227, 227] => get_rand_col(alpha),
        [228, 228, 228] => get_rand_col(alpha),
        [229, 229, 229] => get_rand_col(alpha),
        [230, 230, 230] => get_rand_col(alpha),
        [231, 231, 231] => get_rand_col(alpha),
        [232, 232, 232] => get_rand_col(alpha),
        [233, 233, 233] => get_rand_col(alpha),
        [234, 234, 234] => get_rand_col(alpha),
        [235, 235, 235] => get_rand_col(alpha),
        [236, 236, 236] => get_rand_col(alpha),
        [237, 237, 237] => get_rand_col(alpha),
        [238, 238, 238] => get_rand_col(alpha),
        [239, 239, 239] => get_rand_col(alpha),
        [241, 241, 241] => get_rand_col(alpha),
        [242, 242, 242] => get_rand_col(alpha),
        [243, 243, 243] => get_rand_col(alpha),
        [244, 244, 244] => get_rand_col(alpha),
        [245, 245, 245] => get_rand_col(alpha),
        [246, 246, 246] => get_rand_col(alpha),
        [247, 247, 247] => get_rand_col(alpha),
        [248, 248, 248] => get_rand_col(alpha),
        [249, 249, 249] => get_rand_col(alpha),
        [253, 253, 253] => get_rand_col(alpha),
        [255, 255, 255] => get_rand_col(alpha),

        /*[54, 54, 54] => get_rand_col(alpha),
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
        [192, 255, 255] => get_rand_col(alpha),*/
        [255, 255, 255] => Rgba([255, 255, 255, 16]),
        _ => {
            println!("Color missing: {:?}", rgb);
            get_rand_col_other(alpha)
        },
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
