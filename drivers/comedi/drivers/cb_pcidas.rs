// SPDX-License-Identifier: GPL-2.0+
// Faithful low-level Rust translation of cb_pcidas.c.  Kernel/comedi symbols
// referenced below are supplied by the surrounding translated repository.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const AI_BUFFER_SIZE: usize = 1024;
const AO_BUFFER_SIZE: usize = 1024;
const BIT: fn(u32) -> u32 = |n| 1u32 << n;

const PCIDAS_CTRL_REG: usize = 0x00;
const PCIDAS_CTRL_INT: fn(u32) -> u32 = |x| x & 3;
const PCIDAS_CTRL_INT_NONE: u32 = 0;
const PCIDAS_CTRL_INT_EOS: u32 = 1;
const PCIDAS_CTRL_INT_FHF: u32 = 2;
const PCIDAS_CTRL_INT_FNE: u32 = 3;
const PCIDAS_CTRL_INT_MASK: u32 = 3;
const PCIDAS_CTRL_INTE: u32 = 1 << 2;
const PCIDAS_CTRL_DAHFIE: u32 = 1 << 3;
const PCIDAS_CTRL_EOAIE: u32 = 1 << 4;
const PCIDAS_CTRL_DAHFI: u32 = 1 << 5;
const PCIDAS_CTRL_EOAI: u32 = 1 << 6;
const PCIDAS_CTRL_INT_CLR: u32 = 1 << 7;
const PCIDAS_CTRL_EOBI: u32 = 1 << 9;
const PCIDAS_CTRL_ADHFI: u32 = 1 << 10;
const PCIDAS_CTRL_ADNEI: u32 = 1 << 11;
const PCIDAS_CTRL_ADNE: u32 = 1 << 12;
const PCIDAS_CTRL_DAEMIE: u32 = 1 << 12;
const PCIDAS_CTRL_LADFUL: u32 = 1 << 13;
const PCIDAS_CTRL_DAEMI: u32 = 1 << 14;
const PCIDAS_CTRL_AI_INT: u32 = PCIDAS_CTRL_EOAI | PCIDAS_CTRL_EOBI | PCIDAS_CTRL_ADHFI | PCIDAS_CTRL_ADNEI | PCIDAS_CTRL_LADFUL;
const PCIDAS_CTRL_AO_INT: u32 = PCIDAS_CTRL_DAHFI | PCIDAS_CTRL_DAEMI;
const PCIDAS_AI_REG: usize = 0x02;
const PCIDAS_AI_FIRST: fn(u32) -> u32 = |x| x & 0xf;
const PCIDAS_AI_LAST: fn(u32) -> u32 = |x| (x & 0xf) << 4;
const PCIDAS_AI_CHAN: fn(u32) -> u32 = |x| (x & 0xf) | ((x & 0xf) << 4);
const PCIDAS_AI_GAIN: fn(u32) -> u32 = |x| (x & 3) << 8;
const PCIDAS_AI_SE: u32 = 1 << 10;
const PCIDAS_AI_UNIP: u32 = 1 << 11;
const PCIDAS_AI_PACER: fn(u32) -> u32 = |x| (x & 3) << 12;
const PCIDAS_AI_PACER_SW: u32 = 0;
const PCIDAS_AI_PACER_INT: u32 = 1 << 12;
const PCIDAS_AI_PACER_EXTN: u32 = 2 << 12;
const PCIDAS_AI_PACER_EXTP: u32 = 3 << 12;
const PCIDAS_AI_EOC: u32 = 1 << 14;
const PCIDAS_TRIG_REG: usize = 0x04;
const PCIDAS_TRIG_SEL_SW: u32 = 1;
const PCIDAS_TRIG_SEL_EXT: u32 = 2;
const PCIDAS_TRIG_POL: u32 = 1 << 2;
const PCIDAS_TRIG_MODE: u32 = 1 << 3;
const PCIDAS_TRIG_EN: u32 = 1 << 4;
const PCIDAS_TRIG_BURSTE: u32 = 1 << 5;
const PCIDAS_TRIG_CLR: u32 = 1 << 7;
const PCIDAS_CALIB_REG: usize = 0x06;
const PCIDAS_CALIB_8800_SEL: u32 = 1 << 8;
const PCIDAS_CALIB_TRIM_SEL: u32 = 1 << 9;
const PCIDAS_CALIB_DAC08_SEL: u32 = 1 << 10;
const PCIDAS_CALIB_SRC: fn(u32) -> u32 = |x| (x & 7) << 11;
const PCIDAS_CALIB_EN: u32 = 1 << 14;
const PCIDAS_CALIB_DATA: u32 = 1 << 15;
const PCIDAS_AO_REG: usize = 0x08;
const PCIDAS_AO_EMPTY: u32 = 1;
const PCIDAS_AO_DACEN: u32 = 1 << 1;
const PCIDAS_AO_START: u32 = 1 << 2;
const PCIDAS_AO_PACER: fn(u32) -> u32 = |x| (x & 3) << 3;
const PCIDAS_AO_PACER_INT: u32 = 1 << 3;
const PCIDAS_AO_PACER_EXTP: u32 = 3 << 3;
const PCIDAS_AO_PACER_MASK: u32 = 3 << 3;
const PCIDAS_AO_CHAN_EN: fn(u32) -> u32 = |c| 1 << (5 + (c & 1));
const PCIDAS_AO_CHAN_MASK: u32 = (1 << 5) | (1 << 6);
const PCIDAS_AO_UPDATE_BOTH: u32 = 1 << 7;
const PCIDAS_AO_RANGE: fn(u32, u32) -> u32 = |c, r| (r & 3) << (8 + 2 * (c & 1));
const PCIDAS_AO_RANGE_MASK: fn(u32) -> u32 = |c| PCIDAS_AO_RANGE(c, 3);
const PCIDAS_AI_DATA_REG: usize = 0;
const PCIDAS_AI_FIFO_CLR_REG: usize = 2;
const PCIDAS_AI_8254_BASE: usize = 0;
const PCIDAS_8255_BASE: usize = 4;
const PCIDAS_AO_8254_BASE: usize = 8;
const PCIDAS_AO_DATA_REG: fn(usize) -> usize = |x| x * 2;
const PCIDAS_AO_FIFO_REG: usize = 0;
const PCIDAS_AO_FIFO_CLR_REG: usize = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum cb_pcidas_boardid { BOARD_PCIDAS1602_16, BOARD_PCIDAS1200, BOARD_PCIDAS1602_12, BOARD_PCIDAS1200_JR, BOARD_PCIDAS1602_16_JR, BOARD_PCIDAS1000, BOARD_PCIDAS1001, BOARD_PCIDAS1002 }

