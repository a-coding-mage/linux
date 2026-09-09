// SPDX-License-Identifier: GPL-2.0+
/*
 * Driver for Amplicon PC263 relay board.
 *
 * Copyright (C) 2002 MEV Ltd. <https://www.mev.co.uk/>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: amplc_pc263
 * Description: Amplicon PC263
 * Author: Ian Abbott <abbotti@mev.co.uk>
 * Devices: [Amplicon] PC263 (pc263)
 * Updated: Fri, 12 Apr 2013 15:19:36 +0100
 * Status: works
 *
 * Configuration options:
 *   [0] - I/O port base address
 *
 * The board appears as one subdevice, with 16 digital outputs, each
 * connected to a reed-relay. Relay contacts are closed when output is 1.
 * The state of the outputs can be read.
 */

// Linux/Comedi dependencies are supplied externally.

const PC263_DO_0_7_REG: usize = 0x00;
const PC263_DO_8_15_REG: usize = 0x01;

#[repr(C)]
struct pc263_board {
    name: *const ::core::ffi::c_char,
}

static PC263_BOARDS: [pc263_board; 1] = [pc263_board {
    name: b"pc263\0".as_ptr() as *const ::core::ffi::c_char,
}];

unsafe extern "C" {
    fn comedi_dio_update_state(
        s: *mut comedi_subdevice,
        data: *mut u32,
    ) -> i32;
    fn outb(value: u8, port: usize);
    fn inb(port: usize) -> u8;
    fn comedi_check_request_region(
        dev: *mut comedi_device,
        base: u32,
        len: u32,
        min: u32,
        max: u32,
        flags: u32,
    ) -> i32;
    fn comedi_alloc_subdevices(dev: *mut comedi_device, n: u32) -> i32;
    fn comedi_legacy_detach(dev: *mut comedi_device) -> i32;
}

#[repr(C)]
struct comedi_device {
    iobase: usize,
    subdevices: *mut comedi_subdevice,
}

#[repr(C)]
struct comedi_subdevice {
    type_: u32,
    subdev_flags: u32,
    n_chan: u32,
    maxdata: u32,
    range_table: *const core::ffi::c_void,
    insn_bits: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut u32) -> i32>,
    state: u32,
}

#[repr(C)]
struct comedi_insn {
    n: u32,
}

#[repr(C)]
struct comedi_devconfig {
    options: *const u32,
}

extern "C" {
    static range_digital: core::ffi::c_void;
}

const COMEDI_SUBD_DO: u32 = 2;
const SDF_WRITABLE: u32 = 0x02;

unsafe extern "C" fn pc263_do_insn_bits(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    if comedi_dio_update_state(s, data) != 0 {
        outb((*s).state as u8, (*dev).iobase + PC263_DO_0_7_REG);
        outb(((*s).state >> 8) as u8, (*dev).iobase + PC263_DO_8_15_REG);
    }

    *data.add(1) = (*s).state;

    (*insn).n as i32
}

unsafe extern "C" fn pc263_attach(
    dev: *mut comedi_device,
    it: *mut comedi_devconfig,
) -> i32 {
    let ret = comedi_check_request_region(dev, *(*it).options.add(0), 0x2, 0, 0x7ff, 2);
    if ret != 0 {
        return ret;
    }

    let ret = comedi_alloc_subdevices(dev, 1);
    if ret != 0 {
        return ret;
    }

    // Digital Output subdevice
    let s = &mut *(*dev).subdevices;
    s.type_ = COMEDI_SUBD_DO;
    s.subdev_flags = SDF_WRITABLE;
    s.n_chan = 16;
    s.maxdata = 1;
    s.range_table = &range_digital as *const _ as *const core::ffi::c_void;
    s.insn_bits = Some(pc263_do_insn_bits);

    // read initial relay state
    s.state = inb((*dev).iobase + PC263_DO_0_7_REG) as u32 |
        ((inb((*dev).iobase + PC263_DO_8_15_REG) as u32) << 8);

    0
}

// The following driver registration and module metadata correspond to the
// Linux module_comedi_driver and MODULE_* declarations in the C source.
#[repr(C)]
struct comedi_driver {
    driver_name: *const ::core::ffi::c_char,
    module: *const core::ffi::c_void,
    attach: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_devconfig) -> i32>,
    detach: Option<unsafe extern "C" fn(*mut comedi_device) -> i32>,
    board_name: *const *const ::core::ffi::c_char,
    offset: usize,
    num_names: usize,
}

static mut AMPLC_PC263_DRIVER: comedi_driver = comedi_driver {
    driver_name: b"amplc_pc263\0".as_ptr() as *const ::core::ffi::c_char,
    module: core::ptr::null(),
    attach: Some(pc263_attach),
    detach: Some(comedi_legacy_detach),
    board_name: &PC263_BOARDS[0].name,
    offset: core::mem::size_of::<pc263_board>(),
    num_names: PC263_BOARDS.len(),
};

// module_comedi_driver(amplc_pc263_driver);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi driver for Amplicon PC263 relay board");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
