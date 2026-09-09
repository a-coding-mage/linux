/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Common time accounting prototypes and such for all ppc machines.
 */

/* Stuff for accurate time accounting */
#[repr(C)]
pub struct cpu_accounting_data {
    /* Accumulated cputime values to flush on ticks */
    pub utime: core::ffi::c_ulong,
    pub stime: core::ffi::c_ulong,
    #[cfg(CONFIG_ARCH_HAS_SCALED_CPUTIME)]
    pub utime_scaled: core::ffi::c_ulong,
    #[cfg(CONFIG_ARCH_HAS_SCALED_CPUTIME)]
    pub stime_scaled: core::ffi::c_ulong,
    pub gtime: core::ffi::c_ulong,
    pub hardirq_time: core::ffi::c_ulong,
    pub softirq_time: core::ffi::c_ulong,
    pub steal_time: core::ffi::c_ulong,
    pub idle_time: core::ffi::c_ulong,
    /* Internal counters */
    pub starttime: core::ffi::c_ulong, /* TB value snapshot */
    pub starttime_user: core::ffi::c_ulong, /* TB value on exit to usermode */
    #[cfg(CONFIG_ARCH_HAS_SCALED_CPUTIME)]
    pub startspurr: core::ffi::c_ulong, /* SPURR value snapshot */
    #[cfg(CONFIG_ARCH_HAS_SCALED_CPUTIME)]
    pub utime_sspurr: core::ffi::c_ulong, /* ->user_time when ->startspurr set */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
