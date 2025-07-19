const fs = require("node:fs");

const main = async () => {
  const buffer = fs.readFileSync("./examples/add.wasm");
  const module = await WebAssembly.compile(buffer);
  const instance = await WebAssembly.instantiate(module);

  const { add } = instance.exports;
  console.log(add(1, 2));
};

main();