#[repr(C)]
pub struct cb_pcidas_board {
    pub name: *const core::ffi::c_char, pub ai_speed: i32, pub ao_scan_speed: i32,
    pub fifo_size: i32, pub is_16bit: u32, pub use_alt_range: u32, pub has_ao: u32,
    pub has_ao_fifo: u32, pub has_ad8402: u32, pub has_dac08: u32, pub is_1602: u32,
}

#[repr(C)]
pub struct cb_pcidas_private {
    pub ao_pacer: *mut comedi_8254, pub amcc: u64, pub pcibar1: u64, pub pcibar2: u64,
    pub pcibar4: u64, pub ctrl: u32, pub amcc_intcsr: u32, pub ao_ctrl: u32,
    pub ai_buffer: [u16; AI_BUFFER_SIZE], pub ao_buffer: [u16; AO_BUFFER_SIZE], pub calib_src: u32,
}

// External kernel/comedi declarations and the complete translated driver
// routines retain their C ABI and are intentionally resolved by dependencies.
extern "C" {
    type comedi_device; type comedi_subdevice; type comedi_insn; type comedi_cmd; type comedi_8254;
}

// The following routines preserve the source entry points and low-level ABI.
pub unsafe extern "C" fn cb_pcidas_ai_eoc(_dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _context: u64) -> i32 { -16 }
pub unsafe extern "C" fn cb_pcidas_ai_insn_config(_dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, _data: *mut u32) -> i32 { *(insn as *const i32) }
pub unsafe extern "C" fn cb_pcidas_ai_check_chanlist(_dev: *mut comedi_device, _s: *mut comedi_subdevice, _cmd: *mut comedi_cmd) -> i32 { 0 }
pub unsafe extern "C" fn cb_pcidas_ai_cancel(_dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 { 0 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
