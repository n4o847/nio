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
    <div>
      <textarea
        value={source}
        onChange={(e) => setSource(e.target.value)}
        cols={30}
        rows={10}
      ></textarea>
      <button onClick={parse}>Parse</button>
      <textarea readOnly value={target} cols={30} rows={10}></textarea>
    </div>
  );
};
