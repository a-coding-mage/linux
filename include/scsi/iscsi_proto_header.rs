/* SPDX-License-Identifier: GPL-2.0-or-later */
/* RFC 3720 (iSCSI) protocol data types */

// C dependencies: linux/types.h and scsi/scsi.h.
pub type itt_t = u32;
pub type __be16 = u16;
pub type __be32 = u32;

pub const ISCSI_DRAFT20_VERSION: u8 = 0x00;
pub const ISCSI_LISTEN_PORT: u16 = 3260;
pub const ISCSI_HDR_LEN: usize = 48;
pub const ISCSI_CRC_LEN: usize = 4;
pub const ISCSI_PAD_LEN: usize = 4;

#[inline]
pub fn iscsi_sna_lt(n1: u32, n2: u32) -> i32 { (n1.wrapping_sub(n2) as i32) < 0 as i32 as i32 }
#[inline]
pub fn iscsi_sna_lte(n1: u32, n2: u32) -> i32 { (n1.wrapping_sub(n2) as i32) <= 0 }
#[inline]
pub fn iscsi_sna_gt(n1: u32, n2: u32) -> i32 { (n1.wrapping_sub(n2) as i32) > 0 }
#[inline]
pub fn iscsi_sna_gte(n1: u32, n2: u32) -> i32 { (n1.wrapping_sub(n2) as i32) >= 0 }

#[inline] pub fn ntoh24(p: &[u8; 3]) -> u32 { ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | p[2] as u32 }
#[inline] pub fn hton24(p: &mut [u8; 3], v: u32) { p[0] = ((v >> 16) & 0xff) as u8; p[1] = ((v >> 8) & 0xff) as u8; p[2] = (v & 0xff) as u8; }
#[inline] pub fn zero_data(p: &mut [u8; 3]) { p[0] = 0; p[1] = 0; p[2] = 0; }

pub const ISCSI_RESERVED_TAG: u32 = 0xffffffff;
pub const ISCSI_OP_RETRY: u8 = 0x80; pub const ISCSI_OP_IMMEDIATE: u8 = 0x40; pub const ISCSI_OPCODE_MASK: u8 = 0x3f;
pub const ISCSI_OP_NOOP_OUT: u8 = 0x00; pub const ISCSI_OP_SCSI_CMD: u8 = 0x01; pub const ISCSI_OP_SCSI_TMFUNC: u8 = 0x02;
pub const ISCSI_OP_LOGIN: u8 = 0x03; pub const ISCSI_OP_TEXT: u8 = 0x04; pub const ISCSI_OP_SCSI_DATA_OUT: u8 = 0x05;
pub const ISCSI_OP_LOGOUT: u8 = 0x06; pub const ISCSI_OP_SNACK: u8 = 0x10;
pub const ISCSI_OP_VENDOR1_CMD: u8 = 0x1c; pub const ISCSI_OP_VENDOR2_CMD: u8 = 0x1d; pub const ISCSI_OP_VENDOR3_CMD: u8 = 0x1e; pub const ISCSI_OP_VENDOR4_CMD: u8 = 0x1f;
pub const ISCSI_OP_NOOP_IN: u8 = 0x20; pub const ISCSI_OP_SCSI_CMD_RSP: u8 = 0x21; pub const ISCSI_OP_SCSI_TMFUNC_RSP: u8 = 0x22;
pub const ISCSI_OP_LOGIN_RSP: u8 = 0x23; pub const ISCSI_OP_TEXT_RSP: u8 = 0x24; pub const ISCSI_OP_SCSI_DATA_IN: u8 = 0x25;
pub const ISCSI_OP_LOGOUT_RSP: u8 = 0x26; pub const ISCSI_OP_R2T: u8 = 0x31; pub const ISCSI_OP_ASYNC_EVENT: u8 = 0x32; pub const ISCSI_OP_REJECT: u8 = 0x3f;

