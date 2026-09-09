// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * xfrm_device.c - IPsec device offloading code.
 *
 * Copyright (c) 2015 secunet Security Networks AG
 *
 * Author:
 * Steffen Klassert <steffen.klassert@secunet.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation.

#[cfg(CONFIG_XFRM_OFFLOAD)]
unsafe fn __xfrm_transport_prep(x: *mut xfrm_state, skb: *mut sk_buff, _hsize: c_uint) {
    let xo = xfrm_offload(skb);

    skb_reset_mac_len(skb);
    if (*xo).flags & XFRM_GSO_SEGMENT != 0 {
        (*skb).transport_header -= (*x).props.header_len;
    }

    pskb_pull(skb, skb_transport_offset(skb) + (*x).props.header_len);
}

#[cfg(CONFIG_XFRM_OFFLOAD)]
unsafe fn __xfrm_mode_tunnel_prep(x: *mut xfrm_state, skb: *mut sk_buff, hsize: c_uint) {
    let xo = xfrm_offload(skb);

    if (*xo).flags & XFRM_GSO_SEGMENT != 0 {
        (*skb).transport_header = (*skb).network_header + hsize;
    }

    skb_reset_mac_len(skb);
    pskb_pull(skb, (*skb).mac_len + (*x).props.header_len - (*x).props.enc_hdr_len);
}

#[cfg(CONFIG_XFRM_OFFLOAD)]
unsafe fn __xfrm_mode_beet_prep(x: *mut xfrm_state, skb: *mut sk_buff, hsize: c_uint) {
    let xo = xfrm_offload(skb);
    let mut phlen: c_int = 0;

    if (*xo).flags & XFRM_GSO_SEGMENT != 0 {
        (*skb).transport_header = (*skb).network_header + hsize;
    }

    skb_reset_mac_len(skb);
    if (*x).sel.family != AF_INET6 {
        phlen = IPV4_BEET_PHMAXLEN;
        if (*x).outer_mode.family == AF_INET6 {
            phlen += size_of::<ipv6hdr>() as c_int - size_of::<iphdr>() as c_int;
        }
    }

    pskb_pull(skb, (*skb).mac_len + hsize + ((*x).props.header_len - phlen as _));
}

#[cfg(CONFIG_XFRM_OFFLOAD)]
unsafe fn xfrm_outer_mode_prep(x: *mut xfrm_state, skb: *mut sk_buff) {
    match (*x).outer_mode.encap {
        XFRM_MODE_IPTFS | XFRM_MODE_TUNNEL => {
            if (*x).outer_mode.family == AF_INET {
                return __xfrm_mode_tunnel_prep(x, skb, size_of::<iphdr>() as c_uint);
            }
            if (*x).outer_mode.family == AF_INET6 {
                return __xfrm_mode_tunnel_prep(x, skb, size_of::<ipv6hdr>() as c_uint);
            }
        }
        XFRM_MODE_TRANSPORT => {
            if (*x).outer_mode.family == AF_INET {
                return __xfrm_transport_prep(x, skb, size_of::<iphdr>() as c_uint);
            }
            if (*x).outer_mode.family == AF_INET6 {
                return __xfrm_transport_prep(x, skb, size_of::<ipv6hdr>() as c_uint);
            }
        }
        XFRM_MODE_BEET => {
            if (*x).outer_mode.family == AF_INET {
                return __xfrm_mode_beet_prep(x, skb, size_of::<iphdr>() as c_uint);
            }
            if (*x).outer_mode.family == AF_INET6 {
                return __xfrm_mode_beet_prep(x, skb, size_of::<ipv6hdr>() as c_uint);
            }
        }
        XFRM_MODE_ROUTEOPTIMIZATION | XFRM_MODE_IN_TRIGGER => {}
        _ => {}
    }
}

