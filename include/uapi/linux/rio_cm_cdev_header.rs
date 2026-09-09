/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * Copyright (c) 2015, Integrated Device Technology Inc.
 * Copyright (c) 2015, Prodrive Technologies
 * Copyright (c) 2015, RapidIO Trade Association
 * All rights reserved.
 *
 * This software is available under a choice of one of two licenses.
 * You may choose to be licensed under the terms of the GNU General Public
 * License (GPL) Version 2, or the BSD-3 Clause license.
 */

// Dependency equivalent of: #include <linux/types.h>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_cm_channel {
    pub id: u16,
    pub remote_channel: u16,
    pub remote_destid: u16,
    pub mport_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_cm_msg {
    pub ch_num: u16,
    pub size: u16,
    pub rxto: u32, // receive timeout in mSec. 0 = blocking
    pub msg: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_cm_accept {
    pub ch_num: u16,
    pub pad0: u16,
    pub wait_to: u32, // accept timeout in mSec. 0 = blocking
}

/* RapidIO Channelized Messaging Driver IOCTLs */
pub const RIO_CM_IOC_MAGIC: u8 = b'c';

// Linux _IOC encoding: direction in bits 30..31, size in 16..29,
// type in 8..15, and command number in 0..7.
const IOC_NONE: u32 = 0;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(direction: u32, size: u32, kind: u32, number: u32) -> u32 {
    (direction << 30) | (size << 16) | (kind << 8) | number
}

const fn iow<T>(number: u32) -> u32 {
    ioc(IOC_WRITE, core::mem::size_of::<T>() as u32, RIO_CM_IOC_MAGIC as u32, number)
}

const fn iowr<T>(number: u32) -> u32 {
    ioc(IOC_READ | IOC_WRITE, core::mem::size_of::<T>() as u32, RIO_CM_IOC_MAGIC as u32, number)
}

pub const RIO_CM_EP_GET_LIST_SIZE: u32 = iowr::<u32>(1);
pub const RIO_CM_EP_GET_LIST: u32 = iowr::<u32>(2);
pub const RIO_CM_CHAN_CREATE: u32 = iowr::<u16>(3);
pub const RIO_CM_CHAN_CLOSE: u32 = iow::<u16>(4);
pub const RIO_CM_CHAN_BIND: u32 = iow::<rio_cm_channel>(5);
pub const RIO_CM_CHAN_LISTEN: u32 = iow::<u16>(6);
pub const RIO_CM_CHAN_ACCEPT: u32 = iowr::<rio_cm_accept>(7);
pub const RIO_CM_CHAN_CONNECT: u32 = iow::<rio_cm_channel>(8);
pub const RIO_CM_CHAN_SEND: u32 = iow::<rio_cm_msg>(9);
pub const RIO_CM_CHAN_RECEIVE: u32 = iowr::<rio_cm_msg>(10);
pub const RIO_CM_MPORT_GET_LIST: u32 = iowr::<u32>(11);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
