"use client";
import React, { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";
import { open, ask } from "@tauri-apps/api/dialog";
import { convertFileSrc } from "@tauri-apps/api/tauri";

import BaseContainer from "./util/container";
import Button from "./util/Button";
import { GetExcelsData, SelectFolder } from "./util/Excel";

async function StartRepatch({
  setConvertState,
  SourceExcelData,
  MissingLinesData,
  TargetFolder,
}) {
  let missing_data = new Array();
  let patch_data = new Array();
  let still_missing_data = new Array();
  for (const source of SourceExcelData) {
    if (MissingLinesData.find((e) => e.device_id == source.device_id)) {
    } else {
      missing_data.push(source);
    }
  }
  let all_data = await invoke("get_data");

  for (const missing of missing_data) {
    let is_found = false;
    for (const data of all_data.datas) {
      let found = data.excel_datas.find(
        (e) => e.device_id == missing.device_id
      );
      if (found) {
        patch_data.push({ data: found, image_path: data.image_path });
        is_found = true;
        break;
      }
    }
    if (!is_found) {
      still_missing_data.push(missing);
    }
  }
  console.log(missing_data);
  console.log(still_missing_data);
  console.log(patch_data);
  let patched_data = Object.assign([], MissingLinesData);
  for (const data of patch_data) {
    patched_data.push(data.data);
  }
  console.log(patched_data);
  for (const data of patch_data) {
    let source_image_path = `${data.image_path}/${data.data.measurement_image}`;
    let target_image_path = `${TargetFolder}/${data.data.measurement_image}`;
    await invoke("copy_file", {
      from: source_image_path,
      to: target_image_path,
    });
  }
  try {
    await invoke("write_to_excel", {
      excel_datas: patched_data,
      save_path: `${TargetFolder}/表格数据.xlsx`,
    });
  } catch (error) {
    await ask("无法写入至表格，请检查该表格是否未关闭。", {
      title: "错误",
      type: "warning",
    });
  }
  setConvertState(
    `填充完成，仍然缺失的数据：${JSON.stringify(
      still_missing_data.map((e) => e.device_name)
    )}`
  );
}

export default function Page() {
  const [TargetFolder, setTargetFolder] = useState("");
  const [MissingLinesData, setMissingLinesData] = useState([""]);
  const [ConvertState, setConvertState] = useState("空闲");
  const [SourceExcelData, setSourceExcelData] = useState([]);
  useEffect(() => {});
  return (
    <BaseContainer>
      <div className="grid grid-cols-4 gap-2">
        <Button
          handler={async () => await GetExcelsData(setSourceExcelData)}
          name={"选择文件"}
          description={"选择PMS下载的Excel文件"}
        />
        <Button
          handler={async () => await GetExcelsData(setMissingLinesData)}
          name={"选择文件"}
          description={"选择缺失数据的Excel文件"}
        />
        <Button
          handler={async () => await SelectFolder(setTargetFolder)}
          name={"选择文件夹"}
          description={"选择输出Excel和缺失数据的文件夹"}
        />
        <Button
          handler={async () =>
            await StartRepatch({
              setConvertState: setConvertState,
              SourceExcelData: SourceExcelData,
              MissingLinesData: MissingLinesData,
              TargetFolder: TargetFolder,
            })
          }
          name={"开始重建"}
          description={"按下进行重建"}
        />
        <div className="border-collapse border border-green-800 table-auto col-span-4">
          当前状态：{ConvertState} <br></br>
          源Excel数据: {SourceExcelData.length} 条 <br />
          待填充数据: {MissingLinesData.length} 条 <br />
          目标路径： {TargetFolder} <br />
        </div>
      </div>
    </BaseContainer>
  );
}
