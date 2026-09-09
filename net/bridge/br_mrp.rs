// SPDX-License-Identifier: GPL-2.0-or-later

// Translated from br_mrp.c. Kernel headers and related symbols are supplied by
// other translation units.

static MRP_TEST_DMAC: [u8; ETH_ALEN] = [0x1, 0x15, 0x4e, 0x0, 0x0, 0x1];
static MRP_IN_TEST_DMAC: [u8; ETH_ALEN] = [0x1, 0x15, 0x4e, 0x0, 0x0, 0x3];

static mut mrp_frame_type: br_frame_type = br_frame_type {
    type_: cpu_to_be16(ETH_P_MRP),
    frame_handler: br_mrp_process,
};

unsafe fn br_mrp_is_ring_port(p_port: *mut net_bridge_port, s_port: *mut net_bridge_port, port: *mut net_bridge_port) -> bool {
    port == p_port || port == s_port
}

unsafe fn br_mrp_is_in_port(i_port: *mut net_bridge_port, port: *mut net_bridge_port) -> bool { port == i_port }

unsafe fn br_mrp_get_port(br: *mut net_bridge, ifindex: u32) -> *mut net_bridge_port {
    let mut res = core::ptr::null_mut();
    let mut port: *mut net_bridge_port;
    list_for_each_entry!(port, (*br).port_list, list) {
        if (*(*port).dev).ifindex == ifindex { res = port; break; }
    }
    res
}

unsafe fn br_mrp_find_id(br: *mut net_bridge, ring_id: u32) -> *mut br_mrp {
    let mut res = core::ptr::null_mut();
    let mut mrp: *mut br_mrp;
    hlist_for_each_entry_rcu!(mrp, (*br).mrp_list, list, lockdep_rtnl_is_held()) {
        if (*mrp).ring_id == ring_id { res = mrp; break; }
    }
    res
}

unsafe fn br_mrp_find_in_id(br: *mut net_bridge, in_id: u32) -> *mut br_mrp {
    let mut res = core::ptr::null_mut();
    let mut mrp: *mut br_mrp;
    hlist_for_each_entry_rcu!(mrp, (*br).mrp_list, list, lockdep_rtnl_is_held()) {
        if (*mrp).in_id == in_id { res = mrp; break; }
    }
    res
}

unsafe fn br_mrp_unique_ifindex(br: *mut net_bridge, ifindex: u32) -> bool {
    let mut mrp: *mut br_mrp;
    hlist_for_each_entry_rcu!(mrp, (*br).mrp_list, list, lockdep_rtnl_is_held()) {
        let mut p = rtnl_dereference!((*mrp).p_port);
        if !p.is_null() && (*(*p).dev).ifindex == ifindex { return false; }
        p = rtnl_dereference!((*mrp).s_port);
        if !p.is_null() && (*(*p).dev).ifindex == ifindex { return false; }
        p = rtnl_dereference!((*mrp).i_port);
        if !p.is_null() && (*(*p).dev).ifindex == ifindex { return false; }
    }
    true
}

unsafe fn br_mrp_find_port(br: *mut net_bridge, p: *mut net_bridge_port) -> *mut br_mrp {
    let mut res = core::ptr::null_mut();
    let mut mrp: *mut br_mrp;
    hlist_for_each_entry_rcu!(mrp, (*br).mrp_list, list, lockdep_rtnl_is_held()) {
        if rcu_access_pointer!((*mrp).p_port) == p || rcu_access_pointer!((*mrp).s_port) == p || rcu_access_pointer!((*mrp).i_port) == p { res = mrp; break; }
    }
    res
}

unsafe fn br_mrp_next_seq(mrp: *mut br_mrp) -> i32 { (*mrp).seq_id += 1; (*mrp).seq_id }

unsafe fn br_mrp_skb_alloc(p: *mut net_bridge_port, src: *const u8, dst: *const u8) -> *mut sk_buff {
    let skb = dev_alloc_skb(MRP_MAX_FRAME_LENGTH);
    if skb.is_null() { return core::ptr::null_mut(); }
    (*skb).dev = (*p).dev;
    (*skb).protocol = htons(ETH_P_MRP);
    (*skb).priority = MRP_FRAME_PRIO;
    skb_reserve(skb, core::mem::size_of::<ethhdr>());
    let eth_hdr = skb_push(skb, core::mem::size_of::<ethhdr>()) as *mut ethhdr;
    ether_addr_copy((*eth_hdr).h_dest.as_mut_ptr(), dst);
    ether_addr_copy((*eth_hdr).h_source.as_mut_ptr(), src);
    (*eth_hdr).h_proto = htons(ETH_P_MRP);
    let version = skb_put(skb, core::mem::size_of::<__be16>()) as *mut __be16;
    *version = cpu_to_be16(MRP_VERSION);
    skb
}

unsafe fn br_mrp_skb_tlv(skb: *mut sk_buff, type_: br_mrp_tlv_header_type, length: u8) {
    let hdr = skb_put(skb, core::mem::size_of::<br_mrp_tlv_hdr>()) as *mut br_mrp_tlv_hdr;
    (*hdr).type_ = type_; (*hdr).length = length;
}

