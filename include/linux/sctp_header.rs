/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SCTP kernel reference Implementation and protocol defined structures. */

/* Dependencies supplied by the surrounding translation unit: linux/in.h,
 * linux/in6.h, linux/skbuff.h, and uapi/linux/sctp.h. */

#[repr(C)]
pub struct sctphdr { pub source: __be16, pub dest: __be16, pub vtag: __be32, pub checksum: __le32 }

#[inline]
pub unsafe fn sctp_hdr(skb: *const struct_sk_buff) -> *mut sctphdr {
    skb_transport_header(skb) as *mut sctphdr
}

#[repr(C)]
pub struct sctp_chunkhdr { pub r#type: __u8, pub flags: __u8, pub length: __be16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum sctp_cid {
    SCTP_CID_DATA = 0, SCTP_CID_INIT = 1, SCTP_CID_INIT_ACK = 2,
    SCTP_CID_SACK = 3, SCTP_CID_HEARTBEAT = 4, SCTP_CID_HEARTBEAT_ACK = 5,
    SCTP_CID_ABORT = 6, SCTP_CID_SHUTDOWN = 7, SCTP_CID_SHUTDOWN_ACK = 8,
    SCTP_CID_ERROR = 9, SCTP_CID_COOKIE_ECHO = 10, SCTP_CID_COOKIE_ACK = 11,
    SCTP_CID_ECN_ECNE = 12, SCTP_CID_ECN_CWR = 13, SCTP_CID_SHUTDOWN_COMPLETE = 14,
    SCTP_CID_AUTH = 0x0f, SCTP_CID_I_DATA = 0x40, SCTP_CID_FWD_TSN = 0xc0,
    SCTP_CID_ASCONF = 0xc1, SCTP_CID_I_FWD_TSN = 0xc2, SCTP_CID_ASCONF_ACK = 0x80,
    SCTP_CID_RECONF = 0x82, SCTP_CID_PAD = 0x84,
}

pub const SCTP_CID_ACTION_DISCARD: u8 = 0x00;
pub const SCTP_CID_ACTION_DISCARD_ERR: u8 = 0x40;
pub const SCTP_CID_ACTION_SKIP: u8 = 0x80;
pub const SCTP_CID_ACTION_SKIP_ERR: u8 = 0xc0;
pub const SCTP_CID_ACTION_MASK: u8 = 0xc0;
pub const SCTP_CHUNK_FLAG_T: u8 = 0x01;

#[inline]
pub unsafe fn sctp_test_T_bit(c: *const sctp_chunk) -> __u8 { (*(*c).chunk_hdr).flags & SCTP_CHUNK_FLAG_T }

#[repr(C)] pub struct sctp_paramhdr { pub r#type: __be16, pub length: __be16 }
#[repr(C)] pub enum sctp_param {
    SCTP_PARAM_HEARTBEAT_INFO = 1, SCTP_PARAM_IPV4_ADDRESS = 5, SCTP_PARAM_IPV6_ADDRESS = 6,
    SCTP_PARAM_STATE_COOKIE = 7, SCTP_PARAM_UNRECOGNIZED_PARAMETERS = 8,
    SCTP_PARAM_COOKIE_PRESERVATIVE = 9, SCTP_PARAM_HOST_NAME_ADDRESS = 11,
    SCTP_PARAM_SUPPORTED_ADDRESS_TYPES = 12, SCTP_PARAM_ECN_CAPABLE = 0x8000,
    SCTP_PARAM_RANDOM = 0x8002, SCTP_PARAM_CHUNKS = 0x8003, SCTP_PARAM_HMAC_ALGO = 0x8004,
    SCTP_PARAM_SUPPORTED_EXT = 0x8008, SCTP_PARAM_FWD_TSN_SUPPORT = 0xc000,
    SCTP_PARAM_ADD_IP = 0xc001, SCTP_PARAM_DEL_IP = 0xc002, SCTP_PARAM_ERR_CAUSE = 0xc003,
    SCTP_PARAM_SET_PRIMARY = 0xc004, SCTP_PARAM_SUCCESS_REPORT = 0xc005,
    SCTP_PARAM_ADAPTATION_LAYER_IND = 0xc006, SCTP_PARAM_RESET_OUT_REQUEST = 0x000d,
    SCTP_PARAM_RESET_IN_REQUEST = 0x000e, SCTP_PARAM_RESET_TSN_REQUEST = 0x000f,
    SCTP_PARAM_RESET_RESPONSE = 0x0010, SCTP_PARAM_RESET_ADD_OUT_STREAMS = 0x0011,
    SCTP_PARAM_RESET_ADD_IN_STREAMS = 0x0012,
}
pub const SCTP_PARAM_ACTION_DISCARD: u16 = 0x0000; pub const SCTP_PARAM_ACTION_DISCARD_ERR: u16 = 0x4000;
pub const SCTP_PARAM_ACTION_SKIP: u16 = 0x8000; pub const SCTP_PARAM_ACTION_SKIP_ERR: u16 = 0xc000;
pub const SCTP_PARAM_ACTION_MASK: u16 = 0xc000;

