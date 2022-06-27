import { IFs } from "memfs";
import { WASI, WASIExitError } from "@wasmer/wasi";
import { WasmFs } from "@wasmer/wasmfs";
import WASM_URL from "../../target/wasm32-wasi/release/nio.wasm?url";

const module = await WebAssembly.compileStreaming(fetch(WASM_URL));

export interface ExecOptions {
  args: string[];
}

export class NioVM {
  fs: IFs;

  constructor() {
    const wasmFs = new WasmFs();
    this.fs = wasmFs.fs;
  }

  async exec({ args }: ExecOptions) {
    const wasi = new WASI({
      args: ["nio", ...args],
      env: {},
      bindings: {
        ...WASI.defaultBindings,
        path: WASI.defaultBindings.path.default,
        fs: this.fs,
      },
      preopens: {
        "/": "/",
      },
    });
    const instance = await WebAssembly.instantiate(
      module,
      wasi.getImports(module)
    );
    try {
      wasi.start(instance);
    } catch (err) {
      if (err instanceof WASIExitError) {
        return err.code;
      } else {
        throw err;
      }
    }
  }
}
