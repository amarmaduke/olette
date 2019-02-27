import * as wasm from "olette";

export function load_net(term) { return wasm.load_net(term); }
export function reduce_net(index, kind) { return wasm.reduce(index, kind); }
export function update_net(json) { return wasm.update(json); }
export function rebuild_net(json) { return wasm.rebuild(json);}
