/*
 * arch/arm/plat-orion/include/plat/time.h
 *
 * Marvell Orion SoC time handling.
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// C header guard: __PLAT_TIME_H

extern "C" {
    pub fn orion_time_set_base(timer_base: *mut core::ffi::c_void);

    pub fn orion_time_init(
        bridge_base: *mut core::ffi::c_void,
        bridge_timer1_clr_mask: u32,
        irq: ::core::ffi::c_uint,
        tclk: ::core::ffi::c_uint,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