#[cfg(CONFIG_XFRM_OFFLOAD)]
#[inline]
unsafe fn xmit_xfrm_check_overflow(skb: *mut sk_buff) -> bool {
    let xo = xfrm_offload(skb);
    let mut seq = (*xo).seq.low;

    seq = seq.wrapping_add((*skb).gso_segs());
    if unlikely(seq < (*xo).seq.low) {
        return true;
    }

    false
}

#[cfg(CONFIG_XFRM_OFFLOAD)]
pub unsafe fn validate_xmit_xfrm(
    mut skb: *mut sk_buff,
    features: netdev_features_t,
    again: *mut bool,
) -> *mut sk_buff {
    let mut err: c_int;
    let mut flags: c_ulong;
    let mut x: *mut xfrm_state;
    let mut sd: *mut softnet_data;
    let (mut skb2, mut nskb, mut pskb): (*mut sk_buff, *mut sk_buff, *mut sk_buff) = (null_mut(), null_mut(), null_mut());
    let mut esp_features = features;
    let mut xo = xfrm_offload(skb);
    let dev = (*skb).dev;
    let sp: *mut sec_path;

    if xo.is_null() || (*xo).flags & XFRM_XMIT != 0 { return skb; }

    if features & NETIF_F_HW_ESP == 0 {
        esp_features = features & !(NETIF_F_SG | NETIF_F_CSUM_MASK);
    }

    sp = skb_sec_path(skb);
    x = (*sp).xvec[(*sp).len - 1];
    if (*xo).flags & XFRM_GRO != 0 || (*x).xso.dir == XFRM_DEV_OFFLOAD_IN { return skb; }

    if (*x).xso.type == XFRM_DEV_OFFLOAD_PACKET && (*x).xso.dev != dev {
        kfree_skb(skb); dev_core_stats_tx_dropped_inc(dev); return null_mut();
    }

    local_irq_save(&mut flags);
    sd = this_cpu_ptr(&mut softnet_data);
    err = (!skb_queue_empty(&mut (*sd).xfrm_backlog)) as c_int;
    local_irq_restore(flags);
    if err != 0 { *again = true; return skb; }

    if skb_is_gso(skb) && (unlikely((*x).xso.dev != dev) || unlikely(xmit_xfrm_check_overflow(skb))) {
        esp_features &= !(NETIF_F_HW_ESP | NETIF_F_GSO_ESP);
        let segs = skb_gso_segment(skb, esp_features);
        if IS_ERR(segs) { kfree_skb(skb); dev_core_stats_tx_dropped_inc(dev); return null_mut(); }
        consume_skb(skb); skb = segs;
    }

    if (*skb).next.is_null() {
        esp_features |= (*(*skb).dev).gso_partial_features;
        xfrm_outer_mode_prep(x, skb);
        (*xo).flags |= XFRM_DEV_RESUME;
        err = ((*x).type_offload).as_ref().unwrap().xmit(x, skb, esp_features);
        if err != 0 {
            if err == -EINPROGRESS { return ERR_PTR(-EINPROGRESS); }
            XFRM_INC_STATS(xs_net(x), LINUX_MIB_XFRMOUTSTATEPROTOERROR); kfree_skb(skb); return null_mut();
        }
        skb_push(skb, (*skb).data.offset_from(skb_mac_header(skb)) as _); return skb;
    }

    skb_list_walk_safe(skb, skb2, nskb) {
        esp_features |= (*(*skb2).dev).gso_partial_features;
        skb_mark_not_on_list(skb2);
        xo = xfrm_offload(skb2); (*xo).flags |= XFRM_DEV_RESUME;
        xfrm_outer_mode_prep(x, skb2);
        err = ((*x).type_offload).as_ref().unwrap().xmit(x, skb2, esp_features);
        if err == 0 { (*skb2).next = nskb; }
        else if err != -EINPROGRESS {
            XFRM_INC_STATS(xs_net(x), LINUX_MIB_XFRMOUTSTATEPROTOERROR); (*skb2).next = nskb; kfree_skb_list(skb2); return null_mut();
        } else {
            if skb == skb2 { skb = nskb; } else { (*pskb).next = nskb; } continue;
        }
        skb_push(skb2, (*skb2).data.offset_from(skb_mac_header(skb2)) as _); pskb = skb2;
    }
    if !skb.is_null() { (*skb).prev = pskb; }
    if !skb.is_null() { skb } else { ERR_PTR(-EINPROGRESS) }
}

