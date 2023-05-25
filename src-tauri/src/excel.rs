use std::{vec, path::PathBuf};

use calamine::{open_workbook_auto, Reader};
use chrono::NaiveDate;
use rust_xlsxwriter::{Format, FormatAlign, FormatBorder, Image, Workbook, Worksheet, XlsxError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ExcelData {
    id: f64,
    interval_name: String,
    device_name: String,
    voltage_level: String,
    measurement_image: String,
    thermal: f64,
    normal_corresponding_point_temperature: f64,
    emissivity: f64,
    ambient_temperature: f64,
    temperature_difference: f64,
    temperature_rise: f64,
    distance: f64,
    load_current: f64
}

#[tauri::command(rename_all = "snake_case")]
pub fn write_to_excel(excel_datas: Vec<ExcelData>, save_path: &str) -> Result<(), ()> {
    let mut workbook: Workbook = Workbook::new();
    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    for (i, data) in excel_datas.iter().enumerate() {
        write_excel_line(worksheet, u32::try_from(i).unwrap(), data.clone());
    }

    match workbook.save(save_path) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(()),
    };
}

fn write_excel_line(
    worksheet: &mut Worksheet,
    row: u32,
    ExcelData {
        id,
        interval_name,
        device_name,
        voltage_level,
        measurement_image,
        thermal,
        emissivity,
        ambient_temperature,
        temperature_difference,
        temperature_rise,
        distance,
        load_current,
        normal_corresponding_point_temperature,
    }: ExcelData,
) {
    worksheet.write(row, 0, id).unwrap();
    worksheet.write(row, 1, interval_name).unwrap();
    worksheet.write(row, 2, device_name).unwrap();
    worksheet.write(row, 4, voltage_level).unwrap();
    worksheet.write(row, 10, measurement_image).unwrap();
    worksheet.write(row, 14, distance).unwrap();
    worksheet.write(row, 15, thermal).unwrap();
    worksheet.write(row, 16, normal_corresponding_point_temperature).unwrap();
    worksheet.write(row, 17, temperature_difference).unwrap();
    worksheet.write(row, 18, temperature_rise).unwrap();
    worksheet.write(row, 19, ambient_temperature).unwrap();
    worksheet.write(row, 20, emissivity).unwrap();
    worksheet.write(row, 21, load_current).unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_excel_lines(excel_path: &str) -> Result<Vec<ExcelData>, String> {
    let path = excel_path.to_string();
    let _workbook_result = match open_workbook_auto(path) {
        Ok(workbook) => {
            let mut workbook = workbook;
            let mut data = Vec::new();
            for (sheetname, worksheet) in workbook.worksheets() {
                println!("Sheetname is: {}", sheetname);
                for row in worksheet.rows() {
                    println!("row={:?}, row[0]={:?} row.len()={}", row, row[0], row.len());
                    if row[0].get_float() != None && row.len() >= 22 {
                        data.push(ExcelData {
                            id: row[0].get_float().unwrap_or_default(),
                            interval_name: row[1].get_string().unwrap_or_default().to_string(),
                            device_name: row[2].get_string().unwrap_or_default().to_string().replace("/", "").replace("\\", "").replace("?", "").replace("*", ""),
                            voltage_level: row[4].get_string().unwrap_or_default().to_string(),
                            measurement_image: format!("{}.jpg", row[2].get_string().unwrap_or_default()),
                            distance: row[14].get_float().unwrap_or(1.0f64),
                            thermal: row[15].get_float().unwrap_or(-99999f64),
                            normal_corresponding_point_temperature: row[16].get_float().unwrap_or(-99999f64),
                            temperature_difference: row[17].get_float().unwrap_or(-99999f64),
                            temperature_rise: row[18].get_float().unwrap_or(-99999f64),
                            ambient_temperature: row[19].get_float().unwrap_or(-99999f64),
                            emissivity: row[20].get_float().unwrap_or(0.9f64),
                            load_current: row[21].get_float().unwrap_or(0f64),
                        });
                        println!(
                            "row[0] = {:?}\trow[2]={:?}\trow[4]={:?}",
                            row[0].get_float(),
                            row[2].to_string(),
                            row[4].get_string(),
                        );
                    }
                }
            }
            return Ok(data);
        }
        Err(e) => return Err(From::from(e.to_string())),
    };
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_image_names(excel_path: &str) -> Result<Vec<String>, String> {
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
