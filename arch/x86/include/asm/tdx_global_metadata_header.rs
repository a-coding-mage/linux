/* SPDX-License-Identifier: GPL-2.0 */
/* Automatically generated TDX global metadata structures. */

#[repr(C)]
pub struct tdx_sys_info_version {
    pub minor_version: u16,
    pub major_version: u16,
    pub update_version: u16,
}

#[repr(C)]
pub struct tdx_sys_info_features {
    pub tdx_features0: u64,
}

#[repr(C)]
pub struct tdx_sys_info_tdmr {
    pub max_tdmrs: u16,
    pub max_reserved_per_tdmr: u16,
    pub pamt_4k_entry_size: u16,
    pub pamt_2m_entry_size: u16,
    pub pamt_1g_entry_size: u16,
}

#[repr(C)]
pub struct tdx_sys_info_td_ctrl {
    pub tdr_base_size: u16,
    pub tdcs_base_size: u16,
    pub tdvps_base_size: u16,
}

#[repr(C)]
pub struct tdx_sys_info_td_conf {
    pub attributes_fixed0: u64,
    pub attributes_fixed1: u64,
    pub xfam_fixed0: u64,
    pub xfam_fixed1: u64,
    pub num_cpuid_config: u16,
    pub max_vcpus_per_td: u16,
    pub cpuid_config_leaves: [u64; 128],
    pub cpuid_config_values: [[u64; 2]; 128],
}

#[repr(C)]
pub struct tdx_sys_info_handoff {
    pub module_hv: u16,
}

#[repr(C)]
pub struct tdx_sys_info {
    pub version: tdx_sys_info_version,
    pub features: tdx_sys_info_features,
    pub tdmr: tdx_sys_info_tdmr,
    pub td_ctrl: tdx_sys_info_td_ctrl,
    pub td_conf: tdx_sys_info_td_conf,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
