/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Translated from ASM_TIME_H. The C header guard is omitted from Rust code.

extern "C" {
    pub static mut pcycle_freq_mhz: cycles_t;
    pub static mut thread_freq_mhz: cycles_t;
    pub static mut sleep_clk_freq: cycles_t;

    pub fn setup_percpu_clockdev();
    pub fn ipi_timer();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
