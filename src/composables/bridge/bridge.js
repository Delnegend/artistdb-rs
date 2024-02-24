import * as wasm from "./bridge_bg.wasm";
import { __wbg_set_wasm } from "./bridge_bg.js";
__wbg_set_wasm(wasm);
export * from "./bridge_bg.js";
