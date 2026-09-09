// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright (c) 2016 Mellanox Technologies. All rights reserved. */
/* Copyright (c) 2016 Jiri Pirko <jiri@mellanox.com> */

// Dependencies supplied by the surrounding kernel/devlink translation.

static mut DEVLINKS: XArray = XArray::new(XA_FLAGS_ALLOC);

unsafe fn devlinks_xa_get(index: c_ulong) -> *mut Devlink {
    rcu_read_lock();
    let mut devlink = xa_find(&raw mut DEVLINKS, &index, index, DEVLINK_REGISTERED);
    if devlink.is_null() || devlink_try_get(devlink).is_null() { devlink = core::ptr::null_mut(); }
    rcu_read_unlock();
    devlink
}

// devlink_rels xarray contains 1:1 relationships between devlink object and
// related nested devlink instance.
static mut DEVLINK_RELS: XArray = XArray::new(XA_FLAGS_ALLOC1);
const DEVLINK_REL_IN_USE: u32 = XA_MARK_0;

#[repr(C)]
struct DevlinkRel {
    index: u32,
    refcount: Refcount,
    devlink_index: u32,
    nested_in: DevlinkRelNestedIn,
}
#[repr(C)]
struct DevlinkRelNestedIn {
    devlink_index: u32,
    obj_index: u32,
    notify_cb: DevlinkRelNotifyCb,
    cleanup_cb: DevlinkRelCleanupCb,
    notify_work: DelayedWork,
}

unsafe fn devlink_rel_free(rel: *mut DevlinkRel) { xa_erase(&raw mut DEVLINK_RELS, (*rel).index as c_ulong); kfree(rel.cast()); }
unsafe fn __devlink_rel_get(rel: *mut DevlinkRel) { refcount_inc(&mut (*rel).refcount); }
unsafe fn __devlink_rel_put(rel: *mut DevlinkRel) { if refcount_dec_and_test(&mut (*rel).refcount) { devlink_rel_free(rel); } }

unsafe fn devlink_nested_in_get_lock(mut devlink: *mut Devlink) -> *mut Devlink {
    devl_assert_locked(devlink);
    if (*devlink).rel.is_null() { return core::ptr::null_mut(); }
    devlink = devlinks_xa_get((*(*devlink).rel).nested_in.devlink_index as c_ulong);
    if devlink.is_null() { return devlink; }
    devl_lock(devlink);
    if devl_is_registered(devlink) { return devlink; }
    devl_unlock(devlink); devlink_put(devlink); core::ptr::null_mut()
}

unsafe extern "C" fn devlink_rel_nested_in_notify_work(work: *mut WorkStruct) {
    let rel = container_of(work, core::mem::offset_of!(DevlinkRel, nested_in) + core::mem::offset_of!(DevlinkRelNestedIn, notify_work));
    let rel = rel.cast::<DevlinkRel>();
    let devlink = devlinks_xa_get((*rel).nested_in.devlink_index as c_ulong);
    if devlink.is_null() { __devlink_rel_put(rel); return; }
    if !devl_trylock(devlink) { devlink_put(devlink); schedule_delayed_work(&mut (*rel).nested_in.notify_work, 1); return; }
    if !devl_is_registered(devlink) { devl_unlock(devlink); devlink_put(devlink); __devlink_rel_put(rel); return; }
    if !xa_get_mark(&raw mut DEVLINK_RELS, (*rel).index as c_ulong, DEVLINK_REL_IN_USE) { ((*rel).nested_in.cleanup_cb)(devlink, (*rel).nested_in.obj_index, (*rel).index); }
    ((*rel).nested_in.notify_cb)(devlink, (*rel).nested_in.obj_index);
    devl_unlock(devlink); devlink_put(devlink); __devlink_rel_put(rel);
}

unsafe fn devlink_rel_nested_in_notify_work_schedule(rel: *mut DevlinkRel) { __devlink_rel_get(rel); schedule_delayed_work(&mut (*rel).nested_in.notify_work, 0); }

