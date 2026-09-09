// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2007, 2008, 2009 Siemens AG
 */

// Linux/kernel and ieee802154 dependencies are supplied by other translated files.

/* name for sysfs, %d is appended */
const PHY_NAME: &[u8] = b"phy\0";

/* RCU-protected (and RTNL for writers) */
pub static mut cfg802154_rdev_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
pub static mut cfg802154_rdev_list_generation: c_int = 0;

pub unsafe fn wpan_phy_find(str_: *const c_char) -> *mut wpan_phy {
    if str_.is_null() { return core::ptr::null_mut(); }
    let dev = class_find_device_by_name(&mut wpan_phy_class, str_);
    if dev.is_null() { return core::ptr::null_mut(); }
    container_of!(dev, wpan_phy, dev)
}

#[repr(C)]
pub struct wpan_phy_iter_data {
    pub fn_: Option<unsafe extern "C" fn(*mut wpan_phy, *mut c_void) -> c_int>,
    pub data: *mut c_void,
}

unsafe extern "C" fn wpan_phy_iter(dev: *mut device, data: *mut c_void) -> c_int {
    let wpid = data as *mut wpan_phy_iter_data;
    let phy = container_of!(dev, wpan_phy, dev);
    ((*wpid).fn_.unwrap())(phy, (*wpid).data)
}

pub unsafe extern "C" fn wpan_phy_for_each(
    fn_: Option<unsafe extern "C" fn(*mut wpan_phy, *mut c_void) -> c_int>,
    data: *mut c_void,
) -> c_int {
    let mut wpid = wpan_phy_iter_data { fn_, data };
    class_for_each_device(&mut wpan_phy_class, core::ptr::null_mut(), &mut wpid as *mut _ as *mut c_void, Some(wpan_phy_iter))
}

pub unsafe extern "C" fn cfg802154_rdev_by_wpan_phy_idx(wpan_phy_idx: c_int) -> *mut cfg802154_registered_device {
    let mut result: *mut cfg802154_registered_device = core::ptr::null_mut();
    let mut rdev: *mut cfg802154_registered_device;
    ASSERT_RTNL!();
    list_for_each_entry!(rdev, &mut cfg802154_rdev_list, list, {
        if (*rdev).wpan_phy_idx == wpan_phy_idx { result = rdev; break; }
    });
    result
}

pub unsafe extern "C" fn wpan_phy_idx_to_wpan_phy(wpan_phy_idx: c_int) -> *mut wpan_phy {
    ASSERT_RTNL!();
    let rdev = cfg802154_rdev_by_wpan_phy_idx(wpan_phy_idx);
    if rdev.is_null() { return core::ptr::null_mut(); }
    &mut (*rdev).wpan_phy
}

pub unsafe extern "C" fn wpan_phy_new(ops: *const cfg802154_ops, priv_size: usize) -> *mut wpan_phy {
    static mut wpan_phy_counter: atomic_t = atomic_t { counter: 0 };
    let alloc_size = core::mem::size_of::<cfg802154_registered_device>() + priv_size;
    let rdev = kzalloc(alloc_size, GFP_KERNEL);
    if rdev.is_null() { return core::ptr::null_mut(); }
    let rdev = rdev as *mut cfg802154_registered_device;
    (*rdev).ops = ops;
    (*rdev).wpan_phy_idx = atomic_inc_return(&mut wpan_phy_counter);
    if (*rdev).wpan_phy_idx < 0 {
        atomic_dec(&mut wpan_phy_counter); kfree(rdev as *mut c_void); return core::ptr::null_mut();
    }
    (*rdev).wpan_phy_idx -= 1;
    INIT_LIST_HEAD!(&mut (*rdev).wpan_dev_list);
    device_initialize(&mut (*rdev).wpan_phy.dev);
    dev_set_name(&mut (*rdev).wpan_phy.dev, PHY_NAME.as_ptr() as *const c_char, (*rdev).wpan_phy_idx);
    (*rdev).wpan_phy.dev.class = &mut wpan_phy_class;
    (*rdev).wpan_phy.dev.platform_data = rdev as *mut c_void;
    wpan_phy_net_set(&mut (*rdev).wpan_phy, &mut init_net);
    init_waitqueue_head(&mut (*rdev).dev_wait);
    init_waitqueue_head(&mut (*rdev).wpan_phy.sync_txq);
    spin_lock_init(&mut (*rdev).wpan_phy.queue_lock);
    &mut (*rdev).wpan_phy
}

