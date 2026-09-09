// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Rusty Russell (C)2000 -- This code is GPL.
 * Patrick McHardy (c) 2006-2012
 */

// Kernel headers and "nf_internals.h" provide the types and symbols used here.

static mut nf_queue_handler: *const nf_queue_handler = core::ptr::null();

/*
 * Hook for nfnetlink_queue to register its queue handler.
 * We do this so that most of the NFQUEUE code can be modular.
 *
 * Once the queue is registered it must reinject all packets it
 * receives, no matter what.
 */
pub unsafe fn nf_register_queue_handler(qh: *const nf_queue_handler) {
    /* should never happen, we only have one queueing backend in kernel */
    WARN_ON(!nf_queue_handler.is_null());
    nf_queue_handler = qh;
}

/* The caller must flush their queue before this */
pub unsafe fn nf_unregister_queue_handler() {
    nf_queue_handler = core::ptr::null();
}

unsafe fn nf_queue_sock_put(sk: *mut sock) {
    // CONFIG_INET selects sock_gen_put; otherwise sock_put is used.
    #[cfg(CONFIG_INET)]
    {
        sock_gen_put(sk);
    }
    #[cfg(not(CONFIG_INET))]
    {
        sock_put(sk);
    }
}

unsafe fn nf_queue_entry_release_refs(entry: *mut nf_queue_entry) {
    let state: *mut nf_hook_state = &mut (*entry).state;

    /* Release those devices we held, or Alexey will kill me. */
    dev_put((*entry).skb_dev);
    dev_put((*state).in_);
    dev_put((*state).out);
    if !(*state).sk.is_null() {
        nf_queue_sock_put((*state).sk);
    }

    #[cfg(CONFIG_BRIDGE_NETFILTER)]
    {
        dev_put((*entry).bridge_dev);
        dev_put((*entry).physin);
        dev_put((*entry).physout);
    }
}

pub unsafe fn nf_queue_entry_free(entry: *mut nf_queue_entry) {
    nf_queue_entry_release_refs(entry);
    kfree(entry as *mut core::ffi::c_void);
}

unsafe fn __nf_queue_entry_init_physdevs(entry: *mut nf_queue_entry) {
    #[cfg(CONFIG_BRIDGE_NETFILTER)]
    {
        let skb: *const sk_buff = (*entry).skb;
        let dst: *mut dst_entry = skb_dst(skb);
        let mut dev: *mut net_device = core::ptr::null_mut();

        if nf_bridge_info_exists(skb) {
            (*entry).physin = nf_bridge_get_physindev(skb, (*entry).state.net);
            (*entry).physout = nf_bridge_get_physoutdev(skb);
        } else {
            (*entry).physin = core::ptr::null_mut();
            (*entry).physout = core::ptr::null_mut();
        }

        if (*entry).state.pf == NFPROTO_BRIDGE
            && !dst.is_null()
            && ((*dst).flags & DST_FAKE_RTABLE) != 0
        {
            dev = dst_dev_rcu(dst);
        }

        /* Must hold a reference on the bridge device: dst_hold() protects
         * the dst itself, but the fake rtable is embedded in bridge-private
         * storage that netdevice teardown can free independently.
         */
        (*entry).bridge_dev = dev;
    }
}

/* Bump dev refs so they don't vanish while packet is out */
pub unsafe fn nf_queue_entry_get_refs(entry: *mut nf_queue_entry) -> bool {
    let state: *mut nf_hook_state = &mut (*entry).state;

    if !(*state).sk.is_null() && !refcount_inc_not_zero(&mut (*(*state).sk).sk_refcnt) {
        return false;
    }

    dev_hold((*entry).skb_dev);
    dev_hold((*state).in_);
    dev_hold((*state).out);

    #[cfg(CONFIG_BRIDGE_NETFILTER)]
    {
        dev_hold((*entry).bridge_dev);
        dev_hold((*entry).physin);
        dev_hold((*entry).physout);
    }
    true
}