#[repr(C)] pub enum sctp_error {
    SCTP_ERROR_NO_ERROR=0, SCTP_ERROR_INV_STRM=1, SCTP_ERROR_MISS_PARAM=2,
    SCTP_ERROR_STALE_COOKIE=3, SCTP_ERROR_NO_RESOURCE=4, SCTP_ERROR_DNS_FAILED=5,
    SCTP_ERROR_UNKNOWN_CHUNK=6, SCTP_ERROR_INV_PARAM=7, SCTP_ERROR_UNKNOWN_PARAM=8,
    SCTP_ERROR_NO_DATA=9, SCTP_ERROR_COOKIE_IN_SHUTDOWN=0x0a,
    SCTP_ERROR_RESTART=0x0b, SCTP_ERROR_USER_ABORT=0x0c,
    SCTP_ERROR_PROTO_VIOLATION=0x0d, SCTP_ERROR_NEW_ENCAP_PORT=0x0e,
    SCTP_ERROR_DEL_LAST_IP=0x00a0, SCTP_ERROR_RSRC_LOW=0x00a1,
    SCTP_ERROR_DEL_SRC_IP=0x00a2, SCTP_ERROR_ASCONF_ACK=0x00a3,
    SCTP_ERROR_REQ_REFUSED=0x00a4, SCTP_ERROR_UNSUP_HMAC=0x0105,
}

#[repr(C)] pub struct sctp_datahdr { pub tsn: __be32, pub stream: __be16, pub ssn: __be16, pub ppid: __u32 }
#[repr(C)] pub struct sctp_data_chunk { pub chunk_hdr: sctp_chunkhdr, pub data_hdr: sctp_datahdr }
#[repr(C)] pub union sctp_idatahdr_ppid_fsn { pub ppid: __u32, pub fsn: __be32 }
#[repr(C)] pub struct sctp_idatahdr { pub tsn: __be32, pub stream: __be16, pub reserved: __be16, pub mid: __be32, pub ppid_fsn: sctp_idatahdr_ppid_fsn }
#[repr(C)] pub struct sctp_idata_chunk { pub chunk_hdr: sctp_chunkhdr, pub data_hdr: sctp_idatahdr }
pub const SCTP_DATA_MIDDLE_FRAG: u8=0; pub const SCTP_DATA_LAST_FRAG: u8=1; pub const SCTP_DATA_FIRST_FRAG: u8=2; pub const SCTP_DATA_NOT_FRAG: u8=3; pub const SCTP_DATA_UNORDERED: u8=4; pub const SCTP_DATA_SACK_IMM: u8=8; pub const SCTP_DATA_FRAG_MASK: u8=3;

