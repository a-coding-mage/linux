/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright (c) 2008-2011, Intel Corporation. */

// Translated from linux/dcbnl.h.  The Linux __u* types are represented by
// their Rust fixed-width equivalents.

pub const IEEE_8021QAZ_MAX_TCS: usize = 8;
pub const IEEE_8021QAZ_TSA_STRICT: u8 = 0;
pub const IEEE_8021QAZ_TSA_CB_SHAPER: u8 = 1;
pub const IEEE_8021QAZ_TSA_ETS: u8 = 2;
pub const IEEE_8021QAZ_TSA_VENDOR: u8 = 255;

#[repr(C)]
pub struct ieee_ets {
    pub willing: u8,
    pub ets_cap: u8,
    pub cbs: u8,
    pub tc_tx_bw: [u8; IEEE_8021QAZ_MAX_TCS],
    pub tc_rx_bw: [u8; IEEE_8021QAZ_MAX_TCS],
    pub tc_tsa: [u8; IEEE_8021QAZ_MAX_TCS],
    pub prio_tc: [u8; IEEE_8021QAZ_MAX_TCS],
    pub tc_reco_bw: [u8; IEEE_8021QAZ_MAX_TCS],
    pub tc_reco_tsa: [u8; IEEE_8021QAZ_MAX_TCS],
    pub reco_prio_tc: [u8; IEEE_8021QAZ_MAX_TCS],
}

#[repr(C)]
pub struct ieee_maxrate {
    pub tc_maxrate: [u64; IEEE_8021QAZ_MAX_TCS],
}

#[repr(i32)]
pub enum dcbnl_cndd_states { DCB_CNDD_RESET = 0, DCB_CNDD_EDGE, DCB_CNDD_INTERIOR, DCB_CNDD_INTERIOR_READY }

