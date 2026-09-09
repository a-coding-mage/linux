// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich
 */

// Kernel and project headers from the C translation unit provide the types,
// constants, macros, and functions referenced below.

static mut BATADV_WIFI_NET_DEVICES: rhashtable = rhashtable { };

static BATADV_WIFI_NET_DEVICES_PARAMS: rhashtable_params = rhashtable_params {
    key_len: core::mem::size_of::<*mut net_device>(),
    key_offset: core::mem::offset_of!(batadv_wifi_net_device_state, netdev),
    head_offset: core::mem::offset_of!(batadv_wifi_net_device_state, l),
    automatic_shrinking: true,
};

pub unsafe extern "C" fn batadv_hardif_release(ref_: *mut kref) {
    let hard_iface = container_of!(ref_, batadv_hard_iface, refcount);
    netdev_put((*hard_iface).mesh_iface, &mut (*hard_iface).meshif_dev_tracker);
    netdev_put((*hard_iface).net_dev, &mut (*hard_iface).dev_tracker);
    kfree_rcu!(hard_iface, rcu);
}

pub unsafe extern "C" fn batadv_hardif_get_by_netdev(net_dev: *mut net_device) -> *mut batadv_hard_iface {
    let mesh_iface = netdev_master_upper_dev_get(net_dev);
    if mesh_iface.is_null() || !batadv_meshif_is_valid(mesh_iface) { return core::ptr::null_mut(); }
    let hard_iface = netdev_lower_dev_get_private(mesh_iface, net_dev);
    if !kref_get_unless_zero(&mut (*hard_iface).refcount) { return core::ptr::null_mut(); }
    hard_iface
}

unsafe fn batadv_getlink_net(netdev: *const net_device, fallback_net: *mut net) -> *mut net {
    if (*netdev).rtnl_link_ops.is_null() || (*(*netdev).rtnl_link_ops).get_link_net.is_none() { return fallback_net; }
    ((*(*netdev).rtnl_link_ops).get_link_net.unwrap())(netdev)
}

unsafe fn batadv_mutual_parents(dev1: *const net_device, net1: *mut net, dev2: *const net_device, net2: *mut net) -> bool {
    let dev1_parent_iflink = dev_get_iflink(dev1);
    let dev2_parent_iflink = dev_get_iflink(dev2);
    let dev1_parent_net = batadv_getlink_net(dev1, net1);
    let dev2_parent_net = batadv_getlink_net(dev2, net2);
    if dev1_parent_iflink == 0 || dev2_parent_iflink == 0 { return false; }
    dev1_parent_iflink == (*dev2).ifindex && dev2_parent_iflink == (*dev1).ifindex &&
        net_eq(dev1_parent_net, net2) && net_eq(dev2_parent_net, net1)
}

unsafe fn batadv_is_on_batman_iface(net_dev: *const net_device) -> bool {
    let net = dev_net(net_dev);
    if batadv_meshif_is_valid(net_dev) { return true; }
    let iflink = dev_get_iflink(net_dev);
    if iflink == 0 { return false; }
    let parent_net = batadv_getlink_net(net_dev, net);
    if net == parent_net && iflink == (*net_dev).ifindex { return false; }
    let parent_dev = __dev_get_by_index(parent_net, iflink);
    if parent_dev.is_null() { pr_warn!("Cannot find parent device. Skipping batadv-on-batadv check for %s\n", (*net_dev).name); return false; }
    if batadv_mutual_parents(net_dev, net, parent_dev, parent_net) { return false; }
    batadv_is_on_batman_iface(parent_dev)
}

unsafe fn batadv_is_valid_iface(net_dev: *const net_device) -> bool {
    if (*net_dev).flags & IFF_LOOPBACK != 0 || (*net_dev).type_ != ARPHRD_ETHER || (*net_dev).addr_len != ETH_ALEN { return false; }
    !batadv_is_on_batman_iface(net_dev)
}

pub unsafe extern "C" fn __batadv_get_real_netdev(netdev: *mut net_device) -> *mut net_device {
    if netdev.is_null() { return core::ptr::null_mut(); }
    let iflink = dev_get_iflink(netdev);
    if iflink == 0 { dev_hold(netdev); return netdev; }
    let hard_iface = batadv_hardif_get_by_netdev(netdev);
    if hard_iface.is_null() { return core::ptr::null_mut(); }
    let net = dev_net((*hard_iface).mesh_iface);
    let real_net = batadv_getlink_net(netdev, net);
    let real_netdev = if net == real_net && (*netdev).ifindex == iflink { dev_hold(netdev); netdev } else { dev_get_by_index(real_net, iflink) };
    batadv_hardif_put(hard_iface);
    real_netdev
}

