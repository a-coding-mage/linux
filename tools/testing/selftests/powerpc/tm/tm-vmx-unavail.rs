// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2017, Michael Neuling, IBM Corp.
 * Original: Breno Leitao <brenohl@br.ibm.com> &
 *           Gustavo Bueno Romero <gromero@br.ibm.com>
 * Edited: Michael Neuling
 *
 * Force VMX unavailable during a transaction and see if it corrupts
 * the checkpointed VMX register state after the abort.
 */

// C dependencies originally included:
// <inttypes.h>, <htmintrin.h>, <string.h>, <stdlib.h>, <stdio.h>,
// <pthread.h>, <sys/mman.h>, <unistd.h>, "tm.h", "utils.h"

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::ptr;

type pthread_t = c_ulong;

const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const _SC_NPROCESSORS_ONLN: c_int = 84;

static mut passed: c_int = 0;

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
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
    fn test_harness(test: extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    #[link_name = "__builtin_get_texasr"]
    fn __builtin_get_texasr() -> u64;

    fn _TEXASR_FAILURE_CODE(texasr: u64) -> u64;
    fn _TEXASR_FAILURE_SUMMARY(texasr: u64) -> u64;
    fn _TEXASR_TFIAR_EXACT(texasr: u64) -> u64;
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return EXIT_SUCCESS;
        }
    };
}

extern "C" fn worker(_unused: *mut c_void) -> *mut c_void {
    let mut vmx0: i128 = 0;
    let texasr: u64;

    unsafe {
        asm!(
            "li       3, 1;",
            "std      3, 0({vmx0_ptr});",
            "lvx      0, 0, {vmx0_ptr};",

            /* Wait here a bit so we get scheduled out 255 times */
            "lis      3, 0x3fff;",
            "1: ;",
            "addi     3, 3, -1;",
            "cmpdi    3, 0;",
            "bne      1b;",

            /* Kernel will hopefully turn VMX off now */

            "tbegin. ;",
            "beq      2f;",

            /* Cause VMX unavail. Any VMX instruction */
            "vaddcuw  0,0,0;",

            "tend. ;",
            "b        4f;",

            /* Check VMX0 sanity after abort */
            "2: ;",
            "lvx       1,  0, {vmx0_ptr};",
            "vcmpequb. 2,  0, 1;",
            "bc        4, 24, 3f;",
            "b        5f;",
            "3: ;",
            vmx0_ptr = in(reg) &mut vmx0,
            out("r3") _,
        );
    }

    /* HTM aborted and VMX0 is corrupted */
    unsafe {
        texasr = __builtin_get_texasr();

        printf(b"\n\n==============\n\n\0".as_ptr() as *const c_char);
        printf(
            b"Failure with error: %lx\n\0".as_ptr() as *const c_char,
            _TEXASR_FAILURE_CODE(texasr),
        );
        printf(
            b"Summary error     : %lx\n\0".as_ptr() as *const c_char,
            _TEXASR_FAILURE_SUMMARY(texasr),
        );
        printf(
            b"TFIAR exact       : %lx\n\n\0".as_ptr() as *const c_char,
            _TEXASR_TFIAR_EXACT(texasr),
        );

        passed = 0;
    }
    return ptr::null_mut();

    /* HTM aborted but VMX0 is correct */
    #[allow(unreachable_code)]
    unsafe {
        asm!("5: ;");
    }
    //	printf!("!");
    return ptr::null_mut();

    #[allow(unreachable_code)]
    unsafe {
        asm!("4: ;");
    }
    //	printf!(".");
    return ptr::null_mut();
}

extern "C" fn tm_vmx_unavail_test() -> c_int {
    let threads: c_int;
    let thread: *mut pthread_t;

    unsafe {
        SKIP_IF!(have_htm() == 0);
        SKIP_IF!(htm_is_synthetic() != 0);

        passed = 1;

        threads = (sysconf(_SC_NPROCESSORS_ONLN) * 4) as c_int;
        thread = malloc(core::mem::size_of::<pthread_t>() * threads as usize) as *mut pthread_t;
        if thread.is_null() {
            return EXIT_FAILURE;
        }

        let mut i: u64 = 0;
        while i < threads as u64 {
            pthread_create(thread.add(i as usize), ptr::null(), worker, ptr::null_mut());
            i += 1;
        }

        let mut i: u64 = 0;
        while i < threads as u64 {
            pthread_join(*thread.add(i as usize), ptr::null_mut());
            i += 1;
        }

        free(thread as *mut c_void);

        if passed != 0 {
            EXIT_SUCCESS
        } else {
            EXIT_FAILURE
        }
    }
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            tm_vmx_unavail_test,
            b"tm_vmx_unavail_test\0".as_ptr() as *const c_char,
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
