
                (function() {
                    var wasm;
                    const __exports = {};
                    

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null ||
        cachegetUint8Memory.buffer !== wasm.memory.buffer)
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    return cachegetUint8Memory;
}

function getArrayU8FromWasm(ptr, len) {
    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null)
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null ||
        cachegetUint32Memory.buffer !== wasm.memory.buffer)
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    return cachegetUint32Memory;
}

let cachedDecoder = new TextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachegetUint64Memory = null;
function getUint64Memory() {
    if (cachegetUint64Memory === null ||
        cachegetUint64Memory.buffer !== wasm.memory.buffer)
        cachegetUint64Memory = new BigUint64Array(wasm.memory.buffer);
    return cachegetUint64Memory;
}

function passArray8ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 1);
    getUint8Memory().set(arg, ptr / 1);
    return [ptr, arg.length];
}

let cachedEncoder = new TextEncoder('utf-8');

function passStringToWasm(arg) {

    const buf = cachedEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}

__exports.convert = function(arg0, arg1) {
    const [ptr0, len0] = passArray8ToWasm(arg0);
    const [ptr1, len1] = passStringToWasm(arg1);
    try {
        return Result.__construct(wasm.convert(ptr0, len0, ptr1, len1));
    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);
    }
};

class Result {

                static __construct(ptr) {
                    return new Result(ptr);
                }

                constructor(ptr) {
                    this.ptr = ptr;
                }

            free() {
                const ptr = this.ptr;
                this.ptr = 0;
                wasm.__wbg_result_free(ptr);
            }
        get_data() {
    const retptr = globalArgumentPtr();
    wasm.result_get_data(retptr, this.ptr);
    const mem = getUint32Memory();
    const ptr = mem[retptr / 4];
    const len = mem[retptr / 4 + 1];
    const realRet = getArrayU8FromWasm(ptr, len).slice();
    wasm.__wbindgen_free(ptr, len * 1);
    return realRet;
}
get_err() {
    const retptr = globalArgumentPtr();
    wasm.result_get_err(retptr, this.ptr);
    const mem = getUint32Memory();
    const ptr = mem[retptr / 4];
    const len = mem[retptr / 4 + 1];
    const realRet = getStringFromWasm(ptr, len).slice();
    wasm.__wbindgen_free(ptr, len * 1);
    return realRet;
}
}
__exports.Result = Result;

__exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

__exports.__wbindgen_sin = function(x) { return Math.sin(x); };

__exports.__wbindgen_cos = function(x) { return Math.cos(x); };

__exports.__wbindgen_Math_atan2 = function(x, y) { return Math.atan2(x, y); };

                    function init(wasm_path) {
                        return fetch(wasm_path)
                            .then(response => response.arrayBuffer())
                            .then(buffer => WebAssembly.instantiate(buffer, { './wasm': __exports }))
                            .then(({instance}) => {
                                wasm = init.wasm = instance.exports;
                                return;
                            });
                    };
                    self.wasm_bindgen = Object.assign(init, __exports);
                })();
            