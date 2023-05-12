// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
 )]
use std::fs;
use std::vec;

use calamine::{open_workbook, open_workbook_auto, Error, RangeDeserializerBuilder, Reader, Xlsx};
use tauri::generate_handler;
use tauri::Manager;

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
        .invoke_handler(generate_handler![read_excel_lines, get_image_from_directory, copy_file])
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
        },
        Err(e) => {
            return Err(From::from(e.to_string()))
        }
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
            let ret: Vec<String> = paths.map(|path| path.unwrap().path().to_str().unwrap().to_string()).collect();
            // for path in paths {
            //     println!("Name: {}", path.unwrap().path().display());
            // }
            Ok(ret)
        },
        Err(_) => Err(From::from("error")),
    }
}

#[tauri::command(rename_all = "snake_case")]
fn get_image_from_directory(source: &str) -> Result<Vec<String>, String> {
    match read_directory(source) {
        Ok(o) => {
            Ok(o.into_iter().filter(|data| data.ends_with("jpg") || data.ends_with("jpeg")).collect())
        },
        Err(e) => Err(e)
    }
}

#[tauri::command(rename_all = "snake_case")]
fn copy_file(from: &str, to: &str) -> Result<u64, String> {
    match fs::copy(from, to) {
        Ok(o) => Ok(o),
        Err(e) => Err(e.to_string())
    }
}
