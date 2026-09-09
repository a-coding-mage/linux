// SPDX-License-Identifier: GPL-2.0
/*
 * xfrm4_input.c
 *
 * Changes:
 *	YOSHIFUJI Hideaki @USAGI
 *		Split up af-specific portion
 *	Derek Atkins <derek@ihtfp.com>
 *		Add Encapsulation support
 *
 */

// Linux kernel headers and symbols are supplied by other translation units.

unsafe fn xfrm4_rcv_encap_finish2(
    _net: *mut net,
    _sk: *mut sock,
    skb: *mut sk_buff,
) -> i32 {
    dst_input(skb)
}

unsafe fn xfrm4_rcv_encap_finish(
    _net: *mut net,
    _sk: *mut sock,
    skb: *mut sk_buff,
) -> i32 {
    if skb_dst(skb).is_null() {
        let iph: *const iphdr = ip_hdr(skb);

        if ip_route_input_noref(
            skb,
            (*iph).daddr,
            (*iph).saddr,
            ip4h_dscp(iph),
            (*skb).dev,
        ) != 0
        {
            kfree_skb(skb);
            return NET_RX_DROP;
        }
    }

    if xfrm_trans_queue(skb, Some(xfrm4_rcv_encap_finish2)) != 0 {
        kfree_skb(skb);
        return NET_RX_DROP;
    }

    0
}

pub unsafe extern "C" fn xfrm4_transport_finish(skb: *mut sk_buff, async_: i32) -> i32 {
    let xo: *mut xfrm_offload = xfrm_offload(skb);
    let iph: *mut iphdr = ip_hdr(skb);
    let dev: *mut net_device = (*skb).dev;

    (*iph).protocol = XFRM_MODE_SKB_CB(skb).protocol;

    // #ifndef CONFIG_NETFILTER: preserved as source conditional intent.
    if !cfg!(feature = "CONFIG_NETFILTER") && async_ == 0 {
        return -((*iph).protocol as i32);
    }

    __skb_push(skb, -(skb_network_offset(skb) as isize));
    (*iph).tot_len = htons((*skb).len as u16);
    ip_send_check(iph);

    if !xo.is_null() && ((*xo).flags & XFRM_GRO) != 0 {
        /* The full l2 header needs to be preserved so that re-injecting the packet at l2
         * works correctly in the presence of vlan tags.
         */
        skb_mac_header_rebuild_full(skb, (*xo).orig_mac_len);
        skb_reset_network_header(skb);
        skb_reset_transport_header(skb);
        return 0;
    }

    NF_HOOK(
        NFPROTO_IPV4,
        NF_INET_PRE_ROUTING,
        dev_net(dev),
        core::ptr::null_mut(),
        skb,
        dev,
        core::ptr::null_mut(),
        Some(xfrm4_rcv_encap_finish),
    );
    0
}

