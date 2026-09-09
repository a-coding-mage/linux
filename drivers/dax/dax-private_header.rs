/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2016 Intel Corporation. All rights reserved. */

/* Declarations supplied by the surrounding kernel translation. */
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct dax_device { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct cdev { _private: [u8; 0] }
#[repr(C)] pub struct kref { _private: [u8; 0] }
#[repr(C)] pub struct ida { _private: [u8; 0] }
#[repr(C)] pub struct resource { _private: [u8; 0] }
#[repr(C)] pub struct range { _private: [u8; 0] }
#[repr(C)] pub struct dev_pagemap { _private: [u8; 0] }

pub type phys_addr_t = u64;
pub type pgoff_t = usize;

extern "C" {
    pub fn inode_dax(inode: *mut inode) -> *mut dax_device;
    pub fn dax_inode(dax_dev: *mut dax_device) -> *mut inode;
    pub fn dax_bus_init() -> ::core::ffi::c_int;
    pub fn dax_bus_exit();
}

#[repr(C)]
pub struct dax_region {
    pub id: ::core::ffi::c_int,
    pub target_node: ::core::ffi::c_int,
    pub kref: kref,
    pub dev: *mut device,
    pub align: ::core::ffi::c_uint,
    pub ida: ida,
    pub res: resource,
    pub seed: *mut device,
    pub youngest: *mut device,
}

#[repr(C)]
pub struct dax_mapping {
    pub dev: device,
    pub range_id: ::core::ffi::c_int,
    pub id: ::core::ffi::c_int,
}

#[repr(C)]
pub struct dev_dax_range {
    pub pgoff: usize,
    pub range: range,
    pub mapping: *mut dax_mapping,
}

#[repr(C)]
pub struct dev_dax {
    pub region: *mut dax_region,
    pub dax_dev: *mut dax_device,
    pub cached_size: u64,
    pub align: ::core::ffi::c_uint,
    pub target_node: ::core::ffi::c_int,
    pub dyn_id: bool,
    pub id: ::core::ffi::c_int,
    pub ida: ida,
    pub dev: device,
    pub pgmap: *mut dev_pagemap,
    pub memmap_on_memory: bool,
    pub nr_range: ::core::ffi::c_int,
    pub ranges: *mut dev_dax_range,
}

extern "C" {
    pub fn run_dax(dax_dev: *mut dax_device);
    pub fn dax_pgoff_to_phys(dev_dax: *mut dev_dax, pgoff: pgoff_t, size: usize) -> phys_addr_t;
}

#[inline]
pub unsafe fn to_dev_dax(dev: *mut device) -> *mut dev_dax {
    (dev as *mut u8).sub(::core::mem::offset_of!(dev_dax, dev)) as *mut dev_dax
}

#[inline]
pub unsafe fn to_dax_mapping(dev: *mut device) -> *mut dax_mapping {
    (dev as *mut u8).sub(::core::mem::offset_of!(dax_mapping, dev)) as *mut dax_mapping
}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
#[inline]
pub fn dax_align_valid(align: usize) -> bool {
    if align == PUD_SIZE && cfg!(feature = "CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD") { return true; }
    if align == PMD_SIZE && has_transparent_hugepage() { return true; }
    if align == PAGE_SIZE { return true; }
    false
}

#[cfg(not(feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
#[inline]
pub fn dax_align_valid(align: usize) -> bool { align == PAGE_SIZE }

extern "C" {
    pub fn has_transparent_hugepage() -> bool;
}

extern "C" {
    static PAGE_SIZE: usize;
    static PMD_SIZE: usize;
    static PUD_SIZE: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
