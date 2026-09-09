// SPDX-License-Identifier: GPL-2.0-or-later
/* IPv6 fragment reassembly; Linux INET6 implementation. */

// C dependencies are supplied by the surrounding kernel translation unit.

static IP6_FRAG_CACHE_NAME: &[u8] = b"ip6-frags\0";

unsafe fn ip6_frag_ecn(ipv6h: *const ipv6hdr) -> u8 {
    1u8 << (ipv6_get_dsfield(ipv6h) & INET_ECN_MASK)
}

static mut ip6_frags: inet_frags = inet_frags { __bindgen_anon_1: 0 };

unsafe extern "C" fn ip6_frag_reasm(
    fq: *mut frag_queue, skb: *mut sk_buff, prev_tail: *mut sk_buff,
    dev: *mut net_device, idev: *mut inet6_dev, refs: *mut i32,
) -> i32;

unsafe extern "C" fn ip6_frag_expire(t: *mut timer_list) {
    let frag: *mut inet_frag_queue = timer_container_of(t);
    let fq: *mut frag_queue = container_of(frag);
    ip6frag_expire_frag_queue((*(*fq).q.fqdir).net, fq);
}

unsafe fn fq_find(net: *mut net, id: __be32, hdr: *const ipv6hdr, iif: i32) -> *mut frag_queue {
    let mut key = frag_v6_compare_key {
        id, saddr: (*hdr).saddr, daddr: (*hdr).daddr,
        user: IP6_DEFRAG_LOCAL_DELIVER, iif,
    };
    if (ipv6_addr_type(&(*hdr).daddr) & (IPV6_ADDR_MULTICAST | IPV6_ADDR_LINKLOCAL)) == 0 { key.iif = 0; }
    let q = inet_frag_find((*(*net).ipv6.fqdir), &key);
    if q.is_null() { core::ptr::null_mut() } else { container_of(q) }
}

