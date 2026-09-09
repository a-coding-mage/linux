/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2014 ARM Ltd. */

use core::ffi::{c_char, c_ulong};

/* CONFIG_ARM_PTDUMP_CORE */

#[repr(C)]
pub struct mm_struct;

#[repr(C)]
pub struct seq_file;

#[repr(C)]
pub struct addr_marker {
    pub start_address: c_ulong,
    pub name: *mut c_char,
}

#[repr(C)]
pub struct ptdump_info {
    pub mm: *mut mm_struct,
    pub markers: *const addr_marker,
    pub base_addr: c_ulong,
}

extern "C" {
    pub fn ptdump_walk_pgd(s: *mut seq_file, info: *mut ptdump_info);
}

/* CONFIG_ARM_PTDUMP_DEBUGFS */
#[cfg(CONFIG_ARM_PTDUMP_DEBUGFS)]
pub const EFI_RUNTIME_MAP_END: c_ulong = SZ_1G;

#[cfg(CONFIG_ARM_PTDUMP_DEBUGFS)]
extern "C" {
    pub fn ptdump_debugfs_register(info: *mut ptdump_info, name: *const c_char);
}

#[cfg(not(CONFIG_ARM_PTDUMP_DEBUGFS))]
#[inline]
pub unsafe fn ptdump_debugfs_register(_info: *mut ptdump_info, _name: *const c_char) {}

extern "C" {
    pub fn ptdump_check_wx();
}

#[cfg(CONFIG_ARM_DEBUG_WX)]
#[macro_export]
macro_rules! arm_debug_checkwx {
    () => {
        unsafe { $crate::ptdump_check_wx() }
    };
}

#[cfg(not(CONFIG_ARM_DEBUG_WX))]
#[macro_export]
macro_rules! arm_debug_checkwx {
    () => {{
        loop {
            break;
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
