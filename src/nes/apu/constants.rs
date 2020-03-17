pub const CPU_CLOCK: usize = 1789772;

pub const DIVIDE_COUNT_FOR_240HZ: u16 = 7457;

// ref. http://pgate1.at-ninja.jp/NES_on_FPGA/nes_apu.htm
pub const COUNTER_TABLE: &'static [u8] = &[0x0A, 0xFE, 0x14, 0x02, 0x28, 0x04, 0x50, 0x06, 0xA0,
                                           0x08, 0x3C, 0x0A, 0x0E, 0x0C, 0x1A, 0x0E, 0x0C, 0x10,
                                           0x18, 0x12, 0x30, 0x14, 0x60, 0x16, 0xC0, 0x18, 0x48,
                                           0x1A, 0x10, 0x1C, 0x20, 0x1E];

pub const GROBAL_GAIN: f32 = 0.1;

// ref. http://wiki.nesdev.com/w/index.php/APU_Noise
pub const NOISE_TIMER_PERIOD_TABLE: &'static [u16] = &[0x004, 0x008, 0x010, 0x020, 0x040, 0x060,
                                                      0x080, 0x0A0, 0x0CA, 0x0FE, 0x17C, 0x1FC,
                                                      0x2FA, 0x3F8, 0x7F2, 0xFE4];