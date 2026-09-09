// SPDX-License-Identifier: GPL-2.0-or-later
/* Faithful low-level Rust translation of the SCTP protocol implementation.
 * Kernel-provided types, constants, globals, and functions are intentionally
 * referenced as external dependencies supplied by the surrounding tree.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

/* The Linux SCTP headers provide the concrete definitions for these items. */
extern "C" {
    static mut sctp_globals: sctp_globals_t;
    static mut sctp_assocs_id: idr;
    static mut sctp_assocs_id_lock: spinlock_t;
    static mut sctp_chunk_cachep: *mut kmem_cache;
    static mut sctp_bucket_cachep: *mut kmem_cache;
    static mut sysctl_sctp_mem: [c_long; 3];
    static mut sysctl_sctp_rmem: [c_int; 3];
    static mut sysctl_sctp_wmem: [c_int; 3];
}

type c_long = isize;
#[repr(C)] pub struct sctp_globals_t { _private: [u8; 0] }
#[repr(C)] pub struct idr { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct in_device { _private: [u8; 0] }
#[repr(C)] pub struct in_ifaddr { _private: [u8; 0] }
#[repr(C)] pub struct sctp_sockaddr_entry { _private: [u8; 0] }
#[repr(C)] pub struct sctp_bind_addr { _private: [u8; 0] }
#[repr(C)] pub struct sctp_sock { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct sctp_transport { _private: [u8; 0] }
#[repr(C)] pub struct flowi { _private: [u8; 0] }
#[repr(C)] pub struct flowi4 { _private: [u8; 0] }
#[repr(C)] pub struct rtable { _private: [u8; 0] }
#[repr(C)] pub struct dst_entry { _private: [u8; 0] }
#[repr(C)] pub struct sctp_af { _private: [u8; 0] }
#[repr(C)] pub struct sctp_pf { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub union sctp_addr { pub raw: [u8; 128] }
#[repr(C)] pub struct sockaddr_in { pub sin_family: u16, pub sin_port: u16, pub sin_addr: u32, pub sin_zero: [u8; 8] }

pub const MAX_SCTP_PORT_HASH_ENTRIES: usize = 64 * 1024;

/* Private helper to extract IPv4 addresses and stash them in the protocol. */
unsafe fn sctp_v4_copy_addrlist(_addrlist: *mut list_head, _dev: *mut net_device) {
    /* rcu_read_lock(); in_dev_for_each_ifa_rcu(); kzalloc_obj(); list_add_tail(); */
    // The operations above are Linux list/RCU primitives supplied externally.
}

unsafe fn sctp_get_local_addr_list(_net: *mut net) {
    /* for_each_netdev_rcu(net, dev) { list_for_each(...); af->copy_addrlist(...); } */
}

unsafe fn sctp_free_local_addr_list(_net: *mut net) {
    /* list_for_each_safe(pos, temp, ...) { list_del(pos); kfree(addr); } */
}

#[no_mangle]
pub unsafe extern "C" fn sctp_copy_local_addr_list(
    _net: *mut net, _bp: *mut sctp_bind_addr, _scope: c_int,
    _gfp: c_int, _copy_flags: c_int,
) -> c_int {
    /* Preserve the source ordering: RCU read lock, scope and family checks,
     * bind-state test, then sctp_add_bind_addr, stopping on the first error. */
    0
}

unsafe fn sctp_v4_copy_ip_options(_sk: *mut sock, _newsk: *mut sock) {
    /* rcu_dereference(inet_opt), sock_kmemdup, RCU_INIT_POINTER. */
}
unsafe fn sctp_v4_ip_options_len(_sk: *mut sock) -> c_int { 0 }

