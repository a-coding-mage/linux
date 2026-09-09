/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of net/sctp/sctp.h.  Included dependencies are supplied elsewhere. */

/* CONFIG_IP_SCTP_MODULE: SCTP_PROTOSW_FLAG is 0 for modules, otherwise
 * INET_PROTOSW_PERMANENT. */

extern "C" {
    pub fn sctp_copy_local_addr_list(net: *mut net, addr: *mut sctp_bind_addr, scope: sctp_scope, gfp: gfp_t, flags: c_int) -> c_int;
    pub fn sctp_get_pf_specific(family: sa_family_t) -> *mut sctp_pf;
    pub fn sctp_register_pf(pf: *mut sctp_pf, family: sa_family_t) -> c_int;
    pub fn sctp_addr_wq_mgmt(net: *mut net, entry: *mut sctp_sockaddr_entry, cmd: c_int);
    pub fn sctp_udp_sock_start(net: *mut net) -> c_int;
    pub fn sctp_udp_sock_stop(net: *mut net);
    pub fn sctp_inet_connect(sock: *mut socket, uaddr: *mut sockaddr_unsized, addr_len: c_int, flags: c_int) -> c_int;
    pub fn sctp_backlog_rcv(sk: *mut sock, skb: *mut sk_buff) -> c_int;
    pub fn sctp_inet_listen(sock: *mut socket, backlog: c_int) -> c_int;
    pub fn sctp_write_space(sk: *mut sock);
    pub fn sctp_data_ready(sk: *mut sock);
    pub fn sctp_poll(file: *mut file, sock: *mut socket, wait: *mut poll_table) -> __poll_t;
    pub fn sctp_sock_rfree(skb: *mut sk_buff);
    pub static mut sctp_sockets_allocated: percpu_counter;
    pub fn sctp_asconf_mgmt(sk: *mut sctp_sock, entry: *mut sctp_sockaddr_entry) -> c_int;
    pub fn sctp_skb_recv_datagram(sk: *mut sock, noblock: c_int, err: *mut c_int) -> *mut sk_buff;
    pub fn sctp_transport_walk_start(iter: *mut rhashtable_iter);
    pub fn sctp_transport_walk_stop(iter: *mut rhashtable_iter);
    pub fn sctp_transport_get_next(net: *mut net, iter: *mut rhashtable_iter) -> *mut sctp_transport;
    pub fn sctp_transport_get_idx(net: *mut net, iter: *mut rhashtable_iter, pos: c_int) -> *mut sctp_transport;
    pub fn sctp_transport_lookup_process(cb: sctp_callback_t, net: *mut net, laddr: *const sctp_addr, paddr: *const sctp_addr, p: *mut c_void, dif: c_int) -> c_int;
    pub fn sctp_transport_traverse_process(cb: sctp_callback_t, cb_done: sctp_callback_t, net: *mut net, pos: *mut c_int, p: *mut c_void) -> c_int;
    pub fn sctp_for_each_endpoint(cb: Option<unsafe extern "C" fn(*mut sctp_endpoint, *mut c_void) -> c_int>, net: *mut net, pos: *mut c_int, p: *mut c_void) -> c_int;
    pub fn sctp_get_sctp_info(sk: *mut sock, asoc: *mut sctp_association, info: *mut sctp_info) -> c_int;
    pub fn sctp_primitive_ASSOCIATE(net: *mut net, asoc: *mut sctp_association, arg: *mut c_void) -> c_int;
    pub fn sctp_primitive_SHUTDOWN(net: *mut net, asoc: *mut sctp_association, arg: *mut c_void) -> c_int;
    pub fn sctp_primitive_ABORT(net: *mut net, asoc: *mut sctp_association, arg: *mut c_void) -> c_int;
    pub fn sctp_primitive_SEND(net: *mut net, asoc: *mut sctp_association, arg: *mut c_void) -> c_int;
    pub fn sctp_primitive_REQUESTHEARTBEAT(net: *mut net, asoc: *mut sctp_association, arg: *mut c_void) -> c_int;
    pub fn sctp_primitive_ASCONF(net: *mut net, asoc: *mut sctp_association, arg: *mut c_void) -> c_int;
    pub fn sctp_primitive_RECONF(net: *mut net, asoc: *mut sctp_association, arg: *mut c_void) -> c_int;
    pub fn sctp_rcv(skb: *mut sk_buff) -> c_int;
    pub fn sctp_v4_err(skb: *mut sk_buff, info: u32) -> c_int;
    pub fn sctp_hash_endpoint(ep: *mut sctp_endpoint) -> c_int;
    pub fn sctp_unhash_endpoint(ep: *mut sctp_endpoint);
    pub fn sctp_err_lookup(net: *mut net, family: c_int, skb: *mut sk_buff, hdr: *mut sctphdr, asoc: *mut *mut sctp_association, t: *mut *mut sctp_transport) -> *mut sock;
    pub fn sctp_err_finish(sk: *mut sock, t: *mut sctp_transport);
    pub fn sctp_udp_v4_err(sk: *mut sock, skb: *mut sk_buff) -> c_int;
    pub fn sctp_udp_v6_err(sk: *mut sock, skb: *mut sk_buff) -> c_int;
    pub fn sctp_icmp_frag_needed(sk: *mut sock, asoc: *mut sctp_association, t: *mut sctp_transport, pmtu: u32);
    pub fn sctp_icmp_redirect(sk: *mut sock, t: *mut sctp_transport, skb: *mut sk_buff);
    pub fn sctp_icmp_proto_unreachable(sk: *mut sock, asoc: *mut sctp_association, t: *mut sctp_transport);
    pub fn sctp_transport_hashtable_init() -> c_int;
    pub fn sctp_transport_hashtable_destroy();
    pub fn sctp_hash_transport(t: *mut sctp_transport) -> c_int;
    pub fn sctp_unhash_transport(t: *mut sctp_transport);
    pub fn sctp_addrs_lookup_transport(net: *mut net, laddr: *const sctp_addr, paddr: *const sctp_addr, dif: c_int, sdif: c_int) -> *mut sctp_transport;
    pub fn sctp_epaddr_lookup_transport(ep: *const sctp_endpoint, paddr: *const sctp_addr) -> *mut sctp_transport;
    pub fn sctp_sk_bound_dev_eq(net: *mut net, bound_dev_if: c_int, dif: c_int, sdif: c_int) -> bool;
    pub fn sctp_proc_init(net: *mut net) -> c_int;
    pub fn sctp_offload_init() -> c_int;
    pub fn sctp_sched_ops_init();
    pub fn sctp_send_reset_streams(asoc: *mut sctp_association, params: *mut sctp_reset_streams) -> c_int;
    pub fn sctp_send_reset_assoc(asoc: *mut sctp_association) -> c_int;
    pub fn sctp_send_add_streams(asoc: *mut sctp_association, params: *mut sctp_add_streams) -> c_int;
    pub static mut sctp_chunk_cachep: *mut kmem_cache;
    pub static mut sctp_bucket_cachep: *mut kmem_cache;
    pub static mut sysctl_sctp_mem: [c_long; 3];
    pub static mut sysctl_sctp_rmem: [c_int; 3];
    pub static mut sysctl_sctp_wmem: [c_int; 3];
}

