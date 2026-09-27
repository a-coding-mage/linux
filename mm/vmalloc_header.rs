/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * mm-internal APIs for vmalloc
 */

// Dependency intent: declarations from <linux/vmalloc.h> are supplied by the
// surrounding translation unit.

#[cfg(CONFIG_MMU)]
extern "C" {
    pub fn vmalloc_init();

    pub fn vmap_pages_range_noflush(
        addr: ::kernel::ffi::c_ulong,
        end: ::kernel::ffi::c_ulong,
        prot: pgprot_t,
        pages: *mut *mut page,
        page_shift: ::kernel::ffi::c_uint,
        gfp_mask: gfp_t,
    ) -> ::kernel::ffi::c_int;

    pub fn get_vm_area_page_order(vm: *mut vm_struct) -> ::kernel::ffi::c_uint;
}

#[cfg(not(CONFIG_MMU))]
#[inline]
pub fn vmalloc_init() {}

#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn vmap_pages_range_noflush(
    _addr: ::kernel::ffi::c_ulong,
    _end: ::kernel::ffi::c_ulong,
    _prot: pgprot_t,
    _pages: *mut *mut page,
    _page_shift: ::kernel::ffi::c_uint,
    _gfp_mask: gfp_t,
) -> ::kernel::ffi::c_int {
    // -EINVAL
    -22
}

#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn vunmap_range_noflush(
    _start: ::kernel::ffi::c_ulong,
    _end: ::kernel::ffi::c_ulong,
) {
}

extern "C" {
    pub fn __get_vm_area_node(
        size: ::kernel::ffi::c_ulong,
        align: ::kernel::ffi::c_ulong,
        shift: ::kernel::ffi::c_ulong,
        vm_flags: ::kernel::ffi::c_ulong,
        start: ::kernel::ffi::c_ulong,
        end: ::kernel::ffi::c_ulong,
        node: ::kernel::ffi::c_int,
        gfp_mask: gfp_t,
        caller: *const ::kernel::ffi::c_void,
    ) -> *mut vm_struct;

    pub fn clear_vm_uninitialized_flag(vm: *mut vm_struct);

    pub fn __vmap_pages_range_noflush(
        addr: ::kernel::ffi::c_ulong,
        end: ::kernel::ffi::c_ulong,
        prot: pgprot_t,
        pages: *mut *mut page,
        page_shift: ::kernel::ffi::c_uint,
    ) -> ::kernel::ffi::c_int;

    pub fn vunmap_range_noflush(
        start: ::kernel::ffi::c_ulong,
        end: ::kernel::ffi::c_ulong,
    );

    pub fn __vunmap_range_noflush(
        start: ::kernel::ffi::c_ulong,
        end: ::kernel::ffi::c_ulong,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
