// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPVS:        Source Hashing scheduling module
 *
 * Authors:     Wensong Zhang <wensong@gnuchina.org>
 */

/* The C headers and build-time configuration are supplied by the surrounding
 * kernel translation. */

#[repr(C)]
pub struct IpVsShBucket {
    pub dest: *mut IpVsDest,
}

// CONFIG_IP_VS_SH_TAB_BITS defaults to 8 when not supplied by the build.
pub const CONFIG_IP_VS_SH_TAB_BITS: usize = 8;
pub const IP_VS_SH_TAB_BITS: usize = CONFIG_IP_VS_SH_TAB_BITS;
pub const IP_VS_SH_TAB_SIZE: usize = 1usize << IP_VS_SH_TAB_BITS;
pub const IP_VS_SH_TAB_MASK: usize = IP_VS_SH_TAB_SIZE - 1;

#[repr(C)]
pub struct IpVsShState {
    pub rcu_head: RcuHead,
    pub buckets: [IpVsShBucket; IP_VS_SH_TAB_SIZE],
}

#[inline]
unsafe fn is_unavailable(dest: *mut IpVsDest) -> bool {
    atomic_read(&(*dest).weight) <= 0 || ((*dest).flags & IP_VS_DEST_F_OVERLOAD) != 0
}

#[inline]
unsafe fn ip_vs_sh_hashkey(
    af: i32,
    addr: *const NfInetAddr,
    port: Be16,
    offset: u32,
) -> u32 {
    let mut addr_fold = (*addr).ip;
    // CONFIG_IP_VS_IPV6 conditionally includes the IPv6 address folding.
    if af == AF_INET6 {
        addr_fold = (*addr).ip6[0] ^ (*addr).ip6[1] ^ (*addr).ip6[2] ^ (*addr).ip6[3];
    }
    (offset.wrapping_add(hash_32(ntohs(port) as u32 + ntohl(addr_fold), IP_VS_SH_TAB_BITS as u32)))
        & IP_VS_SH_TAB_MASK as u32
}

#[inline]
unsafe fn ip_vs_sh_get(
    svc: *mut IpVsService,
    s: *mut IpVsShState,
    addr: *const NfInetAddr,
    port: Be16,
) -> *mut IpVsDest {
    let hash = ip_vs_sh_hashkey((*svc).af, addr, port, 0) as usize;
    let dest = rcu_dereference((*s).buckets[hash].dest);
    if dest.is_null() || is_unavailable(dest) { core::ptr::null_mut() } else { dest }
}

#[inline]
unsafe fn ip_vs_sh_get_fallback(
    svc: *mut IpVsService, s: *mut IpVsShState, addr: *const NfInetAddr, port: Be16,
) -> *mut IpVsDest {
    let mut offset: usize;
    let mut roffset: usize;
    let mut hash: usize;
    let ihash = ip_vs_sh_hashkey((*svc).af, addr, port, 0) as usize;
    let mut dest = rcu_dereference((*s).buckets[ihash].dest);
    if dest.is_null() { return core::ptr::null_mut(); }
    if !is_unavailable(dest) { return dest; }
    IP_VS_DBG_BUF(6, "SH: selected unavailable server %s:%d, reselecting", IP_VS_DBG_ADDR((*dest).af, &(*dest).addr), ntohs((*dest).port));
    offset = 0;
    while offset < IP_VS_SH_TAB_SIZE {
        roffset = (offset + ihash) % IP_VS_SH_TAB_SIZE;
        hash = ip_vs_sh_hashkey((*svc).af, addr, port, roffset as u32) as usize;
        dest = rcu_dereference((*s).buckets[hash].dest);
        if dest.is_null() { break; }
        if !is_unavailable(dest) { return dest; }
        IP_VS_DBG_BUF(6, "SH: selected unavailable server %s:%d (offset %d), reselecting", IP_VS_DBG_ADDR((*dest).af, &(*dest).addr), ntohs((*dest).port), roffset);
        offset += 1;
    }
    core::ptr::null_mut()
}

unsafe fn ip_vs_sh_reassign(s: *mut IpVsShState, svc: *mut IpVsService) -> i32 {
    let mut p: *mut ListHead = &mut (*svc).destinations;
    let empty = list_empty(p);
    let mut d_count = 0;
    for i in 0..IP_VS_SH_TAB_SIZE {
        let b = &mut (*s).buckets[i];
        let dest = rcu_dereference_protected(b.dest, 1);
        if !dest.is_null() { ip_vs_dest_put(dest); }
        if empty { RCU_INIT_POINTER(b.dest, core::ptr::null_mut()); }
        else {
            if p == &mut (*svc).destinations { p = (*p).next; }
            let dest = list_entry(p, "ip_vs_dest", "n_list");
            ip_vs_dest_hold(dest);
            RCU_INIT_POINTER(b.dest, dest);
            IP_VS_DBG_BUF(6, "assigned i: %d dest: %s weight: %d\n", i, IP_VS_DBG_ADDR((*dest).af, &(*dest).addr), atomic_read(&(*dest).weight));
            d_count += 1;
            if d_count >= atomic_read(&(*dest).weight) { p = (*p).next; d_count = 0; }
        }
    }
    0
}

