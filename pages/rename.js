"use client";
import React, { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";
import { open, ask } from "@tauri-apps/api/dialog";
import { convertFileSrc } from "@tauri-apps/api/tauri";

import BaseContainer from "./util/container";
import Button from "./util/Button";
import {
  ChangeExcelData,
  GetDataReference,
  GetExcelsData,
  SelectFolder,
} from "./util/Excel";

async function StartConvert(
  SourceFolder,
  TargetFolder,
  DataReference,
  ExcelData,
  setConvertState
) {
  let index = 0;
  for (const data of DataReference) {
    const SourceFileName = data.source_image_name;
    const TargetFileName = data.target_image_name;
    setConvertState(
      `正在复制文件(已完成${index++}/${
        DataReference.length
      }) ${SourceFolder}/${SourceFileName} ${TargetFolder}/${TargetFileName}`
    );
    await invoke("copy_file", {
      from: `${SourceFolder}/${SourceFileName}`,
      to: `${TargetFolder}/${TargetFileName}`,
    });
  }
  try {
    await invoke("write_to_excel", {
      excel_datas: ChangeExcelData(ExcelData),
      save_path: `${TargetFolder}/表格数据.xlsx`,
    });
  } catch (error) {
    await ask("无法写入至表格，请检查该表格是否未关闭。", {
      title: "错误",
      type: "warning",
    });
  }
  await setConvertState(`转换完成`);
}
export default function Page() {
  const [TargetFolder, setTargetFolder] = useState("");
  const [SourceFolder, setSourceFolder] = useState("");
  const [ConvertState, setConvertState] = useState("空闲");
  const [ExcelData, setExcelData] = useState([]);
  const [DataReference, setDataReference] = useState([]);
  useEffect(() => {
    setDataReference(GetDataReference(ExcelData));
  }, [ExcelData]);
  return (
    <BaseContainer>
      <div className="grid grid-cols-4 gap-2">
        <Button
          handler={async () => await GetExcelsData(setExcelData)}
          name={"选择文件"}
          description={"选择待识别的Excel文件"}
        />
        <Button
          handler={async () => await SelectFolder(setSourceFolder)}
          name={"选择文件夹"}
          description={"选择待识别的图片文件夹"}
        />
        <Button
          handler={async () => await SelectFolder(setTargetFolder)}
          name={"选择文件夹"}
          description={"选择输出的图片文件夹"}
        />
        <Button
          handler={async () =>
            await StartConvert(
              SourceFolder,
              TargetFolder,
              DataReference,
              ExcelData,
              setConvertState
            )
          }
          name={"开始转换"}
          description={"按下进行文件名修改"}
        />
        <div className="border-collapse border border-green-800 table-auto col-span-4">
          当前状态：{ConvertState}
        </div>
        <table className="border-collapse border border-green-800 table-auto col-span-4">
          <thead>
            <tr>
              <th className="border border-green-800">
                待转换文件，共{DataReference.length}个
              </th>
              <th className="border border-green-800">
                输出文件，共{DataReference.length}个
              </th>
            </tr>
          </thead>
          <tbody>
            {DataReference.map((element, i) =>
              element.isSame ? (
                ""
              ) : (
                <tr key={i}>
                  <td className="border border-green-800">
                    {element.source_image_name}
                  </td>
                  <td className="border border-green-800">
                    {element.target_image_name}
                  </td>
                </tr>
              )
            )}
          </tbody>
        </table>
      </div>
    </BaseContainer>
  );
}
