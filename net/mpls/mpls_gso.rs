// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MPLS GSO Support
 *
 * Authors: Simon Horman (horms@verge.net.au)
 *
 * Based on: GSO portions of net/ipv4/gre.c
 */

// Dependency symbols supplied by the surrounding kernel translation.

unsafe fn mpls_gso_segment(
    mut skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    let mut segs: *mut sk_buff = ERR_PTR(-EINVAL);
    let mac_offset: u16 = (*skb).mac_header;
    let mpls_features: netdev_features_t;
    let mac_len: u16 = (*skb).mac_len;
    let mpls_protocol: __be16;
    let mpls_hlen: u32;

    if !skb_inner_network_header_was_set(skb) {
        return segs;
    }

    skb_reset_network_header(skb);
    mpls_hlen = skb_inner_network_header(skb) - skb_network_header(skb);
    if unlikely(mpls_hlen == 0 || mpls_hlen % MPLS_HLEN != 0) {
        return segs;
    }
    if unlikely(!pskb_may_pull(skb, mpls_hlen)) {
        return segs;
    }

    /* Setup inner SKB. */
    mpls_protocol = (*skb).protocol;
    (*skb).protocol = (*skb).inner_protocol;

    __skb_pull(skb, mpls_hlen);

    (*skb).mac_len = 0;
    skb_reset_mac_header(skb);

    /* Segment inner packet. */
    mpls_features = (*(*skb).dev).mpls_features & features;
    segs = skb_mac_gso_segment(skb, mpls_features);
    if IS_ERR_OR_NULL(segs) {
        skb_gso_error_unwind(skb, mpls_protocol, mpls_hlen, mac_offset, mac_len);
        return segs;
    }
    skb = segs;

    let mpls_hlen = mpls_hlen + mac_len as u32;
    loop {
        (*skb).mac_len = mac_len;
        (*skb).protocol = mpls_protocol;

        skb_reset_inner_network_header(skb);

        __skb_push(skb, mpls_hlen);

        skb_reset_mac_header(skb);
        skb_set_network_header(skb, mac_len);

        skb = (*skb).next;
        if skb.is_null() {
            break;
        }
    }

    segs
}

static mut mpls_mc_offload: packet_offload = packet_offload {
    type_: cpu_to_be16(ETH_P_MPLS_MC),
    priority: 15,
    callbacks: packet_offload_callbacks {
        gso_segment: Some(mpls_gso_segment),
    },
};

static mut mpls_uc_offload: packet_offload = packet_offload {
    type_: cpu_to_be16(ETH_P_MPLS_UC),
    priority: 15,
    callbacks: packet_offload_callbacks {
        gso_segment: Some(mpls_gso_segment),
    },
};

unsafe fn mpls_gso_init() -> c_int {
    pr_info!("MPLS GSO support\n");

    dev_add_offload(&raw mut mpls_uc_offload);
    dev_add_offload(&raw mut mpls_mc_offload);

    0
}

unsafe fn mpls_gso_exit() {
    dev_remove_offload(&raw mut mpls_uc_offload);
    dev_remove_offload(&raw mut mpls_mc_offload);
}

module_init!(mpls_gso_init);
module_exit!(mpls_gso_exit);

module_description!("MPLS GSO support");
module_author!("Simon Horman <horms@verge.net.au>");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
