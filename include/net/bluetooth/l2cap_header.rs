/* SPDX-License-Identifier: GPL-2.0 */
/* BlueZ L2CAP header translated from C. External kernel types/functions are dependencies. */

pub const L2CAP_DEFAULT_MTU: u32 = 672;
pub const L2CAP_DEFAULT_MIN_MTU: u32 = 48;
pub const L2CAP_SIG_MTU: u32 = 48;
pub const L2CAP_DEFAULT_FLUSH_TO: u32 = 0xffff;
pub const L2CAP_EFS_DEFAULT_FLUSH_TO: u64 = 0xffff_ffff;
pub const L2CAP_DEFAULT_TX_WINDOW: u32 = 63;
pub const L2CAP_DEFAULT_EXT_WINDOW: u32 = 0x3fff;
pub const L2CAP_DEFAULT_MAX_TX: u32 = 3;
pub const L2CAP_DEFAULT_RETRANS_TO: u32 = 2000;
pub const L2CAP_DEFAULT_MONITOR_TO: u32 = 12000;
pub const L2CAP_DEFAULT_MAX_PDU_SIZE: u32 = 1492;
pub const L2CAP_DEFAULT_ACK_TO: u32 = 200;
pub const L2CAP_DEFAULT_MAX_SDU_SIZE: u32 = 0xffff;
pub const L2CAP_DEFAULT_SDU_ITIME: u64 = 0xffff_ffff;
pub const L2CAP_DEFAULT_ACC_LAT: u64 = 0xffff_ffff;
pub const L2CAP_BREDR_MAX_PAYLOAD: u32 = 1019;
pub const L2CAP_LE_MIN_MTU: u32 = 23;
pub const L2CAP_ECRED_CONN_SCID_MAX: u32 = 5;

/* msecs_to_jiffies(...) is a build-environment operation and remains external. */
pub const L2CAP_OPTIONS: u32 = 0x01;
pub const L2CAP_CONNINFO: u32 = 0x02;
pub const L2CAP_LM: u32 = 0x03;
pub const L2CAP_LM_MASTER: u32 = 1; pub const L2CAP_LM_AUTH: u32 = 2;
pub const L2CAP_LM_ENCRYPT: u32 = 4; pub const L2CAP_LM_TRUSTED: u32 = 8;
pub const L2CAP_LM_RELIABLE: u32 = 0x10; pub const L2CAP_LM_SECURE: u32 = 0x20; pub const L2CAP_LM_FIPS: u32 = 0x40;