unsafe fn devlink_rel_alloc() -> *mut DevlinkRel {
    static mut NEXT: u32 = 0;
    let rel = kzalloc::<DevlinkRel>();
    if rel.is_null() { return err_ptr(-ENOMEM); }
    let err = xa_alloc_cyclic(&raw mut DEVLINK_RELS, &mut (*rel).index, rel, XA_LIMIT_32B, &raw mut NEXT, GFP_KERNEL);
    if err < 0 { kfree(rel.cast()); return err_ptr(err); }
    refcount_set(&mut (*rel).refcount, 1);
    init_delayed_work(&mut (*rel).nested_in.notify_work, devlink_rel_nested_in_notify_work);
    rel
}

unsafe fn devlink_rel_put(devlink: *mut Devlink) { let rel = (*devlink).rel; if rel.is_null() { return; } xa_clear_mark(&raw mut DEVLINK_RELS, (*rel).index as c_ulong, DEVLINK_REL_IN_USE); devlink_rel_nested_in_notify_work_schedule(rel); __devlink_rel_put(rel); (*devlink).rel = core::ptr::null_mut(); }

unsafe extern "C" fn devlink_rel_nested_in_clear(rel_index: u32) { xa_clear_mark(&raw mut DEVLINK_RELS, rel_index as c_ulong, DEVLINK_REL_IN_USE); }

unsafe extern "C" fn devlink_rel_nested_in_add(rel_index: *mut u32, devlink_index: u32, obj_index: u32, notify_cb: DevlinkRelNotifyCb, cleanup_cb: DevlinkRelCleanupCb, devlink: *mut Devlink) -> c_int {
    ASSERT_DEVLINK_NOT_REGISTERED!(devlink);
    let rel = devlink_rel_alloc(); if is_err(rel) { return ptr_err(rel); }
    (*rel).devlink_index = (*devlink).index; (*rel).nested_in.devlink_index = devlink_index; (*rel).nested_in.obj_index = obj_index; (*rel).nested_in.notify_cb = notify_cb; (*rel).nested_in.cleanup_cb = cleanup_cb; *rel_index = (*rel).index;
    xa_set_mark(&raw mut DEVLINK_RELS, (*rel).index as c_ulong, DEVLINK_REL_IN_USE); (*devlink).rel = rel; 0
}

unsafe extern "C" fn devlink_rel_nested_in_notify(devlink: *mut Devlink) { let rel = (*devlink).rel; if !rel.is_null() { devlink_rel_nested_in_notify_work_schedule(rel); } }

unsafe fn devlink_rel_find(rel_index: c_ulong) -> *mut DevlinkRel { xa_find(&raw mut DEVLINK_RELS, &rel_index, rel_index, DEVLINK_REL_IN_USE).cast() }
unsafe fn devlink_rel_devlink_get(rel_index: u32) -> *mut Devlink { if rel_index == 0 { return core::ptr::null_mut(); } xa_lock(&raw mut DEVLINK_RELS); let rel = devlink_rel_find(rel_index as c_ulong); let index = if !rel.is_null() { (*rel).devlink_index } else { 0 }; xa_unlock(&raw mut DEVLINK_RELS); if rel.is_null() { core::ptr::null_mut() } else { devlinks_xa_get(index as c_ulong) } }

unsafe extern "C" fn devlink_rel_devlink_handle_put(msg: *mut SkBuff, devlink: *mut Devlink, rel_index: u32, attrtype: c_int, msg_updated: *mut bool) -> c_int { let net = devlink_net(devlink); let rel_devlink = devlink_rel_devlink_get(rel_index); if rel_devlink.is_null() { return 0; } let err = devlink_nl_put_nested_handle(msg, net, rel_devlink, attrtype); devlink_put(rel_devlink); if err == 0 && !msg_updated.is_null() { *msg_updated = true; } err }

