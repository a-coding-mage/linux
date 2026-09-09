/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Linux Security Module interface to other subsystems.
 * BPF may present a single u32 value.
 */

// C dependency: u32 is represented directly by Rust's u32 type.

#[repr(C)]
pub struct lsm_prop_bpf {
    // Preserves the C conditional field under CONFIG_BPF_LSM.
    #[cfg(feature = "CONFIG_BPF_LSM")]
    pub secid: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