macro_rules! constants { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u32 = $v;)* }; }
constants! {
 L2CAP_COMMAND_REJ=1,L2CAP_CONN_REQ=2,L2CAP_CONN_RSP=3,L2CAP_CONF_REQ=4,L2CAP_CONF_RSP=5,L2CAP_DISCONN_REQ=6,L2CAP_DISCONN_RSP=7,L2CAP_ECHO_REQ=8,L2CAP_ECHO_RSP=9,L2CAP_INFO_REQ=0xa,L2CAP_INFO_RSP=0xb,L2CAP_CONN_PARAM_UPDATE_REQ=0x12,L2CAP_CONN_PARAM_UPDATE_RSP=0x13,L2CAP_LE_CONN_REQ=0x14,L2CAP_LE_CONN_RSP=0x15,L2CAP_LE_CREDITS=0x16,L2CAP_ECRED_CONN_REQ=0x17,L2CAP_ECRED_CONN_RSP=0x18,L2CAP_ECRED_RECONF_REQ=0x19,L2CAP_ECRED_RECONF_RSP=0x1a,
 L2CAP_FEAT_FLOWCTL=1,L2CAP_FEAT_RETRANS=2,L2CAP_FEAT_BIDIR_QOS=4,L2CAP_FEAT_ERTM=8,L2CAP_FEAT_STREAMING=0x10,L2CAP_FEAT_FCS=0x20,L2CAP_FEAT_EXT_FLOW=0x40,L2CAP_FEAT_FIXED_CHAN=0x80,L2CAP_FEAT_EXT_WINDOW=0x100,L2CAP_FEAT_UCD=0x200,
 L2CAP_FCS_NONE=0,L2CAP_FCS_CRC16=1,L2CAP_FC_SIG_BREDR=2,L2CAP_FC_CONNLESS=4,L2CAP_FC_ATT=0x10,L2CAP_FC_SIG_LE=0x20,L2CAP_FC_SMP_LE=0x40,L2CAP_FC_SMP_BREDR=0x80,
 L2CAP_CTRL_SAR=0xc000,L2CAP_CTRL_REQSEQ=0x3f00,L2CAP_CTRL_TXSEQ=0x7e,L2CAP_CTRL_SUPERVISE=0xc,L2CAP_CTRL_RETRANS=0x80,L2CAP_CTRL_FINAL=0x80,L2CAP_CTRL_POLL=0x10,L2CAP_CTRL_FRAME_TYPE=1,L2CAP_CTRL_TXSEQ_SHIFT=1,L2CAP_CTRL_SUPER_SHIFT=2,L2CAP_CTRL_POLL_SHIFT=4,L2CAP_CTRL_FINAL_SHIFT=7,L2CAP_CTRL_REQSEQ_SHIFT=8,L2CAP_CTRL_SAR_SHIFT=14,
 L2CAP_EXT_CTRL_TXSEQ=0xfffc0000,L2CAP_EXT_CTRL_SAR=0x30000,L2CAP_EXT_CTRL_SUPERVISE=0x30000,L2CAP_EXT_CTRL_REQSEQ=0xfffc,L2CAP_EXT_CTRL_POLL=0x40000,L2CAP_EXT_CTRL_FINAL=2,L2CAP_EXT_CTRL_FRAME_TYPE=1,L2CAP_EXT_CTRL_FINAL_SHIFT=1,L2CAP_EXT_CTRL_REQSEQ_SHIFT=2,L2CAP_EXT_CTRL_SAR_SHIFT=16,L2CAP_EXT_CTRL_SUPER_SHIFT=16,L2CAP_EXT_CTRL_POLL_SHIFT=18,L2CAP_EXT_CTRL_TXSEQ_SHIFT=18,
 L2CAP_SUPER_RR=0,L2CAP_SUPER_REJ=1,L2CAP_SUPER_RNR=2,L2CAP_SUPER_SREJ=3,L2CAP_SAR_UNSEGMENTED=0,L2CAP_SAR_START=1,L2CAP_SAR_END=2,L2CAP_SAR_CONTINUE=3,L2CAP_REJ_NOT_UNDERSTOOD=0,L2CAP_REJ_MTU_EXCEEDED=1,L2CAP_REJ_INVALID_CID=2
}

#[repr(C)] pub struct sockaddr_l2 { pub l2_family: sa_family_t, pub l2_psm: __le16, pub l2_bdaddr: bdaddr_t, pub l2_cid: __le16, pub l2_bdaddr_type: __u8 }
#[repr(C)] pub struct l2cap_options { pub omtu: __u16,pub imtu: __u16,pub flush_to: __u16,pub mode: __u8,pub fcs: __u8,pub max_tx: __u8,pub txwin_size: __u16 }
#[repr(C)] pub struct l2cap_conninfo { pub hci_handle: __u16, pub dev_class: [__u8;3] }

#[repr(C, packed)] pub struct l2cap_hdr { pub len: __le16, pub cid: __le16 }
pub const L2CAP_LEN_SIZE:u32=2; pub const L2CAP_HDR_SIZE:u32=4; pub const L2CAP_ENH_HDR_SIZE:u32=6; pub const L2CAP_EXT_HDR_SIZE:u32=8; pub const L2CAP_FCS_SIZE:u32=2; pub const L2CAP_SDULEN_SIZE:u32=2; pub const L2CAP_PSMLEN_SIZE:u32=2; pub const L2CAP_ENH_CTRL_SIZE:u32=2; pub const L2CAP_EXT_CTRL_SIZE:u32=4;
#[repr(C, packed)] pub struct l2cap_cmd_hdr { pub code:__u8,pub ident:__u8,pub len:__le16 }
#[repr(C, packed)] pub struct l2cap_cmd_rej_unk { pub reason:__le16 }
#[repr(C, packed)] pub struct l2cap_cmd_rej_mtu { pub reason:__le16,pub max_mtu:__le16 }
#[repr(C, packed)] pub struct l2cap_cmd_rej_cid { pub reason:__le16,pub scid:__le16,pub dcid:__le16 }
#[repr(C, packed)] pub struct l2cap_conn_req { pub psm:__le16,pub scid:__le16 }
#[repr(C, packed)] pub struct l2cap_conn_rsp { pub dcid:__le16,pub scid:__le16,pub result:__le16,pub status:__le16 }