unsafe fn __xfrm4_udp_encap_rcv(sk: *mut sock, skb: *mut sk_buff, pull: bool) -> i32 {
    let up: *mut udp_sock = udp_sk(sk);
    let mut uh: *mut udphdr;
    let mut iph: *mut iphdr;
    let mut iphlen: i32;
    let mut len: i32;
    let mut udpdata: *mut u8;
    let mut udpdata32: *mut __be32;
    let encap_type: u16;

    encap_type = READ_ONCE((*up).encap_type);
    /* if this is not encapsulated socket, then just return now */
    if encap_type == 0 {
        return 1;
    }

    /* If this is a paged skb, make sure we pull up
     * whatever data we need to look at. */
    len = (*skb).len as i32 - core::mem::size_of::<udphdr>() as i32;
    if !pskb_may_pull(
        skb,
        (core::mem::size_of::<udphdr>() as i32 + core::cmp::min(len, 8)) as u32,
    ) {
        return 1;
    }

    /* Now we can get the pointers */
    uh = udp_hdr(skb);
    udpdata = (uh as *mut u8).add(core::mem::size_of::<udphdr>());
    udpdata32 = udpdata as *mut __be32;

    match encap_type {
        _ => {
            /* Check if this is a keepalive packet.  If so, eat it. */
            if len == 1 && *udpdata == 0xff {
                return -EINVAL;
            } else if len > core::mem::size_of::<ip_esp_hdr>() as i32 && *udpdata32 != 0 {
                /* ESP Packet without Non-ESP header */
                len = core::mem::size_of::<udphdr>() as i32;
            } else {
                /* Must be an IKE packet.. pass it through */
                return 1;
            }
        }
    }

    /* At this point we are sure that this is an ESPinUDP packet,
     * so we need to remove 'len' bytes from the packet (the UDP
     * header and optional ESP marker bytes) and then modify the
     * protocol to ESP, and then call into the transform receiver.
     */
    if skb_unclone(skb, GFP_ATOMIC) != 0 {
        return -EINVAL;
    }

    /* Now we can update and verify the packet length... */
    iph = ip_hdr(skb);
    iphlen = ((*iph).ihl as i32) << 2;
    (*iph).tot_len = htons(ntohs((*iph).tot_len).wrapping_sub(len as u16));
    if (*skb).len as i32 < iphlen + len {
        /* packet is too small!?! */
        return -EINVAL;
    }

    /* pull the data buffer up to the ESP header and set the
     * transport header to point to ESP.  Keep UDP on the stack
     * for later.
     */
    if pull {
        __skb_pull(skb, len as u32);
        skb_reset_transport_header(skb);
    } else {
        skb_set_transport_header(skb, len as u32);
    }

    /* process ESP */
    0
}

/* If it's a keepalive packet, then just eat it.
 * If it's an encapsulated packet, then pass it to the
 * IPsec xfrm input.
 * Returns 0 if skb passed to xfrm or was dropped.
 * Returns >0 if skb should be passed to UDP.
 * Returns <0 if skb should be resubmitted (-ret is protocol)
 */
pub unsafe extern "C" fn xfrm4_udp_encap_rcv(sk: *mut sock, skb: *mut sk_buff) -> i32 {
    let ret = __xfrm4_udp_encap_rcv(sk, skb, true);
    if ret == 0 {
        return xfrm4_rcv_encap(skb, IPPROTO_ESP, 0, (*udp_sk(sk)).encap_type);
    }

    if ret < 0 {
        kfree_skb(skb);
        return 0;
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_gro_udp_encap_rcv(
    _sk: *mut sock,
    head: *mut list_head,
    skb: *mut sk_buff,
) -> *mut sk_buff {
    let offset = skb_gro_offset(skb);
    let mut ops: *const net_offload;
    let mut pp: *mut sk_buff = core::ptr::null_mut();
    let len = (*skb).len as i32 - offset;
    let dlen = offset + core::cmp::min(len, 8);
    let udpdata = skb_gro_header(skb, dlen, offset);
    let udpdata32 = udpdata as *mut __be32;
    if udpdata.is_null() {
        return core::ptr::null_mut();
    }

    rcu_read_lock();
    ops = rcu_dereference(inet_offloads[IPPROTO_ESP as usize]);
    if ops.is_null() || (*ops).callbacks.gro_receive.is_none() {
        rcu_read_unlock();
        NAPI_GRO_CB(skb).same_flow = 0;
        NAPI_GRO_CB(skb).flush = 1;
        return core::ptr::null_mut();
    }

    /* check if it is a keepalive or IKE packet */
    if len <= core::mem::size_of::<ip_esp_hdr>() as i32 || *udpdata32 == 0 {
        rcu_read_unlock();
        NAPI_GRO_CB(skb).same_flow = 0;
        NAPI_GRO_CB(skb).flush = 1;
        return core::ptr::null_mut();
    }

    /* set the transport header to ESP */
    skb_set_transport_header(skb, offset as u32);
    NAPI_GRO_CB(skb).proto = IPPROTO_UDP;
    pp = call_gro_receive((*ops).callbacks.gro_receive, head, skb);
    rcu_read_unlock();
    pp
}

pub unsafe extern "C" fn xfrm4_rcv(skb: *mut sk_buff) -> i32 {
    xfrm4_rcv_spi(skb, (*ip_hdr(skb)).protocol, 0)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
