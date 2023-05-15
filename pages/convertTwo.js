"use client";
import React, { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";
import { resolveResource } from '@tauri-apps/api/path'
// alternatively, use `window.__TAURI__.path.resolveResource`
import { readTextFile } from '@tauri-apps/api/fs'
// alternatively, use `window.__TAURI__.fs.readTextFile`
import { open, ask } from "@tauri-apps/api/dialog";
import { convertFileSrc } from '@tauri-apps/api/tauri';

import BaseContainer from "./util/container";
import Button from "./util/Button";

import { createWorker } from "tesseract.js";
import * as ocr from '@paddlejs-models/ocr';

async function SelectExcel() {
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
  let names = await invoke("read_excel_lines", { excel_path: file });
  console.log(names);
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

async function SelectSourceFiles() {
  let file = await open({
    title: "选择图片文件",
    multiple: true,
    filters: [
      {
        name: "Jpeg",
        extensions: ["jpg", "jpeg"],
      },
    ],
  });
  console.log(file);
  return file;
}

async function SelectTargetFolder() {
  let file = await open({
    title: "选择输出文件夹",
    directory: true,
  });
  console.log(file);
  return file;
}

async function StartConvert(SourceFolderFiles, TargetFolder, TargetFileNames) {
  let Thermal = new Map();
  if (TargetFileNames.length === SourceFolderFiles.length) {
    for (let index = 0; index < TargetFileNames.length; index++) {
      const TargetFileName = `${TargetFolder}\\${TargetFileNames[index]}.jpg`;
      const SourceFileName = SourceFolderFiles[index];

      await invoke("copy_file", { from: SourceFileName, to: TargetFileName });

      try {
        let ret = await invoke("read_thermal", { image_path: SourceFileName })
        console.log(`Thermal is :${ret}`)
        Thermal.set(TargetFileNames[index], ret)
      } catch (error) {
        console.log(`Error is: ${error}`)
      }
    }
    await ask("转换完成");
  } else {
    await ask("选择的文件数目不对应", { title: "错误", type: "warning" });
  }
}

async function TestTesseract() {
  let path = convertFileSrc('resources/test.jpg');
  console.log(path)
  let ret = await invoke("read_thermal", { image_path: "C:\\Users\\adqew\\Pictures\\红外\\1000kVT615ACF交流滤波器电流互感器器CT5A相.jpg" })
  console.log(ret)
  // const worker = await createWorker({
  //   workerPath: convertFileSrc('resources/tesseract.js/worker.min.js'),
  //   langPath: convertFileSrc('resources/tessdata/'),
  //   corePath: convertFileSrc('resources/tesseract.js-core/tesseract-core.wasm.js'),
  //   logger: m => console.log(m),
  // });
  // await worker.loadLanguage('eng');
  // await worker.initialize('eng');
  // const { data: { text } } = await worker.recognize(convertFileSrc('resources/test2.jpg'));
  // console.log(text);
  // await worker.terminate();
  // await ocr.init(convertFileSrc('resources/ocr/ch_PP-OCRv2_det_fuse_activation/model.json'), convertFileSrc('resources/ocr/ch_PP-OCRv2_rec_fuse_activation/model.json'));
  // await ocr.init()
  // const img = document.createElement("img");
  // img.src = convertFileSrc('resources/test.jpg')
  // img.crossOrigin='anonymous'

  // const res = await ocr.recognize(img);
  // console.log(res.text);
  // console.log(res.points);
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
      TargetFilePath: `${TargetFolder}\\${TargetFileName}.jpg`,
    });
  }
  return ret;
}

export default function Page() {
  const [SourceFolderFiles, setSourceFolderFiles] = useState([""]);
  const [TargetFolder, setTargetFolder] = useState("");
  const [TargetFileNames, setTargetFileNames] = useState([""]);
  const [AllFilePaths, setAllFilePaths] = useState([[""]]);
  useEffect(() => {
    setAllFilePaths(MergeLines(SourceFolderFiles, TargetFolder, TargetFileNames))
  }, [SourceFolderFiles, TargetFolder, TargetFileNames]);
  return (
    <BaseContainer>
      <div className="grid grid-cols-4 gap-2">
        <Button
          handler={async () => setTargetFileNames(await SelectExcel())}
          name={"选择文件"}
          description={"选择待识别的Excel文件"}
        />
        <Button
          handler={async () => setSourceFolderFiles(await SelectSourceFolder())}
          name={"选择文件夹"}
          description={"选择待识别的图片文件夹"}
        />
        <Button
          handler={async () => setTargetFolder(await SelectTargetFolder())}
          name={"选择文件夹"}
          description={"选择输出的图片文件夹"}
        />
        <Button
          handler={async () =>
            await StartConvert(SourceFolderFiles, TargetFolder, TargetFileNames)
            // await TestTesseract()
          }
          name={"开始转换"}
          description={"按下进行文件名修改"}
        />
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
