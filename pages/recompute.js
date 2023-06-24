"use client";
import React, { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";
import { open, ask } from "@tauri-apps/api/dialog";
import { convertFileSrc } from "@tauri-apps/api/tauri";

import BaseContainer from "./util/container";
import Button from "./util/Button";
import { GetExcelsData, SelectSaveExcelPath } from "../lib/Excel";


async function StartConvert(
  ExcelData,
  TargetPath,
  setConvertState
) {
    await setConvertState(`开始重算`);
  try {
    let excel_datas = []
    for (const data of ExcelData) {
      excel_datas.push(await invoke("reprocess_excel_data", {excel_data: data}))
    }
    console.log(excel_datas);
    await invoke("write_to_excel", {
      excel_datas: excel_datas,
      save_path: TargetPath,
    });
  } catch (error) {
    await ask("无法写入至表格，请检查该表格是否未关闭。", {
      title: "错误",
      type: "warning",
    });
  }
  await setConvertState(`重算完成`);
}

export default function Page() {
  const [TargetPath, setTargetPath] = useState("");
  const [ConvertState, setConvertState] = useState("空闲");
  const [ExcelData, setExcelData] = useState([]);
  return (
    <BaseContainer>
      <div className="grid grid-cols-4 gap-2">
        <Button
          handler={async () => await GetExcelsData(setExcelData)}
          name={"选择文件"}
          description={"选择待重算数据的Excel文件"}
        />
        <Button
          handler={async () => await SelectSaveExcelPath(setTargetPath)}
          name={"保存为"}
          description={"选择输出Excel的文件"}
        />
        <Button
          handler={
            async () =>
              await StartConvert(
                ExcelData,
                TargetPath,
                setConvertState
              )
            // await TestTesseract()
          }
          name={"开始重算"}
          description={"按下进行重建"}
        />
        <div className="border-collapse border border-green-800 table-auto col-span-4">
          当前状态：{ConvertState}
        </div>
      </div>
    </BaseContainer>
  );
}
