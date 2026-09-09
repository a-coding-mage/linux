// SPDX-License-Identifier: GPL-2.0
/* Management Component Transport Protocol (MCTP) - routing implementation. */

// Linux/MCTP declarations are supplied by the surrounding translation unit.

static MCTP_MESSAGE_MAXLEN: ::std::primitive::usize = 64 * 1024;
static MCTP_KEY_LIFETIME: ::std::primitive::usize = 6 * CONFIG_HZ as usize;

unsafe extern "C" {
    fn mctp_flow_prepare_output(skb: *mut sk_buff, dev: *mut mctp_dev);
}

unsafe fn mctp_dst_discard(_dst: *mut mctp_dst, skb: *mut sk_buff) -> i32 {
    kfree_skb(skb); 0
}

unsafe fn mctp_lookup_bind_details(net: *mut net, skb: *mut sk_buff, typ: u8,
                                   dest: u8, src: u8, allow_net_any: bool) -> *mut mctp_sock {
    let cb = mctp_cb(skb); let mut sk: *mut sock;
    WARN_ON_ONCE(!rcu_read_lock_held());
    let hash = mctp_bind_hash(typ, dest, src);
    sk_for_each_rcu!(sk, (*net).mctp.binds[hash]);
    while !sk.is_null() {
        let msk = container_of!(sk, mctp_sock, sk);
        if !allow_net_any && (*msk).bind_net == MCTP_NET_ANY { sk = sk_next_rcu(sk); continue; }
        if (*msk).bind_net != MCTP_NET_ANY && (*msk).bind_net != (*cb).net { sk = sk_next_rcu(sk); continue; }
        if (*msk).bind_type != typ { sk = sk_next_rcu(sk); continue; }
        if (*msk).bind_peer_set && !mctp_address_matches((*msk).bind_peer_addr, src) { sk = sk_next_rcu(sk); continue; }
        if !mctp_address_matches((*msk).bind_local_addr, dest) { sk = sk_next_rcu(sk); continue; }
        return msk;
    }
    core::ptr::null_mut()
}

unsafe fn mctp_lookup_bind(net: *mut net, skb: *mut sk_buff) -> *mut mctp_sock {
    let mh = mctp_hdr(skb); if skb_headlen(skb) == 0 { return core::ptr::null_mut(); }
    let typ = (*(*(skb)).data) & 0x7f;
    let checks = [( (*mh).dest, (*mh).src, false), (MCTP_ADDR_ANY, (*mh).src, false),
                  ((*mh).dest, MCTP_ADDR_ANY, false), (MCTP_ADDR_ANY, MCTP_ADDR_ANY, false),
                  (MCTP_ADDR_ANY, MCTP_ADDR_ANY, true)];
    for (d, s, any) in checks { let m = mctp_lookup_bind_details(net, skb, typ, d, s, any); if !m.is_null() { return m; } }
    core::ptr::null_mut()
}

unsafe fn mctp_key_match(key: *mut mctp_sk_key, net: u32, local: mctp_eid_t, peer: mctp_eid_t, tag: u8) -> bool {
    (*key).net == net && mctp_address_matches((*key).local_addr, local) &&
        mctp_address_matches((*key).peer_addr, peer) && (*key).tag == tag
}

unsafe fn mctp_lookup_key(net: *mut net, skb: *mut sk_buff, netid: u32, peer: mctp_eid_t,
                          irqflags: *mut usize) -> *mut mctp_sk_key {
    let mh = mctp_hdr(skb); let tag = (*mh).flags_seq_tag & (MCTP_HDR_TAG_MASK | MCTP_HDR_FLAG_TO);
    let mut ret = core::ptr::null_mut(); let mut flags = 0usize;
    spin_lock_irqsave!(&mut (*net).mctp.keys_lock, flags);
    hlist_for_each_entry!(key, (*net).mctp.keys, hlist) {
        if !mctp_key_match(key, netid, (*mh).dest, peer, tag) { continue; }
        spin_lock!(&mut (*key).lock);
        if (*key).valid { refcount_inc!(&mut (*key).refs); ret = key; break; }
        spin_unlock!(&mut (*key).lock);
    }
    if !ret.is_null() { spin_unlock!(&mut (*net).mctp.keys_lock); *irqflags = flags; }
    else { spin_unlock_irqrestore!(&mut (*net).mctp.keys_lock, flags); }
    ret
}

