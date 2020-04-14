export default class Audio {
  constructor() {
    this.AUDIO_BUFFER = 1024
    this.SAMPLE_COUNT = 1024 * 16
    this.SAMPLE_MASK = this.SAMPLE_COUNT - 1
    try {
      const AudioContext = window.AudioContext || window.webkitAudioContext
      this.context = new AudioContext()
      if (this.context.sampleRate != 44100) {
        throw "here define default sample rate is 44100"
      }
    } catch (e) {
      throw new Error('Web Audio isn\'t supported in this browser!');
    }
    this.samples = new Float32Array(this.SAMPLE_COUNT)
    this.read_pointer = 0
    this.write_pointer = 0
    let processor = this.context.createScriptProcessor(this.AUIDO_BUFFER, 0, 1)
    processor.onaudioprocess = this.callback.bind(this)
    processor.connect(this.context.destination)
  }

  callback (event) {

    let dest = event.outputBuffer
    let sampleLen = dest.length
    if (((this.write_pointer - this.read_pointer) & this.SAMPLE_MASK) < this.AUDIO_BUFFER ) return
    console.log(sampleLen)
    let channel = dest.getChannelData(0)
    for (let i = 0; i< sampleLen; i++) {
      channel[i] = this.samples[(this.read_pointer + i ) & this.SAMPLE_MASK]
    }


    this.read_pointer = (this.read_pointer + sampleLen) & this.SAMPLE_MASK
  }

  putAudioSamples(output) {
    console.log("put")
    this.samples[this.write_pointer] = output
    this.write_pointer = (this.write_pointer + 1) & this.SAMPLE_MASK
  }
}



