// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPv6 fragment reassembly for connection tracking
 *
 * Copyright (C)2004 USAGI/WIDE Project
 *
 * Author:
 *\tYasuyuki Kozakai @USAGI <yasuyuki.kozakai@toshiba.co.jp>
 *
 * Based on: net/ipv6/reassembly.c
 */

// C dependencies supplied by the surrounding kernel translation.

static NF_FRAGS_CACHE_NAME: &[u8] = b"nf-frags\0";

static mut nf_frag_pernet_id: c_uint = 0;
static mut nf_frags: inet_frags = unsafe { core::mem::zeroed() };

unsafe fn nf_frag_pernet(net: *mut net) -> *mut nft_ct_frag6_pernet {
    net_generic(net, nf_frag_pernet_id)
}

#[cfg(CONFIG_SYSCTL)]
static mut nf_ct_frag6_sysctl_table: [ctl_table; 3] = [
    ctl_table { procname: cstr!("nf_conntrack_frag6_timeout"), maxlen: core::mem::size_of::<c_uint>(), mode: 0o644, proc_handler: Some(proc_dointvec_jiffies), ..unsafe { core::mem::zeroed() } },
    ctl_table { procname: cstr!("nf_conntrack_frag6_low_thresh"), maxlen: core::mem::size_of::<c_ulong>(), mode: 0o644, proc_handler: Some(proc_doulongvec_minmax), ..unsafe { core::mem::zeroed() } },
    ctl_table { procname: cstr!("nf_conntrack_frag6_high_thresh"), maxlen: core::mem::size_of::<c_ulong>(), mode: 0o644, proc_handler: Some(proc_doulongvec_minmax), ..unsafe { core::mem::zeroed() } },
];

#[cfg(CONFIG_SYSCTL)]
unsafe fn nf_ct_frag6_sysctl_register(net: *mut net) -> c_int {
    let mut table = nf_ct_frag6_sysctl_table.as_mut_ptr();
    if !net_eq(net, &raw mut init_net) {
        table = kmemdup(table, core::mem::size_of_val(&nf_ct_frag6_sysctl_table), GFP_KERNEL) as *mut ctl_table;
        if table.is_null() { return -ENOMEM; }
    }
    let nf_frag = nf_frag_pernet(net);
    (*table.add(0)).data = &mut (*(*nf_frag).fqdir).timeout as *mut _ as *mut c_void;
    (*table.add(1)).data = &mut (*(*nf_frag).fqdir).low_thresh as *mut _ as *mut c_void;
    (*table.add(1)).extra2 = &mut (*(*nf_frag).fqdir).high_thresh as *mut _ as *mut c_void;
    (*table.add(2)).data = &mut (*(*nf_frag).fqdir).high_thresh as *mut _ as *mut c_void;
    (*table.add(2)).extra1 = &mut (*(*nf_frag).fqdir).low_thresh as *mut _ as *mut c_void;
    let hdr = register_net_sysctl_sz(net, cstr!("net/netfilter"), table, 3);
    if hdr.is_null() {
        if !net_eq(net, &raw mut init_net) { kfree(table as *mut c_void); }
        return -ENOMEM;
    }
    (*nf_frag).nf_frag_frags_hdr = hdr;
    0
}

#[cfg(CONFIG_SYSCTL)]
unsafe fn nf_ct_frags6_sysctl_unregister(net: *mut net) {
    let nf_frag = nf_frag_pernet(net);
    let table = (*(*nf_frag).nf_frag_frags_hdr).ctl_table_arg;
    unregister_net_sysctl_table((*nf_frag).nf_frag_frags_hdr);
    if !net_eq(net, &raw mut init_net) { kfree(table as *mut c_void); }
}

#[cfg(not(CONFIG_SYSCTL))]
unsafe fn nf_ct_frag6_sysctl_register(_: *mut net) -> c_int { 0 }
#[cfg(not(CONFIG_SYSCTL))]
unsafe fn nf_ct_frags6_sysctl_unregister(_: *mut net) {}

unsafe extern "C" fn nf_ct_frag6_reasm(fq: *mut frag_queue, skb: *mut sk_buff, prev_tail: *mut sk_buff, dev: *mut net_device, refs: *mut c_int) -> c_int;

#[inline]
unsafe fn ip6_frag_ecn(ipv6h: *const ipv6hdr) -> u8 {
    1u8 << (ipv6_get_dsfield(ipv6h) & INET_ECN_MASK)
}

