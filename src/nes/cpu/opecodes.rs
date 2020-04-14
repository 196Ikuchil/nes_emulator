use std::collections::HashMap;
use lazy_static::lazy_static;
use self::Instruction::*;
use self::Addressing::*;


#[derive(Debug)]
pub struct Opecode {
  pub name: Instruction,
  pub mode: Addressing,
  pub cycle: u8,
}

#[derive(Debug, Copy, Clone)]
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
  ISC,
  SLO,
  RLA,
  SRE,
  RRA,

  // current not inplemented
  ANC,
  ALR,
  ARR,
  XAA,
  AHX,
  TAS,
  SHY,
  SHX,
  LAS,
  AXS,

}

#[derive(Debug, PartialEq)]
pub enum Addressing {
  Absolute,
  AbsoluteX,
  AbsoluteY,
  Accumulator,
  Immediate,
  Implied,
  IndexedIndirect,
  AbsoluteIndirect,
  IndirectIndexed,
  Relative,
  Zeropage,
  ZeropageX,
  ZeropageY,
}

trait AddressingOffset {
  fn offset(index: u8) -> Addressing;
}

impl AddressingOffset for Addressing {
  fn offset(index: u8) -> Addressing {
    match index {
      0 => Absolute,
      1 => AbsoluteX,
      2 => AbsoluteY,
      3 => Accumulator,
      4 => Immediate,
      5 => Implied,
      6 => IndexedIndirect,
      7 => AbsoluteIndirect,
      8 => IndirectIndexed,
      9 => Relative,
      10 => Zeropage,
      11 => ZeropageX,
      12 => ZeropageY,
      _ => panic!("unfollowed addressing"),
    }
  }
}

pub const INSTRUCTION_NAMES: &'static [Instruction] = &[
  BRK, ORA, NOP, SLO, NOP, ORA, ASL, SLO,
  PHP, ORA, ASL, ANC, NOP, ORA, ASL, SLO,
  BPL, ORA, NOP, SLO, NOP, ORA, ASL, SLO,
  CLC, ORA, NOP, SLO, NOP, ORA, ASL, SLO,
  JSR, AND, NOP, RLA, BIT, AND, ROL, RLA,
  PLP, AND, ROL, ANC, BIT, AND, ROL, RLA,
  BMI, AND, NOP, RLA, NOP, AND, ROL, RLA,
  SEC, AND, NOP, RLA, NOP, AND, ROL, RLA,
  RTI, EOR, NOP, SRE, NOP, EOR, LSR, SRE,
  PHA, EOR, LSR, ALR, JMP, EOR, LSR, SRE,
  BVC, EOR, NOP, SRE, NOP, EOR, LSR, SRE,
  CLI, EOR, NOP, SRE, NOP, EOR, LSR, SRE,
  RTS, ADC, NOP, RRA, NOP, ADC, ROR, RRA,
  PLA, ADC, ROR, ARR, JMP, ADC, ROR, RRA,
  BVS, ADC, NOP, RRA, NOP, ADC, ROR, RRA,
  SEI, ADC, NOP, RRA, NOP, ADC, ROR, RRA,
  NOP, STA, NOP, SAX, STY, STA, STX, SAX,
  DEY, NOP, TXA, XAA, STY, STA, STX, SAX,
  BCC, STA, NOP, AHX, STY, STA, STX, SAX,
  TYA, STA, TXS, TAS, SHY, STA, SHX, AHX,
  LDY, LDA, LDX, LAX, LDY, LDA, LDX, LAX,
  TAY, LDA, TAX, LAX, LDY, LDA, LDX, LAX,
  BCS, LDA, NOP, LAX, LDY, LDA, LDX, LAX,
  CLV, LDA, TSX, LAS, LDY, LDA, LDX, LAX,
  CPY, CMP, NOP, DCP, CPY, CMP, DEC, DCP,
  INY, CMP, DEX, AXS, CPY, CMP, DEC, DCP,
  BNE, CMP, NOP, DCP, NOP, CMP, DEC, DCP,
  CLD, CMP, NOP, DCP, NOP, CMP, DEC, DCP,
  CPX, SBC, NOP, ISC, CPX, SBC, INC, ISC,
  INX, SBC, NOP, SBC, CPX, SBC, INC, ISC,
  BEQ, SBC, NOP, ISC, NOP, SBC, INC, ISC,
  SED, SBC, NOP, ISC, NOP, SBC, INC, ISC,
];

pub const INSTRUCTION_MODES: &'static [u8]  = &[
	5, 6, 5, 6, 10, 10, 10, 10, 5, 4, 3, 4, 0, 0, 0, 0,
	9, 8, 5, 8, 11, 11, 11, 11, 5, 2, 5, 2, 1, 1, 1, 1,
	0, 6, 5, 6, 10, 10, 10, 10, 5, 4, 3, 4, 0, 0, 0, 0,
	9, 8, 5, 8, 11, 11, 11, 11, 5, 2, 5, 2, 1, 1, 1, 1,
	5, 6, 5, 6, 10, 10, 10, 10, 5, 4, 3, 4, 0, 0, 0, 0,
	9, 8, 5, 8, 11, 11, 11, 11, 5, 2, 5, 2, 1, 1, 1, 1,
	5, 6, 5, 6, 10, 10, 10, 10, 5, 4, 3, 4, 7, 0, 0, 0,
	9, 8, 5, 8, 11, 11, 11, 11, 5, 2, 5, 2, 1, 1, 1, 1,
	4, 6, 4, 6, 10, 10, 10, 10, 5, 4, 5, 4, 0, 0, 0, 0,
	9, 8, 5, 8, 11, 11, 12, 12, 5, 2, 5, 2, 1, 1, 2, 2,
	4, 6, 4, 6, 10, 10, 10, 10, 5, 4, 5, 4, 0, 0, 0, 0,
	9, 8, 5, 8, 11, 11, 12, 12, 5, 2, 5, 2, 1, 1, 2, 2,
	4, 6, 4, 6, 10, 10, 10, 10, 5, 4, 5, 4, 0, 0, 0, 0,
	9, 8, 5, 8, 11, 11, 11, 11, 5, 2, 5, 2, 1, 1, 1, 1,
	4, 6, 4, 6, 10, 10, 10, 10, 5, 4, 5, 4, 0, 0, 0, 0,
	9, 8, 5, 8, 11, 11, 11, 11, 5, 2, 5, 2, 1, 1, 1, 1,
];

pub const INSTRUCTION_CYCLES: &'static [u8]  = &[
	7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6,
	2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
	6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6,
	2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
	6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6,
	2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
	6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6,
	2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
	2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4,
	2, 6, 2, 6, 4, 4, 4, 4, 2, 5, 2, 5, 5, 5, 5, 5,
	2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4,
	2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4,
	2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6,
	2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
	2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6,
	2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
];

pub const INSTRUCTION_PAGE_CROSS_CYCLES: &'static [u8]  = &[
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0,
];


lazy_static! {
  pub static ref OPEMAP: HashMap<u8, Opecode> = {
    let mut m = HashMap::new();
    for i in 0x0..0x100 {
      m.insert(i as u8 , Opecode{ name: INSTRUCTION_NAMES[i], mode: Addressing::offset(INSTRUCTION_MODES[i]), cycle: INSTRUCTION_CYCLES[i]});
    }
    m
  };
}

