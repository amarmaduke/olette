(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/olette.js":
/*!************************!*\
  !*** ../pkg/olette.js ***!
  \************************/
/*! exports provided: __wbg_log_868bedbd060aced6, update, reduce, load_net, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_log_868bedbd060aced6\", function() { return __wbg_log_868bedbd060aced6; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"update\", function() { return update; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"reduce\", function() { return reduce; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"load_net\", function() { return load_net; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _olette_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./olette_bg */ \"../pkg/olette_bg.wasm\");\n/* tslint:disable */\n\n\nconst __wbg_log_868bedbd060aced6_target = console.log;\n\nlet cachedTextDecoder = new TextDecoder('utf-8');\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _olette_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_olette_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nfunction __wbg_log_868bedbd060aced6(arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    __wbg_log_868bedbd060aced6_target(varg0);\n}\n\nlet cachedTextEncoder = new TextEncoder('utf-8');\n\nfunction passStringToWasm(arg) {\n\n    const buf = cachedTextEncoder.encode(arg);\n    const ptr = _olette_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"](buf.length);\n    getUint8Memory().set(buf, ptr);\n    return [ptr, buf.length];\n}\n/**\n* @param {string} arg0\n* @returns {void}\n*/\nfunction update(arg0) {\n    const [ptr0, len0] = passStringToWasm(arg0);\n    try {\n        return _olette_bg__WEBPACK_IMPORTED_MODULE_0__[\"update\"](ptr0, len0);\n\n    } finally {\n        _olette_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](ptr0, len0 * 1);\n\n    }\n\n}\n\nlet cachedGlobalArgumentPtr = null;\nfunction globalArgumentPtr() {\n    if (cachedGlobalArgumentPtr === null) {\n        cachedGlobalArgumentPtr = _olette_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_global_argument_ptr\"]();\n    }\n    return cachedGlobalArgumentPtr;\n}\n\nlet cachegetUint32Memory = null;\nfunction getUint32Memory() {\n    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== _olette_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint32Memory = new Uint32Array(_olette_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint32Memory;\n}\n/**\n* @param {number} arg0\n* @param {string} arg1\n* @returns {string}\n*/\nfunction reduce(arg0, arg1) {\n    const [ptr1, len1] = passStringToWasm(arg1);\n    const retptr = globalArgumentPtr();\n    try {\n        _olette_bg__WEBPACK_IMPORTED_MODULE_0__[\"reduce\"](retptr, arg0, ptr1, len1);\n        const mem = getUint32Memory();\n        const rustptr = mem[retptr / 4];\n        const rustlen = mem[retptr / 4 + 1];\n\n        const realRet = getStringFromWasm(rustptr, rustlen).slice();\n        _olette_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](rustptr, rustlen * 1);\n        return realRet;\n\n\n    } finally {\n        _olette_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](ptr1, len1 * 1);\n\n    }\n\n}\n\n/**\n* @param {string} arg0\n* @returns {string}\n*/\nfunction load_net(arg0) {\n    const [ptr0, len0] = passStringToWasm(arg0);\n    const retptr = globalArgumentPtr();\n    try {\n        _olette_bg__WEBPACK_IMPORTED_MODULE_0__[\"load_net\"](retptr, ptr0, len0);\n        const mem = getUint32Memory();\n        const rustptr = mem[retptr / 4];\n        const rustlen = mem[retptr / 4 + 1];\n\n        const realRet = getStringFromWasm(rustptr, rustlen).slice();\n        _olette_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](rustptr, rustlen * 1);\n        return realRet;\n\n\n    } finally {\n        _olette_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](ptr0, len0 * 1);\n\n    }\n\n}\n\nfunction __wbindgen_throw(ptr, len) {\n    throw new Error(getStringFromWasm(ptr, len));\n}\n\n\n\n//# sourceURL=webpack:///../pkg/olette.js?");

/***/ }),

/***/ "../pkg/olette_bg.wasm":
/*!*****************************!*\
  !*** ../pkg/olette_bg.wasm ***!
  \*****************************/
/*! exports provided: memory, __indirect_function_table, __heap_base, __data_end, update, reduce, load_net, __wbindgen_global_argument_ptr, __wbindgen_malloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./olette */ \"../pkg/olette.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/olette_bg.wasm?");

/***/ }),

/***/ "./src/index.js":
/*!**********************!*\
  !*** ./src/index.js ***!
  \**********************/
/*! exports provided: load_net, reduce_net, update_net */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"load_net\", function() { return load_net; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"reduce_net\", function() { return reduce_net; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"update_net\", function() { return update_net; });\n/* harmony import */ var olette__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! olette */ \"../pkg/olette.js\");\n\n\nfunction load_net(term) { return olette__WEBPACK_IMPORTED_MODULE_0__[\"load_net\"](term); }\nfunction reduce_net(index, kind) { return olette__WEBPACK_IMPORTED_MODULE_0__[\"reduce\"](index, kind); }\nfunction update_net(json) { return olette__WEBPACK_IMPORTED_MODULE_0__[\"update\"](json); }\n\n\n//# sourceURL=webpack:///./src/index.js?");

/***/ })

}]);