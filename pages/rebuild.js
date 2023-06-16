"use client";
import React, { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";
import { open, ask } from "@tauri-apps/api/dialog";
import { convertFileSrc } from "@tauri-apps/api/tauri";

import BaseContainer from "./util/container";
import Button from "./util/Button";

async function SelectExcel(setSourceExcelData, setTargetFileNames) {
  let file = await open({
    title: "选择待打开的Excel文件",
    filters: [
      {
        name: "Excel",
        extensions: ["xls", "xlsx"],
      },
    ],
  });
  console.log(file);
  let lines = await invoke("get_excel_lines", { excel_path: file });
  let names = lines.map((line) => {
    return line.device_name;
  });
  setTargetFileNames(names);
  console.log(names);
  console.log(lines);
  setSourceExcelData(lines);
  return names;
}

async function SelectSourceFolder() {
  let file = await open({
    title: "选择图片文件夹",
    directory: true,
  });
  console.log(file);
  let names = await invoke("get_image_from_directory", { source: file });
  console.log(names);
  return names;
}

async function SelectSourceFiles(setRematchExcelData) {
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
      lines.forEach(line => {
        data.push(line);
      });
    }

    console.log(data);
    setRematchExcelData(data);
  }
}

async function SelectTargetFolder(setTargetFolder) {
  let file = await open({
    title: "选择输出文件夹",
    directory: true,
  });
  console.log(file);
  setTargetFolder(file);
}

async function StartConvert(
  SourceExcelData,
  RematchExcelData,
  TargetFolder,
  setConvertState
) {
  try {
    let matched_data = await invoke("fix_missing_field", {source:SourceExcelData, missing_field_data: RematchExcelData});
    console.log(matched_data);
    await invoke("write_to_excel", {
      excel_datas: matched_data.matched,
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

function MergeLines(SourceExcelData, RematchExcelData) {
  let ret = [];
  console.log("SourceExcelData");
  console.log(SourceExcelData);
  if (SourceExcelData.length <= 0) {
    return [];
  }
  let min_length =
  SourceExcelData.length > RematchExcelData.length
      ? RematchExcelData.length
      : SourceExcelData.length;
  for (let index = 0; index < min_length; index++) {
    const Source = SourceExcelData[index];
    const Rematch = RematchExcelData[index];
    ret.push({
      Source: Source,
      Rematch: Rematch,
    });
  }

  return ret;
}

export default function Page() {
  const [SourceFolderFiles, setSourceFolderFiles] = useState([""]);
  const [TargetFolder, setTargetFolder] = useState("");
  const [TargetFileNames, setTargetFileNames] = useState([""]);
  const [AllFilePaths, setAllFilePaths] = useState([[""]]);
  const [ConvertState, setConvertState] = useState("空闲");
  const [SourceExcelData, setSourceExcelData] = useState([]);
  const [RematchExcelData, setRematchExcelData] = useState([]);
  useEffect(() => {
    setAllFilePaths(
      MergeLines(SourceExcelData, RematchExcelData)
    );
  }, [SourceExcelData, RematchExcelData]);
  return (
    <BaseContainer>
      <div className="grid grid-cols-4 gap-2">
        <Button
          handler={async () =>
            await SelectExcel(setSourceExcelData, setTargetFileNames)
          }
          name={"选择文件"}
          description={"选择原始Excel文件"}
        />
        <Button
          handler={async () => await SelectSourceFiles(setRematchExcelData)}
          name={"选择文件"}
          description={"选择待重建的Excel文件"}
        />
        <Button
          handler={async () => await SelectTargetFolder(setTargetFolder)}
          name={"选择文件夹"}
          description={"选择输出Excel的文件夹"}
        />
        <Button
          handler={
            async () =>
              await StartConvert(
                SourceExcelData,
                RematchExcelData,
                TargetFolder,
                setConvertState
              )
            // await TestTesseract()
          }
          name={"开始重建"}
          description={"按下进行重建"}
        />
        <div className="border-collapse border border-green-800 table-auto col-span-4">
          当前状态：{ConvertState}
        </div>
        <table className="border-collapse border border-green-800 table-auto col-span-4">
          <thead>
            <tr>
              <th className="border border-green-800">
                待转换条目，共{SourceExcelData.length}个
              </th>
              <th className="border border-green-800">
                输出条目，共{RematchExcelData.length}个
              </th>
            </tr>
          </thead>
          <tbody>
            {AllFilePaths.map((element, i) => (
              <tr key={i}>
                <td className="border border-green-800">
                  {JSON.stringify(element.Source)}
                </td>
                <td className="border border-green-800">
                  {JSON.stringify(element.Rematch)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </BaseContainer>
  );
}
