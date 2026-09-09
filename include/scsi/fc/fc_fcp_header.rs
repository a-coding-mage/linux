/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2007 Intel Corporation. All rights reserved.
 *
 * Maintained at www.Open-FCoE.org
 */

// Dependency supplied by scsi/scsi.h.

/*
 * Fibre Channel Protocol for SCSI.
 * From T10 FCP-3, T10 project 1560-D Rev 4, Sept. 13, 2005.
 */

/* Service parameter page parameters (word 3 bits) for Process Login. */
pub const FCP_SPPF_TASK_RETRY_ID: u16 = 0x0200;
pub const FCP_SPPF_RETRY: u16 = 0x0100;
pub const FCP_SPPF_CONF_COMPL: u16 = 0x0080;
pub const FCP_SPPF_OVLY_ALLOW: u16 = 0x0040;
pub const FCP_SPPF_INIT_FCN: u16 = 0x0020;
pub const FCP_SPPF_TARG_FCN: u16 = 0x0010;
pub const FCP_SPPF_RD_XRDY_DIS: u16 = 0x0002;
pub const FCP_SPPF_WR_XRDY_DIS: u16 = 0x0001;

#[repr(C)]
pub struct fcp_cmnd {
    pub fc_lun: scsi_lun,
    pub fc_cmdref: u8,
    pub fc_pri_ta: u8,
    pub fc_tm_flags: u8,
    pub fc_flags: u8,
    pub fc_cdb: [u8; 16],
    pub fc_dl: u32,
}
pub const FCP_CMND_LEN: usize = 32;

#[repr(C)]
pub struct fcp_cmnd32 {
    pub fc_lun: scsi_lun,
    pub fc_cmdref: u8,
    pub fc_pri_ta: u8,
    pub fc_tm_flags: u8,
    pub fc_flags: u8,
    pub fc_cdb: [u8; 32],
    pub fc_dl: u32,
}
pub const FCP_CMND32_LEN: usize = 48;
pub const FCP_CMND32_ADD_LEN: usize = 16 / 4;

pub const FCP_PTA_SIMPLE: u8 = 0;
pub const FCP_PTA_HEADQ: u8 = 1;
pub const FCP_PTA_ORDERED: u8 = 2;
pub const FCP_PTA_ACA: u8 = 4;
pub const FCP_PTA_MASK: u8 = 7;
pub const FCP_PRI_SHIFT: u8 = 3;
pub const FCP_PRI_RESVD_MASK: u8 = 0x80;

pub const FCP_TMF_CLR_ACA: u8 = 0x40;
pub const FCP_TMF_TGT_RESET: u8 = 0x20;
pub const FCP_TMF_LUN_RESET: u8 = 0x10;
pub const FCP_TMF_CLR_TASK_SET: u8 = 0x04;
pub const FCP_TMF_ABT_TASK_SET: u8 = 0x02;

pub const FCP_CFL_LEN_MASK: u8 = 0xfc;
pub const FCP_CFL_LEN_SHIFT: u8 = 2;
pub const FCP_CFL_RDDATA: u8 = 0x02;
pub const FCP_CFL_WRDATA: u8 = 0x01;

#[repr(C)]
pub struct fcp_txrdy {
    pub ft_data_ro: u32,
    pub ft_burst_len: u32,
    pub _ft_resvd: [u8; 4],
}
pub const FCP_TXRDY_LEN: usize = 12;

#[repr(C)]
pub struct fcp_resp {
    pub _fr_resvd: [u8; 8],
    pub fr_retry_delay: u16,
    pub fr_flags: u8,
    pub fr_status: u8,
}
pub const FCP_RESP_LEN: usize = 12;

#[repr(C)]
pub struct fcp_resp_ext {
    pub fr_resid: u32,
    pub fr_sns_len: u32,
    pub fr_rsp_len: u32,
}
pub const FCP_RESP_EXT_LEN: usize = 12;

#[repr(C)]
pub struct fcp_resp_rsp_info {
    pub _fr_resvd: [u8; 3],
    pub rsp_code: u8,
    pub _fr_resvd2: [u8; 4],
}
pub const FCP_RESP_RSP_INFO_LEN4: usize = 4;
pub const FCP_RESP_RSP_INFO_LEN8: usize = 8;

#[repr(C)]
pub struct fcp_resp_with_ext {
    pub resp: fcp_resp,
    pub ext: fcp_resp_ext,
}
pub const FCP_RESP_WITH_EXT: usize = FCP_RESP_LEN + FCP_RESP_EXT_LEN;

pub const FCP_BIDI_RSP: u8 = 0x80;
pub const FCP_BIDI_READ_UNDER: u8 = 0x40;
pub const FCP_BIDI_READ_OVER: u8 = 0x20;
pub const FCP_CONF_REQ: u8 = 0x10;
pub const FCP_RESID_UNDER: u8 = 0x08;
pub const FCP_RESID_OVER: u8 = 0x04;
pub const FCP_SNS_LEN_VAL: u8 = 0x02;
pub const FCP_RSP_LEN_VAL: u8 = 0x01;

#[repr(u32)]
pub enum fcp_resp_rsp_codes {
    FCP_TMF_CMPL = 0,
    FCP_DATA_LEN_INVALID = 1,
    FCP_CMND_FIELDS_INVALID = 2,
    FCP_DATA_PARAM_MISMATCH = 3,
    FCP_TMF_REJECTED = 4,
    FCP_TMF_FAILED = 5,
    FCP_TMF_INVALID_LUN = 9,
}

#[repr(C)]
pub struct fcp_srr {
    pub srr_op: u8,
    pub srr_resvd: [u8; 3],
    pub srr_ox_id: u16,
    pub srr_rx_id: u16,
    pub srr_rel_off: u32,
    pub srr_r_ctl: u8,
    pub srr_resvd2: [u8; 3],
}

pub const FCP_FEAT_TARG: u32 = 1 << 0;
pub const FCP_FEAT_INIT: u32 = 1 << 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
