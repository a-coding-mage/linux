/* SPDX-License-Identifier: GPL-2.0 */

// Declarations and constants supplied by the surrounding io_uring/kernel code
// are intentionally referenced here rather than redefined.

pub const IORING_MAP_OFF_PARAM_REGION: u64 = 0x2000_0000;
pub const IORING_MAP_OFF_ZCRX_REGION: u64 = 0x3000_0000;

pub const IORING_OFF_ZCRX_SHIFT: u32 = 16;

extern "C" {
    pub fn io_pin_pages(
        uaddr: ::core::ffi::c_ulong,
        len: ::core::ffi::c_ulong,
        npages: *mut ::core::ffi::c_int,
    ) -> *mut *mut page;

    // CONFIG_MMU is a build-time condition from the C header.
    #[cfg(not(CONFIG_MMU))]
    pub fn io_uring_nommu_mmap_capabilities(file: *mut file) -> ::core::ffi::c_uint;

    pub fn io_uring_get_unmapped_area(
        file: *mut file,
        addr: ::core::ffi::c_ulong,
        len: ::core::ffi::c_ulong,
        pgoff: ::core::ffi::c_ulong,
        flags: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong;

    pub fn io_uring_mmap(file: *mut file, vma: *mut vm_area_struct) -> ::core::ffi::c_int;

    pub fn io_free_region(user: *mut user_struct, mr: *mut io_mapped_region);
    pub fn io_create_region(
        ctx: *mut io_ring_ctx,
        mr: *mut io_mapped_region,
        reg: *mut io_uring_region_desc,
        mmap_offset: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
}

// Opaque types supplied by other headers.
pub enum page {}
pub enum file {}
pub enum vm_area_struct {}
pub enum user_struct {}
pub enum io_ring_ctx {}
#[repr(C)]
pub struct io_mapped_region {
    pub ptr: *mut ::core::ffi::c_void,
    pub nr_pages: ::core::ffi::c_ulong,
}
pub enum io_uring_region_desc {}

#[inline]
pub unsafe fn io_region_get_ptr(mr: *mut io_mapped_region) -> *mut ::core::ffi::c_void {
    (*mr).ptr
}

#[inline]
pub unsafe fn io_region_is_set(mr: *mut io_mapped_region) -> bool {
    (*mr).nr_pages != 0
}

#[inline]
pub unsafe fn io_region_publish(
    ctx: *mut io_ring_ctx,
    src_region: *mut io_mapped_region,
    dst_region: *mut io_mapped_region,
) {
    /*
     * Once published mmap can find it without holding only the ->mmap_lock
     * and not ->uring_lock.
     */
    // C: guard(mutex)(&ctx->mmap_lock);
    (*dst_region) = (*src_region);
}

#[inline]
pub unsafe fn io_region_size(mr: *mut io_mapped_region) -> usize {
    ((*mr).nr_pages as usize) << PAGE_SHIFT
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
