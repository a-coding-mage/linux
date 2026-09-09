// SPDX-License-Identifier: GPL-2.0+
/*
 * multiq3.c
 * Hardware driver for Quanser Consulting MultiQ-3 board
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1999 Anders Blomdell <anders.blomdell@control.lth.se>
 */

// Driver: multiq3
// Description: Quanser Consulting MultiQ-3
// Devices: [Quanser Consulting] MultiQ-3 (multiq3)
// Author: Anders Blomdell <anders.blomdell@control.lth.se>
// Status: works
// Configuration options: base I/O address, IRQ (unused), optional encoder chips.

const MULTIQ3_DI_REG: u16 = 0x00;
const MULTIQ3_DO_REG: u16 = 0x00;
const MULTIQ3_AO_REG: u16 = 0x02;
const MULTIQ3_AI_REG: u16 = 0x04;
const MULTIQ3_AI_CONV_REG: u16 = 0x04;
const MULTIQ3_STATUS_REG: u16 = 0x06;
const MULTIQ3_STATUS_EOC: u16 = 1 << 3;
const MULTIQ3_STATUS_EOC_I: u16 = 1 << 4;
const MULTIQ3_CTRL_REG: u16 = 0x06;
#[inline] fn MULTIQ3_CTRL_AO_CHAN(x: u16) -> u16 { (x & 0x7) << 0 }
#[inline] fn MULTIQ3_CTRL_RC(x: u16) -> u16 { (x & 0x3) << 0 }
#[inline] fn MULTIQ3_CTRL_AI_CHAN(x: u16) -> u16 { (x & 0x7) << 3 }
#[inline] fn MULTIQ3_CTRL_E_CHAN(x: u16) -> u16 { (x & 0x7) << 3 }
const MULTIQ3_CTRL_EN: u16 = 1 << 6;
const MULTIQ3_CTRL_AZ: u16 = 1 << 7;
const MULTIQ3_CTRL_CAL: u16 = 1 << 8;
const MULTIQ3_CTRL_SH: u16 = 1 << 9;
const MULTIQ3_CTRL_CLK: u16 = 1 << 10;
const MULTIQ3_CTRL_LD: u16 = 3 << 11;
const MULTIQ3_CLK_REG: u16 = 0x08;
const MULTIQ3_ENC_DATA_REG: u16 = 0x0c;
const MULTIQ3_ENC_CTRL_REG: u16 = 0x0e;

const MULTIQ3_CLOCK_DATA: u8 = 0x00;
const MULTIQ3_CLOCK_SETUP: u8 = 0x18;
const MULTIQ3_INPUT_SETUP: u8 = 0x41;
const MULTIQ3_QUAD_X4: u8 = 0x38;
const MULTIQ3_BP_RESET: u8 = 0x01;
const MULTIQ3_CNTR_RESET: u8 = 0x02;
const MULTIQ3_TRSFRPR_CTR: u8 = 0x08;
const MULTIQ3_TRSFRCNTR_OL: u8 = 0x10;
const MULTIQ3_EFLAG_RESET: u8 = 0x06;
const MULTIQ3_MAX_ENC_CHANS: u32 = 8;

unsafe fn multiq3_set_ctrl(dev: *mut comedi_device, bits: u16) {
    // SH and CLK must be kept high at all times.
    outw(MULTIQ3_CTRL_SH | MULTIQ3_CTRL_CLK | bits, (*dev).iobase + MULTIQ3_CTRL_REG as u32);
}

unsafe extern "C" fn multiq3_ai_status(dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, context: libc::c_ulong) -> libc::c_int {
    let status = inw((*dev).iobase + MULTIQ3_STATUS_REG as u32) as libc::c_ulong;
    if status & context != 0 { 0 } else { -libc::EBUSY }
}

unsafe extern "C" fn multiq3_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> libc::c_int {
    let chan = CR_CHAN((*insn).chanspec);
    multiq3_set_ctrl(dev, MULTIQ3_CTRL_EN | MULTIQ3_CTRL_AI_CHAN(chan as u16));
    let mut ret = comedi_timeout(dev, s, insn, Some(multiq3_ai_status), MULTIQ3_STATUS_EOC as libc::c_ulong);
    if ret != 0 { return ret; }
    for i in 0..(*insn).n as isize {
        outw(0, (*dev).iobase + MULTIQ3_AI_CONV_REG as u32);
        ret = comedi_timeout(dev, s, insn, Some(multiq3_ai_status), MULTIQ3_STATUS_EOC_I as libc::c_ulong);
        if ret != 0 { return ret; }
        let mut val = (inb((*dev).iobase + MULTIQ3_AI_REG as u32) as u32) << 8;
        val |= inb((*dev).iobase + MULTIQ3_AI_REG as u32) as u32;
        val &= (*s).maxdata;
        *data.offset(i) = comedi_offset_munge(s, val);
    }
    (*insn).n as libc::c_int
}