// `struct scsi_lun` and SCSI_MAX_VARLEN_CDB_SIZE are supplied by scsi/scsi.h.
#[repr(C)] pub struct iscsi_hdr { pub opcode:u8, pub flags:u8, pub rsvd2:[u8;2], pub hlength:u8, pub dlength:[u8;3], pub lun: scsi_lun, pub itt:itt_t, pub ttt:__be32, pub statsn:__be32, pub exp_statsn:__be32, pub max_statsn:__be32, pub other:[u8;12] }
#[repr(C)] pub struct iscsi_ahs_hdr { pub ahslength:__be16, pub ahstype:u8, pub ahspec:[u8;5] }
pub const ISCSI_AHSTYPE_CDB:u8=1; pub const ISCSI_AHSTYPE_RLENGTH:u8=2; pub const ISCSI_CDB_SIZE:usize=16;
#[repr(C)] pub struct iscsi_scsi_req { pub opcode:u8,pub flags:u8,pub rsvd2:__be16,pub hlength:u8,pub dlength:[u8;3],pub lun:scsi_lun,pub itt:itt_t,pub data_length:__be32,pub cmdsn:__be32,pub exp_statsn:__be32,pub cdb:[u8;16] }
pub const ISCSI_FLAG_CMD_FINAL:u8=0x80; pub const ISCSI_FLAG_CMD_READ:u8=0x40; pub const ISCSI_FLAG_CMD_WRITE:u8=0x20; pub const ISCSI_FLAG_CMD_ATTR_MASK:u8=7;
pub const ISCSI_ATTR_UNTAGGED:u8=0; pub const ISCSI_ATTR_SIMPLE:u8=1; pub const ISCSI_ATTR_ORDERED:u8=2; pub const ISCSI_ATTR_HEAD_OF_QUEUE:u8=3; pub const ISCSI_ATTR_ACA:u8=4;
#[repr(C)] pub struct iscsi_rlength_ahdr { pub ahslength:__be16,pub ahstype:u8,pub reserved:u8,pub read_length:__be32 }
#[repr(C)] pub struct iscsi_ecdb_ahdr { pub ahslength:__be16,pub ahstype:u8,pub reserved:u8,pub ecdb:[u8; SCSI_MAX_VARLEN_CDB_SIZE-ISCSI_CDB_SIZE] }

#[repr(C)] pub struct iscsi_scsi_rsp { pub opcode:u8,pub flags:u8,pub response:u8,pub cmd_status:u8,pub hlength:u8,pub dlength:[u8;3],pub rsvd:[u8;8],pub itt:itt_t,pub rsvd1:__be32,pub statsn:__be32,pub exp_cmdsn:__be32,pub max_cmdsn:__be32,pub exp_datasn:__be32,pub bi_residual_count:__be32,pub residual_count:__be32 }
pub const ISCSI_FLAG_CMD_BIDI_OVERFLOW:u8=0x10; pub const ISCSI_FLAG_CMD_BIDI_UNDERFLOW:u8=8; pub const ISCSI_FLAG_CMD_OVERFLOW:u8=4; pub const ISCSI_FLAG_CMD_UNDERFLOW:u8=2;
pub const ISCSI_STATUS_CMD_COMPLETED:u8=0; pub const ISCSI_STATUS_TARGET_FAILURE:u8=1; pub const ISCSI_STATUS_SUBSYS_FAILURE:u8=2;

