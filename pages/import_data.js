"use client";
import React, { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";
import { open, ask } from "@tauri-apps/api/dialog";
import { convertFileSrc } from "@tauri-apps/api/tauri";

import BaseContainer from "./util/container";
import Button from "./util/Button";

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

async function StartRepatch({
  setConvertState,
  SourceExcelData,
  FullLinesDataImageFolder,
}) {
    setConvertState("开始导入")
    await invoke("import_data", { image_path: FullLinesDataImageFolder, excel_datas: SourceExcelData });
    setConvertState("结束")
}

export default function Page() {
  const [FullLinesDataImageFolder, setFullLinesDataImageFolder] = useState([
    "",
  ]);
  const [ConvertState, setConvertState] = useState("空闲");
  const [SourceExcelData, setSourceExcelData] = useState([]);
  useEffect(() => {
  });
  return (
    <BaseContainer>
      <div className="grid grid-cols-4 gap-2">
        <Button
          handler={async () =>
            await GetExcelsData(setSourceExcelData)
          }
          name={"选择文件"}
          description={"选择导入的Excel文件"}
        />
        <Button
          handler={async () => await SelectFolder(setFullLinesDataImageFolder)}
          name={"选择文件夹"}
          description={"选择导入数据的图片的文件夹"}
        />
        <Button
          handler={
            async () =>
              await StartRepatch({
                setConvertState: setConvertState,
                SourceExcelData: SourceExcelData,
                FullLinesDataImageFolder: FullLinesDataImageFolder,
              })
          }
          name={"开始导入"}
          description={"按下进行导入"}
        />
        <div className="border-collapse border border-green-800 table-auto col-span-4">
          当前状态：{ConvertState}
        </div>
      </div>
    </BaseContainer>
  );
}
