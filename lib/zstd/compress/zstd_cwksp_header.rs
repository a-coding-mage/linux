/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/* Translation of zstd_cwksp.h. Dependencies are supplied by the surrounding crate. */

pub const ZSTD_CWKSP_ASAN_REDZONE_SIZE: usize = 128;
pub const ZSTD_CWKSP_ALIGNMENT_BYTES: usize = 64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ZSTD_cwksp_alloc_phase_e {
    ZSTD_cwksp_alloc_objects,
    ZSTD_cwksp_alloc_aligned_init_once,
    ZSTD_cwksp_alloc_aligned,
    ZSTD_cwksp_alloc_buffers,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ZSTD_cwksp_static_alloc_e {
    ZSTD_cwksp_dynamic_alloc,
    ZSTD_cwksp_static_alloc,
}

#[repr(C)]
pub struct ZSTD_cwksp {
    pub workspace: *mut core::ffi::c_void,
    pub workspaceEnd: *mut core::ffi::c_void,
    pub objectEnd: *mut core::ffi::c_void,
    pub tableEnd: *mut core::ffi::c_void,
    pub tableValidEnd: *mut core::ffi::c_void,
    pub allocStart: *mut core::ffi::c_void,
    pub initOnceStart: *mut core::ffi::c_void,
    pub allocFailed: u8,
    pub workspaceOversizedDuration: i32,
    pub phase: ZSTD_cwksp_alloc_phase_e,
    pub isStatic: ZSTD_cwksp_static_alloc_e,
}

extern "C" {
    pub fn ZSTD_cwksp_create(ws: *mut ZSTD_cwksp, size: usize, customMem: ZSTD_customMem) -> usize;
    pub fn ZSTD_cwksp_free(ws: *mut ZSTD_cwksp, customMem: ZSTD_customMem);
    pub fn ZSTD_cwksp_move(dst: *mut ZSTD_cwksp, src: *mut ZSTD_cwksp);
    pub fn ZSTD_cwksp_available_space(ws: *mut ZSTD_cwksp) -> usize;
    pub fn ZSTD_cwksp_initialAllocStart(ws: *mut ZSTD_cwksp) -> *mut core::ffi::c_void;
}

/* Supplied by ../common/allocations.h. */
#[repr(C)] pub struct ZSTD_customMem { _private: [u8; 0] }

#[inline]
pub unsafe fn ZSTD_cwksp_assert_internal_consistency(ws: *mut ZSTD_cwksp) {
    debug_assert!((*ws).workspace <= (*ws).objectEnd);
    debug_assert!((*ws).objectEnd <= (*ws).tableEnd);
    debug_assert!((*ws).objectEnd <= (*ws).tableValidEnd);
    debug_assert!((*ws).tableEnd <= (*ws).allocStart);
    debug_assert!((*ws).tableValidEnd <= (*ws).allocStart);
    debug_assert!((*ws).allocStart <= (*ws).workspaceEnd);
    debug_assert!((*ws).initOnceStart <= ZSTD_cwksp_initialAllocStart(ws));
    debug_assert!((*ws).workspace <= (*ws).initOnceStart);
}

#[inline] pub fn ZSTD_cwksp_align(size: usize, align: usize) -> usize {
    let mask = align - 1; debug_assert!(align.is_power_of_two());
    size.wrapping_add(mask) & !mask
}
#[inline] pub fn ZSTD_cwksp_alloc_size(size: usize) -> usize { if size == 0 { 0 } else { size } }
#[inline] pub fn ZSTD_cwksp_aligned_alloc_size(size: usize, alignment: usize) -> usize { ZSTD_cwksp_alloc_size(ZSTD_cwksp_align(size, alignment)) }
#[inline] pub fn ZSTD_cwksp_aligned64_alloc_size(size: usize) -> usize { ZSTD_cwksp_aligned_alloc_size(size, ZSTD_CWKSP_ALIGNMENT_BYTES) }
#[inline] pub fn ZSTD_cwksp_slack_space_required() -> usize { ZSTD_CWKSP_ALIGNMENT_BYTES * 2 }

