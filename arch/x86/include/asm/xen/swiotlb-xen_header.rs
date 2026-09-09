/* SPDX-License-Identifier: GPL-2.0 */

// Translated from _ASM_X86_SWIOTLB_XEN_H.

extern "C" {
    pub fn xen_swiotlb_fixup(buf: *mut core::ffi::c_void, nslabs: core::ffi::c_ulong) -> core::ffi::c_int;
    pub fn xen_create_contiguous_region(
        pstart: phys_addr_t,
        order: core::ffi::c_uint,
        address_bits: core::ffi::c_uint,
        dma_handle: *mut dma_addr_t,
    ) -> core::ffi::c_int;
    pub fn xen_destroy_contiguous_region(pstart: phys_addr_t, order: core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
