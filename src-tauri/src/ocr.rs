use directories::{ProjectDirs, UserDirs};
use exif::{In, Tag};
use futures_util::StreamExt;
use std::{
    fs::{self, File},
    io::{self, BufReader, Write},
    path::PathBuf,
};

use image::{imageops, ImageFormat};
use paddleocr::Ppocr;
use serde::{Deserialize, Serialize};

use colors_transform::{Color, Rgb};

use lazy_static::lazy_static;

use regex::Regex;

#[derive(Deserialize)]
pub struct OcrResult {
    code: i32,
    data: Vec<OcrData>,
}

#[derive(Deserialize)]
pub struct OcrData {
    r#box: Vec<Vec<i32>>,
    score: f32,
    text: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OcrError {
    NotFound,
    NotSupportPlatform,
    NotDetectd,
}

pub fn check_if_ocr_lib_downloaded() -> bool {
    match get_ocr_path() {
        Ok(ocr_path) => return ocr_path.join("PaddleOCR_json.exe").exists(),
        Err(_) => return false,
    }
}

pub async fn download_files(url: &str, path: PathBuf) -> Result<(), ()> {
    let mut file = File::create(path).unwrap();
    println!("Downloading {}...", url);

    let mut stream = reqwest::get(url).await.unwrap().bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.unwrap();
        match file.write_all(&chunk) {
            Ok(_) | Err(_)=> continue
        };
    }

    file.flush().unwrap();

    println!("Downloaded {}", url);
    Ok(())
}

pub fn get_download_path() -> Result<PathBuf, String> {
    if let Some(proj_dirs) = UserDirs::new() {
        if let Some(ppocr_dirs) = proj_dirs.download_dir() {
            let fpath = ppocr_dirs.join("PaddleOCR-json.v1.2.1.zip");
            let fname = fpath;
            return Ok(fname);
        }
    }
    return Err(String::from("Error"));
}

pub fn get_ocr_path() -> Result<PathBuf, String> {
    if let Some(ppocr_dir) = ProjectDirs::from("com", "com.miaostay.itmpc", "PaddleOCR-json") {
        return Ok(ppocr_dir.data_local_dir().to_path_buf());
    }
    return Err(String::from("Error"));
}

pub fn unzip_ocr_lib(zip_path: PathBuf, to: PathBuf) -> i32 {
    let file = fs::File::open(zip_path.as_path()).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let outpath = to.join(outpath);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    0
}

fn detect_number(v: OcrResult) -> f64 {
    let mut max = -99999f64;
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[0-9.]*").unwrap();
    }

    for data in &v.data {
        println!("{}", data.text);
        let _out: Vec<f64> = RE.find_iter(data.text.as_str()).map(|mat| {
            max = max.max(mat.as_str().parse::<f64>().unwrap_or(-99999f64));
            max
        }).collect();
        println!("{:?}", _out);
    }
    return max;
}

pub fn detect_image_thermal_ocr(path: &str) -> Result<f64, OcrError> {
    if cfg!(target_os = "linux") {
        return Err(OcrError::NotSupportPlatform);
    }
    if cfg!(target_os = "macos") {
        return Err(OcrError::NotSupportPlatform);
    }
    if let Some(proj_dirs) = ProjectDirs::from("com", "com.miaostay.itmpc", "PaddleOCR-json") {
        let ppocr_dirs = proj_dirs.data_local_dir().join("PaddleOCR_json.exe");
        println!(
            "{}",
            ppocr_dirs.clone().into_os_string().into_string().unwrap()
        );
        let mut p = paddleocr::Ppocr::new(ppocr_dirs).unwrap();
        let ret = p.ocr(path).unwrap();
        match serde_json::from_str::<OcrResult>(&ret) {
            Ok(v) => {
                if v.code != 100 {
                    return Err(OcrError::NotFound);
                }

                return Ok(detect_number(v));
            }
            Err(_) => return Err(OcrError::NotFound),
        }

    }
    Err(OcrError::NotFound)
}