#[repr(C)] pub struct sctp_inithdr { pub init_tag: __be32, pub a_rwnd: __be32, pub num_outbound_streams: __be16, pub num_inbound_streams: __be16, pub initial_tsn: __be32 }
#[repr(C)] pub struct sctp_init_chunk { pub chunk_hdr: sctp_chunkhdr, pub init_hdr: sctp_inithdr }
#[repr(C)] pub struct sctp_ipv4addr_param { pub param_hdr: sctp_paramhdr, pub addr: in_addr }
#[repr(C)] pub struct sctp_ipv6addr_param { pub param_hdr: sctp_paramhdr, pub addr: in6_addr }
#[repr(C)] pub struct sctp_cookie_preserve_param { pub param_hdr: sctp_paramhdr, pub lifespan_increment: __be32 }
#[repr(C)] pub struct sctp_hostname_param { pub param_hdr: sctp_paramhdr, pub hostname: [u8; 0] }
#[repr(C)] pub struct sctp_supported_addrs_param { pub param_hdr: sctp_paramhdr, pub types: [__be16; 0] }
#[repr(C)] pub struct sctp_adaptation_ind_param { pub param_hdr: sctp_paramhdr, pub adaptation_ind: __be32 }
#[repr(C)] pub struct sctp_supported_ext_param { pub param_hdr: sctp_paramhdr, pub chunks: [__u8; 0] }
#[repr(C)] pub struct sctp_random_param { pub param_hdr: sctp_paramhdr, pub random_val: [__u8; 0] }
#[repr(C)] pub struct sctp_chunks_param { pub param_hdr: sctp_paramhdr, pub chunks: [__u8; 0] }
#[repr(C)] pub struct sctp_hmac_algo_param { pub param_hdr: sctp_paramhdr, pub hmac_ids: [__be16; 0] }
#[repr(C)] pub struct sctp_initack_chunk { pub chunk_hdr: sctp_chunkhdr, pub init_hdr: sctp_inithdr }
#[repr(C)] pub struct sctp_cookie_param { pub p: sctp_paramhdr, pub body: [__u8; 0] }
#[repr(C)] pub struct sctp_unrecognized_param { pub param_hdr: sctp_paramhdr, pub unrecognized: sctp_paramhdr }

#[repr(C)] pub struct sctp_gap_ack_block { pub start: __be16, pub end: __be16 }
#[repr(C)] pub union sctp_sack_variable { pub gab: sctp_gap_ack_block, pub dup: __be32 }
#[repr(C)] pub struct sctp_sackhdr { pub cum_tsn_ack: __be32, pub a_rwnd: __be32, pub num_gap_ack_blocks: __be16, pub num_dup_tsns: __be16 }
#[repr(C)] pub struct sctp_sack_chunk { pub chunk_hdr: sctp_chunkhdr, pub sack_hdr: sctp_sackhdr }
#[repr(C)] pub struct sctp_heartbeathdr { pub info: sctp_paramhdr }
#[repr(C)] pub struct sctp_heartbeat_chunk { pub chunk_hdr: sctp_chunkhdr, pub hb_hdr: sctp_heartbeathdr }
#[repr(C)] pub struct sctp_pad_chunk { pub uh: sctp_chunkhdr }
#[repr(C)] pub struct sctp_abort_chunk { pub uh: sctp_chunkhdr }
#[repr(C)] pub struct sctp_shutdownhdr { pub cum_tsn_ack: __be32 }
#[repr(C)] pub struct sctp_shutdown_chunk { pub chunk_hdr: sctp_chunkhdr, pub shutdown_hdr: sctp_shutdownhdr }
#[repr(C)] pub struct sctp_errhdr { pub cause: __be16, pub length: __be16 }
#[repr(C)] pub struct sctp_operr_chunk { pub chunk_hdr: sctp_chunkhdr, pub err_hdr: sctp_errhdr }

