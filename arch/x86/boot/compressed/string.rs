// SPDX-License-Identifier: GPL-2.0
/*
 * This provides an optimized implementation of memcpy, and a simplified
 * implementation of memset and memmove. These are used here because the
 * standard kernel runtime versions are not yet available and we don't
 * trust the gcc built-in implementations as they may do unexpected things
 * (e.g. FPU ops) in the minimal decompression stub execution environment.
 */
// Dependency supplied by the surrounding kernel sources: error.h
// The implementation from ../string.c is likewise supplied externally.

#[cfg(target_pointer_width = "32")]
unsafe fn ____memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void {
    let d = dest as *mut u8;
    let s = src as *const u8;

    // C uses `rep movsl` followed by `rep movsb` here.
    let words = n >> 2;
    for i in 0..words {
        core::ptr::copy_nonoverlapping(s.add(i * 4), d.add(i * 4), 4);
    }
    let offset = words * 4;
    for i in 0..(n & 3) {
        *d.add(offset + i) = *s.add(offset + i);
    }
    dest
}

#[cfg(not(target_pointer_width = "32"))]
unsafe fn ____memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void {
    let d = dest as *mut u8;
    let s = src as *const u8;

    // C uses `rep movsq` followed by `rep movsb` here.
    let words = n >> 3;
    for i in 0..words {
        core::ptr::copy_nonoverlapping(s.add(i * 8), d.add(i * 8), 8);
    }
    let offset = words * 8;
    for i in 0..(n & 7) {
        *d.add(offset + i) = *s.add(offset + i);
    }
    dest
}

pub unsafe fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void {
    let ss = s as *mut i8;

    for i in 0..n {
        *ss.add(i) = c as i8;
    }
    s
}

pub unsafe fn memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, mut n: usize) -> *mut core::ffi::c_void {
    let d = dest as *mut u8;
    let s = src as *const u8;

    if (d as usize) <= (s as usize) || (d as usize).wrapping_sub(s as usize) >= n {
        return ____memcpy(dest, src, n);
    }

    while n > 0 {
        n -= 1;
        *d.add(n) = *s.add(n);
    }

    dest
}

/* Detect and warn about potential overlaps, but handle them with memmove. */
unsafe extern "C" {
    fn warn(message: *const core::ffi::c_char, ...);
}

pub unsafe fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void {
    if (dest as usize) > (src as usize)
        && (dest as usize).wrapping_sub(src as usize) < n
    {
        warn(c"Avoiding potentially unsafe overlapping memcpy()!".as_ptr());
        return memmove(dest, src, n);
    }
    ____memcpy(dest, src, n)
}

#[cfg(feature = "CONFIG_KASAN")]
pub unsafe fn __memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void {
    memset(s, c, n)
}

#[cfg(feature = "CONFIG_KASAN")]
pub unsafe fn __memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void {
    memmove(dest, src, n)
}

#[cfg(feature = "CONFIG_KASAN")]
pub unsafe fn __memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void {
    memcpy(dest, src, n)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
