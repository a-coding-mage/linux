/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of nvme-fc.h. */

pub const NVME_CMD_FORMAT_ID: u8 = 0xFD;
pub const NVME_CMD_FC_ID: u8 = FC_TYPE_NVME;

pub const FCNVME_CMD_FLAGS_DIRMASK: u8 = 0x03;
pub const FCNVME_CMD_FLAGS_WRITE: u8 = 1 << 0;
pub const FCNVME_CMD_FLAGS_READ: u8 = 1 << 1;
pub const FCNVME_CMD_FLAGS_PICWP: u8 = 1 << 2;

pub const FCNVME_CMD_CAT_MASK: u8 = 0x0F;
pub const FCNVME_CMD_CAT_ADMINQ: u8 = 0x01;
pub const FCNVME_CMD_CAT_CSSMASK: u8 = 0x07;
pub const FCNVME_CMD_CAT_CSSFLAG: u8 = 0x08;

#[inline]
pub const fn fccmnd_set_cat_admin(rsv_cat: u8) -> u8 {
    (rsv_cat & !FCNVME_CMD_CAT_MASK) | FCNVME_CMD_CAT_ADMINQ
}

#[inline]
pub const fn fccmnd_set_cat_css(rsv_cat: u8, css: u8) -> u8 {
    (rsv_cat & !FCNVME_CMD_CAT_MASK) | FCNVME_CMD_CAT_CSSFLAG |
        (css & FCNVME_CMD_CAT_CSSMASK)
}

#[repr(C)]
pub struct nvme_fc_cmd_iu {
    pub format_id: __u8,
    pub fc_id: __u8,
    pub iu_len: __be16,
    pub rsvd4: [__u8; 2],
    pub rsv_cat: __u8,
    pub flags: __u8,
    pub connection_id: __be64,
    pub csn: __be32,
    pub data_len: __be32,
    pub sqe: nvme_command,
    pub dps: __u8,
    pub lbads: __u8,
    pub ms: __be16,
    pub rsvd92: __be32,
}

pub const NVME_FC_SIZEOF_ZEROS_RSP: usize = 12;
pub const FCNVME_SC_SUCCESS: u8 = 0;
pub const FCNVME_SC_INVALID_FIELD: u8 = 1;
pub const FCNVME_SC_ILL_CONN_PARAMS: u8 = 3;

#[repr(C)]
pub struct nvme_fc_ersp_iu {
    pub ersp_result: __u8,
    pub rsvd1: __u8,
    pub iu_len: __be16,
    pub rsn: __be32,
    pub xfrd_len: __be32,
    pub rsvd12: __be32,
    pub cqe: nvme_completion,
}

pub const FCNVME_NVME_SR_OPCODE: u8 = 0x01;
pub const FCNVME_NVME_SR_RSP_OPCODE: u8 = 0x02;

#[repr(C)]
pub struct nvme_fc_nvme_sr_iu {
    pub fc_id: __u8,
    pub opcode: __u8,
    pub rsvd2: __u8,
    pub retry_rctl: __u8,
    pub rsvd4: __be32,
}

pub const FCNVME_SRSTAT_ACC: u8 = 0x0;
pub const FCNVME_SRSTAT_LOGICAL_ERR: u8 = 0x3;
pub const FCNVME_SRSTAT_INV_QUALIF: u8 = 0x4;
pub const FCNVME_SRSTAT_UNABL2PERFORM: u8 = 0x9;

pub const FCNVME_LS_RSVD: u8 = 0;
pub const FCNVME_LS_RJT: u8 = 1;
pub const FCNVME_LS_ACC: u8 = 2;
pub const FCNVME_LS_CREATE_ASSOCIATION: u8 = 3;
pub const FCNVME_LS_CREATE_CONNECTION: u8 = 4;
pub const FCNVME_LS_DISCONNECT_ASSOC: u8 = 5;
pub const FCNVME_LS_DISCONNECT_CONN: u8 = 6;

pub const FCNVME_LSDESC_RSVD: u8 = 0x0;
pub const FCNVME_LSDESC_RQST: u8 = 0x1;
pub const FCNVME_LSDESC_RJT: u8 = 0x2;
pub const FCNVME_LSDESC_CREATE_ASSOC_CMD: u8 = 0x3;
pub const FCNVME_LSDESC_CREATE_CONN_CMD: u8 = 0x4;
pub const FCNVME_LSDESC_DISCONN_CMD: u8 = 0x5;
pub const FCNVME_LSDESC_CONN_ID: u8 = 0x6;
pub const FCNVME_LSDESC_ASSOC_ID: u8 = 0x7;

#[inline]
pub fn fcnvme_lsdesc_len(sz: usize) -> __be32 {
    cpu_to_be32((sz - (2 * core::mem::size_of::<u32>())) as u32)
}

