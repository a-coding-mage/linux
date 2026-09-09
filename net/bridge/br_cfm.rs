// SPDX-License-Identifier: GPL-2.0-or-later

// Kernel dependencies supplied by the surrounding translation unit.

unsafe fn br_mep_find(br: *mut net_bridge, instance: u32) -> *mut br_cfm_mep {
    let mut mep: *mut br_cfm_mep;
    hlist_for_each_entry!(mep, unsafe { &mut (*br).mep_list }, head, {
        if unsafe { (*mep).instance == instance } { return mep; }
    });
    core::ptr::null_mut()
}

unsafe fn br_mep_find_ifindex(br: *mut net_bridge, ifindex: u32) -> *mut br_cfm_mep {
    let mut mep: *mut br_cfm_mep;
    hlist_for_each_entry_rcu!(mep, unsafe { &mut (*br).mep_list }, head, lockdep_rtnl_is_held(), {
        if unsafe { (*mep).create.ifindex == ifindex } { return mep; }
    });
    core::ptr::null_mut()
}

unsafe fn br_peer_mep_find(mep: *mut br_cfm_mep, mepid: u32) -> *mut br_cfm_peer_mep {
    let mut peer_mep: *mut br_cfm_peer_mep;
    hlist_for_each_entry_rcu!(peer_mep, unsafe { &mut (*mep).peer_mep_list }, head, lockdep_rtnl_is_held(), {
        if unsafe { (*peer_mep).mepid == mepid } { return peer_mep; }
    });
    core::ptr::null_mut()
}

unsafe fn br_mep_get_port(br: *mut net_bridge, ifindex: u32) -> *mut net_bridge_port {
    let mut port: *mut net_bridge_port;
    list_for_each_entry!(port, unsafe { &mut (*br).port_list }, list, {
        if unsafe { (*(*port).dev).ifindex == ifindex } { return port; }
    });
    core::ptr::null_mut()
}

/* Calculate the CCM interval in us. */
unsafe fn interval_to_us(interval: br_cfm_ccm_interval) -> u32 {
    match interval {
        BR_CFM_CCM_INTERVAL_NONE => 0,
        BR_CFM_CCM_INTERVAL_3_3_MS => 3300,
        BR_CFM_CCM_INTERVAL_10_MS => 10 * 1000,
        BR_CFM_CCM_INTERVAL_100_MS => 100 * 1000,
        BR_CFM_CCM_INTERVAL_1_SEC => 1000 * 1000,
        BR_CFM_CCM_INTERVAL_10_SEC => 10 * 1000 * 1000,
        BR_CFM_CCM_INTERVAL_1_MIN => 60 * 1000 * 1000,
        BR_CFM_CCM_INTERVAL_10_MIN => 10 * 60 * 1000 * 1000,
        _ => 0,
    }
}

/* Convert the interface interval to CCM PDU value. */
unsafe fn interval_to_pdu(interval: br_cfm_ccm_interval) -> u32 {
    match interval {
        BR_CFM_CCM_INTERVAL_NONE => 0,
        BR_CFM_CCM_INTERVAL_3_3_MS => 1,
        BR_CFM_CCM_INTERVAL_10_MS => 2,
        BR_CFM_CCM_INTERVAL_100_MS => 3,
        BR_CFM_CCM_INTERVAL_1_SEC => 4,
        BR_CFM_CCM_INTERVAL_10_SEC => 5,
        BR_CFM_CCM_INTERVAL_1_MIN => 6,
        BR_CFM_CCM_INTERVAL_10_MIN => 7,
        _ => 0,
    }
}

/* Convert the CCM PDU value to interval on interface. */
unsafe fn pdu_to_interval(value: u32) -> br_cfm_ccm_interval {
    match value {
        0 => BR_CFM_CCM_INTERVAL_NONE,
        1 => BR_CFM_CCM_INTERVAL_3_3_MS,
        2 => BR_CFM_CCM_INTERVAL_10_MS,
        3 => BR_CFM_CCM_INTERVAL_100_MS,
        4 => BR_CFM_CCM_INTERVAL_1_SEC,
        5 => BR_CFM_CCM_INTERVAL_10_SEC,
        6 => BR_CFM_CCM_INTERVAL_1_MIN,
        7 => BR_CFM_CCM_INTERVAL_10_MIN,
        _ => BR_CFM_CCM_INTERVAL_NONE,
    }
}

