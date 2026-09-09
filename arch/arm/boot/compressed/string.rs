// SPDX-License-Identifier: GPL-2.0
/*
 * arch/arm/boot/compressed/string.c
 *
 * Small subset of simple string routines
 */

// __NO_FORTIFY and the linux/string.h dependency are supplied by the build.

/*
 * The decompressor is built without KASan but uses the same redirects as the
 * rest of the kernel when CONFIG_KASAN is enabled, defining e.g. memcpy()
 * to __memcpy() but since we are not linking with the main kernel string
 * library in the decompressor, that will lead to link failures.
 *
 * Undefine KASan's versions, define the wrapped functions and alias them to
 * the right names so that when e.g. __memcpy() appear in the code, it will
 * still be linked to this local version of memcpy().
 */
#[cfg(CONFIG_KASAN)]
mod kasan_aliases {
    use core::ffi::c_void;

    // C __alias declarations; the linker aliases these symbols to the local
    // implementations below.
    extern "C" {
        pub fn __memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
        pub fn __memmove(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
        pub fn __memset(s: *mut c_void, c: i32, count: usize) -> *mut c_void;
    }
}

pub unsafe extern "C" fn memcpy(
    dest: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    n: usize,
) -> *mut core::ffi::c_void {
    let mut i = (n >> 3) as isize;
    let mut d = dest as *mut u8;
    let mut s = src as *const u8;

    while i > 0 {
        *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1);
        i -= 1;
    }

    if n & (1 << 2) != 0 {
        for _ in 0..4 { *d = *s; d = d.add(1); s = s.add(1); }
    }
    if n & (1 << 1) != 0 {
        for _ in 0..2 { *d = *s; d = d.add(1); s = s.add(1); }
    }
    if n & 1 != 0 { *d = *s; }
    dest
}

pub unsafe extern "C" fn memmove(
    dest: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    mut count: usize,
) -> *mut core::ffi::c_void {
    if dest == src { return dest; }
    if (dest as usize) < (src as usize) { return memcpy(dest, src, count); }
    let d = dest as *mut u8;
    let s = src as *const u8;
    while count != 0 {
        count -= 1;
        *d.add(count) = *s.add(count);
    }
    dest
}

pub unsafe extern "C" fn strlen(s: *const i8) -> usize {
    let mut sc = s;
    while *sc != 0 { sc = sc.add(1); }
    sc as usize - s as usize
}

pub unsafe extern "C" fn strnlen(s: *const i8, mut count: usize) -> usize {
    let mut sc = s;
    while count != 0 && *sc != 0 { count -= 1; sc = sc.add(1); }
    sc as usize - s as usize
}

pub unsafe extern "C" fn memcmp(cs: *const core::ffi::c_void, ct: *const core::ffi::c_void, count: usize) -> i32 {
    let mut su1 = cs as *const u8;
    let mut su2 = ct as *const u8;
    let end = su1.add(count);
    let mut res = 0;
    while su1 < end {
        res = *su1 as i32 - *su2 as i32;
        su1 = su1.add(1); su2 = su2.add(1);
        if res != 0 { break; }
    }
    res
}

pub unsafe extern "C" fn strcmp(cs: *const i8, ct: *const i8) -> i32 {
    let mut a = cs;
    let mut b = ct;
    loop {
        let c1 = *a as u8; let c2 = *b as u8;
        a = a.add(1); b = b.add(1);
        let res = c1 as i32 - c2 as i32;
        if res != 0 || c1 == 0 { return res; }
    }
}

pub unsafe extern "C" fn memchr(s: *const core::ffi::c_void, c: i32, mut count: usize) -> *mut core::ffi::c_void {
    let mut p = s as *const u8;
    while count != 0 {
        count -= 1;
        if c as u8 == *p { return p as *mut core::ffi::c_void; }
        p = p.add(1);
    }
    core::ptr::null_mut()
}

pub unsafe extern "C" fn strchr(mut s: *const i8, c: i32) -> *mut i8 {
    while *s != c as i8 {
        if *s == 0 { return core::ptr::null_mut(); }
        s = s.add(1);
    }
    s as *mut i8
}

pub unsafe extern "C" fn strrchr(mut s: *const i8, c: i32) -> *mut i8 {
    let mut last = core::ptr::null();
    loop {
        if *s == c as i8 { last = s; }
        let current = *s;
        s = s.add(1);
        if current == 0 { break; }
    }
    last as *mut i8
}

pub unsafe extern "C" fn memset(s: *mut core::ffi::c_void, c: i32, mut count: usize) -> *mut core::ffi::c_void {
    let mut xs = s as *mut i8;
    while count != 0 {
        count -= 1;
        *xs = c as i8;
        xs = xs.add(1);
    }
    s
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
