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
    // transport
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
    m.insert(0x84, Opecode{ name: Instruction::STY, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0x94, Opecode{ name: Instruction::STY, mode: Addressing::ZeropageX, cycle: 4});
    m.insert(0x8C, Opecode{ name: Instruction::STY, mode: Addressing::Absolute, cycle: 4});
    m.insert(0xAA, Opecode{ name: Instruction::TAX, mode: Addressing::Implied, cycle: 2});
    m.insert(0xA8, Opecode{ name: Instruction::TAY, mode: Addressing::Implied, cycle: 2});
    m.insert(0xBA, Opecode{ name: Instruction::TSX, mode: Addressing::Implied, cycle: 2});
    m.insert(0x8A, Opecode{ name: Instruction::TXA, mode: Addressing::Implied, cycle: 2});
    m.insert(0x9A, Opecode{ name: Instruction::TXS, mode: Addressing::Implied, cycle: 2});
    m.insert(0x98, Opecode{ name: Instruction::TYA, mode: Addressing::Implied, cycle: 2});
    // calculate
    m.insert(0x69, Opecode{ name: Instruction::ADC, mode: Addressing::Immediate, cycle: 2});
    m.insert(0x65, Opecode{ name: Instruction::ADC, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0x75, Opecode{ name: Instruction::ADC, mode: Addressing::ZeropageX, cycle: 4});
    m.insert(0x6D, Opecode{ name: Instruction::ADC, mode: Addressing::Absolute, cycle: 4});
    m.insert(0x7D, Opecode{ name: Instruction::ADC, mode: Addressing::AbsoluteX, cycle: 4});
    m.insert(0x79, Opecode{ name: Instruction::ADC, mode: Addressing::AbsoluteY, cycle: 4});
    m.insert(0x61, Opecode{ name: Instruction::ADC, mode: Addressing::IndirectX, cycle: 6});
    m.insert(0x71, Opecode{ name: Instruction::ADC, mode: Addressing::IndirectY, cycle: 5});
    m.insert(0x29, Opecode{ name: Instruction::AND, mode: Addressing::Immediate, cycle: 2});
    m.insert(0x25, Opecode{ name: Instruction::AND, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0x35, Opecode{ name: Instruction::AND, mode: Addressing::ZeropageX, cycle: 4});
    m.insert(0x2D, Opecode{ name: Instruction::AND, mode: Addressing::Absolute, cycle: 4});
    m.insert(0x3D, Opecode{ name: Instruction::AND, mode: Addressing::AbsoluteX, cycle: 4});
    m.insert(0x39, Opecode{ name: Instruction::AND, mode: Addressing::AbsoluteY, cycle: 4});
    m.insert(0x21, Opecode{ name: Instruction::AND, mode: Addressing::IndirectX, cycle: 4});
    m.insert(0x31, Opecode{ name: Instruction::AND, mode: Addressing::IndirectY, cycle: 4});
    m.insert(0x0A, Opecode{ name: Instruction::ASL, mode: Addressing::Accumulator, cycle: 2});
    m.insert(0x06, Opecode{ name: Instruction::ASL, mode: Addressing::Zeropage, cycle: 5});
    m.insert(0x16, Opecode{ name: Instruction::ASL, mode: Addressing::ZeropageX, cycle: 6});
    m.insert(0x0E, Opecode{ name: Instruction::ASL, mode: Addressing::Absolute, cycle: 6});
    m.insert(0x1E, Opecode{ name: Instruction::ASL, mode: Addressing::AbsoluteX, cycle: 7});
    m.insert(0x24, Opecode{ name: Instruction::BIT, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0x2C, Opecode{ name: Instruction::BIT, mode: Addressing::Absolute, cycle: 4});
    m.insert(0xC9, Opecode{ name: Instruction::CMP, mode: Addressing::Immediate, cycle: 2});
    m.insert(0xC5, Opecode{ name: Instruction::CMP, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0xD5, Opecode{ name: Instruction::CMP, mode: Addressing::ZeropageX, cycle: 4});
    m.insert(0xCD, Opecode{ name: Instruction::CMP, mode: Addressing::Absolute, cycle: 4});
    m.insert(0xDD, Opecode{ name: Instruction::CMP, mode: Addressing::AbsoluteX, cycle: 4});
    m.insert(0xD9, Opecode{ name: Instruction::CMP, mode: Addressing::AbsoluteY, cycle: 4});
    m.insert(0xC1, Opecode{ name: Instruction::CMP, mode: Addressing::IndirectX, cycle: 6});
    m.insert(0xD1, Opecode{ name: Instruction::CMP, mode: Addressing::IndirectY, cycle: 5});
    m.insert(0xE0, Opecode{ name: Instruction::CPX, mode: Addressing::Immediate, cycle: 2});
    m.insert(0xE4, Opecode{ name: Instruction::CPX, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0xEC, Opecode{ name: Instruction::CPX, mode: Addressing::Absolute, cycle: 4});
    m.insert(0xC0, Opecode{ name: Instruction::CPY, mode: Addressing::Immediate, cycle: 2});
    m.insert(0xC4, Opecode{ name: Instruction::CPY, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0xCC, Opecode{ name: Instruction::CPY, mode: Addressing::Absolute, cycle: 4});
    m.insert(0xC6, Opecode{ name: Instruction::DEC, mode: Addressing::Zeropage, cycle: 5});
    m.insert(0xD6, Opecode{ name: Instruction::DEC, mode: Addressing::ZeropageX, cycle: 6});
    m.insert(0xCE, Opecode{ name: Instruction::DEC, mode: Addressing::Absolute, cycle: 6});
    m.insert(0xDE, Opecode{ name: Instruction::DEC, mode: Addressing::AbsoluteX, cycle: 7});
    m.insert(0xCA, Opecode{ name: Instruction::DEX, mode: Addressing::Implied, cycle: 2});
    m.insert(0x88, Opecode{ name: Instruction::DEY, mode: Addressing::Implied, cycle: 2});
    m

  };
}

