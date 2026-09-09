/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/* Translated from the Linux RDS UAPI header. */

// Dependencies supplied by the surrounding UAPI translation:
// __kernel_sockaddr_storage and in6_addr.

pub const RDS_IB_ABI_VERSION: u32 = 0x301;
pub const SOL_RDS: u32 = 276;
pub const RDS_CANCEL_SENT_TO: u32 = 1;
pub const RDS_GET_MR: u32 = 2;
pub const RDS_FREE_MR: u32 = 3;
pub const RDS_RECVERR: u32 = 5;
pub const RDS_CONG_MONITOR: u32 = 6;
pub const RDS_GET_MR_FOR_DEST: u32 = 7;
pub const SO_RDS_TRANSPORT: u32 = 8;
pub const SO_RDS_MSG_RXPATH_LATENCY: u32 = 10;
pub const RDS_TRANS_IB: u32 = 0;
pub const RDS_TRANS_GAP: u32 = 1;
pub const RDS_TRANS_TCP: u32 = 2;
pub const RDS_TRANS_COUNT: u32 = 3;
pub const RDS_TRANS_NONE: u32 = !0;
pub const RDS_TRANS_IWARP: u32 = RDS_TRANS_GAP;
// SIOCPROTOPRIVATE is supplied by the socket UAPI header.
pub const SIOCRDSSETTOS: u32 = SIOCPROTOPRIVATE;
pub const SIOCRDSGETTOS: u32 = SIOCPROTOPRIVATE + 1;
pub type rds_tos_t = u8;

pub const RDS_CMSG_RDMA_ARGS: u32 = 1;
pub const RDS_CMSG_RDMA_DEST: u32 = 2;
pub const RDS_CMSG_RDMA_MAP: u32 = 3;
pub const RDS_CMSG_RDMA_STATUS: u32 = 4;
pub const RDS_CMSG_CONG_UPDATE: u32 = 5;
pub const RDS_CMSG_ATOMIC_FADD: u32 = 6;
pub const RDS_CMSG_ATOMIC_CSWP: u32 = 7;
pub const RDS_CMSG_MASKED_ATOMIC_FADD: u32 = 8;
pub const RDS_CMSG_MASKED_ATOMIC_CSWP: u32 = 9;
pub const RDS_CMSG_RXPATH_LATENCY: u32 = 11;
pub const RDS_CMSG_ZCOPY_COOKIE: u32 = 12;
pub const RDS_CMSG_ZCOPY_COMPLETION: u32 = 13;

pub const RDS_INFO_FIRST: u32 = 10000;
pub const RDS_INFO_COUNTERS: u32 = 10000;
pub const RDS_INFO_CONNECTIONS: u32 = 10001;
pub const RDS_INFO_SEND_MESSAGES: u32 = 10003;
pub const RDS_INFO_RETRANS_MESSAGES: u32 = 10004;
pub const RDS_INFO_RECV_MESSAGES: u32 = 10005;
pub const RDS_INFO_SOCKETS: u32 = 10006;
pub const RDS_INFO_TCP_SOCKETS: u32 = 10007;
pub const RDS_INFO_IB_CONNECTIONS: u32 = 10008;
pub const RDS_INFO_CONNECTION_STATS: u32 = 10009;
pub const RDS_INFO_IWARP_CONNECTIONS: u32 = 10010;
pub const RDS6_INFO_CONNECTIONS: u32 = 10011;
pub const RDS6_INFO_SEND_MESSAGES: u32 = 10012;
pub const RDS6_INFO_RETRANS_MESSAGES: u32 = 10013;
pub const RDS6_INFO_RECV_MESSAGES: u32 = 10014;
pub const RDS6_INFO_SOCKETS: u32 = 10015;
pub const RDS6_INFO_TCP_SOCKETS: u32 = 10016;
pub const RDS6_INFO_IB_CONNECTIONS: u32 = 10017;
pub const RDS_INFO_LAST: u32 = 10017;