unsafe fn mctp_key_alloc(msk: *mut mctp_sock, net: u32, local: mctp_eid_t, peer: mctp_eid_t,
                         tag: u8, gfp: gfp_t) -> *mut mctp_sk_key {
    let key = kzalloc_obj!(mctp_sk_key, gfp); if key.is_null() { return core::ptr::null_mut(); }
    (*key).net = net; (*key).peer_addr = peer; (*key).local_addr = local; (*key).tag = tag;
    (*key).sk = &mut (*msk).sk; (*key).valid = true; spin_lock_init!(&mut (*key).lock);
    refcount_set!(&mut (*key).refs, 1); sock_hold((*key).sk); key
}

pub unsafe fn mctp_key_unref(key: *mut mctp_sk_key) {
    if !refcount_dec_and_test!(&mut (*key).refs) { return; }
    let mut flags = 0usize; spin_lock_irqsave!(&mut (*key).lock, flags);
    mctp_dev_release_key((*key).dev, key); spin_unlock_irqrestore!(&mut (*key).lock, flags);
    sock_put((*key).sk); kfree(key);
}

unsafe fn mctp_key_add(key: *mut mctp_sk_key, msk: *mut mctp_sock) -> i32 {
    let net = sock_net(&mut (*msk).sk); let mut flags = 0usize; let mut rc = 0;
    spin_lock_irqsave!(&mut (*net).mctp.keys_lock, flags);
    if sock_flag(&mut (*msk).sk, SOCK_DEAD) { rc = -EINVAL; }
    else { hlist_for_each_entry!(tmp, (*net).mctp.keys, hlist) {
        if mctp_key_match(tmp, (*key).net, (*key).local_addr, (*key).peer_addr, (*key).tag) {
            spin_lock!(&mut (*tmp).lock); if (*tmp).valid { rc = -EEXIST; } spin_unlock!(&mut (*tmp).lock); if rc != 0 { break; }
        }
    } }
    if rc == 0 { refcount_inc!(&mut (*key).refs); (*key).expiry = jiffies() + MCTP_KEY_LIFETIME; timer_reduce!(&mut (*msk).key_expiry, (*key).expiry); hlist_add_head!(&mut (*key).hlist, &mut (*net).mctp.keys); hlist_add_head!(&mut (*key).sklist, &mut (*msk).keys); }
    spin_unlock_irqrestore!(&mut (*net).mctp.keys_lock, flags); rc
}

unsafe fn __mctp_key_done_in(key: *mut mctp_sk_key, net: *mut net, mut flags: usize, reason: usize) {
    trace_mctp_key_release(key, reason); let skb = (*key).reasm_head; (*key).reasm_head = core::ptr::null_mut();
    if !(*key).manual_alloc { (*key).reasm_dead = true; (*key).valid = false; mctp_dev_release_key((*key).dev, key); }
    spin_unlock_irqrestore!(&mut (*key).lock, flags);
    if !(*key).manual_alloc { spin_lock_irqsave!(&mut (*net).mctp.keys_lock, flags); if !hlist_unhashed!(&(*key).hlist) { hlist_del_init!(&mut (*key).hlist); hlist_del_init!(&mut (*key).sklist); mctp_key_unref(key); } spin_unlock_irqrestore!(&mut (*net).mctp.keys_lock, flags); }
    mctp_key_unref(key); kfree_skb(skb);
}

#[cfg(feature = "CONFIG_MCTP_FLOWS")]
unsafe fn mctp_skb_set_flow(skb: *mut sk_buff, key: *mut mctp_sk_key) { let flow = skb_ext_add(skb, SKB_EXT_MCTP); if flow.is_null() { return; } refcount_inc!(&mut (*key).refs); (*flow).key = key; }
#[cfg(not(feature = "CONFIG_MCTP_FLOWS"))] unsafe fn mctp_skb_set_flow(_skb: *mut sk_buff, _key: *mut mctp_sk_key) {}

unsafe fn mctp_frag_queue(key: *mut mctp_sk_key, skb: *mut sk_buff) -> i32 {
    let hdr = mctp_hdr(skb); let this_seq = ((*hdr).flags_seq_tag >> MCTP_HDR_SEQ_SHIFT) & MCTP_HDR_SEQ_MASK;
    if (*key).reasm_head.is_null() { (*key).reasm_head = skb_unshare(skb, GFP_ATOMIC); if (*key).reasm_head.is_null() { return -ENOMEM; } (*key).reasm_tailp = &mut (*skb_shinfo((*key).reasm_head)).frag_list; (*key).last_seq = this_seq; return 0; }
    let exp_seq = ((*key).last_seq + 1) & MCTP_HDR_SEQ_MASK; if this_seq != exp_seq || (*key).reasm_head.len + (*skb).len > MCTP_MESSAGE_MAXLEN { kfree_skb(skb); return -EINVAL; }
    (*skb).next = core::ptr::null_mut(); (*skb).sk = core::ptr::null_mut(); *(*key).reasm_tailp = skb; (*key).reasm_tailp = &mut (*skb).next; (*key).last_seq = this_seq;
    (*key).reasm_head.data_len += (*skb).len; (*key).reasm_head.len += (*skb).len; (*key).reasm_head.truesize += (*skb).truesize; 0
}

