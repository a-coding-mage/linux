// SPDX-License-Identifier: GPL-2.0-or-later
//
// C dependencies supplied by the surrounding kernel translation are intentionally
// referenced by name rather than reimplemented here.

/**
 *	skb_eth_gso_segment - segmentation handler for ethernet protocols.
 *	@skb: buffer to segment
 *	@features: features for the output path (see dev->features)
 *	@type: Ethernet Protocol ID
 */
pub unsafe fn skb_eth_gso_segment(
    skb: *mut sk_buff,
    features: netdev_features_t,
    type_: __be16,
) -> *mut sk_buff {
    let mut segs: *mut sk_buff = ERR_PTR(-EPROTONOSUPPORT);
    let mut ptype: *mut packet_offload;

    rcu_read_lock();
    list_for_each_entry_rcu!(ptype, &mut net_hotdata.offload_base, list, {
        if (*ptype).type_ == type_ && !(*ptype).callbacks.gso_segment.is_null() {
            segs = ((*ptype).callbacks.gso_segment)(skb, features);
            break;
        }
    });
    rcu_read_unlock();

    segs
}

/**
 *	skb_mac_gso_segment - mac layer segmentation handler.
 *	@skb: buffer to segment
 *	@features: features for the output path (see dev->features)
 */
pub unsafe fn skb_mac_gso_segment(
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    let mut segs: *mut sk_buff = ERR_PTR(-EPROTONOSUPPORT);
    let mut ptype: *mut packet_offload;
    let mut vlan_depth: i32 = (*skb).mac_len;
    let type_: __be16 = skb_network_protocol(skb, &mut vlan_depth);

    if unlikely(type_ == 0) {
        return ERR_PTR(-EINVAL);
    }

    __skb_pull(skb, vlan_depth);

    rcu_read_lock();
    list_for_each_entry_rcu!(ptype, &mut net_hotdata.offload_base, list, {
        if (*ptype).type_ == type_ && !(*ptype).callbacks.gso_segment.is_null() {
            segs = ((*ptype).callbacks.gso_segment)(skb, features);
            break;
        }
    });
    rcu_read_unlock();

    __skb_push(skb, skb.data.offset_from(skb_mac_header(skb)) as i32);

    segs
}

/* openvswitch calls this on rx path, so we need a different check.
 */
unsafe fn skb_needs_check(skb: *const sk_buff, tx_path: bool) -> bool {
    if tx_path {
        return (*skb).ip_summed != CHECKSUM_PARTIAL
            && (*skb).ip_summed != CHECKSUM_UNNECESSARY;
    }

    (*skb).ip_summed == CHECKSUM_NONE
}

/**
 *	__skb_gso_segment - Perform segmentation on skb.
 *	@skb: buffer to segment
 *	@features: features for the output path (see dev->features)
 *	@tx_path: whether it is called in TX path
 *
 *	This function segments the given skb and returns a list of segments.
 *
 *	It may return NULL if the skb requires no segmentation.  This is
 *	only possible when GSO is used for verifying header integrity.
 *
 *	Segmentation preserves SKB_GSO_CB_OFFSET bytes of previous skb cb.
 */
pub unsafe fn __skb_gso_segment(
    skb: *mut sk_buff,
    mut features: netdev_features_t,
    tx_path: bool,
) -> *mut sk_buff {
    let segs: *mut sk_buff;

    if unlikely(skb_needs_check(skb, tx_path)) {
        let err: i32;

        /* We're going to init ->check field in TCP or UDP header */
        err = skb_cow_head(skb, 0);
        if err < 0 {
            return ERR_PTR(err);
        }
    }

    /* Only report GSO partial support if it will enable us to
     * support segmentation on this frame without needing additional
     * work.
     */
    if features & NETIF_F_GSO_PARTIAL != 0 {
        let mut partial_features: netdev_features_t = NETIF_F_GSO_ROBUST;
        let dev: *mut net_device = (*skb).dev;

        partial_features |= (*dev).features & (*dev).gso_partial_features;
        if !skb_gso_ok(skb, features | partial_features) {
            features &= !NETIF_F_GSO_PARTIAL;
        }
    }

    // BUILD_BUG_ON(SKB_GSO_CB_OFFSET + sizeof(*SKB_GSO_CB(skb)) > sizeof(skb->cb));

    (*SKB_GSO_CB(skb)).mac_offset = skb_headroom(skb);
    (*SKB_GSO_CB(skb)).encap_level = 0;

    skb_reset_mac_header(skb);
    skb_reset_mac_len(skb);

    segs = skb_mac_gso_segment(skb, features);

    if segs != skb && unlikely(skb_needs_check(skb, tx_path) && !IS_ERR(segs)) {
        skb_warn_bad_offload(skb);
    }

    segs
}

