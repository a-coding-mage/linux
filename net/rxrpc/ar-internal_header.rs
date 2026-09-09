/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of ar-internal.h.  Kernel-provided types and functions are
 * intentionally referenced as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

pub const FCRYPT_ROUNDS: usize = 16;
pub const FCRYPT_BSIZE: usize = 8;
pub const RXRPC_BACKLOG_MAX: usize = 32;
pub const RXRPC_TX_MAX_WINDOW: usize = 128;
pub const RXRPC_SACK_SIZE: usize = 256;
pub const RXRPC_CALL_RTT_AVAIL_MASK: usize = 0xf;
pub const RXRPC_CALL_RTT_PEND_SHIFT: usize = 8;
pub const RXRPC_NR_TXQUEUE: usize = usize::BITS as usize;
pub const RXRPC_TXQ_MASK: usize = RXRPC_NR_TXQUEUE - 1;

pub type __be16 = u16; pub type __be32 = u32; pub type __le64 = u64;
pub type rxrpc_seq_t = u32; pub type rxrpc_serial_t = u32;
pub type ktime_t = i64; pub type time64_t = i64; pub type gfp_t = u32;
pub type sockptr_t = *mut c_void;

#[repr(C)] pub struct fcrypt_key { pub sched: [__be32; FCRYPT_ROUNDS] }
#[repr(C, align(8))] pub union rxrpc_crypt { pub x: [u8; FCRYPT_BSIZE], pub n: [__be32; 2] }