unsafe fn ip6_frag_queue(net: *mut net, fq: *mut frag_queue, skb: *mut sk_buff,
    fhdr: *mut frag_hdr, nhoff: i32, prob_offset: *mut u32, refs: *mut i32,
    idev: *mut inet6_dev) -> i32 {
    let mut offset: i32;
    let mut end: i32;
    let mut fragsize: i32;
    let mut prev_tail: *mut sk_buff;
    let mut dev: *mut net_device;
    let mut err = -ENOENT;
    let mut reason = 0;
    let ecn: u8;
    if (*fq).q.flags & INET_FRAG_COMPLETE != 0 { reason = DUP_FRAG; kfree_skb_reason(skb, reason); return err; }
    err = -EINVAL;
    offset = ntohs((*fhdr).frag_off) as i32 & !0x7;
    end = offset + ntohs((*ipv6_hdr(skb)).payload_len) as i32 -
        (((fhdr.add(1) as *const u8).offset_from((ipv6_hdr(skb) as *const u8).add(1))) as i32);
    if end as u32 > IPV6_MAXPLEN {
        *prob_offset = (&mut (*fhdr).frag_off as *mut _ as *mut u8).offset_from(skb_network_header(skb)) as u32;
        inet_frag_kill(&mut (*fq).q, refs); __IP6_INC_STATS(net, idev, IPSTATS_MIB_REASMFAILS); return -1;
    }
    ecn = ip6_frag_ecn(ipv6_hdr(skb));
    if (*skb).ip_summed == CHECKSUM_COMPLETE {
        let nh = skb_network_header(skb);
        (*skb).csum = csum_sub((*skb).csum, csum_partial(nh, fhdr.add(1) as *const u8, 0));
    }
    if (*fhdr).frag_off & htons(IP6_MF) == 0 {
        if end < (*fq).q.len || ((*fq).q.flags & INET_FRAG_LAST_IN != 0 && end != (*fq).q.len) { inet_frag_kill(&mut (*fq).q, refs); __IP6_INC_STATS(net, idev, IPSTATS_MIB_REASMFAILS); kfree_skb_reason(skb, reason); return err; }
        (*fq).q.flags |= INET_FRAG_LAST_IN; (*fq).q.len = end;
    } else {
        if end & 0x7 != 0 { *prob_offset = offsetof_ipv6hdr_payload_len(); inet_frag_kill(&mut (*fq).q, refs); __IP6_INC_STATS(net, idev, IPSTATS_MIB_REASMFAILS); return -1; }
        if end > (*fq).q.len { if (*fq).q.flags & INET_FRAG_LAST_IN != 0 { inet_frag_kill(&mut (*fq).q, refs); __IP6_INC_STATS(net, idev, IPSTATS_MIB_REASMFAILS); kfree_skb_reason(skb, reason); return err; } (*fq).q.len = end; }
    }
    if end == offset { inet_frag_kill(&mut (*fq).q, refs); __IP6_INC_STATS(net, idev, IPSTATS_MIB_REASMFAILS); kfree_skb_reason(skb, reason); return err; }
    err = -ENOMEM;
    if pskb_pull(skb, fhdr.add(1) as *mut u8 as usize - (*skb).data as usize).is_null() { inet_frag_kill(&mut (*fq).q, refs); __IP6_INC_STATS(net, idev, IPSTATS_MIB_REASMFAILS); kfree_skb_reason(skb, reason); return err; }
    err = pskb_trim_rcsum(skb, (end - offset) as u32); if err != 0 { inet_frag_kill(&mut (*fq).q, refs); __IP6_INC_STATS(net, idev, IPSTATS_MIB_REASMFAILS); kfree_skb_reason(skb, reason); return err; }
    dev = (*skb).dev; barrier(); prev_tail = (*fq).q.fragments_tail;
    err = inet_frag_queue_insert(&mut (*fq).q, skb, offset, end); if err != 0 { if err == IPFRAG_DUP { reason = DUP_FRAG; } __IP6_INC_STATS(net, idev, IPSTATS_MIB_REASM_OVERLAPS); inet_frag_kill(&mut (*fq).q, refs); __IP6_INC_STATS(net, idev, IPSTATS_MIB_REASMFAILS); kfree_skb_reason(skb, reason); return -EINVAL; }
    if !dev.is_null() { (*fq).iif = (*dev).ifindex; }
    (*fq).q.stamp = (*skb).tstamp; (*fq).q.tstamp_type = (*skb).tstamp_type; (*fq).q.meat += (*skb).len; (*fq).ecn |= ecn; add_frag_mem_limit((*fq).q.fqdir, (*skb).truesize);
    fragsize = -skb_network_offset(skb) + (*skb).len as i32; if fragsize > (*fq).q.max_size { (*fq).q.max_size = fragsize; }
    if offset == 0 { (*fq).nhoffset = nhoff; (*fq).q.flags |= INET_FRAG_FIRST_IN; }
    if (*fq).q.flags == (INET_FRAG_FIRST_IN | INET_FRAG_LAST_IN) && (*fq).q.meat == (*fq).q.len {
        let orefdst = (*skb)._skb_refdst; (*skb)._skb_refdst = 0; let r = ip6_frag_reasm(fq, skb, prev_tail, dev, idev, refs); (*skb)._skb_refdst = orefdst; return r;
    }
    skb_dst_drop(skb); -EINPROGRESS
}

