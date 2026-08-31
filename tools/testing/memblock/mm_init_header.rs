/* SPDX-License-Identifier: GPL-2.0-or-later */

#[no_mangle]
pub unsafe extern "C" fn memblock_free_pages(
    pfn: core::ffi::c_ulong,
    order: core::ffi::c_uint,
) {
}

#[inline]
pub unsafe fn accept_memory(start: phys_addr_t, size: core::ffi::c_ulong) {
}

unsafe extern "C" {
    pub fn free_reserved_area(
        start: *mut core::ffi::c_void,
        end: *mut core::ffi::c_void,
        poison: core::ffi::c_int,
        s: *const core::ffi::c_char,
    ) -> core::ffi::c_ulong;
    pub fn free_reserved_page(page: *mut page);
}

#[inline]
pub fn deferred_pages_enabled() -> bool {
    false
}

#[inline]
pub unsafe fn init_deferred_page(pfn: core::ffi::c_ulong, nid: core::ffi::c_int) {
}
