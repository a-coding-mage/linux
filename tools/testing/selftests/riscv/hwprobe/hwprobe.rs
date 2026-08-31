// SPDX-License-Identifier: GPL-2.0-only
// Translated from C. Dependencies originally came from:
// #include "hwprobe.h"
// #include "kselftest.h"

use std::ffi::{c_char, c_int, c_long, c_ulong};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_hwprobe {
    pub key: i64,
    pub value: u64,
}

unsafe extern "C" {
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_exit_fail_msg(msg: *const c_char, ...) -> !;
    fn ksft_test_result(test_success: c_int, msg: *const c_char, ...);
    fn ksft_finished();

    static RISCV_HWPROBE_KEY_BASE_BEHAVIOR: i64;
    static RISCV_HWPROBE_BASE_BEHAVIOR_IMA: u64;

    fn riscv_hwprobe(
        pairs: *mut riscv_hwprobe,
        pair_count: usize,
        cpu_count: usize,
        cpus: *mut c_ulong,
        flags: c_ulong,
    ) -> c_long;
}

unsafe fn c_bool(value: bool) -> c_int {
    if value {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut pairs: [riscv_hwprobe; 8] = [riscv_hwprobe { key: 0, value: 0 }; 8];
    let mut cpus: c_ulong;
    let mut out: c_long;

    ksft_print_header();
    ksft_set_plan(5);

    /* Fake the CPU_SET ops. */
    cpus = !0 as c_ulong;

    /*
     * Just run a basic test: pass enough pairs to get up to the base
     * behavior, and then check to make sure it's sane.
     */
    for i in 0..8 {
        pairs[i as usize].key = i;
    }

    out = riscv_hwprobe(pairs.as_mut_ptr(), 8, 1, &mut cpus, 0);
    if out != 0 {
        ksft_exit_fail_msg(
            b"hwprobe() failed with %ld\n\0".as_ptr() as *const c_char,
            out,
        );
    }

    for i in 0..4 {
        /* Fail if the kernel claims not to recognize a base key. */
        if (i < 4) && (pairs[i as usize].key != i) {
            ksft_exit_fail_msg(
                b"Failed to recognize base key: key != i, key=%lld, i=%ld\n\0".as_ptr()
                    as *const c_char,
                pairs[i as usize].key,
                i as c_long,
            );
        }

        if pairs[i as usize].key != RISCV_HWPROBE_KEY_BASE_BEHAVIOR {
            continue;
        }

        if (pairs[i as usize].value & RISCV_HWPROBE_BASE_BEHAVIOR_IMA) != 0 {
            continue;
        }

        ksft_exit_fail_msg(
            b"Unexpected pair: (%lld, %llu)\n\0".as_ptr() as *const c_char,
            pairs[i as usize].key,
            pairs[i as usize].value,
        );
    }

    out = riscv_hwprobe(pairs.as_mut_ptr(), 8, 0, std::ptr::null_mut(), 0);
    ksft_test_result(
        c_bool(out == 0),
        b"NULL CPU set\n\0".as_ptr() as *const c_char,
    );

    out = riscv_hwprobe(pairs.as_mut_ptr(), 8, 0, &mut cpus, 0);
    ksft_test_result(
        c_bool(out != 0),
        b"Bad CPU set\n\0".as_ptr() as *const c_char,
    );

    out = riscv_hwprobe(pairs.as_mut_ptr(), 8, 1, std::ptr::null_mut(), 0);
    ksft_test_result(
        c_bool(out != 0),
        b"NULL CPU set with non-zero size\n\0".as_ptr() as *const c_char,
    );

    pairs[0].key = RISCV_HWPROBE_KEY_BASE_BEHAVIOR;
    out = riscv_hwprobe(pairs.as_mut_ptr(), 1, 1, &mut cpus, 0);
    ksft_test_result(
        c_bool(out == 0 && pairs[0].key == RISCV_HWPROBE_KEY_BASE_BEHAVIOR),
        b"Existing key is maintained\n\0".as_ptr() as *const c_char,
    );

    pairs[0].key = 0x5555;
    pairs[1].key = 1;
    pairs[1].value = 0xAAAA;
    out = riscv_hwprobe(pairs.as_mut_ptr(), 2, 0, std::ptr::null_mut(), 0);
    ksft_test_result(
        c_bool(
            out == 0
                && pairs[0].key == -1
                && pairs[1].key == 1
                && pairs[1].value != 0xAAAA,
        ),
        b"Unknown key overwritten with -1 and doesn't block other elements\n\0".as_ptr()
            as *const c_char,
    );

    ksft_finished();
    0
}
