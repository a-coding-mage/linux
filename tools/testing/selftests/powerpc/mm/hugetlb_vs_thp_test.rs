// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_long, c_void};

// C dependencies:
// #include <stdio.h>
// #include <sys/mman.h>
// #include <unistd.h>
// #include "utils.h"

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_HUGETLB: c_int = 0x40000;
const MAP_FAILED: *mut c_void = !0 as *mut c_void;

/*
 * This must match the huge page & THP size
 */
const SIZE: usize = 16 * 1024 * 1024;

unsafe extern "C" {
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;

    fn test_harness(
        test_function: Option<unsafe extern "C" fn() -> c_int>,
        name: *const c_char,
    ) -> c_int;
}

unsafe extern "C" fn test_body() -> c_int {
    let addr: *mut c_void;
    let mut p: *mut c_char;

    addr = 0xa0000000usize as *mut c_void;

    p = unsafe {
        mmap(
            addr,
            SIZE,
            PROT_READ | PROT_WRITE,
            MAP_HUGETLB | MAP_ANONYMOUS | MAP_PRIVATE,
            -1,
            0,
        ) as *mut c_char
    };
    if p != MAP_FAILED as *mut c_char {
        /*
         * Typically the mmap will fail because no huge pages are
         * allocated on the system. But if there are huge pages
         * allocated the mmap will succeed. That's fine too, we just
         * munmap here before continuing.  munmap() length of
         * MAP_HUGETLB memory must be hugepage aligned.
         */
        if unsafe { munmap(addr, SIZE) } != 0 {
            unsafe {
                perror(c"munmap".as_ptr());
            }
            return 1;
        }
    }

    p = unsafe {
        mmap(
            addr,
            SIZE,
            PROT_READ | PROT_WRITE,
            MAP_ANONYMOUS | MAP_PRIVATE,
            -1,
            0,
        ) as *mut c_char
    };
    if p == MAP_FAILED as *mut c_char {
        unsafe {
            printf(c"Mapping failed @ %p\n".as_ptr(), addr);
            perror(c"mmap".as_ptr());
        }
        return 1;
    }

    /*
     * Either a user or kernel access is sufficient to trigger the bug.
     * A kernel access is easier to spot & debug, as it will trigger the
     * softlockup or RCU stall detectors, and when the system is kicked
     * into xmon we get a backtrace in the kernel.
     *
     * A good option is:
     *  getcwd(p, SIZE);
     *
     * For the purposes of this testcase it's preferable to spin in
     * userspace, so the harness can kill us if we get stuck. That way we
     * see a test failure rather than a dead system.
     */
    unsafe {
        *p = 0xf;
    }

    unsafe {
        munmap(addr, SIZE);
    }

    0
}

unsafe extern "C" fn test_main() -> c_int {
    let mut i: c_int;

    /*
     * 10,000 because it's a "bunch", and completes reasonably quickly
     */
    i = 0;
    while i < 10000 {
        if unsafe { test_body() } != 0 {
            return 1;
        }
        i += 1;
    }

    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(Some(test_main), c"hugetlb_vs_thp".as_ptr()));
    }
}