pub unsafe extern "C" fn wpan_phy_register(phy: *mut wpan_phy) -> c_int {
    let rdev = wpan_phy_to_rdev(phy); let ret;
    rtnl_lock(); ret = device_add(&mut (*phy).dev);
    if ret != 0 { rtnl_unlock(); return ret; }
    list_add_rcu!(&mut (*rdev).list, &mut cfg802154_rdev_list);
    cfg802154_rdev_list_generation += 1;
    rtnl_unlock(); 0
}

pub unsafe extern "C" fn wpan_phy_unregister(phy: *mut wpan_phy) {
    let rdev = wpan_phy_to_rdev(phy);
    wait_event!((*rdev).dev_wait, { rtnl_lock(); let count = (*rdev).opencount; rtnl_unlock(); count == 0 });
    rtnl_lock();
    WARN_ON!(!list_empty(&mut (*rdev).wpan_dev_list));
    list_del_rcu!(&mut (*rdev).list); synchronize_rcu();
    cfg802154_rdev_list_generation += 1; device_del(&mut (*phy).dev); rtnl_unlock();
}

pub unsafe extern "C" fn wpan_phy_free(phy: *mut wpan_phy) { put_device(&mut (*phy).dev); }

unsafe fn cfg802154_free_peer_structures(wpan_dev: *mut wpan_dev) {
    mutex_lock(&mut (*wpan_dev).association_lock);
    kfree((*wpan_dev).parent as *mut c_void); (*wpan_dev).parent = core::ptr::null_mut();
    let mut child: *mut ieee802154_pan_device; let mut tmp: *mut ieee802154_pan_device;
    list_for_each_entry_safe!(child, tmp, &mut (*wpan_dev).children, node, { list_del!(&mut (*child).node); kfree(child as *mut c_void); });
    (*wpan_dev).nchildren = 0; mutex_unlock(&mut (*wpan_dev).association_lock);
}

pub unsafe extern "C" fn cfg802154_switch_netns(rdev: *mut cfg802154_registered_device, net: *mut net) -> c_int {
    let mut err = 0; let mut wpan_dev: *mut wpan_dev;
    list_for_each_entry!(wpan_dev, &mut (*rdev).wpan_dev_list, list, {
        if (*wpan_dev).netdev.is_null() { continue; }
        (*(*wpan_dev).netdev).netns_immutable = false;
        err = dev_change_net_namespace((*wpan_dev).netdev, net, b"wpan%d\0".as_ptr() as *const c_char);
        if err != 0 { WARN_ON!(err != -ENOMEM); break; }
        (*(*wpan_dev).netdev).netns_immutable = true;
    });
    if err != 0 { return cfg802154_switch_netns_errout(rdev, net, err); }
    err = device_rename(&mut (*rdev).wpan_phy.dev, dev_name(&mut (*rdev).wpan_phy.dev)); WARN_ON!(err != -ENOMEM);
    if err != 0 { return cfg802154_switch_netns_errout(rdev, net, err); }
    wpan_phy_net_set(&mut (*rdev).wpan_phy, net); 0
}

unsafe fn cfg802154_switch_netns_errout(rdev: *mut cfg802154_registered_device, _net: *mut net, mut err: c_int) -> c_int {
    let net = wpan_phy_net(&mut (*rdev).wpan_phy); let mut wpan_dev: *mut wpan_dev;
    list_for_each_entry_continue_reverse!(wpan_dev, &mut (*rdev).wpan_dev_list, list, {
        if (*wpan_dev).netdev.is_null() { continue; }
        (*(*wpan_dev).netdev).netns_immutable = false;
        err = dev_change_net_namespace((*wpan_dev).netdev, net, b"wpan%d\0".as_ptr() as *const c_char);
        WARN_ON!(err != -ENOMEM); (*(*wpan_dev).netdev).netns_immutable = true;
    }); err
}

pub unsafe extern "C" fn cfg802154_dev_free(rdev: *mut cfg802154_registered_device) { kfree(rdev as *mut c_void); }

unsafe fn cfg802154_update_iface_num(rdev: *mut cfg802154_registered_device, _iftype: c_int, num: c_int) { ASSERT_RTNL!(); (*rdev).num_running_ifaces += num; }

