export default class SRAM {
  constructor (filename) {
    this.filename = filename
  }

  save(buf) {
    window.localStorage.setItem(this.filename, String.fromCharCode.apply(null, buf))
    console.log('sram saved')
  }

  load() {
    let buf = 0
    if (window.localStorage.getItem(this.filename) == null) {
      console.log('init sram')
      buf = new Uint8Array(0x2000)
    } else {
      console.log('load sram')
      let b = window.localStorage.getItem(this.filename)
      let a = new ArrayBuffer(b.length)
      buf = new Uint8Array(a)
      for (var i = 0;i< b.length;i++) {
        buf[i] = b.charCodeAt(i)
      }
    }
    return buf
  }
}