pub unsafe extern "C" fn devlink_priv(devlink: *mut Devlink) -> *mut c_void { core::ptr::addr_of_mut!((*devlink).priv_data).cast() }
pub unsafe extern "C" fn priv_to_devlink(priv_: *mut c_void) -> *mut Devlink { container_of(priv_, core::mem::offset_of!(Devlink, priv_data)).cast() }
pub unsafe extern "C" fn devlink_to_dev(devlink: *const Devlink) -> *mut Device { (*devlink).dev }
pub unsafe extern "C" fn devlink_bus_name(devlink: *const Devlink) -> *const c_char { if !(*devlink).dev.is_null() { (*(*devlink).dev).bus.name } else { DEVLINK_INDEX_BUS_NAME.as_ptr() } }
pub unsafe extern "C" fn devlink_dev_name(devlink: *const Devlink) -> *const c_char { if !(*devlink).dev.is_null() { dev_name((*devlink).dev) } else { (*devlink).dev_name_index } }
pub unsafe extern "C" fn devlink_dev_driver_name(devlink: *const Devlink) -> *const c_char { (*(*devlink).dev_driver).name }
pub unsafe extern "C" fn devlink_net(devlink: *const Devlink) -> *mut Net { read_pnet(&(*devlink)._net) }
pub unsafe extern "C" fn devl_assert_locked(devlink: *mut Devlink) { lockdep_assert_held(&(*devlink).lock); }
pub unsafe extern "C" fn devl_lock(devlink: *mut Devlink) { mutex_lock(&mut (*devlink).lock); }
pub unsafe extern "C" fn devl_trylock(devlink: *mut Devlink) -> bool { mutex_trylock(&mut (*devlink).lock) }
pub unsafe extern "C" fn devl_unlock(devlink: *mut Devlink) { mutex_unlock(&mut (*devlink).lock); }
pub unsafe extern "C" fn devlink_try_get(devlink: *mut Devlink) -> *mut Devlink { if refcount_inc_not_zero(&mut (*devlink).refcount) { devlink } else { core::ptr::null_mut() } }

unsafe extern "C" fn devlink_release(work: *mut WorkStruct) { let devlink = container_of(to_rcu_work(work), core::mem::offset_of!(Devlink, rwork)).cast::<Devlink>(); mutex_destroy(&mut (*devlink).lock); lockdep_unregister_key(&mut (*devlink).lock_key); if !(*devlink).dev.is_null() { put_device((*devlink).dev); } else { kfree((*devlink).dev_name_index.cast_mut().cast()); } kvfree(devlink.cast()); }
pub unsafe extern "C" fn devlink_put(devlink: *mut Devlink) { if refcount_dec_and_test(&mut (*devlink).refcount) { queue_rcu_work(system_percpu_wq, &mut (*devlink).rwork); } }

unsafe fn __devlinks_xa_find_get(net: *mut Net, indexp: *mut c_ulong, end: c_ulong) -> *mut Devlink { rcu_read_lock(); loop { let devlink = xa_find(&raw mut DEVLINKS, &*indexp, end, DEVLINK_REGISTERED); if devlink.is_null() { rcu_read_unlock(); return devlink; } if !devlink_try_get(devlink).is_null() && net_eq(devlink_net(devlink), net) { rcu_read_unlock(); return devlink; } if !devlink_try_get(devlink).is_null() { devlink_put(devlink); } *indexp += 1; } }
pub unsafe extern "C" fn devlinks_xa_find_get(net: *mut Net, indexp: *mut c_ulong) -> *mut Devlink { __devlinks_xa_find_get(net, indexp, ULONG_MAX) }
pub unsafe extern "C" fn devlinks_xa_lookup_get(net: *mut Net, index: c_ulong) -> *mut Devlink { __devlinks_xa_find_get(net, &mut { index }, index) }

pub unsafe extern "C" fn devl_register(devlink: *mut Devlink) -> c_int { ASSERT_DEVLINK_NOT_REGISTERED!(devlink); devl_assert_locked(devlink); xa_set_mark(&raw mut DEVLINKS, (*devlink).index as c_ulong, DEVLINK_REGISTERED); devlink_notify_register(devlink); devlink_rel_nested_in_notify(devlink); 0 }
pub unsafe extern "C" fn devlink_register(devlink: *mut Devlink) { devl_lock(devlink); devl_register(devlink); devl_unlock(devlink); }
pub unsafe extern "C" fn devl_unregister(devlink: *mut Devlink) { ASSERT_DEVLINK_REGISTERED!(devlink); devl_assert_locked(devlink); devlink_notify_unregister(devlink); xa_clear_mark(&raw mut DEVLINKS, (*devlink).index as c_ulong, DEVLINK_REGISTERED); devlink_rel_put(devlink); }
pub unsafe extern "C" fn devlink_unregister(devlink: *mut Devlink) { devl_lock(devlink); devl_unregister(devlink); devl_unlock(devlink); }

