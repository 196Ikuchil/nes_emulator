use super::mapper::*;
use super::Data;
use super::Addr;
use super::Rom;
use super::Ram;
use super::PpuConfig;
use super::Ppu;
use super::CpuRegister;
use super::Register;
use std::{thread, time};

#[derive(Debug)]
pub struct Mapper4 {
  register: Data,
  registers: Vec<Data>,
  prg_mode: bool,
  chr_mode: bool,
  prg_offsets: Vec<i32>,
  chr_offsets: Vec<i32>,
  reload: Data,
  counter: Data,
  irq_enabled: bool,
  PRG_ROM_LEN: usize,
  CHR_RAM_LEN: usize,
}

impl Mapper4 {
  pub fn new(prg_rom_len: usize, chr_ram_len: usize) -> Self {
    let mut m = Mapper4 {
      register: 0,
      registers: vec![0; 8],
      prg_mode: false,
      chr_mode: false,
      prg_offsets: vec![0; 0x4],
      chr_offsets: vec![0; 0x8],
      reload: 0,
      counter: 0,
      irq_enabled: false,
      PRG_ROM_LEN: prg_rom_len,
      CHR_RAM_LEN: chr_ram_len,
    };
    m.prg_offsets[0] = m.prg_bank_offset(0);
    m.prg_offsets[1] = m.prg_bank_offset(1);
    m.prg_offsets[2] = m.prg_bank_offset(-2);
    m.prg_offsets[3] = m.prg_bank_offset(-1);
    m
  }


  fn write_register(&mut self, addr: Addr, data: Data, ppu_cfg: &mut PpuConfig) {
    match addr {
      0x8000..=0x9FFF =>{
        if addr%2 == 0 {
          self.write_bank_select(data);
        } else {
          self.write_bank_data(data);
        }
      }
      0xA000..=0xBFFF => {
        if addr%2 == 0 {
          self.write_mirror(data, ppu_cfg);
        } else {
          self.write_protect();
        }
      }
      0xC000..=0xDFFF => {
        if addr%2 == 0 {
          self.write_irq_latch(data);
        } else {
          self.write_irq_reload();
        }
      }
      0xE000..=0xFFFF => {
        if addr%2 == 0 {
          self.write_irq_disable();
        } else {
          self.write_irq_enable();
        }
      }
      _ => panic!("[READ] There is an illegal address (0x{:x}) access on Mapper.", addr),
    }
  }

  fn write_bank_select(&mut self, data: Data) {
    self.prg_mode = (data & 0x40) == 0x40;
    self.chr_mode = (data & 0x80) == 0x80;
    self.register = data & 0x07;
    self.update_offsets();
  }

  fn write_bank_data(&mut self, data: Data) {
    self.registers[self.register as usize] = data;
    self.update_offsets();
  }

  fn update_offsets(&mut self) {
    // println!("mod:{:?} prg offset 0:{:x} 1:{:x} 2:{:x} 3:{:x}",self.prg_mode, self.prg_offsets[0],self.prg_offsets[1],self.prg_offsets[2],self.prg_offsets[3]);
    if self.prg_mode {
      self.prg_offsets[0] = self.prg_bank_offset(-2);
      self.prg_offsets[1] = self.prg_bank_offset(self.registers[7] as i16);
      self.prg_offsets[2] = self.prg_bank_offset(self.registers[6] as i16);
      self.prg_offsets[3] = self.prg_bank_offset(-1);
    } else {
      self.prg_offsets[0] = self.prg_bank_offset(self.registers[6] as i16);
      self.prg_offsets[1] = self.prg_bank_offset(self.registers[7] as i16);
      self.prg_offsets[2] = self.prg_bank_offset(-2);
      self.prg_offsets[3] = self.prg_bank_offset(-1);
    }
    // println!("mod:{:?} chr offset 0:{:x} 1:{:x} 2:{:x} 3:{:x} 4:{:x} 5:{:x} 6:{:x} 7:{:x}",self.chr_mode, self.chr_offsets[0], self.chr_offsets[1], self.chr_offsets[2], self.chr_offsets[3], self.chr_offsets[4], self.chr_offsets[5], self.chr_offsets[6], self.chr_offsets[7],);
    if self.chr_mode {
      self.chr_offsets[0] = self.chr_bank_offset(self.registers[2] as i16);
      self.chr_offsets[1] = self.chr_bank_offset(self.registers[3] as i16);
      self.chr_offsets[2] = self.chr_bank_offset(self.registers[4] as i16);
      self.chr_offsets[3] = self.chr_bank_offset(self.registers[5] as i16);
      self.chr_offsets[4] = self.chr_bank_offset((self.registers[0] & 0xFE) as i16);
      self.chr_offsets[5] = self.chr_bank_offset((self.registers[0] | 0x01) as i16);
      self.chr_offsets[6] = self.chr_bank_offset((self.registers[1] & 0xFE) as i16);
      self.chr_offsets[7] = self.chr_bank_offset((self.registers[1] | 0x01) as i16);
    } else {
      self.chr_offsets[0] = self.chr_bank_offset((self.registers[0] & 0xFE) as i16);
      self.chr_offsets[1] = self.chr_bank_offset((self.registers[0] | 0x01) as i16);
      self.chr_offsets[2] = self.chr_bank_offset((self.registers[1] & 0xFE) as i16);
      self.chr_offsets[3] = self.chr_bank_offset((self.registers[1] | 0x01) as i16);
      self.chr_offsets[4] = self.chr_bank_offset(self.registers[2] as i16);
      self.chr_offsets[5] = self.chr_bank_offset(self.registers[3] as i16);
      self.chr_offsets[6] = self.chr_bank_offset(self.registers[4] as i16);
      self.chr_offsets[7] = self.chr_bank_offset(self.registers[5] as i16);
    }
  }