pub const RDS_INFO_CONNECTION_FLAG_SENDING: u8 = 0x01;
pub const RDS_INFO_CONNECTION_FLAG_CONNECTING: u8 = 0x02;
pub const RDS_INFO_CONNECTION_FLAG_CONNECTED: u8 = 0x04;
pub const TRANSNAMSIZ: usize = 16;

#[repr(C, packed)] pub struct rds_info_counter { pub name: [u8; 32], pub value: u64 }
#[repr(C, packed)] pub struct rds_info_connection { pub next_tx_seq: u64, pub next_rx_seq: u64, pub laddr: u32, pub faddr: u32, pub transport: [u8; TRANSNAMSIZ], pub flags: u8, pub tos: u8 }
#[repr(C, packed)] pub struct rds6_info_connection { pub next_tx_seq: u64, pub next_rx_seq: u64, pub laddr: in6_addr, pub faddr: in6_addr, pub transport: [u8; TRANSNAMSIZ], pub flags: u8 }
pub const RDS_INFO_MESSAGE_FLAG_ACK: u8 = 0x01;
pub const RDS_INFO_MESSAGE_FLAG_FAST_ACK: u8 = 0x02;
#[repr(C, packed)] pub struct rds_info_message { pub seq: u64, pub len: u32, pub laddr: u32, pub faddr: u32, pub lport: u16, pub fport: u16, pub flags: u8, pub tos: u8 }
#[repr(C, packed)] pub struct rds6_info_message { pub seq: u64, pub len: u32, pub laddr: in6_addr, pub faddr: in6_addr, pub lport: u16, pub fport: u16, pub flags: u8, pub tos: u8 }
#[repr(C, packed)] pub struct rds_info_socket { pub sndbuf: u32, pub bound_addr: u32, pub connected_addr: u32, pub bound_port: u16, pub connected_port: u16, pub rcvbuf: u32, pub inum: u64 }
#[repr(C, packed)] pub struct rds6_info_socket { pub sndbuf: u32, pub bound_addr: in6_addr, pub connected_addr: in6_addr, pub bound_port: u16, pub connected_port: u16, pub rcvbuf: u32, pub inum: u64 }
#[repr(C, packed)] pub struct rds_info_tcp_socket { pub local_addr: u32, pub local_port: u16, pub peer_addr: u32, pub peer_port: u16, pub hdr_rem: u64, pub data_rem: u64, pub last_sent_nxt: u32, pub last_expected_una: u32, pub last_seen_una: u32, pub tos: u8 }
#[repr(C, packed)] pub struct rds6_info_tcp_socket { pub local_addr: in6_addr, pub local_port: u16, pub peer_addr: in6_addr, pub peer_port: u16, pub hdr_rem: u64, pub data_rem: u64, pub last_sent_nxt: u32, pub last_expected_una: u32, pub last_seen_una: u32 }

pub const RDS_IB_GID_LEN: usize = 16;
#[repr(C)] pub struct rds_info_rdma_connection { pub src_addr: u32, pub dst_addr: u32, pub src_gid: [u8; 16], pub dst_gid: [u8; 16], pub max_send_wr: u32, pub max_recv_wr: u32, pub max_send_sge: u32, pub rdma_mr_max: u32, pub rdma_mr_size: u32, pub tos: u8, pub sl: u8, pub cache_allocs: u32 }
#[repr(C)] pub struct rds6_info_rdma_connection { pub src_addr: in6_addr, pub dst_addr: in6_addr, pub src_gid: [u8; 16], pub dst_gid: [u8; 16], pub max_send_wr: u32, pub max_recv_wr: u32, pub max_send_sge: u32, pub rdma_mr_max: u32, pub rdma_mr_size: u32, pub tos: u8, pub sl: u8, pub cache_allocs: u32 }

