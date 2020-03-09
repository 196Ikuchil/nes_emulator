
let buf = null

// launch nes
const startArrayBuf = (arrayBuf) => {
  const run = Module.cwrap('run', null, ['number', 'number'])
  const canvas = document.querySelector('canvas')
  const ctx = canvas.getContext('2d')
  Module.NES = {
    ctx,
    canvas,
    image: ctx.createImageData(256, 240),
  }
  canvas.width = 256
  canvas.height = 240

  const nes = new Uint8Array(arrayBuf)
  // add key pad code in tail
  const size = nes.byteLength //+ 1
  const ptr = Module._malloc(size)
  buf = new Uint8Array(Module.HEAPU8.buffer, ptr, size)
  buf.set(nes)

  console.log('run nes')
  run(size,buf.byteOffset)
}

// called from html
export const start = async (rom = './roms/sample1.nes') => {
  const res = await fetch(rom);
  const arrayBuf = await res.arrayBuffer();
  startArrayBuf(arrayBuf);
}