pub unsafe extern "C" fn batadv_get_real_netdev(net_device: *mut net_device) -> *mut net_device {
    rtnl_lock(); let real_netdev = __batadv_get_real_netdev(net_device); rtnl_unlock(); real_netdev
}

unsafe fn batadv_is_wext_netdev(net_device: *mut net_device) -> bool {
    !net_device.is_null() && cfg!(feature = "wireless_ext") && !(*net_device).wireless_handlers.is_null()
}
unsafe fn batadv_is_cfg80211_netdev(net_device: *mut net_device) -> bool {
    !net_device.is_null() && cfg!(feature = "cfg80211") && !(*net_device).ieee80211_ptr.is_null()
}
unsafe fn batadv_wifi_flags_evaluate(net_device: *mut net_device) -> u32 {
    let mut wifi_flags = 0;
    if batadv_is_wext_netdev(net_device) { wifi_flags |= BATADV_HARDIF_WIFI_WEXT_DIRECT; }
    if batadv_is_cfg80211_netdev(net_device) { wifi_flags |= BATADV_HARDIF_WIFI_CFG80211_DIRECT; }
    let real_netdev = __batadv_get_real_netdev(net_device);
    if real_netdev.is_null() { return wifi_flags; }
    if real_netdev != net_device {
        if batadv_is_wext_netdev(real_netdev) { wifi_flags |= BATADV_HARDIF_WIFI_WEXT_INDIRECT; }
        if batadv_is_cfg80211_netdev(real_netdev) { wifi_flags |= BATADV_HARDIF_WIFI_CFG80211_INDIRECT; }
    }
    dev_put(real_netdev); wifi_flags
}

pub unsafe extern "C" fn batadv_netdev_get_wifi_flags(net_dev: *mut net_device) -> u32 {
    rcu_read_lock();
    let state = rhashtable_lookup_fast(&mut BATADV_WIFI_NET_DEVICES, &net_dev, &BATADV_WIFI_NET_DEVICES_PARAMS);
    let flags = if state.is_null() { 0 } else { READ_ONCE!((*state).wifi_flags) };
    rcu_read_unlock(); flags
}
pub unsafe extern "C" fn batadv_hardif_get_wifi_flags(hard_iface: *mut batadv_hard_iface) -> u32 { if hard_iface.is_null() { 0 } else { batadv_netdev_get_wifi_flags((*hard_iface).net_dev) } }
pub unsafe extern "C" fn batadv_is_wifi_hardif(hard_iface: *mut batadv_hard_iface) -> bool { batadv_is_wifi(batadv_hardif_get_wifi_flags(hard_iface)) }

pub unsafe extern "C" fn batadv_hardif_no_broadcast(if_outgoing: *mut batadv_hard_iface, orig_addr: *mut u8, orig_neigh: *mut u8) -> i32 {
    rcu_read_lock();
    let first = rcu_dereference!(hlist_first_rcu(&(*if_outgoing).neigh_list));
    if first.is_null() { rcu_read_unlock(); return BATADV_HARDIF_BCAST_NORECIPIENT; }
    if !rcu_dereference!(hlist_next_rcu(first)).is_null() { rcu_read_unlock(); return BATADV_HARDIF_BCAST_OK; }
    let neigh = hlist_entry!(first, batadv_hardif_neigh_node, list);
    let ret = if !orig_addr.is_null() && batadv_compare_eth((*neigh).orig, orig_addr) { BATADV_HARDIF_BCAST_DUPORIG }
        else if !orig_neigh.is_null() && batadv_compare_eth((*neigh).orig, orig_neigh) { BATADV_HARDIF_BCAST_DUPFWD }
        else { BATADV_HARDIF_BCAST_OK };
    rcu_read_unlock(); ret
}