/**
 * skb_gso_transport_seglen - Return length of individual segments of a gso packet
 *
 * @skb: GSO skb
 *
 * skb_gso_transport_seglen is used to determine the real size of the
 * individual segments, including Layer4 headers (TCP/UDP).
 *
 * The MAC/L2 or network (IP, IPv6) headers are not accounted for.
 */
unsafe fn skb_gso_transport_seglen(skb: *const sk_buff) -> u32 {
    let shinfo: *const skb_shared_info = skb_shinfo(skb);
    let mut thlen: u32 = 0;

    if (*skb).encapsulation {
        thlen = skb_inner_transport_header(skb).offset_from(skb_transport_header(skb)) as u32;

        if likely((*shinfo).gso_type & (SKB_GSO_TCPV4 | SKB_GSO_TCPV6) != 0) {
            thlen += inner_tcp_hdrlen(skb);
        }
    } else if likely((*shinfo).gso_type & (SKB_GSO_TCPV4 | SKB_GSO_TCPV6) != 0) {
        thlen = tcp_hdrlen(skb);
    } else if unlikely(skb_is_gso_sctp(skb)) {
        thlen = core::mem::size_of::<sctphdr>() as u32;
    } else if (*shinfo).gso_type & SKB_GSO_UDP_L4 != 0 {
        thlen = core::mem::size_of::<udphdr>() as u32;
    }
    /* UFO sets gso_size to the size of the fragmentation
     * payload, i.e. the size of the L4 (UDP) header is already
     * accounted for.
     */
    thlen + (*shinfo).gso_size
}

/**
 * skb_gso_network_seglen - Return length of individual segments of a gso packet
 */
unsafe fn skb_gso_network_seglen(skb: *const sk_buff) -> u32 {
    let hdr_len = skb_transport_header(skb).offset_from(skb_network_header(skb)) as u32;
    hdr_len + skb_gso_transport_seglen(skb)
}

/**
 * skb_gso_mac_seglen - Return length of individual segments of a gso packet
 */
unsafe fn skb_gso_mac_seglen(skb: *const sk_buff) -> u32 {
    let hdr_len = skb_transport_header(skb).offset_from(skb_mac_header(skb)) as u32;
    hdr_len + skb_gso_transport_seglen(skb)
}

/**
 * skb_gso_size_check - check the skb size, considering GSO_BY_FRAGS
 */
#[inline]
unsafe fn skb_gso_size_check(
    skb: *const sk_buff,
    mut seg_len: u32,
    max_len: u32,
) -> bool {
    let shinfo: *const skb_shared_info = skb_shinfo(skb);
    let mut iter: *const sk_buff;

    if (*shinfo).gso_size != GSO_BY_FRAGS {
        return seg_len <= max_len;
    }

    seg_len -= GSO_BY_FRAGS;

    skb_walk_frags!(skb, iter, {
        if seg_len + skb_headlen(iter) > max_len {
            return false;
        }
    });

    true
}

pub unsafe fn skb_gso_validate_network_len(skb: *const sk_buff, mtu: u32) -> bool {
    skb_gso_size_check(skb, skb_gso_network_seglen(skb), mtu)
}

pub unsafe fn skb_gso_validate_mac_len(skb: *const sk_buff, len: u32) -> bool {
    skb_gso_size_check(skb, skb_gso_mac_seglen(skb), len)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
