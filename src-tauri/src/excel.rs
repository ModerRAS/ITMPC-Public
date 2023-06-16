use std::collections::HashMap;

use calamine::{open_workbook_auto, Reader};
use rust_xlsxwriter::{Workbook, Worksheet};
use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MatchedData {
    matched: Vec<ExcelData>,
    unmatched: Vec<ExcelData>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ExcelData {
    id: f64,
    interval_name: String,
    device_name: String,
    device_id: String,
    voltage_level: String,
    detection_point_id: String,
    measurement_image: String,
    thermal: f64,
    normal_corresponding_point_temperature: f64,
    emissivity: f64,
    ambient_temperature: f64,
    temperature_rise: f64,
    distance: f64,
    load_current: f64,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn write_to_excel(excel_datas: Vec<ExcelData>, save_path: &str) -> Result<(), ()> {
    let mut workbook: Workbook = Workbook::new();
    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    for (i, data) in excel_datas.iter().enumerate() {
        write_excel_line(
            worksheet,
            u32::try_from(i).unwrap(),
            process_excel_data(data.clone()).await,
        );
    }

    match workbook.save(save_path) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(()),
    };
}

#[tauri::command(rename_all = "snake_case")]
pub async fn process_excel_data(
    ExcelData {
        id,
        interval_name,
        device_name,
        device_id,
        voltage_level,
        detection_point_id,
        measurement_image,
        thermal,
        normal_corresponding_point_temperature,
        emissivity,
        ambient_temperature,
        temperature_rise,
        distance,
        load_current,
    }: ExcelData,
) -> ExcelData {
    let distance = if voltage_level.starts_with("直流") {
        9f64
    } else {
        match voltage_level.as_str() {
            "交流1000kV" => 9f64,
            "交流500kV" => 6f64,
            _ => 3f64,
        }
    };

    let normal_corresponding_point_temperature =
        if normal_corresponding_point_temperature == -99999f64 && thermal != -99999f64 {
            thermal
        } else {
            normal_corresponding_point_temperature
        };

    let temperature_rise = if temperature_rise == -99999f64
        && ambient_temperature != -99999f64
        && thermal != -99999f64
    {
        thermal - ambient_temperature
    } else {
        temperature_rise
    };

    let emissivity = if emissivity == -99999f64 {
        0.9
    } else {
        emissivity
    };

    let load_current = if load_current == -99999f64 {
        0f64
    } else {
        load_current
    };

    return ExcelData {
        id: id,
        interval_name: interval_name,
        device_name: device_name,
        device_id: device_id,
        voltage_level: voltage_level,
        detection_point_id: detection_point_id,
        measurement_image: measurement_image,
        thermal: thermal,
        normal_corresponding_point_temperature: normal_corresponding_point_temperature,
        emissivity: emissivity,
        ambient_temperature: ambient_temperature,
        temperature_rise: temperature_rise,
        distance: distance,
        load_current: load_current,
    };
}

#[tauri::command(rename_all = "snake_case")]
pub async fn fix_missing_field(
    source: Vec<ExcelData>,
    missing_field_data: Vec<ExcelData>,
) -> MatchedData {
    let mut source_map = HashMap::new();
    for s in source {
        source_map.insert(s.device_name.clone(), s);
    }
    let mut matched_data: Vec<ExcelData> = Vec::new();
    let mut unmatched_data: Vec<ExcelData> = Vec::new();
    for r in missing_field_data {
        match source_map.get(&r.device_name) {
            Some(s) => matched_data.push(ExcelData {
                id: s.id,
                interval_name: s.interval_name.clone(),
                device_name: s.device_name.clone(),
                device_id: s.device_id.clone(),
                voltage_level: s.voltage_level.clone(),
                detection_point_id: s.detection_point_id.clone(),
                measurement_image: r.measurement_image.clone(),
                thermal: r.thermal,
                normal_corresponding_point_temperature: r.normal_corresponding_point_temperature,
                emissivity: r.emissivity,
                ambient_temperature: r.ambient_temperature,
                temperature_rise: r.temperature_rise,
                distance: r.distance,
                load_current: r.load_current,
            }),
            None => {
                unmatched_data.push(r.clone());
            }
        }
    }
    return MatchedData {
        matched: matched_data,
        unmatched: unmatched_data
    };
}

#[tauri::command(rename_all = "snake_case")]
pub async fn rematch_excel_data(
    source: Vec<ExcelData>,
    rematch_data: Vec<ExcelData>,
) -> MatchedData {
    let mut source_map = HashMap::new();
    for s in source {
        source_map.insert(s.device_id.clone(), s);
    }
    let mut matched_data: Vec<ExcelData> = Vec::new();
    let mut unmatched_data: Vec<ExcelData> = Vec::new();
    for r in rematch_data {
        match source_map.get(&r.device_id) {
            Some(s) => matched_data.push(ExcelData {
                id: s.id,
                interval_name: s.interval_name.clone(),
                device_name: s.device_name.clone(),
                device_id: s.device_id.clone(),
                voltage_level: s.voltage_level.clone(),
                detection_point_id: s.detection_point_id.clone(),
                measurement_image: r.measurement_image.clone(),
                thermal: r.thermal,
                normal_corresponding_point_temperature: r.normal_corresponding_point_temperature,
                emissivity: r.emissivity,
                ambient_temperature: r.ambient_temperature,
                temperature_rise: r.temperature_rise,
                distance: r.distance,
                load_current: r.load_current,
            }),
            None => {
                unmatched_data.push(r.clone());
            }
        }
    }
    return MatchedData {
        matched: matched_data,
        unmatched: unmatched_data
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
        temperature_rise,
        distance,
        load_current,
        normal_corresponding_point_temperature,
        device_id,
        detection_point_id,
    }: ExcelData,
) {
    worksheet.write(row, 0, id).unwrap();
    worksheet.write(row, 1, interval_name).unwrap();
    worksheet.write(row, 2, device_name).unwrap();
    worksheet.write(row, 3, device_id).unwrap();
    worksheet.write(row, 4, &voltage_level).unwrap();
    worksheet.write(row, 7, detection_point_id).unwrap();
    worksheet.write(row, 9, "整体").unwrap();
    worksheet.write(row, 10, measurement_image).unwrap();
    worksheet.write(row, 14, distance).unwrap();
    worksheet.write(row, 15, thermal).unwrap();
    worksheet
        .write(row, 16, normal_corresponding_point_temperature)
        .unwrap();
    worksheet.write(row, 18, temperature_rise).unwrap();
    worksheet.write(row, 19, ambient_temperature).unwrap();
    worksheet.write(row, 20, emissivity).unwrap();
    worksheet.write(row, 21, load_current).unwrap();
    worksheet.write(row, 23, "正常").unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_excel_lines(excel_path: &str) -> Result<Vec<ExcelData>, String> {
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
                            device_name: row[2].get_string().unwrap_or_default().to_string(),
                            device_id: row[3].get_string().unwrap_or_default().to_string(),
                            voltage_level: row[4].get_string().unwrap_or_default().to_string(),
                            detection_point_id: row[7].get_string().unwrap_or_default().to_string(),
                            measurement_image: row[10]
                                .get_string()
                                .unwrap_or(
                                    format!(
                                        "{}.jpg",
                                        row[2]
                                            .get_string()
                                            .unwrap_or_default()
                                            .to_string()
                                            .replace("/", "")
                                            .replace("\\", "")
                                            .replace("?", "")
                                            .replace("*", "")
                                    )
                                    .as_str(),
                                )
                                .to_string(),
                            distance: row[14].get_float().unwrap_or(-99999f64),
                            thermal: row[15].get_float().unwrap_or(-99999f64),
                            normal_corresponding_point_temperature: row[16]
                                .get_float()
                                .unwrap_or(-99999f64),
                            temperature_rise: row[18].get_float().unwrap_or(-99999f64),
                            ambient_temperature: row[19].get_float().unwrap_or(-99999f64),
                            emissivity: row[20].get_float().unwrap_or(-99999f64),
                            load_current: row[21].get_float().unwrap_or(-99999f64),
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
pub async fn get_image_names(excel_path: &str) -> Result<Vec<String>, String> {
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
}

#[cfg(test)]
mod tests {
    use tauri::async_runtime::block_on;

    use crate::excel::fix_missing_field;

    use super::{ExcelData, rematch_excel_data};

    #[test]
    fn test_rematch_excel_data() {
        let s_data_1 = ExcelData {
            id: 1.0,
            interval_name: "todo!()".to_string(),
            device_name: "todo!()".to_string(),
            device_id: "todo!()".to_string(),
            voltage_level: "交流220kV".to_string(),
            detection_point_id: "todo!()".to_string(),
            measurement_image: "todo!()".to_string(),
            thermal: 10.0,
            normal_corresponding_point_temperature: -99999f64,
            emissivity: 0.9,
            ambient_temperature: 20.0,
            temperature_rise: -99999f64,
            distance: -99999f64,
            load_current: 0.0,
        };
        let s_data_2 = ExcelData {
            id: 2.0,
            interval_name: "todo!()".to_string(),
            device_name: "todo!()3".to_string(),
            device_id: "todo!()".to_string(),
            voltage_level: "交流220kV".to_string(),
            detection_point_id: "todo!()".to_string(),
            measurement_image: "todo!()".to_string(),
            thermal: 10.0,
            normal_corresponding_point_temperature: -99999f64,
            emissivity: 0.9,
            ambient_temperature: 20.0,
            temperature_rise: -99999f64,
            distance: -99999f64,
            load_current: 0.0,
        };
        let r_data_1 = ExcelData {
            id: 1.0,
            interval_name: "todo!()".to_string(),
            device_name: "todo!()".to_string(),
            device_id: "todo!()".to_string(),
            voltage_level: "交流220kV".to_string(),
            detection_point_id: "todo!()4".to_string(),
            measurement_image: "todo!()".to_string(),
            thermal: 10.0,
            normal_corresponding_point_temperature: -99999f64,
            emissivity: 0.9,
            ambient_temperature: 20.0,
            temperature_rise: -99999f64,
            distance: -99999f64,
            load_current: 0.0,
        };
        let r_data_2 = ExcelData {
            id: 3.0,
            interval_name: "todo!()2".to_string(),
            device_name: "todo!()3".to_string(),
            device_id: "".to_string(),
            voltage_level: "交流220kV".to_string(),
            detection_point_id: "".to_string(),
            measurement_image: "todo!()".to_string(),
            thermal: 10.0,
            normal_corresponding_point_temperature: -99999f64,
            emissivity: 0.9,
            ambient_temperature: 20.0,
            temperature_rise: -99999f64,
            distance: -99999f64,
            load_current: 0.0,
        };
        let source = vec![s_data_1, s_data_2];
        let rematch_data = vec![r_data_1, r_data_2];
        let data = block_on(rematch_excel_data(source, rematch_data));
        println!("{:?}", data);
    }

    #[test]
    fn test_fix_missing_field() {
        let s_data_1 = ExcelData {
            id: 1.0,
            interval_name: "todo!()".to_string(),
            device_name: "todo!()".to_string(),
            device_id: "todo!()".to_string(),
            voltage_level: "交流220kV".to_string(),
            detection_point_id: "todo!()".to_string(),
            measurement_image: "todo!()".to_string(),
            thermal: 10.0,
            normal_corresponding_point_temperature: -99999f64,
            emissivity: 0.9,
            ambient_temperature: 20.0,
            temperature_rise: -99999f64,
            distance: -99999f64,
            load_current: 0.0,
        };
        let s_data_2 = ExcelData {
            id: 2.0,
            interval_name: "todo!()".to_string(),
            device_name: "todo!()3".to_string(),
            device_id: "todo!()".to_string(),
            voltage_level: "交流220kV".to_string(),
            detection_point_id: "todo!()".to_string(),
            measurement_image: "todo!()".to_string(),
            thermal: 10.0,
            normal_corresponding_point_temperature: -99999f64,
            emissivity: 0.9,
            ambient_temperature: 20.0,
            temperature_rise: -99999f64,
            distance: -99999f64,
            load_current: 0.0,
        };
        let r_data_1 = ExcelData {
            id: 1.0,
            interval_name: "todo!()".to_string(),
            device_name: "todo!()".to_string(),
            device_id: "todo!()".to_string(),
            voltage_level: "交流220kV".to_string(),
            detection_point_id: "todo!()4".to_string(),
            measurement_image: "todo!()".to_string(),
            thermal: 10.0,
            normal_corresponding_point_temperature: -99999f64,
            emissivity: 0.9,
            ambient_temperature: 20.0,
            temperature_rise: -99999f64,
            distance: -99999f64,
            load_current: 0.0,
        };
        let r_data_2 = ExcelData {
            id: 3.0,
            interval_name: "todo!()2".to_string(),
            device_name: "todo!()3".to_string(),
            device_id: "".to_string(),
            voltage_level: "交流220kV".to_string(),
            detection_point_id: "".to_string(),
            measurement_image: "todo!()".to_string(),
            thermal: 10.0,
            normal_corresponding_point_temperature: -99999f64,
            emissivity: 0.9,
            ambient_temperature: 20.0,
            temperature_rise: 1f64,
            distance: 3f64,
            load_current: 0.0,
        };
        let source = vec![s_data_1, s_data_2];
        let rematch_data = vec![r_data_1, r_data_2];
        let data = block_on(fix_missing_field(source, rematch_data));
        println!("{:?}", data);
    }
}
