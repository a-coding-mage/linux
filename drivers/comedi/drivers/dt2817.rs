// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/dt2817.c
 * Hardware driver for Data Translation DT2817
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998 David A. Schleef <ds@schleef.org>
 */
/*
 * Driver: dt2817
 * Description: Data Translation DT2817
 * Author: ds
 * Status: complete
 * Devices: [Data Translation] DT2817 (dt2817)
 *
 * A very simple digital I/O card.  Four banks of 8 lines, each bank
 * is configurable for input or output.  One wonders why it takes a
 * 50 page manual to describe this thing.
 *
 * The driver (which, btw, is much less than 50 pages) has 1 subdevice
 * with 32 channels, configurable in groups of 8.
 *
 * Configuration options:
 * [0] - I/O port base base address
 */

// External declarations supplied by the surrounding Comedi/Linux bindings.
use core::ffi::c_int;

const DT2817_CR: usize = 0;
const DT2817_DATA: usize = 1;

#[repr(C)]
pub struct comedi_device {
    pub iobase: usize,
    pub subdevices: *mut comedi_subdevice,
}

#[repr(C)]
pub struct comedi_subdevice {
    pub io_bits: u32,
    pub n_chan: u32,
    pub type_: u32,
    pub subdev_flags: u32,
    pub range_table: *mut range_table,
    pub maxdata: u32,
    pub insn_bits: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut u32) -> c_int>,
    pub insn_config: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut u32) -> c_int>,
    pub state: u32,
}

#[repr(C)]
pub struct comedi_insn {
    pub chanspec: u32,
    pub n: u32,
}

#[repr(C)]
pub struct comedi_devconfig {
    pub options: *mut u32,
}

#[repr(C)]
pub struct range_table;

#[repr(C)]
pub struct comedi_driver {
    pub driver_name: *const u8,
    pub module: *mut core::ffi::c_void,
    pub attach: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_devconfig) -> c_int>,
    pub detach: Option<unsafe extern "C" fn(*mut comedi_device) -> c_int>,
}

extern "C" {
    static mut range_digital: range_table;
    fn CR_CHAN(chanspec: u32) -> u32;
    fn comedi_dio_insn_config(dev: *mut comedi_device, s: *mut comedi_subdevice,
                               insn: *mut comedi_insn, data: *mut u32, mask: u32) -> c_int;
    fn comedi_dio_update_state(s: *mut comedi_subdevice, data: *mut u32) -> u32;
    fn comedi_check_request_region(dev: *mut comedi_device, from: u32, len: u32,
                                   min: u32, max: u32, ioport: u32) -> c_int;
    fn comedi_alloc_subdevices(dev: *mut comedi_device, num_subdevices: u32) -> c_int;
    fn comedi_legacy_detach(dev: *mut comedi_device) -> c_int;
    fn outb(value: u8, port: usize);
    fn inb(port: usize) -> u32;
}

const COMEDI_SUBD_DIO: u32 = 0;
const SDF_READABLE: u32 = 1 << 0;
const SDF_WRITABLE: u32 = 1 << 1;

unsafe extern "C" fn dt2817_dio_insn_config(
    dev: *mut comedi_device, s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> c_int {
    let chan = CR_CHAN((*insn).chanspec);
    let mut oe: u32 = 0;
    let mask: u32;

    if chan < 8 {
        mask = 0x000000ff;
    } else if chan < 16 {
        mask = 0x0000ff00;
    } else if chan < 24 {
        mask = 0x00ff0000;
    } else {
        mask = 0xff000000;
    }

    let ret = comedi_dio_insn_config(dev, s, insn, data, mask);
    if ret != 0 {
        return ret;
    }

    if (*s).io_bits & 0x000000ff != 0 { oe |= 0x1; }
    if (*s).io_bits & 0x0000ff00 != 0 { oe |= 0x2; }
    if (*s).io_bits & 0x00ff0000 != 0 { oe |= 0x4; }
    if (*s).io_bits & 0xff000000 != 0 { oe |= 0x8; }

    outb(oe as u8, (*dev).iobase + DT2817_CR);
    (*insn).n as c_int
}

unsafe extern "C" fn dt2817_dio_insn_bits(
    dev: *mut comedi_device, s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> c_int {
    let iobase = (*dev).iobase + DT2817_DATA;
    let mask = comedi_dio_update_state(s, data);
    if mask != 0 {
        if mask & 0x000000ff != 0 { outb(((*s).state & 0xff) as u8, iobase); }
        if mask & 0x0000ff00 != 0 { outb(((((*s).state >> 8) & 0xff)) as u8, iobase + 1); }
        if mask & 0x00ff0000 != 0 { outb(((((*s).state >> 16) & 0xff)) as u8, iobase + 2); }
        if mask & 0xff000000 != 0 { outb(((((*s).state >> 24) & 0xff)) as u8, iobase + 3); }
    }

    let mut val = inb(iobase) as u32;
    val |= inb(iobase + 1) << 8;
    val |= inb(iobase + 2) << 16;
    val |= inb(iobase + 3) << 24;
    *data.add(1) = val;
    (*insn).n as c_int
}

unsafe extern "C" fn dt2817_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int {
    let ret = comedi_check_request_region(dev, *(*it).options.add(0), 0x5, 0x200, 0x3ff, 8);
    if ret != 0 { return ret; }
    let ret = comedi_alloc_subdevices(dev, 1);
    if ret != 0 { return ret; }

    let s = &mut *(*dev).subdevices;
    s.n_chan = 32;
    s.type_ = COMEDI_SUBD_DIO;
    s.subdev_flags = SDF_READABLE | SDF_WRITABLE;
    s.range_table = &raw mut range_digital;
    s.maxdata = 1;
    s.insn_bits = Some(dt2817_dio_insn_bits);
    s.insn_config = Some(dt2817_dio_insn_config);
    s.state = 0;
    outb(0, (*dev).iobase + DT2817_CR);
    0
}

static mut dt2817_driver: comedi_driver = comedi_driver {
    driver_name: b"dt2817\0".as_ptr(),
    module: core::ptr::null_mut(),
    attach: Some(dt2817_attach),
    detach: Some(comedi_legacy_detach),
};

// module_comedi_driver(dt2817_driver);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi low-level driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