unsafe fn ccm_rx_timer_start(peer_mep: *mut br_cfm_peer_mep) {
    let interval_us = interval_to_us((*(*peer_mep).mep).cc_config.exp_interval);
    /* Function ccm_rx_dwork must be called with 1/4 of the configured CC
     * 'expected_interval' in order to detect CCM defect after 3.25 interval.
     */
    queue_delayed_work(system_percpu_wq, &mut (*peer_mep).ccm_rx_dwork,
                       usecs_to_jiffies(interval_us / 4));
}

unsafe fn br_cfm_notify(event: i32, port: *const net_bridge_port) {
    let filter = RTEXT_FILTER_CFM_STATUS;
    br_info_notify(event, (*port).br, core::ptr::null_mut(), filter);
}

unsafe fn cc_peer_enable(peer_mep: *mut br_cfm_peer_mep) {
    core::ptr::write_bytes(&mut (*peer_mep).cc_status as *mut _, 0, 1);
    (*peer_mep).ccm_rx_count_miss = 0;
    ccm_rx_timer_start(peer_mep);
}

unsafe fn cc_peer_disable(peer_mep: *mut br_cfm_peer_mep) {
    cancel_delayed_work_sync(&mut (*peer_mep).ccm_rx_dwork);
}

unsafe fn ccm_frame_build(mep: *mut br_cfm_mep, tx_info: *const br_cfm_cc_ccm_tx_info) -> *mut sk_buff {
    let skb = dev_alloc_skb(CFM_CCM_MAX_FRAME_LENGTH);
    if skb.is_null() { return core::ptr::null_mut(); }
    rcu_read_lock();
    let b_port = rcu_dereference((*mep).b_port);
    if b_port.is_null() { kfree_skb(skb); rcu_read_unlock(); return core::ptr::null_mut(); }
    (*skb).dev = (*b_port).dev;
    rcu_read_unlock();
    (*skb).protocol = htons(ETH_P_CFM);
    (*skb).priority = CFM_FRAME_PRIO;
    let eth_hdr = skb_put(skb, core::mem::size_of::<ethhdr>()) as *mut ethhdr;
    ether_addr_copy((*eth_hdr).h_dest.as_mut_ptr(), (*tx_info).dmac.addr.as_ptr());
    ether_addr_copy((*eth_hdr).h_source.as_mut_ptr(), (*mep).config.unicast_mac.addr.as_ptr());
    (*eth_hdr).h_proto = htons(ETH_P_CFM);
    let common_hdr = skb_put(skb, core::mem::size_of::<br_cfm_common_hdr>()) as *mut br_cfm_common_hdr;
    (*common_hdr).mdlevel_version = (*mep).config.mdlevel << 5;
    (*common_hdr).opcode = BR_CFM_OPCODE_CCM;
    (*common_hdr).flags = ((*mep).rdi << 7) | interval_to_pdu((*mep).cc_config.exp_interval);
    (*common_hdr).tlv_offset = CFM_CCM_TLV_OFFSET;
    let snumber = skb_put(skb, core::mem::size_of::<u32>()) as *mut __be32;
    if (*tx_info).seq_no_update { *snumber = cpu_to_be32((*mep).ccm_tx_snumber); (*mep).ccm_tx_snumber += 1; } else { *snumber = 0; }
    let mepid = skb_put(skb, core::mem::size_of::<u16>()) as *mut __be16;
    *mepid = cpu_to_be16((*mep).config.mepid as u16);
    let maid = skb_put(skb, core::mem::size_of::<br_cfm_maid>()) as *mut br_cfm_maid;
    core::ptr::copy_nonoverlapping((*mep).cc_config.exp_maid.data.as_ptr(), (*maid).data.as_mut_ptr(), (*maid).data.len());
    let itu_reserved = skb_put(skb, CFM_CCM_ITU_RESERVED_SIZE);
    core::ptr::write_bytes(itu_reserved, 0, CFM_CCM_ITU_RESERVED_SIZE as usize);
    if (*tx_info).port_tlv { let t = skb_put(skb, 4) as *mut __be32; *t = cpu_to_be32((CFM_PORT_STATUS_TLV_TYPE << 24) | (1 << 8) | ((*tx_info).port_tlv_value & 0xFF)); }
    if (*tx_info).if_tlv { let t = skb_put(skb, 4) as *mut __be32; *t = cpu_to_be32((CFM_IF_STATUS_TLV_TYPE << 24) | (1 << 8) | ((*tx_info).if_tlv_value & 0xFF)); }
    let e_tlv = skb_put(skb, 1); *e_tlv = CFM_ENDE_TLV_TYPE;
    skb
}