unsafe extern "C" fn nf_ct_frag6_expire(t: *mut timer_list) {
    let frag = timer_container_of!(t, inet_frag_queue, timer);
    let fq = container_of!(frag, frag_queue, q);
    ip6frag_expire_frag_queue((*(*fq).q.fqdir).net, fq);
}

unsafe fn fq_find(net: *mut net, id: __be32, user: u32, hdr: *const ipv6hdr, iif: c_int) -> *mut frag_queue {
    let nf_frag = nf_frag_pernet(net);
    let mut key: frag_v6_compare_key = core::mem::zeroed();
    key.id = id; key.saddr = (*hdr).saddr; key.daddr = (*hdr).daddr; key.user = user; key.iif = iif;
    if ipv6_addr_type(&(*hdr).daddr) & (IPV6_ADDR_MULTICAST | IPV6_ADDR_LINKLOCAL) == 0 { key.iif = 0; }
    let q = inet_frag_find((*nf_frag).fqdir, &key as *const _ as *const c_void);
    if q.is_null() { core::ptr::null_mut() } else { container_of!(q, frag_queue, q) }
}

unsafe fn nf_ct_frag6_queue(fq: *mut frag_queue, skb: *mut sk_buff, fhdr: *const frag_hdr, nhoff: c_int, refs: *mut c_int) -> c_int {
    let payload_len = ntohs((*ipv6_hdr(skb)).payload_len) as c_uint;
    if (*fq).q.flags & INET_FRAG_COMPLETE != 0 { pr_debug!("Already completed\\n"); return -EINVAL; }
    let offset = (ntohs((*fhdr).frag_off) & !0x7) as c_int;
    let end = offset + payload_len as c_int - ((fhdr.add(1) as *const u8).offset_from((ipv6_hdr(skb).add(1) as *const ipv6hdr).cast::<u8>()) as c_int);
    if end as c_uint > IPV6_MAXPLEN { pr_debug!("offset is too large.\\n"); return -EINVAL; }
    let ecn = ip6_frag_ecn(ipv6_hdr(skb));
    if (*skb).ip_summed == CHECKSUM_COMPLETE { let nh = skb_network_header(skb); (*skb).csum = csum_sub((*skb).csum, csum_partial(nh, (fhdr.add(1) as *const u8).offset_from(nh) as usize, 0)); }
    if (*fhdr).frag_off & htons(IP6_MF) == 0 {
        if end < (*fq).q.len || ((*fq).q.flags & INET_FRAG_LAST_IN != 0 && end != (*fq).q.len) { pr_debug!("already received last fragment\\n"); return -EINVAL; }
        (*fq).q.flags |= INET_FRAG_LAST_IN; (*fq).q.len = end;
    } else {
        if end & 0x7 != 0 { pr_debug!("end of fragment not rounded to 8 bytes.\\n"); inet_frag_kill(&mut (*fq).q, refs); return -EPROTO; }
        if end > (*fq).q.len { if (*fq).q.flags & INET_FRAG_LAST_IN != 0 { pr_debug!("last packet already reached.\\n"); return -EINVAL; } (*fq).q.len = end; }
    }
    if end == offset { return -EINVAL; }
    if !pskb_pull(skb, (fhdr.add(1) as *mut u8).offset_from((*skb).data) as usize) { pr_debug!("queue: message is too short.\\n"); return -EINVAL; }
    if pskb_trim_rcsum(skb, (end - offset) as usize) != 0 { pr_debug!("Can't trim\\n"); return -EINVAL; }
    let dev = (*skb).dev; barrier!();
    let prev = (*fq).q.fragments_tail;
    let err = inet_frag_queue_insert(&mut (*fq).q, skb, offset, end);
    if err != 0 { if err == IPFRAG_DUP { kfree_skb_reason(skb, SKB_DROP_REASON_DUP_FRAG); return -EINPROGRESS; } inet_frag_kill(&mut (*fq).q, refs); (*skb).dst = core::ptr::null_mut(); return -EINVAL; }
    if !dev.is_null() { (*fq).iif = (*dev).ifindex; }
    (*fq).q.stamp = (*skb).tstamp; (*fq).q.tstamp_type = (*skb).tstamp_type; (*fq).q.meat += (*skb).len; (*fq).ecn |= ecn; if payload_len > (*fq).q.max_size { (*fq).q.max_size = payload_len; } add_frag_mem_limit((*fq).q.fqdir, (*skb).truesize);
    if offset == 0 { (*fq).nhoffset = nhoff as u8; (*fq).q.flags |= INET_FRAG_FIRST_IN; }
    if (*fq).q.flags == (INET_FRAG_FIRST_IN | INET_FRAG_LAST_IN) && (*fq).q.meat == (*fq).q.len { let old = (*skb)._skb_refdst; (*skb)._skb_refdst = 0; let e = nf_ct_frag6_reasm(fq, skb, prev, dev, refs); (*skb)._skb_refdst = old; return if e != 0 { -EINPROGRESS } else { 0 }; }
    skb_dst_drop(skb); skb_orphan(skb); -EINPROGRESS
}