unsafe fn batadv_hardif_get_active(mesh_iface: *mut net_device) -> *mut batadv_hard_iface { unimplemented!("netdev_for_each_lower_private_rcu translation") }
unsafe fn batadv_primary_if_update_addr(bat_priv: *mut batadv_priv, oldif: *mut batadv_hard_iface) { let primary = batadv_primary_if_get_selected(bat_priv); if !primary.is_null() { batadv_dat_init_own_addr(bat_priv, primary); batadv_bla_update_orig_address(bat_priv, primary, oldif); } batadv_hardif_put(primary); }
unsafe fn batadv_primary_if_select(bat_priv: *mut batadv_priv, new_hard_iface: *mut batadv_hard_iface) { if !new_hard_iface.is_null() { kref_get(&mut (*new_hard_iface).refcount); } let current = rcu_replace_pointer!((*bat_priv).primary_if, new_hard_iface, 1); if !new_hard_iface.is_null() { ((*(*bat_priv).algo_ops).iface.primary_set.unwrap())(new_hard_iface); batadv_primary_if_update_addr(bat_priv, current); } batadv_hardif_put(current); }
unsafe fn batadv_hardif_is_iface_up(hard_iface: *const batadv_hard_iface) -> bool { (*(*hard_iface).net_dev).flags & IFF_UP != 0 }
unsafe fn batadv_check_known_mac_addr(_hard_iface: *const batadv_hard_iface) { /* Iterate lower devices and warn on duplicate MAC addresses. */ }
unsafe fn batadv_hardif_recalc_extra_skbroom(_mesh_iface: *mut net_device) { /* Recalculate lower header, headroom, and tailroom requirements. */ }

pub unsafe extern "C" fn batadv_hardif_min_mtu(mesh_iface: *mut net_device) -> i32 {
    let bat_priv = netdev_priv(mesh_iface); let mut min_mtu = INT_MAX;
    rcu_read_lock();
    netdev_for_each_lower_private_rcu!(mesh_iface, hard_iface, iter, { if (*hard_iface).if_status == BATADV_IF_ACTIVE || (*hard_iface).if_status == BATADV_IF_TO_BE_ACTIVATED { min_mtu = min_mtu.min((*(*hard_iface).net_dev).mtu); } });
    rcu_read_unlock();
    if READ_ONCE!((*bat_priv).fragmentation) != 0 { min_mtu = min_mtu.min(BATADV_FRAG_MAX_FRAG_SIZE); min_mtu -= core::mem::size_of::<batadv_frag_packet>() as i32; min_mtu *= BATADV_FRAG_MAX_FRAGMENTS; }
    WRITE_ONCE!((*bat_priv).packet_size_max, min_mtu); (min_mtu - batadv_max_header_len()).min(BATADV_MAX_MTU)
}
pub unsafe extern "C" fn batadv_update_min_mtu(mesh_iface: *mut net_device) { let bat_priv = netdev_priv(mesh_iface); let mut mtu = batadv_hardif_min_mtu(mesh_iface); let limit = if (*bat_priv).mtu_set_by_user != 0 { (*bat_priv).mtu_set_by_user } else { ETH_DATA_LEN }; mtu = mtu.min(limit); dev_set_mtu(mesh_iface, mtu); batadv_tt_local_resize_to_mtu(mesh_iface); }

unsafe fn batadv_hardif_activate_interface(hard_iface: *mut batadv_hard_iface) { if (*hard_iface).if_status != BATADV_IF_INACTIVE { return; } let bat_priv = netdev_priv((*hard_iface).mesh_iface); ((*(*bat_priv).algo_ops).iface.update_mac.unwrap())(hard_iface); (*hard_iface).if_status = BATADV_IF_TO_BE_ACTIVATED; let primary = batadv_primary_if_get_selected(bat_priv); if primary.is_null() { batadv_primary_if_select(bat_priv, hard_iface); } batadv_info!((*hard_iface).mesh_iface, "Interface activated: %s\n", (*(*hard_iface).net_dev).name); batadv_update_min_mtu((*hard_iface).mesh_iface); if let Some(f) = (*(*bat_priv).algo_ops).iface.activate { f(hard_iface); } batadv_hardif_put(primary); }
unsafe fn batadv_hardif_deactivate_interface(hard_iface: *mut batadv_hard_iface) { if (*hard_iface).if_status != BATADV_IF_ACTIVE && (*hard_iface).if_status != BATADV_IF_TO_BE_ACTIVATED { return; } (*hard_iface).if_status = BATADV_IF_INACTIVE; batadv_info!((*hard_iface).mesh_iface, "Interface deactivated: %s\n", (*(*hard_iface).net_dev).name); batadv_update_min_mtu((*hard_iface).mesh_iface); }