unsafe fn ccm_frame_tx(skb: *mut sk_buff) { skb_reset_network_header(skb); dev_queue_xmit(skb); }

/* This function is called with the configured CC 'expected_interval' in order to drive CCM transmission when enabled. */
unsafe fn ccm_tx_work_expired(work: *mut work_struct) {
    let del_work = to_delayed_work(work);
    let mep = container_of!(del_work, br_cfm_mep, ccm_tx_dwork);
    if time_before_eq((*mep).ccm_tx_end, jiffies) { (*mep).cc_ccm_tx_info.period = 0; return; }
    let skb = ccm_frame_build(mep, &(*mep).cc_ccm_tx_info);
    if !skb.is_null() { ccm_frame_tx(skb); }
    let interval_us = interval_to_us((*mep).cc_config.exp_interval);
    queue_delayed_work(system_percpu_wq, &mut (*mep).ccm_tx_dwork, usecs_to_jiffies(interval_us));
}

/* This function is called with 1/4 of the configured CC 'expected_interval' in order to detect CCM defect after 3.25 interval. */
unsafe fn ccm_rx_work_expired(work: *mut work_struct) {
    let peer_mep = container_of!(to_delayed_work(work), br_cfm_peer_mep, ccm_rx_dwork);
    if (*peer_mep).ccm_rx_count_miss < 13 { (*peer_mep).ccm_rx_count_miss += 1; ccm_rx_timer_start(peer_mep); }
    else { (*peer_mep).cc_status.ccm_defect = true; rcu_read_lock(); let b_port = rcu_dereference((*(*peer_mep).mep).b_port); if !b_port.is_null() { br_cfm_notify(RTM_NEWLINK, b_port); } rcu_read_unlock(); }
}

unsafe fn ccm_tlv_extract(skb: *mut sk_buff, index: u32, peer_mep: *mut br_cfm_peer_mep) -> u32 {
    let mut e: u8 = 0; let e_tlv = skb_header_pointer(skb, index, 1, &mut e); if e_tlv.is_null() { return 0; }
    let mut s: u32 = 0; let s_tlv = skb_header_pointer(skb, index, 4, &mut s); if s_tlv.is_null() { return 0; }
    let h = ntohl(*(s_tlv as *const __be32));
    if h >> 24 == CFM_IF_STATUS_TLV_TYPE { (*peer_mep).cc_status.tlv_seen = true; (*peer_mep).cc_status.if_tlv_value = h & 0xFF; }
    if h >> 24 == CFM_PORT_STATUS_TLV_TYPE { (*peer_mep).cc_status.tlv_seen = true; (*peer_mep).cc_status.port_tlv_value = h & 0xFF; }
    /* The Sender ID TLV and Organization-Specific TLV are not handled. */
    ((h >> 8) & 0xFFFF) + 3
}

