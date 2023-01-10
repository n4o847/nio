import { useCallback, useEffect, useState } from "react";
import { NioVM } from "../lib/vm";

export const Playground: React.FC = () => {
  const [vm, setVM] = useState<NioVM | null>(null);
  const [source, setSource] = useState("");
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
          ].join("")
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
            ].join("")
          );
        });
      } catch (err) {
        setTarget((target) => target + String(err) + "\n");
      }
    }
  }, [vm, source]);

  return (
    <div className="py-6 w-full h-4/5 grid grid-cols-3 gap-2">
      <div className="col-span-2 flex flex-col">
        <textarea
          className="px-2 py-1 border border-gray-300 dark:border-gray-500 rounded font-mono flex-grow"
          value={source}
          onChange={(e) => setSource(e.target.value)}
        ></textarea>
      </div>
      <div className="flex flex-col gap-2">
        <textarea
          className="px-2 py-1 border border-gray-300 dark:border-gray-500 rounded font-mono flex-grow bg-gray-100 dark:bg-gray-500"
          readOnly
          value={target}
        ></textarea>
        <button
          className="p-2 bg-sky-500 hover:bg-sky-600 text-white rounded font-bold"
          onClick={parse}
        >
          Parse
        </button>
      </div>
    </div>
  );
};
