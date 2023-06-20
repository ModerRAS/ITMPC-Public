// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::collections::HashMap;
use std::env;
use std::fs;

use std::path::PathBuf;
use std::vec;
use rnglib::{Language, RNG};
use tauri::generate_handler;
use tauri::Manager;

use ocr::*;

use excel::{get_image_names, get_excel_lines};

use crate::data::get_data;
use crate::data::import_data;
use crate::excel::fix_missing_field;
use crate::excel::process_excel_data;
use crate::excel::reprocess_excel_data;
use crate::excel::write_to_excel;


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
            get_image_names,
            get_image_from_directory,
            copy_file,
            read_thermal,
            prepare_ocr_lib,
            get_excel_lines,
            write_to_excel,
            read_thermals,
            fix_missing_field,
            process_excel_data,
            reprocess_excel_data,
            import_data,
            get_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command(rename_all = "snake_case")]
async fn prepare_ocr_lib() {
    if check_if_ocr_lib_downloaded() {
        return;
    }
    let download_path = get_download_path().unwrap();
    let data_path = get_ocr_path().unwrap();
    download_files("https://ghp.miaostay.com/https://github.com/ModerRAS/ITMPC-Public/releases/download/ocr_lib/PaddleOCR-json.v1.2.1.zip", download_path.clone()).await.unwrap();
    unzip_ocr_lib(download_path, data_path);
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
async fn get_image_from_directory(source: &str) -> Result<Vec<String>, String> {
    match read_directory(source) {
        Ok(o) => Ok(o
            .into_iter()
            .filter(|data| data.ends_with("jpg") || data.ends_with("jpeg"))
            .collect()),
        Err(e) => Err(e),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn copy_file(from: &str, to: &str) -> Result<u64, String> {
    match fs::copy(from, to) {
        Ok(o) => Ok(o),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn read_thermals(image_paths: Vec<String>) -> Result<HashMap<String, f64>, ()> {
    match init_ppocr() {
        Ok(mut p) => return Ok(image_paths.iter().map(|image_path| {
            let temp_dir = env::temp_dir();
            let rng = RNG::try_from(&Language::Elven).unwrap();

            let temp_name = rng.generate_name();
            let output_path = temp_dir.join(temp_name + ".jpg");
            match clip_picture(&PathBuf::from(image_path), &output_path) {
                Ok(_) => match detect_image_thermal_ocr_with_ppocr(&mut p, output_path
                    .clone()
                    .into_os_string()
                    .into_string()
                    .unwrap()
                    .as_str(),) {
                    Ok(t) => {
                        match fs::remove_file(&output_path) {
                            Ok(_) | Err(_) => return (image_path.clone().to_string(), t),
                        };
                    },
                    Err(_) => return (image_path.clone().to_string(), -99_999f64),
                }
                Err(e) => match fs::remove_file(&output_path) {
                    Ok(_) | Err(_) => {
                        println!("{:?}", e);
                        return (image_path.clone().to_string(), -99_999f64);
                    },
                },
            };


        }).collect()),
        Err(_) => Err(()),
    }

}

#[tauri::command(rename_all = "snake_case")]
async fn read_thermal(image_path: &str) -> Result<f64, ()> {
    let temp_dir = env::temp_dir();
    let rng = RNG::try_from(&Language::Elven).unwrap();

    let temp_name = rng.generate_name();
    let output_path = temp_dir.join(temp_name + ".jpg");
    match clip_picture(&PathBuf::from(image_path), &output_path) {
        Ok(_) => match detect_image_thermal_ocr(
            output_path
                .clone()
                .into_os_string()
                .into_string()
                .unwrap()
                .as_str(),
        ) {
            Ok(t) => {
                match fs::remove_file(&output_path) {
                    Ok(_) => return Ok(t),
                    Err(_) => return Err(()),
                };
            }
            Err(e) => match fs::remove_file(&output_path) {
                Ok(_) | Err(_) => {
                    println!("{:?}", e);
                    return Err(());
                },
            },
        },
        Err(e) => match fs::remove_file(&output_path) {
            Ok(_) | Err(_) => {
                println!("{:?}", e);
                return Err(());
            },
        },
    };
}



pub mod excel;

pub mod ocr;

pub mod data;