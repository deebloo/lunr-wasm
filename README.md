# Lunr Wasm

This project is an experiment to port the lunr search engine over to rust and run it in the browser and in node as web assembly.


## Things to know

The only things that are straighforward to pass between the wasm code and js are string and UInt8Arrays.