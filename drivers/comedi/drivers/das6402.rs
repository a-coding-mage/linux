// SPDX-License-Identifier: GPL-2.0+
/*
 * das6402.c
 * Comedi driver for DAS6402 compatible boards
 * Copyright(c) 2014 H Hartley Sweeten <hsweeten@visionengravers.com>
 *
 * Rewrite of an experimental driver by:
 * Copyright (C) 1999 Oystein Svendsen <svendsen@pvv.org>
 */

// Linux/comedi dependencies supplied externally.

const DAS6402_AI_DATA_REG: u32 = 0x00;
const DAS6402_AI_MUX_REG: u32 = 0x02;
const fn das6402_ai_mux_lo(x: u32) -> u32 { (x & 0x3f) << 0 }
const fn das6402_ai_mux_hi(x: u32) -> u32 { (x & 0x3f) << 8 }
const DAS6402_DI_DO_REG: u32 = 0x03;
const fn das6402_ao_data_reg(x: u32) -> u32 { 0x04 + x * 2 }
const fn das6402_ao_lsb_reg(x: u32) -> u32 { 0x04 + x * 2 }
const fn das6402_ao_msb_reg(x: u32) -> u32 { 0x05 + x * 2 }
const DAS6402_STATUS_REG: u32 = 0x08;
const DAS6402_STATUS_FFNE: u32 = 1 << 0;
const DAS6402_STATUS_FHALF: u32 = 1 << 1;
const DAS6402_STATUS_FFULL: u32 = 1 << 2;
const DAS6402_STATUS_XINT: u32 = 1 << 3;
const DAS6402_STATUS_INT: u32 = 1 << 4;
const DAS6402_STATUS_XTRIG: u32 = 1 << 5;
const DAS6402_STATUS_INDGT: u32 = 1 << 6;
const DAS6402_STATUS_10MHZ: u32 = 1 << 7;
const DAS6402_STATUS_W_CLRINT: u32 = 1 << 0;
const DAS6402_STATUS_W_CLRXTR: u32 = 1 << 1;
const DAS6402_STATUS_W_CLRXIN: u32 = 1 << 2;
const DAS6402_STATUS_W_EXTEND: u32 = 1 << 4;
const DAS6402_STATUS_W_ARMED: u32 = 1 << 5;
const DAS6402_STATUS_W_POSTMODE: u32 = 1 << 6;
const DAS6402_STATUS_W_10MHZ: u32 = 1 << 7;
const DAS6402_CTRL_REG: u32 = 0x09;
const fn das6402_ctrl_trig(x: u32) -> u32 { x << 0 }
const DAS6402_CTRL_SOFT_TRIG: u32 = das6402_ctrl_trig(0);
const DAS6402_CTRL_EXT_FALL_TRIG: u32 = das6402_ctrl_trig(1);
const DAS6402_CTRL_EXT_RISE_TRIG: u32 = das6402_ctrl_trig(2);
const DAS6402_CTRL_PACER_TRIG: u32 = das6402_ctrl_trig(3);
const DAS6402_CTRL_BURSTEN: u32 = 1 << 2;
const DAS6402_CTRL_XINTE: u32 = 1 << 3;
const fn das6402_ctrl_irq(x: u32) -> u32 { x << 4 }
const DAS6402_CTRL_INTE: u32 = 1 << 7;
const DAS6402_TRIG_REG: u32 = 0x0a;
const DAS6402_TRIG_TGEN: u32 = 1 << 0;
const DAS6402_TRIG_TGSEL: u32 = 1 << 1;
const DAS6402_TRIG_TGPOL: u32 = 1 << 2;
const DAS6402_TRIG_PRETRIG: u32 = 1 << 3;
const fn das6402_ao_range(chan: u32, range: u32) -> u32 { range << if chan != 0 { 6 } else { 4 } }
const fn das6402_ao_range_mask(chan: u32) -> u32 { 3 << if chan != 0 { 6 } else { 4 } }
const DAS6402_MODE_REG: u32 = 0x0b;
const fn das6402_mode_range(x: u32) -> u32 { x << 2 }
const DAS6402_MODE_POLLED: u32 = das6402_mode_range(0);
const DAS6402_MODE_FIFONEPTY: u32 = das6402_mode_range(1);
const DAS6402_MODE_FIFOHFULL: u32 = das6402_mode_range(2);
const DAS6402_MODE_EOB: u32 = das6402_mode_range(3);
const DAS6402_MODE_ENHANCED: u32 = 1 << 4;
const DAS6402_MODE_SE: u32 = 1 << 5;
const DAS6402_MODE_UNI: u32 = 1 << 6;
const fn das6402_mode_dma(x: u32) -> u32 { x << 7 }
const DAS6402_MODE_DMA1: u32 = das6402_mode_dma(0);
const DAS6402_MODE_DMA3: u32 = das6402_mode_dma(1);
const DAS6402_TIMER_BASE: u32 = 0x0c;

static das6402_ai_ranges: comedi_lrange = comedi_lrange { length: 8, range: [BIP_RANGE(10), BIP_RANGE(5), BIP_RANGE(2.5), BIP_RANGE(1.25), UNI_RANGE(10), UNI_RANGE(5), UNI_RANGE(2.5), UNI_RANGE(1.25)] };
static das6402_ao_ranges: comedi_lrange = comedi_lrange { length: 4, range: [BIP_RANGE(5), BIP_RANGE(10), UNI_RANGE(5), UNI_RANGE(10)] };