unsafe fn nf_ct_frag6_reasm(fq: *mut frag_queue, skb: *mut sk_buff, prev_tail: *mut sk_buff, dev: *mut net_device, refs: *mut c_int) -> c_int {
    inet_frag_kill(&mut (*fq).q, refs);
    let ecn = ip_frag_ecn_table[(*fq).ecn as usize]; if ecn == 0xff { return -EINVAL; }
    let reasm_data = inet_frag_reasm_prepare(&mut (*fq).q, skb, prev_tail); if reasm_data.is_null() { return -EINVAL; }
    let payload_len = -skb_network_offset(skb) - core::mem::size_of::<ipv6hdr>() as c_int + (*fq).q.len - core::mem::size_of::<frag_hdr>() as c_int;
    if payload_len > IPV6_MAXPLEN as c_int { net_dbg_ratelimited!("nf_ct_frag6_reasm: payload len = %d\\n", payload_len); return -EINVAL; }
    (*skb_network_header(skb).add((*fq).nhoffset as usize)) = skb_transport_header(skb).read();
    memmove((*skb).head.add(core::mem::size_of::<frag_hdr>()), (*skb).head, ((*skb).data.offset_from((*skb).head) as usize) - core::mem::size_of::<frag_hdr>());
    if skb_mac_header_was_set(skb) { (*skb).mac_header += core::mem::size_of::<frag_hdr>() as i32; }
    (*skb).network_header += core::mem::size_of::<frag_hdr>() as i32; skb_reset_transport_header(skb);
    inet_frag_reasm_finish(&mut (*fq).q, skb, reasm_data, false); (*skb).ignore_df = 1; (*skb).dev = dev; (*ipv6_hdr(skb)).payload_len = htons(payload_len as u16); ipv6_change_dsfield(ipv6_hdr(skb), 0xff, ecn); (*IP6CB(skb)).frag_max_size = core::mem::size_of::<ipv6hdr>() as u32 + (*fq).q.max_size; (*IP6CB(skb)).flags |= IP6SKB_FRAGMENTED;
    if (*skb).ip_summed == CHECKSUM_COMPLETE { (*skb).csum = csum_partial(skb_network_header(skb), skb_network_header_len(skb), (*skb).csum); }
    (*fq).q.rb_fragments = RB_ROOT; (*fq).q.fragments_tail = core::ptr::null_mut(); (*fq).q.last_run_head = core::ptr::null_mut(); 0
}

unsafe fn find_prev_fhdr(skb: *mut sk_buff, prevhdrp: *mut u8, prevhoff: *mut c_int, fhoff: *mut c_int) -> c_int {
    let mut nexthdr = (*ipv6_hdr(skb)).nexthdr; let netoff = skb_network_offset(skb); let mut prev_nhoff = netoff + core::mem::offset_of!(ipv6hdr, nexthdr) as c_int; let mut start = netoff + core::mem::size_of::<ipv6hdr>() as c_int; let mut len = (*skb).len as c_int - start; let mut prevhdr = NEXTHDR_IPV6;
    while nexthdr != NEXTHDR_FRAGMENT { let mut hdr: ipv6_opt_hdr = core::mem::zeroed(); if !ipv6_ext_hdr(nexthdr) || nexthdr == NEXTHDR_NONE || len < core::mem::size_of::<ipv6_opt_hdr>() as c_int { return -1; } if skb_copy_bits(skb, start, &mut hdr as *mut _ as *mut c_void, core::mem::size_of::<ipv6_opt_hdr>()) != 0 { return -1; } let hdrlen = if nexthdr == NEXTHDR_AUTH { ipv6_authlen(&hdr) } else { ipv6_optlen(&hdr) }; prevhdr = nexthdr; prev_nhoff = start; nexthdr = hdr.nexthdr; len -= hdrlen; start += hdrlen; }
    if len < 0 { return -1; } *prevhdrp = prevhdr; *prevhoff = prev_nhoff; *fhoff = start; 0
}

