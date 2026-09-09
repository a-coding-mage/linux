// SPDX-License-Identifier: GPL-2.0
/*
 * Most of the string-functions are rather heavily hand-optimized,
 * see especially strsep,strstr,str[c]spn. They should work, but are not
 * very easy to understand. Everything is done entirely within the register
 * set, making the functions fast and clean. String instructions have been
 * used through-out, making for "slightly" unclear code :-)
 *
 * AK: On P4 and K7 using non string instruction implementations might be faster
 * for large memory blocks. But most of them are unlikely to be used on large
 * strings.
 */

// __NO_FORTIFY; dependencies from linux/string.h and linux/export.h are external.

#[cfg(__HAVE_ARCH_STRCPY)]
pub unsafe fn strcpy(dest: *mut u8, src: *const u8) -> *mut u8 {
    let mut d = dest;
    let mut s = src;
    loop {
        let v = *s;
        *d = v;
        s = s.add(1);
        d = d.add(1);
        if v == 0 { break; }
    }
    dest
}

#[cfg(__HAVE_ARCH_STRCAT)]
pub unsafe fn strcat(dest: *mut u8, src: *const u8) -> *mut u8 {
    let mut d = dest;
    while *d != 0 { d = d.add(1); }
    let mut s = src;
    loop {
        let v = *s;
        *d = v;
        s = s.add(1);
        d = d.add(1);
        if v == 0 { break; }
    }
    dest
}

#[cfg(__HAVE_ARCH_STRNCAT)]
pub unsafe fn strncat(dest: *mut u8, src: *const u8, count: usize) -> *mut u8 {
    let mut d = dest;
    while *d != 0 { d = d.add(1); }
    let mut s = src;
    let mut n = count;
    while n != 0 {
        let v = *s;
        *d = v;
        d = d.add(1);
        s = s.add(1);
        n -= 1;
        if v == 0 { return dest; }
    }
    *d = 0;
    dest
}

#[cfg(__HAVE_ARCH_STRCMP)]
pub unsafe fn strcmp(cs: *const u8, ct: *const u8) -> i32 {
    let mut a = cs;
    let mut b = ct;
    loop {
        let x = *a;
        let y = *b;
        if x != y { return if x < y { -1 } else { 1 }; }
        if x == 0 { return 0; }
        a = a.add(1); b = b.add(1);
    }
}

#[cfg(__HAVE_ARCH_STRNCMP)]
pub unsafe fn strncmp(cs: *const u8, ct: *const u8, count: usize) -> i32 {
    let mut a = cs;
    let mut b = ct;
    let mut n = count;
    while n != 0 {
        let x = *a;
        let y = *b;
        if x != y { return if x < y { -1 } else { 1 }; }
        if x == 0 { return 0; }
        a = a.add(1); b = b.add(1); n -= 1;
    }
    0
}

#[cfg(__HAVE_ARCH_STRCHR)]
pub unsafe fn strchr(s: *const u8, c: i32) -> *mut u8 {
    let mut p = s;
    let wanted = c as u8;
    loop {
        let v = *p;
        if v == wanted { return p as *mut u8; }
        if v == 0 { return core::ptr::null_mut(); }
        p = p.add(1);
    }
}

#[cfg(__HAVE_ARCH_STRLEN)]
pub unsafe fn strlen(s: *const u8) -> usize {
    let mut p = s;
    while *p != 0 { p = p.add(1); }
    p.offset_from(s) as usize
}

#[cfg(__HAVE_ARCH_MEMCHR)]
pub unsafe fn memchr(cs: *const u8, c: i32, count: usize) -> *mut u8 {
    let mut p = cs;
    let wanted = c as u8;
    for _ in 0..count {
        if *p == wanted { return p as *mut u8; }
        p = p.add(1);
    }
    core::ptr::null_mut()
}

#[cfg(__HAVE_ARCH_MEMSCAN)]
pub unsafe fn memscan(addr: *mut u8, c: i32, size: usize) -> *mut u8 {
    let mut p = addr;
    let wanted = c as u8;
    for _ in 0..size {
        if *p == wanted { return p; }
        p = p.add(1);
    }
    p
}

#[cfg(__HAVE_ARCH_STRNLEN)]
pub unsafe fn strnlen(s: *const u8, count: usize) -> usize {
    let mut p = s;
    let mut n = 0;
    while n < count && *p != 0 {
        p = p.add(1);
        n += 1;
    }
    n
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
