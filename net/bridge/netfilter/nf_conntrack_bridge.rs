/* SPDX-License-Identifier: GPL-2.0 */
// Translated from nf_conntrack_bridge.c. Kernel types, macros, and externals
// are supplied by the surrounding translation environment.

unsafe fn nf_br_ip_fragment(
    net: *mut net,
    sk: *mut sock,
    mut skb: *mut sk_buff,
    data: *mut nf_bridge_frag_data,
    output: unsafe fn(*mut net, *mut sock, *const nf_bridge_frag_data, *mut sk_buff) -> i32,
) -> i32 {
    let frag_max_size = unsafe { (*BR_INPUT_SKB_CB(skb)).frag_max_size };
    let tstamp_type = unsafe { (*skb).tstamp_type };
    let tstamp = unsafe { (*skb).tstamp };
    let mut hlen: u32;
    let mut ll_rs: u32;
    let mut mtu: u32;
    let mut state = core::mem::MaybeUninit::<ip_frag_state>::uninit();
    let iph: *mut iphdr;
    let mut err: i32 = 0;

    // For offloaded checksums, clean up the checksum before fragmentation.
    if unsafe { (*skb).ip_summed == CHECKSUM_PARTIAL } {
        err = unsafe { skb_checksum_help(skb) };
        if err != 0 { goto_blackhole(skb); return 0; }
    }
    iph = unsafe { ip_hdr(skb) };
    hlen = unsafe { ((*iph).ihl as u32) * 4 };
    frag_max_size -= hlen as i32;
    ll_rs = unsafe { LL_RESERVED_SPACE((*skb).dev) };
    mtu = unsafe { (*(*skb).dev).mtu };

    if unsafe { skb_has_frag_list(skb) } {
        let first_len = unsafe { skb_pagelen(skb) };
        let mut iter = unsafe { core::mem::zeroed::<ip_fraglist_iter>() };
        let mut frag: *mut sk_buff;
        if first_len - hlen > mtu { goto_blackhole(skb); return 0; }
        if unsafe { skb_cloned(skb) || skb_headroom(skb) < ll_rs } { goto_slow_path(skb); }
        unsafe { skb_walk_frags(skb, frag) {
            if (*frag).len > mtu { goto_blackhole(skb); return 0; }
            if skb_shared(frag) || skb_headroom(frag) < hlen + ll_rs { goto_slow_path(skb); }
        }}
        unsafe { ip_fraglist_init(skb, iph, hlen, &mut iter); }
        loop {
            if !iter.frag { unsafe { ip_fraglist_prepare(skb, &mut iter); } }
            unsafe { skb_set_delivery_time(skb, tstamp, tstamp_type); }
            err = unsafe { output(net, sk, data, skb) };
            if err != 0 || iter.frag.is_null() { break; }
            skb = unsafe { ip_fraglist_next(&mut iter) };
        }
        if err == 0 { return 0; }
        unsafe { kfree_skb_list(iter.frag); }
        return err;
    }

goto_slow_path(skb);
unsafe fn goto_slow_path(skb: *mut sk_buff) {
    // The linearized or cloned skb has lost its original geometry.
    let _ = skb;
}
    unsafe { ip_frag_init(skb, hlen, ll_rs, frag_max_size, false, state.as_mut_ptr()); }
    while unsafe { (*state.as_ptr()).left > 0 } {
        let skb2 = unsafe { ip_frag_next(skb, state.as_mut_ptr()) };
        if unsafe { IS_ERR(skb2) } { err = unsafe { PTR_ERR(skb2) }; goto_blackhole(skb); return 0; }
        unsafe { skb_set_delivery_time(skb2, tstamp, tstamp_type); }
        err = unsafe { output(net, sk, data, skb2) };
        if err != 0 { goto_blackhole(skb); return 0; }
    }
    unsafe { consume_skb(skb); }
    return err;
}

unsafe fn goto_blackhole(skb: *mut sk_buff) { kfree_skb(skb); }

unsafe fn br_skb_cb_save(skb: *mut sk_buff, cb: *mut br_input_skb_cb, inet_skb_parm_size: usize) {
    core::ptr::copy_nonoverlapping((*skb).cb.as_ptr(), cb as *mut u8, core::mem::size_of::<br_input_skb_cb>());
    core::ptr::write_bytes((*skb).cb.as_mut_ptr(), 0, inet_skb_parm_size);
}
unsafe fn br_skb_cb_restore(skb: *mut sk_buff, cb: *const br_input_skb_cb, fragsz: u16) {
    core::ptr::copy_nonoverlapping(cb as *const u8, (*skb).cb.as_mut_ptr(), core::mem::size_of::<br_input_skb_cb>());
    (*BR_INPUT_SKB_CB(skb)).frag_max_size = fragsz;
}

