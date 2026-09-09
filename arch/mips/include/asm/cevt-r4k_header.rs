/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Kevin D. Kissell
 */

/*
 * Definitions used for common event timer implementation
 * for MIPS 4K-type processors and their MIPS MT variants.
 * Avoids unsightly extern declarations in C files.
 */

// Dependency corresponding to <linux/clockchips.h>.
// Dependency corresponding to <asm/time.h>.

// DECLARE_PER_CPU(struct clock_event_device, mips_clockevent_device);
// Per-CPU storage for this externally defined clock event device.
extern "C" {
    pub static mut mips_clockevent_device: clock_event_device;

    pub fn mips_event_handler(dev: *mut clock_event_device);
    pub fn c0_compare_int_usable() -> ::core::ffi::c_int;
    pub fn c0_compare_interrupt(
        irq: ::core::ffi::c_int,
        dev_id: *mut ::core::ffi::c_void,
    ) -> irqreturn_t;

    pub static mut cp0_timer_irq_installed: ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