macro_rules! packed_struct { ($n:ident { $($f:ident : $t:ty),* $(,)? }) => { #[repr(C, packed)] pub struct $n { $(pub $f:$t,)* } }; }
packed_struct!(l2cap_disconn_req { dcid:__le16, scid:__le16 }); packed_struct!(l2cap_disconn_rsp { dcid:__le16, scid:__le16 });
packed_struct!(l2cap_info_req { r#type:__le16 }); packed_struct!(l2cap_info_rsp { r#type:__le16, result:__le16, data:[__u8;0] });
packed_struct!(l2cap_conf_req { dcid:__le16, flags:__le16, data:[__u8;0] }); packed_struct!(l2cap_conf_rsp { scid:__le16, flags:__le16, result:__le16, data:[__u8;0] });
packed_struct!(l2cap_conf_opt { r#type:__u8, len:__u8, val:[__u8;0] });
packed_struct!(l2cap_conf_rfc { mode:__u8, txwin_size:__u8, max_transmit:__u8, retrans_timeout:__le16, monitor_timeout:__le16, max_pdu_size:__le16 });
packed_struct!(l2cap_conf_efs { id:__u8, stype:__u8, msdu:__le16, sdu_itime:__le32, acc_lat:__le32, flush_to:__le32 });
packed_struct!(l2cap_conn_param_update_req { min:__le16, max:__le16, latency:__le16, to_multiplier:__le16 }); packed_struct!(l2cap_conn_param_update_rsp { result:__le16 });
packed_struct!(l2cap_le_conn_req { psm:__le16, scid:__le16, mtu:__le16, mps:__le16, credits:__le16 }); packed_struct!(l2cap_le_conn_rsp { dcid:__le16, mtu:__le16, mps:__le16, credits:__le16, result:__le16 }); packed_struct!(l2cap_le_credits { cid:__le16, credits:__le16 });
packed_struct!(l2cap_ecred_reconf_req { mtu:__le16, mps:__le16, scid:[__le16;0] }); packed_struct!(l2cap_ecred_reconf_rsp { result:__le16 });
packed_struct!(l2cap_move_chan_cfm { icid:__le16, result:__le16 }); packed_struct!(l2cap_move_chan_cfm_rsp { icid:__le16 });

pub const L2CAP_CMD_HDR_SIZE:u32=4; pub const L2CAP_CONF_OPT_SIZE:u32=2; pub const L2CAP_ECRED_MIN_MTU:u32=64; pub const L2CAP_ECRED_MIN_MPS:u32=64; pub const L2CAP_ECRED_MAX_CID:u32=5;
pub const L2CAP_CONF_SUCCESS:u32=0; pub const L2CAP_CONF_UNACCEPT:u32=1; pub const L2CAP_CONF_REJECT:u32=2; pub const L2CAP_CONF_UNKNOWN:u32=3; pub const L2CAP_CONF_PENDING:u32=4; pub const L2CAP_CONF_EFS_REJECT:u32=5;
pub const L2CAP_CONF_FLAG_CONTINUATION:u32=1; pub const L2CAP_CONF_HINT:u32=0x80; pub const L2CAP_CONF_MASK:u32=0x7f; pub const L2CAP_CONF_MTU:u32=1; pub const L2CAP_CONF_FLUSH_TO:u32=2; pub const L2CAP_CONF_QOS:u32=3; pub const L2CAP_CONF_RFC:u32=4; pub const L2CAP_CONF_FCS:u32=5; pub const L2CAP_CONF_EFS:u32=6; pub const L2CAP_CONF_EWS:u32=7; pub const L2CAP_CONF_MAX_SIZE:u32=22;
pub const L2CAP_MODE_BASIC:u32=0; pub const L2CAP_MODE_RETRANS:u32=1; pub const L2CAP_MODE_FLOWCTL:u32=2; pub const L2CAP_MODE_ERTM:u32=3; pub const L2CAP_MODE_STREAMING:u32=4; pub const L2CAP_MODE_LE_FLOWCTL:u32=0x80; pub const L2CAP_MODE_EXT_FLOWCTL:u32=0x81;

/* PSM, CID, result, information, and state constants retain their C names. */
macro_rules! c { ($($n:ident=$v:expr),* $(,)?) => { $(pub const $n:u32=$v;)* }; }
c!(L2CAP_PSM_SDP=1,L2CAP_PSM_RFCOMM=3,L2CAP_PSM_3DSP=0x21,L2CAP_PSM_IPSP=0x23,L2CAP_PSM_DYN_START=0x1001,L2CAP_PSM_DYN_END=0xffff,L2CAP_PSM_AUTO_END=0x10ff,L2CAP_PSM_LE_DYN_START=0x80,L2CAP_PSM_LE_DYN_END=0xff,L2CAP_CID_SIGNALING=1,L2CAP_CID_CONN_LESS=2,L2CAP_CID_ATT=4,L2CAP_CID_LE_SIGNALING=5,L2CAP_CID_SMP=6,L2CAP_CID_SMP_BREDR=7,L2CAP_CID_DYN_START=0x40,L2CAP_CID_DYN_END=0xffff,L2CAP_CID_LE_DYN_END=0x7f,L2CAP_CR_SUCCESS=0,L2CAP_CR_PEND=1,L2CAP_CR_BAD_PSM=2,L2CAP_CR_SEC_BLOCK=3,L2CAP_CR_NO_MEM=4,L2CAP_CR_INVALID_SCID=6,L2CAP_CR_SCID_IN_USE=7,L2CAP_CS_NO_INFO=0,L2CAP_CS_AUTHEN_PEND=1,L2CAP_CS_AUTHOR_PEND=2,L2CAP_CONF_MAX_CONF_REQ=2,L2CAP_CONF_MAX_CONF_RSP=2);

/* Remaining kernel structs and inline functions are declarations whose field types are external. */
#[repr(C)] pub struct l2cap_seq_list { pub head:__u16,pub tail:__u16,pub mask:__u16,pub list:*mut __u16 }
pub const L2CAP_SEQ_LIST_CLEAR:u32=0xffff; pub const L2CAP_SEQ_LIST_TAIL:u32=0x8000;
extern "C" { pub fn l2cap_chan_hold(c:*mut l2cap_chan); pub fn l2cap_chan_hold_unless_zero(c:*mut l2cap_chan)->*mut l2cap_chan; pub fn l2cap_chan_put(c:*mut l2cap_chan); pub fn l2cap_conn_get(c:*mut l2cap_conn)->*mut l2cap_conn; pub fn l2cap_conn_hold_unless_zero(c:*mut l2cap_conn)->*mut l2cap_conn; pub fn l2cap_conn_put(c:*mut l2cap_conn); }
#[repr(C)] pub struct l2cap_chan { _private: [u8;0] }
#[repr(C)] pub struct l2cap_conn { _private: [u8;0] }
#[repr(C)] pub struct l2cap_user { _private:[u8;0] }
pub type l2cap_chan_func_t = unsafe extern "C" fn(*mut l2cap_chan, *mut core::ffi::c_void);

extern "C" {
 pub fn l2cap_init_sockets()->i32; pub fn l2cap_cleanup_sockets(); pub fn l2cap_is_socket(sock:*mut socket)->bool;
 pub fn __l2cap_le_connect_rsp_defer(c:*mut l2cap_chan); pub fn __l2cap_ecred_conn_rsp_defer(c:*mut l2cap_chan); pub fn __l2cap_connect_rsp_defer(c:*mut l2cap_chan);
 pub fn l2cap_chan_create()->*mut l2cap_chan; pub fn l2cap_chan_close(c:*mut l2cap_chan, reason:i32); pub fn l2cap_chan_add(conn:*mut l2cap_conn,c:*mut l2cap_chan); pub fn __l2cap_chan_add(conn:*mut l2cap_conn,c:*mut l2cap_chan); pub fn l2cap_chan_del(c:*mut l2cap_chan,err:i32); pub fn l2cap_send_conn_req(c:*mut l2cap_chan);
 pub fn l2cap_chan_list(conn:*mut l2cap_conn,func:l2cap_chan_func_t,data:*mut core::ffi::c_void); pub fn l2cap_register_user(conn:*mut l2cap_conn,user:*mut l2cap_user)->i32; pub fn l2cap_unregister_user(conn:*mut l2cap_conn,user:*mut l2cap_user);
}

pub const L2CAP_CHAN_RAW:u32=1; pub const L2CAP_CHAN_CONN_LESS:u32=2; pub const L2CAP_CHAN_CONN_ORIENTED:u32=3; pub const L2CAP_CHAN_FIXED:u32=4;
pub const L2CAP_INFO_CL_MTU_REQ_SENT:u32=1; pub const L2CAP_INFO_FEAT_MASK_REQ_SENT:u32=4; pub const L2CAP_INFO_FEAT_MASK_REQ_DONE:u32=8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
