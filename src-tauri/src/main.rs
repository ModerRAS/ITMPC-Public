// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::fs;
use std::path::Path;
use std::vec;

use calamine::{open_workbook, open_workbook_auto, Error, RangeDeserializerBuilder, Reader, Xlsx};
use flyr::try_parse_flir;
use image::imageops;
use image::GenericImageView;
use image::Pixel;
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
        .invoke_handler(generate_handler![
            read_excel_lines,
            get_image_from_directory,
            copy_file
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
fn detect_cursor(image_path: &str) -> [usize; 2] {
    // let cursor_path = handle.path_resolver()
    //   .resolve_resource("resources/cursor.png")
    //   .expect("failed to resolve resource");
    let red_cursor_img = image::open("resources/red_cursor.png").unwrap();
    let white_cursor_img = image::open("resources/white_cursor.png").unwrap();
    let cursor_imgs = vec![red_cursor_img, white_cursor_img];
    // let mut img = image::open("test.jpg").unwrap();
    let mut img = image::open(image_path).unwrap();
    let height = usize::try_from(img.height()).unwrap();
    let width = usize::try_from(img.width()).unwrap();
    let mut min_point_x = 0;
    let mut min_point_y = 0;
    let mut min_point = i32::MAX;
    // let mut total_points: ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>> = Array::zeros((height - 48, width - 48));
    for x in 0..width - 48 {
        for y in 0..height - 48 {
            let subimg = imageops::crop(
                &mut img,
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                48,
                48,
            );
            let mut total_point: i32 = i32::MAX;
            for cursor_img in &cursor_imgs {
                let mut tmp_point = 0;
                'outer: for c_x in 0..48 {
                    for c_y in 0..48 {
                        let cursor_binding = cursor_img.get_pixel(c_x, c_y);
                        let cursor_channels = cursor_binding.channels();
                        if cursor_channels[3] == 0 {
                            continue;
                        }
                        let subimg_binding = subimg.get_pixel(c_x, c_y);
                        let subimg_channels = subimg_binding.channels();
                        let point = (i32::from(cursor_channels[0]) - i32::from(subimg_channels[0]))
                            .abs()
                            + (i32::from(cursor_channels[1]) - i32::from(subimg_channels[1])).abs()
                            + (i32::from(cursor_channels[2]) - i32::from(subimg_channels[2])).abs();
                        tmp_point += point;
                        if tmp_point > min_point || tmp_point > total_point {
                            break 'outer;
                        }
                    }
                }
                if tmp_point < total_point {
                    total_point = tmp_point;
                }
            }
            if total_point < min_point {
                min_point_x = x;
                min_point_y = y;
                min_point = total_point
            }
        }
    }
    println!(
        "width = {}, height = {}, min_point = {}, min_point_x = {}, min_point_y = {}",
        width, height, min_point, min_point_x, min_point_y
    );
    return [min_point_x, min_point_y];
}

fn read_thermal(image_path: &str, min_point_x: usize, min_point_y: usize) {
    let file_path = Path::new(image_path);
    // let file_path = Path::new("test.jpg");
    let r_kelvin = try_parse_flir(file_path).unwrap().celsius();
    let raw_data = try_parse_flir(file_path).unwrap().raw_data_read;
    println!("{:?}", r_kelvin.shape());
    if r_kelvin.shape() == [480, 640] {
        println!(
            "480, 640 {}",
            r_kelvin[[min_point_y + 24, min_point_x + 24]]
        );
        println!(
            "480, 640 {}",
            r_kelvin[[480 - min_point_y - 24, 640 - min_point_x - 24]]
        );
        println!(
            "480, 640 {}",
            r_kelvin[[480 - min_point_y - 24, min_point_x + 24]]
        );
        println!(
            "480, 640 {}",
            r_kelvin[[min_point_y + 24, 640 - min_point_x - 24]]
        );

        println!(
            "640, 480 {}",
            r_kelvin[[min_point_x + 24, min_point_y + 24]]
        );
        println!(
            "640, 480 {}",
            r_kelvin[[640 - min_point_x - 24, 480 - min_point_y - 24]]
        );
        println!(
            "640, 480 {}",
            r_kelvin[[640 - min_point_x - 24, min_point_y + 24]]
        );
        println!(
            "640, 480 {}",
            r_kelvin[[min_point_x + 24, 480 - min_point_y - 24]]
        );
    } else {
    }
    let mut r_max = f32::MIN;
    let mut r_min = f32::MAX;
    for r in &r_kelvin {
        if r < &r_min {
            r_min = *r;
        }
        if r > &r_max {
            r_max = *r;
        }
    }
    println!("r_min: {}, r_max: {}", r_min, r_max);
    println!("raw data 0,0: {}", &raw_data[[0,0]]);
    println!("{}", &r_kelvin[[0,0]]);
}

#[cfg(test)]
pub mod tests;
