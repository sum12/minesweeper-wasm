
let wasm;

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }
/**
*/
export const CellState = Object.freeze({ Close:0,"0":"Close",Open:1,"1":"Open",Flag:2,"2":"Flag", });
/**
*/
export class Game {

    static __wrap(ptr) {
        const obj = Object.create(Game.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_game_free(ptr);
    }
    /**
    */
    get width() {
        const ret = wasm.__wbg_get_game_width(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set width(arg0) {
        wasm.__wbg_set_game_width(this.ptr, arg0);
    }
    /**
    */
    get height() {
        const ret = wasm.__wbg_get_game_height(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set height(arg0) {
        wasm.__wbg_set_game_height(this.ptr, arg0);
    }
    /**
    */
    get finished() {
        const ret = wasm.__wbg_get_game_finished(this.ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set finished(arg0) {
        wasm.__wbg_set_game_finished(this.ptr, arg0);
    }
    /**
    * @param {number} x
    * @param {number} y
    */
    cell_open(x, y) {
        wasm.game_cell_open(this.ptr, x, y);
    }
    /**
    * @param {number} x
    * @param {number} y
    */
    cell_toggle_flag(x, y) {
        wasm.game_cell_toggle_flag(this.ptr, x, y);
    }
    /**
    * @returns {number}
    */
    cells() {
        const ret = wasm.game_cells(this.ptr);
        return ret;
    }
    /**
    * @param {number} width
    * @param {number} height
    * @param {number} mines
    * @returns {Game}
    */
    static new(width, height, mines) {
        const ret = wasm.game_new(width, height, mines);
        return Game.__wrap(ret);
    }
    /**
    * @param {number} x
    * @param {number} y
    */
    add_mines(x, y) {
        wasm.game_add_mines(this.ptr, x, y);
    }
    /**
    * @param {number} x
    * @param {number} y
    * @returns {number}
    */
    cell_minecount(x, y) {
        const ret = wasm.game_cell_minecount(this.ptr, x, y);
        return ret >>> 0;
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('minesweeper_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_random_d1c00c41674df515 = typeof Math.random == 'function' ? Math.random : notDefined('Math.random');
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

