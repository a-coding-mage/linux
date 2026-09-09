/* SPDX-License-Identifier: GPL-2.0-only */

/* q40ints.c */
extern "C" {
    pub fn q40_init_IRQ();
    pub fn q40_mksound(hz: ::core::ffi::c_uint, ticks: ::core::ffi::c_uint);
    pub fn q40_sched_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
