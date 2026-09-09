/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Translated from icp_qat_fw_init_admin.h. The included icp_qat_fw.h symbols
// (including __u8/__u16/__u32/__u64 and ICP_QAT_FW_NUM_LONGWORDS_4) are
// provided by the surrounding translation unit.

pub const RL_MAX_RP_IDS: usize = 16;

#[repr(u32)]
pub enum icp_qat_fw_init_admin_cmd_id {
    ICP_QAT_FW_INIT_AE = 0,
    ICP_QAT_FW_TRNG_ENABLE = 1,
    ICP_QAT_FW_TRNG_DISABLE = 2,
    ICP_QAT_FW_CONSTANTS_CFG = 3,
    ICP_QAT_FW_STATUS_GET = 4,
    ICP_QAT_FW_COUNTERS_GET = 5,
    ICP_QAT_FW_LOOPBACK = 6,
    ICP_QAT_FW_HEARTBEAT_SYNC = 7,
    ICP_QAT_FW_HEARTBEAT_GET = 8,
    ICP_QAT_FW_COMP_CAPABILITY_GET = 9,
    ICP_QAT_FW_CRYPTO_CAPABILITY_GET = 10,
    ICP_QAT_FW_DC_CHAIN_INIT = 11,
    ICP_QAT_FW_HEARTBEAT_TIMER_SET = 13,
    ICP_QAT_FW_RL_INIT = 15,
    ICP_QAT_FW_TIMER_GET = 19,
    ICP_QAT_FW_CNV_STATS_GET = 20,
    ICP_QAT_FW_PM_STATE_CONFIG = 128,
    ICP_QAT_FW_PM_INFO = 129,
    ICP_QAT_FW_RL_ADD = 134,
    ICP_QAT_FW_RL_UPDATE = 135,
    ICP_QAT_FW_RL_REMOVE = 136,
    ICP_QAT_FW_TL_START = 137,
    ICP_QAT_FW_TL_STOP = 138,
    ICP_QAT_FW_KPT_ENABLE = 144,
    ICP_QAT_FW_SVN_READ = 146,
    ICP_QAT_FW_SVN_COMMIT = 147,
}

#[repr(u32)]
pub enum icp_qat_fw_init_admin_resp_status {
    ICP_QAT_FW_INIT_RESP_STATUS_SUCCESS = 0,
    ICP_QAT_FW_INIT_RESP_STATUS_FAIL = 1,
    ICP_QAT_FW_INIT_RESP_STATUS_RETRY = 2,
    ICP_QAT_FW_INIT_RESP_STATUS_UNSUPPORTED = 4,
}

#[repr(C)]
pub struct icp_qat_fw_init_admin_tl_rp_indexes { pub rp_num_index_0: __u8, pub rp_num_index_1: __u8, pub rp_num_index_2: __u8, pub rp_num_index_3: __u8 }

#[repr(C)]
pub struct icp_qat_fw_init_admin_slice_cnt {
    pub cpr_cnt: __u8, pub xlt_cnt: __u8, pub dcpr_cnt: __u8, pub pke_cnt: __u8,
    pub wat_cnt: __u8, pub wcp_cnt: __u8, pub ucs_cnt: __u8, pub cph_cnt: __u8,
    pub ath_cnt: __u8,
}

#[repr(C)]
pub struct icp_qat_fw_init_admin_sla_config_params {
    pub pcie_in_cir: __u32, pub pcie_in_pir: __u32, pub pcie_out_cir: __u32, pub pcie_out_pir: __u32,
    pub slice_util_cir: __u32, pub slice_util_pir: __u32, pub ae_util_cir: __u32, pub ae_util_pir: __u32,
    pub rp_ids: [__u16; RL_MAX_RP_IDS],
}

#[repr(C)]
pub union icp_qat_fw_init_admin_req_data {
    pub ibuf: icp_qat_fw_init_admin_req_ibuf,
    pub int_timer_ticks: __u32,
    pub heartbeat_ticks: __u32,
    pub node: icp_qat_fw_init_admin_req_node,
    pub idle_filter: __u32,
    pub rp_indexes: icp_qat_fw_init_admin_tl_rp_indexes,
}
#[repr(C)] pub struct icp_qat_fw_init_admin_req_ibuf { pub ibuf_size_in_kb: __u16, pub resrvd3: __u16 }
#[repr(C)] pub struct icp_qat_fw_init_admin_req_node { pub node_id: __u16, pub node_type: __u8, pub svc_type: __u8, pub resrvd5: [__u8; 3], pub rp_count: __u8 }

#[repr(C, packed)]
pub struct icp_qat_fw_init_admin_req {
    pub init_cfg_sz: __u16, pub resrvd1: __u8, pub cmd_id: __u8, pub resrvd2: __u32,
    pub opaque_data: __u64, pub init_cfg_ptr: __u64, pub data: icp_qat_fw_init_admin_req_data,
    pub resrvd4: __u32,
}

