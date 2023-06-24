"use client";
import React, { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";
import { open, ask } from "@tauri-apps/api/dialog";
import { convertFileSrc } from '@tauri-apps/api/tauri';

import BaseContainer from "./util/container";
import Button from "./util/Button";
import { GetExcelsData, SelectSaveExcelPath } from "../lib/Excel";

async function StartConvert(SourceExcelData, TargetExcelData, TargetPath, setConvertState) {

  await setConvertState(`合并中。。。`);

  let SourceExcelDataMapId = new Map();
  let SourceExcelDataMapName = new Map();
  let retdata = []

  let not_found_data = []

  for (const ExcelData of SourceExcelData) {
    SourceExcelDataMapId.set(ExcelData.device_id, ExcelData)
    SourceExcelDataMapName.set(ExcelData.device_name, ExcelData)
  }



  for (const ExcelData of TargetExcelData) {
    let data_id = SourceExcelDataMapId.get(ExcelData.device_id)
    let data_name = SourceExcelDataMapName.get(ExcelData.device_name)
    // console.log(`data: ${data}, ExcelData: ${ExcelData}`)
    // console.log(data)
    // console.log(ExcelData)
    if (data_id) {
      retdata.push(data_id)
    } else if (data_name) {
      retdata.push(data_name)
    } else {
      not_found_data.push(ExcelData)
    }
  }

  console.log(retdata)



  await invoke("write_to_excel", {excel_datas: retdata, save_path: TargetPath})
  await setConvertState(`合并完成`);
  if (not_found_data.length > 0) {
    let printdata = []
    for (const ExcelData of not_found_data) {
      printdata.push(ExcelData.device_name)
    }
    await setConvertState(`未找到以下数据(共${printdata.length}个): ${JSON.stringify(printdata)}`)
  }
}



export default function Page() {
  const [TargetPath, setTargetPath] = useState("");
  const [ConvertState, setConvertState] = useState("空闲");
  const [TargetExcelData, setTargetExcelData] = useState([]);
  const [SourceExcelData, setSourceExcelData] = useState([]);
  return (
    <BaseContainer>
      <div className="grid grid-cols-4 gap-2">
        <Button
          handler={async () => await GetExcelsData(setTargetExcelData)}
          name={"选择文件"}
          description={"选择PMS下载的模板Excel文件"}
        />
        <Button
          handler={async () => await GetExcelsData(setSourceExcelData)}
          name={"选择文件"}
          description={"选择待合并的Excel文件"}
        />
        <Button
          handler={async () => await SelectSaveExcelPath(setTargetPath)}
          name={"保存为文件"}
          description={"保存为Excel文件"}
        />
        <Button
          handler={async () => {
            await StartConvert(SourceExcelData, TargetExcelData, TargetPath, setConvertState)
          }}
          name={"开始转换"}
          description={"按下进行Excel合并"}
        />
        <div className="border-collapse border border-green-800 table-auto col-span-4">
          当前状态：{ConvertState}
        </div>
      </div>
    </BaseContainer>
  );
}