#[repr(C)]
pub struct ieee_qcn {
    pub rpg_enable: [u8; IEEE_8021QAZ_MAX_TCS],
    pub rppp_max_rps: [u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_time_reset: [u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_byte_reset: [u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_threshold: [u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_max_rate: [u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_ai_rate: [u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_hai_rate: [u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_gd: [u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_min_dec_fac: [u32; IEEE_8021QAZ_MAX_TCS],
    pub rpg_min_rate: [u32; IEEE_8021QAZ_MAX_TCS],
    pub cndd_state_machine: [u32; IEEE_8021QAZ_MAX_TCS],
}

#[repr(C)]
pub struct ieee_qcn_stats {
    pub rppp_rp_centiseconds: [u64; IEEE_8021QAZ_MAX_TCS],
    pub rppp_created_rps: [u32; IEEE_8021QAZ_MAX_TCS],
}

#[repr(C)]
pub struct ieee_pfc {
    pub pfc_cap: u8, pub pfc_en: u8, pub mbc: u8, pub delay: u16,
    pub requests: [u64; IEEE_8021QAZ_MAX_TCS],
    pub indications: [u64; IEEE_8021QAZ_MAX_TCS],
}

pub const IEEE_8021Q_MAX_PRIORITIES: usize = 8;
pub const DCBX_MAX_BUFFERS: usize = 8;
#[repr(C)]
pub struct dcbnl_buffer {
    pub prio2buffer: [u8; IEEE_8021Q_MAX_PRIORITIES],
    pub buffer_size: [u32; DCBX_MAX_BUFFERS],
    pub total_size: u32,
}

pub const CEE_DCBX_MAX_PGS: usize = 8;
pub const CEE_DCBX_MAX_PRIO: usize = 8;
#[repr(C)]
pub struct cee_pg { pub willing: u8, pub error: u8, pub pg_en: u8, pub tcs_supported: u8, pub pg_bw: [u8; CEE_DCBX_MAX_PGS], pub prio_pg: [u8; CEE_DCBX_MAX_PGS] }
#[repr(C)]
pub struct cee_pfc { pub willing: u8, pub error: u8, pub pfc_en: u8, pub tcs_supported: u8 }

pub const IEEE_8021QAZ_APP_SEL_ETHERTYPE: u8 = 1;
pub const IEEE_8021QAZ_APP_SEL_STREAM: u8 = 2;
pub const IEEE_8021QAZ_APP_SEL_DGRAM: u8 = 3;
pub const IEEE_8021QAZ_APP_SEL_ANY: u8 = 4;
pub const IEEE_8021QAZ_APP_SEL_DSCP: u8 = 5;
pub const DCB_APP_SEL_PCP: u8 = 255;
#[repr(C)]
pub struct dcb_app { pub selector: u8, pub priority: u8, pub protocol: u16 }
pub const IEEE_8021QAZ_APP_SEL_MAX: u8 = 255;
#[repr(C)]
pub struct dcb_peer_app_info { pub willing: u8, pub error: u8 }
#[repr(C)]
pub struct dcbmsg { pub dcb_family: u8, pub cmd: u8, pub dcb_pad: u16 }

#[repr(i32)]
pub enum dcbnl_commands {
    DCB_CMD_UNDEFINED, DCB_CMD_GSTATE, DCB_CMD_SSTATE, DCB_CMD_PGTX_GCFG, DCB_CMD_PGTX_SCFG,
    DCB_CMD_PGRX_GCFG, DCB_CMD_PGRX_SCFG, DCB_CMD_PFC_GCFG, DCB_CMD_PFC_SCFG, DCB_CMD_SET_ALL,
    DCB_CMD_GPERM_HWADDR, DCB_CMD_GCAP, DCB_CMD_GNUMTCS, DCB_CMD_SNUMTCS, DCB_CMD_PFC_GSTATE,
    DCB_CMD_PFC_SSTATE, DCB_CMD_BCN_GCFG, DCB_CMD_BCN_SCFG, DCB_CMD_GAPP, DCB_CMD_SAPP,
    DCB_CMD_IEEE_SET, DCB_CMD_IEEE_GET, DCB_CMD_GDCBX, DCB_CMD_SDCBX, DCB_CMD_GFEATCFG,
    DCB_CMD_SFEATCFG, DCB_CMD_CEE_GET, DCB_CMD_IEEE_DEL, __DCB_CMD_ENUM_MAX,
    DCB_CMD_MAX = __DCB_CMD_ENUM_MAX as isize - 1,
}

#[repr(i32)]
pub enum dcbnl_attrs {
    DCB_ATTR_UNDEFINED, DCB_ATTR_IFNAME, DCB_ATTR_STATE, DCB_ATTR_PFC_STATE, DCB_ATTR_PFC_CFG,
    DCB_ATTR_NUM_TC, DCB_ATTR_PG_CFG, DCB_ATTR_SET_ALL, DCB_ATTR_PERM_HWADDR, DCB_ATTR_CAP,
    DCB_ATTR_NUMTCS, DCB_ATTR_BCN, DCB_ATTR_APP, DCB_ATTR_IEEE, DCB_ATTR_DCBX, DCB_ATTR_FEATCFG,
    DCB_ATTR_CEE, __DCB_ATTR_ENUM_MAX, DCB_ATTR_MAX = __DCB_ATTR_ENUM_MAX as isize - 1,
}

#[repr(i32)]
pub enum ieee_attrs {
    DCB_ATTR_IEEE_UNSPEC, DCB_ATTR_IEEE_ETS, DCB_ATTR_IEEE_PFC, DCB_ATTR_IEEE_APP_TABLE,
    DCB_ATTR_IEEE_PEER_ETS, DCB_ATTR_IEEE_PEER_PFC, DCB_ATTR_IEEE_PEER_APP, DCB_ATTR_IEEE_MAXRATE,
    DCB_ATTR_IEEE_QCN, DCB_ATTR_IEEE_QCN_STATS, DCB_ATTR_DCB_BUFFER, DCB_ATTR_DCB_APP_TRUST_TABLE,
    DCB_ATTR_DCB_REWR_TABLE, __DCB_ATTR_IEEE_MAX,
}
pub const DCB_ATTR_IEEE_MAX: isize = __DCB_ATTR_IEEE_MAX as isize - 1;

#[repr(i32)]
pub enum ieee_attrs_app { DCB_ATTR_IEEE_APP_UNSPEC, DCB_ATTR_IEEE_APP, DCB_ATTR_DCB_APP, __DCB_ATTR_IEEE_APP_MAX }
pub const DCB_ATTR_IEEE_APP_MAX: isize = __DCB_ATTR_IEEE_APP_MAX as isize - 1;

#[repr(i32)]
pub enum cee_attrs { DCB_ATTR_CEE_UNSPEC, DCB_ATTR_CEE_PEER_PG, DCB_ATTR_CEE_PEER_PFC, DCB_ATTR_CEE_PEER_APP_TABLE, DCB_ATTR_CEE_TX_PG, DCB_ATTR_CEE_RX_PG, DCB_ATTR_CEE_PFC, DCB_ATTR_CEE_APP_TABLE, DCB_ATTR_CEE_FEAT, __DCB_ATTR_CEE_MAX }
pub const DCB_ATTR_CEE_MAX: isize = __DCB_ATTR_CEE_MAX as isize - 1;
#[repr(i32)]
pub enum peer_app_attr { DCB_ATTR_CEE_PEER_APP_UNSPEC, DCB_ATTR_CEE_PEER_APP_INFO, DCB_ATTR_CEE_PEER_APP, __DCB_ATTR_CEE_PEER_APP_MAX }
pub const DCB_ATTR_CEE_PEER_APP_MAX: isize = __DCB_ATTR_CEE_PEER_APP_MAX as isize - 1;
#[repr(i32)]
pub enum cee_attrs_app { DCB_ATTR_CEE_APP_UNSPEC, DCB_ATTR_CEE_APP, __DCB_ATTR_CEE_APP_MAX }
pub const DCB_ATTR_CEE_APP_MAX: isize = __DCB_ATTR_CEE_APP_MAX as isize - 1;

#[repr(i32)]
pub enum dcbnl_pfc_up_attrs { DCB_PFC_UP_ATTR_UNDEFINED, DCB_PFC_UP_ATTR_0, DCB_PFC_UP_ATTR_1, DCB_PFC_UP_ATTR_2, DCB_PFC_UP_ATTR_3, DCB_PFC_UP_ATTR_4, DCB_PFC_UP_ATTR_5, DCB_PFC_UP_ATTR_6, DCB_PFC_UP_ATTR_7, DCB_PFC_UP_ATTR_ALL, __DCB_PFC_UP_ATTR_ENUM_MAX }
pub const DCB_PFC_UP_ATTR_MAX: isize = __DCB_PFC_UP_ATTR_ENUM_MAX as isize - 1;

#[repr(i32)]
pub enum dcbnl_pg_attrs {
    DCB_PG_ATTR_UNDEFINED, DCB_PG_ATTR_TC_0, DCB_PG_ATTR_TC_1, DCB_PG_ATTR_TC_2, DCB_PG_ATTR_TC_3,
    DCB_PG_ATTR_TC_4, DCB_PG_ATTR_TC_5, DCB_PG_ATTR_TC_6, DCB_PG_ATTR_TC_7, DCB_PG_ATTR_TC_MAX,
    DCB_PG_ATTR_TC_ALL, DCB_PG_ATTR_BW_ID_0, DCB_PG_ATTR_BW_ID_1, DCB_PG_ATTR_BW_ID_2,
    DCB_PG_ATTR_BW_ID_3, DCB_PG_ATTR_BW_ID_4, DCB_PG_ATTR_BW_ID_5, DCB_PG_ATTR_BW_ID_6,
    DCB_PG_ATTR_BW_ID_7, DCB_PG_ATTR_BW_ID_MAX, DCB_PG_ATTR_BW_ID_ALL, __DCB_PG_ATTR_ENUM_MAX,
}
pub const DCB_PG_ATTR_MAX: isize = __DCB_PG_ATTR_ENUM_MAX as isize - 1;

#[repr(i32)]
pub enum dcbnl_tc_attrs { DCB_TC_ATTR_PARAM_UNDEFINED, DCB_TC_ATTR_PARAM_PGID, DCB_TC_ATTR_PARAM_UP_MAPPING, DCB_TC_ATTR_PARAM_STRICT_PRIO, DCB_TC_ATTR_PARAM_BW_PCT, DCB_TC_ATTR_PARAM_ALL, __DCB_TC_ATTR_PARAM_ENUM_MAX }
pub const DCB_TC_ATTR_PARAM_MAX: isize = __DCB_TC_ATTR_PARAM_ENUM_MAX as isize - 1;
#[repr(i32)]
pub enum dcbnl_cap_attrs { DCB_CAP_ATTR_UNDEFINED, DCB_CAP_ATTR_ALL, DCB_CAP_ATTR_PG, DCB_CAP_ATTR_PFC, DCB_CAP_ATTR_UP2TC, DCB_CAP_ATTR_PG_TCS, DCB_CAP_ATTR_PFC_TCS, DCB_CAP_ATTR_GSP, DCB_CAP_ATTR_BCN, DCB_CAP_ATTR_DCBX, __DCB_CAP_ATTR_ENUM_MAX }
pub const DCB_CAP_ATTR_MAX: isize = __DCB_CAP_ATTR_ENUM_MAX as isize - 1;

pub const DCB_CAP_DCBX_HOST: u8 = 0x01;
pub const DCB_CAP_DCBX_LLD_MANAGED: u8 = 0x02;
pub const DCB_CAP_DCBX_VER_CEE: u8 = 0x04;
pub const DCB_CAP_DCBX_VER_IEEE: u8 = 0x08;
pub const DCB_CAP_DCBX_STATIC: u8 = 0x10;

#[repr(i32)]
pub enum dcbnl_numtcs_attrs { DCB_NUMTCS_ATTR_UNDEFINED, DCB_NUMTCS_ATTR_ALL, DCB_NUMTCS_ATTR_PG, DCB_NUMTCS_ATTR_PFC, __DCB_NUMTCS_ATTR_ENUM_MAX }
pub const DCB_NUMTCS_ATTR_MAX: isize = __DCB_NUMTCS_ATTR_ENUM_MAX as isize - 1;
#[repr(i32)]
pub enum dcbnl_bcn_attrs { DCB_BCN_ATTR_UNDEFINED = 0, DCB_BCN_ATTR_RP_0, DCB_BCN_ATTR_RP_1, DCB_BCN_ATTR_RP_2, DCB_BCN_ATTR_RP_3, DCB_BCN_ATTR_RP_4, DCB_BCN_ATTR_RP_5, DCB_BCN_ATTR_RP_6, DCB_BCN_ATTR_RP_7, DCB_BCN_ATTR_RP_ALL, DCB_BCN_ATTR_BCNA_0, DCB_BCN_ATTR_BCNA_1, DCB_BCN_ATTR_ALPHA, DCB_BCN_ATTR_BETA, DCB_BCN_ATTR_GD, DCB_BCN_ATTR_GI, DCB_BCN_ATTR_TMAX, DCB_BCN_ATTR_TD, DCB_BCN_ATTR_RMIN, DCB_BCN_ATTR_W, DCB_BCN_ATTR_RD, DCB_BCN_ATTR_RU, DCB_BCN_ATTR_WRTT, DCB_BCN_ATTR_RI, DCB_BCN_ATTR_C, DCB_BCN_ATTR_ALL, __DCB_BCN_ATTR_ENUM_MAX }
pub const DCB_BCN_ATTR_MAX: isize = __DCB_BCN_ATTR_ENUM_MAX as isize - 1;
pub const DCB_ATTR_VALUE_UNDEFINED: u8 = 0xff;
pub const DCB_APP_IDTYPE_ETHTYPE: u8 = 0x00;
pub const DCB_APP_IDTYPE_PORTNUM: u8 = 0x01;
#[repr(i32)]
pub enum dcbnl_app_attrs { DCB_APP_ATTR_UNDEFINED, DCB_APP_ATTR_IDTYPE, DCB_APP_ATTR_ID, DCB_APP_ATTR_PRIORITY, __DCB_APP_ATTR_ENUM_MAX }
pub const DCB_APP_ATTR_MAX: isize = __DCB_APP_ATTR_ENUM_MAX as isize - 1;
pub const DCB_FEATCFG_ERROR: u8 = 0x01;
pub const DCB_FEATCFG_ENABLE: u8 = 0x02;
pub const DCB_FEATCFG_WILLING: u8 = 0x04;
pub const DCB_FEATCFG_ADVERTISE: u8 = 0x08;
#[repr(i32)]
pub enum dcbnl_featcfg_attrs { DCB_FEATCFG_ATTR_UNDEFINED, DCB_FEATCFG_ATTR_ALL, DCB_FEATCFG_ATTR_PG, DCB_FEATCFG_ATTR_PFC, DCB_FEATCFG_ATTR_APP, __DCB_FEATCFG_ATTR_ENUM_MAX }
pub const DCB_FEATCFG_ATTR_MAX: isize = __DCB_FEATCFG_ATTR_ENUM_MAX as isize - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
