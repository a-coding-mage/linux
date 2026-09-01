// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2016, Anton Blanchard, Michael Ellerman, IBM Corp.
 */

// C dependencies:
// stdio.h, stdlib.h, sys/mman.h, time.h, getopt.h
// "utils.h"

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::ptr;

const ITERATIONS: c_int = 5000000;

const MEMSIZE: c_ulong = 1_u64.wrapping_shl(27) as c_ulong;
const PAGE_SIZE: c_ulong = 1_u64.wrapping_shl(16) as c_ulong;
const CHUNK_COUNT: c_ulong = MEMSIZE / PAGE_SIZE;

static mut pg_fault: c_int = 0;
static mut iterations: c_int = ITERATIONS;

static mut options: [libc::option; 3] = [
    libc::option {
        name: b"pgfault\0".as_ptr() as *const c_char,
        has_arg: libc::no_argument,
        flag: unsafe { &raw mut pg_fault },
        val: 1,
    },
    libc::option {
        name: b"iterations\0".as_ptr() as *const c_char,
        has_arg: libc::required_argument,
        flag: ptr::null_mut(),
        val: b'i' as c_int,
    },
    libc::option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

unsafe extern "C" {
    static mut optarg: *mut c_char;

    fn printf(format: *const c_char, ...) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn getopt_long(
        argc: c_int,
        argv: *const *mut c_char,
        optstring: *const c_char,
        longopts: *const libc::option,
        longindex: *mut c_int,
    ) -> c_int;
    fn clock_gettime(clk_id: libc::clockid_t, tp: *mut libc::timespec) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: libc::size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: libc::off_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: libc::size_t) -> c_int;

    fn test_harness_set_timeout(timeout: c_int);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
    fn FAIL_IF(condition: c_int);
}

unsafe fn usage() {
    printf(c"mmap_bench <--pgfault> <--iterations count>\n".as_ptr());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_mmap() -> c_int {
    let mut ts_start: libc::timespec = core::mem::zeroed();
    let mut ts_end: libc::timespec = core::mem::zeroed();
    let mut i: c_ulong = iterations as c_ulong;

    clock_gettime(libc::CLOCK_MONOTONIC, &mut ts_start);

    while {
        let old = i;
        i = i.wrapping_sub(1);
        old != 0
    } {
        let c = mmap(
            ptr::null_mut(),
            MEMSIZE as libc::size_t,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        ) as *mut c_char;
        FAIL_IF((c == libc::MAP_FAILED as *mut c_char) as c_int);
        if pg_fault != 0 {
            let mut count: c_int = 0;
            while count < CHUNK_COUNT as c_int {
                *c.offset((count << 16) as isize) = b'c' as c_char;
                count += 1;
            }
        }
        munmap(c as *mut c_void, MEMSIZE as libc::size_t);
    }

    clock_gettime(libc::CLOCK_MONOTONIC, &mut ts_end);

    printf(
        c"time = %.6f\n".as_ptr(),
        (ts_end.tv_sec - ts_start.tv_sec) as c_long as f64
            + (ts_end.tv_nsec - ts_start.tv_nsec) as c_long as f64 / 1e9_f64,
    );

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut c: i8;
    loop {
        let mut option_index: c_int = 0;

        c = getopt_long(
            argc,
            argv,
            c"".as_ptr(),
            (&raw const options) as *const libc::option,
            &mut option_index,
        ) as i8;

        if c == -1 {
            break;
        }

        match c as c_int {
            0 => {
                if options[option_index as usize].flag != ptr::null_mut() {
                    continue;
                }

                usage();
                exit(1);
            }
            x if x == b'i' as c_int => {
                iterations = atoi(optarg);
            }
            _ => {
                usage();
                exit(1);
            }
        }
    }

    test_harness_set_timeout(300);
    test_harness(test_mmap, c"mmap_bench".as_ptr())
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
