/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 *    copyright            : (C) 2002 by Frank Mori Hess
 ***************************************************************************/

// Dependencies supplied by the translated equivalents of nec7210.h,
// gpibP.h, and plx9050.h.

#[repr(C)]
pub struct cec_priv {
    pub nec7210_priv: nec7210_priv,
    pub pci_device: *mut pci_dev,
    // base address for plx9052 pci chip
    pub plx_iobase: ::core::ffi::c_ulong,
    pub irq: ::core::ffi::c_uint,
}

// offset between consecutive nec7210 registers
static cec_reg_offset: ::core::ffi::c_int = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
