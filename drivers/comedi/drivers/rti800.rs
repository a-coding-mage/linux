// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/rti800.c
 * Hardware driver for Analog Devices RTI-800/815 board
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998 David A. Schleef <ds@schleef.org>
 */

/* Original C implementation; kernel/comedi dependencies are supplied externally. */

const RTI800_CSR: u8 = 0x00;
const RTI800_CSR_BUSY: u8 = 1 << 7;
const RTI800_CSR_DONE: u8 = 1 << 6;
const RTI800_CSR_OVERRUN: u8 = 1 << 5;
const RTI800_CSR_TCR: u8 = 1 << 4;
const RTI800_CSR_DMA_ENAB: u8 = 1 << 3;
const RTI800_CSR_INTR_TC: u8 = 1 << 2;
const RTI800_CSR_INTR_EC: u8 = 1 << 1;
const RTI800_CSR_INTR_OVRN: u8 = 1 << 0;
const RTI800_MUXGAIN: u8 = 0x01;
const RTI800_CONVERT: u8 = 0x02;
const RTI800_ADCLO: u8 = 0x03;
const RTI800_ADCHI: u8 = 0x04;
const RTI800_DAC0LO: u8 = 0x05;
const RTI800_DAC0HI: u8 = 0x06;
const RTI800_DAC1LO: u8 = 0x07;
const RTI800_DAC1HI: u8 = 0x08;
const RTI800_CLRFLAGS: u8 = 0x09;
const RTI800_DI: u8 = 0x0a;
const RTI800_DO: u8 = 0x0b;
const RTI800_9513A_DATA: u8 = 0x0c;
const RTI800_9513A_CNTRL: u8 = 0x0d;
const RTI800_9513A_STATUS: u8 = 0x0d;

static range_rti800_ai_10_bipolar: comedi_lrange = comedi_lrange { length: 4, range: [BIP_RANGE(10.0), BIP_RANGE(1.0), BIP_RANGE(0.1), BIP_RANGE(0.02)] };
static range_rti800_ai_5_bipolar: comedi_lrange = comedi_lrange { length: 4, range: [BIP_RANGE(5.0), BIP_RANGE(0.5), BIP_RANGE(0.05), BIP_RANGE(0.01)] };
static range_rti800_ai_unipolar: comedi_lrange = comedi_lrange { length: 4, range: [UNI_RANGE(10.0), UNI_RANGE(1.0), UNI_RANGE(0.1), UNI_RANGE(0.02)] };

static rti800_ai_ranges: [&'static comedi_lrange; 3] = [&range_rti800_ai_10_bipolar, &range_rti800_ai_5_bipolar, &range_rti800_ai_unipolar];
static rti800_ao_ranges: [&'static comedi_lrange; 2] = [&range_bipolar10, &range_unipolar10];

#[repr(C)]
struct rti800_board { name: *const c_char, has_ao: c_int }

static rti800_boardtypes: [rti800_board; 2] = [
    rti800_board { name: c"rti800".as_ptr(), has_ao: 0 },
    rti800_board { name: c"rti815".as_ptr(), has_ao: 1 },
];

#[repr(C)]
struct rti800_private {
    adc_2comp: bool,
    dac_2comp: [bool; 2],
    ao_range_type_list: [*const comedi_lrange; 2],
    muxgain_bits: u8,
}

unsafe fn rti800_ai_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _context: c_ulong) -> c_int {
    let status = inb((*dev).iobase + RTI800_CSR as c_ulong);
    if status & RTI800_CSR_OVERRUN != 0 { outb(0, (*dev).iobase + RTI800_CLRFLAGS as c_ulong); return -EOVERFLOW; }
    if status & RTI800_CSR_DONE != 0 { return 0; }
    -EBUSY
}

unsafe fn rti800_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut c_uint) -> c_int {
    let devpriv = (*dev).private as *mut rti800_private;
    let chan = CR_CHAN((*insn).chanspec);
    let gain = CR_RANGE((*insn).chanspec);
    inb((*dev).iobase + RTI800_ADCHI as c_ulong);
    outb(0, (*dev).iobase + RTI800_CLRFLAGS as c_ulong);
    let muxgain_bits = (chan | (gain << 5)) as u8;
    if muxgain_bits != (*devpriv).muxgain_bits {
        (*devpriv).muxgain_bits = muxgain_bits;
        outb(muxgain_bits, (*dev).iobase + RTI800_MUXGAIN as c_ulong);
        if (*insn).n > 0 { let delay = if gain == 0 { 10 } else if gain == 1 { 20 } else if gain == 2 { 40 } else { 80 }; udelay(delay); }
    }
    for i in 0..(*insn).n {
        outb(0, (*dev).iobase + RTI800_CONVERT as c_ulong);
        let ret = comedi_timeout(dev, s, insn, Some(rti800_ai_eoc), 0);
        if ret != 0 { return ret; }
        let mut val = inb((*dev).iobase + RTI800_ADCLO as c_ulong) as c_uint;
        val |= ((inb((*dev).iobase + RTI800_ADCHI as c_ulong) & 0xf) as c_uint) << 8;
        if (*devpriv).adc_2comp { val = comedi_offset_munge(s, val); }
        *data.add(i as usize) = val;
    }
    (*insn).n as c_int
}

