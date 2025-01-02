import {
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  getDefaultContext as __emnapiGetDefaultContext,
  WASI as __WASI,
  createOnMessage as __wasmCreateOnMessageForFsProxy,
} from '@napi-rs/wasm-runtime'
import { memfs } from '@napi-rs/wasm-runtime/fs'
import __wasmUrl from './node_bindings.wasm32-wasi.wasm?url'

export const { fs: __fs, vol: __volume } = memfs()

const __wasi = new __WASI({
  version: 'preview1',
  fs: __fs,
  preopens: {
    '/': '/',
  },
})

const __emnapiContext = __emnapiGetDefaultContext()

const __sharedMemory = new WebAssembly.Memory({
  initial: 16384,
  maximum: 65536,
  shared: true,
})

const __wasmFile = await fetch(__wasmUrl).then((res) => res.arrayBuffer())

const {
  instance: __napiInstance,
  module: __wasiModule,
  napiModule: __napiModule,
} = __emnapiInstantiateNapiModuleSync(__wasmFile, {
  context: __emnapiContext,
  asyncWorkPoolSize: 4,
  wasi: __wasi,
  onCreateWorker() {
    const worker = new Worker(new URL('./wasi-worker-browser.mjs', import.meta.url), {
      type: 'module',
    })
    worker.addEventListener('message', __wasmCreateOnMessageForFsProxy(__fs))

    return worker
  },
  overwriteImports(importObject) {
    importObject.env = {
      ...importObject.env,
      ...importObject.napi,
      ...importObject.emnapi,
      memory: __sharedMemory,
    }
    return importObject
  },
  beforeInit({ instance }) {
    __napi_rs_initialize_modules(instance)
  },
})

function __napi_rs_initialize_modules(__napiInstance) {
  __napiInstance.exports['__napi_register__JsIndentOptions_struct_0']?.()
  __napiInstance.exports['__napi_register__JsMagicStringOptions_struct_1']?.()
  __napiInstance.exports['__napi_register__JsGenerateMapOptions_struct_2']?.()
  __napiInstance.exports['__napi_register__JsOverwriteOptions_struct_3']?.()
  __napiInstance.exports['__napi_register__FmsRegex_struct_4']?.()
  __napiInstance.exports['__napi_register__JsSourceMap_struct_5']?.()
  __napiInstance.exports['__napi_register__JsDecodedMap_struct_6']?.()
  __napiInstance.exports['__napi_register__JsMagicString_struct_7']?.()
  __napiInstance.exports['__napi_register__JsMagicString_impl_39']?.()
}
export const MagicString = __napiModule.exports.MagicString
export const JsMagicString = __napiModule.exports.JsMagicString
