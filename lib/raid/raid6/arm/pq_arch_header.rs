/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency supplied by the surrounding translation unit:
// #include <asm/neon.h>
use crate::{raid6_algo_add, raid6_algo_add_default, raid6_calls, raid6_recov_algo_add,
    raid6_recov_calls, cpu_has_neon};

extern "C" {
    pub static raid6_neonx1: raid6_calls;
    pub static raid6_neonx2: raid6_calls;
    pub static raid6_neonx4: raid6_calls;
    pub static raid6_neonx8: raid6_calls;
    pub static raid6_recov_neon: raid6_recov_calls;
}

#[inline(always)]
pub unsafe fn arch_raid6_init() {
    raid6_algo_add_default();
    if cpu_has_neon() {
        raid6_algo_add(&raid6_neonx1);
        raid6_algo_add(&raid6_neonx2);
        raid6_algo_add(&raid6_neonx4);
        raid6_algo_add(&raid6_neonx8);
        raid6_recov_algo_add(&raid6_recov_neon);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