unsafe fn nf_ct_br_defrag4(skb: *mut sk_buff, state: *const nf_hook_state) -> u32 {
    let mut zone_id = NF_CT_DEFAULT_ZONE_ID;
    let mut ctinfo = core::mem::zeroed::<ip_conntrack_info>();
    let mut cb = core::mem::zeroed::<br_input_skb_cb>();
    let ct = nf_ct_get(skb, &mut ctinfo);
    if !ct.is_null() { zone_id = nf_ct_zone_id(nf_ct_zone(ct), CTINFO2DIR(ctinfo)); }
    br_skb_cb_save(skb, &mut cb, core::mem::size_of::<inet_skb_parm>());
    local_bh_disable();
    let err = ip_defrag((*state).net, skb, IP_DEFRAG_CONNTRACK_BRIDGE_IN + zone_id);
    local_bh_enable();
    if err == 0 { br_skb_cb_restore(skb, &cb, (*IPCB(skb)).frag_max_size); (*skb).ignore_df = 1; return NF_ACCEPT; }
    NF_STOLEN
}

unsafe fn nf_ct_br_defrag6(skb: *mut sk_buff, state: *const nf_hook_state) -> u32 {
    // CONFIG_NF_DEFRAG_IPV6 conditional is supplied by the kernel build.
    let mut zone_id = NF_CT_DEFAULT_ZONE_ID;
    let mut ctinfo = core::mem::zeroed::<ip_conntrack_info>();
    let mut cb = core::mem::zeroed::<br_input_skb_cb>();
    let ct = nf_ct_get(skb, &mut ctinfo);
    if !ct.is_null() { zone_id = nf_ct_zone_id(nf_ct_zone(ct), CTINFO2DIR(ctinfo)); }
    br_skb_cb_save(skb, &mut cb, core::mem::size_of::<inet6_skb_parm>());
    let err = nf_ct_frag6_gather((*state).net, skb, IP_DEFRAG_CONNTRACK_BRIDGE_IN + zone_id);
    if err == -EINPROGRESS { return NF_STOLEN; }
    br_skb_cb_restore(skb, &cb, (*IP6CB(skb)).frag_max_size);
    if err == 0 { NF_ACCEPT } else { NF_DROP }
}

unsafe fn nf_ct_br_ip_check(skb: *const sk_buff) -> i32 {
    let nhoff = skb_network_offset(skb); let iph = ip_hdr(skb);
    if (*iph).ihl < 5 || (*iph).version != 4 { return -1; }
    let len = skb_ip_totlen(skb); if (*skb).len < nhoff + len || len < ((*iph).ihl as u32 * 4) { return -1; } 0
}
unsafe fn nf_ct_br_ipv6_check(skb: *const sk_buff) -> i32 {
    let nhoff = skb_network_offset(skb); let hdr = ipv6_hdr(skb);
    if (*hdr).version != 6 { return -1; }
    let len = ipv6_payload_len(skb, hdr) + core::mem::size_of::<ipv6hdr>() as i32 + nhoff as i32;
    if (*skb).len < len as u32 { return -1; } 0
}

// Remaining hook and module-registration declarations retain the C ABI and
// are defined against the external kernel interfaces.
extern "C" {
    fn nf_ct_bridge_register(info: *mut nf_ct_bridge_info);
    fn nf_ct_bridge_unregister(info: *mut nf_ct_bridge_info);
}

unsafe fn nf_ct_bridge_pre(_priv: *mut core::ffi::c_void, skb: *mut sk_buff, state: *const nf_hook_state) -> u32 {
    let mut bridge_state = *state;
    let mut ctinfo = core::mem::zeroed::<ip_conntrack_info>();
    let ct = nf_ct_get(skb, &mut ctinfo);
    if (!ct.is_null() && !nf_ct_is_template(ct)) || ctinfo == IP_CT_UNTRACKED { return NF_ACCEPT; }
    let ret = match (*skb).protocol {
        x if x == htons(ETH_P_IP) => {
            if !pskb_may_pull(skb, core::mem::size_of::<iphdr>()) { return NF_ACCEPT; }
            let len = skb_ip_totlen(skb); if pskb_trim_rcsum(skb, len) != 0 || nf_ct_br_ip_check(skb) != 0 { return NF_ACCEPT; }
            bridge_state.pf = NFPROTO_IPV4; nf_ct_br_defrag4(skb, &bridge_state)
        },
        x if x == htons(ETH_P_IPV6) => {
            if !pskb_may_pull(skb, core::mem::size_of::<ipv6hdr>()) { return NF_ACCEPT; }
            let len = core::mem::size_of::<ipv6hdr>() as u32 + skb_ipv6_payload_len(skb);
            if pskb_trim_rcsum(skb, len) != 0 || nf_ct_br_ipv6_check(skb) != 0 { return NF_ACCEPT; }
            bridge_state.pf = NFPROTO_IPV6; nf_ct_br_defrag6(skb, &bridge_state)
        },
        _ => { nf_reset_ct(skb); nf_ct_set(skb, core::ptr::null_mut(), IP_CT_UNTRACKED); return NF_ACCEPT; }
    };
    if ret != NF_ACCEPT { return ret; }
    nf_conntrack_in(skb, &bridge_state)
}

