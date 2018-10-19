# Optimal Lambda Evaluator Toy

The t & e come for free.

# Building

## Prerequisites
* npm
* rust
* wasm-pack

## Build

In the root directory of the project run `wasm-pack build`, this should produce a `pkg` folder.
Next in the `www` direcotry run `npm run build`, this should produce a `dist` folder.
The `dist` folder is a self-contained website with html, wasm modules, and javascript.

Alternatively, `npm start` in the `www` directory will start a local server for the frontend.
