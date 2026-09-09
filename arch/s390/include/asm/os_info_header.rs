/* SPDX-License-Identifier: GPL-2.0 */
/*
 * OS info memory interface
 *
 * Copyright IBM Corp. 2012
 * Author(s): Michael Holzheu <holzheu@linux.vnet.ibm.com>
 */

// Dependency supplied by the surrounding kernel translation.

pub const OS_INFO_VERSION_MAJOR: u32 = 1;
pub const OS_INFO_VERSION_MINOR: u32 = 1;
pub const OS_INFO_MAGIC: u64 = 0x4f53494e464f535a_u64; /* OSINFOSZ */

pub const OS_INFO_VMCOREINFO: u32 = 0;
pub const OS_INFO_REIPL_BLOCK: u32 = 1;
pub const OS_INFO_FLAGS_ENTRY: u32 = 2;
pub const OS_INFO_RESERVED: u32 = 3;
pub const OS_INFO_IDENTITY_BASE: u32 = 4;
pub const OS_INFO_KASLR_OFFSET: u32 = 5;
pub const OS_INFO_KASLR_OFF_PHYS: u32 = 6;
pub const OS_INFO_VMEMMAP: u32 = 7;
pub const OS_INFO_AMODE31_START: u32 = 8;
pub const OS_INFO_AMODE31_END: u32 = 9;
pub const OS_INFO_IMAGE_START: u32 = 10;
pub const OS_INFO_IMAGE_END: u32 = 11;
pub const OS_INFO_IMAGE_PHYS: u32 = 12;
pub const OS_INFO_MAX: usize = 13;

pub const OS_INFO_FLAG_REIPL_CLEAR: usize = 1usize << 0;

#[repr(C)]
pub union os_info_entry_addr_val {
    pub addr: u64,
    pub val: u64,
}

#[repr(C, packed)]
pub struct os_info_entry {
    pub addr: os_info_entry_addr_val,
    pub size: u64,
    pub csum: u32,
}

#[repr(C, packed)]
pub struct os_info {
    pub magic: u64,
    pub csum: u32,
    pub version_major: u16,
    pub version_minor: u16,
    pub crashkernel_addr: u64,
    pub crashkernel_size: u64,
    pub entry: [os_info_entry; OS_INFO_MAX],
    pub reserved: [u8; 3804],
}

extern "C" {
    pub fn os_info_init();
    pub fn os_info_entry_add_data(nr: i32, ptr: *mut core::ffi::c_void, len: u64);
    pub fn os_info_entry_add_val(nr: i32, val: u64);
    pub fn os_info_crashkernel_add(base: core::ffi::c_ulong, size: core::ffi::c_ulong);
    pub fn os_info_csum(os_info: *mut os_info) -> u32;
}

// CONFIG_CRASH_DUMP conditional from the original header.
#[cfg(feature = "CONFIG_CRASH_DUMP")]
extern "C" {
    pub fn os_info_old_entry(nr: i32, size: *mut core::ffi::c_ulong) -> *mut core::ffi::c_void;
}

#[cfg(feature = "CONFIG_CRASH_DUMP")]
#[inline]
pub unsafe fn os_info_old_value(nr: i32) -> core::ffi::c_ulong {
    let mut size: core::ffi::c_ulong = 0;
    os_info_old_entry(nr, &mut size)
        as usize as core::ffi::c_ulong
}

#[cfg(not(feature = "CONFIG_CRASH_DUMP"))]
#[inline]
pub unsafe fn os_info_old_entry(
    _nr: i32,
    _size: *mut core::ffi::c_ulong,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