/* note: already called with rcu_read_lock */
unsafe fn br_cfm_frame_rx(port: *mut net_bridge_port, skb: *mut sk_buff) -> i32 {
    if (*port).state == BR_STATE_DISABLED { return 0; }
    let mut hdr_store: br_cfm_common_hdr = core::mem::zeroed();
    let hdr = skb_header_pointer(skb, 0, core::mem::size_of::<br_cfm_common_hdr>(), &mut hdr_store) as *const br_cfm_common_hdr;
    if hdr.is_null() { return 1; }
    let br = (*port).br; let mep = br_mep_find_ifindex(br, (*(*port).dev).ifindex);
    if mep.is_null() { return 0; }
    let mdlevel = (*hdr).mdlevel_version >> 5;
    if mdlevel > (*mep).config.mdlevel { return 0; }
    if (*hdr).mdlevel_version & 0x1F != 0 { (*mep).status.version_unexp_seen = true; return 1; }
    if mdlevel < (*mep).config.mdlevel { (*mep).status.rx_level_low_seen = true; return 1; }
    if (*hdr).opcode == BR_CFM_OPCODE_CCM {
        let mut maid_store: br_cfm_maid = core::mem::zeroed();
        let maid = skb_header_pointer(skb, CFM_CCM_PDU_MAID_OFFSET, core::mem::size_of::<br_cfm_maid>(), &mut maid_store) as *const br_cfm_maid;
        if maid.is_null() || memcmp((*maid).data.as_ptr(), (*mep).cc_config.exp_maid.data.as_ptr(), (*maid).data.len()) != 0 { return 1; }
        let mut mepid_store: __be16 = 0; let mepid = skb_header_pointer(skb, CFM_CCM_PDU_MEPID_OFFSET, 2, &mut mepid_store) as *const __be16;
        if mepid.is_null() { return 1; }
        let peer_mep = br_peer_mep_find(mep, ntohs(*mepid) as u32); if peer_mep.is_null() { return 1; }
        if (*mep).cc_config.exp_interval != pdu_to_interval((*hdr).flags & 0x07) { return 1; }
        if (*peer_mep).cc_status.ccm_defect { (*peer_mep).cc_status.ccm_defect = false; br_cfm_notify(RTM_NEWLINK, port); ccm_rx_timer_start(peer_mep); }
        (*peer_mep).cc_status.seen = true; (*peer_mep).ccm_rx_count_miss = 0; (*peer_mep).cc_status.rdi = (*hdr).flags & 0x80 != 0;
        let mut snumber_store: __be32 = 0; let snumber = skb_header_pointer(skb, CFM_CCM_PDU_SEQNR_OFFSET, 4, &mut snumber_store) as *const __be32;
        if snumber.is_null() { return 1; }
        if ntohl(*snumber) != (*mep).ccm_rx_snumber + 1 { (*peer_mep).cc_status.seq_unexp_seen = true; }
        (*mep).ccm_rx_snumber = ntohl(*snumber);
        let mut index = CFM_CCM_PDU_TLV_OFFSET; let mut max = 0; loop { let size = ccm_tlv_extract(skb, index, peer_mep); index += size; max += 1; if size == 0 || max >= 4 { break; } }
        return 1;
    }
    (*mep).status.opcode_unexp_seen = true; 1
}

