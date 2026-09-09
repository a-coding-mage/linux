/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Part of the HSI character device driver.
 *
 * Copyright (C) 2010 Nokia Corporation. All rights reserved.
 *
 * Contact: Andras Domokos <andras.domokos at nokia.com>
 */

/* C dependency: <linux/types.h> */

pub const HSI_CHAR_MAGIC: u32 = b'k' as u32;

/* Linux ioctl encoding, corresponding to _IOC/_IOW/_IOR/_IOWR. */
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << IOC_DIRSHIFT)
        | (ty << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)
}

const fn io(nr: u32) -> u32 {
    ioc(0, HSI_CHAR_MAGIC, nr, 0)
}

const fn iow<T>(nr: u32) -> u32 {
    ioc(IOC_WRITE, HSI_CHAR_MAGIC, nr, core::mem::size_of::<T>() as u32)
}

const fn ior<T>(nr: u32) -> u32 {
    ioc(IOC_READ, HSI_CHAR_MAGIC, nr, core::mem::size_of::<T>() as u32)
}

const fn iowr<T>(nr: u32) -> u32 {
    ioc(
        IOC_READ | IOC_WRITE,
        HSI_CHAR_MAGIC,
        nr,
        core::mem::size_of::<T>() as u32,
    )
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsc_rx_config {
    pub mode: u32,
    pub flow: u32,
    pub channels: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsc_tx_config {
    pub mode: u32,
    pub channels: u32,
    pub speed: u32,
    pub arb_mode: u32,
}

pub const HSC_RESET: u32 = io(16);
pub const HSC_SET_PM: u32 = io(17);
pub const HSC_SEND_BREAK: u32 = io(18);
pub const HSC_SET_RX: u32 = iow::<hsc_rx_config>(19);
pub const HSC_GET_RX: u32 = iow::<hsc_rx_config>(20);
pub const HSC_SET_TX: u32 = iow::<hsc_tx_config>(21);
pub const HSC_GET_TX: u32 = iow::<hsc_tx_config>(22);

pub const HSC_PM_DISABLE: u32 = 0;
pub const HSC_PM_ENABLE: u32 = 1;

pub const HSC_MODE_STREAM: u32 = 1;
pub const HSC_MODE_FRAME: u32 = 2;
pub const HSC_FLOW_SYNC: u32 = 0;
pub const HSC_ARB_RR: u32 = 0;
pub const HSC_ARB_PRIO: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
