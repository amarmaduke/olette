# Optimal Lambda Evaluator Toy

The t & e come for free.

# Building

## Prerequisites
* npm
* rust
* wasm-pack

## Build

In the root directory of the project run `wasm-pack build`, this should produce a `pkg` folder. Next in the `www` directory install the following modules by running the following.

* `npm install webpack`
* `npm install webpack-cli`
* `npm install copy-webpack-plugin`
* `npm install d3`
* `npm install webpack-dev-server`
* `npm install check-dependencies --save-dev`

Next, run `npm link –local olette` and finally `npm run build` to produce a dist folder. The `dist` folder is a self-contained website with html, wasm modules, and javascript. Alternatively, `npm start` in the `www` directory will start a local server for the frontend.
