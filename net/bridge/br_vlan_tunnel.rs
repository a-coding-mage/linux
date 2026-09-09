// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Bridge per vlan tunnel port dst_metadata handling code
 *
 * Authors:
 * Roopa Prabhu <roopa@cumulusnetworks.com>
 */

// Linux dependencies supplied by the surrounding translation unit.

unsafe fn br_vlan_tunid_cmp(
    arg: *mut rhashtable_compare_arg,
    ptr: *const core::ffi::c_void,
) -> i32 {
    let vle = ptr as *const net_bridge_vlan;
    let tunid = *( (*arg).key as *const __be64 );

    ((*vle).tinfo.tunnel_id != tunid) as i32
}

static br_vlan_tunnel_rht_params: rhashtable_params = rhashtable_params {
    head_offset: core::mem::offset_of!(net_bridge_vlan, tnode),
    key_offset: core::mem::offset_of!(net_bridge_vlan, tinfo) +
        core::mem::offset_of!(net_bridge_vlan_tinfo, tunnel_id),
    key_len: core::mem::size_of::<__be64>(),
    nelem_hint: 3,
    obj_cmpfn: Some(br_vlan_tunid_cmp),
    automatic_shrinking: true,
};

unsafe fn br_vlan_tunnel_lookup(
    tbl: *mut rhashtable,
    tunnel_id: __be64,
) -> *mut net_bridge_vlan {
    rhashtable_lookup_fast(tbl, &tunnel_id as *const __be64 as *const core::ffi::c_void,
                           &br_vlan_tunnel_rht_params)
}

unsafe fn vlan_tunnel_info_release(vlan: *mut net_bridge_vlan) {
    let tdst = rtnl_dereference((*vlan).tinfo.tunnel_dst);

    WRITE_ONCE((*vlan).tinfo.tunnel_id, 0);
    RCU_INIT_POINTER((*vlan).tinfo.tunnel_dst, core::ptr::null_mut());
    dst_release(&mut (*tdst).dst);
}

pub unsafe fn vlan_tunnel_info_del(vg: *mut net_bridge_vlan_group,
                                   vlan: *mut net_bridge_vlan) {
    if !rcu_access_pointer((*vlan).tinfo.tunnel_dst) {
        return;
    }
    rhashtable_remove_fast(&mut (*vg).tunnel_hash, &mut (*vlan).tnode,
                           &br_vlan_tunnel_rht_params);
    vlan_tunnel_info_release(vlan);
}

unsafe fn __vlan_tunnel_info_add(vg: *mut net_bridge_vlan_group,
                                 vlan: *mut net_bridge_vlan,
                                 tun_id: u32) -> i32 {
    let mut metadata = rtnl_dereference((*vlan).tinfo.tunnel_dst);
    let key: __be64 = key32_to_tunnel_id(cpu_to_be32(tun_id));
    let mut flags: ip_tunnel_flags = core::mem::zeroed();
    let err: i32;

    if !metadata.is_null() {
        return -EEXIST;
    }

    __set_bit(IP_TUNNEL_KEY_BIT, &mut flags);
    metadata = __ip_tun_set_dst(0, 0, 0, 0, 0, flags, key, 0);
    if metadata.is_null() {
        return -EINVAL;
    }

    (*metadata).u.tun_info.mode |= IP_TUNNEL_INFO_TX | IP_TUNNEL_INFO_BRIDGE;
    rcu_assign_pointer((*vlan).tinfo.tunnel_dst, metadata);
    WRITE_ONCE((*vlan).tinfo.tunnel_id, key);

    err = rhashtable_lookup_insert_fast(&mut (*vg).tunnel_hash, &mut (*vlan).tnode,
                                        &br_vlan_tunnel_rht_params);
    if err == 0 {
        return 0;
    }

    vlan_tunnel_info_release(vlan);
    err
}

/* Must be protected by RTNL.
 * Must be called with vid in range from 1 to 4094 inclusive.
 */
pub unsafe fn nbp_vlan_tunnel_info_add(port: *const net_bridge_port,
                                       vid: u16, tun_id: u32) -> i32 {
    ASSERT_RTNL();

    let vg = nbp_vlan_group(port);
    let vlan = br_vlan_find(vg, vid);
    if vlan.is_null() {
        return -EINVAL;
    }

    __vlan_tunnel_info_add(vg, vlan, tun_id)
}