unsafe extern "C" fn multiq3_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> libc::c_int {
    let chan = CR_CHAN((*insn).chanspec) as isize;
    let mut val = *(*s).readback.offset(chan);
    for i in 0..(*insn).n as isize {
        val = *data.offset(i);
        multiq3_set_ctrl(dev, MULTIQ3_CTRL_LD | MULTIQ3_CTRL_AO_CHAN(chan as u16));
        outw(val as u16, (*dev).iobase + MULTIQ3_AO_REG as u32);
        multiq3_set_ctrl(dev, 0);
    }
    *(*s).readback.offset(chan) = val;
    (*insn).n as libc::c_int
}

unsafe extern "C" fn multiq3_di_insn_bits(dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> libc::c_int {
    *data.offset(1) = inw((*dev).iobase + MULTIQ3_DI_REG as u32) as u32;
    (*insn).n as libc::c_int
}

unsafe extern "C" fn multiq3_do_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> libc::c_int {
    if comedi_dio_update_state(s, data) != 0 { outw((*s).state as u16, (*dev).iobase + MULTIQ3_DO_REG as u32); }
    *data.offset(1) = (*s).state;
    (*insn).n as libc::c_int
}

unsafe extern "C" fn multiq3_encoder_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> libc::c_int {
    let chan = CR_CHAN((*insn).chanspec) as u16;
    for i in 0..(*insn).n as isize {
        multiq3_set_ctrl(dev, MULTIQ3_CTRL_EN | MULTIQ3_CTRL_E_CHAN(chan));
        outb(MULTIQ3_BP_RESET, (*dev).iobase + MULTIQ3_ENC_CTRL_REG as u32);
        outb(MULTIQ3_TRSFRCNTR_OL, (*dev).iobase + MULTIQ3_ENC_CTRL_REG as u32);
        let mut val = inb((*dev).iobase + MULTIQ3_ENC_DATA_REG as u32) as u32;
        val |= (inb((*dev).iobase + MULTIQ3_ENC_DATA_REG as u32) as u32) << 8;
        val |= (inb((*dev).iobase + MULTIQ3_ENC_DATA_REG as u32) as u32) << 16;
        *data.offset(i) = val.wrapping_add(((*s).maxdata + 1) >> 1) & (*s).maxdata;
    }
    (*insn).n as libc::c_int
}

unsafe fn multiq3_encoder_reset(dev: *mut comedi_device, chan: u32) {
    multiq3_set_ctrl(dev, MULTIQ3_CTRL_EN | MULTIQ3_CTRL_E_CHAN(chan as u16));
    outb(MULTIQ3_EFLAG_RESET, (*dev).iobase + MULTIQ3_ENC_CTRL_REG as u32);
    outb(MULTIQ3_BP_RESET, (*dev).iobase + MULTIQ3_ENC_CTRL_REG as u32);
    outb(MULTIQ3_CLOCK_DATA, (*dev).iobase + MULTIQ3_ENC_DATA_REG as u32);
    outb(MULTIQ3_CLOCK_SETUP, (*dev).iobase + MULTIQ3_ENC_CTRL_REG as u32);
    outb(MULTIQ3_INPUT_SETUP, (*dev).iobase + MULTIQ3_ENC_CTRL_REG as u32);
    outb(MULTIQ3_QUAD_X4, (*dev).iobase + MULTIQ3_ENC_CTRL_REG as u32);
    outb(MULTIQ3_CNTR_RESET, (*dev).iobase + MULTIQ3_ENC_CTRL_REG as u32);
}

unsafe extern "C" fn multiq3_encoder_insn_config(dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> libc::c_int {
    let chan = CR_CHAN((*insn).chanspec);
    match *data { INSN_CONFIG_RESET => multiq3_encoder_reset(dev, chan), _ => return -libc::EINVAL }
    (*insn).n as libc::c_int
}

// External kernel/comedi declarations and driver registration are supplied by the surrounding crate.
extern "C" {
    fn outw(value: u16, port: u32); fn inw(port: u32) -> u16;
    fn outb(value: u8, port: u32); fn inb(port: u32) -> u8;
    fn CR_CHAN(chanspec: u32) -> u32;
    fn comedi_timeout(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, f: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,libc::c_ulong)->libc::c_int>, context: libc::c_ulong) -> libc::c_int;
    fn comedi_offset_munge(s: *mut comedi_subdevice, val: u32) -> u32;
    fn comedi_dio_update_state(s: *mut comedi_subdevice, data: *mut u32) -> libc::c_int;
}
#[repr(C)] pub struct comedi_device { pub iobase: u32, pub subdevices: *mut comedi_subdevice }
#[repr(C)] pub struct comedi_subdevice { pub state: u32, pub maxdata: u32, pub readback: *mut u32, pub type_: u32, pub subdev_flags: u32, pub n_chan: u32, pub range_table: *const core::ffi::c_void, pub insn_read: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->libc::c_int>, pub insn_write: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->libc::c_int>, pub insn_bits: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->libc::c_int>, pub insn_config: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->libc::c_int> }
#[repr(C)] pub struct comedi_insn { pub chanspec: u32, pub n: u32 }
#[repr(C)] pub struct comedi_devconfig { pub options: [u32; 8] }
const INSN_CONFIG_RESET: u32 = 0;

unsafe extern "C" fn multiq3_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> libc::c_int {
    let mut ret = comedi_check_request_region(dev, (*it).options[0], 0x10, 0, 0x3ff, 16);
    if ret != 0 { return ret; }
    ret = comedi_alloc_subdevices(dev, 5);
    if ret != 0 { return ret; }
    let subs = (*dev).subdevices;
    let s = &mut *subs.add(0); s.type_ = COMEDI_SUBD_AI; s.subdev_flags = SDF_READABLE | SDF_GROUND; s.n_chan = 8; s.maxdata = 0x1fff; s.range_table = &range_bipolar5; s.insn_read = Some(multiq3_ai_insn_read);
    let s = &mut *subs.add(1); s.type_ = COMEDI_SUBD_AO; s.subdev_flags = SDF_WRITABLE; s.n_chan = 8; s.maxdata = 0x0fff; s.range_table = &range_bipolar5; s.insn_write = Some(multiq3_ao_insn_write); ret = comedi_alloc_subdev_readback(s); if ret != 0 { return ret; }
    let s = &mut *subs.add(2); s.type_ = COMEDI_SUBD_DI; s.subdev_flags = SDF_READABLE; s.n_chan = 16; s.maxdata = 1; s.range_table = &range_digital; s.insn_bits = Some(multiq3_di_insn_bits);
    let s = &mut *subs.add(3); s.type_ = COMEDI_SUBD_DO; s.subdev_flags = SDF_WRITABLE; s.n_chan = 16; s.maxdata = 1; s.range_table = &range_digital; s.insn_bits = Some(multiq3_do_insn_bits);
    let s = &mut *subs.add(4); s.type_ = COMEDI_SUBD_COUNTER; s.subdev_flags = SDF_READABLE | SDF_LSAMPL; s.n_chan = (*it).options[2] * 2; s.maxdata = 0x00ffffff; s.range_table = &range_unknown; s.insn_read = Some(multiq3_encoder_insn_read); s.insn_config = Some(multiq3_encoder_insn_config); if s.n_chan > MULTIQ3_MAX_ENC_CHANS { s.n_chan = MULTIQ3_MAX_ENC_CHANS; }
    for i in 0..s.n_chan { multiq3_encoder_reset(dev, i); } 0
}
extern "C" { fn comedi_check_request_region(*mut comedi_device,u32,u32,u32,u32,u32)->libc::c_int; fn comedi_alloc_subdevices(*mut comedi_device,u32)->libc::c_int; fn comedi_alloc_subdev_readback(*mut comedi_subdevice)->libc::c_int; }
extern "C" { static range_bipolar5: core::ffi::c_void; static range_digital: core::ffi::c_void; static range_unknown: core::ffi::c_void; }
const COMEDI_SUBD_AI:u32=1; const COMEDI_SUBD_AO:u32=2; const COMEDI_SUBD_DI:u32=3; const COMEDI_SUBD_DO:u32=4; const COMEDI_SUBD_COUNTER:u32=5; const SDF_READABLE:u32=1; const SDF_WRITABLE:u32=2; const SDF_GROUND:u32=4; const SDF_LSAMPL:u32=8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