// The remaining route operations retain the C implementation's external kernel calls and layout.
pub unsafe fn mctp_default_net(net: *mut net) -> u32 { READ_ONCE!((*net).mctp.default_net) }
pub unsafe fn mctp_default_net_set(net: *mut net, index: u32) -> i32 { if index == 0 { return -EINVAL; } WRITE_ONCE!((*net).mctp.default_net, index); 0 }

unsafe fn mctp_dst_input(dst: *mut mctp_dst, mut skb: *mut sk_buff) -> i32 {
    let net = dev_net((*skb).dev); skb_orphan(skb);
    if (*skb).pkt_type == PACKET_OUTGOING { (*skb).pkt_type = PACKET_LOOPBACK; }
    if (*skb).len < core::mem::size_of::<mctp_hdr>() + 1 { kfree_skb(skb); return -EINVAL; }
    let mh = mctp_hdr(skb); let netid = mctp_cb(skb).net; skb_pull(skb, core::mem::size_of::<mctp_hdr>());
    let ver = (*mh).ver & MCTP_HDR_VER_MASK; if ver < MCTP_VER_MIN || ver > MCTP_VER_MAX { kfree_skb(skb); return -EINVAL; }
    let flags = (*mh).flags_seq_tag & (MCTP_HDR_FLAG_SOM | MCTP_HDR_FLAG_EOM);
    let tag = (*mh).flags_seq_tag & (MCTP_HDR_TAG_MASK | MCTP_HDR_FLAG_TO); let mut irq = 0usize;
    rcu_read_lock(); let mut key = mctp_lookup_key(net, skb, netid, (*mh).src, &mut irq);
    let mut msk = core::ptr::null_mut();
    if flags & MCTP_HDR_FLAG_SOM != 0 { if !key.is_null() { msk = container_of!((*key).sk, mctp_sock, sk); } else if tag & MCTP_HDR_FLAG_TO != 0 { msk = mctp_lookup_bind(net, skb); }
        if msk.is_null() { rcu_read_unlock(); kfree_skb(skb); return -ENOENT; }
        if flags & MCTP_HDR_FLAG_EOM != 0 { let rc = sock_queue_rcv_skb(&mut (*msk).sk, skb); if !key.is_null() { __mctp_key_done_in(key, net, irq, MCTP_TRACE_KEY_REPLIED); } rcu_read_unlock(); if rc != 0 { kfree_skb(skb); } return rc; }
        if key.is_null() { key = mctp_key_alloc(msk, netid, (*mh).dest, (*mh).src, tag, GFP_ATOMIC); if key.is_null() { rcu_read_unlock(); kfree_skb(skb); return -ENOMEM; } mctp_frag_queue(key, skb); let rc = mctp_key_add(key, msk); if rc == 0 { trace_mctp_key_acquire(key); } mctp_key_unref(key); key = core::ptr::null_mut(); }
        else { if !(*key).reasm_head.is_null() || (*key).reasm_dead { __mctp_key_done_in(key, net, irq, MCTP_TRACE_KEY_INVALIDATED); key = core::ptr::null_mut(); } else { mctp_frag_queue(key, skb); } }
    } else if !key.is_null() { if (*key).reasm_head.is_null() { rcu_read_unlock(); spin_unlock_irqrestore!(&mut (*key).lock, irq); mctp_key_unref(key); kfree_skb(skb); return -EINVAL; } mctp_frag_queue(key, skb); if flags & MCTP_HDR_FLAG_EOM != 0 { let rc = sock_queue_rcv_skb((*key).sk, (*key).reasm_head); if rc == 0 { (*key).reasm_head = core::ptr::null_mut(); } __mctp_key_done_in(key, net, irq, MCTP_TRACE_KEY_REPLIED); key = core::ptr::null_mut(); } }
    else { rcu_read_unlock(); kfree_skb(skb); return -ENOENT; }
    rcu_read_unlock(); if !key.is_null() { spin_unlock_irqrestore!(&mut (*key).lock, irq); mctp_key_unref(key); } kfree_skb(skb); 0
}

