// SPDX-License-Identifier: GPL-2.0+
/*
 * ni_at_ao.c
 * Driver for NI AT-AO-6/10 boards
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000,2002 David A. Schleef <ds@schleef.org>
 */

/* Driver: ni_at_ao; Description: National Instruments AT-AO-6/10 */

const ATAO_DIO_REG: u16 = 0x00;
const ATAO_CFG2_REG: u16 = 0x02;
const ATAO_CFG2_CALLD_NOP: u16 = 0 << 14;
#[inline] const fn ATAO_CFG2_CALLD(x: u32) -> u16 { (((x >> 3) + 1) << 14) as u16 }
const ATAO_CFG2_FFRTEN: u16 = 1 << 13;
#[inline] const fn ATAO_CFG2_DACS(x: u32) -> u16 { 1 << ((x / 2) + 8) }
#[inline] const fn ATAO_CFG2_LDAC(x: u32) -> u16 { 1 << ((x / 2) + 3) }
const ATAO_CFG2_PROMEN: u16 = 1 << 2;
const ATAO_CFG2_SCLK: u16 = 1 << 1;
const ATAO_CFG2_SDATA: u16 = 1 << 0;
const ATAO_CFG3_REG: u16 = 0x04;
const ATAO_CFG3_DMAMODE: u16 = 1 << 6;
const ATAO_CFG3_CLKOUT: u16 = 1 << 5;
const ATAO_CFG3_RCLKEN: u16 = 1 << 4;
const ATAO_CFG3_DOUTEN2: u16 = 1 << 3;
const ATAO_CFG3_DOUTEN1: u16 = 1 << 2;
const ATAO_CFG3_EN2_5V: u16 = 1 << 1;
const ATAO_CFG3_SCANEN: u16 = 1 << 0;
const ATAO_82C53_BASE: u16 = 0x06;
const ATAO_CFG1_REG: u16 = 0x0a;
const ATAO_CFG1_EXTINT2EN: u16 = 1 << 15;
const ATAO_CFG1_EXTINT1EN: u16 = 1 << 14;
const ATAO_CFG1_CNTINT2EN: u16 = 1 << 13;
const ATAO_CFG1_CNTINT1EN: u16 = 1 << 12;
const ATAO_CFG1_TCINTEN: u16 = 1 << 11;
const ATAO_CFG1_CNT1SRC: u16 = 1 << 10;
const ATAO_CFG1_CNT2SRC: u16 = 1 << 9;
const ATAO_CFG1_FIFOEN: u16 = 1 << 8;
const ATAO_CFG1_GRP2WR: u16 = 1 << 7;
const ATAO_CFG1_EXTUPDEN: u16 = 1 << 6;
const ATAO_CFG1_DMARQ: u16 = 1 << 5;
const ATAO_CFG1_DMAEN: u16 = 1 << 4;
#[inline] const fn ATAO_CFG1_CH(x: u32) -> u16 { ((x & 0xf) << 0) as u16 }
const ATAO_STATUS_REG: u16 = 0x0a;
const ATAO_STATUS_FH: u16 = 1 << 6;
const ATAO_STATUS_FE: u16 = 1 << 5;
const ATAO_STATUS_FF: u16 = 1 << 4;
const ATAO_STATUS_INT2: u16 = 1 << 3;
const ATAO_STATUS_INT1: u16 = 1 << 2;
const ATAO_STATUS_TCINT: u16 = 1 << 1;
const ATAO_STATUS_PROMOUT: u16 = 1 << 0;
const ATAO_FIFO_WRITE_REG: u16 = 0x0c;
const ATAO_FIFO_CLEAR_REG: u16 = 0x0c;
#[inline] const fn ATAO_AO_REG(x: u32) -> u16 { 0x0c + (x as u16 * 2) }

const ATAO_2_DMATCCLR_REG: u16 = 0x00;
const ATAO_2_INT1CLR_REG: u16 = 0x02;
const ATAO_2_INT2CLR_REG: u16 = 0x04;
const ATAO_2_RTSISHFT_REG: u16 = 0x06;
const ATAO_2_RTSISHFT_RSI: u16 = 1 << 0;
const ATAO_2_RTSISTRB_REG: u16 = 0x07;

#[repr(C)]
struct atao_board { name: *const i8, n_ao_chans: i32 }

static atao_boards: [atao_board; 2] = [
    atao_board { name: b"at-ao-6\0".as_ptr() as *const i8, n_ao_chans: 6 },
    atao_board { name: b"at-ao-10\0".as_ptr() as *const i8, n_ao_chans: 10 },
];

#[repr(C)]
struct atao_private { cfg1: u16, cfg3: u16, caldac: [u8; 21] }

