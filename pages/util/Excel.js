
import { invoke } from "@tauri-apps/api/tauri";
import { open, ask } from "@tauri-apps/api/dialog";
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
    title: "选择输出文件夹",
    directory: true,
  });
  console.log(file);
  setFolder(file);
}

export { GetExcelsData, SelectFolder };