pub fn init_ppocr() -> Result<Ppocr, OcrError> {
    if cfg!(target_os = "linux") {
        return Err(OcrError::NotSupportPlatform);
    }
    if cfg!(target_os = "macos") {
        return Err(OcrError::NotSupportPlatform);
    }
    if let Some(proj_dirs) = ProjectDirs::from("com", "com.miaostay.itmpc", "PaddleOCR-json") {
        let ppocr_dirs = proj_dirs.data_local_dir().join("PaddleOCR_json.exe");
        println!(
            "{}",
            ppocr_dirs.clone().into_os_string().into_string().unwrap()
        );
        let p: paddleocr::Ppocr = paddleocr::Ppocr::new(ppocr_dirs).unwrap();
        return Ok(p);
    } else {
        return Err(OcrError::NotFound);
    }
}

pub fn detect_image_thermal_ocr_with_ppocr(
    p: &mut paddleocr::Ppocr,
    path: &str,
) -> Result<f64, OcrError> {
    let ret = p.ocr(path).unwrap();
    match serde_json::from_str::<OcrResult>(&ret) {
        Ok(v) => {
            if v.code != 100 {
                return Err(OcrError::NotFound);
            }
            return Ok(detect_number(v));
        }
        Err(_) => return Err(OcrError::NotFound),
    }
}

fn rotate_image(input_path: &PathBuf) -> Result<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, ()> {
    let file = match File::open(input_path) {
        Ok(o) => o,
        Err(_) => return Err(()),
    };
    let mut img = match image::open(input_path) {
        Ok(o) => o,
        Err(_) => return Err(()),
    };
    let mut bufreader = BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = match exifreader.read_from_container(&mut bufreader) {
        Ok(o) => o,
        Err(_) => {
            println!("cannot get exif");
            return Ok((&mut img).clone().into_rgba8());
        }
    };
    match exif.get_field(Tag::Orientation, In::PRIMARY) {
        Some(orientation) => match orientation.value.get_uint(0) {
            Some(_v @ 1) => Ok((&mut img).clone().into_rgba8()),
            Some(_v @ 3) => Ok(imageops::rotate180(&mut img)),
            Some(_v @ 6) => Ok(imageops::rotate90(&mut img)),
            Some(_v @ 8) => Ok(imageops::rotate270(&mut img)),
            _ => {
                println!("cannot match orientation");
                Ok((&mut img).clone().into_rgba8())
            }
        },
        None => {
            println!("cannot get field");
            Ok((&mut img).clone().into_rgba8())
        }
    }
}

pub fn clip_picture(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), ()> {
    match rotate_image(input_path) {
        Ok(mut img) => {
            binary_picture(&mut img);
            let subimg = imageops::crop(&mut img, 0, 0, 145, 100);
            // subimg.get_pixel(0, 0)
            match subimg
                .to_image()
                .save_with_format(output_path, ImageFormat::Jpeg)
            {
                Ok(_) => Ok(()),
                Err(_) => {
                    println!("Subimg error");
                    Err(())
                }
            }
        }
        Err(_) => {
            println!("rotate_image error");
            return Err(());
        }
    }
}

fn binary_picture(img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
    for (_x,_yy, pixel) in img.enumerate_pixels_mut() {
        let mut hsl_color = Rgb::from(
            f32::from(pixel[0]),
            f32::from(pixel[1]),
            f32::from(pixel[2]),
        )
        .to_hsl();
        if hsl_color.get_lightness() > 65f32 {
            hsl_color = hsl_color.set_lightness(0f32);
        } else {
            hsl_color = hsl_color.set_lightness(100f32);
        }
        let rgb_color = hsl_color.to_rgb();
        *pixel = image::Rgba([rgb_color.get_red() as u8, rgb_color.get_green() as u8, rgb_color.get_blue() as u8, pixel[3]]);
    }
}
