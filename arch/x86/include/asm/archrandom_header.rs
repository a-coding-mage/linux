/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This file is part of the Linux kernel.
 *
 * Copyright (c) 2011-2014, Intel Corporation
 * Authors: Fenghua Yu <fenghua.yu@intel.com>,
 *          H. Peter Anvin <hpa@linux.intel.com>
 */

// C dependencies: <asm/processor.h> and <asm/cpufeature.h>

pub const RDRAND_RETRY_LOOPS: u32 = 10;

/* Unconditional execution of RDRAND and RDSEED */

pub unsafe fn rdrand_long(v: *mut usize) -> bool {
    let mut ok: u8;
    let mut retry: u32 = RDRAND_RETRY_LOOPS;
    loop {
        let value: usize;
        core::arch::asm!(
            "rdrand {value}",
            "setc {ok}",
            value = lateout(reg) value,
            ok = lateout(reg_byte) ok,
            options(nostack, preserves_flags),
        );
        *v = value;
        if ok != 0 {
            return true;
        }
        retry = retry.wrapping_sub(1);
        if retry == 0 {
            break;
        }
    }
    false
}

pub unsafe fn rdseed_long(v: *mut usize) -> bool {
    let ok: u8;
    let value: usize;
    core::arch::asm!(
        "rdseed {value}",
        "setc {ok}",
        value = lateout(reg) value,
        ok = lateout(reg_byte) ok,
        options(nostack, preserves_flags),
    );
    *v = value;
    ok != 0
}

/*
 * These are the generic interfaces; they must not be declared if
 * the stubs in <linux/random.h> are to be invoked.
 */

pub unsafe fn arch_get_random_longs(v: *mut usize, max_longs: usize) -> usize {
    if max_longs != 0 && cpu_feature_enabled(X86_FEATURE_RDRAND) && rdrand_long(v) {
        1
    } else {
        0
    }
}

pub unsafe fn arch_get_random_seed_longs(v: *mut usize, max_longs: usize) -> usize {
    if max_longs != 0 && cpu_feature_enabled(X86_FEATURE_RDSEED) && rdseed_long(v) {
        1
    } else {
        0
    }
}

// Under !CONFIG_UML:
// extern "C" { fn x86_init_rdrand(c: *mut cpuinfo_x86); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
