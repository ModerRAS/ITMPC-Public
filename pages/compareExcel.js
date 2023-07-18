"use client";
import React, { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";
import { open, ask } from "@tauri-apps/api/dialog";
import { convertFileSrc } from "@tauri-apps/api/tauri";

import BaseContainer from "./util/container";
import Button from "./util/Button";
import { GetExcelsData, SelectFolder } from "../lib/Excel";

async function StartRepatch({
  setConvertState,
  TargetFolder: TargetFolder,
  OldData,
  NewData,
  setDiffLost: setDiffLost,
  setDiffMore: setDiffMore,
}) {
  let old_data = new Map();
  let new_data = new Map();
  OldData.forEach((data) => {
    old_data.set(data.device_id, data);
  });
  NewData.forEach((data) => {
    new_data.set(data.device_id, data);
  });
  let diff_lost = new Array();
  let diff_more = new Array();
  NewData.forEach((data) => {
    if (!old_data.has(data.device_id)) {
      diff_lost.push(data);
    }
  });
  OldData.forEach((data) => {
    if (!new_data.has(data.device_id)) {
      diff_more.push(data);
    }
  });
  setDiffLost(JSON.stringify(diff_lost));
  setDiffMore(JSON.stringify(diff_more))
  try {
    await invoke("write_to_excel", {
      excel_datas: diff_lost,
      save_path: `${TargetFolder}/缺少数据.xlsx`,
    });
  } catch (error) {
    await ask("无法写入至表格，请检查该表格是否未关闭。", {
      title: "错误",
      type: "warning",
    });
  }
  try {
    await invoke("write_to_excel", {
      excel_datas: diff_more,
      save_path: `${TargetFolder}/多余数据.xlsx`,
    });
  } catch (error) {
    await ask("无法写入至表格，请检查该表格是否未关闭。", {
      title: "错误",
      type: "warning",
    });
  }
//   setConvertState(
//     `旧Excel缺失的数据：${JSON.stringify(
//       diff_lost
//     )}\n旧Excel多余的数据：${JSON.stringify(diff_more)}`
//   );
}

export default function Page() {
  const [TargetFolder, setTargetFolder] = useState("");
  const [NewData, setNewData] = useState([""]);
  const [ConvertState, setConvertState] = useState("空闲");
  const [OldData, setOldData] = useState([]);
  const [DiffLost, setDiffLost] = useState("");
  const [DiffMore, setDiffMore] = useState("");
  useEffect(() => {});
  return (
    <BaseContainer>
      <div className="grid grid-cols-4 gap-2">
        <Button
          handler={async () => await GetExcelsData(setOldData)}
          name={"选择文件"}
          description={"选择老的PMS下载的Excel文件"}
        />
        <Button
          handler={async () => await GetExcelsData(setNewData)}
          name={"选择文件"}
          description={"选择新的PMS下载的Excel文件"}
        />
        <Button
          handler={async () => await SelectFolder(setTargetFolder)}
          name={"选择文件夹"}
          description={"选择输出的图片文件夹"}
        />
        <Button
          handler={async () =>
            await StartRepatch({
              setConvertState: setConvertState,
              TargetFolder: TargetFolder,
              OldData: OldData,
              NewData: NewData,
              setDiffLost: setDiffLost,
              setDiffMore: setDiffMore,
            })
          }
          name={"开始比对"}
          description={"按下进行比对"}
        />
        <div className="border-collapse border border-green-800 table-auto col-span-4">
          当前状态：{ConvertState} <br></br>
        </div>
        <div className="border-collapse border border-green-800 table-auto col-span-4">
          {DiffLost} <br></br>
        </div>
        <div className="border-collapse border border-green-800 table-auto col-span-4">
          {DiffMore} <br></br>
        </div>
      </div>
    </BaseContainer>
  );
}
