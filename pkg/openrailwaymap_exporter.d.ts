/* tslint:disable */
/* eslint-disable */
/**
* Initializes the main application component and renders it in the given root element.
*
* This function is meant to be called from JavaScript via WebAssembly to initialize and render
* the main `App` component inside the specified root element. It sets up the panic hook for
* better error messages and uses Yew's renderer to attach the `App` component to the DOM.
*
* # Arguments
*
* * `root` - A `web_sys::Element` representing the root element where the `App` component will be rendered.
*
* # Example
*
* In an HTML file:
*
* ```html
* <body>
*     <div id="app" />
*     <script type="module">
*       import init, { init_app } from "./pkg/openrailwaymap_exporter.js";
*       var root = document.getElementById("app");
*       init().then(async () => {
*         try {
*           init_app(root);
*         } catch (e) {
*           console.error(e);
*         }
*       });
*     </script>
*   </body>
* ```
* @param {Element} root
*/
export function init_app(root: Element): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init_app: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he1c49487f2e5f55f: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h02b137aef052830b: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9682abf6924fe696: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7717eceeb43c1d24: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