#[repr(C)] pub enum rds_message_rxpath_latency { RDS_MSG_RX_HDR_TO_DGRAM_START = 0, RDS_MSG_RX_DGRAM_REASSEMBLE, RDS_MSG_RX_DGRAM_DELIVERED, RDS_MSG_RX_DGRAM_TRACE_MAX }
#[repr(C)] pub struct rds_rx_trace_so { pub rx_traces: u8, pub rx_trace_pos: [u8; 3] }
#[repr(C)] pub struct rds_cmsg_rx_trace { pub rx_traces: u8, pub rx_trace_pos: [u8; 3], pub rx_trace: [u64; 3] }

pub const RDS_CONG_MONITOR_SIZE: u32 = 64;
#[inline] pub const fn RDS_CONG_MONITOR_BIT(port: u32) -> u32 { port % RDS_CONG_MONITOR_SIZE }
#[inline] pub const fn RDS_CONG_MONITOR_MASK(port: u32) -> u64 { 1u64 << RDS_CONG_MONITOR_BIT(port) }
pub type rds_rdma_cookie_t = u64;
#[repr(C)] pub struct rds_iovec { pub addr: u64, pub bytes: u64 }
#[repr(C)] pub struct rds_get_mr_args { pub vec: rds_iovec, pub cookie_addr: u64, pub flags: u64 }
#[repr(C)] pub struct rds_get_mr_for_dest_args { pub dest_addr: __kernel_sockaddr_storage, pub vec: rds_iovec, pub cookie_addr: u64, pub flags: u64 }
#[repr(C)] pub struct rds_free_mr_args { pub cookie: rds_rdma_cookie_t, pub flags: u64 }
#[repr(C)] pub struct rds_rdma_args { pub cookie: rds_rdma_cookie_t, pub remote_vec: rds_iovec, pub local_vec_addr: u64, pub nr_local: u64, pub flags: u64, pub user_token: u64 }
#[repr(C)] pub struct rds_atomic_args { pub cookie: rds_rdma_cookie_t, pub local_addr: u64, pub remote_addr: u64, pub atomic: rds_atomic_args_union, pub flags: u64, pub user_token: u64 }
#[repr(C)] pub union rds_atomic_args_union { pub cswp: rds_atomic_cswap, pub fadd: rds_atomic_fadd, pub m_cswp: rds_atomic_masked_cswap, pub m_fadd: rds_atomic_masked_fadd }
#[repr(C)] pub struct rds_atomic_cswap { pub compare: u64, pub swap: u64 }
#[repr(C)] pub struct rds_atomic_fadd { pub add: u64 }
#[repr(C)] pub struct rds_atomic_masked_cswap { pub compare: u64, pub swap: u64, pub compare_mask: u64, pub swap_mask: u64 }
#[repr(C)] pub struct rds_atomic_masked_fadd { pub add: u64, pub nocarry_mask: u64 }
#[repr(C)] pub struct rds_rdma_notify { pub user_token: u64, pub status: i32 }
pub const RDS_RDMA_SUCCESS: u32 = 0; pub const RDS_RDMA_REMOTE_ERROR: u32 = 1; pub const RDS_RDMA_CANCELED: u32 = 2; pub const RDS_RDMA_DROPPED: u32 = 3; pub const RDS_RDMA_OTHER_ERROR: u32 = 4;
pub const RDS_MAX_ZCOOKIES: usize = 8;
#[repr(C)] pub struct rds_zcopy_cookies { pub num: u32, pub cookies: [u32; RDS_MAX_ZCOOKIES] }
pub const RDS_RDMA_READWRITE: u64 = 0x0001; pub const RDS_RDMA_FENCE: u64 = 0x0002; pub const RDS_RDMA_INVALIDATE: u64 = 0x0004; pub const RDS_RDMA_USE_ONCE: u64 = 0x0008; pub const RDS_RDMA_DONTWAIT: u64 = 0x0010; pub const RDS_RDMA_NOTIFY_ME: u64 = 0x0020; pub const RDS_RDMA_SILENT: u64 = 0x0040;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
