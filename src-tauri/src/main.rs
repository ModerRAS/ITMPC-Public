// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::vec;

use calamine::{open_workbook, open_workbook_auto, Error, RangeDeserializerBuilder, Reader, Xlsx};

use chrono::NaiveDate;
use rust_xlsxwriter::{Format, FormatAlign, FormatBorder, Image, Workbook, XlsxError};

use exif::In;
use exif::Tag;
use flyr::try_parse_flir;
use image::imageops;
use image::GenericImageView;
use image::ImageBuffer;
use image::ImageError;
use image::ImageFormat;
use image::Pixel;
use rand::{thread_rng, Rng};
use rnglib::{Language, RNG};
use serde::Serialize;
use tauri::generate_handler;
use tauri::Manager;

use serde::Deserialize;
use serde_json;

use directories::{BaseDirs, UserDirs, ProjectDirs};

#[derive(Deserialize)]
struct OcrResult {
    code: i32,
    data: Vec<OcrData>,
}

#[derive(Deserialize)]
struct OcrData {
    #[serde(rename = "box")]
    Box: Vec<Vec<i32>>,
    score: f32,
    text: String,
}

#[derive(Deserialize, Serialize)]
enum OcrError {
    NotFound,
    NotSupportPlatform,
    NotDetectd,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(generate_handler![
            read_excel_lines,
            get_image_from_directory,
            copy_file,
            read_thermal
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command(rename_all = "snake_case")]
fn read_excel_lines(excel_path: &str) -> Result<Vec<String>, String> {
    let path = excel_path.to_string();
    let _workbook_result = match open_workbook_auto(path) {
        Ok(workbook) => {
            let mut workbook = workbook;
            let mut data = Vec::new();
            if let Some(Ok(r)) = workbook.worksheet_range_at(1) {
                for row in r.rows() {
                    // println!("row={:?}, row[0]={:?}", row, row[0]);
                    if row[0].get_float() != None {
                        data.push(row[2].to_string());
                        println!(
                            "row[0] = {:?}\trow[2]={:?}",
                            row[0].get_float(),
                            row[2].to_string()
                        );
                    }
                }
                return Ok(data);
            } else {
                return Err(From::from("cannot get sheets"));
            }
        }
        Err(e) => return Err(From::from(e.to_string())),
    };
    // if let Some(result) = iter.next() {
    //     let (label, value): (String, f64) = result?;
    //     assert_eq!(label, "celsius");
    //     assert_eq!(value, 22.2222);
    //     Ok(vec!["1".to_string(), "2".to_string()])
    // } else {
    //     Err(From::from("expected at least one record but got none"))
    // }
}

fn read_directory(source: &str) -> Result<Vec<String>, String> {
    let paths = fs::read_dir(source);
    match paths {
        Ok(paths) => {
            let ret: Vec<String> = paths
                .map(|path| path.unwrap().path().to_str().unwrap().to_string())
                .collect();
            // for path in paths {
            //     println!("Name: {}", path.unwrap().path().display());
            // }
            Ok(ret)
        }
        Err(_) => Err(From::from("error")),
    }
}

#[tauri::command(rename_all = "snake_case")]
fn get_image_from_directory(source: &str) -> Result<Vec<String>, String> {
    match read_directory(source) {
        Ok(o) => Ok(o
            .into_iter()
            .filter(|data| data.ends_with("jpg") || data.ends_with("jpeg"))
            .collect()),
        Err(e) => Err(e),
    }
}

#[tauri::command(rename_all = "snake_case")]
fn copy_file(from: &str, to: &str) -> Result<u64, String> {
    match fs::copy(from, to) {
        Ok(o) => Ok(o),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command(rename_all = "snake_case")]
fn write_to_excel() {
    let mut workbook = Workbook::new();


}

#[tauri::command(rename_all = "snake_case")]
fn read_thermal(image_path: &str) -> Result<f32, ()> {
    let temp_dir = env::temp_dir();
    let rng = RNG::try_from(&Language::Elven).unwrap();

    let temp_name = rng.generate_name();
    let output_path = temp_dir.join(temp_name + ".jpg");
    match clip_picture(&PathBuf::from(image_path), &output_path) {
        Ok(_) => match detect_image_thermal_ocr(output_path.clone().into_os_string().into_string().unwrap().as_str()) {
            Ok(t) => {
                match fs::remove_file(&output_path) {
                    Ok(_) => return Ok(t),
                    Err(_) => return Err(()),
                };

            },
            Err(_) => match fs::remove_file(&output_path) {
                Ok(_) | Err(_) => return Err(()),
            },
        },
        Err(_) => match fs::remove_file(&output_path) {
            Ok(_) | Err(_) => return Err(()),
        },
    };
}

fn detect_image_thermal_ocr(path: &str) -> Result<f32, OcrError> {
    if cfg!(target_os = "linux") {
        return Err(OcrError::NotSupportPlatform);
    } else if cfg!(target_os = "macos") {
        return Err(OcrError::NotSupportPlatform);
    }
    if let Some(proj_dirs) = ProjectDirs::from("com", "com.miaostay.itmpc",  "PaddleOCR-json") {
        let ppocr_dirs = proj_dirs.data_local_dir().join("PaddleOCR_json.exe");
        println!("{}", ppocr_dirs.clone().into_os_string().into_string().unwrap());
        // Lin: /home/alice/.config/barapp
        // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
        // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
        let mut p = paddleocr::Ppocr::new(ppocr_dirs)
        .unwrap();
        let ret = p.ocr(path).unwrap();
        match serde_json::from_str::<OcrResult>(&ret) {
            Ok(v) => {
                if v.code != 100 {
                    return Err(OcrError::NotFound);
                }
                for data in &v.data {
                    match data.text.parse::<f32>() {
                        Ok(text) => return Ok(text),
                        Err(_) => continue,
                    }
                }
            },
            Err(_) => {
                return Err(OcrError::NotFound)
            }
        }

        return Err(OcrError::NotDetectd);
        // println!("{}", ret);
    }
    Err(OcrError::NotFound)

}

fn clip_picture(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), ()> {
    let file = match File::open(input_path) {
        Ok(o) => o,
        Err(_) => return Err(()),
    };
    let mut bufreader = BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = match exifreader.read_from_container(&mut bufreader) {
        Ok(o) => o,
        Err(_) => return Err(()),
    };
    let mut img = match image::open(input_path) {
        Ok(o) => o,
        Err(_) => return Err(()),
    };

    match exif.get_field(Tag::Orientation, In::PRIMARY) {
        Some(orientation) => match orientation.value.get_uint(0) {
            Some(v @ 1) => {
                let subimg = imageops::crop(&mut img, 0, 0, 145, 45);
                // subimg.get_pixel(0, 0)
                match subimg
                    .to_image()
                    .save_with_format(output_path, ImageFormat::Jpeg)
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                }
            }
            Some(v @ 3) => {
                let mut img = imageops::rotate180(&mut img);
                let subimg = imageops::crop(&mut img, 0, 0, 145, 45);
                // subimg.get_pixel(0, 0)
                match subimg
                    .to_image()
                    .save_with_format(output_path, ImageFormat::Jpeg)
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                }
            }
            Some(v @ 6) => {
                let mut img = imageops::rotate90(&mut img);
                let subimg = imageops::crop(&mut img, 0, 0, 145, 45);
                // subimg.get_pixel(0, 0)
                match subimg
                    .to_image()
                    .save_with_format(output_path, ImageFormat::Jpeg)
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                }
            }
            Some(v @ 8) => {
                let mut img = imageops::rotate270(&mut img);
                let subimg = imageops::crop(&mut img, 0, 0, 145, 45);
                // subimg.get_pixel(0, 0)
                match subimg
                    .to_image()
                    .save_with_format(output_path, ImageFormat::Jpeg)
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                }
            }
            _ => Err(()),
        },
        None => Err(()),
    }
}

fn download_ocr_lib() {
    sevenz_rust::decompress_file("data/sample.7z", "data/sample").expect("complete");
}

#[cfg(test)]
pub mod tests;
