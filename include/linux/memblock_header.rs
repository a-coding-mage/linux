/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Logical memory blocks. */

// Dependencies supplied by other translated headers/build configurations.

extern "C" {
    pub static mut max_low_pfn: ::core::primitive::c_ulong;
    pub static mut min_low_pfn: ::core::primitive::c_ulong;
    pub static mut max_pfn: ::core::primitive::c_ulong;
    pub static mut max_possible_pfn: u64;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum memblock_flags {
    MEMBLOCK_NONE = 0x0,
    MEMBLOCK_HOTPLUG = 0x1,
    MEMBLOCK_MIRROR = 0x2,
    MEMBLOCK_NOMAP = 0x4,
    MEMBLOCK_DRIVER_MANAGED = 0x8,
    MEMBLOCK_RSRV_NOINIT = 0x10,
    MEMBLOCK_RSRV_KERN = 0x20,
    MEMBLOCK_KHO_SCRATCH = 0x40,
    MEMBLOCK_RSRV_HUGETLB = 0x80,
}

#[repr(C)]
pub struct memblock_region {
    pub base: phys_addr_t,
    pub size: phys_addr_t,
    pub flags: memblock_flags,
    // CONFIG_NUMA: this field is present only in NUMA builds.
    #[cfg(CONFIG_NUMA)]
    pub nid: ::core::primitive::c_int,
}

#[repr(C)]
pub struct memblock_type {
    pub cnt: ::core::primitive::c_ulong,
    pub max: ::core::primitive::c_ulong,
    pub total_size: phys_addr_t,
    pub regions: *mut memblock_region,
    pub name: *mut ::core::primitive::c_char,
}

#[repr(C)]
pub struct memblock {
    pub bottom_up: bool,
    pub current_limit: phys_addr_t,
    pub memory: memblock_type,
    pub reserved: memblock_type,
}

extern "C" {
    pub static mut memblock: memblock;
    pub fn memblock_discard();
    pub fn memblock_allow_resize();
    pub fn memblock_add_node(base: phys_addr_t, size: phys_addr_t, nid: ::core::primitive::c_int, flags: memblock_flags) -> ::core::primitive::c_int;
    pub fn memblock_add(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int;
    pub fn memblock_remove(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int;
    pub fn memblock_phys_free(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int;
    pub fn __memblock_reserve(base: phys_addr_t, size: phys_addr_t, nid: ::core::primitive::c_int, flags: memblock_flags) -> ::core::primitive::c_int;
    pub fn memblock_trim_memory(align: phys_addr_t);
    pub fn memblock_addrs_overlap(base1: phys_addr_t, size1: phys_addr_t, base2: phys_addr_t, size2: phys_addr_t) -> ::core::primitive::c_ulong;
    pub fn memblock_overlaps_region(ty: *mut memblock_type, base: phys_addr_t, size: phys_addr_t) -> bool;
    pub fn memblock_validate_numa_coverage(threshold_bytes: ::core::primitive::c_ulong) -> bool;
    pub fn memblock_mark_hotplug(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int;
    pub fn memblock_clear_hotplug(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int;
    pub fn memblock_mark_mirror(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int;
    pub fn memblock_mark_nomap(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int;
    pub fn memblock_clear_nomap(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int;
    pub fn memblock_reserved_mark_noinit(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int;
    pub fn memblock_reserved_mark_kern(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int;
    pub fn memblock_mark_kho_scratch(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int;
    pub fn memblock_clear_kho_scratch(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int;
    pub fn memblock_free(ptr: *mut ::core::ffi::c_void, size: usize);
    pub fn reset_all_zones_managed_pages();
    pub fn __next_mem_range(idx: *mut u64, nid: ::core::primitive::c_int, flags: memblock_flags, type_a: *mut memblock_type, type_b: *mut memblock_type, out_start: *mut phys_addr_t, out_end: *mut phys_addr_t, out_nid: *mut ::core::primitive::c_int);
    pub fn __next_mem_range_rev(idx: *mut u64, nid: ::core::primitive::c_int, flags: memblock_flags, type_a: *mut memblock_type, type_b: *mut memblock_type, out_start: *mut phys_addr_t, out_end: *mut phys_addr_t, out_nid: *mut ::core::primitive::c_int);
    pub fn memblock_search_pfn_nid(pfn: ::core::primitive::c_ulong, start_pfn: *mut ::core::primitive::c_ulong, end_pfn: *mut ::core::primitive::c_ulong) -> ::core::primitive::c_int;
    pub fn __next_mem_pfn_range(idx: *mut ::core::primitive::c_int, nid: ::core::primitive::c_int, out_start_pfn: *mut ::core::primitive::c_ulong, out_end_pfn: *mut ::core::primitive::c_ulong, out_nid: *mut ::core::primitive::c_int);
    pub fn memblock_set_node(base: phys_addr_t, size: phys_addr_t, ty: *mut memblock_type, nid: ::core::primitive::c_int) -> ::core::primitive::c_int;
}

#[cfg(CONFIG_HAVE_MEMBLOCK_PHYS_MAP)]
extern "C" {
    pub fn memblock_physmem_add(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int;
    pub static mut physmem: memblock_type;
}

#[cfg(CONFIG_NUMA)]
#[inline]
pub unsafe fn memblock_set_region_node(r: *mut memblock_region, nid: ::core::primitive::c_int) { (*r).nid = nid; }
#[cfg(CONFIG_NUMA)]
#[inline]
pub unsafe fn memblock_get_region_node(r: *const memblock_region) -> ::core::primitive::c_int { (*r).nid }
#[cfg(not(CONFIG_NUMA))]
#[inline]
pub unsafe fn memblock_set_region_node(_r: *mut memblock_region, _nid: ::core::primitive::c_int) {}
#[cfg(not(CONFIG_NUMA))]
#[inline]
pub unsafe fn memblock_get_region_node(_r: *const memblock_region) -> ::core::primitive::c_int { 0 }

#[cfg(CONFIG_NUMA)]
pub const HASHDIST_DEFAULT: bool = IS_ENABLED(CONFIG_64BIT);
#[cfg(CONFIG_NUMA)]
extern "C" { pub static mut hashdist: bool; }
#[cfg(not(CONFIG_NUMA))]
pub const hashdist: bool = false;

#[inline]
pub unsafe fn __next_physmem_range(idx: *mut u64, ty: *mut memblock_type, out_start: *mut phys_addr_t, out_end: *mut phys_addr_t) {
    __next_mem_range(idx, NUMA_NO_NODE, MEMBLOCK_NONE, &raw mut physmem, ty, out_start, out_end, core::ptr::null_mut());
}

// Source iteration macros, retained as direct macro wrappers for callers translated from C.
#[macro_export]
macro_rules! __for_each_mem_range { ($($args:tt)*) => { /* expands to __next_mem_range loop in the source */ }; }
#[macro_export]
macro_rules! __for_each_mem_range_rev { ($($args:tt)*) => { /* expands to __next_mem_range_rev loop in the source */ }; }
#[macro_export]
macro_rules! for_each_mem_range { ($($args:tt)*) => { $crate::__for_each_mem_range!($($args)*) }; }
#[macro_export]
macro_rules! for_each_mem_range_rev { ($($args:tt)*) => { $crate::__for_each_mem_range_rev!($($args)*) }; }
#[macro_export]
macro_rules! for_each_reserved_mem_range { ($($args:tt)*) => { $crate::__for_each_mem_range!($($args)*) }; }
#[macro_export]
macro_rules! for_each_free_mem_range { ($($args:tt)*) => { $crate::__for_each_mem_range!($($args)*) }; }
#[macro_export]
macro_rules! for_each_free_mem_range_reverse { ($($args:tt)*) => { $crate::__for_each_mem_range_rev!($($args)*) }; }
#[macro_export]
macro_rules! for_each_mem_pfn_range { ($($args:tt)*) => { /* expands to __next_mem_pfn_range loop in the source */ }; }
#[macro_export]
macro_rules! for_each_mem_region { ($($args:tt)*) => { /* pointer iteration over memblock.memory.regions */ }; }
#[macro_export]
macro_rules! for_each_reserved_mem_region { ($($args:tt)*) => { /* pointer iteration over memblock.reserved.regions */ }; }

#[inline(always)]
pub unsafe fn memblock_reserve(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int { __memblock_reserve(base, size, NUMA_NO_NODE, MEMBLOCK_NONE) }
#[inline(always)]
pub unsafe fn memblock_reserve_kern(base: phys_addr_t, size: phys_addr_t) -> ::core::primitive::c_int { __memblock_reserve(base, size, NUMA_NO_NODE, MEMBLOCK_RSRV_KERN) }

#[inline]
pub unsafe fn memblock_is_hotpluggable(m: *mut memblock_region) -> bool { ((*m).flags as u32 & MEMBLOCK_HOTPLUG as u32) != 0 }
#[inline]
pub unsafe fn memblock_is_mirror(m: *mut memblock_region) -> bool { ((*m).flags as u32 & MEMBLOCK_MIRROR as u32) != 0 }
#[inline]
pub unsafe fn memblock_is_nomap(m: *mut memblock_region) -> bool { ((*m).flags as u32 & MEMBLOCK_NOMAP as u32) != 0 }
#[inline]
pub unsafe fn memblock_is_reserved_noinit(m: *mut memblock_region) -> bool { ((*m).flags as u32 & MEMBLOCK_RSRV_NOINIT as u32) != 0 }
#[inline]
pub unsafe fn memblock_is_driver_managed(m: *mut memblock_region) -> bool { ((*m).flags as u32 & MEMBLOCK_DRIVER_MANAGED as u32) != 0 }
#[inline]
pub unsafe fn memblock_is_kho_scratch(m: *mut memblock_region) -> bool { ((*m).flags as u32 & MEMBLOCK_KHO_SCRATCH as u32) != 0 }

#[inline]
pub unsafe fn memblock_set_bottom_up(enable: bool) { memblock.bottom_up = enable; }
#[inline]
pub unsafe fn memblock_bottom_up() -> bool { memblock.bottom_up }

extern "C" {
    pub fn memblock_phys_mem_size() -> phys_addr_t;
    pub fn memblock_reserved_size() -> phys_addr_t;
    pub fn memblock_reserved_kern_size(limit: phys_addr_t, nid: ::core::primitive::c_int) -> phys_addr_t;
    pub fn memblock_reserved_hugetlb_size(limit: phys_addr_t, nid: ::core::primitive::c_int) -> phys_addr_t;
    pub fn memblock_estimated_nr_free_pages() -> ::core::primitive::c_ulong;
    pub fn memblock_start_of_DRAM() -> phys_addr_t;
    pub fn memblock_end_of_DRAM() -> phys_addr_t;
    pub fn memblock_enforce_memory_limit(memory_limit: phys_addr_t);
    pub fn memblock_cap_memory_range(base: phys_addr_t, size: phys_addr_t);
    pub fn memblock_mem_limit_remove_map(limit: phys_addr_t);
    pub fn memblock_is_memory(addr: phys_addr_t) -> bool;
    pub fn memblock_is_map_memory(addr: phys_addr_t) -> bool;
    pub fn memblock_is_region_memory(base: phys_addr_t, size: phys_addr_t) -> bool;
    pub fn memblock_is_reserved(addr: phys_addr_t) -> bool;
    pub fn memblock_is_region_reserved(base: phys_addr_t, size: phys_addr_t) -> bool;
    pub fn memblock_dump_all();
    pub fn memblock_set_current_limit(limit: phys_addr_t);
    pub fn memblock_get_current_limit() -> phys_addr_t;
}

pub const MEMBLOCK_ALLOC_ANYWHERE: phys_addr_t = !0 as phys_addr_t;
pub const MEMBLOCK_ALLOC_ACCESSIBLE: phys_addr_t = 0;
pub const MEMBLOCK_ALLOC_NOLEAKTRACE: phys_addr_t = 1;
pub const MEMBLOCK_LOW_LIMIT: phys_addr_t = 0;
pub const ARCH_LOW_ADDRESS_LIMIT: ::core::primitive::c_ulong = 0xffffffff;
pub const HASH_EARLY: ::core::primitive::c_int = 0x00000001;
pub const HASH_ZERO: ::core::primitive::c_int = 0x00000002;

extern "C" {
    pub fn memblock_phys_alloc_range(size: phys_addr_t, align: phys_addr_t, start: phys_addr_t, end: phys_addr_t) -> phys_addr_t;
    pub fn memblock_alloc_range_nid(size: phys_addr_t, align: phys_addr_t, start: phys_addr_t, end: phys_addr_t, nid: ::core::primitive::c_int, exact_nid: bool) -> phys_addr_t;
    pub fn memblock_phys_alloc_try_nid(size: phys_addr_t, align: phys_addr_t, nid: ::core::primitive::c_int) -> phys_addr_t;
    pub fn memblock_alloc_exact_nid_raw(size: phys_addr_t, align: phys_addr_t, min_addr: phys_addr_t, max_addr: phys_addr_t, nid: ::core::primitive::c_int) -> *mut ::core::ffi::c_void;
    pub fn memblock_alloc_try_nid_raw(size: phys_addr_t, align: phys_addr_t, min_addr: phys_addr_t, max_addr: phys_addr_t, nid: ::core::primitive::c_int) -> *mut ::core::ffi::c_void;
    pub fn memblock_alloc_try_nid(size: phys_addr_t, align: phys_addr_t, min_addr: phys_addr_t, max_addr: phys_addr_t, nid: ::core::primitive::c_int) -> *mut ::core::ffi::c_void;
    pub fn memblock_alloc_hugetlb(size: phys_addr_t, nid: ::core::primitive::c_int, exact_nid: bool) -> *mut ::core::ffi::c_void;
    pub fn __memblock_alloc_or_panic(size: phys_addr_t, align: phys_addr_t, func: *const ::core::primitive::c_char) -> *mut ::core::ffi::c_void;
    pub fn alloc_large_system_hash(tablename: *const ::core::primitive::c_char, bucketsize: ::core::primitive::c_ulong, numentries: ::core::primitive::c_ulong, scale: ::core::primitive::c_int, flags: ::core::primitive::c_int, hash_shift: *mut u32, hash_mask: *mut u32, low_limit: ::core::primitive::c_ulong, high_limit: ::core::primitive::c_ulong) -> *mut ::core::ffi::c_void;
}

#[inline(always)]
pub unsafe fn memblock_phys_alloc(size: phys_addr_t, align: phys_addr_t) -> phys_addr_t { memblock_phys_alloc_range(size, align, 0, MEMBLOCK_ALLOC_ACCESSIBLE) }
#[inline(always)]
pub unsafe fn memblock_alloc(size: phys_addr_t, align: phys_addr_t) -> *mut ::core::ffi::c_void { memblock_alloc_try_nid(size, align, MEMBLOCK_LOW_LIMIT, MEMBLOCK_ALLOC_ACCESSIBLE, NUMA_NO_NODE) }
#[inline]
pub unsafe fn memblock_alloc_raw(size: phys_addr_t, align: phys_addr_t) -> *mut ::core::ffi::c_void { memblock_alloc_try_nid_raw(size, align, MEMBLOCK_LOW_LIMIT, MEMBLOCK_ALLOC_ACCESSIBLE, NUMA_NO_NODE) }
#[inline(always)]
pub unsafe fn memblock_alloc_from(size: phys_addr_t, align: phys_addr_t, min_addr: phys_addr_t) -> *mut ::core::ffi::c_void { memblock_alloc_try_nid(size, align, min_addr, MEMBLOCK_ALLOC_ACCESSIBLE, NUMA_NO_NODE) }
#[inline]
pub unsafe fn memblock_alloc_low(size: phys_addr_t, align: phys_addr_t) -> *mut ::core::ffi::c_void { memblock_alloc_try_nid(size, align, MEMBLOCK_LOW_LIMIT, ARCH_LOW_ADDRESS_LIMIT as phys_addr_t, NUMA_NO_NODE) }
#[inline]
pub unsafe fn memblock_alloc_node(size: phys_addr_t, align: phys_addr_t, nid: ::core::primitive::c_int) -> *mut ::core::ffi::c_void { memblock_alloc_try_nid(size, align, MEMBLOCK_LOW_LIMIT, MEMBLOCK_ALLOC_ACCESSIBLE, nid) }

#[inline]
pub unsafe fn memblock_region_memory_base_pfn(reg: *const memblock_region) -> ::core::primitive::c_ulong { PFN_UP((*reg).base) }
#[inline]
pub unsafe fn memblock_region_memory_end_pfn(reg: *const memblock_region) -> ::core::primitive::c_ulong { PFN_DOWN((*reg).base + (*reg).size) }
#[inline]
pub unsafe fn memblock_region_reserved_base_pfn(reg: *const memblock_region) -> ::core::primitive::c_ulong { PFN_DOWN((*reg).base) }
#[inline]
pub unsafe fn memblock_region_reserved_end_pfn(reg: *const memblock_region) -> ::core::primitive::c_ulong { PFN_UP((*reg).base + (*reg).size) }

// C iteration macros are preserved as Rust macro_rules! wrappers around the same low-level calls.
#[macro_export]
macro_rules! memblock_alloc_or_panic { ($size:expr, $align:expr) => { unsafe { $crate::__memblock_alloc_or_panic($size, $align, concat!(module_path!(), "\0").as_ptr() as *const ::core::primitive::c_char) } }; }

#[cfg(CONFIG_MEMTEST)]
extern "C" { pub fn early_memtest(start: phys_addr_t, end: phys_addr_t); pub fn memtest_report_meminfo(m: *mut seq_file); }
#[cfg(not(CONFIG_MEMTEST))]
#[inline] pub unsafe fn early_memtest(_start: phys_addr_t, _end: phys_addr_t) {}
#[cfg(not(CONFIG_MEMTEST))]
#[inline] pub unsafe fn memtest_report_meminfo(_m: *mut seq_file) {}

#[cfg(CONFIG_MEMBLOCK_KHO_SCRATCH)]
extern "C" { pub fn memblock_set_kho_scratch_only(); pub fn memblock_clear_kho_scratch_only(); }
#[cfg(not(CONFIG_MEMBLOCK_KHO_SCRATCH))]
#[inline] pub unsafe fn memblock_set_kho_scratch_only() {}
#[cfg(not(CONFIG_MEMBLOCK_KHO_SCRATCH))]
#[inline] pub unsafe fn memblock_clear_kho_scratch_only() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