pub unsafe fn nf_queue_nf_hook_drop(net: *mut net) {
    let qh: *const nf_queue_handler;

    rcu_read_lock();
    qh = nf_queue_handler;
    if !qh.is_null() {
        ((*qh).nf_hook_drop)(net);
    }
    rcu_read_unlock();
}

unsafe fn nf_ip_saveroute(skb: *const sk_buff, entry: *mut nf_queue_entry) {
    let rt_info: *mut ip_rt_info = nf_queue_entry_reroute(entry);

    if (*entry).state.hook == NF_INET_LOCAL_OUT {
        let iph: *const iphdr = ip_hdr(skb);

        (*rt_info).tos = (*iph).tos;
        (*rt_info).daddr = (*iph).daddr;
        (*rt_info).saddr = (*iph).saddr;
        (*rt_info).mark = (*skb).mark;
    }
}

unsafe fn nf_ip6_saveroute(skb: *const sk_buff, entry: *mut nf_queue_entry) {
    let rt_info: *mut ip6_rt_info = nf_queue_entry_reroute(entry);

    if (*entry).state.hook == NF_INET_LOCAL_OUT {
        let iph: *const ipv6hdr = ipv6_hdr(skb);

        (*rt_info).daddr = (*iph).daddr;
        (*rt_info).saddr = (*iph).saddr;
        (*rt_info).mark = (*skb).mark;
    }
}

unsafe fn __nf_queue(
    skb: *mut sk_buff,
    state: *const nf_hook_state,
    index: u32,
    queuenum: u32,
) -> i32 {
    let mut entry: *mut nf_queue_entry = core::ptr::null_mut();
    let qh: *const nf_queue_handler;
    let route_key_size: usize;
    let mut status: i32;

    /* QUEUE == DROP if no one is waiting, to be safe. */
    qh = nf_queue_handler;
    if qh.is_null() {
        return -ESRCH;
    }

    route_key_size = match (*state).pf {
        AF_INET => core::mem::size_of::<ip_rt_info>(),
        AF_INET6 => core::mem::size_of::<ip6_rt_info>(),
        _ => 0,
    };

    if skb_sk_is_prefetched(skb) {
        let sk: *mut sock = (*skb).sk;

        if !sk_is_refcounted(sk) {
            if !refcount_inc_not_zero(&mut (*sk).sk_refcnt) {
                return -ENOTCONN;
            }

            /* drop refcount on skb_orphan */
            (*skb).destructor = Some(sock_edemux);
        }
    }

    entry = kmalloc(core::mem::size_of::<nf_queue_entry>() + route_key_size, GFP_ATOMIC)
        as *mut nf_queue_entry;
    if entry.is_null() {
        return -ENOMEM;
    }

    if !skb_dst(skb).is_null() && !skb_dst_force(skb) {
        kfree(entry as *mut core::ffi::c_void);
        return -ENETDOWN;
    }

    (*entry).skb = skb;
    (*entry).skb_dev = (*skb).dev;
    (*entry).state = *state;
    (*entry).hook_index = index;
    (*entry).size = core::mem::size_of::<nf_queue_entry>() + route_key_size;
    __nf_queue_entry_init_physdevs(entry);

    if !nf_queue_entry_get_refs(entry) {
        kfree(entry as *mut core::ffi::c_void);
        return -ENOTCONN;
    }

    match (*entry).state.pf {
        AF_INET => nf_ip_saveroute(skb, entry),
        AF_INET6 => nf_ip6_saveroute(skb, entry),
        _ => {}
    }

    status = ((*qh).outfn)(entry, queuenum);
    if status < 0 {
        nf_queue_entry_free(entry);
        return status;
    }

    0
}

/* Packets leaving via this function must come back through nf_reinject(). */
pub unsafe fn nf_queue(
    skb: *mut sk_buff,
    state: *mut nf_hook_state,
    index: u32,
    verdict: u32,
) -> i32 {
    let ret = __nf_queue(skb, state, index, verdict >> NF_VERDICT_QBITS);
    if ret < 0 {
        if ret == -ESRCH && (verdict & NF_VERDICT_FLAG_QUEUE_BYPASS) != 0 {
            return 1;
        }
        kfree_skb(skb);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
