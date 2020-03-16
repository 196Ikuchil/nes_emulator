mergeInto(LibraryManager.library, {
  canvas_render: function (ptr, len) {
    Module.NES.buf = new Uint8Array(Module.HEAPU8.buffer, ptr, len);
    Module.NES.image.data.set(Module.NES.buf);
    Module.NES.ctx.putImageData(Module.NES.image, 0, 0);
  },
  start_oscillator: function (index) {
    Module.NES.oscs[index].start()
  },
  stop_oscillator: function (index) {
    Module.NES.oscs[index].stop()
  },
  set_oscillator_frequency: function (index, freq) {
    Module.NES.oscs[index].setFrequency(freq)
  },
  change_oscillator_frequency: function (index, freq) {
    Module.NES.oscs[index].changeFrequency(freq)
  },
  set_oscillator_volume: function (index, volume) {
    Module.NES.oscs[index].setVolume(volume)
  },
  set_oscillator_duty: function (index, duty) {
    Module.NES.oscs[index].setDuty(duty)
  }
});