#[no_mangle]
pub unsafe extern "C" fn nf_ct_frag6_gather(net: *mut net, skb: *mut sk_buff, user: u32) -> c_int {
    let savethdr = (*skb).transport_header; let mut nexthdr = NEXTHDR_FRAGMENT; let (mut fhoff, mut nhoff) = (0, 0); let mut fhdr; let mut fq; let mut refs = 0; let mut prevhdr = 0;
    if (*ipv6_hdr(skb)).payload_len == 0 { pr_debug!("payload len = 0\\n"); return 0; }
    if find_prev_fhdr(skb, &mut prevhdr, &mut nhoff, &mut fhoff) < 0 { return 0; }
    if ipv6frag_thdr_truncated(skb, fhoff, &mut nexthdr) { pr_debug!("Drop incomplete fragment\\n"); return 0; }
    if !pskb_may_pull(skb, fhoff as usize + core::mem::size_of::<frag_hdr>()) { return -ENOMEM; }
    skb_set_transport_header(skb, fhoff); fhdr = skb_transport_header(skb) as *mut frag_hdr;
    rcu_read_lock(); fq = fq_find(net, (*fhdr).identification, user, ipv6_hdr(skb), if !(*skb).dev.is_null() { (*(*skb).dev).ifindex } else { 0 }); if fq.is_null() { rcu_read_unlock(); return -ENOMEM; }
    spin_lock_bh(&mut (*fq).q.lock); let mut ret = nf_ct_frag6_queue(fq, skb, fhdr, nhoff, &mut refs); if ret == -EPROTO { (*skb).transport_header = savethdr; ret = 0; } spin_unlock_bh(&mut (*fq).q.lock); rcu_read_unlock(); inet_frag_putn(&mut (*fq).q, refs); ret
}

unsafe fn nf_ct_net_init(net: *mut net) -> c_int { let nf_frag = nf_frag_pernet(net); let mut res = fqdir_init(&mut (*nf_frag).fqdir, &mut nf_frags, net); if res < 0 { return res; } (*(*nf_frag).fqdir).high_thresh = IPV6_FRAG_HIGH_THRESH; (*(*nf_frag).fqdir).low_thresh = IPV6_FRAG_LOW_THRESH; (*(*nf_frag).fqdir).timeout = IPV6_FRAG_TIMEOUT; res = nf_ct_frag6_sysctl_register(net); if res < 0 { fqdir_exit((*nf_frag).fqdir); } res }
unsafe fn nf_ct_net_pre_exit(net: *mut net) { fqdir_pre_exit((*nf_frag_pernet(net)).fqdir); }
unsafe fn nf_ct_net_exit(net: *mut net) { let nf_frag = nf_frag_pernet(net); nf_ct_frags6_sysctl_unregister(net); fqdir_exit((*nf_frag).fqdir); }

static mut nf_ct_net_ops: pernet_operations = pernet_operations { init: Some(nf_ct_net_init), pre_exit: Some(nf_ct_net_pre_exit), exit: Some(nf_ct_net_exit), id: &raw mut nf_frag_pernet_id, size: core::mem::size_of::<nft_ct_frag6_pernet>() };
static mut nfct_rhash_params: rhashtable_params = rhashtable_params { head_offset: core::mem::offset_of!(inet_frag_queue, node), hashfn: Some(ip6frag_key_hashfn), obj_hashfn: Some(ip6frag_obj_hashfn), obj_cmpfn: Some(ip6frag_obj_cmpfn), automatic_shrinking: true, ..unsafe { core::mem::zeroed() } };

#[no_mangle]
pub unsafe extern "C" fn nf_ct_frag6_init() -> c_int { nf_frags.constructor = Some(ip6frag_init); nf_frags.destructor = None; nf_frags.qsize = core::mem::size_of::<frag_queue>(); nf_frags.frag_expire = Some(nf_ct_frag6_expire); nf_frags.frags_cache_name = NF_FRAGS_CACHE_NAME.as_ptr() as *const c_char; nf_frags.rhash_params = nfct_rhash_params; let mut ret = inet_frags_init(&mut nf_frags); if ret != 0 { return ret; } ret = register_pernet_subsys(&mut nf_ct_net_ops); if ret != 0 { inet_frags_fini(&mut nf_frags); } ret }

#[no_mangle]
pub unsafe extern "C" fn nf_ct_frag6_cleanup() { unregister_pernet_subsys(&mut nf_ct_net_ops); inet_frags_fini(&mut nf_frags); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
