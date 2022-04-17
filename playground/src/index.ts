async function main() {
  const nio = await import("../crate/pkg");

  const $source = document.querySelector("#source") as HTMLTextAreaElement;
  const $target = document.querySelector("#target") as HTMLTextAreaElement;
  const $parse = document.querySelector("#parse") as HTMLButtonElement;

  $parse.addEventListener("click", () => {
    let result: string;
    try {
      result = nio.parse($source.value);
    } catch (err) {
      result = String(err);
    }
    $target.value = result;
  });
}

main();
