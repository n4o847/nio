"use client";

import dynamic from "next/dynamic";
import { useTheme } from "nextra-theme-docs";
import { type FC, useCallback, useEffect, useState } from "react";
import { NioVM } from "@/features/playground/lib/vm";

const Editor = dynamic(
  () =>
    import("@/features/playground/components/Editor").then(
      ({ Editor }) => Editor,
    ),
  {
    ssr: false,
  },
);

export const Playground: FC = () => {
  const { resolvedTheme } = useTheme();

  const [vm, setVM] = useState<NioVM | null>(null);
  const [source, setSource] = useState(
    ["def add(x: Int, y: Int): Int = x + y"].join("\n"),
  );
  const [target, setTarget] = useState("");

  useEffect(() => {
    NioVM.load().then((vm) => {
      setVM(vm);
    });
  }, []);

  useEffect(() => {
    if (vm) {
      vm.exec({ args: ["--version"] }).then(() => {
        setTarget(
          [
            vm.fs.readFileSync("/dev/stdout", "utf8"),
            vm.fs.readFileSync("/dev/stderr", "utf8"),
          ].join(""),
        );
      });
    }
  }, [vm]);

  const parse = useCallback(() => {
    if (vm) {
      try {
        vm.fs.appendFileSync("/dev/stdin", source);
        vm.exec({ args: ["parse"] }).then(() => {
          setTarget(
            [
              vm.fs.readFileSync("/dev/stdout", "utf8"),
              vm.fs.readFileSync("/dev/stderr", "utf8"),
            ].join(""),
          );
        });
      } catch (err) {
        setTarget((target) => `${target + String(err)}\n`);
      }
    }
  }, [vm, source]);

  return (
    <div className="h-[calc(100vh-var(--nextra-navbar-height))] flex flex-col">
      <div>
        <div
          className={[
            "mx-auto max-w-(--nextra-content-width) pl-[max(env(safe-area-inset-left),1.5rem)] pr-[max(env(safe-area-inset-right),1.5rem)]",
            "h-12 flex justify-end items-center",
          ].join(" ")}
        >
          <span className="me-auto font-bold">Playground</span>
          <button
            type="button"
            className="w-24 px-2 py-1 bg-sky-500 hover:bg-sky-600 text-white rounded font-bold cursor-pointer"
            onClick={parse}
          >
            Parse
          </button>
        </div>
      </div>
      <div className="flex-1 overflow-y-hidden flex">
        <div className="w-3/4">
          <Editor
            className="h-full"
            theme={resolvedTheme === "dark" ? "dark" : "light"}
            defaultValue={source}
            onChange={(value) => setSource(value)}
          />
        </div>
        <div className="w-1/4">
          <textarea
            className="w-full h-full outline-none font-mono text-[14px]/[19px]"
            value={target}
            readOnly
          />
        </div>
      </div>
    </div>
  );
};
