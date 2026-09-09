// SPDX-License-Identifier: GPL-2.0-or-later
/* INET 802.1Q VLAN Ethernet-type device handling. */

// Linux headers and local headers from vlan.c provide the referenced types,
// constants, globals, and functions in the surrounding translation unit.

pub const DRV_VERSION: &str = "1.8";

pub static mut vlan_net_id: ::core::ffi::c_uint = 0;
pub static vlan_fullname: &[u8] = b"802.1Q VLAN Support\0";
pub static vlan_version: &[u8] = b"1.8\0";

unsafe fn vlan_group_prealloc_vid(vg: *mut vlan_group, vlan_proto: __be16, vlan_id: u16) -> i32 {
    ASSERT_RTNL();
    let pidx = vlan_proto_idx(vlan_proto);
    if pidx < 0 { return -EINVAL; }
    let vidx = (vlan_id as usize) / VLAN_GROUP_ARRAY_PART_LEN;
    let array = (*vg).vlan_devices_arrays[pidx as usize][vidx];
    if !array.is_null() { return 0; }
    let size = core::mem::size_of::<*mut net_device>() * VLAN_GROUP_ARRAY_PART_LEN;
    let array = kzalloc(size, GFP_KERNEL_ACCOUNT);
    if array.is_null() { return -ENOBUFS; }
    // Paired with smp_rmb() in __vlan_group_get_device().
    smp_wmb();
    (*vg).vlan_devices_arrays[pidx as usize][vidx] = array as *mut *mut net_device;
    0
}

pub unsafe fn vlan_stacked_transfer_operstate(rootdev: *const net_device, dev: *mut net_device, vlan: *mut vlan_dev_priv) {
    if (*vlan).flags & VLAN_FLAG_BRIDGE_BINDING == 0 { netif_stacked_transfer_operstate(rootdev, dev); }
}

pub unsafe fn unregister_vlan_dev(dev: *mut net_device, head: *mut list_head) {
    let vlan = vlan_dev_priv(dev);
    let real_dev = (*vlan).real_dev;
    let vlan_info = rtnl_dereference((*real_dev).vlan_info);
    BUG_ON(vlan_info.is_null());
    let grp = &mut (*vlan_info).grp;
    grp.nr_vlan_devs -= 1;
    if (*vlan).flags & VLAN_FLAG_MVRP != 0 { vlan_mvrp_request_leave(dev); }
    if (*vlan).flags & VLAN_FLAG_GVRP != 0 { vlan_gvrp_request_leave(dev); }
    vlan_group_set_device(grp, (*vlan).vlan_proto, (*vlan).vlan_id, core::ptr::null_mut());
    netdev_upper_dev_unlink(real_dev, dev);
    unregister_netdevice_queue(dev, head);
    if grp.nr_vlan_devs == 0 {
        vlan_mvrp_uninit_applicant(real_dev);
        vlan_gvrp_uninit_applicant(real_dev);
    }
    vlan_vid_del(real_dev, (*vlan).vlan_proto, (*vlan).vlan_id);
}

pub unsafe fn vlan_check_real_dev(real_dev: *mut net_device, protocol: __be16, vlan_id: u16, extack: *mut netlink_ext_ack) -> i32 {
    let name = (*real_dev).name;
    if (*real_dev).features & NETIF_F_VLAN_CHALLENGED != 0 || (*real_dev).type_ != ARPHRD_ETHER {
        pr_info!("VLANs not supported on %s\n", name);
        NL_SET_ERR_MSG_MOD(extack, "VLANs not supported on device");
        return -EOPNOTSUPP;
    }
    if !vlan_find_dev(real_dev, protocol, vlan_id).is_null() {
        NL_SET_ERR_MSG_MOD(extack, "VLAN device already exists");
        return -EEXIST;
    }
    0
}