#[repr(C)]
pub struct fcnvme_ls_rqst_w0 { pub ls_cmd: u8, pub zeros: [u8; 3] }
#[repr(C)]
pub struct fcnvme_lsdesc_rqst { pub desc_tag: __be32, pub desc_len: __be32, pub w0: fcnvme_ls_rqst_w0, pub rsvd12: __be32 }

pub const FCNVME_RJT_RC_NONE: u8 = 0;
pub const FCNVME_RJT_RC_INVAL: u8 = 0x01;
pub const FCNVME_RJT_RC_LOGIC: u8 = 0x03;
pub const FCNVME_RJT_RC_UNAB: u8 = 0x09;
pub const FCNVME_RJT_RC_UNSUP: u8 = 0x0b;
pub const FCNVME_RJT_RC_INV_ASSOC: u8 = 0x40;
pub const FCNVME_RJT_RC_INV_CONN: u8 = 0x41;
pub const FCNVME_RJT_RC_INV_PARAM: u8 = 0x42;
pub const FCNVME_RJT_RC_INSUF_RES: u8 = 0x43;
pub const FCNVME_RJT_RC_VENDOR: u8 = 0xff;
pub const FCNVME_RJT_EXP_NONE: u8 = 0x00;
pub const FCNVME_RJT_EXP_OXID_RXID: u8 = 0x17;
pub const FCNVME_RJT_EXP_UNAB_DATA: u8 = 0x2a;
pub const FCNVME_RJT_EXP_INV_LEN: u8 = 0x2d;
pub const FCNVME_RJT_EXP_INV_ERSP_RAT: u8 = 0x40;
pub const FCNVME_RJT_EXP_INV_CTLR_ID: u8 = 0x41;
pub const FCNVME_RJT_EXP_INV_QUEUE_ID: u8 = 0x42;
pub const FCNVME_RJT_EXP_INV_SQSIZE: u8 = 0x43;
pub const FCNVME_RJT_EXP_INV_HOSTID: u8 = 0x44;
pub const FCNVME_RJT_EXP_INV_HOSTNQN: u8 = 0x45;
pub const FCNVME_RJT_EXP_INV_SUBNQN: u8 = 0x46;

#[repr(C)]
pub struct fcnvme_lsdesc_rjt { pub desc_tag: __be32, pub desc_len: __be32, pub rsvd8: u8, pub reason_code: u8, pub reason_explanation: u8, pub vendor: u8, pub rsvd12: __be32 }

pub const FCNVME_ASSOC_HOSTNQN_LEN: usize = 256;
pub const FCNVME_ASSOC_SUBNQN_LEN: usize = 256;

#[repr(C)]
pub struct fcnvme_lsdesc_cr_assoc_cmd {
    pub desc_tag: __be32, pub desc_len: __be32, pub ersp_ratio: __be16, pub rsvd10: __be16,
    pub rsvd12: [__be32; 9], pub cntlid: __be16, pub sqsize: __be16, pub rsvd52: __be32,
    pub hostid: uuid_t, pub hostnqn: [u8; FCNVME_ASSOC_HOSTNQN_LEN],
    pub subnqn: [u8; FCNVME_ASSOC_SUBNQN_LEN], pub rsvd584: [__be32; 108],
}

pub const FCNVME_LSDESC_CRA_CMD_DESC_MINLEN: usize = core::mem::offset_of!(fcnvme_lsdesc_cr_assoc_cmd, rsvd584);
pub const FCNVME_LSDESC_CRA_CMD_DESC_MIN_DESCLEN: usize = FCNVME_LSDESC_CRA_CMD_DESC_MINLEN - core::mem::offset_of!(fcnvme_lsdesc_cr_assoc_cmd, ersp_ratio);

#[repr(C)]
pub struct fcnvme_lsdesc_cr_conn_cmd { pub desc_tag: __be32, pub desc_len: __be32, pub ersp_ratio: __be16, pub rsvd10: __be16, pub rsvd12: [__be32; 9], pub qid: __be16, pub sqsize: __be16, pub rsvd52: __be32 }
#[repr(C)]
pub struct fcnvme_lsdesc_disconn_cmd { pub desc_tag: __be32, pub desc_len: __be32, pub rsvd8: [__be32; 4] }
#[repr(C)]
pub struct fcnvme_lsdesc_conn_id { pub desc_tag: __be32, pub desc_len: __be32, pub connection_id: __be64 }
#[repr(C)]
pub struct fcnvme_lsdesc_assoc_id { pub desc_tag: __be32, pub desc_len: __be32, pub association_id: __be64 }

