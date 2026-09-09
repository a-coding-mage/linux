/* SPDX-License-Identifier: GPL-2.0 */
// Dependency intent: declarations from <linux/reboot.h> are supplied externally.

unsafe extern "C" {
    pub fn mmp_timer_init(irq: ::core::ffi::c_int, rate: ::core::ffi::c_ulong);

    // The C __init annotation is a linker/build attribute and has no direct
    // file-local Rust equivalent.
    pub fn mmp_map_io();
    pub fn mmp2_map_io();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