pub unsafe extern "C" fn __devlink_alloc(ops: *const DevlinkOps, priv_size: usize, net: *mut Net, dev: *mut Device, driver: *const DeviceDriver) -> *mut Devlink { WARN_ON(ops.is_null() || driver.is_null()); if !devlink_reload_actions_valid(ops) { return core::ptr::null_mut(); } let devlink = kvzalloc_flex::<Devlink>(priv_size); if devlink.is_null() { return devlink; } static mut LAST_ID: u32 = 0; let ret = xa_alloc_cyclic(&raw mut DEVLINKS, &mut (*devlink).index, devlink, XA_LIMIT_31B, &raw mut LAST_ID, GFP_KERNEL); if ret < 0 { kvfree(devlink.cast()); return core::ptr::null_mut(); } if !dev.is_null() { (*devlink).dev = get_device(dev); } else { (*devlink).dev_name_index = kasprintf(GFP_KERNEL, c"%u", (*devlink).index); if (*devlink).dev_name_index.is_null() { xa_erase(&raw mut DEVLINKS, (*devlink).index as c_ulong); kvfree(devlink.cast()); return core::ptr::null_mut(); } } (*devlink).ops = ops; (*devlink).dev_driver = driver; write_pnet(&mut (*devlink)._net, net); refcount_set(&mut (*devlink).refcount, 1); devlink }
pub unsafe extern "C" fn devlink_alloc_ns(ops: *const DevlinkOps, priv_size: usize, net: *mut Net, dev: *mut Device) -> *mut Devlink { WARN_ON(dev.is_null()); __devlink_alloc(ops, priv_size, net, dev, (*dev).driver) }
pub unsafe extern "C" fn devlink_free(devlink: *mut Devlink) { ASSERT_DEVLINK_NOT_REGISTERED!(devlink); devl_lock(devlink); WARN_ON(devlink_rates_check(devlink, core::ptr::null_mut(), core::ptr::null_mut())); devl_unlock(devlink); devlink_rel_put(devlink); xa_destroy(&mut (*devlink).nested_rels); xa_destroy(&mut (*devlink).snapshot_ids); xa_destroy(&mut (*devlink).params); xa_destroy(&mut (*devlink).ports); xa_erase(&raw mut DEVLINKS, (*devlink).index as c_ulong); devlink_put(devlink); }

// Network-namespace teardown and module initialization retain the original
// kernel registration ordering and callbacks.
unsafe extern "C" fn devlink_pernet_pre_exit(net: *mut Net) { let mut index = 0; let mut devlink; while { devlink = devlinks_xa_find_get(net, &mut index); !devlink.is_null() } { let mut actions = 0; devl_dev_lock(devlink, true); if devl_is_registered(devlink) { devlink_reload(devlink, &raw mut INIT_NET, DEVLINK_RELOAD_ACTION_DRIVER_REINIT, DEVLINK_RELOAD_LIMIT_UNSPEC, &mut actions, core::ptr::null_mut()); } devl_dev_unlock(devlink, true); devlink_put(devlink); index += 1; } }
static mut DEVLINK_PERNET_OPS: PernetOperations = PernetOperations { pre_exit: Some(devlink_pernet_pre_exit) };
static mut DEVLINK_PORT_NETDEVICE_NB: NotifierBlock = NotifierBlock { notifier_call: Some(devlink_port_netdevice_event) };
unsafe extern "C" fn devlink_init() -> c_int { let mut err = register_pernet_subsys(&raw mut DEVLINK_PERNET_OPS); if err != 0 { WARN_ON(err); return err; } err = genl_register_family(&raw mut DEVLINK_NL_FAMILY); if err != 0 { unregister_pernet_subsys(&raw mut DEVLINK_PERNET_OPS); WARN_ON(err); return err; } err = register_netdevice_notifier(&raw mut DEVLINK_PORT_NETDEVICE_NB); if err != 0 { genl_unregister_family(&raw mut DEVLINK_NL_FAMILY); unregister_pernet_subsys(&raw mut DEVLINK_PERNET_OPS); } WARN_ON(err); err }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
