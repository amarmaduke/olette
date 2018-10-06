import * as wasm from "yalar";

export function load_net(term) { return wasm.load_net(term); }
export function reduce_net(index, kind) { return wasm.reduce(index, kind); }
