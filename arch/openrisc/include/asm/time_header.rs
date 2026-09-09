/*
 * OpenRISC timer API
 *
 * Copyright (C) 2017 by Stafford Horne (shorne@gmail.com)
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

extern "C" {
    pub fn openrisc_clockevent_init();

    pub fn openrisc_timer_set(count: core::ffi::c_ulong);
    pub fn openrisc_timer_set_next(delta: core::ffi::c_ulong);

    // Preserved from the CONFIG_SMP build-time condition in the C header.
    #[cfg(feature = "CONFIG_SMP")]
    pub fn synchronise_count_master(cpu: core::ffi::c_int);

    #[cfg(feature = "CONFIG_SMP")]
    pub fn synchronise_count_slave(cpu: core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
