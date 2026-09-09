// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// External declarations supplied by the Linux clock and timer subsystems.
unsafe extern "C" {
    fn of_clk_init(clk_match: *const core::ffi::c_void);
    fn timer_probe();
}

// __init
pub unsafe fn time_init() {
    of_clk_init(core::ptr::null());
    timer_probe();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