#[repr(C)]
pub union icp_qat_fw_init_admin_resp_status_data {
    pub resrvd2: __u32,
    pub version: icp_qat_fw_init_admin_resp_version,
    pub extended_features: __u32,
    pub errors: icp_qat_fw_init_admin_resp_errors,
}
#[repr(C)] pub struct icp_qat_fw_init_admin_resp_version { pub version_minor_num: __u16, pub version_major_num: __u16 }
#[repr(C)] pub struct icp_qat_fw_init_admin_resp_errors { pub error_count: __u16, pub latest_error: __u16 }

#[repr(C)]
pub union icp_qat_fw_init_admin_resp_data {
    pub resrvd3: [__u32; ICP_QAT_FW_NUM_LONGWORDS_4 as usize],
    pub version_info: icp_qat_fw_init_admin_resp_version_info,
    pub counters: icp_qat_fw_init_admin_resp_counters,
    pub comp_capabilities: icp_qat_fw_init_admin_resp_comp_capabilities,
    pub crypto_capabilities: icp_qat_fw_init_admin_resp_crypto_capabilities,
    pub timestamp: icp_qat_fw_init_admin_resp_timestamp,
    pub counts: icp_qat_fw_init_admin_resp_counts,
    pub slices: icp_qat_fw_init_admin_slice_cnt,
    pub fw_capabilities: __u16,
    pub svn: icp_qat_fw_init_admin_resp_svn,
}
#[repr(C)] pub struct icp_qat_fw_init_admin_resp_version_info { pub version_patch_num: __u32, pub context_id: __u8, pub ae_id: __u8, pub resrvd4: __u16, pub resrvd5: __u64 }
#[repr(C)] pub struct icp_qat_fw_init_admin_resp_counters { pub req_rec_count: __u64, pub resp_sent_count: __u64 }
#[repr(C)] pub struct icp_qat_fw_init_admin_resp_comp_capabilities { pub compression_algos: __u16, pub checksum_algos: __u16, pub deflate_capabilities: __u32, pub resrvd6: __u32, pub lzs_capabilities: __u32 }
#[repr(C)] pub struct icp_qat_fw_init_admin_resp_crypto_capabilities { pub cipher_algos: __u32, pub hash_algos: __u32, pub keygen_algos: __u16, pub other: __u16, pub public_key_algos: __u16, pub prime_algos: __u16 }
#[repr(C)] pub struct icp_qat_fw_init_admin_resp_timestamp { pub timestamp: __u64, pub resrvd7: __u64 }
#[repr(C)] pub struct icp_qat_fw_init_admin_resp_counts { pub successful_count: __u32, pub unsuccessful_count: __u32, pub resrvd8: __u64 }
#[repr(C)] pub struct icp_qat_fw_init_admin_resp_svn { pub enforced_min_svn: __u8, pub permanent_min_svn: __u8, pub active_svn: __u8, pub resrvd9: __u8, pub svn_status: __u16, pub resrvd10: __u16, pub resrvd11: __u64 }

#[repr(C, packed)]
pub struct icp_qat_fw_init_admin_resp { pub flags: __u8, pub resrvd1: __u8, pub status: __u8, pub cmd_id: __u8, pub status_data: icp_qat_fw_init_admin_resp_status_data, pub opaque_data: __u64, pub data: icp_qat_fw_init_admin_resp_data }

pub const ICP_QAT_FW_SYNC: u32 = ICP_QAT_FW_HEARTBEAT_SYNC as u32;
pub const ICP_QAT_FW_CAPABILITIES_GET: u32 = ICP_QAT_FW_CRYPTO_CAPABILITY_GET as u32;
pub const ICP_QAT_NUMBER_OF_PM_EVENTS: usize = 8;

#[repr(C)]
pub struct icp_qat_fw_init_admin_pm_info {
    pub max_pwrreq: __u16, pub min_pwrreq: __u16, pub resvrd1: __u16, pub pwr_state: __u8, pub resvrd2: __u8, pub fusectl0: __u32,
    pub sys_pm: __u32, pub host_msg: __u32, pub unknown: __u32, pub local_ssm: __u32, pub timer: __u32,
    pub event_log: [__u32; ICP_QAT_NUMBER_OF_PM_EVENTS],
    pub fw_init: __u32, pub pwrreq: __u32, pub status: __u32, pub main: __u32, pub thread: __u32,
    pub pm_enable: __u32, pub pm_active_status: __u32, pub pm_managed_status: __u32, pub pm_domain_status: __u32, pub active_constraint: __u32,
    pub resvrd3: [__u32; 6],
}

#[repr(C)]
pub struct icp_qat_fw_init_admin_kpt_cfg { pub swk_cnt_per_fn: __u32, pub swk_cnt_per_pasid: __u32, pub swk_ttl_in_secs: __u32, pub swk_shared_disable: __u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