  fn prg_bank_offset(&self, index: i16) -> i32 {
    let mut i = if index >= 0x80 {
      index - 0x100
    } else {
      index
    };
    i %= (self.PRG_ROM_LEN as i32 / 0x2000) as i16;
    let mut offset = i as i32 * 0x2000;
    if offset < 0 {
      offset += self.PRG_ROM_LEN as i32;
    }
    offset
  }

  fn chr_bank_offset(&self, index: i16) -> i32 {
    let mut i = if index >= 0x80 {
      index - 0x100
    } else {
      index
    };
    i %= (self.CHR_RAM_LEN as i32 / 0x0400) as i16;
    let mut offset = i as i32 * 0x0400;
    if offset < 0 {
      offset += self.CHR_RAM_LEN as i32;
    }
    // println!("{:?}",offset);
    offset
  }

  fn write_mirror(&mut self, data: Data, ppu_cfg: &mut PpuConfig) {
    match data & 0x01 {
      0x00 => ppu_cfg.is_horizontal_mirror = false, // vertical
      0x01 => ppu_cfg.is_horizontal_mirror = true,
      _ => panic!("calculation error in mirror {:?}", data),
    }
  }

  fn write_protect(&self) {
  }

  fn write_irq_latch(&mut self, data: Data) {
    self.reload = data;
  }

  fn write_irq_reload(&mut self) {
    self.counter = 0;
  }

  fn write_irq_disable (&mut self) {
    self.irq_enabled = false;
  }

  fn write_irq_enable (&mut self) {
    self.irq_enabled = true;
  }


  fn handle_scan_line(&mut self, cpu_register: &mut Register) {
    if self.counter == 0 {
      self. counter = self.reload
    } else {
      self.counter -= 1;
      if self.counter == 0 && self.irq_enabled {
          cpu_register.set_interrupt_irq();
      }
    }
  }
}

impl Mapper for Mapper4 {
  fn get_cram_index(&self, addr: Addr) -> Addr {
    let bank = addr / 0x0400;
    let offset = addr % 0x0400;
    (self.chr_offsets[bank as usize] + offset as i32) as Addr
  }

  fn read(&mut self, addr: Addr, prg_rom: &Rom, sram: &Ram) -> Data {
    let ten_millis = time::Duration::from_millis(100);
    // thread::sleep(ten_millis);
    match addr {
      0x6000..=0x7FFF => sram.read(addr - 0x6000),
      0x8000..=0xFFFF =>  {
        // print!("{:x} ", addr);
        let address = addr - 0x8000;
        let bank = address / 0x2000;
        let offset = address % 0x2000;
        // print!("addr{:x} bank{:x} offset{:x}",address,bank,offset);
        // println!("prg value {:x}", prg_rom.read(self.prg_offsets[bank as usize] as u32 + offset as u32 ));
        prg_rom.read(self.prg_offsets[bank as usize] as u32 + offset as u32)
      },
      _ => panic!("[READ] There is an illegal address (0x{:x}) access on Mapper.", addr),
    }
  }

  fn write(&mut self, addr: Addr, data: Data, sram: &mut Ram, ppu_cfg: &mut PpuConfig) {
    match addr {
      0x6000..=0x7FFF => sram.write(addr - 0x6000, data),
      0x8000..=0xFFFF => self.write_register(addr, data, ppu_cfg),
      _ => panic!("[READ] There is an illegal address (0x{:x}) access on Mapper.", addr),
    }
  }

  fn step(&mut self, ppu: &Ppu, cpu_register: &mut Register) {
    if ppu.cycle != 280 { // TODO: this *should* be 260
      return
    }
    if 239 < ppu.line && ppu.line <261 {
      return
    }
    if !ppu.is_background_enabled() && !ppu.is_sprite_enabled() {
      return
    }
    self.handle_scan_line(cpu_register)
  }
}