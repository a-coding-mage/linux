/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2025 Intel Corporation */

// Linux __le* types are represented by their underlying integer widths here.
pub const LIBIE_AQ_MAX_BUF_LEN: usize = 4096;

#[repr(C)]
pub struct libie_aqc_generic {
    pub param0: u32,
    pub param1: u32,
    pub addr_high: u32,
    pub addr_low: u32,
}

#[repr(C)]
pub struct libie_aqc_get_ver {
    pub rom_ver: u32,
    pub fw_build: u32,
    pub fw_branch: u8,
    pub fw_major: u8,
    pub fw_minor: u8,
    pub fw_patch: u8,
    pub api_branch: u8,
    pub api_major: u8,
    pub api_minor: u8,
    pub api_patch: u8,
}

#[repr(C)]
pub struct libie_aqc_driver_ver {
    pub major_ver: u8,
    pub minor_ver: u8,
    pub build_ver: u8,
    pub subbuild_ver: u8,
    pub reserved: [u8; 4],
    pub addr_high: u32,
    pub addr_low: u32,
}

#[repr(i32)]
pub enum libie_aq_res_id { LIBIE_AQC_RES_ID_NVM = 1, LIBIE_AQC_RES_ID_SDP = 2, LIBIE_AQC_RES_ID_CHNG_LOCK = 3, LIBIE_AQC_RES_ID_GLBL_LOCK = 4 }
#[repr(i32)]
pub enum libie_aq_res_access_type { LIBIE_AQC_RES_ACCESS_READ = 1, LIBIE_AQC_RES_ACCESS_WRITE = 2 }

pub const LIBIE_AQ_RES_NVM_READ_DFLT_TIMEOUT_MS: u32 = 3000;
pub const LIBIE_AQ_RES_NVM_WRITE_DFLT_TIMEOUT_MS: u32 = 180000;
pub const LIBIE_AQ_RES_CHNG_LOCK_DFLT_TIMEOUT_MS: u32 = 1000;
pub const LIBIE_AQ_RES_GLBL_LOCK_DFLT_TIMEOUT_MS: u32 = 3000;
pub const LIBIE_AQ_RES_GLBL_SUCCESS: u32 = 0;
pub const LIBIE_AQ_RES_GLBL_IN_PROG: u32 = 1;
pub const LIBIE_AQ_RES_GLBL_DONE: u32 = 2;

#[repr(C)]
pub struct libie_aqc_req_res { pub res_id: u16, pub access_type: u16, pub timeout: u32, pub res_number: u32, pub status: u16, pub reserved: [u8; 2] }
#[repr(C)]
pub struct libie_aqc_list_caps { pub cmd_flags: u8, pub pf_index: u8, pub reserved: [u8; 2], pub count: u32, pub addr_high: u32, pub addr_low: u32 }

