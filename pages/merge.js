"use client";
import React, { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";
import { open, ask } from "@tauri-apps/api/dialog";
import { convertFileSrc } from '@tauri-apps/api/tauri';

import BaseContainer from "./util/container";
import Button from "./util/Button";

async function SelectExcel(setTargetExcelData) {
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
  console.log(lines)
  setTargetExcelData(lines)
  return lines;
}

async function SelectSourceFolder() {
  let file = await open({
    title: "选择待合并的Excel文件",
    directory: true,
  });
  console.log(file);
  let names = await invoke("get_image_from_directory", { source: file });
  console.log(names);
  return names;
}

async function SelectSourceFiles(setSourceExcelData, setSourceFolderFiles) {
  let files = await open({
    title: "选择待合并的Excel文件",
    multiple: true,
    filters: [
      {
        name: "Excel",
        extensions: ["xls", "xlsx"],
      },
    ],
  });
  let source = [];
  console.log(files);
  if (files?.length > 0) {
    files.forEach(async file => {
      let lines = await invoke("get_excel_lines", { excel_path: file });
      source.push(...lines)
    });
    console.log(source);
    setSourceExcelData(source);
    setSourceFolderFiles(files);
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

async function StartConvert(SourceExcelData, TargetExcelData, TargetFolder, setConvertState) {

  await setConvertState(`合并中。。。`);

  let SourceExcelDataMap = new Map();

  let retdata = []

  let not_found_data = []

  for (const ExcelData of SourceExcelData) {
    SourceExcelDataMap.set(ExcelData.device_name, ExcelData)
  }



  for (const ExcelData of TargetExcelData) {
    let data = SourceExcelDataMap.get(ExcelData.device_name)
    // console.log(`data: ${data}, ExcelData: ${ExcelData}`)
    // console.log(data)
    // console.log(ExcelData)
    if (data) {
      retdata.push(data)
    } else {
      not_found_data.push(ExcelData)
    }
  }

  console.log(retdata)



  await invoke("write_to_excel", {excel_datas: retdata, save_path: `${TargetFolder}/表格数据.xlsx`})
  await setConvertState(`合并完成`);
  if (not_found_data.length > 0) {
    let printdata = []
    for (const ExcelData of not_found_data) {
      printdata.push(ExcelData.device_name)
    }
    await setConvertState(`未找到以下数据(共${printdata.length}个): ${JSON.stringify(printdata)}`)
  }
}

function MergeLines(SourceFolderFiles, TargetFolder, TargetFileNames) {
  let ret = [];
  if (TargetFolder == "") {
    return [];
  }
  if (TargetFileNames.length <= 0) {
    return [];
  }
  let min_length =
    SourceFolderFiles.length > TargetFileNames.length
      ? TargetFileNames.length
      : SourceFolderFiles.length;
  for (let index = 0; index < min_length; index++) {
    const SourceFolderFile = SourceFolderFiles[index];
    const TargetFileName = TargetFileNames[index];
    ret.push({
      SourceFilePath: SourceFolderFile,
      TargetFilePath: `${TargetFolder}\\合并表格.xlsx`,
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
  const [TargetExcelData, setTargetExcelData] = useState([]);
  const [SourceExcelData, setSourceExcelData] = useState([]);
  useEffect(() => {
    setAllFilePaths(MergeLines(SourceFolderFiles, TargetFolder, TargetFileNames))
  }, [SourceFolderFiles, TargetFolder, TargetFileNames]);
  return (
    <BaseContainer>
      <div className="grid grid-cols-4 gap-2">
        <Button
          handler={async () => await SelectExcel(setTargetExcelData)}
          name={"选择文件"}
          description={"选择PMS下载的模板Excel文件"}
        />
        <Button
          handler={async () => await SelectSourceFiles(setSourceExcelData, setSourceFolderFiles)}
          name={"选择文件"}
          description={"选择待合并的Excel文件"}
        />
        <Button
          handler={async () => await SelectTargetFolder(setTargetFolder)}
          name={"选择文件夹"}
          description={"选择输出的Excel文件存放目录"}
        />
        <Button
          handler={async () =>
            await StartConvert(SourceExcelData, TargetExcelData, TargetFolder, setConvertState)
            // await TestTesseract()
          }
          name={"开始转换"}
          description={"按下进行Excel合并"}
        />
        <div className="border-collapse border border-green-800 table-auto col-span-4">
          当前状态：{ConvertState}
        </div>
        <table className="border-collapse border border-green-800 table-auto col-span-4">
          <thead>
            <tr>
              <th className="border border-green-800">待转换文件，共{SourceFolderFiles.length}个</th>
              <th className="border border-green-800">输出文件，共{TargetFileNames.length}个</th>
            </tr>
          </thead>
          <tbody>
            {AllFilePaths.map(
              (element, i) => (
                <tr key={i}>
                  <td className="border border-green-800">{element.SourceFilePath}</td>
                  <td className="border border-green-800">{element.TargetFilePath}</td>
                </tr>
              )
            )}
          </tbody>
        </table>
      </div>
    </BaseContainer>
  );
}