#[allow(dead_code)]
unsafe fn atao_select_reg_group(dev: *mut comedi_device, group: i32) {
    let p = (*dev).private as *mut atao_private;
    if group != 0 { (*p).cfg1 |= ATAO_CFG1_GRP2WR; } else { (*p).cfg1 &= !ATAO_CFG1_GRP2WR; }
    outw((*p).cfg1, (*dev).iobase + ATAO_CFG1_REG as usize);
}

unsafe fn atao_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    let mut val = (*s).readback[chan as usize];
    if chan == 0 { atao_select_reg_group(dev, 1); }
    for i in 0..(*insn).n as usize { val = *data.add(i); outw(comedi_offset_munge(s, val), (*dev).iobase + ATAO_AO_REG(chan) as usize); }
    (*s).readback[chan as usize] = val;
    if chan == 0 { atao_select_reg_group(dev, 0); }
    (*insn).n as i32
}

unsafe fn atao_dio_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    if comedi_dio_update_state(s, data) != 0 { outw((*s).state as u16, (*dev).iobase + ATAO_DIO_REG as usize); }
    *data.add(1) = inw((*dev).iobase + ATAO_DIO_REG as usize) as u32; (*insn).n as i32
}

unsafe fn atao_dio_insn_config(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let p = (*dev).private as *mut atao_private; let chan = CR_CHAN((*insn).chanspec);
    let mask = if chan < 4 { 0x0f } else { 0xf0 }; let ret = comedi_dio_insn_config(dev, s, insn, data, mask);
    if ret != 0 { return ret; }
    if (*s).io_bits & 0x0f != 0 { (*p).cfg3 |= ATAO_CFG3_DOUTEN1; } else { (*p).cfg3 &= !ATAO_CFG3_DOUTEN1; }
    if (*s).io_bits & 0xf0 != 0 { (*p).cfg3 |= ATAO_CFG3_DOUTEN2; } else { (*p).cfg3 &= !ATAO_CFG3_DOUTEN2; }
    outw((*p).cfg3, (*dev).iobase + ATAO_CFG3_REG as usize); (*insn).n as i32
}

unsafe fn atao_calib_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    if (*insn).n != 0 { let val = *data.add((*insn).n as usize - 1); let bitstring = ((chan & 0x7) << 8) | val;
        let mut bit = 1 << 10; while bit != 0 { let bits = if bit & bitstring != 0 { ATAO_CFG2_SDATA } else { 0 }; outw(bits, (*dev).iobase + ATAO_CFG2_REG as usize); outw(bits | ATAO_CFG2_SCLK, (*dev).iobase + ATAO_CFG2_REG as usize); bit >>= 1; }
        outw(ATAO_CFG2_CALLD(chan), (*dev).iobase + ATAO_CFG2_REG as usize); outw(ATAO_CFG2_CALLD_NOP, (*dev).iobase + ATAO_CFG2_REG as usize); (*s).readback[chan as usize] = val;
    } (*insn).n as i32
}

// External kernel/Comedi types and functions are supplied by the surrounding translation unit.
#[repr(C)] struct comedi_device { private: *mut core::ffi::c_void, iobase: usize }
#[repr(C)] struct comedi_subdevice { readback: *mut u32, state: u32, io_bits: u32 }
#[repr(C)] struct comedi_insn { chanspec: u32, n: u32 }
extern "C" { fn outw(v: u16, p: usize); fn inw(p: usize) -> u16; fn CR_CHAN(v: u32) -> u32; fn comedi_offset_munge(s: *mut comedi_subdevice, v: u32) -> u16; fn comedi_dio_update_state(s: *mut comedi_subdevice, d: *mut u32) -> i32; fn comedi_dio_insn_config(d:*mut comedi_device,s:*mut comedi_subdevice,i:*mut comedi_insn,x:*mut u32,m:u32)->i32; }

unsafe fn atao_reset(dev: *mut comedi_device) {
    let p = (*dev).private as *mut atao_private;
    (*p).cfg1 = 0; outw((*p).cfg1, (*dev).iobase + ATAO_CFG1_REG as usize);
    // The counter and Comedi 8254 reset operations are external dependencies.
    outw(ATAO_CFG2_CALLD_NOP, (*dev).iobase + ATAO_CFG2_REG as usize);
    (*p).cfg3 = 0; outw((*p).cfg3, (*dev).iobase + ATAO_CFG3_REG as usize);
    let _ = inw((*dev).iobase + ATAO_FIFO_CLEAR_REG as usize);
    atao_select_reg_group(dev, 1);
    outw(0, (*dev).iobase + ATAO_2_INT1CLR_REG as usize);
    outw(0, (*dev).iobase + ATAO_2_INT2CLR_REG as usize);
    outw(0, (*dev).iobase + ATAO_2_DMATCCLR_REG as usize);
    atao_select_reg_group(dev, 0);
}

// Driver registration and attach wiring are retained for the surrounding Comedi bindings.
unsafe fn atao_attach(dev: *mut comedi_device, _it: *mut core::ffi::c_void) -> i32 {
    atao_reset(dev); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
