// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/amplc_dio200.c
 *
 * Driver for Amplicon PC212E, PC214E, PC215E, PC218E, PC272E.
 *
 * Copyright (C) 2005-2013 MEV Ltd. <https://www.mev.co.uk/>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998,2000 David A. Schleef <ds@schleef.org>
 */

// Driver documentation and configuration notes from the original source are
// retained in the source comments above and in the corresponding C driver.

// Dependencies supplied by the surrounding Comedi driver environment.
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct dio200_board {
    pub name: *const c_char,
    pub n_subdevs: c_uint,
    pub sdtype: [c_int; 7],
    pub sdinfo: [u8; 7],
    pub has_int_sce: bool,
    pub has_clk_gat_sce: bool,
}

#[repr(C)]
pub struct comedi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comedi_devconfig {
    pub options: [c_uint; 2],
}

#[repr(C)]
pub struct comedi_driver {
    pub driver_name: *const c_char,
    pub module: *mut c_void,
    pub attach: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_devconfig) -> c_int>,
    pub detach: Option<unsafe extern "C" fn(*mut comedi_device) -> c_int>,
    pub board_name: *const *const c_char,
    pub offset: usize,
    pub num_names: usize,
}

// Values declared by amplc_dio200.h.
extern "C" {
    static THIS_MODULE: *mut c_void;
    static sd_8254: c_int;
    static sd_8255: c_int;
    static sd_intr: c_int;

    fn comedi_check_request_region(
        dev: *mut comedi_device,
        from: c_uint,
        len: c_uint,
        min: c_uint,
        max: c_uint,
        align: c_uint,
    ) -> c_int;
    fn amplc_dio200_common_attach(
        dev: *mut comedi_device,
        irq: c_uint,
        flags: c_uint,
    ) -> c_int;
    fn comedi_legacy_detach(dev: *mut comedi_device) -> c_int;
}

#[repr(C)]
pub struct Dio200IsaBoard {
    pub name: *const c_char,
    pub n_subdevs: c_uint,
    pub sdtype: [c_int; 7],
    pub sdinfo: [u8; 7],
    pub has_int_sce: bool,
    pub has_clk_gat_sce: bool,
}

/* Board descriptions. */
#[no_mangle]
pub static dio200_isa_boards: [Dio200IsaBoard; 5] = [
    Dio200IsaBoard {
        name: b"pc212e\0".as_ptr() as *const c_char,
        n_subdevs: 6,
        sdtype: [unsafe { sd_8255 }, unsafe { sd_8254 }, unsafe { sd_8254 }, unsafe { sd_8254 }, unsafe { sd_8254 }, unsafe { sd_intr }, 0],
        sdinfo: [0x00, 0x08, 0x0c, 0x10, 0x14, 0x3f, 0],
        has_int_sce: true,
        has_clk_gat_sce: true,
    },
    Dio200IsaBoard {
        name: b"pc214e\0".as_ptr() as *const c_char,
        n_subdevs: 4,
        sdtype: [unsafe { sd_8255 }, unsafe { sd_8255 }, unsafe { sd_8254 }, unsafe { sd_intr }, 0, 0, 0],
        sdinfo: [0x00, 0x08, 0x10, 0x01, 0, 0, 0],
        has_int_sce: false,
        has_clk_gat_sce: false,
    },
    Dio200IsaBoard {
        name: b"pc215e\0".as_ptr() as *const c_char,
        n_subdevs: 5,
        sdtype: [unsafe { sd_8255 }, unsafe { sd_8255 }, unsafe { sd_8254 }, unsafe { sd_8254 }, unsafe { sd_intr }, 0, 0],
        sdinfo: [0x00, 0x08, 0x10, 0x14, 0x3f, 0, 0],
        has_int_sce: true,
        has_clk_gat_sce: true,
    },
    Dio200IsaBoard {
        name: b"pc218e\0".as_ptr() as *const c_char,
        n_subdevs: 7,
        sdtype: [unsafe { sd_8254 }, unsafe { sd_8254 }, unsafe { sd_8255 }, unsafe { sd_8254 }, unsafe { sd_8254 }, unsafe { sd_intr }, 0],
        sdinfo: [0x00, 0x04, 0x08, 0x0c, 0x10, 0x14, 0x3f],
        has_int_sce: true,
        has_clk_gat_sce: true,
    },
    Dio200IsaBoard {
        name: b"pc272e\0".as_ptr() as *const c_char,
        n_subdevs: 4,
        sdtype: [unsafe { sd_8255 }, unsafe { sd_8255 }, unsafe { sd_8255 }, unsafe { sd_intr }, 0, 0, 0],
        sdinfo: [0x00, 0x08, 0x10, 0x3f, 0, 0, 0],
        has_int_sce: true,
        has_clk_gat_sce: false,
    },
];

unsafe extern "C" fn dio200_attach(
    dev: *mut comedi_device,
    it: *mut comedi_devconfig,
) -> c_int {
    let ret = comedi_check_request_region(dev, (*it).options[0], 0x20, 0, 0xfff, 0x20);
    if ret != 0 {
        return ret;
    }
    amplc_dio200_common_attach(dev, (*it).options[1], 0)
}

#[no_mangle]
pub static mut amplc_dio200_driver: comedi_driver = comedi_driver {
    driver_name: b"amplc_dio200\0".as_ptr() as *const c_char,
    module: core::ptr::null_mut(),
    attach: Some(dio200_attach),
    detach: Some(comedi_legacy_detach),
    board_name: &dio200_isa_boards[0].name,
    offset: core::mem::size_of::<Dio200IsaBoard>(),
    num_names: dio200_isa_boards.len(),
};

// module_comedi_driver(amplc_dio200_driver);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi driver for Amplicon 200 Series ISA DIO boards");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
