// SPDX-License-Identifier: GPL-2.0+
/*
 * adq12b.c
 * Driver for MicroAxial ADQ12-B data acquisition and control card
 * written by jeremy theler <thelerg@ib.cnea.gov.ar>
 *	instituto balseiro
 *	commission nacional de energia atomica
 *	universidad nacional de cuyo
 *	argentina
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

/* Driver: adq12b; Description: Driver for MicroAxial ADQ12-B data acquisition and control card */
/* Devices: [MicroAxial] ADQ12-B (adq12b); Author: jeremy theler <thelerg@ib.cnea.gov.ar> */
/* Updated: Thu, 21 Feb 2008 02:56:27 -0300; Status: works */
/* Configuration options and hardware jumper documentation are as in the original source. */
/* Analog input is subdevice 0; digital input and output are subdevice 1; PACER unsupported. */

// Dependencies supplied by the surrounding Comedi/Linux bindings.

const ADQ12B_CTREG: u32 = 0x00;
const ADQ12B_CTREG_MSKP: u32 = 1 << 7;
const ADQ12B_CTREG_GTP: u32 = 1 << 6;
const ADQ12B_STINR: u32 = 0x00;
const ADQ12B_STINR_OUT2: u32 = 1 << 7;
const ADQ12B_STINR_OUTP: u32 = 1 << 6;
const ADQ12B_STINR_EOC: u32 = 1 << 5;
const ADQ12B_STINR_IN_MASK: u32 = 0x1f << 0;
const ADQ12B_OUTBR: u32 = 0x04;
const ADQ12B_ADLOW: u32 = 0x08;
const ADQ12B_ADHIG: u32 = 0x09;
const ADQ12B_TIMER_BASE: u32 = 0x0c;

const fn adq12b_ctreg_range(x: u32) -> u32 { x << 4 }
const fn adq12b_ctreg_chan(x: u32) -> u32 { x }

static range_adq12b_ai_bipolar: comedi_lrange = comedi_lrange {
    length: 4,
    range: [BIP_RANGE(5), BIP_RANGE(2), BIP_RANGE(1), BIP_RANGE(0.5)],
};

static range_adq12b_ai_unipolar: comedi_lrange = comedi_lrange {
    length: 4,
    range: [UNI_RANGE(5), UNI_RANGE(2), UNI_RANGE(1), UNI_RANGE(0.5)],
};

#[repr(C)]
struct adq12b_private {
    last_ctreg: u32,
}

unsafe fn adq12b_ai_eoc(
    dev: *mut comedi_device, _s: *mut comedi_subdevice,
    _insn: *mut comedi_insn, _context: libc::c_ulong,
) -> libc::c_int {
    let status = inb((*dev).iobase + ADQ12B_STINR);
    if status & ADQ12B_STINR_EOC != 0 { 0 } else { -EBUSY }
}

unsafe fn adq12b_ai_insn_read(
    dev: *mut comedi_device, s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> libc::c_int {
    let devpriv = (*dev).private as *mut adq12b_private;
    let chan = CR_CHAN((*insn).chanspec);
    let range = CR_RANGE((*insn).chanspec);
    let val = adq12b_ctreg_range(range) | adq12b_ctreg_chan(chan);
    if val != (*devpriv).last_ctreg {
        outb(val, (*dev).iobase + ADQ12B_CTREG);
        (*devpriv).last_ctreg = val;
        usleep_range(50, 100);
    }
    let _ = inb((*dev).iobase + ADQ12B_ADLOW);
    for i in 0..(*insn).n {
        let ret = comedi_timeout(dev, s, insn, Some(adq12b_ai_eoc), 0);
        if ret != 0 { return ret; }
        let value = ((inb((*dev).iobase + ADQ12B_ADHIG) as u32) << 8)
            | inb((*dev).iobase + ADQ12B_ADLOW) as u32;
        *data.add(i as usize) = value;
    }
    (*insn).n as libc::c_int
}

unsafe fn adq12b_di_insn_bits(
    dev: *mut comedi_device, _s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> libc::c_int {
    *data.add(1) = inb((*dev).iobase + ADQ12B_STINR) & ADQ12B_STINR_IN_MASK;
    (*insn).n as libc::c_int
}

unsafe fn adq12b_do_insn_bits(
    dev: *mut comedi_device, s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> libc::c_int {
    let mask = comedi_dio_update_state(s, data);
    if mask != 0 {
        for chan in 0..8u32 {
            if (mask >> chan) & 1 != 0 {
                let val = ((*s).state >> chan) & 1;
                outb((val << 3) | chan, (*dev).iobase + ADQ12B_OUTBR);
            }
        }
    }
    *data.add(1) = (*s).state;
    (*insn).n as libc::c_int
}

unsafe fn adq12b_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> libc::c_int {
    let ret = comedi_check_request_region(dev, (*it).options[0], 0x10, 0x300, 0x3af, 0x20);
    if ret != 0 { return ret; }
    let devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<adq12b_private>()) as *mut adq12b_private;
    if devpriv.is_null() { return -ENOMEM; }
    (*devpriv).last_ctreg = u32::MAX;
    let ret = comedi_alloc_subdevices(dev, 3);
    if ret != 0 { return ret; }

    let s = (*dev).subdevices.add(0);
    (*s).type_ = COMEDI_SUBD_AI;
    if (*it).options[2] != 0 { (*s).subdev_flags = SDF_READABLE | SDF_DIFF; (*s).n_chan = 8; }
    else { (*s).subdev_flags = SDF_READABLE | SDF_GROUND; (*s).n_chan = 16; }
    (*s).maxdata = 0xfff;
    (*s).range_table = if (*it).options[1] != 0 { &range_adq12b_ai_unipolar } else { &range_adq12b_ai_bipolar };
    (*s).insn_read = Some(adq12b_ai_insn_read);

    let s = (*dev).subdevices.add(1);
    (*s).type_ = COMEDI_SUBD_DI; (*s).subdev_flags = SDF_READABLE; (*s).n_chan = 5; (*s).maxdata = 1;
    (*s).range_table = &range_digital; (*s).insn_bits = Some(adq12b_di_insn_bits);

    let s = (*dev).subdevices.add(2);
    (*s).type_ = COMEDI_SUBD_DO; (*s).subdev_flags = SDF_WRITABLE; (*s).n_chan = 8; (*s).maxdata = 1;
    (*s).range_table = &range_digital; (*s).insn_bits = Some(adq12b_do_insn_bits);
    0
}

static mut adq12b_driver: comedi_driver = comedi_driver {
    driver_name: "adq12b", module: THIS_MODULE, attach: Some(adq12b_attach), detach: Some(comedi_legacy_detach),
};
module_comedi_driver!(adq12b_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
