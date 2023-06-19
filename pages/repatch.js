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
async function SelectExcelFiles(setExcelFiles) {
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
  setExcelFiles(file);
}
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
  TargetFileNames,
  MissingLinesData,
  FullLinesData,
  FullLinesDataImageFolder,
  TargetFolder,
}) {
  let missing_data = new Array();
  let patch_data = new Array();
  let still_missing_data = new Array();
  for (const source of SourceExcelData) {
    if (MissingLinesData.find(e => e.device_id == source.device_id)) {

    } else {
      missing_data.push(source)
    }
  }
  for (const missing of missing_data) {
    let found = FullLinesData.find(e => e.device_id == missing.device_id)
    if (found) {
      patch_data.push(found)
    }
    else {
      console.log("missing data")
      console.log(missing)
      still_missing_data.push(missing)
    }
  }
  console.log(missing_data)
  console.log(still_missing_data)

}

async function StartConvert(
  SourceExcelData,
  RematchExcelData,
  TargetFolder,
  setConvertState
) {
  try {
    let matched_data = await invoke("fix_missing_field", {
      source: SourceExcelData,
      missing_field_data: RematchExcelData,
    });
    let excel_datas = [];
    for (const data of matched_data.matched) {
      excel_datas.push(
        await invoke("process_excel_data", { excel_data: data })
      );
    }
    console.log(matched_data);
    await invoke("write_to_excel", {
      excel_datas: excel_datas,
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
  const [MissingLinesData, setMissingLinesData] = useState([""]);
  const [FullLinesData, setFullLinesData] = useState([""]);
  const [FullLinesDataImageFolder, setFullLinesDataImageFolder] = useState([
    "",
  ]);
  const [AllFilePaths, setAllFilePaths] = useState([[""]]);
  const [ConvertState, setConvertState] = useState("空闲");
  const [SourceExcelData, setSourceExcelData] = useState([]);
  const [RematchExcelData, setRematchExcelData] = useState([]);
  useEffect(() => {
    setAllFilePaths(MergeLines(SourceExcelData, RematchExcelData));
  }, [SourceExcelData, RematchExcelData]);
  return (
    <BaseContainer>
      <div className="grid grid-cols-4 gap-2">
        <Button
          handler={async () =>
            await SelectExcel(setSourceExcelData, setTargetFileNames)
          }
          name={"选择文件"}
          description={"选择PMS下载的Excel文件"}
        />
        <Button
          handler={async () => await GetExcelsData(setMissingLinesData)}
          name={"选择文件"}
          description={"选择缺失数据的Excel文件"}
        />
        <Button
          handler={async () => await GetExcelsData(setFullLinesData)}
          name={"选择文件"}
          description={"选择完整数据的Excel文件"}
        />
        <Button
          handler={async () => await SelectFolder(setFullLinesDataImageFolder)}
          name={"选择文件夹"}
          description={"选择完整数据的图片的文件夹"}
        />
        <Button
          handler={async () => await SelectFolder(setTargetFolder)}
          name={"选择文件夹"}
          description={"选择输出Excel和缺失数据的文件夹"}
        />
        <Button
          handler={
            async () =>
              await StartRepatch({
                setConvertState: setConvertState,
                SourceExcelData: SourceExcelData,
                TargetFileNames: TargetFileNames,
                MissingLinesData: MissingLinesData,
                FullLinesData: FullLinesData,
                FullLinesDataImageFolder: FullLinesDataImageFolder,
                TargetFolder: TargetFolder,
              })
            // await StartConvert(
            //   SourceExcelData,
            //   RematchExcelData,
            //   TargetFolder,
            //   setConvertState
            // )
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
