// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/amplc_pc236.c
 * Driver for Amplicon PC36AT DIO boards.
 *
 * Copyright (C) 2002 MEV Ltd. <https://www.mev.co.uk/>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */
/*
 * Driver: amplc_pc236
 * Description: Amplicon PC36AT
 * Author: Ian Abbott <abbotti@mev.co.uk>
 * Devices: [Amplicon] PC36AT (pc36at)
 * Updated: Fri, 25 Jul 2014 15:32:40 +0000
 * Status: works
 *
 * Configuration options - PC36AT:
 *   [0] - I/O port base address
 *   [1] - IRQ (optional)
 *
 * The PC36AT board has a single 8255 appearing as subdevice 0.
 *
 * Subdevice 1 pretends to be a digital input device, but it always returns
 * 0 when read. However, if you run a command with scan_begin_src=TRIG_EXT,
 * a rising edge on port C bit 3 acts as an external trigger, which can be
 * used to wake up tasks. This is like the comedi_parport device, but the
 * only way to physically disable the interrupt on the PC36AT is to remove
 * the IRQ jumper. If no interrupt is connected, then subdevice 1 is unused.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Definitions supplied by the Linux Comedi headers and amplc_pc236.h.
pub enum comedi_device {}
pub enum comedi_devconfig {}
pub enum pc236_private {}

#[repr(C)]
pub struct pc236_board {
    pub name: *const c_char,
}

#[repr(C)]
pub struct comedi_driver {
    pub driver_name: *const c_char,
    pub module: *mut c_void,
    pub attach: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_devconfig) -> c_int>,
    pub detach: Option<unsafe extern "C" fn(*mut comedi_device) -> c_int>,
    pub board_name: *const *const c_char,
    pub offset: usize,
    pub num_names: c_uint,
}

unsafe extern "C" {
    fn comedi_alloc_devpriv(dev: *mut comedi_device, size: usize) -> *mut c_void;
    fn comedi_check_request_region(
        dev: *mut comedi_device,
        start: c_uint,
        len: c_uint,
        min: c_uint,
        max: c_uint,
        align: c_uint,
    ) -> c_int;
    fn amplc_pc236_common_attach(
        dev: *mut comedi_device,
        iobase: c_uint,
        irq: c_uint,
        unused: c_uint,
    ) -> c_int;
    fn comedi_legacy_detach(dev: *mut comedi_device) -> c_int;
}

// The kernel module pointer and registration macro are supplied by the build environment.
unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    fn module_comedi_driver(driver: *mut comedi_driver);
}

unsafe extern "C" {
    fn comedi_device_iobase(dev: *mut comedi_device) -> c_uint;
    fn comedi_devconfig_option(config: *mut comedi_devconfig, index: usize) -> c_uint;
}

unsafe extern "C" fn pc236_attach(
    dev: *mut comedi_device,
    it: *mut comedi_devconfig,
) -> c_int {
    let devpriv: *mut pc236_private = comedi_alloc_devpriv(
        dev,
        core::mem::size_of::<pc236_private>(),
    ) as *mut pc236_private;
    if devpriv.is_null() {
        return -12; // -ENOMEM
    }

    let ret = comedi_check_request_region(
        dev,
        comedi_devconfig_option(it, 0),
        0x4,
        0,
        0xfff,
        4,
    );
    if ret != 0 {
        return ret;
    }

    amplc_pc236_common_attach(
        dev,
        comedi_device_iobase(dev),
        comedi_devconfig_option(it, 1),
        0,
    )
}

static mut PC236_BOARDS: [pc236_board; 1] = [pc236_board {
    name: b"pc36at\0".as_ptr() as *const c_char,
}];

static mut AMPLC_PC236_DRIVER: comedi_driver = comedi_driver {
    driver_name: b"amplc_pc236\0".as_ptr() as *const c_char,
    module: core::ptr::null_mut(), // THIS_MODULE
    attach: Some(pc236_attach),
    detach: Some(comedi_legacy_detach),
    board_name: core::ptr::null(),
    offset: core::mem::size_of::<pc236_board>(),
    num_names: 1,
};

// Equivalent to module_comedi_driver(amplc_pc236_driver).
#[allow(dead_code)]
unsafe fn register_driver() {
    AMPLC_PC236_DRIVER.module = THIS_MODULE;
    AMPLC_PC236_DRIVER.board_name = &PC236_BOARDS[0].name;
    module_comedi_driver(&mut AMPLC_PC236_DRIVER);
}

// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi driver for Amplicon PC36AT DIO boards");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