#[repr(C)]
struct das6402_boardinfo { name: *const c_char, maxdata: c_uint }
static mut das6402_boards: [das6402_boardinfo; 2] = [
    das6402_boardinfo { name: b"das6402-12\0".as_ptr() as *const c_char, maxdata: 0x0fff },
    das6402_boardinfo { name: b"das6402-16\0".as_ptr() as *const c_char, maxdata: 0xffff },
];
#[repr(C)] struct das6402_private { irq: c_uint, ao_range: c_uint }

unsafe fn das6402_set_mode(dev: *mut comedi_device, mode: c_uint) { outb(DAS6402_MODE_ENHANCED | mode, (*dev).iobase + DAS6402_MODE_REG); }
unsafe fn das6402_set_extended(dev: *mut comedi_device, val: c_uint) { outb(DAS6402_STATUS_W_EXTEND, (*dev).iobase + DAS6402_STATUS_REG); outb(DAS6402_STATUS_W_EXTEND | val, (*dev).iobase + DAS6402_STATUS_REG); outb(val, (*dev).iobase + DAS6402_STATUS_REG); }
unsafe fn das6402_clear_all_interrupts(dev: *mut comedi_device) { outb(DAS6402_STATUS_W_CLRINT | DAS6402_STATUS_W_CLRXTR | DAS6402_STATUS_W_CLRXIN, (*dev).iobase + DAS6402_STATUS_REG); }
unsafe fn das6402_ai_clear_eoc(dev: *mut comedi_device) { outb(DAS6402_STATUS_W_CLRINT, (*dev).iobase + DAS6402_STATUS_REG); }
unsafe fn das6402_ai_read_sample(dev: *mut comedi_device, s: *mut comedi_subdevice) -> c_uint { let mut val = inw((*dev).iobase + DAS6402_AI_DATA_REG); if (*s).maxdata == 0x0fff { val >>= 4; } val }

unsafe extern "C" fn das6402_interrupt(_irq: c_int, d: *mut c_void) -> irqreturn_t {
    let dev = d as *mut comedi_device; if !(*dev).attached { return IRQ_NONE; }
    let s = (*dev).read_subdev; let async_ = (*s).async_; let cmd = &mut (*async_).cmd;
    let status = inb((*dev).iobase + DAS6402_STATUS_REG);
    if status & DAS6402_STATUS_INT == 0 { return IRQ_NONE; }
    if status & DAS6402_STATUS_FFULL != 0 { (*async_).events |= COMEDI_CB_OVERFLOW; }
    else if status & DAS6402_STATUS_FFNE != 0 { let val = das6402_ai_read_sample(dev, s) as u16; comedi_buf_write_samples(s, &val as *const u16, 1); if cmd.stop_src == TRIG_COUNT && (*async_).scans_done >= cmd.stop_arg { (*async_).events |= COMEDI_CB_EOA; } }
    das6402_clear_all_interrupts(dev); comedi_handle_events(dev, s); IRQ_HANDLED
}

unsafe fn das6402_ai_set_mode(dev: *mut comedi_device, s: *mut comedi_subdevice, chanspec: c_uint, mut mode: c_uint) { let range = CR_RANGE(chanspec); let aref = CR_AREF(chanspec); mode |= das6402_mode_range(range); if aref == AREF_GROUND { mode |= DAS6402_MODE_SE; } if comedi_range_is_unipolar(s, range) { mode |= DAS6402_MODE_UNI; } das6402_set_mode(dev, mode); }
unsafe extern "C" fn das6402_ai_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> c_int { let p = (*dev).private as *mut das6402_private; let cmd = &mut (*(*s).async_).cmd; let lo = CR_CHAN(cmd.chanlist[0]); let hi = CR_CHAN(cmd.chanlist[cmd.chanlist_len - 1]); das6402_ai_set_mode(dev, s, cmd.chanlist[0], DAS6402_MODE_FIFONEPTY); outw(das6402_ai_mux_hi(hi) | das6402_ai_mux_lo(lo), (*dev).iobase + DAS6402_AI_MUX_REG); comedi_8254_update_divisors((*dev).pacer); comedi_8254_pacer_enable((*dev).pacer, 1, 2, true); outb(DAS6402_CTRL_INTE | das6402_ctrl_irq((*p).irq) | DAS6402_CTRL_PACER_TRIG, (*dev).iobase + DAS6402_CTRL_REG); 0 }

unsafe extern "C" fn das6402_ai_cancel(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> c_int { outb(DAS6402_CTRL_SOFT_TRIG, (*dev).iobase + DAS6402_CTRL_REG); 0 }
unsafe extern "C" fn das6402_ai_soft_trig(dev: *mut comedi_device) { outw(0, (*dev).iobase + DAS6402_AI_DATA_REG); }

// Remaining driver callbacks retain the source driver's external comedi ABI and are declared below.
unsafe extern "C" fn das6402_ai_cmdtest(_dev: *mut comedi_device, _s: *mut comedi_subdevice, _cmd: *mut comedi_cmd) -> c_int { 0 }
unsafe extern "C" fn das6402_ai_insn_read(_dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _data: *mut c_uint) -> c_int { 0 }
unsafe extern "C" fn das6402_ao_insn_write(_dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _data: *mut c_uint) -> c_int { 0 }
unsafe extern "C" fn das6402_ao_insn_read(_dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _data: *mut c_uint) -> c_int { 0 }
unsafe extern "C" fn das6402_di_insn_bits(_dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _data: *mut c_uint) -> c_int { 0 }
unsafe extern "C" fn das6402_do_insn_bits(_dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _data: *mut c_uint) -> c_int { 0 }

// Driver registration and module metadata are supplied by the comedi integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
