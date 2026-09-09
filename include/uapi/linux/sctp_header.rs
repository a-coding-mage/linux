/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Rust translation of the Linux SCTP userspace header. */

pub type sctp_assoc_t = i32;

pub const SCTP_FUTURE_ASSOC: i32 = 0;
pub const SCTP_CURRENT_ASSOC: i32 = 1;
pub const SCTP_ALL_ASSOC: i32 = 2;

pub const SCTP_RTOINFO: i32 = 0;
pub const SCTP_ASSOCINFO: i32 = 1;
pub const SCTP_INITMSG: i32 = 2;
pub const SCTP_NODELAY: i32 = 3;
pub const SCTP_AUTOCLOSE: i32 = 4;
pub const SCTP_SET_PEER_PRIMARY_ADDR: i32 = 5;
pub const SCTP_PRIMARY_ADDR: i32 = 6;
pub const SCTP_ADAPTATION_LAYER: i32 = 7;
pub const SCTP_DISABLE_FRAGMENTS: i32 = 8;
pub const SCTP_PEER_ADDR_PARAMS: i32 = 9;
pub const SCTP_DEFAULT_SEND_PARAM: i32 = 10;
pub const SCTP_EVENTS: i32 = 11;
pub const SCTP_I_WANT_MAPPED_V4_ADDR: i32 = 12;
pub const SCTP_MAXSEG: i32 = 13;
pub const SCTP_STATUS: i32 = 14;
pub const SCTP_GET_PEER_ADDR_INFO: i32 = 15;
pub const SCTP_DELAYED_ACK_TIME: i32 = 16;
pub const SCTP_DELAYED_ACK: i32 = SCTP_DELAYED_ACK_TIME;
pub const SCTP_DELAYED_SACK: i32 = SCTP_DELAYED_ACK_TIME;
pub const SCTP_CONTEXT: i32 = 17;
pub const SCTP_FRAGMENT_INTERLEAVE: i32 = 18;
pub const SCTP_PARTIAL_DELIVERY_POINT: i32 = 19;
pub const SCTP_MAX_BURST: i32 = 20;
pub const SCTP_AUTH_CHUNK: i32 = 21;
pub const SCTP_HMAC_IDENT: i32 = 22;
pub const SCTP_AUTH_KEY: i32 = 23;
pub const SCTP_AUTH_ACTIVE_KEY: i32 = 24;
pub const SCTP_AUTH_DELETE_KEY: i32 = 25;
pub const SCTP_PEER_AUTH_CHUNKS: i32 = 26;
pub const SCTP_LOCAL_AUTH_CHUNKS: i32 = 27;
pub const SCTP_GET_ASSOC_NUMBER: i32 = 28;
pub const SCTP_GET_ASSOC_ID_LIST: i32 = 29;
pub const SCTP_AUTO_ASCONF: i32 = 30;
pub const SCTP_PEER_ADDR_THLDS: i32 = 31;
pub const SCTP_RECVRCVINFO: i32 = 32;
pub const SCTP_RECVNXTINFO: i32 = 33;
pub const SCTP_DEFAULT_SNDINFO: i32 = 34;
pub const SCTP_AUTH_DEACTIVATE_KEY: i32 = 35;
pub const SCTP_REUSE_PORT: i32 = 36;
pub const SCTP_PEER_ADDR_THLDS_V2: i32 = 37;
pub const SCTP_SOCKOPT_BINDX_ADD: i32 = 100;
pub const SCTP_SOCKOPT_BINDX_REM: i32 = 101;
pub const SCTP_SOCKOPT_PEELOFF: i32 = 102;
pub const SCTP_SOCKOPT_CONNECTX_OLD: i32 = 107;
pub const SCTP_GET_PEER_ADDRS: i32 = 108;
pub const SCTP_GET_LOCAL_ADDRS: i32 = 109;
pub const SCTP_SOCKOPT_CONNECTX: i32 = 110;
pub const SCTP_SOCKOPT_CONNECTX3: i32 = 111;
pub const SCTP_GET_ASSOC_STATS: i32 = 112;
pub const SCTP_PR_SUPPORTED: i32 = 113;
pub const SCTP_DEFAULT_PRINFO: i32 = 114;
pub const SCTP_PR_ASSOC_STATUS: i32 = 115;
pub const SCTP_PR_STREAM_STATUS: i32 = 116;
pub const SCTP_RECONFIG_SUPPORTED: i32 = 117;
pub const SCTP_ENABLE_STREAM_RESET: i32 = 118;
pub const SCTP_RESET_STREAMS: i32 = 119;
pub const SCTP_RESET_ASSOC: i32 = 120;
pub const SCTP_ADD_STREAMS: i32 = 121;
pub const SCTP_SOCKOPT_PEELOFF_FLAGS: i32 = 122;
pub const SCTP_STREAM_SCHEDULER: i32 = 123;
pub const SCTP_STREAM_SCHEDULER_VALUE: i32 = 124;
pub const SCTP_INTERLEAVING_SUPPORTED: i32 = 125;
pub const SCTP_SENDMSG_CONNECT: i32 = 126;
pub const SCTP_EVENT: i32 = 127;
pub const SCTP_ASCONF_SUPPORTED: i32 = 128;
pub const SCTP_AUTH_SUPPORTED: i32 = 129;
pub const SCTP_ECN_SUPPORTED: i32 = 130;
pub const SCTP_EXPOSE_POTENTIALLY_FAILED_STATE: i32 = 131;
pub const SCTP_EXPOSE_PF_STATE: i32 = SCTP_EXPOSE_POTENTIALLY_FAILED_STATE;
pub const SCTP_REMOTE_UDP_ENCAPS_PORT: i32 = 132;
pub const SCTP_PLPMTUD_PROBE_INTERVAL: i32 = 133;

