// SPDX-License-Identifier: GPL-2.0
// Direct Rust translation of the Linux IPv4 fragmentation implementation.

const PR_FMT: &str = "IPv4: ";
static IP_FRAG_CACHE_NAME: &[u8] = b"ip4-frags\0";

#[repr(C)]
pub struct ipq {
    pub q: inet_frag_queue,
    pub ecn: u8,
    pub max_df_size: u16,
    pub iif: i32,
    pub rid: u32,
    pub peer: *mut inet_peer,
}

unsafe fn ip4_frag_ecn(tos: u8) -> u8 { 1u8 << (tos & INET_ECN_MASK) }

static mut ip4_frags: inet_frags = unsafe { core::mem::zeroed() };

unsafe fn ip4_frag_init(q: *mut inet_frag_queue, a: *const core::ffi::c_void) {
    let qp = container_of!(q, ipq, q);
    let key = &*(a as *const frag_v4_compare_key);
    let net = (*(*q).fqdir).net;
    let mut p: *mut inet_peer = core::ptr::null_mut();
    (*q).key.v4 = *key;
    (*qp).ecn = 0;
    if (*(*q).fqdir).max_dist != 0 {
        rcu_read_lock();
        p = inet_getpeer((*net).ipv4.peers, key.saddr, key.vif);
        if !p.is_null() && !refcount_inc_not_zero(&mut (*p).refcnt) { p = core::ptr::null_mut(); }
        rcu_read_unlock();
    }
    (*qp).peer = p;
}

unsafe fn ip4_frag_free(q: *mut inet_frag_queue) {
    let qp = container_of!(q, ipq, q);
    if !(*qp).peer.is_null() { inet_putpeer((*qp).peer); }
}

unsafe fn frag_expire_skip_icmp(user: u32) -> bool {
    user == IP_DEFRAG_AF_PACKET ||
        ip_defrag_user_in_between(user, IP_DEFRAG_CONNTRACK_IN, __IP_DEFRAG_CONNTRACK_IN_END) ||
        ip_defrag_user_in_between(user, IP_DEFRAG_CONNTRACK_BRIDGE_IN, __IP_DEFRAG_CONNTRACK_BRIDGE_IN)
}

unsafe fn ip_expire(t: *mut timer_list) {
    let mut reason = SKB_DROP_REASON_FRAG_REASM_TIMEOUT;
    let frag = timer_container_of!(t, inet_frag_queue, timer);
    let mut head: *mut sk_buff = core::ptr::null_mut();
    let qp = container_of!(frag, ipq, q);
    let net = (*(*qp).q.fqdir).net;
    let mut refs = 1;
    rcu_read_lock(); spin_lock(&mut (*qp).q.lock);
    if (*qp).q.flags & INET_FRAG_COMPLETE != 0 { goto!(out); }
    (*qp).q.flags |= INET_FRAG_DROP;
    inet_frag_kill(&mut (*qp).q, &mut refs);
    if READ_ONCE!((*(*qp).q.fqdir).dead) { inet_frag_queue_flush(&mut (*qp).q, 0); goto!(out); }
    __IP_INC_STATS(net, IPSTATS_MIB_REASMFAILS); __IP_INC_STATS(net, IPSTATS_MIB_REASMTIMEOUT);
    if (*qp).q.flags & INET_FRAG_FIRST_IN == 0 { goto!(out); }
    head = inet_frag_pull_head(&mut (*qp).q);
    if head.is_null() { goto!(out); }
    (*head).dev = dev_get_by_index_rcu(net, (*qp).iif);
    if (*head).dev.is_null() { goto!(out); }
    let iph = ip_hdr(head);
    reason = ip_route_input_noref(head, (*iph).daddr, (*iph).saddr, ip4h_dscp(iph), (*head).dev);
    if reason != 0 { goto!(out); }
    reason = SKB_DROP_REASON_FRAG_REASM_TIMEOUT;
    if frag_expire_skip_icmp((*qp).q.key.v4.user) && (*skb_rtable(head)).rt_type != RTN_LOCAL { goto!(out); }
    spin_unlock(&mut (*qp).q.lock); icmp_send(head, ICMP_TIME_EXCEEDED, ICMP_EXC_FRAGTIME, 0); goto!(out_rcu_unlock);
    out: { spin_unlock(&mut (*qp).q.lock); }
    out_rcu_unlock: { rcu_read_unlock(); kfree_skb_reason(head, reason); inet_frag_putn(&mut (*qp).q, refs); }
}

