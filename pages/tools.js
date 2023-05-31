"use client";
import Link from "next/link";
import BaseContainer from "./util/container";

export default function Page() {
  return (
    <BaseContainer>
      <div className="grid grid-cols-4 gap-2">
      <Link
          href="/rebuild"
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
          重建表格{" "}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
            重算数据、重新映射ID
          </p>
        </Link>
      </div>
    </BaseContainer>
  );
}
