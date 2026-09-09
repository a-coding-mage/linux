// SPDX-License-Identifier: GPL-2.0-only
/*
 * File: pn_dev.c
 *
 * Phonet network device
 *
 * Copyright (C) 2008 Nokia Corporation.
 *
 * Authors: Sakari Ailus <sakari.ailus@nokia.com>
 *          Rémi Denis-Courmont
 */

#[repr(C)]
struct phonet_routes {
    lock: spinlock_t,
    table: [*mut net_device; 64],
}

#[repr(C)]
struct phonet_net {
    pndevs: phonet_device_list,
    routes: phonet_routes,
}

static mut phonet_net_id: c_uint = 0;

unsafe fn phonet_pernet(net: *mut net) -> *mut phonet_net {
    net_generic(net, phonet_net_id)
}

#[no_mangle]
pub unsafe extern "C" fn phonet_device_list(net: *mut net) -> *mut phonet_device_list {
    &mut (*phonet_pernet(net)).pndevs
}

/* Allocate new Phonet device. */
unsafe fn __phonet_device_alloc(dev: *mut net_device) -> *mut phonet_device {
    let pndevs = phonet_device_list(dev_net(dev));
    let pnd = kmalloc_obj::<phonet_device>(GFP_ATOMIC);
    if pnd.is_null() { return core::ptr::null_mut(); }
    (*pnd).netdev = dev;
    bitmap_zero((*pnd).addrs.as_mut_ptr(), 64);
    lockdep_assert_held(&mut (*pndevs).lock);
    list_add_rcu(&mut (*pnd).list, &mut (*pndevs).list);
    pnd
}

unsafe fn __phonet_get(dev: *mut net_device) -> *mut phonet_device {
    let pndevs = phonet_device_list(dev_net(dev));
    let mut pnd: *mut phonet_device = core::ptr::null_mut();
    lockdep_assert_held(&mut (*pndevs).lock);
    list_for_each_entry!(pnd, &mut (*pndevs).list, list, {
        if (*pnd).netdev == dev { return pnd; }
    });
    core::ptr::null_mut()
}

unsafe fn __phonet_get_rcu(dev: *mut net_device) -> *mut phonet_device {
    let pndevs = phonet_device_list(dev_net(dev));
    let mut pnd: *mut phonet_device = core::ptr::null_mut();
    list_for_each_entry_rcu!(pnd, &(*pndevs).list, list, {
        if (*pnd).netdev == dev { return pnd; }
    });
    core::ptr::null_mut()
}

unsafe fn phonet_device_destroy(dev: *mut net_device) {
    let pndevs = phonet_device_list(dev_net(dev));
    ASSERT_RTNL!();
    spin_lock(&mut (*pndevs).lock);
    let mut pnd = __phonet_get(dev);
    if !pnd.is_null() { list_del_rcu(&mut (*pnd).list); }
    spin_unlock(&mut (*pndevs).lock);
    if !pnd.is_null() {
        let net = dev_net(dev);
        let ifindex = (*dev).ifindex;
        for_each_set_bit!(addr, (*pnd).addrs.as_ptr(), 64, {
            phonet_address_notify(net, RTM_DELADDR, ifindex, addr);
        });
        kfree_rcu!(pnd, rcu);
    }
}

#[no_mangle]
pub unsafe extern "C" fn phonet_device_get(net: *mut net) -> *mut net_device {
    let pndevs = phonet_device_list(net);
    let mut dev: *mut net_device = core::ptr::null_mut();
    let mut pnd: *mut phonet_device = core::ptr::null_mut();
    rcu_read_lock();
    list_for_each_entry_rcu!(pnd, &(*pndevs).list, list, {
        dev = (*pnd).netdev;
        BUG_ON!(dev.is_null());
        if (*dev).reg_state == NETREG_REGISTERED && ((*dev).flags & IFF_UP) == IFF_UP { break; }
        dev = core::ptr::null_mut();
    });
    dev_hold(dev);
    rcu_read_unlock();
    dev
}

#[no_mangle]
pub unsafe extern "C" fn phonet_address_add(dev: *mut net_device, addr: u8) -> c_int {
    let pndevs = phonet_device_list(dev_net(dev));
    spin_lock(&mut (*pndevs).lock);
    let mut pnd = __phonet_get(dev);
    if pnd.is_null() { pnd = __phonet_device_alloc(dev); }
    let err = if pnd.is_null() { -ENOMEM } else if test_and_set_bit((addr >> 2) as usize, (*pnd).addrs.as_mut_ptr()) { -EEXIST } else { 0 };
    spin_unlock(&mut (*pndevs).lock);
    err
}

