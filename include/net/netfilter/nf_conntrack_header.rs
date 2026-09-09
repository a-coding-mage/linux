/* SPDX-License-Identifier: GPL-2.0 */
/* Connection state tracking for netfilter. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

/* C header dependencies are supplied by the surrounding translation unit. */

#[repr(C)]
pub struct nf_ct_udp {
    pub stream_ts: ::core::ffi::c_ulong,
}

#[repr(C)]
pub union nf_conntrack_proto {
    pub sctp: ip_ct_sctp,
    pub tcp: ip_ct_tcp,
    pub udp: nf_ct_udp,
    pub gre: nf_ct_gre,
    pub tmpl_padto: u32,
}

#[repr(C)]
pub union nf_conntrack_expect_proto {
    pub _unused: (),
}

#[repr(C)]
pub struct nf_conntrack_net_ecache {
    pub dwork: delayed_work,
    pub dying_lock: spinlock_t,
    pub dying_list: hlist_nulls_head,
}

#[repr(C)]
pub struct nf_conntrack_net {
    pub count: atomic_t,
    pub expect_count: u32,
    pub users4: u32,
    pub users6: u32,
    pub users_bridge: u32,
    #[cfg(CONFIG_SYSCTL)]
    pub sysctl_header: *mut ctl_table_header,
    #[cfg(CONFIG_NF_CONNTRACK_EVENTS)]
    pub ecache: nf_conntrack_net_ecache,
}

#[repr(C)]
pub struct nf_conn {
    pub ct_general: nf_conntrack,
    pub lock: spinlock_t,
    pub timeout: u32,
    #[cfg(CONFIG_NF_CONNTRACK_ZONES)]
    pub zone: nf_conntrack_zone,
    pub tuplehash: [nf_conntrack_tuple_hash; IP_CT_DIR_MAX as usize],
    pub status: ::core::ffi::c_ulong,
    pub ct_net: possible_net_t,
    #[cfg(CONFIG_NF_NAT)]
    pub nat_bysource: hlist_node,
    pub __nfct_init_offset: (),
    pub master: *mut nf_conn,
    #[cfg(CONFIG_NF_CONNTRACK_MARK)]
    pub mark: u32,
    #[cfg(CONFIG_NF_CONNTRACK_SECMARK)]
    pub secmark: u32,
    pub ext: *mut nf_ct_ext,
    pub proto: nf_conntrack_proto,
}

#[inline]
pub unsafe fn nf_ct_to_nf_conn(nfct: *const nf_conntrack) -> *mut nf_conn {
    container_of!(nfct, nf_conn, ct_general)
}

#[inline]
pub unsafe fn nf_ct_tuplehash_to_ctrack(hash: *const nf_conntrack_tuple_hash) -> *mut nf_conn {
    container_of!(hash, nf_conn, tuplehash[(*hash).tuple.dst.dir as usize])
}

#[inline]
pub unsafe fn nf_ct_l3num(ct: *const nf_conn) -> u16 {
    (*ct).tuplehash[IP_CT_DIR_ORIGINAL as usize].tuple.src.l3num
}

#[inline]
pub unsafe fn nf_ct_protonum(ct: *const nf_conn) -> u8 {
    (*ct).tuplehash[IP_CT_DIR_ORIGINAL as usize].tuple.dst.protonum
}

#[inline]
pub unsafe fn nf_ct_tuple(ct: *mut nf_conn, dir: usize) -> *mut nf_conntrack_tuple {
    &mut (*ct).tuplehash[dir].tuple
}

#[inline]
pub unsafe fn master_ct(conntr: *mut nf_conn) -> *mut nf_conn { (*conntr).master }