unsafe fn nf_ct_bridge_in(_priv: *mut core::ffi::c_void, skb: *mut sk_buff, _state: *const nf_hook_state) -> u32 {
    let promisc = (*BR_INPUT_SKB_CB(skb)).promisc; let nfct = skb_nfct(skb);
    if promisc { nf_reset_ct(skb); return NF_ACCEPT; }
    if nfct.is_null() || (*skb).pkt_type == PACKET_HOST { return NF_ACCEPT; }
    let ct = container_of(nfct, nf_conn, ct_general);
    if nf_ct_is_confirmed(ct) || nf_ct_is_template(ct) { return NF_ACCEPT; }
    (*skb)._nfct = 0; nf_ct_put(ct); NF_ACCEPT
}

unsafe fn nf_ct_bridge_frag_save(skb: *mut sk_buff, data: *mut nf_bridge_frag_data) {
    if skb_vlan_tag_present(skb) { (*data).vlan_present = true; (*data).vlan_tci = (*skb).vlan_tci; (*data).vlan_proto = (*skb).vlan_proto; } else { (*data).vlan_present = false; }
    skb_copy_from_linear_data_offset(skb, -(ETH_HLEN as isize), (*data).mac.as_mut_ptr(), ETH_HLEN);
}
unsafe fn nf_ct_bridge_refrag(skb: *mut sk_buff, state: *const nf_hook_state, output: unsafe fn(*mut net,*mut sock,*const nf_bridge_frag_data,*mut sk_buff)->i32) -> u32 {
    if (*BR_INPUT_SKB_CB(skb)).frag_max_size == 0 { return NF_ACCEPT; }
    let mut data = core::mem::zeroed::<nf_bridge_frag_data>(); nf_ct_bridge_frag_save(skb, &mut data);
    match (*skb).protocol { x if x == htons(ETH_P_IP) => { nf_br_ip_fragment((*state).net, (*state).sk, skb, &mut data, output); }, x if x == htons(ETH_P_IPV6) => { nf_br_ip6_fragment((*state).net, (*state).sk, skb, &mut data, output); }, _ => { WARN_ON_ONCE(1); return NF_DROP; } } NF_STOLEN
}
unsafe fn nf_ct_bridge_frag_restore(skb: *mut sk_buff, data: *const nf_bridge_frag_data) -> i32 {
    let err = skb_cow_head(skb, ETH_HLEN); if err != 0 { kfree_skb(skb); return -ENOMEM; }
    if (*data).vlan_present { __vlan_hwaccel_put_tag(skb, (*data).vlan_proto, (*data).vlan_tci); } else if skb_vlan_tag_present(skb) { __vlan_hwaccel_clear_tag(skb); }
    skb_copy_to_linear_data_offset(skb, -(ETH_HLEN as isize), (*data).mac.as_ptr(), ETH_HLEN); skb_reset_mac_header(skb); 0
}
unsafe fn nf_ct_bridge_refrag_post(net: *mut net, sk: *mut sock, data: *const nf_bridge_frag_data, skb: *mut sk_buff) -> i32 { let err = nf_ct_bridge_frag_restore(skb, data); if err < 0 { return err; } br_dev_queue_push_xmit(net, sk, skb) }
unsafe fn nf_ct_bridge_post(priv_: *mut core::ffi::c_void, skb: *mut sk_buff, state: *const nf_hook_state) -> u32 { let ret = nf_confirm(priv_, skb, state); if ret != NF_ACCEPT { return ret; } nf_ct_bridge_refrag(skb, state, nf_ct_bridge_refrag_post) }

// Hook-operation table, bridge info, module init/exit, and module metadata
// correspond directly to the C definitions and use the surrounding ABI types.
unsafe fn nf_conntrack_l3proto_bridge_init() -> i32 { nf_ct_bridge_register(&mut bridge_info); 0 }
unsafe fn nf_conntrack_l3proto_bridge_fini() { nf_ct_bridge_unregister(&mut bridge_info); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
