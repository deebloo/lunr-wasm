import init, { Lunr } from "./pkg/lunr_wasm";

export async function main() {
  await init();

  const prebuild_index_data: number[] = await fetch(
    "./search_index.json"
  ).then((res) => res.json());

  let built_index = new Uint8Array(prebuild_index_data);

  const index = Lunr.new();

  index.import(built_index);

  // index.add_document("0", "Cyclone Seroja (pictured) makes landfall in Indonesia and East Timor, killing at least 113 people and displacing thousands of others.");
  // index.add_document("1", "The Statute of Anne, the first legislation in Great Britain providing for copyright regulated by the government and courts, received royal assent and went into effect five days later.");
  // index.add_document("2", "Eunice Pinney (1770–1849) was an American folk artist active in the towns of Windsor and Simsbury, Connecticut. According to art historian Jean Lipman, a specialist in American folk painting, Pinney and her contemporary Mary Ann Willson are considered two of the earliest American painters to work in the medium of watercolor.");

  const res = index.search("Pinney statute");

  console.log(res);
}