unsafe fn ip_find(net: *mut net, iph: *mut iphdr, user: u32, vif: i32) -> *mut ipq {
    let key = frag_v4_compare_key { saddr: (*iph).saddr, daddr: (*iph).daddr, user, vif, id: (*iph).id, protocol: (*iph).protocol };
    let q = inet_frag_find((*net).ipv4.fqdir, &key as *const _ as *const _);
    if q.is_null() { core::ptr::null_mut() } else { container_of!(q, ipq, q) }
}

unsafe fn ip_frag_too_far(qp: *mut ipq) -> i32 {
    let peer = (*qp).peer; let max = (*(*qp).q.fqdir).max_dist;
    if peer.is_null() || max == 0 { return 0; }
    let start = (*qp).rid; let end = atomic_inc_return(&mut (*peer).rid); (*qp).rid = end;
    let rc = if !(*qp).q.fragments_tail.is_null() && end.wrapping_sub(start) > max { 1 } else { 0 };
    if rc != 0 { __IP_INC_STATS((*(*qp).q.fqdir).net, IPSTATS_MIB_REASMFAILS); } rc
}

unsafe fn ip_frag_reinit(qp: *mut ipq) -> i32 {
    if !mod_timer_pending(&mut (*qp).q.timer, jiffies + (*(*qp).q.fqdir).timeout) { return -ETIMEDOUT; }
    inet_frag_queue_flush(&mut (*qp).q, SKB_DROP_REASON_FRAG_TOO_FAR); (*qp).q.flags = 0; (*qp).q.len = 0; (*qp).q.meat = 0; (*qp).iif = 0; (*qp).ecn = 0; 0
}

