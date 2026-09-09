/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Rust translation of net/sock.h.  Names supplied by the Linux headers are
 * intentionally left as external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s32 = i32;
pub type __be16 = u16;
pub type __be32 = u32;
pub type uintptr_t = usize;

#[repr(C)]
pub struct socket_lock_t {
    pub owned: c_int,
    pub slock: spinlock_t,
    pub wq: wait_queue_head_t,
}

#[repr(C)]
pub struct sock_common {
    pub skc_addrpair: __u64,
    pub skc_daddr: __be32,
    pub skc_rcv_saddr: __be32,
    pub skc_hash: c_uint,
    pub skc_u16hashes: [__u16; 2],
    pub skc_portpair: __u32,
    pub skc_dport: __be16,
    pub skc_num: __u16,
    pub skc_family: u16,
    pub skc_state: u8,
    pub skc_reuse: u8,
    pub skc_reuseport: u8,
    pub skc_ipv6only: u8,
    pub skc_net_refcnt: u8,
    pub skc_bypass_prot_mem: u8,
    pub skc_bound_dev_if: c_int,
    pub skc_bind_node: hlist_node,
    pub skc_prot: *mut proto,
    pub skc_net: possible_net_t,
    pub skc_cookie: atomic64_t,
    pub skc_flags: c_ulong,
    pub skc_node: hlist_node,
    pub skc_tx_queue_mapping: u16,
    pub skc_incoming_cpu: c_int,
    pub skc_refcnt: refcount_t,
    pub skc_rxhash: __u32,
}

#[repr(C)]
pub struct sock {
    pub __sk_common: sock_common,
    pub sk_drops: atomic_t,
    pub sk_peek_off: __s32,
    pub sk_error_queue: sk_buff_head,
    pub sk_receive_queue: sk_buff_head,
    pub sk_backlog: sock_backlog,
    pub sk_rx_dst: *mut dst_entry,
    pub sk_rx_dst_ifindex: c_int,
    pub sk_rx_dst_cookie: __u32,
    pub sk_userlocks: u8,
    pub sk_rcvbuf: c_int,
    pub sk_filter: *mut sk_filter,
    pub sk_wq: *mut socket_wq,
    pub sk_data_ready: Option<unsafe extern "C" fn(*mut sock)>,
    pub sk_rcvtimeo: c_long,
    pub sk_rcvlowat: c_int,
    pub sk_err: c_int,
    pub sk_socket: *mut socket,
    pub sk_lock: socket_lock_t,
    pub sk_reserved_mem: __u32,
    pub sk_forward_alloc: c_int,
    pub sk_tsflags: __u32,
    pub sk_write_pending: c_int,
    pub sk_wmem_queued: c_int,
    pub sk_wmem_alloc: refcount_t,
    pub sk_tsq_flags: c_ulong,
    pub sk_write_queue: sk_buff_head,
    pub sk_pacing_rate: c_ulong,
    pub sk_zckey: atomic_t,
    pub sk_tskey: atomic_t,
    pub sk_dst_pending_confirm: __u32,
    pub sk_pacing_status: __u32,
    pub sk_max_pacing_rate: c_ulong,
    pub sk_sndtimeo: c_long,
    pub sk_priority: __u32,
    pub sk_mark: __u32,
    pub sk_protocol: u16,
    pub sk_type: u16,
    pub sk_dst_cache: *mut dst_entry,
    pub sk_route_caps: netdev_features_t,
    pub sk_gso_type: u16,
    pub sk_gso_max_segs: u16,
    pub sk_gso_max_size: c_uint,
    pub sk_allocation: gfp_t,
    pub sk_txhash: __u32,
    pub sk_sndbuf: c_int,
    pub sk_pacing_shift: u8,
    pub sk_use_task_frag: bool,
    pub sk_shutdown: u8,
    pub sk_lingertime: c_ulong,
    pub sk_prot_creator: *mut proto,
    pub sk_ack_backlog: __u32,
    pub sk_max_ack_backlog: __u32,
    pub sk_ino: __u64,
    pub sk_txrehash: u8,
    pub sk_bpf_cb_flags: u8,
    pub sk_user_data: *mut c_void,
    pub sk_state_change: Option<unsafe extern "C" fn(*mut sock)>,
    pub sk_write_space: Option<unsafe extern "C" fn(*mut sock)>,
    pub sk_error_report: Option<unsafe extern "C" fn(*mut sock)>,
    pub sk_backlog_rcv: Option<unsafe extern "C" fn(*mut sock, *mut sk_buff) -> c_int>,
    pub sk_destruct: Option<unsafe extern "C" fn(*mut sock)>,
}

