// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2015 Pablo Neira Ayuso <pablo@netfilter.org>
 */

// Linux kernel dependencies supplied by other translation units.

unsafe fn nf_do_netdev_egress(
    skb: *mut sk_buff,
    dev: *mut net_device,
    hook: nf_dev_hooks,
) {
    if hook == NF_NETDEV_INGRESS && skb_mac_header_was_set(skb) {
        if skb_cow_head(skb, (*skb).mac_len) != 0 {
            kfree_skb(skb);
            return;
        }

        skb_push(skb, (*skb).mac_len);
    }

    (*skb).dev = dev;
    skb_clear_tstamp(skb);
    local_bh_disable();
    if nf_dev_xmit_recursion() != 0 {
        local_bh_enable();
        kfree_skb(skb);
        return;
    }
    nf_dev_xmit_recursion_inc();
    dev_queue_xmit(skb);
    nf_dev_xmit_recursion_dec();
    local_bh_enable();
}

pub unsafe fn nf_fwd_netdev_egress(pkt: *const nft_pktinfo, oif: i32) {
    let dev: *mut net_device;

    dev = dev_get_by_index_rcu(nft_net(pkt), oif);
    if dev.is_null() {
        kfree_skb((*pkt).skb);
        return;
    }

    nf_do_netdev_egress((*pkt).skb, dev, nft_hook(pkt));
}

pub unsafe fn nf_dup_netdev_egress(pkt: *const nft_pktinfo, oif: i32) {
    let dev: *mut net_device;
    let skb: *mut sk_buff;

    dev = dev_get_by_index_rcu(nft_net(pkt), oif);
    if dev.is_null() {
        return;
    }

    skb = skb_clone((*pkt).skb, GFP_ATOMIC);
    if !skb.is_null() {
        nf_do_netdev_egress(skb, dev, nft_hook(pkt));
    }
}

pub unsafe fn nft_fwd_dup_netdev_offload(
    ctx: *mut nft_offload_ctx,
    flow: *mut nft_flow_rule,
    id: flow_action_id,
    oif: i32,
) -> i32 {
    let entry: *mut flow_action_entry;
    let dev: *mut net_device;

    dev = dev_get_by_index((*ctx).net, oif);
    if dev.is_null() {
        return -EOPNOTSUPP;
    }

    entry = nft_flow_action_entry_next(ctx, flow);
    if entry.is_null() {
        dev_put(dev);
        return -E2BIG;
    }

    (*entry).id = id;
    /* nft_flow_rule_destroy() releases the reference on this device. */
    (*entry).dev = dev;

    0
}

// EXPORT_SYMBOL_GPL(nf_fwd_netdev_egress);
// EXPORT_SYMBOL_GPL(nf_dup_netdev_egress);
// EXPORT_SYMBOL_GPL(nft_fwd_dup_netdev_offload);

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Pablo Neira Ayuso <pablo@netfilter.org>");
// MODULE_DESCRIPTION("Netfilter packet duplication support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
