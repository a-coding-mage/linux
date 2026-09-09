/* SPDX-License-Identifier: BSD-3-Clause */
/*
 * Copyright(c) 2011 Texas Instruments, Inc.
 * Copyright(c) 2011 Google, Inc.
 * All rights reserved.
 */

// Translated from rsc_table.h. Types and symbols supplied by other headers are
// intentionally referenced as external dependencies.

#[repr(C, packed)]
pub struct resource_table {
    pub ver: u32,
    pub num: u32,
    pub reserved: [u32; 2],
    pub offset: [u32; 0],
}

#[repr(C, packed)]
pub struct fw_rsc_hdr {
    pub type_: u32,
    pub data: [u8; 0],
}

#[repr(i32)]
pub enum fw_resource_type {
    RSC_CARVEOUT = 0,
    RSC_DEVMEM = 1,
    RSC_TRACE = 2,
    RSC_VDEV = 3,
    RSC_LAST = 4,
    RSC_VENDOR_START = 128,
    RSC_VENDOR_END = 512,
}

pub const FW_RSC_ADDR_ANY: u32 = (-1i32) as u32;

#[repr(C, packed)]
pub struct fw_rsc_carveout {
    pub da: u32,
    pub pa: u32,
    pub len: u32,
    pub flags: u32,
    pub reserved: u32,
    pub name: [u8; 32],
}

#[repr(C, packed)]
pub struct fw_rsc_devmem {
    pub da: u32,
    pub pa: u32,
    pub len: u32,
    pub flags: u32,
    pub reserved: u32,
    pub name: [u8; 32],
}

#[repr(C, packed)]
pub struct fw_rsc_trace {
    pub da: u32,
    pub len: u32,
    pub reserved: u32,
    pub name: [u8; 32],
}

#[repr(C, packed)]
pub struct fw_rsc_vdev_vring {
    pub da: u32,
    pub align: u32,
    pub num: u32,
    pub notifyid: u32,
    pub pa: u32,
}

#[repr(C, packed)]
pub struct fw_rsc_vdev {
    pub id: u32,
    pub notifyid: u32,
    pub dfeatures: u32,
    pub gfeatures: u32,
    pub config_len: u32,
    pub status: u8,
    pub num_of_vrings: u8,
    pub reserved: [u8; 2],
    pub vring: [fw_rsc_vdev_vring; 0],
}

// `struct device` is supplied by the surrounding kernel bindings.
pub enum device {}

extern "C" {
    pub fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

pub const EINVAL: i32 = 22;

pub unsafe fn rsc_table_for_each_entry(
    table: *mut resource_table,
    table_sz: usize,
    dev: *mut device,
    cb: Option<unsafe extern "C" fn(u32, *mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void) -> i32>,
    data: *mut core::ffi::c_void,
) -> i32 {
    let mut i: u32 = 0;
    let mut ret: i32;

    while i < (*table).num {
        let offset = *((table as *mut u8).add(core::mem::offset_of!(resource_table, offset) + (i as usize) * core::mem::size_of::<u32>()) as *const u32);
        let hdr: *mut fw_rsc_hdr;
        let avail: i32;
        let rsc_offset: i32;
        let rsc: *mut core::ffi::c_void;

        if (offset as usize) < core::mem::size_of::<resource_table>()
            || (offset as usize) >= table_sz
            || table_sz - offset as usize < core::mem::size_of::<fw_rsc_hdr>()
        {
            dev_err(dev, b"rsc table is truncated\0".as_ptr() as *const core::ffi::c_char);
            return -EINVAL;
        }

        hdr = (table as *mut u8).add(offset as usize) as *mut fw_rsc_hdr;
        avail = (table_sz - offset as usize - core::mem::size_of::<fw_rsc_hdr>()) as i32;
        rsc_offset = (offset as usize + core::mem::size_of::<fw_rsc_hdr>()) as i32;
        rsc = (hdr as *mut u8).add(core::mem::size_of::<fw_rsc_hdr>()) as *mut core::ffi::c_void;

        ret = cb.unwrap()((*hdr).type_, rsc, rsc_offset, avail, data);
        if ret != 0 {
            return ret;
        }
        i = i.wrapping_add(1);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
