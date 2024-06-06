import fs from 'fs'

const mathwasm = fs.readFileSync('./math.wasm');

const math = await WebAssembly.instantiate(new Uint8Array(mathwasm)).then(res => res.instance.exports);

console.log(math.echo(55));