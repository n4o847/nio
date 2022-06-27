import { NioVM } from "./vm";

async function main() {
  const $source = document.querySelector("#source") as HTMLTextAreaElement;
  const $target = document.querySelector("#target") as HTMLTextAreaElement;
  const $parse = document.querySelector("#parse") as HTMLButtonElement;

  const vm = await NioVM.load();

  await vm.exec({ args: ["--version"] });
  $target.value = [
    vm.fs.readFileSync("/dev/stdout", "utf8"),
    vm.fs.readFileSync("/dev/stderr", "utf8"),
  ].join("");

  $parse.addEventListener("click", async () => {
    let result: string;
    try {
      // vm.fs.writeFileSync("/dev/stdin", "");
      vm.fs.appendFileSync("/dev/stdin", $source.value);
      await vm.exec({ args: ["parse"] });
      result = [
        vm.fs.readFileSync("/dev/stdout", "utf8"),
        vm.fs.readFileSync("/dev/stderr", "utf8"),
      ].join("");
    } catch (err) {
      result = $target.value + String(err) + "\n";
    }
    $target.value = result;
    $target.scrollTop = $target.scrollHeight;
  });
}

main();
