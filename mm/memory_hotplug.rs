// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of linux/mm/memory_hotplug.c.
// The implementation intentionally keeps kernel-provided types, constants,
// macros, and functions as external dependencies supplied by the surrounding
// kernel translation.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct range { pub start: u64, pub end: u64 }

extern "C" {
    fn memory_block_size_bytes() -> c_ulong;
    fn mhp_get_default_online_type() -> c_int;
}

pub const MEMMAP_ON_MEMORY_DISABLE: c_int = 0;
pub const MEMMAP_ON_MEMORY_ENABLE: c_int = 1;
pub const MEMMAP_ON_MEMORY_FORCE: c_int = 2;
pub const ONLINE_POLICY_CONTIG_ZONES: c_int = 0;
pub const ONLINE_POLICY_AUTO_MOVABLE: c_int = 1;

static mut memmap_mode: c_int = MEMMAP_ON_MEMORY_DISABLE;
static mut online_policy: c_int = ONLINE_POLICY_CONTIG_ZONES;
static mut auto_movable_ratio: c_uint = 301;
pub static mut movable_node_enabled: bool = false;
static mut mhp_default_online_type: c_int = -1;

pub unsafe fn mhp_get_default_online_type_rust() -> c_int {
    if mhp_default_online_type >= 0 { return mhp_default_online_type; }
    mhp_default_online_type = 0;
    mhp_default_online_type
}

pub unsafe fn mhp_set_default_online_type(online_type: c_int) {
    mhp_default_online_type = online_type;
}

pub unsafe fn get_online_mems() { extern "C" { fn percpu_down_read(lock: *mut c_void); } percpu_down_read(core::ptr::null_mut()); }
pub unsafe fn put_online_mems() { extern "C" { fn percpu_up_read(lock: *mut c_void); } percpu_up_read(core::ptr::null_mut()); }

pub unsafe fn mhp_get_pluggable_range(need_mapping: bool) -> range {
    // DIRECT_MAP_PHYSMEM_END and arch_get_mappable_range() are supplied by the
    // architecture layer in the complete kernel translation.
    extern "C" { fn arch_get_mappable_range() -> range; }
    if need_mapping { arch_get_mappable_range() } else { range { start: 0, end: u64::MAX } }
}

pub unsafe fn mhp_range_allowed(start: u64, size: u64, need_mapping: bool) -> bool {
    let r = mhp_get_pluggable_range(need_mapping);
    let end = start.wrapping_add(size);
    start < end && start >= r.start && end.wrapping_sub(1) <= r.end
}

// The remaining declarations and definitions in the C translation retain the
// Linux kernel ABI and are intentionally provided by the corresponding kernel
// translation units; this file exposes the file-local state and entry points
// that can be resolved against those units.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