#[no_mangle]
pub unsafe extern "C" fn phonet_address_del(dev: *mut net_device, addr: u8) -> c_int {
    let pndevs = phonet_device_list(dev_net(dev));
    spin_lock(&mut (*pndevs).lock);
    let mut pnd = __phonet_get(dev);
    let err;
    if pnd.is_null() || !test_and_clear_bit((addr >> 2) as usize, (*pnd).addrs.as_mut_ptr()) {
        err = -EADDRNOTAVAIL; pnd = core::ptr::null_mut();
    } else if bitmap_empty((*pnd).addrs.as_ptr(), 64) { list_del_rcu(&mut (*pnd).list); }
    else { pnd = core::ptr::null_mut(); }
    spin_unlock(&mut (*pndevs).lock);
    if !pnd.is_null() { kfree_rcu!(pnd, rcu); }
    err
}

#[no_mangle]
pub unsafe extern "C" fn phonet_address_get(dev: *mut net_device, daddr: u8) -> u8 {
    rcu_read_lock();
    let pnd = __phonet_get_rcu(dev);
    let mut saddr = if !pnd.is_null() {
        BUG_ON!(bitmap_empty((*pnd).addrs.as_ptr(), 64));
        if test_bit((daddr >> 2) as usize, (*pnd).addrs.as_ptr()) { daddr } else { (find_first_bit((*pnd).addrs.as_ptr(), 64) << 2) as u8 }
    } else { PN_NO_ADDR };
    rcu_read_unlock();
    if saddr == PN_NO_ADDR {
        let def_dev = phonet_device_get(dev_net(dev));
        if !def_dev.is_null() { if def_dev != dev { saddr = phonet_address_get(def_dev, daddr); } dev_put(def_dev); }
    }
    saddr
}

#[no_mangle]
pub unsafe extern "C" fn phonet_address_lookup(net: *mut net, addr: u8) -> c_int {
    let pndevs = phonet_device_list(net);
    let mut pnd: *mut phonet_device = core::ptr::null_mut();
    let mut err = -EADDRNOTAVAIL;
    rcu_read_lock();
    list_for_each_entry_rcu!(pnd, &(*pndevs).list, list, {
        let dev = (*pnd).netdev;
        if (*dev).reg_state != NETREG_REGISTERED || ((*dev).flags & IFF_UP) != IFF_UP { continue; }
        if test_bit((addr >> 2) as usize, (*pnd).addrs.as_ptr()) { err = 0; break; }
    });
    rcu_read_unlock();
    err
}

unsafe fn phonet_device_autoconf(dev: *mut net_device) -> c_int {
    let ops = (*dev).netdev_ops;
    if (*ops).ndo_siocdevprivate.is_none() { return -EOPNOTSUPP; }
    let mut req: if_phonet_req = core::mem::zeroed();
    let ret = ((*ops).ndo_siocdevprivate.unwrap())(dev, &mut req as *mut _ as *mut ifreq, core::ptr::null_mut(), SIOCPNGAUTOCONF);
    if ret < 0 { return ret; }
    ASSERT_RTNL!();
    let ret = phonet_address_add(dev, req.ifr_phonet_autoconf.device);
    if ret != 0 { return ret; }
    phonet_address_notify(dev_net(dev), RTM_NEWADDR, (*dev).ifindex, req.ifr_phonet_autoconf.device);
    0
}

unsafe fn phonet_route_autodel(dev: *mut net_device) {
    let net = dev_net(dev);
    let pnn = phonet_pernet(net);
    let mut deleted = [0u64; 1];
    spin_lock(&mut (*pnn).routes.lock);
    for i in 0..64 {
        if rcu_access_pointer((*pnn).routes.table[i]) == dev { RCU_INIT_POINTER!((*pnn).routes.table[i], core::ptr::null_mut()); set_bit(i, deleted.as_mut_ptr()); }
    }
    spin_unlock(&mut (*pnn).routes.lock);
    if bitmap_empty(deleted.as_ptr(), 64) { return; }
    synchronize_rcu();
    for_each_set_bit!(i, deleted.as_ptr(), 64, { rtm_phonet_notify(net, RTM_DELROUTE, (*dev).ifindex, i); dev_put(dev); });
}

