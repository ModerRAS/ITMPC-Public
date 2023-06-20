use std::{path::PathBuf, fs};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tokio::fs::{read_to_string, write};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MatchedData {
    pub matched: Vec<ExcelData>,
    pub unmatched: Vec<ExcelData>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ExcelData {
    pub id: f64,
    pub interval_name: String,
    pub device_name: String,
    pub device_id: String,
    pub voltage_level: String,
    pub detection_point_id: String,
    pub measurement_image: String,
    pub thermal: f64,
    pub normal_corresponding_point_temperature: f64,
    pub emissivity: f64,
    pub ambient_temperature: f64,
    pub temperature_rise: f64,
    pub distance: f64,
    pub load_current: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Data {
    pub image_path: String,
    pub excel_datas: Vec<ExcelData>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct JsonData {
    pub datas: Vec<Data>,
}

impl JsonData {
    pub async fn load() -> Result<JsonData, ()> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "com.miaostay.itmpc", "jsondata") {
            fs::create_dir_all(proj_dirs.data_local_dir()).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
            let json_data_path = proj_dirs.data_local_dir().join("data.json");
            match serde_json::from_str::<JsonData>(
                &read_to_string(&json_data_path).await.unwrap_or_default(),
            ) {
                Ok(o) => return Ok(o),
                Err(e) => println!("{:?}", e),
            }
        }
        return Err(());
    }
    pub async fn save(&mut self) {
        if let Some(proj_dirs) = ProjectDirs::from("com", "com.miaostay.itmpc", "jsondata") {
            fs::create_dir_all(proj_dirs.data_local_dir()).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
            let json_data_path = proj_dirs.data_local_dir().join("data.json");
            match write(
                json_data_path,
                serde_json::to_string(self).unwrap_or_default(),
            )
            .await
            {
                Ok(_) => {}
                Err(e) => println!("{:?}", e),
            };
        }
    }

    pub async fn push(&mut self, image_path: PathBuf, excel_datas: Vec<ExcelData>) {
        self.push_str(String::from(image_path.to_str().unwrap_or_default()), excel_datas).await;
    }
    pub async fn push_str(&mut self, image_path: String, excel_datas: Vec<ExcelData>) {
        self.datas.push(Data {
            image_path: image_path,
            excel_datas: excel_datas,
        });
        self.save().await;
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn import_data(image_path: String, excel_datas: Vec<ExcelData>) {
    match JsonData::load().await {
        Ok(mut o) => o.push_str(image_path, excel_datas).await,
        Err(_) => {
            let mut json_data = JsonData {
                datas: vec![Data {
                    image_path: image_path,
                    excel_datas,
                }],
            };
            json_data.save().await;
        },
    };

}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_data() -> Result<JsonData, ()> {
    match JsonData::load().await {
        Ok(o) => return Ok(o),
        Err(e) => return Err(e),
    };

}
