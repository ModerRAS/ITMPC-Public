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
    thermal: f32,
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
    }: ExcelData,
) {
    worksheet.write(row, 0, id).unwrap();
    worksheet.write(row, 1, interval_name).unwrap();
    worksheet.write(row, 2, device_name).unwrap();
    worksheet.write(row, 4, voltage_level).unwrap();
    worksheet.write(row, 10, measurement_image).unwrap();
    worksheet.write(row, 15, thermal).unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_excel_lines(excel_path: &str) -> Result<Vec<ExcelData>, String> {
    let path = excel_path.to_string();
    let _workbook_result = match open_workbook_auto(path) {
        Ok(workbook) => {
            let mut workbook = workbook;
            let mut data = Vec::new();
            if let Some(Ok(r)) = workbook.worksheet_range_at(1) {
                for row in r.rows() {
                    // println!("row={:?}, row[0]={:?}", row, row[0]);
                    if row[0].get_float() != None {
                        data.push(ExcelData {
                            id: row[0].get_float().unwrap(),
                            interval_name: row[1].get_string().unwrap().to_string(),
                            device_name: row[2].get_string().unwrap().to_string(),
                            voltage_level: row[4].get_string().unwrap().to_string(),
                            measurement_image: format!("{}.jpg", row[2].get_string().unwrap()),
                            thermal: -99999f32,
                        });
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
