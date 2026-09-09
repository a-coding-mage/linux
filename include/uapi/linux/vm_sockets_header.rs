/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * VMware vSockets Driver
 *
 * Copyright (C) 2007-2013 VMware, Inc. All rights reserved.
 */

/* C header dependencies are supplied by the surrounding UAPI translation. */

/* Option names for vSockets socket buffers and connection behavior. */
pub const SO_VM_SOCKETS_BUFFER_SIZE: i32 = 0;
pub const SO_VM_SOCKETS_BUFFER_MIN_SIZE: i32 = 1;
pub const SO_VM_SOCKETS_BUFFER_MAX_SIZE: i32 = 2;
pub const SO_VM_SOCKETS_PEER_HOST_VM_ID: i32 = 3;
pub const SO_VM_SOCKETS_TRUSTED: i32 = 5;
pub const SO_VM_SOCKETS_CONNECT_TIMEOUT_OLD: i32 = 6;
pub const SO_VM_SOCKETS_NONBLOCK_TXRX: i32 = 7;
pub const SO_VM_SOCKETS_CONNECT_TIMEOUT_NEW: i32 = 8;

/* The source condition depends on target/build configuration. */
#[cfg(not(target_os = "linux"))]
pub const SO_VM_SOCKETS_CONNECT_TIMEOUT: i32 = SO_VM_SOCKETS_CONNECT_TIMEOUT_NEW;

pub const VMADDR_CID_ANY: u32 = !0u32;
pub const VMADDR_PORT_ANY: u32 = !0u32;
pub const VMADDR_CID_HYPERVISOR: u32 = 0;
pub const VMADDR_CID_LOCAL: u32 = 1;
pub const VMADDR_CID_HOST: u32 = 2;
pub const VMADDR_FLAG_TO_HOST: u8 = 0x01;

pub const VM_SOCKETS_INVALID_VERSION: u32 = !0u32;

#[inline]
pub const fn VM_SOCKETS_VERSION_EPOCH(v: u32) -> u32 {
    (v & 0xFF000000) >> 24
}

#[inline]
pub const fn VM_SOCKETS_VERSION_MAJOR(v: u32) -> u32 {
    (v & 0x00FF0000) >> 16
}

#[inline]
pub const fn VM_SOCKETS_VERSION_MINOR(v: u32) -> u32 {
    v & 0x0000FFFF
}

#[repr(C)]
pub struct sockaddr_vm {
    pub svm_family: __kernel_sa_family_t,
    pub svm_reserved1: u16,
    pub svm_port: u32,
    pub svm_cid: u32,
    pub svm_flags: __u8,
    pub svm_zero: [u8; core::mem::size_of::<struct_sockaddr>()
        - core::mem::size_of::<sa_family_t>()
        - core::mem::size_of::<u16>()
        - core::mem::size_of::<u32>()
        - core::mem::size_of::<u32>()
        - core::mem::size_of::<__u8>()],
}

pub const IOCTL_VM_SOCKETS_GET_LOCAL_CID: _ = _IO(7, 0xb9);

pub const SOL_VSOCK: i32 = 287;
pub const VSOCK_RECVERR: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
