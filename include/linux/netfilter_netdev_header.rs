/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux netfilter/netdevice bindings.

#[cfg(CONFIG_NETFILTER_INGRESS)]
#[inline]
pub unsafe fn nf_hook_ingress_active(skb: *const sk_buff) -> bool {
    #[cfg(CONFIG_JUMP_LABEL)]
    {
        if !static_key_false(&nf_hooks_needed[NFPROTO_NETDEV][NF_NETDEV_INGRESS]) {
            return false;
        }
    }
    rcu_access_pointer((*(*skb).dev).nf_hooks_ingress)
}

// caller must hold rcu_read_lock
#[cfg(CONFIG_NETFILTER_INGRESS)]
#[inline]
pub unsafe fn nf_hook_ingress(skb: *mut sk_buff) -> i32 {
    let e: *mut nf_hook_entries = rcu_dereference((*(*skb).dev).nf_hooks_ingress);
    let mut state: nf_hook_state = core::mem::MaybeUninit::uninit().assume_init();
    let ret: i32;

    /* Must recheck the ingress hook head, in the event it became NULL
     * after the check in nf_hook_ingress_active evaluated to true.
     */
    if unlikely(e.is_null()) {
        return 0;
    }

    nf_hook_state_init(
        &mut state,
        NF_NETDEV_INGRESS,
        NFPROTO_NETDEV,
        (*skb).dev,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        dev_net((*skb).dev),
        core::ptr::null_mut(),
    );
    ret = nf_hook_slow(skb, &mut state, e, 0);
    if ret == 0 {
        return -1;
    }

    ret
}

#[cfg(not(CONFIG_NETFILTER_INGRESS))]
#[inline]
pub unsafe fn nf_hook_ingress_active(_skb: *mut sk_buff) -> i32 {
    0
}

#[cfg(not(CONFIG_NETFILTER_INGRESS))]
#[inline]
pub unsafe fn nf_hook_ingress(_skb: *mut sk_buff) -> i32 {
    0
}

#[cfg(CONFIG_NETFILTER_EGRESS)]
#[inline]
pub unsafe fn nf_hook_egress_active() -> bool {
    #[cfg(CONFIG_JUMP_LABEL)]
    {
        if !static_key_false(&nf_hooks_needed[NFPROTO_NETDEV][NF_NETDEV_EGRESS]) {
            return false;
        }
    }
    true
}

/**
 * nf_hook_egress - classify packets before transmission
 * @skb: packet to be classified
 * @rc: result code which shall be returned by __dev_queue_xmit() on failure
 * @dev: netdev whose egress hooks shall be applied to @skb
 *
 * Caller must hold rcu_read_lock.
 *
 * On ingress, packets are classified first by tc, then by netfilter.
 * On egress, the order is reversed for symmetry.  Conceptually, tc and
 * netfilter can be thought of as layers, with netfilter layered above tc:
 * When tc redirects a packet to another interface, netfilter is not applied
 * because the packet is on the tc layer.
 *
 * The nf_skip_egress flag controls whether netfilter is applied on egress.
 * It is updated by __netif_receive_skb_core() and __dev_queue_xmit() when the
 * packet passes through tc and netfilter.  Because __dev_queue_xmit() may be
 * called recursively by tunnel drivers such as vxlan, the flag is reverted to
 * false after sch_handle_egress().  This ensures that netfilter is applied
 * both on the overlay and underlying network.
 *
 * Returns: @skb on success or %NULL if the packet was consumed or filtered.
 */
#[cfg(CONFIG_NETFILTER_EGRESS)]
#[inline]
pub unsafe fn nf_hook_egress(
    skb: *mut sk_buff,
    rc: *mut i32,
    dev: *mut net_device,
) -> *mut sk_buff {
    let e: *mut nf_hook_entries;
    let mut state: nf_hook_state = core::mem::MaybeUninit::uninit().assume_init();
    let ret: i32;

    #[cfg(CONFIG_NETFILTER_SKIP_EGRESS)]
    if (*skb).nf_skip_egress {
        return skb;
    }

    e = rcu_dereference_check((*dev).nf_hooks_egress, rcu_read_lock_bh_held());
    if e.is_null() {
        return skb;
    }

    nf_hook_state_init(
        &mut state,
        NF_NETDEV_EGRESS,
        NFPROTO_NETDEV,
        core::ptr::null_mut(),
        dev,
        core::ptr::null_mut(),
        dev_net(dev),
        core::ptr::null_mut(),
    );

    /* nf assumes rcu_read_lock, not just read_lock_bh */
    rcu_read_lock();
    ret = nf_hook_slow(skb, &mut state, e, 0);
    rcu_read_unlock();

    if ret == 1 {
        skb
    } else if ret < 0 {
        *rc = NET_XMIT_DROP;
        core::ptr::null_mut()
    } else {
        // ret == 0
        *rc = NET_XMIT_SUCCESS;
        core::ptr::null_mut()
    }
}

#[cfg(not(CONFIG_NETFILTER_EGRESS))]
#[inline]
pub unsafe fn nf_hook_egress_active() -> bool {
    false
}

#[cfg(not(CONFIG_NETFILTER_EGRESS))]
#[inline]
pub unsafe fn nf_hook_egress(
    skb: *mut sk_buff,
    _rc: *mut i32,
    _dev: *mut net_device,
) -> *mut sk_buff {
    skb
}

#[inline]
pub unsafe fn nf_skip_egress(skb: *mut sk_buff, skip: bool) {
    #[cfg(CONFIG_NETFILTER_SKIP_EGRESS)]
    {
        (*skb).nf_skip_egress = skip;
    }
}

#[inline]
pub unsafe fn nf_hook_netdev_init(dev: *mut net_device) {
    #[cfg(CONFIG_NETFILTER_INGRESS)]
    {
        RCU_INIT_POINTER!((*dev).nf_hooks_ingress, core::ptr::null_mut());
    }
    #[cfg(CONFIG_NETFILTER_EGRESS)]
    {
        RCU_INIT_POINTER!((*dev).nf_hooks_egress, core::ptr::null_mut());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
