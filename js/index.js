window.Module = {}
// Initializing the memory with 20 pages (20 * 64KiB = 1.25 MiB)
const memory = new WebAssembly.Memory({initial: 20});

fetchAndInstantiate("./markdown.wasm", {})
    .then(mod => {
        Module.memory      = mod.exports.memory;
        Module.alloc       = mod.exports.alloc;
        Module.dealloc     = mod.exports.dealloc;
        Module.dealloc_str = mod.exports.dealloc_str;
        Module.translate = function(str) {
            let buf = newString(Module, str);
            console.log(new Uint8Array(Module.memory.buffer, buf, 10));
            let outptr = mod.exports.translate(buf);
            console.log(new Uint8Array(Module.memory.buffer, outptr, 10));

            let result = copyCStr(Module, outptr);
            return result;
        };
    });

function myTranslate(text) {
    document.getElementById('display').innerHTML
        = Module.translate(text)
}


//link: https://www.hellorust.com/demos/import-memory/index.html
function fetchAndInstantiate(url, importObject) {
    return fetch(url).then(response =>
        response.arrayBuffer()
    ).then(bytes =>
        WebAssembly.instantiate(bytes, importObject)
    ).then(results =>
        results.instance
    );
}

function newString(module, str) {
    const utf8Encoder = new TextEncoder("UTF-8");
    let string_buffer = utf8Encoder.encode(str)
    let len = string_buffer.length
    let ptr = module.alloc(len+1)

    let memory = new Uint8Array(module.memory.buffer);
    for (i = 0; i < len; i++) {
        memory[ptr+i] = string_buffer[i]
    }

    memory[ptr+len] = 0;
    return ptr;
}

// Copy a nul-terminated string from the buffer pointed to.
// Consumes the old data and thus deallocated it.
function copyCStr(module, ptr) {
    let orig_ptr = ptr;
    const collectCString = function* () {
        let memory = new Uint8Array(module.memory.buffer);
        while (memory[ptr] !== 0) {
            if (memory[ptr] === undefined) { throw new Error("Tried to read undef mem") }
            yield memory[ptr]
            ptr += 1
        }
    }

    const buffer_as_u8 = new Uint8Array(collectCString())
    const utf8Decoder = new TextDecoder("UTF-8");
    const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
    module.dealloc_str(orig_ptr);
    return buffer_as_utf8
}