/* Must be protected by RTNL.
 * Must be called with vid in range from 1 to 4094 inclusive.
 */
pub unsafe fn nbp_vlan_tunnel_info_delete(port: *const net_bridge_port,
                                          vid: u16) -> i32 {
    ASSERT_RTNL();

    let vg = nbp_vlan_group(port);
    let v = br_vlan_find(vg, vid);
    if v.is_null() {
        return -ENOENT;
    }

    vlan_tunnel_info_del(vg, v);
    0
}

unsafe fn __vlan_tunnel_info_flush(vg: *mut net_bridge_vlan_group) {
    let mut vlan: *mut net_bridge_vlan;
    let mut tmp: *mut net_bridge_vlan;

    list_for_each_entry_safe!(vlan, tmp, &mut (*vg).vlan_list, vlist, {
        vlan_tunnel_info_del(vg, vlan);
    });
}

pub unsafe fn nbp_vlan_tunnel_info_flush(port: *mut net_bridge_port) {
    ASSERT_RTNL();

    let vg = nbp_vlan_group(port);
    __vlan_tunnel_info_flush(vg);
}

pub unsafe fn vlan_tunnel_init(vg: *mut net_bridge_vlan_group) -> i32 {
    rhashtable_init(&mut (*vg).tunnel_hash, &br_vlan_tunnel_rht_params)
}

pub unsafe fn vlan_tunnel_deinit(vg: *mut net_bridge_vlan_group) {
    rhashtable_destroy(&mut (*vg).tunnel_hash);
}

pub unsafe fn br_handle_ingress_vlan_tunnel(
    skb: *mut sk_buff,
    p: *mut net_bridge_port,
    vg: *mut net_bridge_vlan_group,
) {
    let tinfo = skb_tunnel_info(skb);
    if vg.is_null() || tinfo.is_null() {
        return;
    }

    /* if already tagged, ignore */
    if skb_vlan_tagged(skb) {
        return;
    }

    /* lookup vid, given tunnel id */
    let vlan = br_vlan_tunnel_lookup(&mut (*vg).tunnel_hash, (*tinfo).key.tun_id);
    if vlan.is_null() {
        return;
    }

    skb_dst_drop(skb);
    __vlan_hwaccel_put_tag(skb, (*(*p).br).vlan_proto, (*vlan).vid);
}

pub unsafe fn br_handle_egress_vlan_tunnel(
    skb: *mut sk_buff,
    vlan: *mut net_bridge_vlan,
) -> i32 {
    let mut flags: ip_tunnel_flags = core::mem::zeroed();
    let tunnel_dst: *mut metadata_dst;
    let tunnel_id: __be64;

    if vlan.is_null() {
        return 0;
    }

    tunnel_id = READ_ONCE((*vlan).tinfo.tunnel_id);
    if tunnel_id == 0 || unlikely(!skb_vlan_tag_present(skb)) {
        return 0;
    }

    skb_dst_drop(skb);
    /* For 802.1ad (QinQ), skb_vlan_pop() incorrectly moves the C-VLAN
     * from payload to hwaccel after clearing S-VLAN. We only need to
     * clear the hwaccel S-VLAN; the C-VLAN must stay in payload for
     * correct VXLAN encapsulation. This is also correct for 802.1Q
     * where no C-VLAN exists in payload.
     */
    __vlan_hwaccel_clear_tag(skb);

    if (*BR_INPUT_SKB_CB(skb).backup_nhid != 0 {
        __set_bit(IP_TUNNEL_KEY_BIT, &mut flags);
        tunnel_dst = __ip_tun_set_dst(0, 0, 0, 0, 0, flags, tunnel_id, 0);
        if tunnel_dst.is_null() {
            return -ENOMEM;
        }

        (*tunnel_dst).u.tun_info.mode |= IP_TUNNEL_INFO_TX | IP_TUNNEL_INFO_BRIDGE;
        (*tunnel_dst).u.tun_info.key.nhid = (*BR_INPUT_SKB_CB(skb)).backup_nhid;
        skb_dst_set(skb, &mut (*tunnel_dst).dst);

        return 0;
    }

    tunnel_dst = rcu_dereference((*vlan).tinfo.tunnel_dst);
    if !tunnel_dst.is_null() && dst_hold_safe(&mut (*tunnel_dst).dst) {
        skb_dst_set(skb, &mut (*tunnel_dst).dst);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