#[cfg(CONFIG_XFRM_OFFLOAD)]
pub unsafe fn xfrm_dev_state_add(net: *mut net, x: *mut xfrm_state, xuo: *const xfrm_user_offload, extack: *mut netlink_ext_ack) -> c_int {
    let packet = (*xuo).flags & XFRM_OFFLOAD_PACKET != 0;
    if (*xuo).flags & !(XFRM_OFFLOAD_IPV6 | XFRM_OFFLOAD_INBOUND | XFRM_OFFLOAD_PACKET) != 0 { NL_SET_ERR_MSG(extack, "Unrecognized flags in offload request"); return -EINVAL; }
    if ((*xuo).flags & XFRM_OFFLOAD_INBOUND != 0 && (*x).dir == XFRM_SA_DIR_OUT) || ((*xuo).flags & XFRM_OFFLOAD_INBOUND == 0 && (*x).dir == XFRM_SA_DIR_IN) { NL_SET_ERR_MSG(extack, "Mismatched SA and offload direction"); return -EINVAL; }
    if (*xuo).flags & XFRM_OFFLOAD_INBOUND != 0 && (*xuo).if_id != 0 { NL_SET_ERR_MSG(extack, "XFRM if_id is not supported in RX path"); return -EINVAL; }
    if (*x).tfcpad { NL_SET_ERR_MSG(extack, "TFC padding can't be offloaded"); return -EINVAL; }
    let mut dev = dev_get_by_index(net, (*xuo).ifindex);
    if dev.is_null() {
        let mut p: xfrm_dst_lookup_params = zeroed(); p.net = net;
        p.saddr = if (*xuo).flags & XFRM_OFFLOAD_INBOUND == 0 { &mut (*x).props.saddr } else { &mut (*x).id.daddr };
        p.daddr = if (*xuo).flags & XFRM_OFFLOAD_INBOUND == 0 { &mut (*x).id.daddr } else { &mut (*x).props.saddr };
        p.mark = xfrm_smark_get(0, x); let dst = __xfrm_dst_lookup((*x).props.family, &p);
        if IS_ERR(dst) { return if packet { -EINVAL } else { 0 }; }
        dev = (*dst).dev; dev_hold(dev); dst_release(dst);
    }
    let xso = &mut (*x).xso;
    if (*dev).xfrmdev_ops.is_null() || (*(*dev).xfrmdev_ops).xdo_dev_state_add.is_none() { xso.dev = null_mut(); dev_put(dev); return if packet { -EINVAL } else { 0 }; }
    if !packet && (*x).props.flags & XFRM_STATE_ESN != 0 && (*(*dev).xfrmdev_ops).xdo_dev_state_advance_esn.is_none() { NL_SET_ERR_MSG(extack, "Device doesn't support offload with ESN"); xso.dev = null_mut(); dev_put(dev); return -EINVAL; }
    if (*x).type_offload.is_null() { NL_SET_ERR_MSG(extack, "Type doesn't support offload"); dev_put(dev); return -EINVAL; }
    xso.dev = dev; xso.ifindex = (*dev).ifindex; netdev_tracker_alloc(dev, &mut xso.dev_tracker, GFP_ATOMIC);
    xso.dir = if (*xuo).flags & XFRM_OFFLOAD_INBOUND != 0 { XFRM_DEV_OFFLOAD_IN } else { XFRM_DEV_OFFLOAD_OUT };
    xso.r#type = if packet { XFRM_DEV_OFFLOAD_PACKET } else { XFRM_DEV_OFFLOAD_CRYPTO };
    let err = ((*(*dev).xfrmdev_ops).xdo_dev_state_add.unwrap())(dev, x, extack);
    if err != 0 { xso.dev = null_mut(); xso.dir = 0; netdev_put(dev, &mut xso.dev_tracker); xso.r#type = XFRM_DEV_OFFLOAD_UNSPECIFIED; xfrm_unset_type_offload(x); if err != -EOPNOTSUPP || packet { NL_SET_ERR_MSG_WEAK(extack, "Device failed to offload this state"); return err; } }
    0
}

#[cfg(CONFIG_XFRM_OFFLOAD)]
pub unsafe fn xfrm_dev_policy_add(_net: *mut net, xp: *mut xfrm_policy, xuo: *mut xfrm_user_offload, dir: u8, extack: *mut netlink_ext_ack) -> c_int {
    if (*xuo).flags == 0 || (*xuo).flags & !XFRM_OFFLOAD_PACKET != 0 { NL_SET_ERR_MSG(extack, "Unrecognized flags in offload request"); return -EINVAL; }
    let dev = dev_get_by_index(_net, (*xuo).ifindex); if dev.is_null() { return -EINVAL; }
    let xdo = &mut (*xp).xdo;
    if (*dev).xfrmdev_ops.is_null() || (*(*dev).xfrmdev_ops).xdo_dev_policy_add.is_none() { xdo.dev = null_mut(); dev_put(dev); NL_SET_ERR_MSG(extack, "Policy offload is not supported"); return -EINVAL; }
    xdo.dev = dev; netdev_tracker_alloc(dev, &mut xdo.dev_tracker, GFP_ATOMIC); xdo.r#type = XFRM_DEV_OFFLOAD_PACKET;
    xdo.dir = match dir { XFRM_POLICY_IN => XFRM_DEV_OFFLOAD_IN, XFRM_POLICY_OUT => XFRM_DEV_OFFLOAD_OUT, XFRM_POLICY_FWD => XFRM_DEV_OFFLOAD_FWD, _ => { xdo.dev = null_mut(); netdev_put(dev, &mut xdo.dev_tracker); NL_SET_ERR_MSG(extack, "Unrecognized offload direction"); return -EINVAL; } };
    let err = ((*(*dev).xfrmdev_ops).xdo_dev_policy_add.unwrap())(xp, extack); if err != 0 { xdo.dev = null_mut(); xdo.r#type = XFRM_DEV_OFFLOAD_UNSPECIFIED; xdo.dir = 0; netdev_put(dev, &mut xdo.dev_tracker); NL_SET_ERR_MSG_WEAK(extack, "Device failed to offload this policy"); return err; } 0
}

#[cfg(CONFIG_XFRM_OFFLOAD)]
pub unsafe fn xfrm_dev_offload_ok(skb: *mut sk_buff, x: *mut xfrm_state) -> bool {
    let dst = skb_dst(skb); let xd = dst as *mut xfrm_dst; let dev = (*x).xso.dev;
    if (*x).type_offload.is_null() || ((*x).xso.r#type == XFRM_DEV_OFFLOAD_UNSPECIFIED && (*x).encap) { return false; }
    if (dev.is_null() || dev == xfrm_dst_path(dst).dev) && (*(*xd).child).xfrm.is_null() {
        let mtu = xfrm_state_mtu(x, (*xd).child_mtu_cached);
        if (*skb).len <= mtu || (skb_is_gso(skb) && skb_gso_validate_network_len(skb, mtu)) { if dev.is_null() { return true; } }
        else { return false; }
    } else { return false; }
    let tunnel = (*x).xso.r#type == XFRM_DEV_OFFLOAD_PACKET && (*x).props.mode == XFRM_MODE_TUNNEL;
    match (*(*skb_dst(skb)).ops).family { AF_INET => { if (*ip_hdr(skb)).ihl != 5 || (tunnel && xfrm4_tunnel_check_size(skb)) { return false; } }, AF_INET6 => { if ipv6_ext_hdr((*ipv6_hdr(skb)).nexthdr) || (tunnel && xfrm6_tunnel_check_size(skb)) { return false; } }, _ => {} }
    if !(*(*dev).xfrmdev_ops).xdo_dev_offload_ok.is_none() { return ((*(*dev).xfrmdev_ops).xdo_dev_offload_ok.unwrap())(skb, x); } true
}

#[cfg(CONFIG_XFRM_OFFLOAD)]
pub unsafe fn xfrm_dev_resume(mut skb: *mut sk_buff) {
    let dev = (*skb).dev; let mut ret = NETDEV_TX_BUSY; let txq = netdev_core_pick_tx(dev, skb, null_mut());
    HARD_TX_LOCK(dev, txq, smp_processor_id()); if !netif_xmit_frozen_or_stopped(txq) { skb = dev_hard_start_xmit(skb, dev, txq, &mut ret); } HARD_TX_UNLOCK(dev, txq);
    if !dev_xmit_complete(ret) { let mut flags = 0; local_irq_save(&mut flags); let sd = this_cpu_ptr(&mut softnet_data); skb_queue_tail(&mut (*sd).xfrm_backlog, skb); raise_softirq_irqoff(NET_TX_SOFTIRQ); local_irq_restore(flags); }
}

#[cfg(CONFIG_XFRM_OFFLOAD)]
pub unsafe fn xfrm_dev_backlog(sd: *mut softnet_data) { let q = &mut (*sd).xfrm_backlog; if skb_queue_empty(q) { return; } let mut list: sk_buff_head = zeroed(); __skb_queue_head_init(&mut list); spin_lock(&mut q.lock); skb_queue_splice_init(q, &mut list); spin_unlock(&mut q.lock); while !skb_queue_empty(&mut list) { xfrm_dev_resume(__skb_dequeue(&mut list)); } }

unsafe fn xfrm_api_check(dev: *mut net_device) -> c_int { if (*dev).features & NETIF_F_HW_ESP_TX_CSUM != 0 && (*dev).features & NETIF_F_HW_ESP == 0 { return NOTIFY_BAD; } if (*dev).features & NETIF_F_HW_ESP != 0 && ((*dev).xfrmdev_ops.is_null() || (*(*dev).xfrmdev_ops).xdo_dev_state_add.is_none() || (*(*dev).xfrmdev_ops).xdo_dev_state_delete.is_none()) { return NOTIFY_BAD; } NOTIFY_DONE }
unsafe fn xfrm_dev_down(dev: *mut net_device) -> c_int { if (*dev).features & NETIF_F_HW_ESP != 0 { xfrm_dev_state_flush(dev_net(dev), dev, true); xfrm_dev_policy_flush(dev_net(dev), dev, true); } NOTIFY_DONE }
unsafe fn xfrm_dev_unregister(dev: *mut net_device) -> c_int { xfrm_dev_state_flush(dev_net(dev), dev, true); xfrm_dev_policy_flush(dev_net(dev), dev, true); NOTIFY_DONE }
unsafe fn xfrm_dev_event(_this: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> c_int { let dev = netdev_notifier_info_to_dev(ptr); match event { NETDEV_REGISTER | NETDEV_FEAT_CHANGE => xfrm_api_check(dev), NETDEV_DOWN => xfrm_dev_down(dev), NETDEV_UNREGISTER => xfrm_dev_unregister(dev), _ => NOTIFY_DONE } }
static mut xfrm_dev_notifier: notifier_block = notifier_block { notifier_call: Some(xfrm_dev_event) };
pub unsafe fn xfrm_dev_init() { register_netdevice_notifier(&mut xfrm_dev_notifier); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