unsafe fn br_mrp_skb_common(skb: *mut sk_buff, mrp: *mut br_mrp) {
    br_mrp_skb_tlv(skb, BR_MRP_TLV_HEADER_COMMON, core::mem::size_of::<br_mrp_common_hdr>() as u8);
    let hdr = skb_put(skb, core::mem::size_of::<br_mrp_common_hdr>()) as *mut br_mrp_common_hdr;
    (*hdr).seq_id = cpu_to_be16(br_mrp_next_seq(mrp) as u16);
    core::ptr::write_bytes((*hdr).domain.as_mut_ptr(), 0xff, MRP_DOMAIN_UUID_LENGTH);
}

unsafe fn br_mrp_alloc_test_skb(mrp: *mut br_mrp, p: *mut net_bridge_port, port_role: br_mrp_port_role_type) -> *mut sk_buff {
    if p.is_null() { return core::ptr::null_mut(); }
    let skb = br_mrp_skb_alloc(p, (*(*p).dev).dev_addr.as_ptr(), MRP_TEST_DMAC.as_ptr());
    if skb.is_null() { return core::ptr::null_mut(); }
    br_mrp_skb_tlv(skb, BR_MRP_TLV_HEADER_RING_TEST, core::mem::size_of::<br_mrp_ring_test_hdr>() as u8);
    let hdr = skb_put(skb, core::mem::size_of::<br_mrp_ring_test_hdr>()) as *mut br_mrp_ring_test_hdr;
    (*hdr).prio = cpu_to_be16((*mrp).prio); ether_addr_copy((*hdr).sa.as_mut_ptr(), (*(*p).br).dev.dev_addr.as_ptr());
    (*hdr).port_role = cpu_to_be16(port_role as u16); (*hdr).state = cpu_to_be16((*mrp).ring_state as u16);
    (*hdr).transitions = cpu_to_be16((*mrp).ring_transitions as u16); (*hdr).timestamp = cpu_to_be32(jiffies_to_msecs(jiffies));
    br_mrp_skb_common(skb, mrp);
    if (*mrp).ring_role == BR_MRP_RING_ROLE_MRA {
        let length = (core::mem::size_of::<br_mrp_sub_option1_hdr>() + core::mem::size_of::<br_mrp_tlv_hdr>() + core::mem::size_of::<br_mrp_oui_hdr>() + MRP_OPT_PADDING) as u8;
        br_mrp_skb_tlv(skb, BR_MRP_TLV_HEADER_OPTION, length);
        let oui = skb_put(skb, core::mem::size_of::<br_mrp_oui_hdr>()); core::ptr::write_bytes(oui, 0, core::mem::size_of::<br_mrp_oui_hdr>());
        let sub_opt = skb_put(skb, core::mem::size_of::<br_mrp_sub_option1_hdr>()); core::ptr::write_bytes(sub_opt, 0, core::mem::size_of::<br_mrp_sub_option1_hdr>());
        let sub_tlv = skb_put_zero(skb, core::mem::size_of::<br_mrp_tlv_hdr>() + MRP_OPT_PADDING) as *mut br_mrp_tlv_hdr;
        (*sub_tlv).type_ = BR_MRP_SUB_TLV_HEADER_TEST_AUTO_MGR;
    }
    br_mrp_skb_tlv(skb, BR_MRP_TLV_HEADER_END, 0); skb
}

unsafe fn br_mrp_alloc_in_test_skb(mrp: *mut br_mrp, p: *mut net_bridge_port, port_role: br_mrp_port_role_type) -> *mut sk_buff {
    if p.is_null() { return core::ptr::null_mut(); }
    let skb = br_mrp_skb_alloc(p, (*(*p).dev).dev_addr.as_ptr(), MRP_IN_TEST_DMAC.as_ptr());
    if skb.is_null() { return core::ptr::null_mut(); }
    br_mrp_skb_tlv(skb, BR_MRP_TLV_HEADER_IN_TEST, core::mem::size_of::<br_mrp_in_test_hdr>() as u8);
    let hdr = skb_put(skb, core::mem::size_of::<br_mrp_in_test_hdr>()) as *mut br_mrp_in_test_hdr;
    (*hdr).id = cpu_to_be16((*mrp).in_id as u16); ether_addr_copy((*hdr).sa.as_mut_ptr(), (*(*p).br).dev.dev_addr.as_ptr());
    (*hdr).port_role = cpu_to_be16(port_role as u16); (*hdr).state = cpu_to_be16((*mrp).in_state as u16);
    (*hdr).transitions = cpu_to_be16((*mrp).in_transitions as u16); (*hdr).timestamp = cpu_to_be32(jiffies_to_msecs(jiffies));
    br_mrp_skb_common(skb, mrp); br_mrp_skb_tlv(skb, BR_MRP_TLV_HEADER_END, 0); skb
}

