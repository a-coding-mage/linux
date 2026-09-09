/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * comedi/drivers/amplc_pc236.h
 * Header for "amplc_pc236", "amplc_pci236" and "amplc_pc236_common".
 *
 * Copyright (C) 2002-2014 MEV Ltd. <https://www.mev.co.uk/>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

use core::ffi::{c_char, c_uint, c_ulong};

#[repr(C)]
pub struct comedi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pc236_board {
    pub name: *const c_char,
    pub intr_update_cb:
        Option<unsafe extern "C" fn(dev: *mut comedi_device, enable: bool)>,
    pub intr_chk_clr_cb:
        Option<unsafe extern "C" fn(dev: *mut comedi_device) -> bool>,
}

#[repr(C)]
pub struct pc236_private {
    /* PLX PCI9052 config registers in PCIBAR1 */
    pub lcr_iobase: c_ulong,
    pub enable_irq: bool,
}

unsafe extern "C" {
    pub fn amplc_pc236_common_attach(
        dev: *mut comedi_device,
        iobase: c_ulong,
        irq: c_uint,
        req_irq_flags: c_ulong,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
