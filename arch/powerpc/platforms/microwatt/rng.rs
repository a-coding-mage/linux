// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Derived from arch/powerpc/platforms/powernv/rng.c, which is:
 * Copyright 2013, Michael Ellerman, IBM Corporation.
 */

// pr_fmt(fmt) is "microwatt-rng: " followed by fmt.

// The following symbols are supplied by the corresponding kernel/platform
// dependencies.

const DARN_ERR: core::ffi::c_ulong = 0xFFFF_FFFF_FFFF_FFFF;

unsafe fn microwatt_get_random_darn(v: *mut core::ffi::c_ulong) -> i32 {
    let val: core::ffi::c_ulong;

    /* Using DARN with L=1 - 64-bit conditioned random number */
    // This instruction is available on the target PowerPC platform.
    unsafe {
        core::arch::asm!("darn {0}, 1", out(reg) val);
    }

    if val == DARN_ERR {
        return 0;
    }

    unsafe {
        *v = val;
    }

    1
}

pub unsafe fn microwatt_rng_init() {
    let mut val: core::ffi::c_ulong = 0;
    let mut i: i32 = 0;

    while i < 10 {
        if unsafe { microwatt_get_random_darn(&mut val) } != 0 {
            // ppc_md.get_random_seed = microwatt_get_random_darn;
            unsafe {
                crate::ppc_md.get_random_seed = Some(microwatt_get_random_darn);
            }
            return;
        }
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