pub type sctp_callback_t = Option<unsafe extern "C" fn(*mut sctp_endpoint, *mut sctp_transport, *mut c_void) -> c_int>;

pub const SCTP_MIB_NUM: usize = 0;
pub const SCTP_MIB_CURRESTAB: usize = 1;
pub const SCTP_MIB_ACTIVEESTABS: usize = 2;
pub const SCTP_MIB_PASSIVEESTABS: usize = 3;
pub const SCTP_MIB_ABORTEDS: usize = 4;
pub const SCTP_MIB_SHUTDOWNS: usize = 5;
pub const SCTP_MIB_OUTOFBLUES: usize = 6;
pub const SCTP_MIB_CHECKSUMERRORS: usize = 7;
pub const SCTP_MIB_OUTCTRLCHUNKS: usize = 8;
pub const SCTP_MIB_OUTORDERCHUNKS: usize = 9;
pub const SCTP_MIB_OUTUNORDERCHUNKS: usize = 10;
pub const SCTP_MIB_INCTRLCHUNKS: usize = 11;
pub const SCTP_MIB_INORDERCHUNKS: usize = 12;
pub const SCTP_MIB_INUNORDERCHUNKS: usize = 13;
pub const SCTP_MIB_FRAGUSRMSGS: usize = 14;
pub const SCTP_MIB_REASMUSRMSGS: usize = 15;
pub const SCTP_MIB_OUTSCTPPACKS: usize = 16;
pub const SCTP_MIB_INSCTPPACKS: usize = 17;
pub const SCTP_MIB_T1_INIT_EXPIREDS: usize = 18;
pub const SCTP_MIB_T1_COOKIE_EXPIREDS: usize = 19;
pub const SCTP_MIB_T2_SHUTDOWN_EXPIREDS: usize = 20;
pub const SCTP_MIB_T3_RTX_EXPIREDS: usize = 21;
pub const SCTP_MIB_T4_RTO_EXPIREDS: usize = 22;
pub const SCTP_MIB_T5_SHUTDOWN_GUARD_EXPIREDS: usize = 23;
pub const SCTP_MIB_DELAY_SACK_EXPIREDS: usize = 24;
pub const SCTP_MIB_AUTOCLOSE_EXPIREDS: usize = 25;
pub const SCTP_MIB_T1_RETRANSMITS: usize = 26;
pub const SCTP_MIB_T3_RETRANSMITS: usize = 27;
pub const SCTP_MIB_PMTUD_RETRANSMITS: usize = 28;
pub const SCTP_MIB_FAST_RETRANSMITS: usize = 29;
pub const SCTP_MIB_IN_PKT_SOFTIRQ: usize = 30;
pub const SCTP_MIB_IN_PKT_BACKLOG: usize = 31;
pub const SCTP_MIB_IN_PKT_DISCARDS: usize = 32;
pub const SCTP_MIB_IN_DATA_CHUNK_DISCARDS: usize = 33;
pub const SCTP_MIB_MAX: usize = 34;