unsafe fn ip_frag_queue(qp: *mut ipq, skb: *mut sk_buff, refs: *mut i32) -> i32 {
    let net = (*(*qp).q.fqdir).net;
    if (*qp).q.flags & INET_FRAG_COMPLETE != 0 { kfree_skb_reason(skb, DUP_FRAG); return -ENOENT; }
    if (*skb).ip_summed != CHECKSUM_UNNECESSARY && (*skb).ip_summed != CHECKSUM_NONE { (*skb).ip_summed = CHECKSUM_NONE; }
    let iph = ip_hdr(skb); let mut offset = ntohs((*iph).frag_off); let flags = offset & !IP_OFFSET;
    offset = (offset & IP_OFFSET) << 3;
    let ihl = ip_hdrlen(skb); let end = offset + (*skb).len - skb_network_offset(skb) - ihl;
    if flags & IP_MF == 0 { if end < (*qp).q.len || ((*qp).q.flags & INET_FRAG_LAST_IN != 0 && end != (*qp).q.len) { inet_frag_kill(&mut (*qp).q, refs); kfree_skb_reason(skb, -EINVAL); return -EINVAL; } (*qp).q.flags |= INET_FRAG_LAST_IN; (*qp).q.len = end; }
    else { let end2 = end & !7; if end2 > (*qp).q.len { if (*qp).q.flags & INET_FRAG_LAST_IN != 0 { inet_frag_kill(&mut (*qp).q, refs); kfree_skb_reason(skb, -EINVAL); return -EINVAL; } (*qp).q.len = end2; } }
    if end == offset || !pskb_pull(skb, skb_network_offset(skb) + ihl) { inet_frag_kill(&mut (*qp).q, refs); kfree_skb_reason(skb, -ENOMEM); return -ENOMEM; }
    if pskb_trim_rcsum(skb, end - offset) != 0 { inet_frag_kill(&mut (*qp).q, refs); kfree_skb_reason(skb, -ENOMEM); return -ENOMEM; }
    let prev_tail = (*qp).q.fragments_tail; let dev = (*skb).dev;
    let err = inet_frag_queue_insert(&mut (*qp).q, skb, offset, end);
    if err != 0 { inet_frag_kill(&mut (*qp).q, refs); __IP_INC_STATS(net, IPSTATS_MIB_REASM_OVERLAPS); kfree_skb_reason(skb, -EINVAL); return -EINVAL; }
    if !dev.is_null() { (*qp).iif = (*dev).ifindex; }
    (*qp).q.meat += (*skb).len; (*qp).ecn |= ip4_frag_ecn((*ip_hdr(skb)).tos); add_frag_mem_limit((*qp).q.fqdir, (*skb).truesize);
    if offset == 0 { (*qp).q.flags |= INET_FRAG_FIRST_IN; }
    let fragsize = (*skb).len + ihl; if fragsize > (*qp).q.max_size { (*qp).q.max_size = fragsize; }
    if (*ip_hdr(skb)).frag_off & htons(IP_DF) != 0 && fragsize > (*qp).max_df_size { (*qp).max_df_size = fragsize as u16; }
    if (*qp).q.flags == (INET_FRAG_FIRST_IN | INET_FRAG_LAST_IN) && (*qp).q.meat == (*qp).q.len { let r = ip_frag_reasm(qp, skb, prev_tail, dev, refs); if r != 0 { inet_frag_kill(&mut (*qp).q, refs); } return r; }
    skb_dst_drop(skb); skb_orphan(skb); -EINPROGRESS
}

unsafe fn ip_frag_reasm(qp: *mut ipq, skb: *mut sk_buff, prev_tail: *mut sk_buff, dev: *mut net_device, refs: *mut i32) -> i32 {
    let net = (*(*qp).q.fqdir).net; inet_frag_kill(&mut (*qp).q, refs);
    let reasm_data = inet_frag_reasm_prepare(&mut (*qp).q, skb, prev_tail); if reasm_data.is_null() { __IP_INC_STATS(net, IPSTATS_MIB_REASMFAILS); return -ENOMEM; }
    let len = ip_hdrlen(skb) + (*qp).q.len; if len > 65535 { __IP_INC_STATS(net, IPSTATS_MIB_REASMFAILS); return -E2BIG; }
    inet_frag_reasm_finish(&mut (*qp).q, skb, reasm_data, ip_frag_coalesce_ok(qp)); (*skb).dev = dev;
    IPCB!(skb).frag_max_size = core::cmp::max((*qp).max_df_size as u32, (*qp).q.max_size); let iph = ip_hdr(skb); (*iph).tot_len = htons(len as u16); (*iph).tos |= ip_frag_ecn_table[(*qp).ecn];
    (*iph).frag_off = if (*qp).max_df_size as u32 == (*qp).q.max_size { IPCB!(skb).flags |= IPSKB_FRAG_PMTU; htons(IP_DF) } else { 0 }; ip_send_check(iph); __IP_INC_STATS(net, IPSTATS_MIB_REASMOKS); (*qp).q.rb_fragments = RB_ROOT; (*qp).q.fragments_tail = core::ptr::null_mut(); (*qp).q.last_run_head = core::ptr::null_mut(); 0
}

unsafe fn ip_frag_coalesce_ok(qp: *const ipq) -> bool { (*qp).q.key.v4.user == IP_DEFRAG_LOCAL_DELIVER }

