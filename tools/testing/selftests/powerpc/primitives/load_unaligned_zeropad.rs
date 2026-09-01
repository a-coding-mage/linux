// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Userspace test harness for load_unaligned_zeropad. Creates two
 * pages and uses mprotect to prevent access to the second page and
 * a SEGV handler that walks the exception tables and runs the fixup
 * routine.
 *
 * The results are compared against a normal load that is that is
 * performed while access to the second page is enabled via mprotect.
 *
 * Copyright (C) 2014 Anton Blanchard <anton@au.ibm.com>, IBM
 */

// C dependencies:
// #include <stdlib.h>
// #include <string.h>
// #include <stdio.h>
// #include <stdbool.h>
// #include <signal.h>
// #include <unistd.h>
// #include <sys/mman.h>
// #include "word-at-a-time.h"
// #include "utils.h"

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const FIXUP_SECTION: &str = ".ex_fixup";

#[inline]
unsafe fn __fls(x: c_ulong) -> c_ulong {
    let lz: c_int;

    #[cfg(target_pointer_width = "64")]
    asm!("cntlzd {0},{1}", out(reg) lz, in(reg) x);
    #[cfg(target_pointer_width = "32")]
    asm!("cntlzw {0},{1}", out(reg) lz, in(reg) x);

    (size_of::<c_ulong>() as c_ulong)
        .wrapping_sub(1)
        .wrapping_sub(lz as c_ulong)
}

static mut PAGE_SIZE: c_int = 0;
static mut MEM_REGION: *mut c_char = ptr::null_mut();

unsafe fn protect_region() -> c_int {
    if mprotect(
        MEM_REGION.add(PAGE_SIZE as usize) as *mut c_void,
        PAGE_SIZE as usize,
        PROT_NONE,
    ) != 0
    {
        perror(c"mprotect".as_ptr());
        return 1;
    }

    0
}

unsafe fn unprotect_region() -> c_int {
    if mprotect(
        MEM_REGION.add(PAGE_SIZE as usize) as *mut c_void,
        PAGE_SIZE as usize,
        PROT_READ | PROT_WRITE,
    ) != 0
    {
        perror(c"mprotect".as_ptr());
        return 1;
    }

    0
}

unsafe extern "C" {
    static mut __start___ex_table: [c_char; 0];
    static mut __stop___ex_table: [c_char; 0];
}

#[repr(C)]
struct extbl_entry {
    insn: c_int,
    fixup: c_int,
}

unsafe extern "C" {
    fn UCONTEXT_NIA(uc: *mut ucontext_t) -> *mut c_ulong;
    fn load_unaligned_zeropad(p: *mut c_char) -> c_ulong;
    fn test_harness(test: unsafe fn() -> c_int, name: *const c_char) -> c_int;

    fn abort() -> !;
    fn getpagesize() -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn mprotect(addr: *mut c_void, len: usize, prot: c_int) -> c_int;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
}

#[allow(non_camel_case_types)]
type siginfo_t = libc::siginfo_t;
#[allow(non_camel_case_types)]
type sigset_t = libc::sigset_t;
#[allow(non_camel_case_types)]
type ucontext_t = libc::ucontext_t;

#[repr(C)]
struct sigaction {
    sa_sigaction: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: Option<unsafe extern "C" fn()>,
}

const PROT_NONE: c_int = 0x0;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const SA_SIGINFO: c_int = 4;
const SIGSEGV: c_int = 11;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

unsafe extern "C" fn segv_handler(signr: c_int, info: *mut siginfo_t, ptr: *mut c_void) {
    let uc: *mut ucontext_t = ptr as *mut ucontext_t;
    let addr: c_ulong = (*info).si_addr() as c_ulong;
    let ip: *mut c_ulong = UCONTEXT_NIA(uc);
    let entry: *mut extbl_entry = __start___ex_table.as_mut_ptr() as *mut extbl_entry;

    while entry < (__stop___ex_table.as_mut_ptr() as *mut extbl_entry) {
        let insn: c_ulong;
        let fixup: c_ulong;

        insn = (&raw mut (*entry).insn as c_ulong).wrapping_add((*entry).insn as c_ulong);
        fixup = (&raw mut (*entry).fixup as c_ulong).wrapping_add((*entry).fixup as c_ulong);

        if insn == *ip {
            *ip = fixup;
            return;
        }
    }

    printf(
        c"No exception table match for NIA %lx ADDR %lx\n".as_ptr(),
        *ip,
        addr,
    );
    abort();
}

unsafe fn setup_segv_handler() {
    let mut action: sigaction = core::mem::zeroed();

    action.sa_sigaction = segv_handler;
    action.sa_flags = SA_SIGINFO;
    sigaction(SIGSEGV, &action, ptr::null_mut());
}

unsafe fn do_one_test(p: *mut c_char, page_offset: c_int) -> c_int {
    let should: c_ulong;
    let got: c_ulong;

    FAIL_IF!(unprotect_region() != 0);
    should = *(p as *mut c_ulong);
    FAIL_IF!(protect_region() != 0);

    got = load_unaligned_zeropad(p);

    if should != got {
        printf(
            c"offset %u load_unaligned_zeropad returned 0x%lx, should be 0x%lx\n".as_ptr(),
            page_offset,
            got,
            should,
        );
        return 1;
    }

    0
}

unsafe fn test_body() -> c_int {
    let mut i: c_ulong;

    PAGE_SIZE = getpagesize();
    MEM_REGION = mmap(
        ptr::null_mut(),
        (PAGE_SIZE * 2) as usize,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut c_char;

    FAIL_IF!(MEM_REGION == MAP_FAILED as *mut c_char);

    i = 0;
    while i < PAGE_SIZE as c_ulong {
        *MEM_REGION.add(i as usize) = i as c_char;
        i += 1;
    }

    ptr::write_bytes(MEM_REGION.add(PAGE_SIZE as usize), 0, PAGE_SIZE as usize);

    setup_segv_handler();

    i = 0;
    while i < PAGE_SIZE as c_ulong {
        FAIL_IF!(do_one_test(MEM_REGION.add(i as usize), i as c_int) != 0);
        i += 1;
    }

    0
}

fn main() -> c_int {
    unsafe { test_harness(test_body, c"load_unaligned_zeropad".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
