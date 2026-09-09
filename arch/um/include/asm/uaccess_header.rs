/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002 Jeff Dike (jdike@karaya.com)
 * Copyright (C) 2015 Richard Weinberger (richard@nod.at)
 */

// C header dependencies: asm/elf.h, linux/unaligned.h, sysdep/faultinfo.h,
// and asm-generic/uaccess.h supply the referenced symbols.

#[inline]
pub fn __under_task_size(addr: usize, size: usize) -> bool {
    (addr < TASK_SIZE) && (addr.wrapping_add(size) < TASK_SIZE)
}

#[inline]
pub fn __addr_range_nowrap(addr: usize, size: usize) -> bool {
    addr <= addr.wrapping_add(size)
}

extern "C" {
    pub fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    pub fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    pub fn __clear_user(mem: *mut core::ffi::c_void, len: usize) -> usize;
}

#[inline]
pub unsafe fn __access_ok(ptr: *const core::ffi::c_void, size: usize) -> i32 {
    let addr = ptr as usize;
    if __addr_range_nowrap(addr, size) && __under_task_size(addr, size) {
        1
    } else {
        0
    }
}

// The original header defines INLINE_COPY_USER and aliases __access_ok and
// __clear_user for asm-generic/uaccess.h.
pub const INLINE_COPY_USER: () = ();

// C macro: __get_kernel_nofault(dst, src, type, err_label)
//
// Preserves the original fault handling, zeroing, unaligned read, barrier,
// and error-label control flow.
#[macro_export]
macro_rules! __get_kernel_nofault {
    ($dst:expr, $src:expr, $type:ty, $err_label:lifetime) => {{
        let mut __faulted: i32;
        ___backtrack_faulted!(__faulted);
        if __faulted != 0 {
            unsafe { *($dst as *mut $type) = 0 as $type; }
            break $err_label;
        }
        unsafe {
            *($dst as *mut $type) = get_unaligned($src as *const $type);
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        unsafe { (*current).thread.segv_continue = core::ptr::null_mut(); }
    }};
}

// C macro: __put_kernel_nofault(dst, src, type, err_label)
#[macro_export]
macro_rules! __put_kernel_nofault {
    ($dst:expr, $src:expr, $type:ty, $err_label:lifetime) => {{
        let mut __faulted: i32;
        ___backtrack_faulted!(__faulted);
        if __faulted != 0 {
            break $err_label;
        }
        unsafe {
            put_unaligned(*($src as *const $type), $dst as *mut $type);
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        unsafe { (*current).thread.segv_continue = core::ptr::null_mut(); }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