unsafe fn ipv6_frag_rcv(skb: *mut sk_buff) -> i32 {
    let mut hdr = ipv6_hdr(skb); let net = skb_dst_dev_net(skb); let idev = if !(*skb).dev.is_null() { __in6_dev_stats_get((*skb).dev, skb) } else { core::ptr::null_mut() };
    if IP6CB(skb).flags & IP6SKB_FRAGMENTED != 0 || (*hdr).payload_len == 0 { __IP6_INC_STATS(net, idev, IPSTATS_MIB_INHDRERRORS); icmpv6_param_prob(skb, ICMPV6_HDR_FIELD, skb_network_header_len(skb)); return -1; }
    __IP6_INC_STATS(net, idev, IPSTATS_MIB_REASMREQDS);
    if !pskb_may_pull(skb, skb_transport_offset(skb) + size_of::<frag_hdr>()) { __IP6_INC_STATS(net, idev, IPSTATS_MIB_INHDRERRORS); icmpv6_param_prob(skb, ICMPV6_HDR_FIELD, skb_network_header_len(skb)); return -1; }
    hdr = ipv6_hdr(skb); let fhdr = skb_transport_header(skb) as *mut frag_hdr;
    if (*fhdr).frag_off & htons(IP6_OFFSET | IP6_MF) == 0 { (*skb).transport_header += size_of::<frag_hdr>() as u16; __IP6_INC_STATS(net, idev, IPSTATS_MIB_REASMOKS); IP6CB(skb).nhoff = fhdr as *mut u8 as usize as u8; IP6CB(skb).flags |= IP6SKB_FRAGMENTED; IP6CB(skb).frag_max_size = ntohs((*hdr).payload_len) as u32 + size_of::<ipv6hdr>() as u32; return 1; }
    let iif = if !(*skb).dev.is_null() { (*(*skb).dev).ifindex } else { 0 }; rcu_read_lock(); let fq = fq_find(net, (*fhdr).identification, hdr, iif);
    if fq.is_null() { rcu_read_unlock(); __IP6_INC_STATS(net, idev, IPSTATS_MIB_REASMFAILS); kfree_skb(skb); return -1; }
    let mut prob_offset = 0; let mut refs = 0; spin_lock(&mut (*fq).q.lock); (*fq).iif = iif; let ret = ip6_frag_queue(net, fq, skb, fhdr, IP6CB(skb).nhoff as i32, &mut prob_offset, &mut refs, idev); spin_unlock(&mut (*fq).q.lock); rcu_read_unlock(); inet_frag_putn(&mut (*fq).q, refs); if prob_offset != 0 { __IP6_INC_STATS(net, idev, IPSTATS_MIB_INHDRERRORS); icmpv6_param_prob(skb, ICMPV6_HDR_FIELD, prob_offset); } ret
}

static mut frag_protocol: inet6_protocol = inet6_protocol { handler: ipv6_frag_rcv, flags: INET6_PROTO_NOPOLICY };

// CONFIG_SYSCTL-dependent declarations and registration are supplied by the kernel build.
unsafe fn ipv6_frag_init() -> i32 {
    (*(&mut ip6_frags)).constructor = Some(ip6frag_init);
    (*(&mut ip6_frags)).destructor = None;
    (*(&mut ip6_frags)).qsize = size_of::<frag_queue>();
    (*(&mut ip6_frags)).frag_expire = Some(ip6_frag_expire);
    (*(&mut ip6_frags)).frags_cache_name = IP6_FRAG_CACHE_NAME.as_ptr() as *const i8;
    let mut ret = inet_frags_init(&mut ip6_frags);
    if ret != 0 { return ret; }
    ret = inet6_add_protocol(&frag_protocol, IPPROTO_FRAGMENT);
    if ret != 0 { inet_frags_fini(&mut ip6_frags); return ret; }
    ret = ip6_frags_sysctl_register();
    if ret != 0 { inet6_del_protocol(&frag_protocol, IPPROTO_FRAGMENT); inet_frags_fini(&mut ip6_frags); }
    ret
}

unsafe fn ipv6_frag_exit() {
    ip6_frags_sysctl_unregister();
    unregister_pernet_subsys(&mut ip6_frags_ops);
    inet6_del_protocol(&frag_protocol, IPPROTO_FRAGMENT);
    inet_frags_fini(&mut ip6_frags);
}

unsafe fn ip6_frags_sysctl_register() -> i32 { 0 }
unsafe fn ip6_frags_sysctl_unregister() {}
unsafe fn ipv6_frags_init_net(_net: *mut net) -> i32 { 0 }
unsafe fn ipv6_frags_pre_exit_net(_net: *mut net) {}
unsafe fn ipv6_frags_exit_net(_net: *mut net) {}
static mut ip6_frags_ops: pernet_operations = pernet_operations { init: None, pre_exit: None, exit: None };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
