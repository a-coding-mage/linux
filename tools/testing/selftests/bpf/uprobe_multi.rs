// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const MADV_POPULATE_READ: c_int = 22;
const MADV_PAGEOUT: c_int = 21;
const _SC_PAGESIZE: c_int = 30;

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn sysconf(name: c_int) -> c_long;
    fn madvise(addr: *mut c_void, length: usize, advice: c_int) -> c_int;
    fn mincore(addr: *mut c_void, length: usize, vec: *mut c_uchar) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;

    static mut build_id_start: [c_char; 0];
    static mut build_id_end: [c_char; 0];
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

type c_uchar = u8;

#[unsafe(no_mangle)]
pub extern "C" fn uprobe() -> c_int {
    0
}

/*
 * C preprocessor helpers translated from:
 *
 *   #define __PASTE(a, b) a##b
 *   #define PASTE(a, b) __PASTE(a, b)
 *   #define NAME(name, idx) PASTE(name, idx)
 *   #define DEF(name, idx) int __attribute__((weak)) NAME(name, idx)(void) { return 0; }
 *   #define CALL(name, idx) NAME(name, idx)();
 *   #define F(body, name, idx) body(name, idx)
 *   #define F10/F100/F1000/F10000 ...
 *
 * The original file expands these macros into weak definitions for
 * uprobe_multi_func_00000 through uprobe_multi_func_49999 and calls each of
 * them from bench(). Rust has no local stable equivalent of C token pasting for
 * defining identifiers without an external macro dependency, so the exact
 * macro-generated symbol set is preserved here as source-level intent.
 */

macro_rules! f10 {
    ($body:ident, $name:ident, $idx:tt) => {
        $body!($name, $idx, 0);
        $body!($name, $idx, 1);
        $body!($name, $idx, 2);
        $body!($name, $idx, 3);
        $body!($name, $idx, 4);
        $body!($name, $idx, 5);
        $body!($name, $idx, 6);
        $body!($name, $idx, 7);
        $body!($name, $idx, 8);
        $body!($name, $idx, 9);
    };
}

macro_rules! f100 {
    ($body:ident, $name:ident, $idx:tt) => {
        f10!($body, $name, ($idx, 0));
        f10!($body, $name, ($idx, 1));
        f10!($body, $name, ($idx, 2));
        f10!($body, $name, ($idx, 3));
        f10!($body, $name, ($idx, 4));
        f10!($body, $name, ($idx, 5));
        f10!($body, $name, ($idx, 6));
        f10!($body, $name, ($idx, 7));
        f10!($body, $name, ($idx, 8));
        f10!($body, $name, ($idx, 9));
    };
}

macro_rules! f1000 {
    ($body:ident, $name:ident, $idx:tt) => {
        f100!($body, $name, ($idx, 0));
        f100!($body, $name, ($idx, 1));
        f100!($body, $name, ($idx, 2));
        f100!($body, $name, ($idx, 3));
        f100!($body, $name, ($idx, 4));
        f100!($body, $name, ($idx, 5));
        f100!($body, $name, ($idx, 6));
        f100!($body, $name, ($idx, 7));
        f100!($body, $name, ($idx, 8));
        f100!($body, $name, ($idx, 9));
    };
}

macro_rules! f10000 {
    ($body:ident, $name:ident, $idx:tt) => {
        f1000!($body, $name, ($idx, 0));
        f1000!($body, $name, ($idx, 1));
        f1000!($body, $name, ($idx, 2));
        f1000!($body, $name, ($idx, 3));
        f1000!($body, $name, ($idx, 4));
        f1000!($body, $name, ($idx, 5));
        f1000!($body, $name, ($idx, 6));
        f1000!($body, $name, ($idx, 7));
        f1000!($body, $name, ($idx, 8));
        f1000!($body, $name, ($idx, 9));
    };
}

macro_rules! def {
    ($name:ident, $idx:tt, $last:tt) => {
        /* C token-pasted weak definition: NAME($name, $idx)$last(void) { return 0; } */
    };
}

macro_rules! call {
    ($name:ident, $idx:tt, $last:tt) => {
        /* C token-pasted call: NAME($name, $idx)$last(); */
    };
}

f10000!(def, uprobe_multi_func_, 0);
f10000!(def, uprobe_multi_func_, 1);
f10000!(def, uprobe_multi_func_, 2);
f10000!(def, uprobe_multi_func_, 3);
f10000!(def, uprobe_multi_func_, 4);

