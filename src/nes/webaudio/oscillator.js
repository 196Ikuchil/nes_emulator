
export default class Oscillator {
  constructor (type) {
    try {
      const AudioContext = window.AudioContext || window.webkitAudioContext
      this.context = new AudioContext()
    } catch (e) {
      throw new Error('Web Audio isn\'t supported in this browser!')
    }
    this.type = type || 'square'
    this.oscillator = this.createOscillator({kind: this.type})

    this.setVolume(0)
    this.setDuty(0.5)
    this.playing = false
  }

  start () {
    if (this.playing) {
      this.stop()
    }
    this.playing = true
    this.oscillator.start(0)
  }

  stop () {
    if (this.stop) {
      this.setVolume(0)
      this.playing = false
      this.oscillator.stop(this.context.currentTime)
      this.oscillator = this.createOscillator({kind: this.type})
      this.setDuty(0.5)
    }
  }

  close () {
    this.context.close()
    console.log("close?")
  }

  createOscillator(options = {}) {
    const oscillator = this.context.createOscillator()
    if (options.kind) {
      if (options.kind == 'square') {
        oscillator.type = 'sawtooth'
      } else {
        oscillator.type = options.kind
      }
    }
    oscillator.type = 'sawtooth'
    if (options.frequency) oscillator.frequency.value = options.frequency
    if (options.harmonics) {
      const waveform = this.context.createPeriodicWave(
        new Float32Array(options.harmonics.real),
        new Float32Array(options.harmonics.imag)
      )
      oscillator.setPeriodicWave(waveform);
    }
    var inverter = this.context.createGain() // for square
    inverter.gain.value = -1
    this.delay = this.context.createDelay() // for duty
    this.duty = 0.5

    this.gain = this.context.createGain()
    this.gain.gain.volume = 0.01
    this.gain.gain.value = 1
    oscillator.connect(inverter)
    oscillator.connect(this.gain)
    inverter.connect(this.delay)
    this.delay.connect(this.gain)
    this.gain.connect(this.context.destination)
    return oscillator
  }

  setDuty(duty) {
    this.duty = duty
    this.changeDuty()
  }

  changeDuty(freq = this.oscillator.frequency.value) {
    this.delay.delayTime.value = (1.0 - this.duty) / (freq + 0.1)
  }

  setFrequency (frequency) {
    this.oscillator.frequency.value = frequency
    this.changeDuty()
  }

  changeFrequency (frequency) {
    this.oscillator.frequency.setValueAtTime(frequency, this.context.currentTime)
    this.oscillator.frequency.value = frequency
    this.changeDuty(frequency)
  }

  setVolume (volume) {
    volume = Math.max(0, Math.min(1, volume))
    this.gain.gain.value = volume
  }
}