unsafe fn ip_vs_sh_flush(s: *mut IpVsShState) {
    for i in 0..IP_VS_SH_TAB_SIZE {
        let b = &mut (*s).buckets[i];
        let dest = rcu_dereference_protected(b.dest, 1);
        if !dest.is_null() { ip_vs_dest_put(dest); RCU_INIT_POINTER(b.dest, core::ptr::null_mut()); }
    }
}

unsafe fn ip_vs_sh_init_svc(svc: *mut IpVsService) -> i32 {
    let s = kzalloc_obj::<IpVsShState>();
    if s.is_null() { return -ENOMEM; }
    (*svc).sched_data = s as *mut _;
    IP_VS_DBG(6, "SH hash table (memory=%zdbytes) allocated for current service\n", core::mem::size_of::<IpVsShBucket>() * IP_VS_SH_TAB_SIZE);
    ip_vs_sh_reassign(s, svc); 0
}

unsafe fn ip_vs_sh_done_svc(svc: *mut IpVsService) {
    let s = (*svc).sched_data as *mut IpVsShState;
    ip_vs_sh_flush(s); kfree_rcu(s, rcu_head);
    IP_VS_DBG(6, "SH hash table (memory=%zdbytes) released\n", core::mem::size_of::<IpVsShBucket>() * IP_VS_SH_TAB_SIZE);
}

unsafe fn ip_vs_sh_dest_changed(svc: *mut IpVsService, _dest: *mut IpVsDest) -> i32 {
    ip_vs_sh_reassign((*svc).sched_data as *mut IpVsShState, svc); 0
}

#[inline]
unsafe fn ip_vs_sh_get_port(skb: *const SkBuff, iph: *mut IpVsIphdr) -> Be16 {
    let mut ports = [0u16; 2];
    match (*iph).protocol {
        IPPROTO_TCP | IPPROTO_UDP | IPPROTO_SCTP => {
            if skb_header_pointer(skb, (*iph).len, core::mem::size_of_val(&ports), ports.as_mut_ptr() as *mut _) .is_null() { return 0; }
            if !ip_vs_iph_inverse(iph) { ports[0] } else { ports[1] }
        },
        _ => 0,
    }
}

unsafe fn ip_vs_sh_schedule(svc: *mut IpVsService, skb: *const SkBuff, iph: *mut IpVsIphdr) -> *mut IpVsDest {
    let hash_addr = if ip_vs_iph_inverse(iph) { &(*iph).daddr } else { &(*iph).saddr };
    IP_VS_DBG(6, "ip_vs_sh_schedule(): Scheduling...\n");
    let port = if ((*svc).flags & IP_VS_SVC_F_SCHED_SH_PORT) != 0 { ip_vs_sh_get_port(skb, iph) } else { 0 };
    let s = (*svc).sched_data as *mut IpVsShState;
    let dest = if ((*svc).flags & IP_VS_SVC_F_SCHED_SH_FALLBACK) != 0 { ip_vs_sh_get_fallback(svc, s, hash_addr, port) } else { ip_vs_sh_get(svc, s, hash_addr, port) };
    if dest.is_null() { ip_vs_scheduler_err(svc, "no destination available"); return core::ptr::null_mut(); }
    IP_VS_DBG_BUF(6, "SH: source IP address %s --> server %s:%d\n", IP_VS_DBG_ADDR((*svc).af, hash_addr), IP_VS_DBG_ADDR((*dest).af, &(*dest).addr), ntohs((*dest).port));
    dest
}

static mut ip_vs_sh_scheduler: IpVsScheduler = IpVsScheduler {
    name: "sh", refcnt: ATOMIC_INIT(0), module: THIS_MODULE,
    n_list: LIST_HEAD_INIT(), init_service: Some(ip_vs_sh_init_svc), done_service: Some(ip_vs_sh_done_svc),
    add_dest: Some(ip_vs_sh_dest_changed), del_dest: Some(ip_vs_sh_dest_changed), upd_dest: Some(ip_vs_sh_dest_changed), schedule: Some(ip_vs_sh_schedule),
};

unsafe fn ip_vs_sh_init() -> i32 { register_ip_vs_scheduler(&mut ip_vs_sh_scheduler) }
unsafe fn ip_vs_sh_cleanup() { unregister_ip_vs_scheduler(&mut ip_vs_sh_scheduler); synchronize_rcu(); }

// module_init(ip_vs_sh_init); module_exit(ip_vs_sh_cleanup);
// MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("ipvs source hashing scheduler");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
