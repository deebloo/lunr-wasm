import init, { Lunr } from 'lunr-wasm';

export async function main() {
    await init('/lunr-wasm/lunr_wasm_bg.wasm');

    const index = Lunr.new();

    index.add_document("0", "Hello World. This is an example document");
}