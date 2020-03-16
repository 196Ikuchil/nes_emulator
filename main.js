import Oscillator from './src/nes/webaudio/oscillator.js'
let buf = null

const convertKeyCode = (key) => {
  switch (key) {
    case 88: return 0x01 // X A
    case 90: return 0x02 // Z B
    case 65: return 0x04 // A SELECT
    case 83: return 0x08 // S START
    case 38: return 0x10 // up up
    case 40: return 0x20 // down down
    case 37: return 0x40 // L L
    case 39: return 0x39 // R R
  }
}

const onKeydown = (e) => {
  if (buf != null)
    buf[buf.length - 1] |= convertKeyCode(e.keyCode)
}

const onKeyup = (e) => {
  if (buf != null)
    buf[buf.length - 1] &= ~convertKeyCode(e.keyCode)
}

const setupKeyHandler = () => {
  if (typeof window !== 'undefined') {
    document.addEventListener('keydown', onKeydown)
    document.addEventListener('keyup', onKeyup)
  }
}

setupKeyHandler()

// launch nes
const startArrayBuf = (arrayBuf) => {
  const run = Module.cwrap('run', null, ['number', 'number'])
  const canvas = document.querySelector('canvas')
  const ctx = canvas.getContext('2d')
  if (Module.NES) {
    Module.NES.oscs.forEach(o => o.close());
  }
  Module.NES = {
    ctx,
    canvas,
    image: ctx.createImageData(256, 240),
    oscs: [new Oscillator(), new Oscillator()],
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
export const start = async (rom = './roms/apu/square_timer_div2.nes') => {
  const res = await fetch(rom);
  const arrayBuf = await res.arrayBuffer();
  startArrayBuf(arrayBuf);
}