macro_rules! opaque { ($($n:ident),* $(,)?) => { $(#[repr(C)] pub struct $n { _private: [u8; 0] })* }; }
opaque!(proc_dir_entry, list_head, spinlock_t, atomic_t, rwlock_t, work_struct,
 timer_list, hlist_head, mutex, socket, task_struct, completion, page_frag_cache,
 sk_buff_head, rb_root, key, sock, sockaddr_rxrpc, rcu_head, refcount_t, hlist_node,
 net, sk_buff, kvec, bio_vec, seqlock_t, rb_node, workqueue_struct, key_type,
 seq_file, msghdr, file, seq_operations, pernet_operations, idr, wait_queue_head_t,
 minmax, rxrpc_key_token, des_ctx, rxrpc_peer, rxrpc_connection, rxrpc_call,
 rxrpc_txbuf, rxrpc_txqueue, rxgk_context, rxrpc_sock, rxrpc_local, rxrpc_net,
 rxrpc_bundle, rxrpc_security);

#[repr(C)] pub struct rxrpc_host_header { pub epoch:u32,pub cid:u32,pub callNumber:u32,pub seq:u32,pub serial:u32,pub type_:u8,pub flags:u8,pub userStatus:u8,pub securityIndex:u8,pub _rsvd:u16,pub serviceId:u16 }
#[repr(C)] pub struct rxrpc_skb_priv { pub hdr: rxrpc_host_header, pub _private: [u8; 48] }
#[repr(C)] pub struct rxrpc_backlog { pub peer_backlog_head:u16,pub peer_backlog_tail:u16,pub conn_backlog_head:u16,pub conn_backlog_tail:u16,pub call_backlog_head:u16,pub call_backlog_tail:u16,pub peer_backlog:[*mut rxrpc_peer;32],pub conn_backlog:[*mut rxrpc_connection;32],pub call_backlog:[*mut rxrpc_call;32] }

#[repr(C)] pub struct rxrpc_net { pub proc_net:*mut proc_dir_entry,pub epoch:u32,pub calls:list_head,pub call_lock:spinlock_t,pub nr_calls:atomic_t,pub nr_conns:atomic_t,pub bundle_proc_list:list_head,pub conn_proc_list:list_head,pub service_conns:list_head,pub conn_lock:rwlock_t,pub service_conn_reaper:work_struct,pub service_conn_reap_timer:timer_list,pub live:bool,pub nr_client_conns:atomic_t,pub local_endpoints:hlist_head,pub local_mutex:mutex,pub peer_keepalive_cursor:u8,pub peer_keepalive_base:time64_t,pub peer_keepalive:[list_head;32],pub peer_keepalive_new:list_head,pub peer_keepalive_timer:timer_list,pub peer_keepalive_work:work_struct,pub stats:[atomic_t;32] }

#[repr(C)] pub struct rxrpc_sock { pub sk:sock,pub app_ops:*const c_void,pub local:*mut rxrpc_local,pub backlog:*mut rxrpc_backlog,pub recvmsg_oobq:sk_buff_head,pub pending_oobq:rb_root,pub oob_id_counter:u64,pub incoming_lock:spinlock_t,pub sock_calls:list_head,pub to_be_accepted:list_head,pub recvmsg_q:list_head,pub recvmsg_lock:spinlock_t,pub key:*mut key,pub securities:*mut key,pub calls:rb_root,pub flags:usize,pub call_lock:rwlock_t,pub min_sec_level:u32,pub exclusive:bool,pub second_service:u16,pub service_upgrade:[u16;2],pub family:u16,pub srx:sockaddr_rxrpc,pub connect_srx:sockaddr_rxrpc }

#[repr(C)] pub struct rxrpc_conn_proto { pub index_key:u64 }
#[repr(C)] pub struct rxrpc_conn_parameters { pub local:*mut rxrpc_local,pub peer:*mut rxrpc_peer,pub key:*mut key,pub exclusive:bool,pub upgrade:bool,pub service_id:u16,pub security_level:u32 }
#[repr(C)] pub struct rxrpc_ack_summary { pub ack_serial:rxrpc_serial_t,pub acked_serial:rxrpc_serial_t,pub in_flight:u16,pub nr_new_hacks:u16,pub nr_new_sacks:u16,pub nr_new_snacks:u16,pub ack_reason:u8,pub flags:u8,pub change:u8 }
#[repr(C)] pub struct rxrpc_txbuf { pub ref_:refcount_t,pub seq:rxrpc_seq_t,pub serial:rxrpc_serial_t,pub call_debug_id:u32,pub debug_id:u32,pub len:u16,pub space:u16,pub offset:u16,pub crypto_header:u16,pub sec_header:u16,pub pkt_len:u16,pub alloc_size:u16,pub flags:u32,pub cksum:__be16,pub jumboable:bool,pub data:*mut c_void }
#[repr(C)] pub struct rxrpc_send_data_req { pub now:ktime_t,pub tq:*mut rxrpc_txqueue,pub seq:rxrpc_seq_t,pub n:c_int,pub retrans:bool,pub did_send:bool,pub tlp_probe:bool,pub trace:c_int }
#[repr(C)] pub struct rxrpc_txqueue { pub next:*mut rxrpc_txqueue,pub xmit_ts_base:ktime_t,pub qbase:rxrpc_seq_t,pub nr_reported_acks:u8,pub segment_acked:usize,pub segment_lost:usize,pub segment_retransmitted:usize,pub rtt_samples:usize,pub ever_retransmitted:usize,pub bufs:[*mut rxrpc_txbuf;RXRPC_NR_TXQUEUE],pub segment_serial:[u32;RXRPC_NR_TXQUEUE],pub segment_xmit_ts:[u32;RXRPC_NR_TXQUEUE] }

pub const RXRPC_CLIENT_INITIATED:u8 = 1;
#[inline] pub unsafe fn rxrpc_to_server(sp:*const rxrpc_skb_priv)->bool { ((*sp).hdr.flags & RXRPC_CLIENT_INITIATED)!=0 }
#[inline] pub unsafe fn rxrpc_to_client(sp:*const rxrpc_skb_priv)->bool { !rxrpc_to_server(sp) }
#[inline] pub unsafe fn rxrpc_sending_to_server(txb:*const rxrpc_txbuf)->bool { ((*txb).flags & RXRPC_CLIENT_INITIATED as u32)!=0 }
#[inline] pub unsafe fn rxrpc_sending_to_client(txb:*const rxrpc_txbuf)->bool { !rxrpc_sending_to_server(txb) }
#[inline] pub fn before(seq1:u32,seq2:u32)->bool { (seq1.wrapping_sub(seq2) as i32)<0 }
#[inline] pub fn before_eq(seq1:u32,seq2:u32)->bool { (seq1.wrapping_sub(seq2) as i32)<=0 }
#[inline] pub fn after(seq1:u32,seq2:u32)->bool { (seq1.wrapping_sub(seq2) as i32)>0 }
#[inline] pub fn after_eq(seq1:u32,seq2:u32)->bool { (seq1.wrapping_sub(seq2) as i32)>=0 }
#[inline] pub fn earliest(a:u32,b:u32)->u32 { if before(a,b){a}else{b} }
#[inline] pub fn latest(a:u32,b:u32)->u32 { if after(a,b){a}else{b} }

extern "C" {
 pub fn fcrypt_preparekey(key:*mut fcrypt_key, raw_key:*const u8);
 pub fn fcrypt_pcbc_encrypt(key:*const fcrypt_key, iv:*const u8, src:*const c_void,dst:*mut c_void,nblocks:usize);
 pub fn fcrypt_pcbc_decrypt(key:*const fcrypt_key, iv:*const u8, src:*const c_void,dst:*mut c_void,nblocks:usize);
 pub static mut rxrpc_n_rx_skbs: atomic_t; pub static mut rxrpc_workqueue:*mut workqueue_struct;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
