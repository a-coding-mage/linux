/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies provided by the original C includes:
// #include <linux/spinlock.h>
// #include <asm/prom.h>

extern "C" {
    pub fn of_console_init();

    pub static mut prom_early_allocated: ::core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
