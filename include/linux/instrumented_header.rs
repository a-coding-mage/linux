/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This header provides generic wrappers for memory access instrumentation that
 * the compiler cannot emit for: KASAN, KCSAN, KMSAN.
 */

use core::ffi::c_void;

extern "C" {
    fn kasan_check_read(v: *const c_void, size: usize);
    fn kasan_check_write(v: *const c_void, size: usize);
    fn kcsan_check_read(v: *const c_void, size: usize);
    fn kcsan_check_write(v: *const c_void, size: usize);
    fn kcsan_check_read_write(v: *const c_void, size: usize);
    fn kcsan_check_atomic_read(v: *const c_void, size: usize);
    fn kcsan_check_atomic_write(v: *const c_void, size: usize);
    fn kcsan_check_atomic_read_write(v: *const c_void, size: usize);
    fn kmsan_copy_to_user(to: *mut c_void, from: *const c_void, n: usize, left: usize);
    fn kmsan_unpoison_memory(v: *const c_void, size: usize);
    fn kmsan_memmove(to: *mut c_void, from: *const c_void, n: usize);
}

/// Instrument a regular read access.
#[inline(always)]
pub unsafe fn instrument_read(v: *const c_void, size: usize) {
    kasan_check_read(v, size);
    kcsan_check_read(v, size);
}

/// Instrument a regular write access.
#[inline(always)]
pub unsafe fn instrument_write(v: *const c_void, size: usize) {
    kasan_check_write(v, size);
    kcsan_check_write(v, size);
}

/// Instrument a regular read-write access.
#[inline(always)]
pub unsafe fn instrument_read_write(v: *const c_void, size: usize) {
    kasan_check_write(v, size);
    kcsan_check_read_write(v, size);
}

#[inline(always)]
pub unsafe fn instrument_atomic_check_alignment(v: *const c_void, size: usize) {
    // The C implementation performs this block only when exports are enabled
    // and CONFIG_DEBUG_ATOMIC is enabled.
    #[cfg(feature = "debug_atomic")]
    {
        let mut mask = size.wrapping_sub(1);
        #[cfg(feature = "debug_atomic_largest_align")]
        {
            mask &= core::mem::align_of::<isize>().wrapping_sub(1);
        }
        // Equivalent to WARN_ON_ONCE((unsigned long)v & mask).
        let _ = (v as usize) & mask;
    }
}

/// Instrument an atomic read access.
#[inline(always)]
pub unsafe fn instrument_atomic_read(v: *const c_void, size: usize) {
    kasan_check_read(v, size);
    kcsan_check_atomic_read(v, size);
    instrument_atomic_check_alignment(v, size);
}

/// Instrument an atomic write access.
#[inline(always)]
pub unsafe fn instrument_atomic_write(v: *const c_void, size: usize) {
    kasan_check_write(v, size);
    kcsan_check_atomic_write(v, size);
    instrument_atomic_check_alignment(v, size);
}

/// Instrument an atomic read-write access.
#[inline(always)]
pub unsafe fn instrument_atomic_read_write(v: *const c_void, size: usize) {
    kasan_check_write(v, size);
    kcsan_check_atomic_read_write(v, size);
    instrument_atomic_check_alignment(v, size);
}

/// Instrument reads from kernel memory due to copy_to_user.
#[inline(always)]
pub unsafe fn instrument_copy_to_user(to: *mut c_void, from: *const c_void, n: usize) {
    kasan_check_read(from, n);
    kcsan_check_read(from, n);
    kmsan_copy_to_user(to, from, n, 0);
}

/// Instrument writes to kernel memory before copy_from_user.
#[inline(always)]
pub unsafe fn instrument_copy_from_user_before(
    to: *const c_void,
    _from: *const c_void,
    n: usize,
) {
    kasan_check_write(to, n);
    kcsan_check_write(to, n);
}

/// Instrument writes to kernel memory after copy_from_user.
#[inline(always)]
pub unsafe fn instrument_copy_from_user_after(
    to: *const c_void,
    _from: *const c_void,
    n: usize,
    left: usize,
) {
    kmsan_unpoison_memory(to, n.wrapping_sub(left));
}

/// Instrument memory accesses before a non-instrumented memcpy.
#[inline(always)]
pub unsafe fn instrument_memcpy_before(to: *mut c_void, from: *const c_void, n: usize) {
    kasan_check_write(to as *const c_void, n);
    kasan_check_read(from, n);
    kcsan_check_write(to as *const c_void, n);
    kcsan_check_read(from, n);
}

/// Instrument memory accesses after a non-instrumented memcpy.
#[inline(always)]
pub unsafe fn instrument_memcpy_after(
    to: *mut c_void,
    from: *const c_void,
    n: usize,
    left: usize,
) {
    kmsan_memmove(to, from, n.wrapping_sub(left));
}

/// Add instrumentation to get_user()-like operations.
#[macro_export]
macro_rules! instrument_get_user {
    ($to:expr) => {{
        let mut __tmp: u64 = ($to) as u64;
        unsafe {
            $crate::kmsan_unpoison_memory(
                (&mut __tmp as *mut u64).cast::<core::ffi::c_void>(),
                core::mem::size_of::<u64>(),
            );
        }
        $to = __tmp;
    }};
}

/// Add instrumentation to put_user()-like operations.
#[macro_export]
macro_rules! instrument_put_user {
    ($from:expr, $ptr:expr, $size:expr) => {{
        unsafe {
            $crate::kmsan_copy_to_user(
                ($ptr).cast::<core::ffi::c_void>(),
                (&$from as *const _).cast::<core::ffi::c_void>(),
                core::mem::size_of_val(&$from),
                0,
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
