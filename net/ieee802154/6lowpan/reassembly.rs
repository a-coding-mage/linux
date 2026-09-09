// SPDX-License-Identifier: GPL-2.0-or-later
/* 6LoWPAN fragment reassembly */

const LOWPAN_FRAGS_CACHE_NAME: &[u8] = b"lowpan-frags\0";

static mut LOWPAN_FRAGS: inet_frags = unsafe { core::mem::zeroed() };

unsafe fn lowpan_frag_reasm(
    fq: *mut lowpan_frag_queue,
    skb: *mut sk_buff,
    prev: *mut sk_buff,
    ldev: *mut net_device,
    refs: *mut i32,
) -> i32;

unsafe fn lowpan_frag_init(q: *mut inet_frag_queue, a: *const core::ffi::c_void) {
    let key = a as *const frag_lowpan_compare_key;
    BUILD_BUG_ON!(core::mem::size_of::<frag_lowpan_compare_key>() > core::mem::size_of::<inet_frag_queue_key>());
    core::ptr::copy_nonoverlapping(
        key,
        &mut (*q).key as *mut _ as *mut frag_lowpan_compare_key,
        1,
    );
}

unsafe fn lowpan_frag_expire(t: *mut timer_list) {
    let frag = timer_container_of!(t, timer);
    let fq = container_of!(frag, frag_queue, q);
    let mut refs: i32 = 1;

    spin_lock(&mut (*fq).q.lock);
    if (*fq).q.flags & INET_FRAG_COMPLETE != 0 {
        spin_unlock(&mut (*fq).q.lock);
        inet_frag_putn(&mut (*fq).q, refs);
        return;
    }
    inet_frag_kill(&mut (*fq).q, &mut refs);
    spin_unlock(&mut (*fq).q.lock);
    inet_frag_putn(&mut (*fq).q, refs);
}

unsafe fn fq_find(
    net: *mut net,
    cb: *const lowpan_802154_cb,
    src: *const ieee802154_addr,
    dst: *const ieee802154_addr,
) -> *mut lowpan_frag_queue {
    let lowpan = net_ieee802154_lowpan(net);
    let mut key: frag_lowpan_compare_key = core::mem::zeroed();
    (*key).tag = (*cb).d_tag;
    (*key).d_size = (*cb).d_size;
    (*key).src = *src;
    (*key).dst = *dst;
    let q = inet_frag_find((*lowpan).fqdir, &key);
    if q.is_null() { return core::ptr::null_mut(); }
    container_of!(q, lowpan_frag_queue, q)
}

unsafe fn lowpan_frag_queue(
    fq: *mut lowpan_frag_queue, skb: *mut sk_buff, frag_type: u8, refs: *mut i32,
) -> i32 {
    let prev_tail;
    let ldev;
    let offset: i32;
    let end: i32;
    let err: i32;

    BUILD_BUG_ON!(core::mem::size_of::<lowpan_802154_cb>() > core::mem::size_of::<inet_skb_parm>());
    BUILD_BUG_ON!(core::mem::size_of::<lowpan_802154_cb>() > core::mem::size_of::<inet6_skb_parm>());
    if (*fq).q.flags & INET_FRAG_COMPLETE != 0 { kfree_skb(skb); return -1; }
    offset = (lowpan_802154_cb(skb).d_offset as i32) << 3;
    end = lowpan_802154_cb(skb).d_size as i32;
    if offset + (*skb).len as i32 == end {
        if end < (*fq).q.len || ((*fq).q.flags & INET_FRAG_LAST_IN != 0 && end != (*fq).q.len) { kfree_skb(skb); return -1; }
        (*fq).q.flags |= INET_FRAG_LAST_IN;
        (*fq).q.len = end;
    } else if end > (*fq).q.len {
        if (*fq).q.flags & INET_FRAG_LAST_IN != 0 { kfree_skb(skb); return -1; }
        (*fq).q.len = end;
    }
    ldev = (*skb).dev;
    if !ldev.is_null() { (*skb).dev = core::ptr::null_mut(); }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    prev_tail = (*fq).q.fragments_tail;
    err = inet_frag_queue_insert(&mut (*fq).q, skb, offset, end);
    if err != 0 { kfree_skb(skb); return -1; }
    (*fq).q.stamp = (*skb).tstamp;
    (*fq).q.tstamp_type = (*skb).tstamp_type;
    if frag_type == LOWPAN_DISPATCH_FRAG1 { (*fq).q.flags |= INET_FRAG_FIRST_IN; }
    (*fq).q.meat += (*skb).len;
    add_frag_mem_limit((*fq).q.fqdir, (*skb).truesize);
    if (*fq).q.flags == (INET_FRAG_FIRST_IN | INET_FRAG_LAST_IN) && (*fq).q.meat == (*fq).q.len {
        let orefdst = (*skb)._skb_refdst;
        (*skb)._skb_refdst = 0;
        let res = lowpan_frag_reasm(fq, skb, prev_tail, ldev, refs);
        (*skb)._skb_refdst = orefdst;
        return res;
    }
    skb_dst_drop(skb);
    return -1;
}

