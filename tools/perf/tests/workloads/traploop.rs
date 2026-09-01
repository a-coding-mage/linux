// SPDX-License-Identifier: GPL-2.0
// C dependencies: <stdlib.h>, "../tests.h"

use core::arch::asm;
use core::ffi::{c_char, c_int};

const BENCH_RUNS: c_int = 999999;

unsafe extern "C" {
    fn atoi(nptr: *const c_char) -> c_int;
}

#[cfg(target_arch = "aarch64")]
unsafe fn trap_bench() {
    let val: core::ffi::c_ulong;

    asm!("mrs {0}, ID_AA64ISAR0_EL1", out(reg) val); /* TRAP + ERET */
}

#[cfg(not(target_arch = "aarch64"))]
unsafe fn trap_bench() {}

unsafe fn traploop(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut num_loops: c_int = BENCH_RUNS;

    if argc > 0 {
        num_loops = atoi(*argv.offset(0));
    }

    let mut i: c_int = 0;
    while i < num_loops {
        trap_bench();
        i += 1;
    }

    0
}

// External test harness macro from "../tests.h": DEFINE_WORKLOAD(traploop);
DEFINE_WORKLOAD!(traploop);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