#[repr(C)]
pub struct sctp_mib { pub mibs: [c_ulong; SCTP_MIB_MAX] }

pub unsafe fn sctp_max_rto(asoc: *mut sctp_association, trans: *mut sctp_transport) {
    if (*asoc).stats.max_obs_rto < (*trans).rto as u64 {
        (*asoc).stats.max_obs_rto = (*trans).rto;
        memset(&mut (*asoc).stats.obs_rto_ipaddr as *mut _ as *mut c_void, 0, core::mem::size_of::<sockaddr_storage>());
        memcpy(&mut (*asoc).stats.obs_rto_ipaddr as *mut _ as *mut c_void, &(*trans).ipaddr as *const _ as *const c_void, (*trans).af_specific.sockaddr_len as usize);
    }
}

#[cfg(CONFIG_SCTP_DBG_OBJCNT)]
extern "C" {
    pub static mut sctp_dbg_objcnt_sock: atomic_t; pub static mut sctp_dbg_objcnt_ep: atomic_t;
    pub static mut sctp_dbg_objcnt_assoc: atomic_t; pub static mut sctp_dbg_objcnt_transport: atomic_t;
    pub static mut sctp_dbg_objcnt_chunk: atomic_t; pub static mut sctp_dbg_objcnt_bind_addr: atomic_t;
    pub static mut sctp_dbg_objcnt_bind_bucket: atomic_t; pub static mut sctp_dbg_objcnt_addr: atomic_t;
    pub static mut sctp_dbg_objcnt_datamsg: atomic_t; pub static mut sctp_dbg_objcnt_keys: atomic_t;
    pub fn sctp_dbg_objcnt_init(net: *mut net);
}

#[cfg(CONFIG_SYSCTL)]
extern "C" { pub fn sctp_sysctl_register(); pub fn sctp_sysctl_unregister(); pub fn sctp_sysctl_net_register(net: *mut net) -> c_int; pub fn sctp_sysctl_net_unregister(net: *mut net); }

/* Direct translations of the remaining C macros and inline helpers. */
pub const fn sctp_sat_len(x: usize) -> usize { core::mem::size_of::<sctp_paramhdr>() + x * core::mem::size_of::<u16>() }
pub unsafe fn sctp_assoc2id(asoc: *const sctp_association) -> sctp_assoc_t { if asoc.is_null() { 0 } else { (*asoc).assoc_id } }
extern "C" { pub fn sctp_id2assoc(sk: *mut sock, id: sctp_assoc_t) -> *mut sctp_association; }
pub unsafe fn ipver2af(ipver: u8) -> c_int { match ipver { 4 => AF_INET, 6 => AF_INET6, _ => 0 } }
pub unsafe fn param_type2af(ty: __be16) -> c_int { match ty { SCTP_PARAM_IPV4_ADDRESS => AF_INET, SCTP_PARAM_IPV6_ADDRESS => AF_INET6, _ => 0 } }
pub unsafe fn __sctp_style(sk: *const sock, style: sctp_socket_type) -> c_int { (sctp_sk(sk).type_ == style) as c_int }
pub unsafe fn __sctp_state(asoc: *const sctp_association, state: sctp_state) -> c_int { ((*asoc).state == state) as c_int }
pub unsafe fn __sctp_sstate(sk: *const sock, state: sctp_sock_state) -> c_int { ((*sk).sk_state == state) as c_int }
pub unsafe fn sctp_v6_map_v4(addr: *mut sctp_addr) { (*addr).v4.sin_family = AF_INET; (*addr).v4.sin_port = (*addr).v6.sin6_port; (*addr).v4.sin_addr.s_addr = (*addr).v6.sin6_addr.s6_addr32[3]; }
pub unsafe fn sctp_v4_map_v6(addr: *mut sctp_addr) { let port = (*addr).v4.sin_port; (*addr).v6.sin6_addr.s6_addr32[3] = (*addr).v4.sin_addr.s_addr; (*addr).v6.sin6_port=port; (*addr).v6.sin6_family=AF_INET6; (*addr).v6.sin6_flowinfo=0; (*addr).v6.sin6_scope_id=0; (*addr).v6.sin6_addr.s6_addr32[0]=0; (*addr).v6.sin6_addr.s6_addr32[1]=0; (*addr).v6.sin6_addr.s6_addr32[2] = htonl(0x0000ffff); }
/* sctp_skb_for_each, sctp_walk_params, sctp_walk_errors, sctp_walk_fwdtsn,
 * sctp_for_each_hentry, sctp_style, sctp_state, sctp_sstate and statistics
 * macros retain their original kernel-macro semantics and are supplied by
 * the corresponding translated dependency headers. */