unsafe fn lowpan_frag_reasm(fq: *mut lowpan_frag_queue, skb: *mut sk_buff, prev_tail: *mut sk_buff, ldev: *mut net_device, refs: *mut i32) -> i32 {
    inet_frag_kill(&mut (*fq).q, refs);
    let data = inet_frag_reasm_prepare(&mut (*fq).q, skb, prev_tail);
    if data.is_null() { return -1; }
    inet_frag_reasm_finish(&mut (*fq).q, skb, data, false);
    (*skb).dev = ldev;
    (*skb).tstamp = (*fq).q.stamp;
    (*fq).q.rb_fragments = RB_ROOT;
    (*fq).q.fragments_tail = core::ptr::null_mut();
    (*fq).q.last_run_head = core::ptr::null_mut();
    1
}

unsafe fn lowpan_frag_rx_handlers_result(skb: *mut sk_buff, res: lowpan_rx_result) -> i32 {
    match res {
        RX_QUEUED => NET_RX_SUCCESS,
        RX_CONTINUE => { net_warn_ratelimited!("%s: received unknown dispatch\n", "lowpan_frag_rx_handlers_result"); NET_RX_DROP },
        _ => NET_RX_DROP,
    }
}

unsafe fn lowpan_frag_rx_h_iphc(skb: *mut sk_buff) -> lowpan_rx_result {
    if !lowpan_is_iphc(*skb_network_header(skb)) { return RX_CONTINUE; }
    if lowpan_iphc_decompress(skb) < 0 { return RX_DROP; }
    RX_QUEUED
}

unsafe fn lowpan_invoke_frag_rx_handlers(skb: *mut sk_buff) -> i32 {
    let mut res = lowpan_frag_rx_h_iphc(skb);
    if res == RX_CONTINUE { res = lowpan_rx_h_ipv6(skb); }
    lowpan_frag_rx_handlers_result(skb, res)
}

const LOWPAN_FRAG_DGRAM_SIZE_HIGH_MASK: u8 = 0x07;
const LOWPAN_FRAG_DGRAM_SIZE_HIGH_SHIFT: u32 = 8;

unsafe fn lowpan_get_cb(skb: *mut sk_buff, frag_type: u8, cb: *mut lowpan_802154_cb) -> i32 {
    let mut fail = lowpan_fetch_skb(skb, &mut 0u8, 1);
    let mut high = 0u8; let mut low = 0u8; let mut d_tag: u16 = 0;
    fail |= lowpan_fetch_skb(skb, &mut high, 1);
    fail |= lowpan_fetch_skb(skb, &mut low, 1);
    (*cb).d_size = (((high & LOWPAN_FRAG_DGRAM_SIZE_HIGH_MASK) as u16) << LOWPAN_FRAG_DGRAM_SIZE_HIGH_SHIFT) | low as u16;
    fail |= lowpan_fetch_skb(skb, &mut d_tag, 2);
    (*cb).d_tag = u16::from_be(d_tag);
    if frag_type == LOWPAN_DISPATCH_FRAGN { fail |= lowpan_fetch_skb(skb, &mut (*cb).d_offset, 1); }
    else { skb_reset_network_header(skb); (*cb).d_offset = 0; fail |= ((*cb).d_size < core::mem::size_of::<ipv6hdr>() as u16) as i32; fail |= ((*skb).len == 0) as i32; }
    if fail != 0 { -EIO } else { 0 }
}

pub unsafe fn lowpan_frag_rcv(skb: *mut sk_buff, frag_type: u8) -> i32 {
    let net = dev_net((*skb).dev); let cb = lowpan_802154_cb(skb); let mut hdr = core::mem::zeroed();
    if ieee802154_hdr_peek_addrs(skb, &mut hdr) < 0 || lowpan_get_cb(skb, frag_type, cb) < 0 { kfree_skb(skb); return -1; }
    if frag_type == LOWPAN_DISPATCH_FRAG1 && lowpan_invoke_frag_rx_handlers(skb) == NET_RX_DROP { kfree_skb(skb); return -1; }
    if (*cb).d_size > IPV6_MIN_MTU { net_warn_ratelimited!("lowpan_frag_rcv: datagram size exceeds MTU\n"); kfree_skb(skb); return -1; }
    rcu_read_lock(); let fq = fq_find(net, cb, &hdr.source, &hdr.dest);
    if !fq.is_null() { let mut refs = 0; spin_lock(&mut (*fq).q.lock); let ret = lowpan_frag_queue(fq, skb, frag_type, &mut refs); spin_unlock(&mut (*fq).q.lock); rcu_read_unlock(); inet_frag_putn(&mut (*fq).q, refs); return ret; }
    rcu_read_unlock(); kfree_skb(skb); -1
}

