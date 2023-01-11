import { IFs } from "memfs";
import { WASI, WASIExitError } from "@wasmer/wasi";
import { WasmFs } from "@wasmer/wasmfs";

const WASM_URL = new URL(
  "../../../../target/wasm32-wasi/release/nio.wasm",
  import.meta.url
);

let moduleCache: WebAssembly.Module | null = null;

export interface ConstructorOptions {
  module: WebAssembly.Module;
  fs: IFs;
}

export interface ExecOptions {
  args: string[];
}

export class NioVM {
  module: WebAssembly.Module;
  fs: IFs;

  static async load() {
    if (!moduleCache) {
      moduleCache = await WebAssembly.compileStreaming(fetch(WASM_URL));
    }
    const wasmFs = new WasmFs();
    return new NioVM({
      module: moduleCache,
      fs: wasmFs.fs,
    });
  }

  constructor({ module, fs }: ConstructorOptions) {
    this.module = module;
    this.fs = fs;
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
      this.module,
      wasi.getImports(this.module)
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
