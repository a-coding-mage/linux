/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by other translation units.
#[repr(C)]
pub struct ctl_table_header {
    _private: [u8; 0],
}

#[repr(C)]
pub struct prot_inuse {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netns_core {
    /* core sysctls */
    pub sysctl_hdr: *mut ctl_table_header,

    pub sysctl_somaxconn: ::core::ffi::c_int,
    pub sysctl_txq_reselection: ::core::ffi::c_int,
    pub sysctl_optmem_max: ::core::ffi::c_int,
    pub sysctl_txrehash: u8,
    pub sysctl_tstamp_allow_data: u8,
    pub sysctl_bypass_prot_mem: u8,

    // CONFIG_PROC_FS
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub prot_inuse: *mut prot_inuse,

    // IS_ENABLED(CONFIG_RPS) && IS_ENABLED(CONFIG_SYSCTL)
    #[cfg(all(feature = "CONFIG_RPS", feature = "CONFIG_SYSCTL"))]
    pub rps_default_mask: *mut cpumask,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