unsafe fn rti800_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut c_uint) -> c_int {
    let devpriv = (*dev).private as *mut rti800_private;
    let chan = CR_CHAN((*insn).chanspec) as usize;
    let reg_lo = if chan != 0 { RTI800_DAC1LO } else { RTI800_DAC0LO };
    let reg_hi = if chan != 0 { RTI800_DAC1HI } else { RTI800_DAC0HI };
    for i in 0..(*insn).n { let mut val = *data.add(i as usize); (*s).readback.add(chan).write(val); if (*devpriv).dac_2comp[chan] { val = comedi_offset_munge(s, val); } outb((val & 0xff) as u8, (*dev).iobase + reg_lo as c_ulong); outb(((val >> 8) & 0xff) as u8, (*dev).iobase + reg_hi as c_ulong); }
    (*insn).n as c_int
}

unsafe fn rti800_di_insn_bits(dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut c_uint) -> c_int { *data.add(1) = inb((*dev).iobase + RTI800_DI as c_ulong) as c_uint; (*insn).n as c_int }

unsafe fn rti800_do_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut c_uint) -> c_int {
    if comedi_dio_update_state(s, data) != 0 { outb(((*s).state ^ 0xff) as u8, (*dev).iobase + RTI800_DO as c_ulong); }
    *data.add(1) = (*s).state; (*insn).n as c_int
}

unsafe fn rti800_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int {
    let board = (*dev).board_ptr as *const rti800_board;
    let ret = comedi_check_request_region(dev, (*it).options[0], 0x10, 0, 0x3ff, 16); if ret != 0 { return ret; }
    outb(0, (*dev).iobase + RTI800_CSR as c_ulong); inb((*dev).iobase + RTI800_ADCHI as c_ulong); outb(0, (*dev).iobase + RTI800_CLRFLAGS as c_ulong);
    let devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<rti800_private>()) as *mut rti800_private; if devpriv.is_null() { return -ENOMEM; }
    (*devpriv).adc_2comp = (*it).options[4] == 0; (*devpriv).dac_2comp = [(*it).options[6] == 0, (*it).options[8] == 0]; (*devpriv).muxgain_bits = 0xff;
    let ret = comedi_alloc_subdevices(dev, 4); if ret != 0 { return ret; }
    let s = (*dev).subdevices;
    (*s).type_ = COMEDI_SUBD_AI; (*s).subdev_flags = SDF_READABLE | SDF_GROUND; (*s).n_chan = if (*it).options[2] != 0 { 16 } else { 8 }; (*s).insn_read = Some(rti800_ai_insn_read); (*s).maxdata = 0x0fff; (*s).range_table = if (*it).options[3] < rti800_ai_ranges.len() { rti800_ai_ranges[(*it).options[3] as usize] } else { &range_unknown };
    let s = s.add(1);
    if (*board).has_ao != 0 { (*s).type_ = COMEDI_SUBD_AO; (*s).subdev_flags = SDF_WRITABLE; (*s).n_chan = 2; (*s).maxdata = 0x0fff; (*s).range_table_list = (*devpriv).ao_range_type_list.as_mut_ptr(); (*devpriv).ao_range_type_list[0] = if (*it).options[5] < rti800_ao_ranges.len() { rti800_ao_ranges[(*it).options[5] as usize] } else { &range_unknown }; (*devpriv).ao_range_type_list[1] = if (*it).options[7] < rti800_ao_ranges.len() { rti800_ao_ranges[(*it).options[7] as usize] } else { &range_unknown }; (*s).insn_write = Some(rti800_ao_insn_write); let ret = comedi_alloc_subdev_readback(s); if ret != 0 { return ret; } } else { (*s).type_ = COMEDI_SUBD_UNUSED; }
    let s = s.add(1); (*s).type_ = COMEDI_SUBD_DI; (*s).subdev_flags = SDF_READABLE; (*s).n_chan = 8; (*s).insn_bits = Some(rti800_di_insn_bits); (*s).maxdata = 1; (*s).range_table = &range_digital;
    let s = s.add(1); (*s).type_ = COMEDI_SUBD_DO; (*s).subdev_flags = SDF_WRITABLE; (*s).n_chan = 8; (*s).insn_bits = Some(rti800_do_insn_bits); (*s).maxdata = 1; (*s).range_table = &range_digital;
    0
}

// There is also an Am9513 timer on these boards. This subdevice is not currently supported.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
