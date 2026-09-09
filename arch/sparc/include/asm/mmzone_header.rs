/* SPDX-License-Identifier: GPL-2.0 */

// The declarations below are present when CONFIG_NUMA is enabled.
#[cfg(feature = "CONFIG_NUMA")]
unsafe extern "C" {
    pub static mut numa_cpu_lookup_table: [core::ffi::c_int; 0];
    pub static mut numa_cpumask_lookup_table: [crate::cpumask_t; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
