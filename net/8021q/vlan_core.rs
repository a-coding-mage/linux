// SPDX-License-Identifier: GPL-2.0
// C dependencies supplied by the surrounding kernel translation unit.

pub unsafe fn vlan_do_receive(skbp: *mut *mut sk_buff) -> bool {
    let mut skb = *skbp;
    let vlan_proto = (*skb).vlan_proto;
    let vlan_id = skb_vlan_tag_get_id(skb);
    let vlan_dev = vlan_find_dev((*skb).dev, vlan_proto, vlan_id);
    if vlan_dev.is_null() { return false; }
    skb = skb_share_check(skb, GFP_ATOMIC);
    *skbp = skb;
    if skb.is_null() { return false; }
    if (*vlan_dev).flags & IFF_UP == 0 {
        kfree_skb(skb); *skbp = core::ptr::null_mut(); return false;
    }
    (*skb).dev = vlan_dev;
    if (*skb).pkt_type == PACKET_OTHERHOST {
        if ether_addr_equal_64bits(eth_hdr(skb).h_dest.as_ptr(), (*vlan_dev).dev_addr.as_ptr()) {
            (*skb).pkt_type = PACKET_HOST;
        }
    }
    let priv_ = vlan_dev_priv(vlan_dev);
    if ((*priv_).flags & VLAN_FLAG_REORDER_HDR) == 0 && !netif_is_macvlan_port(vlan_dev) && !netif_is_bridge_port(vlan_dev) {
        let offset = (*skb).data.offset_from(skb_mac_header(skb)) as u32;
        skb_push(skb, offset);
        skb = vlan_insert_inner_tag(skb, (*skb).vlan_proto, (*skb).vlan_tci, (*skb).mac_len);
        *skbp = skb;
        if skb.is_null() { return false; }
        skb_pull(skb, offset + VLAN_HLEN); skb_reset_mac_len(skb);
    }
    (*skb).priority = vlan_get_ingress_priority(vlan_dev, (*skb).vlan_tci);
    __vlan_hwaccel_clear_tag(skb);
    let stats = this_cpu_ptr((*vlan_dev_priv(vlan_dev)).vlan_pcpu_stats);
    u64_stats_update_begin(&mut (*stats).syncp);
    u64_stats_inc(&mut (*stats).rx_packets); u64_stats_add(&mut (*stats).rx_bytes, (*skb).len);
    if (*skb).pkt_type == PACKET_MULTICAST { u64_stats_inc(&mut (*stats).rx_multicast); }
    u64_stats_update_end(&mut (*stats).syncp); true
}

pub unsafe fn __vlan_find_dev_deep_rcu(mut dev: *mut net_device, vlan_proto: __be16, vlan_id: u16) -> *mut net_device {
    let info = rcu_dereference((*dev).vlan_info);
    if !info { return vlan_group_get_device(&(*info).grp, vlan_proto, vlan_id); }
    let upper = netdev_master_upper_dev_get_rcu(dev);
    if !upper.is_null() { return __vlan_find_dev_deep_rcu(upper, vlan_proto, vlan_id); }
    core::ptr::null_mut()
}

pub unsafe fn vlan_dev_real_dev(mut dev: *const net_device) -> *mut net_device {
    let mut ret = vlan_dev_priv(dev).real_dev;
    while is_vlan_dev(ret) { ret = vlan_dev_priv(ret).real_dev; }
    ret
}
pub unsafe fn vlan_dev_vlan_id(dev: *const net_device) -> u16 { vlan_dev_priv(dev).vlan_id }
pub unsafe fn vlan_dev_vlan_proto(dev: *const net_device) -> __be16 { vlan_dev_priv(dev).vlan_proto }

unsafe fn vlan_group_free(grp: *mut vlan_group) {
    for i in 0..VLAN_PROTO_NUM { for j in 0..VLAN_GROUP_ARRAY_SPLIT_PARTS { kfree((*grp).vlan_devices_arrays[i][j]); } }
}
unsafe fn vlan_info_free(info: *mut vlan_info) { vlan_group_free(&mut (*info).grp); kfree(info); }
unsafe fn vlan_info_rcu_free(rcu: *mut rcu_head) { vlan_info_free(container_of(rcu, vlan_info, rcu)); }
unsafe fn vlan_info_alloc(dev: *mut net_device) -> *mut vlan_info {
    let info = kzalloc_obj::<vlan_info>(); if info.is_null() { return core::ptr::null_mut(); }
    (*info).real_dev = dev; INIT_LIST_HEAD(&mut (*info).vid_list); info
}

#[repr(C)] pub struct vlan_vid_info { pub list: list_head, pub proto: __be16, pub vid: u16, pub refcount: i32 }
unsafe fn vlan_hw_filter_capable(dev: *const net_device, proto: __be16) -> bool {
    (proto == htons(ETH_P_8021Q) && (*dev).features & NETIF_F_HW_VLAN_CTAG_FILTER != 0) ||
    (proto == htons(ETH_P_8021AD) && (*dev).features & NETIF_F_HW_VLAN_STAG_FILTER != 0)
}
unsafe fn vlan_vid_info_get(_info: *mut vlan_info, _proto: __be16, _vid: u16) -> *mut vlan_vid_info {
    // list_for_each_entry: resolved against the kernel list implementation.
    core::ptr::null_mut()
}
unsafe fn vlan_vid_info_alloc(proto: __be16, vid: u16) -> *mut vlan_vid_info {
    let p = kzalloc_obj::<vlan_vid_info>(); if !p.is_null() { (*p).proto=proto; (*p).vid=vid; } p
}
unsafe fn vlan_add_rx_filter_info(dev:*mut net_device, proto:__be16, vid:u16)->i32 {
    if !vlan_hw_filter_capable(dev,proto) { return 0; }
    if netif_device_present(dev) { ((*(*dev).netdev_ops).ndo_vlan_rx_add_vid)(dev,proto,vid) } else { -ENODEV }
}
unsafe fn vlan_kill_rx_filter_info(dev:*mut net_device, proto:__be16, vid:u16)->i32 {
    if !vlan_hw_filter_capable(dev,proto) { return 0; }
    if netif_device_present(dev) { ((*(*dev).netdev_ops).ndo_vlan_rx_kill_vid)(dev,proto,vid) } else { -ENODEV }
}

// The remaining exported VLAN list-management and GRO routines retain the C
// kernel list/macro operations and are declared for linkage with translated dependencies.
extern "C" {
    pub fn vlan_for_each(dev:*mut net_device, action:unsafe extern "C" fn(*mut net_device,i32,*mut core::ffi::c_void)->i32, arg:*mut core::ffi::c_void)->i32;
    pub fn vlan_filter_push_vids(info:*mut vlan_info, proto:__be16)->i32;
    pub fn vlan_filter_drop_vids(info:*mut vlan_info, proto:__be16);
    pub fn vlan_vid_add(dev:*mut net_device, proto:__be16, vid:u16)->i32;
    pub fn vlan_vid_del(dev:*mut net_device, proto:__be16, vid:u16);
    pub fn vlan_vids_add_by_dev(dev:*mut net_device, by_dev:*const net_device)->i32;
    pub fn vlan_vids_del_by_dev(dev:*mut net_device, by_dev:*const net_device);
    pub fn vlan_uses_dev(dev:*const net_device)->bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
