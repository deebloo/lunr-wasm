import init, { Lunr } from "./pkg/lunr_wasm";

export async function main() {
  await init();

  const prebuilt_index: Uint8Array = await fetch("./search_index.bin")
    .then((res) => res.arrayBuffer())
    .then((res) => new Uint8Array(res));

  const index = Lunr.new();

  index.load_index(prebuilt_index);

  const res = index.search("Cyclone Pinney");

  console.log(res);
}
