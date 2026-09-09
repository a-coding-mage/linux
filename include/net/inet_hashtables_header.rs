/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from inet_hashtables.h. Included dependencies are supplied externally. */

pub const FASTREUSEPORT_ANY: i32 = 1;
pub const FASTREUSEPORT_STRICT: i32 = 2;

#[repr(C)]
pub struct inet_ehash_bucket {
    pub chain: hlist_nulls_head,
}

#[repr(C)]
pub struct inet_bind_bucket {
    pub ib_net: possible_net_t,
    pub l3mdev: i32,
    pub port: u16,
    pub fastreuse: i8,
    pub fastreuseport: i8,
    pub fastuid: kuid_t,
    // Present when CONFIG_IPV6 is enabled.
    pub fast_v6_rcv_saddr: in6_addr,
    pub fast_rcv_saddr: __be32,
    pub fast_sk_family: u16,
    pub fast_ipv6_only: bool,
    pub node: hlist_node,
    pub bhash2: hlist_head,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct inet_bind2_bucket {
    pub ib_net: possible_net_t,
    pub l3mdev: i32,
    pub port: u16,
    // CONFIG_IPV6 layout selects addr_type and v6_rcv_saddr; otherwise rcv_saddr is __be32.
    pub addr_type: u16,
    pub v6_rcv_saddr: in6_addr,
    pub node: hlist_node,
    pub bhash_node: hlist_node,
    pub owners: hlist_head,
    pub fastreuse: i8,
    pub fastreuseport: i8,
}

#[inline]
pub unsafe fn ib_net(ib: *const inet_bind_bucket) -> *mut net {
    read_pnet(&(*ib).ib_net)
}

#[inline]
pub unsafe fn ib2_net(ib: *const inet_bind2_bucket) -> *mut net {
    read_pnet(&(*ib).ib_net)
}

#[macro_export]
macro_rules! inet_bind_bucket_for_each { ($tb:expr, $head:expr) => { hlist_for_each_entry!($tb, $head, node) }; }

#[repr(C)]
pub struct inet_bind_hashbucket {
    pub lock: spinlock_t,
    pub chain: hlist_head,
}

pub const LISTENING_NULLS_BASE: u32 = 1u32 << 29;

#[repr(C)]
pub struct inet_listen_hashbucket {
    pub lock: spinlock_t,
    pub nulls_head: hlist_nulls_head,
}

pub const INET_LHTABLE_SIZE: usize = 32;

#[repr(C)]
pub struct inet_hashinfo {
    pub ehash: *mut inet_ehash_bucket,
    pub ehash_locks: *mut spinlock_t,
    pub ehash_mask: u32,
    pub ehash_locks_mask: u32,
    pub bind_bucket_cachep: *mut kmem_cache,
    pub bhash: *mut inet_bind_hashbucket,
    pub bind2_bucket_cachep: *mut kmem_cache,
    pub bhash2: *mut inet_bind_hashbucket,
    pub bhash_size: u32,
    pub lhash2_mask: u32,
    pub lhash2: *mut inet_listen_hashbucket,
    pub pernet: bool,
}

#[inline]
pub unsafe fn tcp_get_hashinfo(sk: *const sock) -> *mut inet_hashinfo {
    (*sock_net(sk)).ipv4.tcp_death_row.hashinfo
}

#[inline]
pub unsafe fn inet_lhash2_bucket(h: *mut inet_hashinfo, hash: u32) -> *mut inet_listen_hashbucket {
    &mut *(*h).lhash2.add((hash & (*h).lhash2_mask) as usize)
}

#[inline]
pub unsafe fn inet_ehash_bucket(hashinfo: *mut inet_hashinfo, hash: u32) -> *mut inet_ehash_bucket {
    &mut *(*hashinfo).ehash.add((hash & (*hashinfo).ehash_mask) as usize)
}

#[inline]
pub unsafe fn inet_ehash_lockp(hashinfo: *mut inet_hashinfo, hash: u32) -> *mut spinlock_t {
    &mut *(*hashinfo).ehash_locks.add((hash & (*hashinfo).ehash_locks_mask) as usize)
}

extern "C" {
    pub fn inet_ehash_locks_alloc(hashinfo: *mut inet_hashinfo) -> i32;
}

#[inline]
pub unsafe fn inet_ehash_locks_free(hashinfo: *mut inet_hashinfo) {
    kvfree((*hashinfo).ehash_locks as *mut _);
    (*hashinfo).ehash_locks = core::ptr::null_mut();
}

extern "C" {
    pub fn inet_pernet_hashinfo_alloc(hashinfo: *mut inet_hashinfo, ehash_entries: u32) -> *mut inet_hashinfo;
    pub fn inet_pernet_hashinfo_free(hashinfo: *mut inet_hashinfo);
    pub fn inet_bind_bucket_create(cachep: *mut kmem_cache, net: *mut net, head: *mut inet_bind_hashbucket, snum: u16, l3mdev: i32) -> *mut inet_bind_bucket;
    pub fn inet_bind_bucket_destroy(tb: *mut inet_bind_bucket);
    pub fn inet_bind_bucket_match(tb: *const inet_bind_bucket, net: *const net, port: u16, l3mdev: i32) -> bool;
    pub fn inet_bind2_bucket_create(cachep: *mut kmem_cache, net: *mut net, head: *mut inet_bind_hashbucket, tb: *mut inet_bind_bucket, sk: *const sock) -> *mut inet_bind2_bucket;
    pub fn inet_bind2_bucket_destroy(cachep: *mut kmem_cache, tb: *mut inet_bind2_bucket);
    pub fn inet_bind2_bucket_find(head: *const inet_bind_hashbucket, net: *const net, port: u16, l3mdev: i32, sk: *const sock) -> *mut inet_bind2_bucket;
    pub fn inet_bind2_bucket_match_addr_any(tb: *const inet_bind2_bucket, net: *const net, port: u16, l3mdev: i32, sk: *const sock) -> bool;
}

#[inline]
pub unsafe fn inet_bhashfn(net: *const net, lport: u16, bhash_size: u32) -> u32 {
    (lport as u32 + net_hash_mix(net)) & (bhash_size - 1)
}

#[inline]
pub unsafe fn inet_bhashfn_portaddr(hinfo: *const inet_hashinfo, sk: *const sock, net: *const net, port: u16) -> *mut inet_bind_hashbucket {
    let hash = if (*sk).sk_family == AF_INET6 { ipv6_portaddr_hash(net, &(*sk).sk_v6_rcv_saddr, port) } else { ipv4_portaddr_hash(net, (*sk).sk_rcv_saddr, port) };
    &mut *(*hinfo).bhash2.add((hash & ((*hinfo).bhash_size - 1)) as usize)
}

#[inline]
pub unsafe fn inet_use_hash2_on_bind(sk: *const sock) -> bool {
    if (*sk).sk_family == AF_INET6 {
        if ipv6_addr_any(&(*sk).sk_v6_rcv_saddr) { return false; }
        if !ipv6_addr_v4mapped(&(*sk).sk_v6_rcv_saddr) { return true; }
    }
    (*sk).sk_rcv_saddr != htonl(INADDR_ANY)
}

extern "C" {
    pub fn inet_bhash2_addr_any_hashbucket(sk: *const sock, net: *const net, port: i32) -> *mut inet_bind_hashbucket;
    pub fn inet_bhash2_update_saddr(sk: *mut sock, saddr: *mut core::ffi::c_void, family: i32) -> i32;
    pub fn inet_bhash2_reset_saddr(sk: *mut sock);
    pub fn inet_bind_hash(sk: *mut sock, tb: *mut inet_bind_bucket, tb2: *mut inet_bind2_bucket, port: u16);
    pub fn __inet_inherit_port(sk: *const sock, child: *mut sock) -> i32;
    pub fn inet_put_port(sk: *mut sock);
    pub fn inet_hashinfo2_init(h: *mut inet_hashinfo, name: *const i8, numentries: c_ulong, scale: i32, low_limit: c_ulong, high_limit: c_ulong);
    pub fn inet_ehash_insert(sk: *mut sock, osk: *mut sock, found_dup_sk: *mut bool) -> bool;
    pub fn inet_ehash_nolisten(sk: *mut sock, osk: *mut sock, found_dup_sk: *mut bool) -> bool;
    pub fn inet_hash(sk: *mut sock) -> i32;
    pub fn inet_unhash(sk: *mut sock);
}

extern "C" {
    pub fn __inet_lookup_listener(net: *const net, skb: *mut sk_buff, doff: i32, saddr: __be32, sport: __be16, daddr: __be32, hnum: u16, dif: i32, sdif: i32) -> *mut sock;
}

#[inline]
pub unsafe fn inet_lookup_listener(net: *mut net, skb: *mut sk_buff, doff: i32, saddr: __be32, sport: __be16, daddr: __be32, dport: __be16, dif: i32, sdif: i32) -> *mut sock {
    __inet_lookup_listener(net, skb, doff, saddr, sport, daddr, ntohs(dport), dif, sdif)
}

#[macro_export]
macro_rules! INET_COMBINED_PORTS { ($sport:expr, $dport:expr) => { (($dport as u32) << 16) | (($sport as u16) as u32) }; }
#[macro_export]
macro_rules! INET_ADDR_COOKIE { ($name:ident, $saddr:expr, $daddr:expr) => { let $name: u64 = (($saddr as u64) << 32) | ($daddr as u64); }; }

#[inline]
pub unsafe fn inet_match(net: *const net, sk: *const sock, cookie: u64, ports: u32, dif: i32, sdif: i32) -> bool {
    if !net_eq(sock_net(sk), net) || READ_ONCE!((*sk).sk_portpair) != ports || (*sk).sk_addrpair != cookie { return false; }
    inet_sk_bound_dev_eq(net, READ_ONCE!((*sk).sk_bound_dev_if), dif, sdif)
}

pub type inet_ehashfn_t = unsafe extern "C" fn(*const net, __be32, u16, __be32, __be16) -> u32;
extern "C" {
    pub fn __inet_lookup_established(net: *const net, saddr: __be32, sport: __be16, daddr: __be32, hnum: u16, dif: i32, sdif: i32) -> *mut sock;
    pub static mut inet_ehashfn: inet_ehashfn_t;
    pub static mut udp_ehashfn: inet_ehashfn_t;
    pub fn inet_lookup_reuseport(net: *const net, sk: *mut sock, skb: *mut sk_buff, doff: i32, saddr: __be32, sport: __be16, daddr: __be32, hnum: u16, ehashfn: inet_ehashfn_t) -> *mut sock;
    pub fn inet_lookup_run_sk_lookup(net: *const net, protocol: i32, skb: *mut sk_buff, doff: i32, saddr: __be32, sport: __be16, daddr: __be32, hnum: u16, dif: i32, ehashfn: inet_ehashfn_t) -> *mut sock;
}

#[inline]
pub unsafe fn inet_lookup_established(net: *mut net, saddr: __be32, sport: __be16, daddr: __be32, dport: __be16, dif: i32) -> *mut sock { __inet_lookup_established(net, saddr, sport, daddr, ntohs(dport), dif, 0) }

#[inline]
pub unsafe fn __inet_lookup(net: *mut net, skb: *mut sk_buff, doff: i32, saddr: __be32, sport: __be16, daddr: __be32, dport: __be16, dif: i32, sdif: i32, refcounted: *mut bool) -> *mut sock {
    let hnum = ntohs(dport);
    let sk = __inet_lookup_established(net, saddr, sport, daddr, hnum, dif, sdif);
    *refcounted = true;
    if !sk.is_null() { return sk; }
    *refcounted = false;
    __inet_lookup_listener(net, skb, doff, saddr, sport, daddr, hnum, dif, sdif)
}

#[inline]
pub unsafe fn inet_lookup(net: *mut net, skb: *mut sk_buff, doff: i32, saddr: __be32, sport: __be16, daddr: __be32, dport: __be16, dif: i32) -> *mut sock {
    let mut refcounted = false;
    let sk = __inet_lookup(net, skb, doff, saddr, sport, daddr, dport, dif, 0, &mut refcounted);
    if !sk.is_null() && !refcounted && !refcount_inc_not_zero(&mut (*sk).sk_refcnt) { return core::ptr::null_mut(); }
    sk
}

#[inline]
pub unsafe fn inet_steal_sock(net: *mut net, skb: *mut sk_buff, doff: i32, saddr: __be32, sport: __be16, daddr: __be32, dport: __be16, refcounted: *mut bool, ehashfn: inet_ehashfn_t) -> *mut sock {
    let mut prefetched = false;
    let sk = skb_steal_sock(skb, refcounted, &mut prefetched);
    if sk.is_null() || !prefetched || !sk_fullsock(sk) { return sk; }
    if (*sk).sk_protocol == IPPROTO_TCP {
        if (*sk).sk_state != TCP_LISTEN { return sk; }
    } else if (*sk).sk_protocol == IPPROTO_UDP {
        if (*sk).sk_state != TCP_CLOSE { return sk; }
    } else { return sk; }
    let reuse_sk = inet_lookup_reuseport(net, sk, skb, doff, saddr, sport, daddr, ntohs(dport), ehashfn);
    if reuse_sk.is_null() { sk } else { reuse_sk }
}

#[inline]
pub unsafe fn __inet_lookup_skb(skb: *mut sk_buff, doff: i32, sport: __be16, dport: __be16, sdif: i32, refcounted: *mut bool) -> *mut sock {
    let net = skb_dst_dev_net_rcu(skb);
    let iph = ip_hdr(skb);
    let sk = inet_steal_sock(net, skb, doff, (*iph).saddr, sport, (*iph).daddr, dport, refcounted, inet_ehashfn);
    if !sk.is_null() { return sk; }
    __inet_lookup(net, skb, doff, (*iph).saddr, sport, (*iph).daddr, dport, inet_iif(skb), sdif, refcounted)
}

#[inline]
pub unsafe fn sk_daddr_set(sk: *mut sock, addr: __be32) {
    (*sk).sk_daddr = addr;
    ipv6_addr_set_v4mapped(addr, &mut (*sk).sk_v6_daddr);
}

#[inline]
pub unsafe fn sk_rcv_saddr_set(sk: *mut sock, addr: __be32) {
    (*sk).sk_rcv_saddr = addr;
    ipv6_addr_set_v4mapped(addr, &mut (*sk).sk_v6_rcv_saddr);
}

// Remaining lookup helpers retain the C control flow and call external kernel primitives.
extern "C" {
    pub fn __inet_hash_connect(death_row: *mut inet_timewait_death_row, sk: *mut sock, port_offset: u64, hash_port0: u32, check_established: *mut core::ffi::c_void) -> i32;
    pub fn inet_hash_connect(death_row: *mut inet_timewait_death_row, sk: *mut sock) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