#[repr(C)] pub struct sctp_ecnehdr { pub lowest_tsn: __be32 }
#[repr(C)] pub struct sctp_ecne_chunk { pub chunk_hdr: sctp_chunkhdr, pub ence_hdr: sctp_ecnehdr }
#[repr(C)] pub struct sctp_cwrhdr { pub lowest_tsn: __be32 }
#[repr(C)] pub struct sctp_fwdtsn_skip { pub stream: __be16, pub ssn: __be16 }
#[repr(C)] pub struct sctp_fwdtsn_hdr { pub new_cum_tsn: __be32 }
#[repr(C)] pub struct sctp_fwdtsn_chunk { pub chunk_hdr: sctp_chunkhdr, pub fwdtsn_hdr: sctp_fwdtsn_hdr }
#[repr(C)] pub struct sctp_ifwdtsn_skip { pub stream: __be16, pub reserved: __u8, pub flags: __u8, pub mid: __be32 }
#[repr(C)] pub struct sctp_ifwdtsn_hdr { pub new_cum_tsn: __be32 }
#[repr(C)] pub struct sctp_ifwdtsn_chunk { pub chunk_hdr: sctp_chunkhdr, pub fwdtsn_hdr: sctp_ifwdtsn_hdr }
#[repr(C)] pub struct sctp_addip_param { pub param_hdr: sctp_paramhdr, pub crr_id: __be32 }
#[repr(C)] pub struct sctp_addiphdr { pub serial: __be32 }
#[repr(C)] pub struct sctp_addip_chunk { pub chunk_hdr: sctp_chunkhdr, pub addip_hdr: sctp_addiphdr }
#[repr(C)] pub struct sctp_authhdr { pub shkey_id: __be16, pub hmac_id: __be16 }
#[repr(C)] pub struct sctp_auth_chunk { pub chunk_hdr: sctp_chunkhdr, pub auth_hdr: sctp_authhdr }
#[repr(C)] pub struct sctp_infox { pub sctpinfo: *mut sctp_info, pub asoc: *mut sctp_association }
#[repr(C)] pub struct sctp_reconf_chunk { pub chunk_hdr: sctp_chunkhdr }
#[repr(C)] pub struct sctp_strreset_outreq { pub param_hdr: sctp_paramhdr, pub request_seq: __be32, pub response_seq: __be32, pub send_reset_at_tsn: __be32, pub list_of_streams: [__be16; 0] }
#[repr(C)] pub struct sctp_strreset_inreq { pub param_hdr: sctp_paramhdr, pub request_seq: __be32, pub list_of_streams: [__be16; 0] }
#[repr(C)] pub struct sctp_strreset_tsnreq { pub param_hdr: sctp_paramhdr, pub request_seq: __be32 }
#[repr(C)] pub struct sctp_strreset_addstrm { pub param_hdr: sctp_paramhdr, pub request_seq: __be32, pub number_of_streams: __be16, pub reserved: __be16 }
pub const SCTP_STRRESET_NOTHING_TO_DO:u32=0; pub const SCTP_STRRESET_PERFORMED:u32=1; pub const SCTP_STRRESET_DENIED:u32=2; pub const SCTP_STRRESET_ERR_WRONG_SSN:u32=3; pub const SCTP_STRRESET_ERR_IN_PROGRESS:u32=4; pub const SCTP_STRRESET_ERR_BAD_SEQNO:u32=5; pub const SCTP_STRRESET_IN_PROGRESS:u32=6;
#[repr(C)] pub struct sctp_strreset_resp { pub param_hdr: sctp_paramhdr, pub response_seq: __be32, pub result: __be32 }
#[repr(C)] pub struct sctp_strreset_resptsn { pub param_hdr: sctp_paramhdr, pub response_seq: __be32, pub result: __be32, pub senders_next_tsn: __be32, pub receivers_next_tsn: __be32 }
pub const SCTP_DSCP_SET_MASK:u32=0x1; pub const SCTP_DSCP_VAL_MASK:u32=0xfc; pub const SCTP_FLOWLABEL_SET_MASK:u32=0x100000; pub const SCTP_FLOWLABEL_VAL_MASK:u32=0xfffff;
#[repr(C)] pub struct sctp_new_encap_port_hdr { pub cur_port: __be16, pub new_port: __be16 }
#[inline] pub const fn SCTP_PAD4(s: u32) -> u32 { (s.wrapping_add(3)) & !3 }
#[inline] pub const fn SCTP_TRUNC4(s: u32) -> u32 { s & !3 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