// The notifier, per-network exit handlers, module initialization, and cleanup retain
// their C registration interfaces and are represented below using the supplied kernel
// declarations. Their bodies preserve the original state transitions.
pub unsafe extern "C" fn cfg802154_netdev_notifier_call(_nb: *mut notifier_block, state: c_ulong, ptr: *mut c_void) -> c_int {
    let dev = netdev_notifier_info_to_dev(ptr); let wpan_dev = (*dev).ieee802154_ptr;
    if wpan_dev.is_null() { return NOTIFY_DONE; }
    let rdev = wpan_phy_to_rdev((*wpan_dev).wpan_phy);
    match state {
        NETDEV_REGISTER => { (*dev).netns_immutable = true; (*wpan_dev).identifier = { (*rdev).wpan_dev_id += 1; (*rdev).wpan_dev_id }; list_add_rcu!(&mut (*wpan_dev).list, &mut (*rdev).wpan_dev_list); (*rdev).devlist_generation += 1; mutex_init(&mut (*wpan_dev).association_lock); INIT_LIST_HEAD!(&mut (*wpan_dev).children); (*wpan_dev).max_associations = SZ_16K; (*wpan_dev).netdev = dev; }
        NETDEV_DOWN => { cfg802154_update_iface_num(rdev, (*wpan_dev).iftype, -1); (*rdev).opencount -= 1; wake_up(&mut (*rdev).dev_wait); }
        NETDEV_UP => { cfg802154_update_iface_num(rdev, (*wpan_dev).iftype, 1); (*rdev).opencount += 1; }
        NETDEV_UNREGISTER => { cfg802154_free_peer_structures(wpan_dev); if !list_empty(&mut (*wpan_dev).list) { list_del_rcu!(&mut (*wpan_dev).list); (*rdev).devlist_generation += 1; } synchronize_rcu(); INIT_LIST_HEAD!(&mut (*wpan_dev).list); }
        _ => return NOTIFY_DONE,
} NOTIFY_OK
}

static mut cfg802154_netdev_notifier: notifier_block = notifier_block { notifier_call: Some(cfg802154_netdev_notifier_call) };

unsafe extern "C" fn cfg802154_pernet_exit(net: *mut net) {
    rtnl_lock();
    let mut rdev: *mut cfg802154_registered_device;
    list_for_each_entry!(rdev, &mut cfg802154_rdev_list, list, {
        if net_eq(wpan_phy_net(&mut (*rdev).wpan_phy), net) { cfg802154_switch_netns(rdev, &mut init_net); }
    });
    rtnl_unlock();
}

static mut cfg802154_pernet_ops: pernet_operations = pernet_operations { exit: Some(cfg802154_pernet_exit) };

unsafe extern "C" fn wpan_phy_class_init() -> c_int {
    let mut rc = register_pernet_device(&mut cfg802154_pernet_ops);
    if rc != 0 { return rc; }
    rc = wpan_phy_sysfs_init();
    if rc != 0 { unregister_pernet_device(&mut cfg802154_pernet_ops); return rc; }
    rc = register_netdevice_notifier(&mut cfg802154_netdev_notifier);
    if rc != 0 { wpan_phy_sysfs_exit(); unregister_pernet_device(&mut cfg802154_pernet_ops); return rc; }
    rc = ieee802154_nl_init();
    if rc != 0 { unregister_netdevice_notifier(&mut cfg802154_netdev_notifier); wpan_phy_sysfs_exit(); unregister_pernet_device(&mut cfg802154_pernet_ops); return rc; }
    rc = nl802154_init();
    if rc != 0 { ieee802154_nl_exit(); unregister_netdevice_notifier(&mut cfg802154_netdev_notifier); wpan_phy_sysfs_exit(); unregister_pernet_device(&mut cfg802154_pernet_ops); return rc; }
    0
}

unsafe extern "C" fn wpan_phy_class_exit() {
    nl802154_exit();
    ieee802154_nl_exit();
    unregister_netdevice_notifier(&mut cfg802154_netdev_notifier);
    wpan_phy_sysfs_exit();
    unregister_pernet_device(&mut cfg802154_pernet_ops);
}

// C init/module registration annotations retained as intent for the kernel build.
// subsys_initcall(wpan_phy_class_init);
// module_exit(wpan_phy_class_exit);
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("IEEE 802.15.4 configuration interface");
// MODULE_AUTHOR("Dmitry Eremin-Solenikov");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
