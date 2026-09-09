/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/map.h
 *
 *  Copyright (C) 1999-2000 Russell King
 *
 *  Page table mapping constructs and function prototypes
 */

// Dependency supplied by asm/io.h in the original source.

#[repr(C)]
pub struct map_desc {
    pub virtual_: ::core::ffi::c_ulong,
    pub pfn: ::core::ffi::c_ulong,
    pub length: ::core::ffi::c_ulong,
    pub type_: ::core::ffi::c_uint,
}

/* types 0-3 are defined in asm/io.h */
pub const MT_UNCACHED: ::core::ffi::c_uint = 4;
pub const MT_CACHECLEAN: ::core::ffi::c_uint = 5;
pub const MT_MINICLEAN: ::core::ffi::c_uint = 6;
pub const MT_LOW_VECTORS: ::core::ffi::c_uint = 7;
pub const MT_HIGH_VECTORS: ::core::ffi::c_uint = 8;
pub const MT_MEMORY_RWX: ::core::ffi::c_uint = 9;
pub const MT_MEMORY_RW: ::core::ffi::c_uint = 10;
pub const MT_MEMORY_RO: ::core::ffi::c_uint = 11;
pub const MT_ROM: ::core::ffi::c_uint = 12;
pub const MT_MEMORY_RWX_NONCACHED: ::core::ffi::c_uint = 13;
pub const MT_MEMORY_RW_DTCM: ::core::ffi::c_uint = 14;
pub const MT_MEMORY_RWX_ITCM: ::core::ffi::c_uint = 15;
pub const MT_MEMORY_RW_SO: ::core::ffi::c_uint = 16;
pub const MT_MEMORY_DMA_READY: ::core::ffi::c_uint = 17;

/* CONFIG_MMU declarations from the original conditional section. */
#[cfg(feature = "CONFIG_MMU")]
unsafe extern "C" {
    pub fn iotable_init(map: *mut map_desc, num: ::core::ffi::c_int);
    pub fn vm_reserve_area_early(
        addr: ::core::ffi::c_ulong,
        size: ::core::ffi::c_ulong,
        caller: *mut ::core::ffi::c_void,
    );
    pub fn create_mapping_late(mm: *mut mm_struct, md: *mut map_desc, ng: bool);

    #[cfg(feature = "CONFIG_DEBUG_LL")]
    pub fn debug_ll_addr(
        paddr: *mut ::core::ffi::c_ulong,
        vaddr: *mut ::core::ffi::c_ulong,
    );
    #[cfg(feature = "CONFIG_DEBUG_LL")]
    pub fn debug_ll_io_init();

    pub fn get_mem_type(type_: ::core::ffi::c_uint) -> *const mem_type;
    /* external interface to remap single page with appropriate type */
    pub fn ioremap_page(
        virt: ::core::ffi::c_ulong,
        phys: ::core::ffi::c_ulong,
        mtype: *const mem_type,
    ) -> ::core::ffi::c_int;
}

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_DEBUG_LL")))]
#[inline]
pub fn debug_ll_io_init() {}

/* Forward declarations supplied by other translated headers. */
#[cfg(feature = "CONFIG_MMU")]
pub enum mm_struct {}
#[cfg(feature = "CONFIG_MMU")]
pub enum mem_type {}

/* CONFIG_MMU-disabled macros are intentionally no-ops. */
#[cfg(not(feature = "CONFIG_MMU"))]
#[inline]
pub unsafe fn iotable_init(_map: *mut map_desc, _num: ::core::ffi::c_int) {}

#[cfg(not(feature = "CONFIG_MMU"))]
#[inline]
pub unsafe fn vm_reserve_area_early(
    _addr: ::core::ffi::c_ulong,
    _size: ::core::ffi::c_ulong,
    _caller: *mut ::core::ffi::c_void,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