static mut cfm_frame_type: br_frame_type = br_frame_type { r#type: cpu_to_be16(ETH_P_CFM), frame_handler: br_cfm_frame_rx };

pub unsafe fn br_cfm_mep_create(br: *mut net_bridge, instance: u32, create: *mut br_cfm_mep_create, extack: *mut netlink_ext_ack) -> i32 {
    ASSERT_RTNL!();
    if (*create).domain == BR_CFM_VLAN { NL_SET_ERR_MSG_MOD!(extack, "VLAN domain not supported"); return -EINVAL; }
    if (*create).domain != BR_CFM_PORT { NL_SET_ERR_MSG_MOD!(extack, "Invalid domain value"); return -EINVAL; }
    if (*create).direction == BR_CFM_MEP_DIRECTION_UP { NL_SET_ERR_MSG_MOD!(extack, "Up-MEP not supported"); return -EINVAL; }
    if (*create).direction != BR_CFM_MEP_DIRECTION_DOWN { NL_SET_ERR_MSG_MOD!(extack, "Invalid direction value"); return -EINVAL; }
    let p = br_mep_get_port(br, (*create).ifindex); if p.is_null() { NL_SET_ERR_MSG_MOD!(extack, "Port is not related to bridge"); return -EINVAL; }
    let mut mep = br_mep_find(br, instance); if !mep.is_null() { NL_SET_ERR_MSG_MOD!(extack, "MEP instance already exists"); return -EEXIST; }
    if !br_mep_find_ifindex(br, (*create).ifindex).is_null() { NL_SET_ERR_MSG_MOD!(extack, "Only one Port MEP on a port allowed"); return -EINVAL; }
    mep = kzalloc_obj!(br_cfm_mep); if mep.is_null() { return -ENOMEM; }
    (*mep).create = *create; (*mep).instance = instance; rcu_assign_pointer!((*mep).b_port, p); INIT_HLIST_HEAD!(&mut (*mep).peer_mep_list); INIT_DELAYED_WORK!(&mut (*mep).ccm_tx_dwork, ccm_tx_work_expired);
    if hlist_empty!(&(*br).mep_list) { br_add_frame(br, &raw mut cfm_frame_type); } hlist_add_tail_rcu!(&mut (*mep).head, &mut (*br).mep_list); 0
}

unsafe fn mep_delete_implementation(br: *mut net_bridge, mep: *mut br_cfm_mep) {
    ASSERT_RTNL!();
    let mut peer_mep: *mut br_cfm_peer_mep; let mut n_store: *mut hlist_node;
    hlist_for_each_entry_safe!(peer_mep, n_store, &mut (*mep).peer_mep_list, head, { disable_delayed_work_sync(&mut (*peer_mep).ccm_rx_dwork); hlist_del_rcu!(&mut (*peer_mep).head); kfree_rcu!(peer_mep, rcu); });
    cancel_delayed_work_sync(&mut (*mep).ccm_tx_dwork); RCU_INIT_POINTER!((*mep).b_port, core::ptr::null_mut()); hlist_del_rcu!(&mut (*mep).head); kfree_rcu!(mep, rcu); if hlist_empty!(&(*br).mep_list) { br_del_frame(br, &raw mut cfm_frame_type); }
}

pub unsafe fn br_cfm_mep_delete(br: *mut net_bridge, instance: u32, extack: *mut netlink_ext_ack) -> i32 { ASSERT_RTNL!(); let mep = br_mep_find(br, instance); if mep.is_null() { NL_SET_ERR_MSG_MOD!(extack, "MEP instance does not exists"); return -ENOENT; } mep_delete_implementation(br, mep); 0 }

pub unsafe fn br_cfm_mep_config_set(br: *mut net_bridge, instance: u32, config: *const br_cfm_mep_config, extack: *mut netlink_ext_ack) -> i32 { ASSERT_RTNL!(); let mep = br_mep_find(br, instance); if mep.is_null() { NL_SET_ERR_MSG_MOD!(extack, "MEP instance does not exists"); return -ENOENT; } (*mep).config = *config; 0 }

pub unsafe fn br_cfm_cc_config_set(br: *mut net_bridge, instance: u32, config: *const br_cfm_cc_config, extack: *mut netlink_ext_ack) -> i32 { ASSERT_RTNL!(); let mep = br_mep_find(br, instance); if mep.is_null() { NL_SET_ERR_MSG_MOD!(extack, "MEP instance does not exists"); return -ENOENT; } if memcmp(config, &(*mep).cc_config, core::mem::size_of::<br_cfm_cc_config>()) == 0 { return 0; } let mut peer_mep: *mut br_cfm_peer_mep; if (*config).enable && !(*mep).cc_config.enable { hlist_for_each_entry!(peer_mep, &mut (*mep).peer_mep_list, head, { cc_peer_enable(peer_mep); }); } if !(*config).enable && (*mep).cc_config.enable { hlist_for_each_entry!(peer_mep, &mut (*mep).peer_mep_list, head, { cc_peer_disable(peer_mep); }); } (*mep).cc_config = *config; (*mep).ccm_rx_snumber = 0; (*mep).ccm_tx_snumber = 1; 0 }

pub unsafe fn br_cfm_cc_peer_mep_add(br: *mut net_bridge, instance: u32, mepid: u32, extack: *mut netlink_ext_ack) -> i32 { ASSERT_RTNL!(); let mep = br_mep_find(br, instance); if mep.is_null() { NL_SET_ERR_MSG_MOD!(extack, "MEP instance does not exists"); return -ENOENT; } if !br_peer_mep_find(mep, mepid).is_null() { NL_SET_ERR_MSG_MOD!(extack, "Peer MEP-ID already exists"); return -EEXIST; } let peer_mep = kzalloc_obj!(br_cfm_peer_mep); if peer_mep.is_null() { return -ENOMEM; } (*peer_mep).mepid = mepid; (*peer_mep).mep = mep; INIT_DELAYED_WORK!(&mut (*peer_mep).ccm_rx_dwork, ccm_rx_work_expired); if (*mep).cc_config.enable { cc_peer_enable(peer_mep); } hlist_add_tail_rcu!(&mut (*peer_mep).head, &mut (*mep).peer_mep_list); 0 }

pub unsafe fn br_cfm_cc_peer_mep_remove(br: *mut net_bridge, instance: u32, mepid: u32, extack: *mut netlink_ext_ack) -> i32 { ASSERT_RTNL!(); let mep = br_mep_find(br, instance); if mep.is_null() { NL_SET_ERR_MSG_MOD!(extack, "MEP instance does not exists"); return -ENOENT; } let peer_mep = br_peer_mep_find(mep, mepid); if peer_mep.is_null() { NL_SET_ERR_MSG_MOD!(extack, "Peer MEP-ID does not exists"); return -ENOENT; } disable_delayed_work_sync(&mut (*peer_mep).ccm_rx_dwork); hlist_del_rcu!(&mut (*peer_mep).head); kfree_rcu!(peer_mep, rcu); 0 }

pub unsafe fn br_cfm_cc_rdi_set(br: *mut net_bridge, instance: u32, rdi: bool, extack: *mut netlink_ext_ack) -> i32 { ASSERT_RTNL!(); let mep = br_mep_find(br, instance); if mep.is_null() { NL_SET_ERR_MSG_MOD!(extack, "MEP instance does not exists"); return -ENOENT; } (*mep).rdi = rdi; 0 }

pub unsafe fn br_cfm_cc_ccm_tx(br: *mut net_bridge, instance: u32, tx_info: *const br_cfm_cc_ccm_tx_info, extack: *mut netlink_ext_ack) -> i32 { ASSERT_RTNL!(); let mep = br_mep_find(br, instance); if mep.is_null() { NL_SET_ERR_MSG_MOD!(extack, "MEP instance does not exists"); return -ENOENT; } if memcmp(tx_info, &(*mep).cc_ccm_tx_info, core::mem::size_of::<br_cfm_cc_ccm_tx_info>()) == 0 { if (*mep).cc_ccm_tx_info.period == 0 { return 0; } (*mep).ccm_tx_end = jiffies + usecs_to_jiffies((*tx_info).period * 1000000); return 0; } if (*tx_info).period == 0 && (*mep).cc_ccm_tx_info.period == 0 { (*mep).cc_ccm_tx_info = *tx_info; return 0; } if (*tx_info).period != 0 && (*mep).cc_ccm_tx_info.period != 0 { (*mep).ccm_tx_end = jiffies + usecs_to_jiffies((*tx_info).period * 1000000); (*mep).cc_ccm_tx_info = *tx_info; return 0; } if (*tx_info).period == 0 && (*mep).cc_ccm_tx_info.period != 0 { cancel_delayed_work_sync(&mut (*mep).ccm_tx_dwork); (*mep).cc_ccm_tx_info = *tx_info; return 0; } if interval_to_us((*mep).cc_config.exp_interval) == 0 { NL_SET_ERR_MSG_MOD!(extack, "Invalid CCM interval"); return -EINVAL; } (*mep).ccm_tx_end = jiffies + usecs_to_jiffies((*tx_info).period * 1000000); queue_delayed_work(system_percpu_wq, &mut (*mep).ccm_tx_dwork, 0); (*mep).cc_ccm_tx_info = *tx_info; 0 }

pub unsafe fn br_cfm_mep_count(br: *mut net_bridge, count: *mut u32) -> i32 { *count = 0; rcu_read_lock(); let mut mep: *mut br_cfm_mep; hlist_for_each_entry_rcu!(mep, &mut (*br).mep_list, head, { *count += 1; }); rcu_read_unlock(); 0 }
pub unsafe fn br_cfm_peer_mep_count(br: *mut net_bridge, count: *mut u32) -> i32 { *count = 0; rcu_read_lock(); let mut mep: *mut br_cfm_mep; let mut peer_mep: *mut br_cfm_peer_mep; hlist_for_each_entry_rcu!(mep, &mut (*br).mep_list, head, { hlist_for_each_entry_rcu!(peer_mep, &mut (*mep).peer_mep_list, head, { *count += 1; }); }); rcu_read_unlock(); 0 }
pub unsafe fn br_cfm_created(br: *mut net_bridge) -> bool { !hlist_empty!(&(*br).mep_list) }

/* Deletes the CFM instances on a specific bridge port */
pub unsafe fn br_cfm_port_del(br: *mut net_bridge, port: *mut net_bridge_port) { ASSERT_RTNL!(); let mut mep: *mut br_cfm_mep; let mut n_store: *mut hlist_node; hlist_for_each_entry_safe!(mep, n_store, &mut (*br).mep_list, head, { if (*mep).create.ifindex == (*(*port).dev).ifindex { mep_delete_implementation(br, mep); } }); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
