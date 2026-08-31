// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Test basic matrix multiply assist (MMA) functionality if available.
 *
 * Copyright 2020, Alistair Popple, IBM Corp.
 */

use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_int};

// Dependencies from "utils.h".
unsafe extern "C" {
    fn have_hwcap2(feature: u64) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    static PPC_FEATURE2_ARCH_3_1: u64;
    static PPC_FEATURE2_MMA: u64;
}

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
}

unsafe extern "C" {
    fn test_mma(x: *mut [u16; 8], y: *mut [u16; 8], z: *mut [u32; 4 * 4]);
}

// C source used SKIP_IF_MSG from utils.h here.
macro_rules! SKIP_IF_MSG {
    ($cond:expr, $msg:expr) => {
        if $cond {
            return test_harness_skip($msg.as_ptr() as *const c_char);
        }
    };
}

unsafe fn test_harness_skip(_msg: *const c_char) -> c_int {
    /*
     * TODO: map utils.h's SKIP_IF_MSG skip-return behavior when that external
     * dependency is available to this translated file.
     */
    0
}

unsafe extern "C" fn mma() -> c_int {
    let mut i: c_int;
    let mut rc: c_int = 0;
    let mut x: [u16; 8] = [1, 0, 2, 0, 3, 0, 4, 0];
    let mut y: [u16; 8] = [1, 0, 2, 0, 3, 0, 4, 0];
    let mut z: MaybeUninit<[u32; 4 * 4]> = MaybeUninit::uninit();
    let exp: [u32; 4 * 4] = [
        1, 2, 3, 4,
        2, 4, 6, 8,
        3, 6, 9, 12,
        4, 8, 12, 16,
    ];

    SKIP_IF_MSG!(
        have_hwcap2(PPC_FEATURE2_ARCH_3_1) == 0,
        c"Need ISAv3.1"
    );
    SKIP_IF_MSG!(have_hwcap2(PPC_FEATURE2_MMA) == 0, c"Need MMA");

    test_mma(&mut x, &mut y, z.as_mut_ptr());

    let z = z.assume_init();

    i = 0;
    while i < 16 {
        printf(c"MMA[%d] = %d ".as_ptr(), i, z[i as usize]);

        if z[i as usize] == exp[i as usize] {
            printf(c" (Correct)\n".as_ptr());
        } else {
            printf(c" (Incorrect)\n".as_ptr());
            rc = 1;
        }

        i += 1;
    }

    rc
}

fn main() {
    unsafe {
        std::process::exit(test_harness(mma, c"mma".as_ptr()));
    }
}