pub unsafe fn ip_defrag(net: *mut net, skb: *mut sk_buff, user: u32) -> i32 {
    __IP_INC_STATS(net, IPSTATS_MIB_REASMREQDS); rcu_read_lock(); let dev = if !(*skb).dev.is_null() { (*skb).dev } else { skb_dst_dev_rcu(skb) }; let qp = ip_find(net, ip_hdr(skb), user, l3mdev_master_ifindex_rcu(dev));
    if !qp.is_null() { let mut refs = 0; spin_lock(&mut (*qp).q.lock); let ret = ip_frag_queue(qp, skb, &mut refs); spin_unlock(&mut (*qp).q.lock); rcu_read_unlock(); inet_frag_putn(&mut (*qp).q, refs); ret } else { rcu_read_unlock(); __IP_INC_STATS(net, IPSTATS_MIB_REASMFAILS); kfree_skb(skb); -ENOMEM }
}

pub unsafe fn ip_check_defrag(net: *mut net, skb: *mut sk_buff, user: u32) -> *mut sk_buff {
    if (*skb).protocol != htons(ETH_P_IP) { return skb; }
    let netoff = skb_network_offset(skb); let mut iph: iphdr = core::mem::zeroed(); if skb_copy_bits(skb, netoff, &mut iph as *mut _ as *mut _, core::mem::size_of::<iphdr>()) < 0 || iph.ihl < 5 || iph.version != 4 { return skb; }
    let len = ntohs(iph.tot_len); if (*skb).len < netoff + len as usize || len < (iph.ihl as usize * 4) { return skb; }
    if ip_is_fragment(&iph) { let s = skb_share_check(skb, GFP_ATOMIC); if s.is_null() { return core::ptr::null_mut(); } if !pskb_may_pull(s, netoff + iph.ihl as usize * 4) { kfree_skb(s); return core::ptr::null_mut(); } if pskb_trim_rcsum(s, netoff + len as usize) != 0 { kfree_skb(s); return core::ptr::null_mut(); } memset(IPCB!(s), 0, core::mem::size_of::<inet_skb_parm>()); if ip_defrag(net, s, user) != 0 { return core::ptr::null_mut(); } skb_clear_hash(s); return s; } skb
}

unsafe fn ipv4_frags_init_net(net: *mut net) -> i32 {
    let mut res = fqdir_init(&mut (*net).ipv4.fqdir, &mut ip4_frags, net); if res < 0 { return res; }
    (*(*net).ipv4.fqdir).high_thresh = 4 * 1024 * 1024; (*(*net).ipv4.fqdir).low_thresh = 3 * 1024 * 1024; (*(*net).ipv4.fqdir).timeout = IP_FRAG_TIME; (*(*net).ipv4.fqdir).max_dist = 64;
    res = ip4_frags_ns_ctl_register(net); if res < 0 { fqdir_exit((*net).ipv4.fqdir); } res
}
unsafe fn ipv4_frags_pre_exit_net(net: *mut net) { fqdir_pre_exit((*net).ipv4.fqdir); }
unsafe fn ipv4_frags_exit_net(net: *mut net) { ip4_frags_ns_ctl_unregister(net); fqdir_exit((*net).ipv4.fqdir); }
unsafe fn ip4_frags_ns_ctl_register(_net: *mut net) -> i32 { 0 }
unsafe fn ip4_frags_ns_ctl_unregister(_net: *mut net) {}

pub unsafe fn ipfrag_init() {
    ip4_frags.constructor = Some(ip4_frag_init); ip4_frags.destructor = Some(ip4_frag_free); ip4_frags.qsize = core::mem::size_of::<ipq>(); ip4_frags.frag_expire = Some(ip_expire); ip4_frags.frags_cache_name = IP_FRAG_CACHE_NAME.as_ptr() as *const _;
    if inet_frags_init(&mut ip4_frags) != 0 { panic!("IP: failed to allocate ip4_frags cache\n"); }
    register_pernet_subsys(&mut ip4_frags_ops);
}

static mut ip4_frags_ops: pernet_operations = unsafe { core::mem::zeroed() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