extern "C" { pub static mut sctp_prot: proto; pub static mut sctpv6_prot: proto; pub fn sctp_put_port(sk: *mut sock); pub static mut sctp_assocs_id: idr; pub static mut sctp_assocs_id_lock: spinlock_t; }

/* CONFIG_IPV6 disabled stubs retain the header's conditional interface. */
#[cfg(not(CONFIG_IPV6))] pub unsafe fn sctp_v6_pf_init() {}
#[cfg(not(CONFIG_IPV6))] pub unsafe fn sctp_v6_pf_exit() {}
#[cfg(not(CONFIG_IPV6))] pub unsafe fn sctp_v6_protosw_init() -> c_int { 0 }
#[cfg(not(CONFIG_IPV6))] pub unsafe fn sctp_v6_protosw_exit() {}
#[cfg(not(CONFIG_IPV6))] pub unsafe fn sctp_v6_add_protocol() -> c_int { 0 }
#[cfg(not(CONFIG_IPV6))] pub unsafe fn sctp_v6_del_protocol() {}

pub unsafe fn sctp_assoc_to_state(asoc: *const sctp_association) -> sctp_sstat_state { (*asoc).state + 1 }
pub unsafe fn sctp_list_dequeue(list: *mut list_head) -> *mut list_head {
    let result = (*list).next;
    if !list_empty(list) { list_del_init(result); result } else { core::ptr::null_mut() }
}
pub unsafe fn sctp_skb_set_owner_r(skb: *mut sk_buff, sk: *mut sock) {
    let event = sctp_skb2event(skb); skb_orphan(skb); (*skb).sk = sk;
    (*skb).destructor = Some(sctp_sock_rfree); atomic_add((*event).rmem_len, &mut (*sk).sk_rmem_alloc); sk_mem_charge(sk, (*event).rmem_len);
}
pub unsafe fn sctp_list_single_entry(head: *mut list_head) -> c_int { list_is_singular(head) as c_int }
pub unsafe fn sctp_chunk_pending(chunk: *const sctp_chunk) -> bool { !list_empty(&(*chunk).list as *const _ as *mut _) }
pub unsafe fn sctp_transport_dst_check(t: *mut sctp_transport) -> *mut dst_entry { if !(*t).dst.is_null() && !dst_check((*t).dst, (*t).dst_cookie) { sctp_transport_dst_release(t); } (*t).dst }
pub unsafe fn sctp_newsk_ready(sk: *const sock) -> bool { sock_flag(sk, SOCK_DEAD) || !(*sk).sk_socket.is_null() }
pub unsafe fn sctp_sock_set_nodelay(sk: *mut sock) { lock_sock(sk); (*sctp_sk(sk)).nodelay = true; release_sock(sk); }

/* The following declarations preserve the inline helpers' external kernel
 * operations without introducing implementations for dependency symbols. */
extern "C" {
    fn net_hash_mix(net: *mut net) -> u32;
    static sctp_port_hashsize: u32; static sctp_ep_hashsize: u32;
    fn sctp_sk(sk: *const sock) -> *mut sctp_sock;
    fn sctp_transport_dst_release(t: *mut sctp_transport);
    fn dst_check(dst: *mut dst_entry, cookie: u32) -> bool;
    fn htonl(x: u32) -> u32;
    fn memset(dst: *mut c_void, value: c_int, n: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn list_empty(list: *mut list_head) -> bool; fn list_del_init(list: *mut list_head);
    fn list_is_singular(list: *mut list_head) -> bool;
    fn sctp_skb2event(skb: *mut sk_buff) -> *mut sctp_ulpevent; fn skb_orphan(skb: *mut sk_buff);
    fn atomic_add(value: c_int, ptr: *mut atomic_t); fn sk_mem_charge(sk: *mut sock, value: c_int);
    fn lock_sock(sk: *mut sock); fn release_sock(sk: *mut sock); fn sock_flag(sk: *const sock, flag: c_int) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
