use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Opecode {
  pub name: Instruction,
  pub mode: Addressing,
  pub cycle: u8,
}

#[derive(Debug)]
pub enum Instruction {
  // transport
  LDA,
  LDX,
  LDY,
  STA,
  STX,
  STY,
  TAX,
  TAY,
  TSX,
  TXA,
  TXS,
  TYA,
  // calculate
  ADC,
  AND,
  ASL,
  BIT,
  CMP,
  CPX,
  CPY,
  DEC,
  DEX,
  DEY,
  EOR,
  INC,
  INX,
  INY,
  LSR,
  ORA,
  ROL,
  ROR,
  SBC,
  // stack
  PHA,
  PHP,
  PLA,
  PLP,
  // jump
  JMP,
  JSR,
  RTS,
  RTI,
  // branch
  BCC,
  BCS,
  BEQ,
  BMI,
  BNE,
  BPL,
  BVC,
  BVS,
  // change flag
  CLC,
  CLD,
  CLI,
  CLV,
  SEC,
  SED,
  SEI,
  // others
  BRK,
  NOP,
}

#[derive(Debug, PartialEq)]
pub enum Addressing {
  Implied,
  Accumulator,
  Immediate,
  Zeropage,
  ZeropageX,
  ZeropageY,
  Relative,
  Absolute,
  AbsoluteX,
  AbsoluteY,
  Indirect,
  IndirectX,
  IndirectY,
}

lazy_static! {
  pub static ref OPEMAP: HashMap<u8, Opecode> = {
    let mut m = HashMap::new();
    m.insert(0xA9, Opecode{ name: Instruction::LDA, mode: Addressing::Immediate, cycle: 2});
    m.insert(0xA5, Opecode{ name: Instruction::LDA, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0xB5, Opecode{ name: Instruction::LDA, mode: Addressing::ZeropageX, cycle: 4});
    m.insert(0xAD, Opecode{ name: Instruction::LDA, mode: Addressing::Absolute, cycle: 4});
    m.insert(0xBD, Opecode{ name: Instruction::LDA, mode: Addressing::AbsoluteX, cycle: 4});
    m.insert(0xB9, Opecode{ name: Instruction::LDA, mode: Addressing::AbsoluteY, cycle: 4});
    m.insert(0xA1, Opecode{ name: Instruction::LDA, mode: Addressing::IndirectX, cycle: 6});
    m.insert(0xB1, Opecode{ name: Instruction::LDA, mode: Addressing::IndirectY, cycle: 5});
    m.insert(0xA2, Opecode{ name: Instruction::LDX, mode: Addressing::Immediate, cycle: 2});
    m.insert(0xA6, Opecode{ name: Instruction::LDX, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0xB6, Opecode{ name: Instruction::LDX, mode: Addressing::ZeropageY, cycle: 4});
    m.insert(0xAE, Opecode{ name: Instruction::LDX, mode: Addressing::Absolute, cycle: 4});
    m.insert(0xBE, Opecode{ name: Instruction::LDX, mode: Addressing::AbsoluteY, cycle: 4});
    m.insert(0xA0, Opecode{ name: Instruction::LDY, mode: Addressing::Immediate, cycle: 2});
    m.insert(0xA4, Opecode{ name: Instruction::LDY, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0xB4, Opecode{ name: Instruction::LDY, mode: Addressing::ZeropageX, cycle: 4});
    m.insert(0xAC, Opecode{ name: Instruction::LDY, mode: Addressing::Absolute, cycle: 4});
    m.insert(0xBC, Opecode{ name: Instruction::LDY, mode: Addressing::AbsoluteX, cycle: 4});
    m.insert(0x85, Opecode{ name: Instruction::STA, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0x95, Opecode{ name: Instruction::STA, mode: Addressing::ZeropageX, cycle: 4});
    m.insert(0x8D, Opecode{ name: Instruction::STA, mode: Addressing::Absolute, cycle: 4});
    m.insert(0x9D, Opecode{ name: Instruction::STA, mode: Addressing::AbsoluteX, cycle: 5});
    m.insert(0x99, Opecode{ name: Instruction::STA, mode: Addressing::AbsoluteY, cycle: 5});
    m.insert(0x81, Opecode{ name: Instruction::STA, mode: Addressing::IndirectX, cycle: 6});
    m.insert(0x91, Opecode{ name: Instruction::STA, mode: Addressing::IndirectY, cycle: 6});
    m.insert(0x86, Opecode{ name: Instruction::STX, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0x96, Opecode{ name: Instruction::STX, mode: Addressing::ZeropageY, cycle: 4});
    m.insert(0x8E, Opecode{ name: Instruction::STX, mode: Addressing::Absolute, cycle: 4});
    m
  };
}

