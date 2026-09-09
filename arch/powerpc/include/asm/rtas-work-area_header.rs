/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from the C header.  Linux build-time dependencies and
// configuration conditions are supplied by the surrounding translation.

#[repr(C)]
pub struct rtas_work_area {
    /* private: Use the APIs provided below. */
    pub buf: *mut core::ffi::c_char,
    pub size: usize,
}

pub const RTAS_WORK_AREA_MAX_ALLOC_SZ: usize = 128 * 1024;

/*
 * rtas_work_area_alloc() - Acquire a work area of the requested size.
 * @size_: Allocation size. Must be compile-time constant and not more
 *         than %RTAS_WORK_AREA_MAX_ALLOC_SZ.
 *
 * Allocate a buffer suitable for passing to RTAS functions that have
 * a memory address parameter, often (but not always) referred to as a
 * "work area" in PAPR. Although callers are allowed to block while
 * holding a work area, the amount of memory reserved for this purpose
 * is limited, and allocations should be short-lived. A good guideline
 * is to release any allocated work area before returning from a
 * system call.
 *
 * This function does not fail. It blocks until the allocation
 * succeeds. To prevent deadlocks, callers are discouraged from
 * allocating more than one work area simultaneously in a single task
 * context.
 *
 * Context: This function may sleep.
 * Return: A &struct rtas_work_area descriptor for the allocated work area.
 */
#[macro_export]
macro_rules! rtas_work_area_alloc {
    ($size:expr) => {{
        const _: () = assert!($size > 0);
        const _: () = assert!($size <= $crate::RTAS_WORK_AREA_MAX_ALLOC_SZ);
        unsafe { $crate::__rtas_work_area_alloc($size) }
    }};
}

/* Do not call __rtas_work_area_alloc() directly. Use rtas_work_area_alloc(). */
extern "C" {
    pub fn __rtas_work_area_alloc(size: usize) -> *mut rtas_work_area;
    pub fn rtas_work_area_free(area: *mut rtas_work_area);
}

pub unsafe fn rtas_work_area_raw_buf(
    area: *const rtas_work_area,
) -> *mut core::ffi::c_char {
    (*area).buf
}

pub unsafe fn rtas_work_area_size(area: *const rtas_work_area) -> usize {
    (*area).size
}

pub unsafe fn rtas_work_area_phys(area: *const rtas_work_area) -> phys_addr_t {
    __pa((*area).buf)
}

/*
 * Early setup for the work area allocator. Call from
 * rtas_initialize() only.
 */

#[cfg(feature = "CONFIG_PPC_PSERIES")]
extern "C" {
    pub fn rtas_work_area_reserve_arena(limit: phys_addr_t);
}

#[cfg(not(feature = "CONFIG_PPC_PSERIES"))]
#[inline]
pub fn rtas_work_area_reserve_arena(_limit: phys_addr_t) {}

// Supplied by the surrounding PowerPC translation.
extern "C" {
    fn __pa(addr: *const core::ffi::c_char) -> phys_addr_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