extern "C" {
    pub static mut init_net: net;
    pub fn nf_conntrack_tuple_taken(tuple: *const nf_conntrack_tuple, ignored_conntrack: *const nf_conn) -> ::core::ffi::c_int;
    pub fn nf_ct_destroy(nfct: *mut nf_conntrack);
    pub fn nf_conntrack_tcp_set_closing(ct: *mut nf_conn);
    pub fn nf_ct_netns_get(net: *mut net, nfproto: u8) -> ::core::ffi::c_int;
    pub fn nf_ct_netns_put(net: *mut net, nfproto: u8);
    pub fn nf_ct_alloc_hashtable(sizep: *mut u32, nulls: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    pub fn nf_conntrack_hash_check_insert(ct: *mut nf_conn) -> ::core::ffi::c_int;
    pub fn nf_ct_delete(ct: *mut nf_conn, pid: u32, report: ::core::ffi::c_int) -> bool;
    pub fn nf_ct_get_tuplepr(skb: *const sk_buff, nhoff: u32, l3num: u16, net: *mut net, tuple: *mut nf_conntrack_tuple) -> bool;
    pub fn __nf_ct_refresh_acct(ct: *mut nf_conn, ctinfo: ip_conntrack_info, extra_jiffies: u32, bytes: u32);
    pub fn nf_ct_kill_acct(ct: *mut nf_conn, ctinfo: ip_conntrack_info, skb: *const sk_buff) -> bool;
}

#[inline]
pub unsafe fn nf_ct_refresh_acct(ct: *mut nf_conn, ctinfo: ip_conntrack_info, skb: *const sk_buff, extra_jiffies: u32) { __nf_ct_refresh_acct(ct, ctinfo, extra_jiffies, (*skb).len); }
#[inline]
pub unsafe fn nf_ct_refresh(ct: *mut nf_conn, extra_jiffies: u32) { __nf_ct_refresh_acct(ct, 0 as ip_conntrack_info, extra_jiffies, 0); }
#[inline]
pub unsafe fn nf_ct_kill(ct: *mut nf_conn) -> bool { nf_ct_delete(ct, 0, 0) }

#[inline]
pub unsafe fn nf_ct_net(ct: *const nf_conn) -> *mut net { read_pnet(&(*ct).ct_net) }

#[inline]
pub unsafe fn nf_ct_get(skb: *const sk_buff, ctinfo: *mut ip_conntrack_info) -> *mut nf_conn {
    let nfct = skb_get_nfct(skb);
    *ctinfo = (nfct & NFCT_INFOMASK) as ip_conntrack_info;
    (nfct & NFCT_PTRMASK) as *mut nf_conn
}

#[inline]
pub unsafe fn nf_ct_put(ct: *mut nf_conn) {
    if !ct.is_null() && refcount_dec_and_test(&mut (*ct).ct_general.use_) { nf_ct_destroy(&mut (*ct).ct_general); }
}

#[repr(C)]
pub struct nf_ct_iter_data { pub net: *mut net, pub data: *mut ::core::ffi::c_void, pub portid: u32, pub report: ::core::ffi::c_int }

extern "C" {
    pub fn nf_ct_iterate_cleanup_net(iter: Option<unsafe extern "C" fn(*mut nf_conn, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>, iter_data: *const nf_ct_iter_data);
    pub fn nf_ct_iterate_destroy(iter: Option<unsafe extern "C" fn(*mut nf_conn, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>, data: *mut ::core::ffi::c_void);
    pub fn nf_conntrack_free(ct: *mut nf_conn);
    pub fn nf_conntrack_alloc(net: *mut net, zone: *const nf_conntrack_zone, orig: *const nf_conntrack_tuple, repl: *const nf_conntrack_tuple, gfp: gfp_t) -> *mut nf_conn;
}

#[inline] pub unsafe fn nf_ct_is_template(ct: *const nf_conn) -> ::core::ffi::c_int { test_bit(IPS_TEMPLATE_BIT, &(*ct).status) }
#[inline] pub unsafe fn nf_ct_is_confirmed(ct: *const nf_conn) -> ::core::ffi::c_int { test_bit(IPS_CONFIRMED_BIT, &(*ct).status) }
#[inline] pub unsafe fn nf_ct_is_dying(ct: *const nf_conn) -> ::core::ffi::c_int { test_bit(IPS_DYING_BIT, &(*ct).status) }

#[inline]
pub unsafe fn nf_is_loopback_packet(skb: *const sk_buff) -> bool { !(*skb).dev.is_null() && (*skb).skb_iif != 0 && ((*(*skb).dev).flags & IFF_LOOPBACK) != 0 }

#[inline]
pub unsafe fn nf_conntrack_alter_reply(ct: *mut nf_conn, newreply: *const nf_conntrack_tuple) {
    if WARN_ON(nf_ct_is_confirmed(ct) != 0) { return; }
    (*ct).tuplehash[IP_CT_DIR_REPLY as usize].tuple = *newreply;
}

#[inline] pub unsafe fn nf_ct_expires(ct: *const nf_conn) -> ::core::ffi::c_ulong { max(READ_ONCE((*ct).timeout) as i32 - nfct_time_stamp() as i32, 0) as ::core::ffi::c_ulong }
#[inline] pub unsafe fn nf_ct_is_expired(ct: *const nf_conn) -> bool { (READ_ONCE((*ct).timeout).wrapping_sub(nfct_time_stamp()) as i32) <= 0 }
#[inline] pub unsafe fn nf_ct_should_gc(ct: *const nf_conn) -> bool { if nf_ct_is_confirmed(ct) == 0 { return false; } smp_acquire__after_ctrl_dep(); nf_ct_is_expired(ct) && nf_ct_is_dying(ct) == 0 }

pub const NF_CT_DAY: u32 = 86400 * HZ;

extern "C" {
    pub fn nf_conntrack_set_hashsize(val: *const ::core::ffi::c_char, kp: *const kernel_param) -> ::core::ffi::c_int;
    pub fn nf_conntrack_hash_resize(hashsize: u32) -> ::core::ffi::c_int;
    pub static mut nf_conntrack_hash: *mut hlist_nulls_head;
    pub static mut nf_conntrack_htable_size: u32;
    pub static mut nf_conntrack_generation: seqcount_spinlock_t;
    pub static mut nf_conntrack_max: u32;
    pub fn nf_ct_tmpl_alloc(net: *mut net, zone: *const nf_conntrack_zone, flags: gfp_t) -> *mut nf_conn;
    pub fn nf_ct_tmpl_free(tmpl: *mut nf_conn);
    pub fn nf_ct_get_id(ct: *const nf_conn) -> u32;
    pub fn nf_conntrack_count(net: *const net) -> u32;
    pub static mut nf_conntrack_net_id: u32;
    pub fn nf_ct_skb_network_trim(skb: *mut sk_buff, family: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn nf_ct_handle_fragments(net: *mut net, skb: *mut sk_buff, zone: u16, family: u8, proto: *mut u8, mru: *mut u16) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn nf_conntrack_get_ht(hash: *mut *mut hlist_nulls_head, hsize: *mut u32) {
    let mut sequence: u32;
    let mut hptr: *mut hlist_nulls_head;
    let mut hsz: u32;
    loop {
        sequence = read_seqcount_begin(&nf_conntrack_generation);
        hsz = nf_conntrack_htable_size;
        hptr = nf_conntrack_hash;
        if !read_seqcount_retry(&nf_conntrack_generation, sequence) { break; }
    }
    *hash = hptr;
    *hsize = hsz;
}

#[inline] pub unsafe fn nf_ct_pernet(net: *const net) -> *mut nf_conntrack_net { net_generic(net, nf_conntrack_net_id) }
#[inline] pub unsafe fn NF_CT_STAT_INC(net: *mut net, count: usize) { __this_cpu_inc((*net).ct.stat, count); }
#[inline] pub unsafe fn NF_CT_STAT_INC_ATOMIC(net: *mut net, count: usize) { this_cpu_inc((*net).ct.stat, count); }
#[inline] pub unsafe fn NF_CT_STAT_ADD_ATOMIC(net: *mut net, count: usize, v: u32) { this_cpu_add((*net).ct.stat, count, v); }

#[inline] pub unsafe fn nf_ct_set(skb: *mut sk_buff, ct: *mut nf_conn, info: ip_conntrack_info) { skb_set_nfct(skb, ct as usize | info as usize); }
#[inline] pub unsafe fn nf_ct_pernet(net: *const net) -> *mut nf_conntrack_net { net_generic(net, nf_conntrack_net_id) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
