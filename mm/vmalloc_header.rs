/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * mm-internal APIs for vmalloc
 */

// Dependency intent: declarations from <linux/vmalloc.h> are supplied by the
// surrounding translation unit.

#[cfg(feature = "CONFIG_MMU")]
extern "C" {
    pub fn vmalloc_init();

    pub fn vmap_pages_range_noflush(
        addr: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
        prot: pgprot_t,
        pages: *mut *mut page,
        page_shift: ::core::ffi::c_uint,
        gfp_mask: gfp_t,
    ) -> ::core::ffi::c_int;

    pub fn get_vm_area_page_order(vm: *mut vm_struct) -> ::core::ffi::c_uint;
}

#[cfg(not(feature = "CONFIG_MMU"))]
#[inline]
pub fn vmalloc_init() {}

#[cfg(not(feature = "CONFIG_MMU"))]
#[inline]
pub unsafe fn vmap_pages_range_noflush(
    _addr: ::core::ffi::c_ulong,
    _end: ::core::ffi::c_ulong,
    _prot: pgprot_t,
    _pages: *mut *mut page,
    _page_shift: ::core::ffi::c_uint,
    _gfp_mask: gfp_t,
) -> ::core::ffi::c_int {
    // -EINVAL
    -22
}

#[cfg(not(feature = "CONFIG_MMU"))]
#[inline]
pub unsafe fn vunmap_range_noflush(
    _start: ::core::ffi::c_ulong,
    _end: ::core::ffi::c_ulong,
) {
}

extern "C" {
    pub fn __get_vm_area_node(
        size: ::core::ffi::c_ulong,
        align: ::core::ffi::c_ulong,
        shift: ::core::ffi::c_ulong,
        vm_flags: ::core::ffi::c_ulong,
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
        node: ::core::ffi::c_int,
        gfp_mask: gfp_t,
        caller: *const ::core::ffi::c_void,
    ) -> *mut vm_struct;

    pub fn clear_vm_uninitialized_flag(vm: *mut vm_struct);

    pub fn __vmap_pages_range_noflush(
        addr: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
        prot: pgprot_t,
        pages: *mut *mut page,
        page_shift: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn vunmap_range_noflush(
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );

    pub fn __vunmap_range_noflush(
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