#[repr(C)] pub struct sock_backlog { pub rmem_alloc: atomic_t, pub len: c_int, pub head: *mut sk_buff, pub tail: *mut sk_buff }
#[repr(C)] pub struct proto_accept_arg { pub flags: c_int, pub err: c_int, pub is_empty: c_int, pub kern: bool }

#[repr(C)]
pub struct proto { pub close: Option<unsafe extern "C" fn(*mut sock, c_long)> }

pub enum sk_pacing { SK_PACING_NONE = 0, SK_PACING_NEEDED = 1, SK_PACING_FQ = 2 }
pub enum sock_flags { SOCK_DEAD, SOCK_DONE, SOCK_URGINLINE, SOCK_KEEPOPEN, SOCK_LINGER, SOCK_DESTROY, SOCK_BROADCAST, SOCK_TIMESTAMP, SOCK_ZAPPED, SOCK_USE_WRITE_QUEUE, SOCK_DBG, SOCK_RCVTSTAMP, SOCK_RCVTSTAMPNS, SOCK_LOCALROUTE, SOCK_MEMALLOC, SOCK_TIMESTAMPING_RX_SOFTWARE, SOCK_FASYNC, SOCK_RXQ_OVFL, SOCK_ZEROCOPY, SOCK_WIFI_STATUS, SOCK_NOFCS, SOCK_FILTER_LOCKED, SOCK_SELECT_ERR_QUEUE, SOCK_RCU_FREE, SOCK_TXTIME, SOCK_XDP, SOCK_TSTAMP_NEW, SOCK_RCVMARK, SOCK_RCVPRIORITY, SOCK_TIMESTAMPING_ANY }

pub const SK_USER_DATA_NOCOPY: usize = 1;
pub const SK_USER_DATA_BPF: usize = 2;
pub const SK_USER_DATA_PSOCK: usize = 4;
pub const SK_USER_DATA_PTRMASK: usize = !(SK_USER_DATA_NOCOPY | SK_USER_DATA_BPF | SK_USER_DATA_PSOCK);
pub const SK_NO_REUSE: c_int = 0;
pub const SK_CAN_REUSE: c_int = 1;
pub const SK_FORCE_REUSE: c_int = 2;
pub const SHUTDOWN_MASK: c_int = 3;
pub const RCV_SHUTDOWN: c_int = 1;
pub const SEND_SHUTDOWN: c_int = 2;
pub const PROT_SOCK: c_int = 1024;

extern "C" {
    pub fn sk_set_peek_off(sk: *mut sock, val: c_int) -> c_int;
    pub fn sk_alloc(net: *mut net, family: c_int, priority: gfp_t, prot: *mut proto, kern: c_int) -> *mut sock;
    pub fn sk_free(sk: *mut sock);
    pub fn sk_clone(sk: *const sock, priority: gfp_t, lock: bool) -> *mut sock;
    pub fn sk_stream_write_space(sk: *mut sock);
    pub fn proto_register(prot: *mut proto, alloc_slab: c_int) -> c_int;
    pub fn proto_unregister(prot: *mut proto);
}

/* External Linux types referenced by this header. */
pub type c_long = i64;
pub enum spinlock_t {}
pub enum wait_queue_head_t {}
pub enum hlist_node {}
pub enum possible_net_t {}
pub enum atomic64_t {}
pub enum atomic_t {}
pub enum refcount_t {}
pub enum sk_buff_head {}
pub enum sk_buff {}
pub enum dst_entry {}
pub enum sk_filter {}
pub enum socket_wq {}
pub enum socket {}
pub enum net {}
pub type gfp_t = c_uint;
pub type netdev_features_t = c_ulong;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