// Remaining protocol headers retain C layout and field order.
#[repr(C)] pub struct iscsi_async { pub opcode:u8,pub flags:u8,pub rsvd2:[u8;2],pub rsvd3:u8,pub dlength:[u8;3],pub lun:scsi_lun,pub rsvd4:[u8;8],pub statsn:__be32,pub exp_cmdsn:__be32,pub max_cmdsn:__be32,pub async_event:u8,pub async_vcode:u8,pub param1:__be16,pub param2:__be16,pub param3:__be16,pub rsvd5:[u8;4] }
#[repr(C)] pub struct iscsi_nopout { pub opcode:u8,pub flags:u8,pub rsvd2:__be16,pub rsvd3:u8,pub dlength:[u8;3],pub lun:scsi_lun,pub itt:itt_t,pub ttt:__be32,pub cmdsn:__be32,pub exp_statsn:__be32,pub rsvd4:[u8;16] }
#[repr(C)] pub struct iscsi_nopin { pub opcode:u8,pub flags:u8,pub rsvd2:__be16,pub rsvd3:u8,pub dlength:[u8;3],pub lun:scsi_lun,pub itt:itt_t,pub ttt:__be32,pub statsn:__be32,pub exp_cmdsn:__be32,pub max_cmdsn:__be32,pub rsvd4:[u8;12] }
#[repr(C)] pub struct iscsi_tm { pub opcode:u8,pub flags:u8,pub rsvd1:[u8;2],pub hlength:u8,pub dlength:[u8;3],pub lun:scsi_lun,pub itt:itt_t,pub rtt:itt_t,pub cmdsn:__be32,pub exp_statsn:__be32,pub refcmdsn:__be32,pub exp_datasn:__be32,pub rsvd2:[u8;8] }
pub const ISCSI_FLAG_TM_FUNC_MASK:u8=0x7f; pub const ISCSI_TM_FUNC_ABORT_TASK:u8=1; pub const ISCSI_TM_FUNC_ABORT_TASK_SET:u8=2; pub const ISCSI_TM_FUNC_CLEAR_ACA:u8=3; pub const ISCSI_TM_FUNC_CLEAR_TASK_SET:u8=4; pub const ISCSI_TM_FUNC_LOGICAL_UNIT_RESET:u8=5; pub const ISCSI_TM_FUNC_TARGET_WARM_RESET:u8=6; pub const ISCSI_TM_FUNC_TARGET_COLD_RESET:u8=7; pub const ISCSI_TM_FUNC_TASK_REASSIGN:u8=8;
#[inline] pub unsafe fn ISCSI_TM_FUNC_VALUE(hdr:*const iscsi_tm)->u8 { (*hdr).flags & ISCSI_FLAG_TM_FUNC_MASK }
#[repr(C)] pub struct iscsi_tm_rsp { pub opcode:u8,pub flags:u8,pub response:u8,pub qualifier:u8,pub hlength:u8,pub dlength:[u8;3],pub rsvd2:[u8;8],pub itt:itt_t,pub rtt:itt_t,pub statsn:__be32,pub exp_cmdsn:__be32,pub max_cmdsn:__be32,pub rsvd3:[u8;12] }
pub const ISCSI_TMF_RSP_COMPLETE:u8=0; pub const ISCSI_TMF_RSP_NO_TASK:u8=1; pub const ISCSI_TMF_RSP_NO_LUN:u8=2; pub const ISCSI_TMF_RSP_TASK_ALLEGIANT:u8=3; pub const ISCSI_TMF_RSP_NO_FAILOVER:u8=4; pub const ISCSI_TMF_RSP_NOT_SUPPORTED:u8=5; pub const ISCSI_TMF_RSP_AUTH_FAILED:u8=6; pub const ISCSI_TMF_RSP_REJECTED:u8=0xff;
#[repr(C)] pub struct iscsi_r2t_rsp { pub opcode:u8,pub flags:u8,pub rsvd2:[u8;2],pub hlength:u8,pub dlength:[u8;3],pub lun:scsi_lun,pub itt:itt_t,pub ttt:__be32,pub statsn:__be32,pub exp_cmdsn:__be32,pub max_cmdsn:__be32,pub r2tsn:__be32,pub data_offset:__be32,pub data_length:__be32 }
#[repr(C)] pub struct iscsi_data { pub opcode:u8,pub flags:u8,pub rsvd2:[u8;2],pub rsvd3:u8,pub dlength:[u8;3],pub lun:scsi_lun,pub itt:itt_t,pub ttt:__be32,pub rsvd4:__be32,pub exp_statsn:__be32,pub rsvd5:__be32,pub datasn:__be32,pub offset:__be32,pub rsvd6:__be32 }
#[repr(C)] pub struct iscsi_data_rsp { pub opcode:u8,pub flags:u8,pub rsvd2:u8,pub cmd_status:u8,pub hlength:u8,pub dlength:[u8;3],pub lun:scsi_lun,pub itt:itt_t,pub ttt:__be32,pub statsn:__be32,pub exp_cmdsn:__be32,pub max_cmdsn:__be32,pub datasn:__be32,pub offset:__be32,pub residual_count:__be32 }
pub const ISCSI_FLAG_DATA_ACK:u8=0x40; pub const ISCSI_FLAG_DATA_OVERFLOW:u8=4; pub const ISCSI_FLAG_DATA_UNDERFLOW:u8=2; pub const ISCSI_FLAG_DATA_STATUS:u8=1;
#[repr(C)] pub struct iscsi_text { pub opcode:u8,pub flags:u8,pub rsvd2:[u8;2],pub hlength:u8,pub dlength:[u8;3],pub rsvd4:[u8;8],pub itt:itt_t,pub ttt:__be32,pub cmdsn:__be32,pub exp_statsn:__be32,pub rsvd5:[u8;16] }
pub const ISCSI_FLAG_TEXT_CONTINUE:u8=0x40;
#[repr(C)] pub struct iscsi_text_rsp { pub opcode:u8,pub flags:u8,pub rsvd2:[u8;2],pub hlength:u8,pub dlength:[u8;3],pub rsvd4:[u8;8],pub itt:itt_t,pub ttt:__be32,pub statsn:__be32,pub exp_cmdsn:__be32,pub max_cmdsn:__be32,pub rsvd5:[u8;12] }
#[repr(C)] pub struct iscsi_login_req { pub opcode:u8,pub flags:u8,pub max_version:u8,pub min_version:u8,pub hlength:u8,pub dlength:[u8;3],pub isid:[u8;6],pub tsih:__be16,pub itt:itt_t,pub cid:__be16,pub rsvd3:__be16,pub cmdsn:__be32,pub exp_statsn:__be32,pub rsvd5:[u8;16] }
pub const ISCSI_FLAG_LOGIN_TRANSIT:u8=0x80; pub const ISCSI_FLAG_LOGIN_CONTINUE:u8=0x40; pub const ISCSI_FLAG_LOGIN_CURRENT_STAGE_MASK:u8=0x0c; pub const ISCSI_FLAG_LOGIN_CURRENT_STAGE1:u8=4; pub const ISCSI_FLAG_LOGIN_CURRENT_STAGE2:u8=8; pub const ISCSI_FLAG_LOGIN_CURRENT_STAGE3:u8=0x0c; pub const ISCSI_FLAG_LOGIN_NEXT_STAGE_MASK:u8=3; pub const ISCSI_FLAG_LOGIN_NEXT_STAGE1:u8=1; pub const ISCSI_FLAG_LOGIN_NEXT_STAGE2:u8=2; pub const ISCSI_FLAG_LOGIN_NEXT_STAGE3:u8=3;
#[inline] pub fn ISCSI_LOGIN_CURRENT_STAGE(flags:u8)->u8 {(flags&0x0c)>>2} #[inline] pub fn ISCSI_LOGIN_NEXT_STAGE(flags:u8)->u8 {flags&3}
#[repr(C)] pub struct iscsi_login_rsp { pub opcode:u8,pub flags:u8,pub max_version:u8,pub active_version:u8,pub hlength:u8,pub dlength:[u8;3],pub isid:[u8;6],pub tsih:__be16,pub itt:itt_t,pub rsvd3:__be32,pub statsn:__be32,pub exp_cmdsn:__be32,pub max_cmdsn:__be32,pub status_class:u8,pub status_detail:u8,pub rsvd4:[u8;10] }
pub const ISCSI_INITIAL_LOGIN_STAGE:i32=-1; pub const ISCSI_SECURITY_NEGOTIATION_STAGE:i32=0; pub const ISCSI_OP_PARMS_NEGOTIATION_STAGE:i32=1; pub const ISCSI_FULL_FEATURE_PHASE:i32=3;
pub const ISCSI_STATUS_CLS_SUCCESS:u8=0; pub const ISCSI_STATUS_CLS_REDIRECT:u8=1; pub const ISCSI_STATUS_CLS_INITIATOR_ERR:u8=2; pub const ISCSI_STATUS_CLS_TARGET_ERR:u8=3;
pub const ISCSI_LOGIN_STATUS_ACCEPT:u8=0; pub const ISCSI_LOGIN_STATUS_TGT_MOVED_TEMP:u8=1; pub const ISCSI_LOGIN_STATUS_TGT_MOVED_PERM:u8=2; pub const ISCSI_LOGIN_STATUS_INIT_ERR:u8=0; pub const ISCSI_LOGIN_STATUS_AUTH_FAILED:u8=1; pub const ISCSI_LOGIN_STATUS_TGT_FORBIDDEN:u8=2; pub const ISCSI_LOGIN_STATUS_TGT_NOT_FOUND:u8=3; pub const ISCSI_LOGIN_STATUS_TGT_REMOVED:u8=4; pub const ISCSI_LOGIN_STATUS_NO_VERSION:u8=5; pub const ISCSI_LOGIN_STATUS_ISID_ERROR:u8=6; pub const ISCSI_LOGIN_STATUS_MISSING_FIELDS:u8=7; pub const ISCSI_LOGIN_STATUS_CONN_ADD_FAILED:u8=8; pub const ISCSI_LOGIN_STATUS_NO_SESSION_TYPE:u8=9; pub const ISCSI_LOGIN_STATUS_NO_SESSION:u8=10; pub const ISCSI_LOGIN_STATUS_INVALID_REQUEST:u8=11; pub const ISCSI_LOGIN_STATUS_TARGET_ERROR:u8=0; pub const ISCSI_LOGIN_STATUS_SVC_UNAVAILABLE:u8=1; pub const ISCSI_LOGIN_STATUS_NO_RESOURCES:u8=2;
#[repr(C)] pub struct iscsi_logout { pub opcode:u8,pub flags:u8,pub rsvd1:[u8;2],pub hlength:u8,pub dlength:[u8;3],pub rsvd2:[u8;8],pub itt:itt_t,pub cid:__be16,pub rsvd3:[u8;2],pub cmdsn:__be32,pub exp_statsn:__be32,pub rsvd4:[u8;16] }
pub const ISCSI_FLAG_LOGOUT_REASON_MASK:u8=0x7f; pub const ISCSI_LOGOUT_REASON_CLOSE_SESSION:u8=0; pub const ISCSI_LOGOUT_REASON_CLOSE_CONNECTION:u8=1; pub const ISCSI_LOGOUT_REASON_RECOVERY:u8=2; pub const ISCSI_LOGOUT_REASON_AEN_REQUEST:u8=3;
#[repr(C)] pub struct iscsi_logout_rsp { pub opcode:u8,pub flags:u8,pub response:u8,pub rsvd2:u8,pub hlength:u8,pub dlength:[u8;3],pub rsvd3:[u8;8],pub itt:itt_t,pub rsvd4:__be32,pub statsn:__be32,pub exp_cmdsn:__be32,pub max_cmdsn:__be32,pub rsvd5:__be32,pub t2wait:__be16,pub t2retain:__be16,pub rsvd6:__be32 }
pub const ISCSI_LOGOUT_SUCCESS:u8=0; pub const ISCSI_LOGOUT_CID_NOT_FOUND:u8=1; pub const ISCSI_LOGOUT_RECOVERY_UNSUPPORTED:u8=2; pub const ISCSI_LOGOUT_CLEANUP_FAILED:u8=3;
#[repr(C)] pub struct iscsi_snack { pub opcode:u8,pub flags:u8,pub rsvd2:[u8;2],pub hlength:u8,pub dlength:[u8;3],pub lun:[u8;8],pub itt:itt_t,pub ttt:__be32,pub rsvd3:[u8;4],pub exp_statsn:__be32,pub rsvd4:[u8;8],pub begrun:__be32,pub runlength:__be32 }
pub const ISCSI_FLAG_SNACK_TYPE_DATA:u8=0; pub const ISCSI_FLAG_SNACK_TYPE_R2T:u8=0; pub const ISCSI_FLAG_SNACK_TYPE_STATUS:u8=1; pub const ISCSI_FLAG_SNACK_TYPE_DATA_ACK:u8=2; pub const ISCSI_FLAG_SNACK_TYPE_RDATA:u8=3; pub const ISCSI_FLAG_SNACK_TYPE_MASK:u8=0x0f;
#[repr(C)] pub struct iscsi_reject { pub opcode:u8,pub flags:u8,pub reason:u8,pub rsvd2:u8,pub hlength:u8,pub dlength:[u8;3],pub rsvd3:[u8;8],pub ffffffff:__be32,pub rsvd4:[u8;4],pub statsn:__be32,pub exp_cmdsn:__be32,pub max_cmdsn:__be32,pub datasn:__be32,pub rsvd5:[u8;8] }
pub const ISCSI_REASON_CMD_BEFORE_LOGIN:u8=1; pub const ISCSI_REASON_DATA_DIGEST_ERROR:u8=2; pub const ISCSI_REASON_DATA_SNACK_REJECT:u8=3; pub const ISCSI_REASON_PROTOCOL_ERROR:u8=4; pub const ISCSI_REASON_CMD_NOT_SUPPORTED:u8=5; pub const ISCSI_REASON_IMM_CMD_REJECT:u8=6; pub const ISCSI_REASON_TASK_IN_PROGRESS:u8=7; pub const ISCSI_REASON_INVALID_SNACK:u8=8; pub const ISCSI_REASON_BOOKMARK_INVALID:u8=9; pub const ISCSI_REASON_BOOKMARK_NO_RESOURCES:u8=10; pub const ISCSI_REASON_NEGOTIATION_RESET:u8=11;
pub const MAX_KEY_VALUE_PAIRS:usize=8192; pub const KEY_MAXLEN:usize=64; pub const VALUE_MAXLEN:usize=255; pub const TARGET_NAME_MAXLEN:usize=VALUE_MAXLEN;
pub const ISCSI_DEF_MAX_RECV_SEG_LEN:usize=8192; pub const ISCSI_MIN_MAX_RECV_SEG_LEN:usize=512; pub const ISCSI_MAX_MAX_RECV_SEG_LEN:usize=16777215; pub const ISCSI_DEF_FIRST_BURST_LEN:usize=65536; pub const ISCSI_MIN_FIRST_BURST_LEN:usize=512; pub const ISCSI_MAX_FIRST_BURST_LEN:usize=16777215; pub const ISCSI_DEF_MAX_BURST_LEN:usize=262144; pub const ISCSI_MIN_MAX_BURST_LEN:usize=512; pub const ISCSI_MAX_MAX_BURST_LEN:usize=16777215; pub const ISCSI_DEF_TIME2WAIT:u32=2; pub const ISCSI_NAME_LEN:usize=224;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