fn bench() -> c_int {
    f10000!(call, uprobe_multi_func_, 0);
    f10000!(call, uprobe_multi_func_, 1);
    f10000!(call, uprobe_multi_func_, 2);
    f10000!(call, uprobe_multi_func_, 3);
    f10000!(call, uprobe_multi_func_, 4);
    0
}

/*
 * C SystemTap probe macros translated from:
 *
 *   #define PROBE STAP_PROBE(test, usdt);
 *   #define PROBE10/PROBE100/PROBE1000/PROBE10000 ...
 *
 * STAP_PROBE is supplied by <sdt.h>. There is no file-local Rust equivalent for
 * the emitted probe note/asm, so each macro expansion preserves the external
 * probe side effect as a narrow TODO.
 */
macro_rules! probe {
    () => {
        /* TODO: external dependency STAP_PROBE(test, usdt); */
    };
}

macro_rules! probe10 {
    () => {
        probe!();
        probe!();
        probe!();
        probe!();
        probe!();
        probe!();
        probe!();
        probe!();
        probe!();
        probe!();
    };
}

macro_rules! probe100 {
    () => {
        probe10!();
        probe10!();
        probe10!();
        probe10!();
        probe10!();
        probe10!();
        probe10!();
        probe10!();
        probe10!();
        probe10!();
    };
}

macro_rules! probe1000 {
    () => {
        probe100!();
        probe100!();
        probe100!();
        probe100!();
        probe100!();
        probe100!();
        probe100!();
        probe100!();
        probe100!();
        probe100!();
    };
}

macro_rules! probe10000 {
    () => {
        probe1000!();
        probe1000!();
        probe1000!();
        probe1000!();
        probe1000!();
        probe1000!();
        probe1000!();
        probe1000!();
        probe1000!();
        probe1000!();
    };
}

fn usdt() -> c_int {
    probe10000!();
    probe10000!();
    probe10000!();
    probe10000!();
    probe10000!();
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trigger_uprobe(build_id_resident: bool) -> c_int {
    let page_sz: c_int = unsafe { sysconf(_SC_PAGESIZE) as c_int };
    let mut addr: *mut c_void;

    let mut vec: [c_uchar; 1] = [0; 1];
    let mut poll: c_int = 0;

    /* page-align build ID start */
    addr = ((unsafe { build_id_start.as_ptr() } as usize) & !((page_sz - 1) as usize)) as *mut c_void;

    /*
     * to guarantee MADV_PAGEOUT work reliably, we need to ensure that
     * memory range is mapped into current process, so we unconditionally
     * do MADV_POPULATE_READ, and then MADV_PAGEOUT, if necessary
     */
    unsafe {
        madvise(addr, page_sz as usize, MADV_POPULATE_READ);
    }
    if !build_id_resident {
        loop {
            unsafe {
                madvise(addr, page_sz as usize, MADV_PAGEOUT);
                /* check if page has been evicted */
                mincore(addr, page_sz as usize, vec.as_mut_ptr());
            }
            if !(vec[0] & 1) != 0 {
                break;
            }
            /* if page is still resident re-attempt MADV_POPULATE_READ/MADV_PAGEOUT */
            unsafe {
                madvise(addr, page_sz as usize, MADV_POPULATE_READ);
            }
            poll += 1;
            unsafe {
                usleep(100);
            }
            if !(poll < 500) {
                break;
            }
        }
    }
    let _ = uprobe();

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    if argc != 2 {
        unsafe {
            fprintf(
                stderr,
                c"usage: %s <bench|usdt|uprobe-paged-out|uprobe-paged-in>\n".as_ptr(),
                *argv,
            );
        }
        return -1;
    }

    unsafe {
        if strcmp(c"bench".as_ptr(), *argv.add(1)) == 0 {
            return bench();
        }
        if strcmp(c"usdt".as_ptr(), *argv.add(1)) == 0 {
            return usdt();
        }
        if strcmp(c"uprobe-paged-out".as_ptr(), *argv.add(1)) == 0 {
            return trigger_uprobe(false /* page-out build ID */);
        }
        if strcmp(c"uprobe-paged-in".as_ptr(), *argv.add(1)) == 0 {
            return trigger_uprobe(true /* page-in build ID */);
        }

        fprintf(
            stderr,
            c"usage: %s <bench|usdt|uprobe-paged-out|uprobe-paged-in>\n".as_ptr(),
            *argv,
        );
    }
    -1
}