pub unsafe fn lowpan_net_frag_init() -> i32 {
    LOWPAN_FRAGS.qsize = core::mem::size_of::<frag_queue>();
    LOWPAN_FRAGS.frags_cache_name = LOWPAN_FRAGS_CACHE_NAME.as_ptr() as *const i8;
    LOWPAN_FRAGS.constructor = Some(lowpan_frag_init);
    LOWPAN_FRAGS.frag_expire = Some(lowpan_frag_expire);
    inet_frags_init(&mut LOWPAN_FRAGS)
}

pub unsafe fn lowpan_net_frag_exit() {
    inet_frags_fini(&mut LOWPAN_FRAGS);
}

// CONFIG_SYSCTL: the following declarations correspond to the kernel's
// per-network sysctl registration hooks.  Their concrete ctl_table types and
// registration functions are supplied by the surrounding kernel bindings.
#[cfg(feature = "CONFIG_SYSCTL")]
unsafe fn lowpan_frags_ns_sysctl_register(net: *mut net) -> i32 {
    let lowpan = net_ieee802154_lowpan(net);
    (*lowpan).sysctl.frags_hdr = register_net_sysctl(net, "net/ieee802154/6lowpan", core::ptr::null_mut());
    if (*lowpan).sysctl.frags_hdr.is_null() { -ENOMEM } else { 0 }
}

#[cfg(not(feature = "CONFIG_SYSCTL"))]
unsafe fn lowpan_frags_ns_sysctl_register(_net: *mut net) -> i32 { 0 }

#[cfg(feature = "CONFIG_SYSCTL")]
unsafe fn lowpan_frags_ns_sysctl_unregister(net: *mut net) {
    let lowpan = net_ieee802154_lowpan(net);
    unregister_net_sysctl_table((*lowpan).sysctl.frags_hdr);
}

#[cfg(not(feature = "CONFIG_SYSCTL"))]
unsafe fn lowpan_frags_ns_sysctl_unregister(_net: *mut net) {}

unsafe fn lowpan_frags_init_net(net: *mut net) -> i32 {
    let lowpan = net_ieee802154_lowpan(net);
    let ret = fqdir_init(&mut (*lowpan).fqdir, &mut LOWPAN_FRAGS, net);
    if ret < 0 { return ret; }
    (*(*lowpan).fqdir).high_thresh = IPV6_FRAG_HIGH_THRESH;
    (*(*lowpan).fqdir).low_thresh = IPV6_FRAG_LOW_THRESH;
    (*(*lowpan).fqdir).timeout = IPV6_FRAG_TIMEOUT;
    let ret = lowpan_frags_ns_sysctl_register(net);
    if ret < 0 { fqdir_exit((*lowpan).fqdir); }
    ret
}

unsafe fn lowpan_frags_pre_exit_net(net: *mut net) {
    fqdir_pre_exit((*net_ieee802154_lowpan(net)).fqdir);
}

unsafe fn lowpan_frags_exit_net(net: *mut net) {
    let lowpan = net_ieee802154_lowpan(net);
    lowpan_frags_ns_sysctl_unregister(net);
    fqdir_exit((*lowpan).fqdir);
}

unsafe fn lowpan_key_hashfn(data: *const core::ffi::c_void, _len: u32, seed: u32) -> u32 {
    jhash2(data as *const u32, core::mem::size_of::<frag_lowpan_compare_key>() / core::mem::size_of::<u32>(), seed)
}

unsafe fn lowpan_obj_hashfn(data: *const core::ffi::c_void, _len: u32, seed: u32) -> u32 {
    let fq = data as *const inet_frag_queue;
    lowpan_key_hashfn(&(*fq).key as *const _ as *const _, 0, seed)
}

unsafe fn lowpan_obj_cmpfn(arg: *const rhashtable_compare_arg, ptr: *const core::ffi::c_void) -> i32 {
    let key = (*arg).key as *const frag_lowpan_compare_key;
    let fq = ptr as *const inet_frag_queue;
    (memcmp(&(*fq).key as *const _ as *const _, key as *const _, core::mem::size_of::<frag_lowpan_compare_key>()) != 0) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
