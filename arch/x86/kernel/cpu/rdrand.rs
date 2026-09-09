// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file is part of the Linux kernel.
 *
 * Copyright (c) 2011, Intel Corporation
 * Authors: Fenghua Yu <fenghua.yu@intel.com>,
 *          H. Peter Anvin <hpa@linux.intel.com>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// `cpuinfo_x86`, `cpu_has`, `rdrand_long`, `clear_cpu_cap`,
// `X86_FEATURE_RDRAND`, `X86_FEATURE_RDSEED`, and `pr_emerg`.

/*
 * RDRAND has Built-In-Self-Test (BIST) that runs on every invocation.
 * Run the instruction a few times as a sanity check. Also make sure
 * it's not outputting the same value over and over, which has happened
 * as a result of past CPU bugs.
 *
 * If it fails, it is simple to disable RDRAND and RDSEED here.
 */

pub unsafe fn x86_init_rdrand(c: *mut cpuinfo_x86) {
    const SAMPLES: usize = 8;
    const MIN_CHANGE: usize = 5;
    let mut sample: core::ffi::c_ulong = 0;
    let mut prev: core::ffi::c_ulong = 0;
    let mut failure = false;
    let mut i: usize;
    let mut changed: usize;

    if !cpu_has(c, X86_FEATURE_RDRAND) {
        return;
    }

    changed = 0;
    i = 0;
    while i < SAMPLES {
        if !rdrand_long(&mut sample) {
            failure = true;
            break;
        }
        if i != 0 && sample != prev {
            changed += 1;
        }
        prev = sample;
        i += 1;
    }
    if changed < MIN_CHANGE {
        failure = true;
    }

    if failure {
        clear_cpu_cap(c, X86_FEATURE_RDRAND);
        clear_cpu_cap(c, X86_FEATURE_RDSEED);
        pr_emerg!("RDRAND is not reliable on this platform; disabling.\n");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
