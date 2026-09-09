/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const IP6_DEFRAG_LOCAL_DELIVER: u32 = 0;
pub const IP6_DEFRAG_CONNTRACK_IN: u32 = 1;
pub const __IP6_DEFRAG_CONNTRACK_IN: u32 = IP6_DEFRAG_CONNTRACK_IN + u16::MAX as u32;
pub const IP6_DEFRAG_CONNTRACK_OUT: u32 = __IP6_DEFRAG_CONNTRACK_IN + 1;
pub const __IP6_DEFRAG_CONNTRACK_OUT: u32 = IP6_DEFRAG_CONNTRACK_OUT + u16::MAX as u32;
pub const IP6_DEFRAG_CONNTRACK_BRIDGE_IN: u32 = __IP6_DEFRAG_CONNTRACK_OUT + 1;
pub const __IP6_DEFRAG_CONNTRACK_BRIDGE_IN: u32 =
    IP6_DEFRAG_CONNTRACK_BRIDGE_IN + u16::MAX as u32;

/*
 *	Equivalent of ipv4 struct ip
 */
#[repr(C)]
pub struct frag_queue {
    pub q: inet_frag_queue,
    pub iif: i32,
    pub nhoffset: u16,
    pub ecn: u8,
}

#[inline]
pub unsafe fn ip6frag_init(q: *mut inet_frag_queue, a: *const core::ffi::c_void) {
    let fq = container_of!(q, frag_queue, q);
    let key = a as *const frag_v6_compare_key;

    (*q).key.v6 = *key;
    (*fq).ecn = 0;
}

#[inline]
pub unsafe fn ip6frag_key_hashfn(
    data: *const core::ffi::c_void,
    _len: u32,
    seed: u32,
) -> u32 {
    jhash2(
        data,
        core::mem::size_of::<frag_v6_compare_key>() / core::mem::size_of::<u32>(),
        seed,
    )
}

#[inline]
pub unsafe fn ip6frag_obj_hashfn(
    data: *const core::ffi::c_void,
    _len: u32,
    seed: u32,
) -> u32 {
    let fq = data as *const inet_frag_queue;

    jhash2(
        &(*fq).key.v6 as *const _ as *const u32,
        core::mem::size_of::<frag_v6_compare_key>() / core::mem::size_of::<u32>(),
        seed,
    )
}

#[inline]
pub unsafe fn ip6frag_obj_cmpfn(
    arg: *mut rhashtable_compare_arg,
    ptr: *const core::ffi::c_void,
) -> i32 {
    let key = (*arg).key as *const frag_v6_compare_key;
    let fq = ptr as *const inet_frag_queue;

    if memcmp(
        &(*fq).key as *const _ as *const core::ffi::c_void,
        key as *const core::ffi::c_void,
        core::mem::size_of_val(&*key),
    ) != 0 {
        1
    } else {
        0
    }
}

#[inline]
pub unsafe fn ip6frag_expire_frag_queue(net: *mut net, fq: *mut frag_queue) {
    let mut dev: *mut net_device = core::ptr::null_mut();
    let head: *mut sk_buff;
    let mut refs: i32 = 1;

    macro_rules! goto_out {
        () => {{
            spin_unlock(&mut (*fq).q.lock);
            rcu_read_unlock();
            inet_frag_putn(&mut (*fq).q, refs);
            return;
        }}
    }

    rcu_read_lock();
    spin_lock(&mut (*fq).q.lock);

    if (*fq).q.flags & INET_FRAG_COMPLETE != 0 {
        goto_out!();
    }

    (*fq).q.flags |= INET_FRAG_DROP;
    inet_frag_kill(&mut (*fq).q, &mut refs);

    /* Paired with the WRITE_ONCE() in fqdir_pre_exit(). */
    if read_once((*fq).q.fqdir.as_ref().unwrap().dead) {
        inet_frag_queue_flush(&mut (*fq).q, 0);
        goto_out!();
    }

    dev = dev_get_by_index_rcu(net, (*fq).iif);
    if dev.is_null() {
        goto_out!();
    }

    __IP6_INC_STATS(net, __in6_dev_get(dev), IPSTATS_MIB_REASMFAILS);
    __IP6_INC_STATS(net, __in6_dev_get(dev), IPSTATS_MIB_REASMTIMEOUT);

    /* Don't send error if the first segment did not arrive. */
    if (*fq).q.flags & INET_FRAG_FIRST_IN == 0 {
        goto_out!();
    }

    /* sk_buff::dev and sk_buff::rbnode are unionized. */
    head = inet_frag_pull_head(&mut (*fq).q);
    if head.is_null() {
        goto_out!();
    }

    (*head).dev = dev;
    spin_unlock(&mut (*fq).q.lock);

    icmpv6_send(head, ICMPV6_TIME_EXCEED, ICMPV6_EXC_FRAGTIME, 0);
    kfree_skb_reason(head, SKB_DROP_REASON_FRAG_REASM_TIMEOUT);

    rcu_read_unlock();
    inet_frag_putn(&mut (*fq).q, refs);
}

/* Check if the upper layer header is truncated in the first fragment. */
#[inline]
pub unsafe fn ipv6frag_thdr_truncated(
    skb: *mut sk_buff,
    start: i32,
    nexthdrp: *mut u8,
) -> bool {
    let mut nexthdr = *nexthdrp;
    let mut frag_off: u16 = 0;
    let mut offset = ipv6_skip_exthdr(skb, start, &mut nexthdr, &mut frag_off);
    if offset < 0 || (frag_off & htons(IP6_OFFSET)) != 0 {
        return false;
    }
    match nexthdr {
        NEXTHDR_TCP => offset += core::mem::size_of::<tcphdr>() as i32,
        NEXTHDR_UDP => offset += core::mem::size_of::<udphdr>() as i32,
        NEXTHDR_ICMP => offset += core::mem::size_of::<icmp6hdr>() as i32,
        _ => offset += 1,
    }
    offset > (*skb).len as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