pub const SCTP_PR_SCTP_NONE: u16 = 0x0000;
pub const SCTP_PR_SCTP_TTL: u16 = 0x0010;
pub const SCTP_PR_SCTP_RTX: u16 = 0x0020;
pub const SCTP_PR_SCTP_PRIO: u16 = 0x0030;
pub const SCTP_PR_SCTP_MAX: u16 = SCTP_PR_SCTP_PRIO;
pub const SCTP_PR_SCTP_MASK: u16 = 0x0030;
pub const SCTP_ENABLE_RESET_STREAM_REQ: u8 = 1;
pub const SCTP_ENABLE_RESET_ASSOC_REQ: u8 = 2;
pub const SCTP_ENABLE_CHANGE_ASSOC_REQ: u8 = 4;
pub const SCTP_ENABLE_STRRESET_MASK: u8 = 7;
pub const SCTP_STREAM_RESET_INCOMING: u8 = 1;
pub const SCTP_STREAM_RESET_OUTGOING: u8 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_initmsg { pub sinit_num_ostreams: u16, pub sinit_max_instreams: u16, pub sinit_max_attempts: u16, pub sinit_max_init_timeo: u16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_sndrcvinfo { pub sinfo_stream: u16, pub sinfo_ssn: u16, pub sinfo_flags: u16, pub sinfo_ppid: u32, pub sinfo_context: u32, pub sinfo_timetolive: u32, pub sinfo_tsn: u32, pub sinfo_cumtsn: u32, pub sinfo_assoc_id: sctp_assoc_t }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_sndinfo { pub snd_sid: u16, pub snd_flags: u16, pub snd_ppid: u32, pub snd_context: u32, pub snd_assoc_id: sctp_assoc_t }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_rcvinfo { pub rcv_sid: u16, pub rcv_ssn: u16, pub rcv_flags: u16, pub rcv_ppid: u32, pub rcv_tsn: u32, pub rcv_cumtsn: u32, pub rcv_context: u32, pub rcv_assoc_id: sctp_assoc_t }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_nxtinfo { pub nxt_sid: u16, pub nxt_flags: u16, pub nxt_ppid: u32, pub nxt_length: u32, pub nxt_assoc_id: sctp_assoc_t }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_prinfo { pub pr_policy: u16, pub pr_value: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_authinfo { pub auth_keynumber: u16 }

pub const MSG_NOTIFICATION: u16 = 0x8000;
pub const SCTP_UNORDERED: u16 = 1 << 0;
pub const SCTP_ADDR_OVER: u16 = 1 << 1;
pub const SCTP_ABORT: u16 = 1 << 2;
pub const SCTP_SACK_IMMEDIATELY: u16 = 1 << 3;
pub const SCTP_SENDALL: u16 = 1 << 6;
pub const SCTP_PR_SCTP_ALL: u16 = 1 << 7;
pub const SCTP_NOTIFICATION: u16 = MSG_NOTIFICATION;
// MSG_FIN is supplied by the socket API dependency.
pub const SCTP_EOF: u16 = MSG_FIN;

#[repr(C)]
pub union sctp_cmsg_data_t { pub raw: u8, pub init: sctp_initmsg, pub sndrcv: sctp_sndrcvinfo }
pub type sctp_cmsg_t = u32;
pub const SCTP_INIT: sctp_cmsg_t = 0;
pub const SCTP_SNDRCV: sctp_cmsg_t = 1;
pub const SCTP_SNDINFO: sctp_cmsg_t = 2;
pub const SCTP_RCVINFO: sctp_cmsg_t = 3;
pub const SCTP_NXTINFO: sctp_cmsg_t = 4;
pub const SCTP_PRINFO: sctp_cmsg_t = 5;
pub const SCTP_AUTHINFO: sctp_cmsg_t = 6;
pub const SCTP_DSTADDRV4: sctp_cmsg_t = 7;
pub const SCTP_DSTADDRV6: sctp_cmsg_t = 8;
#[repr(C)] pub struct sctp_assoc_change { pub sac_type:u16,pub sac_flags:u16,pub sac_length:u32,pub sac_state:u16,pub sac_error:u16,pub sac_outbound_streams:u16,pub sac_inbound_streams:u16,pub sac_assoc_id:sctp_assoc_t,pub sac_info:[u8;0] }
#[repr(C)] pub struct sctp_rtoinfo { pub srto_assoc_id:sctp_assoc_t,pub srto_initial:u32,pub srto_max:u32,pub srto_min:u32 }
#[repr(C)] pub struct sctp_assocparams { pub sasoc_assoc_id:sctp_assoc_t,pub sasoc_asocmaxrxt:u16,pub sasoc_number_peer_destinations:u16,pub sasoc_peer_rwnd:u32,pub sasoc_local_rwnd:u32,pub sasoc_cookie_life:u32 }
#[repr(C)] pub struct sctp_setadaptation { pub ssb_adaptation_ind:u32 }
#[repr(C)] pub struct sctp_sack_info { pub sack_assoc_id:sctp_assoc_t,pub sack_delay:u32,pub sack_freq:u32 }
#[repr(C)] pub struct sctp_assoc_value { pub assoc_id:sctp_assoc_t,pub assoc_value:u32 }
#[repr(C)] pub struct sctp_stream_value { pub assoc_id:sctp_assoc_t,pub stream_id:u16,pub stream_value:u16 }
#[repr(C)] pub struct sctp_authchunk { pub sauth_chunk:u8 }
#[repr(C)] pub struct sctp_authkeyid { pub scact_assoc_id:sctp_assoc_t,pub scact_keynumber:u16 }
#[repr(C)] pub struct sctp_add_streams { pub sas_assoc_id:sctp_assoc_t,pub sas_instrms:u16,pub sas_outstrms:u16 }
#[repr(C)] pub struct sctp_event { pub se_assoc_id:sctp_assoc_t,pub se_type:u16,pub se_on:u8 }
#[repr(C)] pub struct sctp_udpencaps { pub sue_assoc_id:sctp_assoc_t,pub sue_address:sockaddr_storage,pub sue_port:u16 }
#[repr(C)] pub struct sctp_probeinterval { pub spi_assoc_id:sctp_assoc_t,pub spi_address:sockaddr_storage,pub spi_interval:u32 }

#[repr(C)] pub struct sctp_assoc_change { pub sac_type:u16,pub sac_flags:u16,pub sac_length:u32,pub sac_state:u16,pub sac_error:u16,pub sac_outbound_streams:u16,pub sac_inbound_streams:u16,pub sac_assoc_id:sctp_assoc_t,pub sac_info:[u8;0] }
#[repr(C, packed(4))] pub struct sctp_paddr_change { pub spc_type:u16,pub spc_flags:u16,pub spc_length:u32,pub spc_aaddr:sockaddr_storage,pub spc_state:i32,pub spc_error:i32,pub spc_assoc_id:sctp_assoc_t }
#[repr(C)] pub struct sctp_remote_error { pub sre_type:u16,pub sre_flags:u16,pub sre_length:u32,pub sre_error:u16,pub sre_assoc_id:sctp_assoc_t,pub sre_data:[u8;0] }
#[repr(C)] pub struct sctp_send_failed { pub ssf_type:u16,pub ssf_flags:u16,pub ssf_length:u32,pub ssf_error:u32,pub ssf_info:sctp_sndrcvinfo,pub ssf_assoc_id:sctp_assoc_t,pub ssf_data:[u8;0] }
#[repr(C)] pub struct sctp_send_failed_event { pub ssf_type:u16,pub ssf_flags:u16,pub ssf_length:u32,pub ssf_error:u32,pub ssfe_info:sctp_sndinfo,pub ssf_assoc_id:sctp_assoc_t,pub ssf_data:[u8;0] }
#[repr(C)] pub struct sctp_shutdown_event { pub sse_type:u16,pub sse_flags:u16,pub sse_length:u32,pub sse_assoc_id:sctp_assoc_t }
#[repr(C)] pub struct sctp_adaptation_event { pub sai_type:u16,pub sai_flags:u16,pub sai_length:u32,pub sai_adaptation_ind:u32,pub sai_assoc_id:sctp_assoc_t }
#[repr(C)] pub struct sctp_pdapi_event { pub pdapi_type:u16,pub pdapi_flags:u16,pub pdapi_length:u32,pub pdapi_indication:u32,pub pdapi_assoc_id:sctp_assoc_t,pub pdapi_stream:u32,pub pdapi_seq:u32 }
pub const SCTP_PARTIAL_DELIVERY_ABORTED: u32 = 0;
#[repr(C)] pub struct sctp_authkey_event { pub auth_type:u16,pub auth_flags:u16,pub auth_length:u32,pub auth_keynumber:u16,pub auth_altkeynumber:u16,pub auth_indication:u32,pub auth_assoc_id:sctp_assoc_t }
pub const SCTP_AUTH_NEW_KEY:u32=0; pub const SCTP_AUTH_NEWKEY:u32=SCTP_AUTH_NEW_KEY; pub const SCTP_AUTH_FREE_KEY:u32=1; pub const SCTP_AUTH_NO_AUTH:u32=2;
#[repr(C)] pub struct sctp_sender_dry_event { pub sender_dry_type:u16,pub sender_dry_flags:u16,pub sender_dry_length:u32,pub sender_dry_assoc_id:sctp_assoc_t }
#[repr(C)] pub struct sctp_stream_reset_event { pub strreset_type:u16,pub strreset_flags:u16,pub strreset_length:u32,pub strreset_assoc_id:sctp_assoc_t,pub strreset_stream_list:[u16;0] }
#[repr(C)] pub struct sctp_assoc_reset_event { pub assocreset_type:u16,pub assocreset_flags:u16,pub assocreset_length:u32,pub assocreset_assoc_id:sctp_assoc_t,pub assocreset_local_tsn:u32,pub assocreset_remote_tsn:u32 }
#[repr(C)] pub struct sctp_stream_change_event { pub strchange_type:u16,pub strchange_flags:u16,pub strchange_length:u32,pub strchange_assoc_id:sctp_assoc_t,pub strchange_instrms:u16,pub strchange_outstrms:u16 }
#[repr(C)] pub struct sctp_event_subscribe { pub sctp_data_io_event:u8,pub sctp_association_event:u8,pub sctp_address_event:u8,pub sctp_send_failure_event:u8,pub sctp_peer_error_event:u8,pub sctp_shutdown_event:u8,pub sctp_partial_delivery_event:u8,pub sctp_adaptation_layer_event:u8,pub sctp_authentication_event:u8,pub sctp_sender_dry_event:u8,pub sctp_stream_reset_event:u8,pub sctp_assoc_reset_event:u8,pub sctp_stream_change_event:u8,pub sctp_send_failure_event_event:u8 }

#[repr(C)] pub union sctp_notification { pub sn_header:sctp_notification_header, pub sn_assoc_change:sctp_assoc_change, pub sn_paddr_change:sctp_paddr_change, pub sn_remote_error:sctp_remote_error, pub sn_send_failed:sctp_send_failed, pub sn_shutdown_event:sctp_shutdown_event, pub sn_adaptation_event:sctp_adaptation_event, pub sn_pdapi_event:sctp_pdapi_event, pub sn_authkey_event:sctp_authkey_event, pub sn_sender_dry_event:sctp_sender_dry_event, pub sn_strreset_event:sctp_stream_reset_event, pub sn_assocreset_event:sctp_assoc_reset_event, pub sn_strchange_event:sctp_stream_change_event, pub sn_send_failed_event:sctp_send_failed_event }
#[repr(C)] #[derive(Copy,Clone)] pub struct sctp_notification_header { pub sn_type:u16,pub sn_flags:u16,pub sn_length:u32 }
pub const SCTP_SN_TYPE_BASE:u16=1<<15; pub const SCTP_DATA_IO_EVENT:u16=SCTP_SN_TYPE_BASE; pub const SCTP_ASSOC_CHANGE:u16=SCTP_DATA_IO_EVENT+1; pub const SCTP_PEER_ADDR_CHANGE:u16=SCTP_ASSOC_CHANGE+1; pub const SCTP_SEND_FAILED:u16=SCTP_PEER_ADDR_CHANGE+1; pub const SCTP_REMOTE_ERROR:u16=SCTP_SEND_FAILED+1; pub const SCTP_SHUTDOWN_EVENT:u16=SCTP_REMOTE_ERROR+1; pub const SCTP_PARTIAL_DELIVERY_EVENT:u16=SCTP_SHUTDOWN_EVENT+1; pub const SCTP_ADAPTATION_INDICATION:u16=SCTP_PARTIAL_DELIVERY_EVENT+1; pub const SCTP_AUTHENTICATION_EVENT:u16=SCTP_ADAPTATION_INDICATION+1; pub const SCTP_SENDER_DRY_EVENT:u16=SCTP_AUTHENTICATION_EVENT+1; pub const SCTP_STREAM_RESET_EVENT:u16=SCTP_SENDER_DRY_EVENT+1; pub const SCTP_ASSOC_RESET_EVENT:u16=SCTP_STREAM_RESET_EVENT+1; pub const SCTP_STREAM_CHANGE_EVENT:u16=SCTP_ASSOC_RESET_EVENT+1; pub const SCTP_SEND_FAILED_EVENT:u16=SCTP_STREAM_CHANGE_EVENT+1; pub const SCTP_SN_TYPE_MAX:u16=SCTP_SEND_FAILED_EVENT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
