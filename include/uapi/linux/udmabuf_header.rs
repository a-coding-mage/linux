/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Equivalent of the Linux UAPI types used by this header.

pub const UDMABUF_FLAGS_CLOEXEC: u32 = 0x01;

#[repr(C)]
pub struct udmabuf_create {
    pub memfd: u32,
    pub flags: u32,
    pub offset: u64,
    pub size: u64,
}

#[repr(C)]
pub struct udmabuf_create_item {
    pub memfd: u32,
    pub __pad: u32,
    pub offset: u64,
    pub size: u64,
}

#[repr(C)]
pub struct udmabuf_create_list {
    pub flags: u32,
    pub count: u32,
    pub list: [udmabuf_create_item; 0],
}

// _IOW('u', 0x42, struct udmabuf_create)
pub const UDMABUF_CREATE: u32 = 0x4018_7542;

// _IOW('u', 0x43, struct udmabuf_create_list)
pub const UDMABUF_CREATE_LIST: u32 = 0x4008_7543;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