pub const LIBIE_AQC_CAPS_SWITCH_MODE: u16 = 0x0001;
pub const LIBIE_AQC_CAPS_MNG_MODE: u16 = 0x0002;
pub const LIBIE_AQC_CAPS_NPAR_ACTIVE: u16 = 0x0003;
pub const LIBIE_AQC_CAPS_OS2BMC_CAP: u16 = 0x0004;
pub const LIBIE_AQC_CAPS_VALID_FUNCTIONS: u16 = 0x0005;
pub const LIBIE_AQC_MAX_VALID_FUNCTIONS: u16 = 0x8;
pub const LIBIE_AQC_CAPS_SRIOV: u16 = 0x0012;
pub const LIBIE_AQC_CAPS_VF: u16 = 0x0013;
pub const LIBIE_AQC_CAPS_VMDQ: u16 = 0x0014;
pub const LIBIE_AQC_CAPS_8021QBG: u16 = 0x0015;
pub const LIBIE_AQC_CAPS_8021QBR: u16 = 0x0016;
pub const LIBIE_AQC_CAPS_VSI: u16 = 0x0017;
pub const LIBIE_AQC_CAPS_DCB: u16 = 0x0018;
pub const LIBIE_AQC_CAPS_FCOE: u16 = 0x0021;
pub const LIBIE_AQC_CAPS_ISCSI: u16 = 0x0022;
pub const LIBIE_AQC_CAPS_RSS: u16 = 0x0040;
pub const LIBIE_AQC_CAPS_RXQS: u16 = 0x0041;
pub const LIBIE_AQC_CAPS_TXQS: u16 = 0x0042;
pub const LIBIE_AQC_CAPS_MSIX: u16 = 0x0043;
pub const LIBIE_AQC_CAPS_VF_MSIX: u16 = 0x0044;
pub const LIBIE_AQC_CAPS_FD: u16 = 0x0045;
pub const LIBIE_AQC_CAPS_1588: u16 = 0x0046;
pub const LIBIE_AQC_CAPS_MAX_MTU: u16 = 0x0047;
pub const LIBIE_AQC_CAPS_NVM_VER: u16 = 0x0048;
pub const LIBIE_AQC_CAPS_PENDING_NVM_VER: u16 = 0x0049;
pub const LIBIE_AQC_CAPS_OROM_VER: u16 = 0x004A;
pub const LIBIE_AQC_CAPS_PENDING_OROM_VER: u16 = 0x004B;
pub const LIBIE_AQC_CAPS_NET_VER: u16 = 0x004C;
pub const LIBIE_AQC_CAPS_PENDING_NET_VER: u16 = 0x004D;
pub const LIBIE_AQC_CAPS_RDMA: u16 = 0x0051;
pub const LIBIE_AQC_CAPS_LED: u16 = 0x0061;
pub const LIBIE_AQC_CAPS_SDP: u16 = 0x0062;
pub const LIBIE_AQC_CAPS_MDIO: u16 = 0x0063;
pub const LIBIE_AQC_CAPS_WSR_PROT: u16 = 0x0064;
pub const LIBIE_AQC_CAPS_SENSOR_READING: u16 = 0x0067;
pub const LIBIE_AQC_INLINE_IPSEC: u16 = 0x0070;
pub const LIBIE_AQC_CAPS_NUM_ENABLED_PORTS: u16 = 0x0072;
pub const LIBIE_AQC_CAPS_PCIE_RESET_AVOIDANCE: u16 = 0x0076;
pub const LIBIE_AQC_CAPS_POST_UPDATE_RESET_RESTRICT: u16 = 0x0077;
pub const LIBIE_AQC_CAPS_NVM_MGMT: u16 = 0x0080;
pub const LIBIE_AQC_CAPS_EXT_TOPO_DEV_IMG0: u16 = 0x0081;
pub const LIBIE_AQC_CAPS_EXT_TOPO_DEV_IMG1: u16 = 0x0082;
pub const LIBIE_AQC_CAPS_EXT_TOPO_DEV_IMG2: u16 = 0x0083;
pub const LIBIE_AQC_CAPS_EXT_TOPO_DEV_IMG3: u16 = 0x0084;
pub const LIBIE_AQC_CAPS_TX_SCHED_TOPO_COMP_MODE: u16 = 0x0085;
pub const LIBIE_AQC_CAPS_NAC_TOPOLOGY: u16 = 0x0087;
pub const LIBIE_AQC_CAPS_FW_LAG_SUPPORT: u16 = 0x0092;
pub const LIBIE_AQC_BIT_ROCEV2_LAG: u32 = 1 << 0;
pub const LIBIE_AQC_BIT_SRIOV_LAG: u32 = 1 << 1;
pub const LIBIE_AQC_BIT_SRIOV_AA_LAG: u32 = 1 << 2;
pub const LIBIE_AQC_CAPS_EEE: u16 = 0x009B;
pub const LIBIE_AQC_CAPS_FLEX10: u16 = 0x00F1;
pub const LIBIE_AQC_CAPS_CEM: u16 = 0x00F2;

#[repr(C)]
pub struct libie_aqc_list_caps_elem { pub cap: u16, pub major_ver: u8, pub minor_ver: u8, pub number: u32, pub logical_id: u32, pub phys_id: u32, pub rsvd1: u64, pub rsvd2: u64 }

#[repr(i32)]
pub enum libie_adminq_opc { libie_aqc_opc_fw_logs_config = 0xFF30, libie_aqc_opc_fw_logs_register = 0xFF31, libie_aqc_opc_fw_logs_query = 0xFF32, libie_aqc_opc_fw_logs_event = 0xFF33 }
#[repr(i32)]
pub enum libie_aqc_fw_logging_mod {
    LIBIE_AQC_FW_LOG_ID_GENERAL = 0, LIBIE_AQC_FW_LOG_ID_CTRL, LIBIE_AQC_FW_LOG_ID_LINK, LIBIE_AQC_FW_LOG_ID_LINK_TOPO, LIBIE_AQC_FW_LOG_ID_DNL, LIBIE_AQC_FW_LOG_ID_I2C, LIBIE_AQC_FW_LOG_ID_SDP, LIBIE_AQC_FW_LOG_ID_MDIO, LIBIE_AQC_FW_LOG_ID_ADMINQ, LIBIE_AQC_FW_LOG_ID_HDMA, LIBIE_AQC_FW_LOG_ID_LLDP, LIBIE_AQC_FW_LOG_ID_DCBX, LIBIE_AQC_FW_LOG_ID_DCB, LIBIE_AQC_FW_LOG_ID_XLR, LIBIE_AQC_FW_LOG_ID_NVM, LIBIE_AQC_FW_LOG_ID_AUTH, LIBIE_AQC_FW_LOG_ID_VPD, LIBIE_AQC_FW_LOG_ID_IOSF, LIBIE_AQC_FW_LOG_ID_PARSER, LIBIE_AQC_FW_LOG_ID_SW, LIBIE_AQC_FW_LOG_ID_SCHEDULER, LIBIE_AQC_FW_LOG_ID_TXQ, LIBIE_AQC_FW_LOG_ID_RSVD, LIBIE_AQC_FW_LOG_ID_POST, LIBIE_AQC_FW_LOG_ID_WATCHDOG, LIBIE_AQC_FW_LOG_ID_TASK_DISPATCH, LIBIE_AQC_FW_LOG_ID_MNG, LIBIE_AQC_FW_LOG_ID_SYNCE, LIBIE_AQC_FW_LOG_ID_HEALTH, LIBIE_AQC_FW_LOG_ID_TSDRV, LIBIE_AQC_FW_LOG_ID_PFREG, LIBIE_AQC_FW_LOG_ID_MDLVER, LIBIE_AQC_FW_LOG_ID_MAX
}

