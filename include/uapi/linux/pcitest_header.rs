/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * pcitest.h - PCI test uapi defines
 *
 * Copyright (C) 2017 Texas Instruments
 * Author: Kishon Vijay Abraham I <kishon@ti.com>
 *
 */

// The following helpers are the Linux _IOC, _IO, and _IOW encodings.
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_NONE: u32 = 0;
const IOC_WRITE: u32 = 1;

const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << IOC_DIRSHIFT)
        | (ty << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)
}

const fn io(ty: u32, nr: u32) -> u32 {
    ioc(IOC_NONE, ty, nr, 0)
}

const fn iow<T>(ty: u32, nr: u32) -> u32 {
    ioc(IOC_WRITE, ty, nr, core::mem::size_of::<T>() as u32)
}

pub const PCITEST_BAR: u32 = io(b'P' as u32, 0x1);
pub const PCITEST_INTX_IRQ: u32 = io(b'P' as u32, 0x2);
pub const PCITEST_LEGACY_IRQ: u32 = PCITEST_INTX_IRQ;
pub const PCITEST_MSI: u32 = iow::<core::ffi::c_int>(b'P' as u32, 0x3);
pub const PCITEST_WRITE: u32 = iow::<core::ffi::c_ulong>(b'P' as u32, 0x4);
pub const PCITEST_READ: u32 = iow::<core::ffi::c_ulong>(b'P' as u32, 0x5);
pub const PCITEST_COPY: u32 = iow::<core::ffi::c_ulong>(b'P' as u32, 0x6);
pub const PCITEST_MSIX: u32 = iow::<core::ffi::c_int>(b'P' as u32, 0x7);
pub const PCITEST_SET_IRQTYPE: u32 = iow::<core::ffi::c_int>(b'P' as u32, 0x8);
pub const PCITEST_GET_IRQTYPE: u32 = io(b'P' as u32, 0x9);
pub const PCITEST_BARS: u32 = io(b'P' as u32, 0xa);
pub const PCITEST_DOORBELL: u32 = io(b'P' as u32, 0xb);
pub const PCITEST_BAR_SUBRANGE: u32 = io(b'P' as u32, 0xc);
pub const PCITEST_CLEAR_IRQ: u32 = io(b'P' as u32, 0x10);

pub const PCITEST_IRQ_TYPE_UNDEFINED: i32 = -1;
pub const PCITEST_IRQ_TYPE_INTX: i32 = 0;
pub const PCITEST_IRQ_TYPE_MSI: i32 = 1;
pub const PCITEST_IRQ_TYPE_MSIX: i32 = 2;
pub const PCITEST_IRQ_TYPE_AUTO: i32 = 3;

pub const PCITEST_FLAGS_USE_DMA: u32 = 0x00000001;

#[repr(C)]
pub struct pci_endpoint_test_xfer_param {
    pub size: core::ffi::c_ulong,
    pub flags: core::ffi::c_uchar,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
