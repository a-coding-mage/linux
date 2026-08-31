// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2015, Michael Neuling, IBM Corp.
 *
 * Original: Michael Neuling 3/4/2014
 * Modified: Rashmica Gupta 8/12/2015
 *
 * Check if any of the Transaction Memory SPRs get corrupted.
 * - TFIAR  - stores address of location of transaction failure
 * - TFHAR  - stores address of software failure handler (if transaction
 *   fails)
 * - TEXASR - lots of info about the transacion(s)
 *
 * (1) create more threads than cpus
 * (2) in each thread:
 * 	(a) set TFIAR and TFHAR a unique value
 * 	(b) loop for awhile, continually checking to see if
 * 	either register has been corrupted.
 *
 * (3) Loop:
 * 	(a) begin transaction
 *    	(b) abort transaction
 *	(c) check TEXASR to see if FS has been corrupted
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type pthread_t = c_ulong;

const EXIT_FAILURE: c_int = 1;
const _SC_NPROCESSORS_ONLN: c_int = 84;

unsafe extern "C" {
    static SPRN_TFIAR: c_ulong;
    static SPRN_TFHAR: c_ulong;
    static SPRN_TEXASR: c_ulong;
    static TEXASR_FS: u64;

    fn printf(format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn sysconf(name: c_int) -> c_long;

    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn test_harness(test_function: extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    fn mfspr(spr: c_ulong) -> u64;
    fn mtspr(spr: c_ulong, value: c_ulong);

    /* Supplied by the selftest support headers; the C macro may return early. */
    fn SKIP_IF(condition: c_int);
}

static mut num_loops: c_int = 1000000;
static mut passed: c_int = 1;

extern "C" fn tfiar_tfhar(in_: *mut c_void) -> *mut c_void {
    let mut tfhar: c_ulong;
    let mut tfhar_rd: c_ulong;
    let mut tfiar: c_ulong;
    let mut tfiar_rd: c_ulong;
    let mut i: c_int;

    unsafe {
        /* TFIAR: Last bit has to be high so userspace can read register */
        tfiar = in_ as c_ulong + 1;
        tfiar += 2;
        mtspr(SPRN_TFIAR, tfiar);

        /* TFHAR: Last two bits are reserved */
        tfhar = in_ as c_ulong;
        tfhar &= !0x3_u64 as c_ulong;
        tfhar += 4;
        mtspr(SPRN_TFHAR, tfhar);

        i = 0;
        while i < num_loops {
            tfhar_rd = mfspr(SPRN_TFHAR) as c_ulong;
            tfiar_rd = mfspr(SPRN_TFIAR) as c_ulong;
            if tfhar != tfhar_rd || tfiar != tfiar_rd {
                passed = 0;
                return ptr::null_mut();
            }
            i += 1;
        }
    }
    ptr::null_mut()
}

extern "C" fn texasr(_in: *mut c_void) -> *mut c_void {
    let mut i: c_ulong;
    let mut result: u64 = 0;

    unsafe {
        i = 0;
        while i < num_loops as c_ulong {
            asm!(
                "tbegin.",
                "beq    3f",
                "tabort. 0",
                "tend.",
                "3:",
                options(nostack)
            );

            /* Check the TEXASR */
            result = mfspr(SPRN_TEXASR);
            if result & TEXASR_FS == 0 {
                passed = 0;
                return ptr::null_mut();
            }
            i += 1;
        }
    }
    ptr::null_mut()
}

extern "C" fn test_tmspr() -> c_int {
    let mut thread: *mut pthread_t;
    let thread_num: c_int;
    let mut i: c_ulong;

    unsafe {
        SKIP_IF((have_htm() == 0) as c_int);
        SKIP_IF((htm_is_synthetic() != 0) as c_int);

        /* To cause some context switching */
        thread_num = 10 * sysconf(_SC_NPROCESSORS_ONLN) as c_int;

        thread = malloc(thread_num as usize * size_of::<pthread_t>()) as *mut pthread_t;
        if thread.is_null() {
            return EXIT_FAILURE;
        }

        /* Test TFIAR and TFHAR */
        i = 0;
        while i < thread_num as c_ulong {
            if pthread_create(
                thread.add(i as usize),
                ptr::null(),
                tfiar_tfhar,
                i as *mut c_void,
            ) != 0
            {
                return EXIT_FAILURE;
            }
            i += 2;
        }

        /* Test TEXASR */
        i = 1;
        while i < thread_num as c_ulong {
            if pthread_create(thread.add(i as usize), ptr::null(), texasr, i as *mut c_void) != 0 {
                return EXIT_FAILURE;
            }
            i += 2;
        }

        i = 0;
        while i < thread_num as c_ulong {
            if pthread_join(*thread.add(i as usize), ptr::null_mut()) != 0 {
                return EXIT_FAILURE;
            }
            i += 1;
        }

        free(thread as *mut c_void);

        if passed != 0 {
            0
        } else {
            1
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        if argc > 1 {
            if strcmp(*argv.add(1), b"-h\0".as_ptr() as *const c_char) == 0 {
                printf(b"Syntax:\t [<num loops>]\n\0".as_ptr() as *const c_char);
                return 0;
            } else {
                num_loops = atoi(*argv.add(1));
            }
        }
        test_harness(test_tmspr, b"tm_tmspr\0".as_ptr() as *const c_char)
    }
}
