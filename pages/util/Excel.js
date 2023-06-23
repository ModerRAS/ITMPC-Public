import { invoke } from "@tauri-apps/api/tauri";
import { open, save, ask } from "@tauri-apps/api/dialog";
import { convertFileSrc } from "@tauri-apps/api/tauri";

async function GetExcelsData(setExcelData) {
  let file = await open({
    title: "选择Excel文件",
    multiple: true,
    filters: [
      {
        name: "Excel",
        extensions: ["xls", "xlsx"],
      },
    ],
  });
  console.log(file);
  let data = new Array();
  if (file) {
    for (let index = 0; index < file.length; index++) {
      const f = file[index];
      let lines = await invoke("get_excel_lines", { excel_path: f });
      lines.forEach((line) => {
        data.push(line);
      });
    }

    console.log(data);
    setExcelData(data);
  }
}

async function SelectFolder(setFolder) {
  let file = await open({
    title: "选择文件夹",
    directory: true,
  });
  console.log(file);
  setFolder(file);
}

async function SelectSaveExcelPath(setPath) {
  let file = await save({
    title: "保存为",
    filters: [
      {
        name: "Excel",
        extensions: ["xlsx"],
      },
    ],
  });
  console.log(file);
  setPath(file);
}

function GetDataReference(ExcelData) {
  return ExcelData.map((e) => {
    let source_image_name = e.measurement_image
      .replaceAll("/", "")
      .replaceAll("\\", "")
      .replaceAll("?", "")
      .replaceAll("*", "");
    let target_image_name = `${e.device_name
      .replaceAll("/", "")
      .replaceAll("\\", "")
      .replaceAll("?", "")
      .replaceAll("*", "")}.jpg`;
    return {
      device_name: e.device_name,
      isSame: source_image_name == target_image_name,
      source_image_name: source_image_name,
      target_image_name: target_image_name,
    };
  });
}

function ChangeExcelData(ExcelData) {
  return ExcelData.map((e) => {
    let target_image_name = `${e.device_name
      .replaceAll("/", "")
      .replaceAll("\\", "")
      .replaceAll("?", "")
      .replaceAll("*", "")}.jpg`;
    let data = { measurement_image: target_image_name, ...e };
    return data;
  });
}

export {
  GetExcelsData,
  SelectFolder,
  SelectSaveExcelPath,
  GetDataReference,
  ChangeExcelData,
};