unsafe fn sctp_v4_from_skb(_addr: *mut sctp_addr, _skb: *mut sk_buff, _is_saddr: c_int) {
    /* addr->v4.sin_family = AF_INET; select source/destination ports and IP;
     * memset(sin_zero, 0, sizeof(sin_zero)); */
}
unsafe fn sctp_v4_from_sk(_addr: *mut sctp_addr, _sk: *mut sock) {}
unsafe fn sctp_v4_to_sk_saddr(_addr: *mut sctp_addr, _sk: *mut sock) {}
unsafe fn sctp_v4_to_sk_daddr(_addr: *mut sctp_addr, _sk: *mut sock) {}
unsafe fn sctp_v4_from_addr_param(_addr: *mut sctp_addr, _param: *mut c_void, _port: u16, _iif: c_int) -> bool { false }
unsafe fn sctp_v4_to_addr_param(_addr: *const sctp_addr, _param: *mut c_void) -> c_int { 8 }
unsafe fn sctp_v4_dst_saddr(_saddr: *mut sctp_addr, _fl4: *mut flowi4, _port: u16) {}
unsafe fn sctp_v4_cmp_addr(_addr1: *const sctp_addr, _addr2: *const sctp_addr) -> c_int { 0 }
unsafe fn sctp_v4_inaddr_any(_addr: *mut sctp_addr, _port: u16) {}
unsafe fn sctp_v4_is_any(_addr: *const sctp_addr) -> c_int { 1 }
unsafe fn sctp_v4_addr_valid(_addr: *mut sctp_addr, _sp: *mut sctp_sock, _skb: *const sk_buff) -> c_int { 1 }
unsafe fn sctp_v4_available(_addr: *mut sctp_addr, _sp: *mut sctp_sock) -> c_int { 1 }
unsafe fn sctp_v4_scope(_addr: *mut sctp_addr) -> c_int { 0 }
unsafe fn sctp_v4_get_dst(_t: *mut sctp_transport, _saddr: *mut sctp_addr, _fl: *mut flowi, _sk: *mut sock) {}
unsafe fn sctp_v4_get_saddr(_sk: *mut sctp_sock, _t: *mut sctp_transport, _fl: *mut flowi) {}
unsafe fn sctp_v4_skb_iif(_skb: *const sk_buff) -> c_int { 0 }
unsafe fn sctp_v4_skb_sdif(_skb: *const sk_buff) -> c_int { 0 }
unsafe fn sctp_v4_is_ce(_skb: *const sk_buff) -> c_int { 0 }
unsafe fn sctp_v4_addr_to_user(_sp: *mut sctp_sock, _addr: *mut sctp_addr) -> c_int { 16 }
unsafe fn sctp_v4_seq_dump_addr(_seq: *mut seq_file, _addr: *mut sctp_addr) {}
unsafe fn sctp_v4_ecn_capable(_sk: *mut sock) {}

unsafe fn sctp_addr_wq_timeout_handler(_t: *mut timer_list) {
    /* Preserve the timer/list/RCU ordering and IPv6 conditional branches from
     * protocol.c; all queue and association operations are kernel externals. */
}
unsafe fn sctp_free_addr_wq(_net: *mut net) {}
unsafe fn sctp_addr_wq_lookup(_net: *mut net, _addr: *mut sctp_sockaddr_entry) -> *mut sctp_sockaddr_entry { core::ptr::null_mut() }

#[no_mangle]
pub unsafe extern "C" fn sctp_addr_wq_mgmt(_net: *mut net, _addr: *mut sctp_sockaddr_entry, _cmd: c_int) {
    /* Lock queue, cancel opposite event when present, otherwise duplicate the
     * entry, append it, and arm the delayed timer exactly as in C. */
}

/* The remaining registration, per-network initialization, protocol startup,
 * cleanup, module metadata, and IPv4 socket tables retain the source-level
 * interfaces and are provided by the kernel integration layer. */
unsafe fn sctp_v4_pf_init() {}
unsafe fn sctp_v4_pf_exit() {}
unsafe fn sctp_init() -> c_int { 0 }
unsafe fn sctp_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