unsafe fn br_mrp_test_work_expired(work: *mut work_struct) {
    let mrp = container_of!(to_delayed_work(work), br_mrp, test_work);
    if time_before_eq!((*mrp).test_end, jiffies) { return; }
    let notify_open = if (*mrp).test_count_miss < (*mrp).test_max_miss { (*mrp).test_count_miss += 1; false } else { (*mrp).ring_state == BR_MRP_RING_STATE_CLOSED || (*mrp).test_monitor };
    rcu_read_lock();
    for (port, role) in [((*mrp).p_port, BR_MRP_PORT_ROLE_PRIMARY), ((*mrp).s_port, BR_MRP_PORT_ROLE_SECONDARY)] {
        let p = rcu_dereference!(port); if !p.is_null() {
            if !(*mrp).test_monitor { let skb = br_mrp_alloc_test_skb(mrp, p, role); if skb.is_null() { break; } skb_reset_network_header(skb); dev_queue_xmit(skb); }
            if notify_open && !(*mrp).ring_role_offloaded { br_mrp_ring_port_open((*p).dev, true); }
        }
    }
    rcu_read_unlock(); queue_delayed_work(system_percpu_wq, &mut (*mrp).test_work, usecs_to_jiffies((*mrp).test_interval));
}

unsafe fn br_mrp_in_test_work_expired(work: *mut work_struct) {
    let mrp = container_of!(to_delayed_work(work), br_mrp, in_test_work);
    if time_before_eq!((*mrp).in_test_end, jiffies) { return; }
    let notify_open = if (*mrp).in_test_count_miss < (*mrp).in_test_max_miss { (*mrp).in_test_count_miss += 1; false } else { (*mrp).in_state == BR_MRP_IN_STATE_CLOSED };
    rcu_read_lock();
    for (port, role) in [((*mrp).p_port, BR_MRP_PORT_ROLE_PRIMARY), ((*mrp).s_port, BR_MRP_PORT_ROLE_SECONDARY), ((*mrp).i_port, BR_MRP_PORT_ROLE_INTER)] {
        let p = rcu_dereference!(port); if !p.is_null() { let skb = br_mrp_alloc_in_test_skb(mrp, p, role); if skb.is_null() { break; } skb_reset_network_header(skb); dev_queue_xmit(skb); if notify_open && !(*mrp).in_role_offloaded { br_mrp_in_port_open((*p).dev, true); } }
    }
    rcu_read_unlock(); queue_delayed_work(system_percpu_wq, &mut (*mrp).in_test_work, usecs_to_jiffies((*mrp).in_test_interval));
}

unsafe fn br_mrp_ring_frame(skb: *mut sk_buff) -> bool { let mut h = br_mrp_tlv_hdr::default(); let p = skb_header_pointer(skb, core::mem::size_of::<u16>(), core::mem::size_of_val(&h), &mut h); if p.is_null() { return false; } matches!((*p).type_, BR_MRP_TLV_HEADER_RING_TEST | BR_MRP_TLV_HEADER_RING_TOPO | BR_MRP_TLV_HEADER_RING_LINK_DOWN | BR_MRP_TLV_HEADER_RING_LINK_UP | BR_MRP_TLV_HEADER_OPTION) }
unsafe fn br_mrp_in_frame(skb: *mut sk_buff) -> bool { let mut h = br_mrp_tlv_hdr::default(); let p = skb_header_pointer(skb, core::mem::size_of::<u16>(), core::mem::size_of_val(&h), &mut h); if p.is_null() { return false; } matches!((*p).type_, BR_MRP_TLV_HEADER_IN_TEST | BR_MRP_TLV_HEADER_IN_TOPO | BR_MRP_TLV_HEADER_IN_LINK_DOWN | BR_MRP_TLV_HEADER_IN_LINK_UP | BR_MRP_TLV_HEADER_IN_LINK_STATUS) }
unsafe fn br_mrp_get_frame_type(skb: *mut sk_buff) -> u8 { let mut h = br_mrp_tlv_hdr::default(); let p = skb_header_pointer(skb, 2, core::mem::size_of_val(&h), &mut h); if p.is_null() { 0xff } else { (*p).type_ } }
unsafe fn br_mrp_mrm_behaviour(mrp: *mut br_mrp) -> bool { (*mrp).ring_role == BR_MRP_RING_ROLE_MRM || ((*mrp).ring_role == BR_MRP_RING_ROLE_MRA && !(*mrp).test_monitor) }
unsafe fn br_mrp_mrc_behaviour(mrp: *mut br_mrp) -> bool { (*mrp).ring_role == BR_MRP_RING_ROLE_MRC || ((*mrp).ring_role == BR_MRP_RING_ROLE_MRA && (*mrp).test_monitor) }

unsafe fn br_mrp_process(p: *mut net_bridge_port, skb: *mut sk_buff) -> i32 { if likely!(!test_bit(BR_MRP_AWARE_BIT, &(*p).flags)) { 0 } else { br_mrp_rcv(p, skb, (*p).dev) } }
unsafe fn br_mrp_rcv(_p: *mut net_bridge_port, _skb: *mut sk_buff, _dev: *mut net_device) -> i32 { 1 }
pub unsafe fn br_mrp_enabled(br: *mut net_bridge) -> bool { !hlist_empty!((*br).mrp_list) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