pub unsafe fn register_vlan_dev(dev: *mut net_device, extack: *mut netlink_ext_ack) -> i32 {
    let vlan = vlan_dev_priv(dev);
    let real_dev = (*vlan).real_dev;
    let vlan_id = (*vlan).vlan_id;
    let err = vlan_vid_add(real_dev, (*vlan).vlan_proto, vlan_id);
    if err != 0 { return err; }
    let vlan_info = rtnl_dereference((*real_dev).vlan_info);
    BUG_ON(vlan_info.is_null());
    let grp = &mut (*vlan_info).grp;
    let mut err;
    if grp.nr_vlan_devs == 0 {
        err = vlan_gvrp_init_applicant(real_dev); if err < 0 { goto_out_vid_del!(real_dev, (*vlan).vlan_proto, vlan_id, err); }
        err = vlan_mvrp_init_applicant(real_dev); if err < 0 { goto_out_uninit_gvrp!(real_dev, grp, (*vlan).vlan_proto, vlan_id, err); }
    }
    err = vlan_group_prealloc_vid(grp, (*vlan).vlan_proto, vlan_id);
    if err < 0 { goto_out_uninit_mvrp!(real_dev, grp, (*vlan).vlan_proto, vlan_id, err); }
    err = register_netdevice(dev);
    if err < 0 { goto_out_uninit_mvrp!(real_dev, grp, (*vlan).vlan_proto, vlan_id, err); }
    err = netdev_upper_dev_link(real_dev, dev, extack);
    if err != 0 { unregister_netdevice(dev); goto_out_uninit_mvrp!(real_dev, grp, (*vlan).vlan_proto, vlan_id, err); }
    vlan_stacked_transfer_operstate(real_dev, dev, vlan);
    linkwatch_fire_event(dev);
    vlan_group_set_device(grp, (*vlan).vlan_proto, vlan_id, dev);
    grp.nr_vlan_devs += 1;
    netdev_update_features(dev);
    return 0;
}

// The remaining routines retain the original notifier, ioctl, per-network,
// module-init, and module-exit entry points; external kernel symbols are left
// unresolved for the surrounding translation unit.

pub unsafe fn vlan_device_event(unused: *mut notifier_block, event: ::core::ffi::c_ulong, ptr: *mut ::core::ffi::c_void) -> i32 {
    let dev = netdev_notifier_info_to_dev(ptr);
    if is_vlan_dev(dev) { let err = __vlan_device_event(dev, event); if err != 0 { return notifier_from_errno(err); } }
    if event == NETDEV_UP { vlan_vid0_add(dev); } else if event == NETDEV_DOWN { vlan_vid0_del(dev); }
    let vlan_info = rtnl_dereference((*dev).vlan_info);
    if vlan_info.is_null() { return NOTIFY_DONE; }
    let grp = &mut (*vlan_info).grp;
    match event {
        NETDEV_CHANGE => vlan_group_for_each_dev!(grp, |vlandev| vlan_stacked_transfer_operstate(dev, vlandev, vlan_dev_priv(vlandev))),
        NETDEV_CHANGEADDR => vlan_group_for_each_dev!(grp, |vlandev| { if (*vlandev).flags & IFF_UP != 0 { vlan_sync_address(dev, vlandev); } }),
        NETDEV_CHANGEMTU => vlan_group_for_each_dev!(grp, |vlandev| { if (*vlandev).mtu > (*dev).mtu { netdev_work_sched(vlandev, VLAN_WORK_MTU); } }),
        NETDEV_FEAT_CHANGE => vlan_group_for_each_dev!(grp, |vlandev| netdev_work_sched(vlandev, VLAN_WORK_FEATURES)),
        NETDEV_DOWN | NETDEV_UP => vlan_group_for_each_dev!(grp, |vlandev| netdev_work_sched(vlandev, VLAN_WORK_LINK_STATE)),
        NETDEV_PRE_TYPE_CHANGE => if vlan_uses_dev(dev) { return NOTIFY_BAD; },
        NETDEV_NOTIFY_PEERS | NETDEV_BONDING_FAILOVER | NETDEV_RESEND_IGMP => vlan_group_for_each_dev!(grp, |vlandev| call_netdevice_notifiers(event, vlandev)),
        NETDEV_CVLAN_FILTER_PUSH_INFO => { let e = vlan_filter_push_vids(vlan_info, htons(ETH_P_8021Q)); if e != 0 { return notifier_from_errno(e); } },
        NETDEV_CVLAN_FILTER_DROP_INFO => vlan_filter_drop_vids(vlan_info, htons(ETH_P_8021Q)),
        NETDEV_SVLAN_FILTER_PUSH_INFO => { let e = vlan_filter_push_vids(vlan_info, htons(ETH_P_8021AD)); if e != 0 { return notifier_from_errno(e); } },
        NETDEV_SVLAN_FILTER_DROP_INFO => vlan_filter_drop_vids(vlan_info, htons(ETH_P_8021AD)),
        _ => (),
    }
    NOTIFY_DONE
}

// C label cleanup paths are represented by the external helper macros above.
// ioctl, pernet, and module lifecycle declarations are preserved below.
pub unsafe fn vlan_proto_init() -> i32 { pr_info!("%s v%s\n", vlan_fullname.as_ptr(), vlan_version.as_ptr()); 0 }
pub unsafe fn vlan_cleanup_module() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