pub const FCNVME_RS_RCTL_CMND: u8 = 0x6;
pub const FCNVME_RS_RCTL_DATA: u8 = 0x1;
pub const FCNVME_RS_RCTL_CONF: u8 = 0x3;
pub const FCNVME_RS_RCTL_SR: u8 = 0x9;
pub const FCNVME_RS_RCTL_XFER_RDY: u8 = 0x5;
pub const FCNVME_RS_RCTL_RSP: u8 = 0x7;
pub const FCNVME_RS_RCTL_ERSP: u8 = 0x8;
pub const FCNVME_RS_RCTL_SR_RSP: u8 = 0xA;

#[repr(C)]
pub struct fcnvme_ls_rjt { pub w0: fcnvme_ls_rqst_w0, pub desc_list_len: __be32, pub rqst: fcnvme_lsdesc_rqst, pub rjt: fcnvme_lsdesc_rjt }
#[repr(C)]
pub struct fcnvme_ls_acc_hdr { pub w0: fcnvme_ls_rqst_w0, pub desc_list_len: __be32, pub rqst: fcnvme_lsdesc_rqst }
#[repr(C)]
pub struct fcnvme_ls_cr_assoc_rqst { pub w0: fcnvme_ls_rqst_w0, pub desc_list_len: __be32, pub assoc_cmd: fcnvme_lsdesc_cr_assoc_cmd }
pub const FCNVME_LSDESC_CRA_RQST_MINLEN: usize = core::mem::offset_of!(fcnvme_ls_cr_assoc_rqst, assoc_cmd) + FCNVME_LSDESC_CRA_CMD_DESC_MINLEN;
pub const FCNVME_LSDESC_CRA_RQST_MIN_LISTLEN: usize = FCNVME_LSDESC_CRA_CMD_DESC_MINLEN;
#[repr(C)]
pub struct fcnvme_ls_cr_assoc_acc { pub hdr: fcnvme_ls_acc_hdr, pub associd: fcnvme_lsdesc_assoc_id, pub connectid: fcnvme_lsdesc_conn_id }
#[repr(C)]
pub struct fcnvme_ls_cr_conn_rqst { pub w0: fcnvme_ls_rqst_w0, pub desc_list_len: __be32, pub associd: fcnvme_lsdesc_assoc_id, pub connect_cmd: fcnvme_lsdesc_cr_conn_cmd }
#[repr(C)]
pub struct fcnvme_ls_cr_conn_acc { pub hdr: fcnvme_ls_acc_hdr, pub connectid: fcnvme_lsdesc_conn_id }
#[repr(C)]
pub struct fcnvme_ls_disconnect_assoc_rqst { pub w0: fcnvme_ls_rqst_w0, pub desc_list_len: __be32, pub associd: fcnvme_lsdesc_assoc_id, pub discon_cmd: fcnvme_lsdesc_disconn_cmd }
#[repr(C)]
pub struct fcnvme_ls_disconnect_assoc_acc { pub hdr: fcnvme_ls_acc_hdr }
#[repr(C)]
pub struct fcnvme_ls_disconnect_conn_rqst { pub w0: fcnvme_ls_rqst_w0, pub desc_list_len: __be32, pub associd: fcnvme_lsdesc_assoc_id, pub connectid: fcnvme_lsdesc_conn_id }
#[repr(C)]
pub struct fcnvme_ls_disconnect_conn_acc { pub hdr: fcnvme_ls_acc_hdr }

pub const FC_TWO_TIMES_R_A_TOV: u32 = 2 * (FC_DEF_R_A_TOV / 1000);
pub const NVME_FC_LS_TIMEOUT_SEC: u32 = FC_TWO_TIMES_R_A_TOV;
pub const NVME_FC_TGTOP_TIMEOUT_SEC: u32 = FC_TWO_TIMES_R_A_TOV;
pub const NVME_FC_TRADDR_NNLEN: usize = 3;
pub const NVME_FC_TRADDR_OXNNLEN: usize = 5;
pub const NVME_FC_TRADDR_HEXNAMELEN: usize = 16;
pub const NVME_FC_TRADDR_MINLENGTH: usize = 2 * (NVME_FC_TRADDR_NNLEN + NVME_FC_TRADDR_HEXNAMELEN) + 1;
pub const NVME_FC_TRADDR_MAXLENGTH: usize = 2 * (NVME_FC_TRADDR_OXNNLEN + NVME_FC_TRADDR_HEXNAMELEN) + 1;
pub const NVME_FC_TRADDR_MIN_PN_OFFSET: usize = NVME_FC_TRADDR_NNLEN + NVME_FC_TRADDR_HEXNAMELEN + 1;
pub const NVME_FC_TRADDR_MAX_PN_OFFSET: usize = NVME_FC_TRADDR_OXNNLEN + NVME_FC_TRADDR_HEXNAMELEN + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
