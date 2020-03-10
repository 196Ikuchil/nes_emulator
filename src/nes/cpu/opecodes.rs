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
  // undocumented
  LAX,
  SAX,
  DCP,
  ISB,
  SLO,
  RLA,
  SRE,
  RRA,
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
  AbsoluteIndirect,
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
    m.insert(0x21, Opecode{ name: Instruction::AND, mode: Addressing::IndirectX, cycle: 6});
    m.insert(0x31, Opecode{ name: Instruction::AND, mode: Addressing::IndirectY, cycle: 5});
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
    m.insert(0x49, Opecode{ name: Instruction::EOR, mode: Addressing::Immediate , cycle: 2});
    m.insert(0x45, Opecode{ name: Instruction::EOR, mode: Addressing::Zeropage , cycle: 3});
    m.insert(0x55, Opecode{ name: Instruction::EOR, mode: Addressing::ZeropageX , cycle: 4});
    m.insert(0x4D, Opecode{ name: Instruction::EOR, mode: Addressing::Absolute , cycle: 4});
    m.insert(0x5D, Opecode{ name: Instruction::EOR, mode: Addressing::AbsoluteX , cycle: 4});
    m.insert(0x59, Opecode{ name: Instruction::EOR, mode: Addressing::AbsoluteY , cycle: 4});
    m.insert(0x41, Opecode{ name: Instruction::EOR, mode: Addressing::IndirectX , cycle: 6});
    m.insert(0x51, Opecode{ name: Instruction::EOR, mode: Addressing::IndirectY , cycle: 5});
    m.insert(0xE6, Opecode{ name: Instruction::INC, mode: Addressing::Zeropage , cycle: 5});
    m.insert(0xF6, Opecode{ name: Instruction::INC, mode: Addressing::ZeropageX , cycle: 6});
    m.insert(0xEE, Opecode{ name: Instruction::INC, mode: Addressing::Absolute , cycle: 6});
    m.insert(0xFE, Opecode{ name: Instruction::INC, mode: Addressing::AbsoluteX , cycle: 7});
    m.insert(0xE8, Opecode{ name: Instruction::INX, mode: Addressing::Implied , cycle: 2});
    m.insert(0xC8, Opecode{ name: Instruction::INY, mode: Addressing::Implied , cycle: 2});
    m.insert(0x4A, Opecode{ name: Instruction::LSR, mode: Addressing::Accumulator, cycle: 2});
    m.insert(0x46, Opecode{ name: Instruction::LSR, mode: Addressing::Zeropage, cycle: 5});
    m.insert(0x56, Opecode{ name: Instruction::LSR, mode: Addressing::ZeropageX, cycle: 6});
    m.insert(0x4E, Opecode{ name: Instruction::LSR, mode: Addressing::Absolute, cycle: 6});
    m.insert(0x5E, Opecode{ name: Instruction::LSR, mode: Addressing::AbsoluteX, cycle: 7});
    m.insert(0x09, Opecode{ name: Instruction::ORA, mode: Addressing::Immediate, cycle: 2});
    m.insert(0x05, Opecode{ name: Instruction::ORA, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0x15, Opecode{ name: Instruction::ORA, mode: Addressing::ZeropageX, cycle: 4});
    m.insert(0x0D, Opecode{ name: Instruction::ORA, mode: Addressing::Absolute, cycle: 4});
    m.insert(0x1D, Opecode{ name: Instruction::ORA, mode: Addressing::AbsoluteX, cycle: 4});
    m.insert(0x19, Opecode{ name: Instruction::ORA, mode: Addressing::AbsoluteY, cycle: 4});
    m.insert(0x01, Opecode{ name: Instruction::ORA, mode: Addressing::IndirectX, cycle: 6});
    m.insert(0x11, Opecode{ name: Instruction::ORA, mode: Addressing::IndirectY, cycle: 5});
    m.insert(0x2A, Opecode{ name: Instruction::ROL, mode: Addressing::Accumulator, cycle: 2});
    m.insert(0x26, Opecode{ name: Instruction::ROL, mode: Addressing::Zeropage, cycle: 5});
    m.insert(0x36, Opecode{ name: Instruction::ROL, mode: Addressing::ZeropageX, cycle: 6});
    m.insert(0x2E, Opecode{ name: Instruction::ROL, mode: Addressing::Absolute, cycle: 6});
    m.insert(0x3E, Opecode{ name: Instruction::ROL, mode: Addressing::AbsoluteX, cycle: 7});
    m.insert(0x6A, Opecode{ name: Instruction::ROR, mode: Addressing::Accumulator, cycle: 2});
    m.insert(0x66, Opecode{ name: Instruction::ROR, mode: Addressing::Zeropage, cycle: 5});
    m.insert(0x76, Opecode{ name: Instruction::ROR, mode: Addressing::ZeropageX, cycle: 6});
    m.insert(0x6E, Opecode{ name: Instruction::ROR, mode: Addressing::Absolute, cycle: 6});
    m.insert(0x7E, Opecode{ name: Instruction::ROR, mode: Addressing::AbsoluteX, cycle: 7});
    m.insert(0xE9, Opecode{ name: Instruction::SBC, mode: Addressing::Immediate, cycle: 2});
    m.insert(0xE5, Opecode{ name: Instruction::SBC, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0xF5, Opecode{ name: Instruction::SBC, mode: Addressing::ZeropageX, cycle: 4});
    m.insert(0xED, Opecode{ name: Instruction::SBC, mode: Addressing::Absolute, cycle: 4});
    m.insert(0xFD, Opecode{ name: Instruction::SBC, mode: Addressing::AbsoluteX, cycle: 4});
    m.insert(0xF9, Opecode{ name: Instruction::SBC, mode: Addressing::AbsoluteY, cycle: 4});
    m.insert(0xE1, Opecode{ name: Instruction::SBC, mode: Addressing::IndirectX, cycle: 6});
    m.insert(0xF1, Opecode{ name: Instruction::SBC, mode: Addressing::IndirectY, cycle: 5});
    // stack
    m.insert(0x48, Opecode{ name: Instruction::PHA, mode: Addressing::Implied, cycle: 3});
    m.insert(0x08, Opecode{ name: Instruction::PHP, mode: Addressing::Implied, cycle: 3});
    m.insert(0x68, Opecode{ name: Instruction::PLA, mode: Addressing::Implied, cycle: 4});
    m.insert(0x28, Opecode{ name: Instruction::PLP, mode: Addressing::Implied, cycle: 4});
    // jump
    m.insert(0x4C, Opecode{ name: Instruction::JMP, mode: Addressing::Absolute, cycle: 3});
    m.insert(0x6C, Opecode{ name: Instruction::JMP, mode: Addressing::AbsoluteIndirect, cycle: 5});
    m.insert(0x20, Opecode{ name: Instruction::JSR, mode: Addressing::Absolute, cycle: 6});
    m.insert(0x60, Opecode{ name: Instruction::RTS, mode: Addressing::Implied, cycle: 6});
    m.insert(0x40, Opecode{ name: Instruction::RTI, mode: Addressing::Implied, cycle: 6});
    // branch
    m.insert(0x90, Opecode{ name: Instruction::BCC, mode: Addressing::Relative, cycle:2});
    m.insert(0xB0, Opecode{ name: Instruction::BCS, mode: Addressing::Relative, cycle:2});
    m.insert(0xF0, Opecode{ name: Instruction::BEQ, mode: Addressing::Relative, cycle:2});
    m.insert(0x30, Opecode{ name: Instruction::BMI, mode: Addressing::Relative, cycle:2});
    m.insert(0xD0, Opecode{ name: Instruction::BNE, mode: Addressing::Relative, cycle:2});
    m.insert(0x10, Opecode{ name: Instruction::BPL, mode: Addressing::Relative, cycle:2});
    m.insert(0x50, Opecode{ name: Instruction::BVC, mode: Addressing::Relative, cycle:2});
    m.insert(0x70, Opecode{ name: Instruction::BVS, mode: Addressing::Relative, cycle:2});
    // flag
    m.insert(0x18, Opecode{ name: Instruction::CLC, mode: Addressing::Implied, cycle: 2});
    m.insert(0xD8, Opecode{ name: Instruction::CLD, mode: Addressing::Implied, cycle: 2});
    m.insert(0x58, Opecode{ name: Instruction::CLI, mode: Addressing::Implied, cycle: 2});
    m.insert(0xB8, Opecode{ name: Instruction::CLV, mode: Addressing::Implied, cycle: 2});
    m.insert(0x38, Opecode{ name: Instruction::SEC, mode: Addressing::Implied, cycle: 2});
    m.insert(0xF8, Opecode{ name: Instruction::SED, mode: Addressing::Implied, cycle: 2});
    m.insert(0x78, Opecode{ name: Instruction::SEI, mode: Addressing::Implied, cycle: 2});
    // others
    m.insert(0x00, Opecode{ name: Instruction::BRK, mode: Addressing::Implied, cycle: 7});
    m.insert(0xEA, Opecode{ name: Instruction::NOP, mode: Addressing::Implied, cycle: 2});
    m.insert(0x1A, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x3A, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x5A, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x7A, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0xDA, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0xFA, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x02, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x12, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x22, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x32, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x42, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x52, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x62, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x72, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x92, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0xB2, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0xD2, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0xF2, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x80, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x82, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0x89, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0xC2, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 2 });
    m.insert(0xE2, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 3 });
    m.insert(0x04, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 3 });
    m.insert(0x44, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 3 });
    m.insert(0x64, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 3 });
    m.insert(0x14, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 4 });
    m.insert(0x34, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 4 });
    m.insert(0x54, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 4 });
    m.insert(0x74, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 4 });
    m.insert(0xD4, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 4 });
    m.insert(0xF4, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 4 });
    m.insert(0x0C, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 4 });
    m.insert(0x1C, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 4 });
    m.insert(0x3C, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 4 });
    m.insert(0x5C, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 4 });
    m.insert(0x7C, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 4 });
    m.insert(0xDC, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 4 });
    m.insert(0xFC, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: 4 });

    // undocumented
    m.insert(0xA7, Opecode { name: Instruction::LAX, mode: Addressing::Zeropage, cycle: 3 });
    m.insert(0xB7, Opecode { name: Instruction::LAX, mode: Addressing::ZeropageY, cycle: 3 });
    m.insert(0xAF, Opecode { name: Instruction::LAX, mode: Addressing::Absolute, cycle: 4 });
    m.insert(0xBF, Opecode { name: Instruction::LAX, mode: Addressing::AbsoluteY, cycle: 4 });
    m.insert(0xA3, Opecode { name: Instruction::LAX, mode: Addressing::IndirectX, cycle: 6 });
    m.insert(0xB3, Opecode { name: Instruction::LAX, mode: Addressing::IndirectY, cycle:5 });
    m.insert(0x87, Opecode { name: Instruction::SAX, mode: Addressing::Zeropage, cycle: 3});
    m.insert(0x97, Opecode { name: Instruction::SAX, mode: Addressing::ZeropageY, cycle: 4});
    m.insert(0x8F, Opecode { name: Instruction::SAX, mode: Addressing::Absolute, cycle: 4 });
    m.insert(0x83, Opecode { name: Instruction::SAX, mode: Addressing::IndirectX, cycle:6 });
    m.insert(0xC7, Opecode { name: Instruction::DCP, mode: Addressing::Zeropage, cycle: 5 });
    m.insert(0xD7, Opecode { name: Instruction::DCP, mode: Addressing::ZeropageX, cycle: 6 });
    m.insert(0xCF, Opecode { name: Instruction::DCP, mode: Addressing::Absolute, cycle: 6 });
    m.insert(0xDF, Opecode { name: Instruction::DCP, mode: Addressing::AbsoluteX, cycle: 7 });
    m.insert(0xDB, Opecode { name: Instruction::DCP, mode: Addressing::AbsoluteY, cycle:2 });
    m.insert(0xC3, Opecode { name: Instruction::DCP, mode: Addressing::IndirectX, cycle: 8 });
    m.insert(0xD3, Opecode { name: Instruction::DCP, mode: Addressing::IndirectY, cycle: 8 });
    m.insert(0xE7, Opecode { name: Instruction::ISB, mode: Addressing::Zeropage, cycle: 5 });
    m.insert(0xF7, Opecode { name: Instruction::ISB, mode: Addressing::ZeropageX, cycle:6 });
    m.insert(0xEF, Opecode { name: Instruction::ISB, mode: Addressing::Absolute, cycle: 6 });
    m.insert(0xFF, Opecode { name: Instruction::ISB, mode: Addressing::AbsoluteX, cycle: 7 });
    m.insert(0xFB, Opecode { name: Instruction::ISB, mode: Addressing::AbsoluteY, cycle: 2 });
    m.insert(0xE3, Opecode { name: Instruction::ISB, mode: Addressing::IndirectX, cycle: 8});
    m.insert(0xF3, Opecode { name: Instruction::ISB, mode: Addressing::IndirectY, cycle: 8});
    m.insert(0x07, Opecode { name: Instruction::SLO, mode: Addressing::Zeropage, cycle: 5 });
    m.insert(0x17, Opecode { name: Instruction::SLO, mode: Addressing::ZeropageX, cycle: 6 });
    m.insert(0x0F, Opecode { name: Instruction::SLO, mode: Addressing::Absolute, cycle: 6 });
    m.insert(0x1F, Opecode { name: Instruction::SLO, mode: Addressing::AbsoluteX, cycle:7 });
    m.insert(0x1B, Opecode { name: Instruction::SLO, mode: Addressing::AbsoluteY, cycle: 7 });
    m.insert(0x03, Opecode { name: Instruction::SLO, mode: Addressing::IndirectX, cycle: 8});
    m.insert(0x13, Opecode { name: Instruction::SLO, mode: Addressing::IndirectY, cycle: 8 });
    m.insert(0x27, Opecode { name: Instruction::RLA, mode: Addressing::Zeropage, cycle: 5 });
    m.insert(0x37, Opecode { name: Instruction::RLA, mode: Addressing::ZeropageX, cycle: 6 });
    m.insert(0x2F, Opecode { name: Instruction::RLA, mode: Addressing::Absolute, cycle:6});
    m.insert(0x3F, Opecode { name: Instruction::RLA, mode: Addressing::AbsoluteX, cycle:7});
    m.insert(0x3B, Opecode { name: Instruction::RLA, mode: Addressing::AbsoluteY, cycle: 7 });
    m.insert(0x23, Opecode { name: Instruction::RLA, mode: Addressing::IndirectX, cycle: 8 });
    m.insert(0x33, Opecode { name: Instruction::RLA, mode: Addressing::IndirectY, cycle: 8 }, );
    m.insert(0x47, Opecode { name: Instruction::SRE, mode: Addressing::Zeropage, cycle: 5});
    m.insert(0x57, Opecode { name: Instruction::SRE, mode: Addressing::ZeropageX, cycle: 6 });
    m.insert(0x4F, Opecode { name: Instruction::SRE, mode: Addressing::Absolute, cycle: 6 });
    m.insert(0x5F, Opecode { name: Instruction::SRE, mode: Addressing::AbsoluteX, cycle:7 });
    m.insert(0x5B, Opecode { name: Instruction::SRE, mode: Addressing::AbsoluteY, cycle:7});
    m.insert(0x43, Opecode { name: Instruction::SRE, mode: Addressing::IndirectX, cycle: 8 });
    m.insert(0x53, Opecode { name: Instruction::SRE, mode: Addressing::IndirectY, cycle: 8 }, );
    m.insert(0x67, Opecode { name: Instruction::RRA, mode: Addressing::Zeropage, cycle: 5});
    m.insert(0x77, Opecode { name: Instruction::RRA, mode: Addressing::ZeropageX, cycle: 6});
    m.insert(0x6F, Opecode { name: Instruction::RRA, mode: Addressing::Absolute, cycle: 6 });
    m.insert(0x7F, Opecode { name: Instruction::RRA, mode: Addressing::AbsoluteX, cycle: 7 });
    m.insert(0x7B, Opecode { name: Instruction::RRA, mode: Addressing::AbsoluteY, cycle:7 });
    m.insert(0x63, Opecode { name: Instruction::RRA, mode: Addressing::IndirectX, cycle: 8 });
    m.insert(0x73, Opecode { name: Instruction::RRA, mode: Addressing::IndirectY, cycle: 8});

    // nestest
    m.insert(0xEB, Opecode { name: Instruction::SBC, mode: Addressing::Immediate, cycle: 2 });
    m
  };
}

