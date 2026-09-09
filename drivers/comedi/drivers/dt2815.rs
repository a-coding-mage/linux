// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/dt2815.c
 * Hardware driver for Data Translation DT2815
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1999 Anders Blomdell <anders.blomdell@control.lth.se>
 */
/*
 * Driver: dt2815
 * Description: Data Translation DT2815
 * Author: ds
 * Status: mostly complete, untested
 * Devices: [Data Translation] DT2815 (dt2815)
 *
 * I'm not sure anyone has ever tested this board.  If you have information
 * contrary, please update.
 *
 * Configuration options and the original driver comments are preserved in
 * the source-level translation.
 */

const DT2815_DATA: usize = 0;
const DT2815_STATUS: usize = 1;

#[repr(C)]
pub struct dt2815_private {
    pub range_type_list: [*const comedi_lrange; 8],
    pub ao_readback: [u32; 8],
}

#[repr(C)]
pub struct comedi_lrange;
#[repr(C)]
pub struct comedi_device {
    pub iobase: usize,
    pub private: *mut dt2815_private,
    pub subdevices: *mut comedi_subdevice,
    pub class_dev: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct comedi_subdevice {
    pub type_: u32,
    pub subdev_flags: u32,
    pub maxdata: u32,
    pub n_chan: u32,
    pub insn_write: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut u32) -> i32>,
    pub insn_read: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut u32) -> i32>,
    pub range_table_list: *mut [*const comedi_lrange; 8],
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

extern "C" {
    fn inb(port: usize) -> u8;
    fn outb(value: u32, port: usize);
    fn usleep_range(min: u32, max: u32);
    fn comedi_check_request_region(dev: *mut comedi_device, from: u32, len: u32, min: u32, max: u32, io_size: u32) -> i32;
    fn comedi_alloc_subdevices(dev: *mut comedi_device, num: u32) -> i32;
    fn comedi_alloc_devpriv(dev: *mut comedi_device, size: usize) -> *mut dt2815_private;
    fn comedi_timeout(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, callback: unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, usize) -> i32, context: usize) -> i32;
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    static range_4_20mA: comedi_lrange;
    static range_0_32mA: comedi_lrange;
    static range_bipolar5: comedi_lrange;
    static range_unipolar5: comedi_lrange;
}

const COMEDI_SUBD_AO: u32 = 2;
const SDF_WRITABLE: u32 = 0x0001;
const EBUSY: i32 = 16;
const ENOMEM: i32 = 12;
const ENODEV: i32 = 19;

#[inline]
unsafe fn cr_chan(chanspec: u32) -> usize { (chanspec & 0xff) as usize }

pub unsafe extern "C" fn dt2815_ao_status(dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, context: usize) -> i32 {
    let status = inb((*dev).iobase + DT2815_STATUS);
    if status as usize == context { 0 } else { -EBUSY }
}

pub unsafe extern "C" fn dt2815_ao_insn_read(dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let devpriv = (*dev).private;
    let chan = cr_chan((*insn).chanspec);
    for i in 0..(*insn).n as usize { *data.add(i) = (*devpriv).ao_readback[chan]; }
    (*insn).n as i32
}

pub unsafe extern "C" fn dt2815_ao_insn(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let devpriv = (*dev).private;
    let chan = cr_chan((*insn).chanspec);
    for i in 0..(*insn).n as usize {
        // FIXME: lo bit 0 chooses voltage output or current output
        let lo = ((*data.add(i) & 0x0f) << 4) | ((chan as u32) << 1) | 0x01;
        let hi = (*data.add(i) & 0xff0) >> 4;
        let ret = comedi_timeout(dev, s, insn, dt2815_ao_status, 0x00);
        if ret != 0 { return ret; }
        outb(lo, (*dev).iobase + DT2815_DATA);
        let ret = comedi_timeout(dev, s, insn, dt2815_ao_status, 0x10);
        if ret != 0 { return ret; }
        outb(hi, (*dev).iobase + DT2815_DATA);
        (*devpriv).ao_readback[chan] = *data.add(i);
    }
    (*insn).n as i32
}

pub unsafe extern "C" fn dt2815_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32 {
    let options = (*it).options;
    let ret = comedi_check_request_region(dev, *options.add(0), 0x2, 0x200, 0x3ff, 2);
    if ret != 0 { return ret; }
    let ret = comedi_alloc_subdevices(dev, 1);
    if ret != 0 { return ret; }
    let devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<dt2815_private>());
    if devpriv.is_null() { return -ENOMEM; }
    let s = &mut *(*dev).subdevices;
    s.type_ = COMEDI_SUBD_AO;
    s.subdev_flags = SDF_WRITABLE;
    s.maxdata = 0xfff;
    s.n_chan = 8;
    s.insn_write = Some(dt2815_ao_insn);
    s.insn_read = Some(dt2815_ao_insn_read);
    s.range_table_list = &mut (*devpriv).range_type_list;
    let current_range_type = if *options.add(3) != 0 { &range_4_20mA } else { &range_0_32mA };
    let voltage_range_type = if *options.add(2) != 0 { &range_bipolar5 } else { &range_unipolar5 };
    for i in 0..8 { (*devpriv).range_type_list[i] = if *options.add(5 + i) != 0 { current_range_type } else { voltage_range_type }; }
    if inb((*dev).iobase + DT2815_STATUS) == 0xff { dev_err((*dev).class_dev, b"No hardware detected at I/O base 0x%lx\0".as_ptr(), (*dev).iobase); return -ENODEV; }
    outb(0x00, (*dev).iobase + DT2815_STATUS);
    for i in 0..100 {
        usleep_range(1000, 3000);
        let status = inb((*dev).iobase + DT2815_STATUS);
        if status == 4 {
            let program = ((*options.add(4) & 0x3) << 3) | 0x7;
            outb(program, (*dev).iobase + DT2815_DATA);
            dev_dbg((*dev).class_dev, b"program: 0x%x (@t=%d)\n\0".as_ptr(), program, i);
            break;
        } else if status != 0x00 {
            dev_dbg((*dev).class_dev, b"unexpected status 0x%x (@t=%d)\n\0".as_ptr(), status, i);
            if status & 0x60 != 0 { outb(0x00, (*dev).iobase + DT2815_STATUS); }
        }
    }
    0
}

#[repr(C)]
pub struct comedi_driver {
    pub driver_name: *const u8,
    pub module: *mut core::ffi::c_void,
    pub attach: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_devconfig) -> i32>,
    pub detach: Option<unsafe extern "C" fn(*mut comedi_device) -> i32>,
}

extern "C" {
    fn comedi_legacy_detach(dev: *mut comedi_device) -> i32;
}

// Equivalent of module_comedi_driver(dt2815_driver).
#[no_mangle]
pub static mut dt2815_driver: comedi_driver = comedi_driver {
    driver_name: b"dt2815\0".as_ptr(),
    module: core::ptr::null_mut(),
    attach: Some(dt2815_attach),
    detach: Some(comedi_legacy_detach),
};

// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi low-level driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