unsafe fn mctp_dst_output(dst: *mut mctp_dst, skb: *mut sk_buff) -> i32 {
    (*skb).protocol = htons(ETH_P_MCTP); (*skb).pkt_type = PACKET_OUTGOING; (*skb).dev = (*dst).dev.dev;
    if (*skb).len > (*dst).mtu { kfree_skb(skb); return -EMSGSIZE; }
    let rc = dev_hard_header(skb, (*skb).dev, ntohs((*skb).protocol), core::ptr::null(), (*skb).dev.dev_addr, (*skb).len);
    if rc < 0 { kfree_skb(skb); return -EHOSTUNREACH; }
    mctp_flow_prepare_output(skb, (*dst).dev); let rc = dev_queue_xmit(skb); if rc != 0 { net_xmit_errno(rc) } else { 0 }
}

unsafe fn mctp_route_release(rt: *mut mctp_route) { if refcount_dec_and_test!(&mut (*rt).refs) { if (*rt).dst_type == MCTP_ROUTE_DIRECT { mctp_dev_put((*rt).dev); } kfree_rcu(rt, rcu); } }
unsafe fn mctp_route_alloc() -> *mut mctp_route { let rt = kzalloc_obj!(mctp_route); if rt.is_null() { return rt; } INIT_LIST_HEAD!(&mut (*rt).list); refcount_set!(&mut (*rt).refs, 1); (*rt).output = Some(mctp_dst_discard); rt }

unsafe fn mctp_reserve_tag(net: *mut net, key: *mut mctp_sk_key, msk: *mut mctp_sock) { let mns = &mut (*net).mctp; lockdep_assert_held!(&mns.keys_lock); (*key).expiry = jiffies() + MCTP_KEY_LIFETIME; timer_reduce!(&mut (*msk).key_expiry, (*key).expiry); hlist_add_head_rcu!(&mut (*key).hlist, &mut mns.keys); hlist_add_head_rcu!(&mut (*key).sklist, &mut (*msk).keys); refcount_inc!(&mut (*key).refs); }

pub unsafe fn mctp_alloc_local_tag(msk: *mut mctp_sock, netid: u32, local: mctp_eid_t, mut peer: mctp_eid_t, manual: bool, tagp: *mut u8) -> *mut mctp_sk_key {
    if peer == MCTP_ADDR_NULL { peer = MCTP_ADDR_ANY; } let net = sock_net(&mut (*msk).sk); let key = mctp_key_alloc(msk, netid, local, peer, 0, GFP_KERNEL); if key.is_null() { return ERR_PTR!(-ENOMEM); }
    let mut tagbits: u8 = 0xff; let mut flags = 0usize; spin_lock_irqsave!(&mut (*net).mctp.keys_lock, flags);
    hlist_for_each_entry!(tmp, (*net).mctp.keys, hlist) { if (*tmp).net != netid || (*tmp).tag & MCTP_HDR_FLAG_TO != 0 { continue; } if peer != MCTP_ADDR_ANY && !mctp_address_matches((*tmp).peer_addr, peer) { continue; } if local != MCTP_ADDR_ANY && !mctp_address_matches((*tmp).local_addr, local) { continue; } spin_lock!(&mut (*tmp).lock); if (*tmp).valid { tagbits &= !(1u8 << (*tmp).tag); } spin_unlock!(&mut (*tmp).lock); if tagbits == 0 { break; } }
    if tagbits != 0 { (*key).tag = tagbits.trailing_zeros() as u8; mctp_reserve_tag(net, key, msk); trace_mctp_key_acquire(key); (*key).manual_alloc = manual; *tagp = (*key).tag; } spin_unlock_irqrestore!(&mut (*net).mctp.keys_lock, flags); if tagbits == 0 { mctp_key_unref(key); return ERR_PTR!(-EBUSY); } key
}

unsafe fn mctp_route_netid(rt: *mut mctp_route) -> u32 { if (*rt).dst_type == MCTP_ROUTE_DIRECT { READ_ONCE!((*rt).dev.net) } else { (*rt).gateway.net } }
unsafe fn mctp_rt_match_eid(rt: *mut mctp_route, net: u32, eid: mctp_eid_t) -> bool { mctp_route_netid(rt) == net && (*rt).min <= eid && (*rt).max >= eid }
unsafe fn mctp_rt_compare_exact(a: *mut mctp_route, b: *mut mctp_route) -> bool { ASSERT_RTNL!(); mctp_route_netid(a) == mctp_route_netid(b) && (*a).min == (*b).min && (*a).max == (*b).max }

pub unsafe fn mctp_dst_release(dst: *mut mctp_dst) { mctp_dev_put((*dst).dev); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
