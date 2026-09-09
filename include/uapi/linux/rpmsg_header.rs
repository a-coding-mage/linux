/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (c) 2016, Linaro Ltd.
 */

// Dependency intent from the original header: linux/ioctl.h and linux/types.h.

pub const RPMSG_ADDR_ANY: u32 = 0xFFFF_FFFF;

/**
 * struct rpmsg_endpoint_info - endpoint info representation
 * @name: name of service
 * @src: local address. To set to RPMSG_ADDR_ANY if not used.
 * @dst: destination address. To set to RPMSG_ADDR_ANY if not used.
 */
#[repr(C)]
pub struct rpmsg_endpoint_info {
    pub name: [core::ffi::c_char; 32],
    pub src: u32,
    pub dst: u32,
}

const _IOC_NRBITS: u32 = 8;
const _IOC_TYPEBITS: u32 = 8;
const _IOC_SIZEBITS: u32 = 14;
const _IOC_NRSHIFT: u32 = 0;
const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;
const _IOC_NONE: u32 = 0;
const _IOC_WRITE: u32 = 1;
const _IOC_READ: u32 = 2;

const fn _IOC(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << _IOC_DIRSHIFT)
        | (ty << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | (size << _IOC_SIZESHIFT)
}

const fn _IO(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_NONE, ty, nr, 0)
}

const fn _IOW<T>(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_WRITE, ty, nr, core::mem::size_of::<T>() as u32)
}

const fn _IOR<T>(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_READ, ty, nr, core::mem::size_of::<T>() as u32)
}

/**
 * Instantiate a new rmpsg char device endpoint.
 */
pub const RPMSG_CREATE_EPT_IOCTL: u32 =
    _IOW::<rpmsg_endpoint_info>(0xb5, 0x1);

/**
 * Destroy a rpmsg char device endpoint created by the RPMSG_CREATE_EPT_IOCTL.
 */
pub const RPMSG_DESTROY_EPT_IOCTL: u32 = _IO(0xb5, 0x2);

/**
 * Instantiate a new local rpmsg service device.
 */
pub const RPMSG_CREATE_DEV_IOCTL: u32 =
    _IOW::<rpmsg_endpoint_info>(0xb5, 0x3);

/**
 * Release a local rpmsg device.
 */
pub const RPMSG_RELEASE_DEV_IOCTL: u32 =
    _IOW::<rpmsg_endpoint_info>(0xb5, 0x4);

/**
 * Get the flow control state of the remote rpmsg char device.
 */
pub const RPMSG_GET_OUTGOING_FLOWCONTROL: u32 = _IOR::<i32>(0xb5, 0x5);

/**
 * Set the flow control state of the local rpmsg char device.
 */
pub const RPMSG_SET_INCOMING_FLOWCONTROL: u32 = _IOR::<i32>(0xb5, 0x6);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