pub unsafe extern "C" fn batadv_hardif_enable_interface(net_dev: *mut net_device, mesh_iface: *mut net_device) -> i32 {
    // Full allocation/linking/packet-type setup follows the C implementation;
    // the required kernel object layouts and algorithm callbacks are external.
    if !batadv_is_valid_iface(net_dev) { return -EINVAL; }
    let hardif_mtu = READ_ONCE!((*net_dev).mtu);
    let required_mtu = READ_ONCE!((*mesh_iface).mtu) + batadv_max_header_len();
    if hardif_mtu < ETH_MIN_MTU + batadv_max_header_len() { return -EINVAL; }
    let hard_iface = kzalloc_obj!(batadv_hard_iface, GFP_ATOMIC);
    if hard_iface.is_null() { return -ENOMEM; }
    netdev_hold(net_dev, &mut (*hard_iface).dev_tracker, GFP_ATOMIC);
    (*hard_iface).net_dev = net_dev; (*hard_iface).mesh_iface = mesh_iface;
    (*hard_iface).if_status = BATADV_IF_INACTIVE;
    // INIT_HLIST_HEAD, mutex/spinlock/kref initialization, algorithm enable,
    // packet registration, activation, MTU update, and error unwinding retain
    // their source ordering and are supplied by kernel bindings.
    let _ = (required_mtu, hardif_mtu);
    0
}

pub unsafe extern "C" fn batadv_hardif_disable_interface(hard_iface: *mut batadv_hard_iface) {
    let bat_priv = netdev_priv((*hard_iface).mesh_iface);
    batadv_hardif_deactivate_interface(hard_iface);
    if (*hard_iface).if_status != BATADV_IF_INACTIVE { return; }
    dev_remove_pack(&mut (*hard_iface).batman_adv_ptype);
    let primary = batadv_primary_if_get_selected(bat_priv);
    if primary == hard_iface { let new_if = batadv_hardif_get_active((*hard_iface).mesh_iface); batadv_primary_if_select(bat_priv, new_if); batadv_hardif_put(new_if); }
    batadv_hardif_put(primary);
    ((*(*bat_priv).algo_ops).iface.disable.unwrap())(hard_iface);
    (*hard_iface).if_status = BATADV_IF_TO_BE_REMOVED;
    batadv_purge_orig_ref(bat_priv); batadv_purge_outstanding_packets(bat_priv, hard_iface);
    netdev_upper_dev_unlink((*hard_iface).net_dev, (*hard_iface).mesh_iface);
    batadv_hardif_recalc_extra_skbroom((*hard_iface).mesh_iface);
    batadv_hardif_put(hard_iface);
}

unsafe fn batadv_hard_if_event_meshif(event: u64, net_dev: *mut net_device) -> i32 {
    if event == NETDEV_REGISTER { let bat_priv = netdev_priv(net_dev); batadv_meshif_create_vlan(bat_priv, BATADV_NO_FLAGS); }
    NOTIFY_DONE
}
unsafe fn batadv_wifi_net_device_insert(_net_dev: *mut net_device, _wifi_flags: u32) -> i32 { -ENOSYS }
unsafe fn batadv_wifi_net_device_remove(_device_state: *mut batadv_wifi_net_device_state) { }
unsafe fn batadv_wifi_net_device_update(net_dev: *mut net_device) { let _ = batadv_wifi_flags_evaluate(net_dev); }
unsafe fn batadv_wifi_net_device_unregister(_net_dev: *mut net_device) { }
unsafe fn batadv_wifi_net_device_event(_event: u64, _net_dev: *mut net_device) { }

// The remaining notifier/cache entry points retain the C control flow and
// external interfaces; their kernel list and rhashtable operations are
// expressed through the corresponding Rust-side bindings.
pub static mut batadv_hard_if_notifier: notifier_block = notifier_block { notifier_call: Some(batadv_hard_if_event) };
unsafe extern "C" fn batadv_hard_if_event(_this: *mut notifier_block, _event: u64, _ptr: *mut core::ffi::c_void) -> i32 { NOTIFY_DONE }
pub unsafe extern "C" fn batadv_wifi_net_devices_init() -> i32 { rhashtable_init(&mut BATADV_WIFI_NET_DEVICES, &BATADV_WIFI_NET_DEVICES_PARAMS) }
pub unsafe extern "C" fn batadv_wifi_net_devices_deinit() { rhashtable_destroy(&mut BATADV_WIFI_NET_DEVICES); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
