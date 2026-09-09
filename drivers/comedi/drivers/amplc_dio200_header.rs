/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * comedi/drivers/amplc_dio.h
 *
 * Header for amplc_dio200.c, amplc_dio200_common.c and
 * amplc_dio200_pci.c.
 *
 * Copyright (C) 2005-2013 MEV Ltd. <https://www.mev.co.uk/>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998,2000 David A. Schleef <ds@schleef.org>
 */

use core::ffi::{c_char, c_ulong};

pub struct comedi_device;

/*
 * Subdevice types.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dio200_sdtype {
    sd_none,
    sd_intr,
    sd_8255,
    sd_8254,
    sd_timer,
}

pub const DIO200_MAX_SUBDEVS: usize = 8;
pub const DIO200_MAX_ISNS: usize = 6;

#[repr(C)]
pub struct dio200_board {
    pub name: *const c_char,
    pub mainbar: u8,
    pub n_subdevs: u16,
    /* number of subdevices */
    pub sdtype: [u8; DIO200_MAX_SUBDEVS],
    /* enum dio200_sdtype */
    pub sdinfo: [u8; DIO200_MAX_SUBDEVS],
    /* depends on sdtype */
    /* The following C bit-fields occupy one unsigned int. */
    pub has_int_sce: u32,
    /* has interrupt enable/status reg */
    pub has_clk_gat_sce: u32,
    /* has clock/gate selection registers */
    pub is_pcie: u32,
    /* has enhanced features */
}

unsafe extern "C" {
    pub fn amplc_dio200_common_attach(
        dev: *mut comedi_device,
        irq: u32,
        req_irq_flags: c_ulong,
    ) -> i32;

    /* Used by initialization of PCIe boards. */
    pub fn amplc_dio200_set_enhance(dev: *mut comedi_device, val: u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