#[inline]
pub unsafe fn ZSTD_cwksp_bytes_to_align_ptr(ptr: *mut core::ffi::c_void, alignBytes: usize) -> usize {
    let mask = alignBytes - 1;
    let bytes = (alignBytes - (ptr as usize & mask)) & mask;
    debug_assert!(alignBytes.is_power_of_two()); debug_assert!(bytes < alignBytes); bytes
}

#[inline]
pub unsafe fn ZSTD_cwksp_initialAllocStart_local(ws: *mut ZSTD_cwksp) -> *mut core::ffi::c_void {
    let mut p = (*ws).workspaceEnd as *mut u8;
    debug_assert!(ZSTD_CWKSP_ALIGNMENT_BYTES.is_power_of_two());
    p = p.sub(p as usize % ZSTD_CWKSP_ALIGNMENT_BYTES); p as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn ZSTD_cwksp_owns_buffer(ws: *const ZSTD_cwksp, ptr: *const core::ffi::c_void) -> i32 {
    if !ptr.is_null() && (*ws).workspace <= ptr as *mut _ && ptr as *mut _ < (*ws).workspaceEnd { 1 } else { 0 }
}

/* The remaining inline operations retain C pointer arithmetic and external zstd macro dependencies. */
#[inline] pub unsafe fn ZSTD_cwksp_sizeof(ws: *const ZSTD_cwksp) -> usize { (*ws).workspaceEnd as usize - (*ws).workspace as usize }
#[inline] pub unsafe fn ZSTD_cwksp_used(ws: *const ZSTD_cwksp) -> usize { ((*ws).tableEnd as usize - (*ws).workspace as usize) + ((*ws).workspaceEnd as usize - (*ws).allocStart as usize) }
#[inline] pub unsafe fn ZSTD_cwksp_available_space_local(ws: *mut ZSTD_cwksp) -> usize { (*ws).allocStart as usize - (*ws).tableEnd as usize }

/* Full declarations below preserve the header's externally supplied implementations/macros. */
extern "C" {
    pub fn ZSTD_cwksp_reserve_buffer(ws: *mut ZSTD_cwksp, bytes: usize) -> *mut u8;
    pub fn ZSTD_cwksp_reserve_aligned_init_once(ws: *mut ZSTD_cwksp, bytes: usize) -> *mut core::ffi::c_void;
    pub fn ZSTD_cwksp_reserve_aligned64(ws: *mut ZSTD_cwksp, bytes: usize) -> *mut core::ffi::c_void;
    pub fn ZSTD_cwksp_reserve_table(ws: *mut ZSTD_cwksp, bytes: usize) -> *mut core::ffi::c_void;
    pub fn ZSTD_cwksp_reserve_object(ws: *mut ZSTD_cwksp, bytes: usize) -> *mut core::ffi::c_void;
    pub fn ZSTD_cwksp_reserve_object_aligned(ws: *mut ZSTD_cwksp, byteSize: usize, alignment: usize) -> *mut core::ffi::c_void;
    pub fn ZSTD_cwksp_mark_tables_dirty(ws: *mut ZSTD_cwksp);
    pub fn ZSTD_cwksp_mark_tables_clean(ws: *mut ZSTD_cwksp);
    pub fn ZSTD_cwksp_clean_tables(ws: *mut ZSTD_cwksp);
    pub fn ZSTD_cwksp_clear_tables(ws: *mut ZSTD_cwksp);
    pub fn ZSTD_cwksp_clear(ws: *mut ZSTD_cwksp);
    pub fn ZSTD_cwksp_init(ws: *mut ZSTD_cwksp, start: *mut core::ffi::c_void, size: usize, isStatic: ZSTD_cwksp_static_alloc_e);
    pub fn ZSTD_cwksp_reserve_failed(ws: *const ZSTD_cwksp) -> i32;
    pub fn ZSTD_cwksp_check_available(ws: *mut ZSTD_cwksp, additionalNeededSpace: usize) -> i32;
    pub fn ZSTD_cwksp_check_too_large(ws: *mut ZSTD_cwksp, additionalNeededSpace: usize) -> i32;
    pub fn ZSTD_cwksp_check_wasteful(ws: *mut ZSTD_cwksp, additionalNeededSpace: usize) -> i32;
    pub fn ZSTD_cwksp_bump_oversized_duration(ws: *mut ZSTD_cwksp, additionalNeededSpace: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
