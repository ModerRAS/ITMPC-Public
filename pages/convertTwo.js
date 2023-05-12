"use client";
import React, { useEffect } from "react";


import { invoke } from "@tauri-apps/api/tauri";
import BaseContainer from "./util/container";

export default function Page() {
  useEffect(() => {
    invoke("read_excel_lines", { excel_path: "World" }).then(console.log).catch(console.error);
  }, []);
  return (
    <BaseContainer>
<h1>Test 2</h1>
    </BaseContainer>
  );
}
