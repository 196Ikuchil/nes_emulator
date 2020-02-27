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

    m
  };
}