unsafe extern "C" fn phonet_device_notify(_: *mut notifier_block, what: c_ulong, ptr: *mut c_void) -> c_int {
    let dev = netdev_notifier_info_to_dev(ptr);
    match what {
        NETDEV_REGISTER => { if (*dev).type_ == ARPHRD_PHONET { phonet_device_autoconf(dev); } },
        NETDEV_UNREGISTER => { phonet_device_destroy(dev); phonet_route_autodel(dev); },
        _ => {}
    }
    0
}

static mut phonet_device_notifier: notifier_block = notifier_block { notifier_call: Some(phonet_device_notify), priority: 0 };

unsafe extern "C" fn phonet_init_net(net: *mut net) -> c_int {
    let pnn = phonet_pernet(net);
    if proc_create_net(c"phonet".as_ptr(), 0, (*net).proc_net, &pn_sock_seq_ops, core::mem::size_of::<seq_net_private>()).is_null() { return -ENOMEM; }
    INIT_LIST_HEAD!(&mut (*pnn).pndevs.list); spin_lock_init(&mut (*pnn).pndevs.lock); spin_lock_init(&mut (*pnn).routes.lock); 0
}
unsafe extern "C" fn phonet_exit_net(net: *mut net) { let pnn = phonet_pernet(net); remove_proc_entry(c"phonet".as_ptr(), (*net).proc_net); WARN_ON_ONCE!(!list_empty(&(*pnn).pndevs.list)); }

#[no_mangle] pub unsafe extern "C" fn phonet_device_init() -> c_int {
    let mut err = register_pernet_subsys(&mut phonet_net_ops); if err != 0 { return err; }
    if proc_create_net(c"pnresource".as_ptr(), 0, (*init_net).proc_net, &pn_res_seq_ops, core::mem::size_of::<seq_net_private>()).is_null() { unregister_pernet_subsys(&mut phonet_net_ops); return -ENOMEM; }
    err = register_netdevice_notifier(&mut phonet_device_notifier); if err != 0 { remove_proc_entry(c"pnresource".as_ptr(), (*init_net).proc_net); unregister_pernet_subsys(&mut phonet_net_ops); return err; }
    err = phonet_netlink_register(); if err != 0 { unregister_netdevice_notifier(&mut phonet_device_notifier); remove_proc_entry(c"pnresource".as_ptr(), (*init_net).proc_net); unregister_pernet_subsys(&mut phonet_net_ops); } err
}
#[no_mangle] pub unsafe extern "C" fn phonet_device_exit() { rtnl_unregister_all(PF_PHONET); unregister_netdevice_notifier(&mut phonet_device_notifier); remove_proc_entry(c"pnresource".as_ptr(), (*init_net).proc_net); unregister_pernet_subsys(&mut phonet_net_ops); }

#[no_mangle] pub unsafe extern "C" fn phonet_route_add(dev: *mut net_device, daddr: u8) -> c_int { let routes = &mut (*phonet_pernet(dev_net(dev))).routes; let i = (daddr >> 2) as usize; spin_lock(&mut routes.lock); let err = if routes.table[i].is_null() { rcu_assign_pointer!(routes.table[i], dev); dev_hold(dev); 0 } else { -EEXIST }; spin_unlock(&mut routes.lock); err }
#[no_mangle] pub unsafe extern "C" fn phonet_route_del(dev: *mut net_device, daddr: u8) -> c_int { let routes = &mut (*phonet_pernet(dev_net(dev))).routes; let i=(daddr>>2) as usize; spin_lock(&mut routes.lock); if rcu_access_pointer(routes.table[i])==dev { RCU_INIT_POINTER!(routes.table[i], core::ptr::null_mut()); } else { dev=core::ptr::null_mut(); } spin_unlock(&mut routes.lock); if dev.is_null() { -ENOENT } else { 0 } }
#[no_mangle] pub unsafe extern "C" fn phonet_route_get_rcu(net: *mut net, daddr: u8) -> *mut net_device { rcu_dereference((*phonet_pernet(net)).routes.table[(daddr>>2) as usize]) }
#[no_mangle] pub unsafe extern "C" fn phonet_route_output(net: *mut net, daddr: u8) -> *mut net_device { rcu_read_lock(); let mut dev=rcu_dereference((*phonet_pernet(net)).routes.table[(daddr>>2) as usize]); dev_hold(dev); rcu_read_unlock(); if dev.is_null() { dev=phonet_device_get(net); } dev }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