pub const LIBIE_AQC_FW_LOG_CONF_UART_EN: u8 = 1 << 0;
pub const LIBIE_AQC_FW_LOG_CONF_AQ_EN: u8 = 1 << 1;
pub const LIBIE_AQC_FW_LOG_QUERY_REGISTERED: u8 = 1 << 2;
pub const LIBIE_AQC_FW_LOG_CONF_SET_VALID: u8 = 1 << 3;
pub const LIBIE_AQC_FW_LOG_AQ_REGISTER: u8 = 1 << 0;
pub const LIBIE_AQC_FW_LOG_AQ_QUERY: u8 = 1 << 2;
pub const LIBIE_AQC_FW_LOG_MIN_RESOLUTION: u16 = 1;
pub const LIBIE_AQC_FW_LOG_MAX_RESOLUTION: u16 = 128;

#[repr(C)]
pub struct libie_aqc_fw_log_sync { pub fw_rt_lsb: u32 }
#[repr(C)]
pub struct libie_aqc_fw_log_cfg { pub log_resolution: u16, pub mdl_cnt: u16 }
#[repr(C)]
pub union libie_aqc_fw_log_ops { pub sync: libie_aqc_fw_log_sync, pub cfg: libie_aqc_fw_log_cfg }
#[repr(C)]
pub struct libie_aqc_fw_log { pub cmd_flags: u8, pub rsp_flag: u8, pub fw_rt_msb: u16, pub ops: libie_aqc_fw_log_ops, pub addr_high: u32, pub addr_low: u32 }
#[repr(C)]
pub struct libie_aqc_fw_log_cfg_resp { pub module_identifier: u16, pub log_level: u8, pub rsvd0: u8 }

#[repr(C)]
pub union libie_aq_desc_params { pub raw: [u8; 16], pub generic: libie_aqc_generic, pub get_ver: libie_aqc_get_ver, pub driver_ver: libie_aqc_driver_ver, pub res_owner: libie_aqc_req_res, pub get_cap: libie_aqc_list_caps, pub fw_log: libie_aqc_fw_log }
#[repr(C)]
pub struct libie_aq_desc { pub flags: u16, pub opcode: u16, pub datalen: u16, pub retval: u16, pub cookie_high: u32, pub cookie_low: u32, pub params: libie_aq_desc_params }

pub const LIBIE_AQ_LG_BUF: u32 = 512;
pub const LIBIE_AQ_FLAG_DD: u16 = 1 << 0;
pub const LIBIE_AQ_FLAG_CMP: u16 = 1 << 1;
pub const LIBIE_AQ_FLAG_ERR: u16 = 1 << 2;
pub const LIBIE_AQ_FLAG_VFE: u16 = 1 << 3;
pub const LIBIE_AQ_FLAG_LB: u16 = 1 << 9;
pub const LIBIE_AQ_FLAG_RD: u16 = 1 << 10;
pub const LIBIE_AQ_FLAG_VFC: u16 = 1 << 11;
pub const LIBIE_AQ_FLAG_BUF: u16 = 1 << 12;
pub const LIBIE_AQ_FLAG_SI: u16 = 1 << 13;
pub const LIBIE_AQ_FLAG_EI: u16 = 1 << 14;
pub const LIBIE_AQ_FLAG_FE: u16 = 1 << 15;

#[repr(i32)]
pub enum libie_aq_err { LIBIE_AQ_RC_OK = 0, LIBIE_AQ_RC_EPERM = 1, LIBIE_AQ_RC_ENOENT = 2, LIBIE_AQ_RC_ESRCH = 3, LIBIE_AQ_RC_EIO = 5, LIBIE_AQ_RC_EAGAIN = 8, LIBIE_AQ_RC_ENOMEM = 9, LIBIE_AQ_RC_EACCES = 10, LIBIE_AQ_RC_EBUSY = 12, LIBIE_AQ_RC_EEXIST = 13, LIBIE_AQ_RC_EINVAL = 14, LIBIE_AQ_RC_ENOSPC = 16, LIBIE_AQ_RC_ENOSYS = 17, LIBIE_AQ_RC_EMODE = 21, LIBIE_AQ_RC_ENOSEC = 24, LIBIE_AQ_RC_EBADSIG = 25, LIBIE_AQ_RC_ESVN = 26, LIBIE_AQ_RC_EBADMAN = 27, LIBIE_AQ_RC_EBADBUF = 28 }

pub unsafe fn libie_aq_raw(desc: *mut libie_aq_desc) -> *mut u8 { (*desc).params.raw.as_mut_ptr() }

unsafe extern "C" { pub fn libie_aq_str(err: libie_aq_err) -> *const core::ffi::c_char; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
