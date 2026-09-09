// SPDX-License-Identifier: GPL-2.0
/*
 * Devices PM QoS constraints management
 *
 * Copyright (C) 2011 Texas Instruments, Inc.
 *
 * This module exposes the interface to kernel space for specifying
 * per-device PM QoS dependencies. It provides infrastructure for registration
 * of:
 *
 * Dependents on a QoS value : register requests
 * Watchers of QoS value : get notified when target QoS value changes
 *
 * This QoS design is best effort based. Dependents register their QoS needs.
 * Watchers register to keep track of the current QoS needs of the system.
 * Watchers can register a per-device notification callback using the
 * dev_pm_qos_*_notifier API. The notification chain data is stored in the
 * per-device constraint data struct.
 */

// C dependencies supplied by the surrounding kernel translation unit.

static mut dev_pm_qos_mtx: mutex = DEFINE_MUTEX!();
static mut dev_pm_qos_sysfs_mtx: mutex = DEFINE_MUTEX!();

/// __dev_pm_qos_flags - Check PM QoS flags for a given device.
/// This routine must be called with dev->power.lock held.
pub unsafe fn __dev_pm_qos_flags(dev: *mut device, mask: s32) -> pm_qos_flags_status {
    let qos = (*dev).power.qos;
    let pqf: *mut pm_qos_flags;
    let val: s32;

    lockdep_assert_held(&mut (*dev).power.lock);
    if IS_ERR_OR_NULL(qos) {
        return PM_QOS_FLAGS_UNDEFINED;
    }
    pqf = &mut (*qos).flags;
    if list_empty(&mut (*pqf).list) {
        return PM_QOS_FLAGS_UNDEFINED;
    }
    val = (*pqf).effective_flags & mask;
    if val != 0 {
        return if val == mask { PM_QOS_FLAGS_ALL } else { PM_QOS_FLAGS_SOME };
    }
    PM_QOS_FLAGS_NONE
}

/// dev_pm_qos_flags - Check PM QoS flags for a given device (locked).
pub unsafe fn dev_pm_qos_flags(dev: *mut device, mask: s32) -> pm_qos_flags_status {
    let mut irqflags: ulong = 0;
    let ret: pm_qos_flags_status;
    spin_lock_irqsave(&mut (*dev).power.lock, &mut irqflags);
    ret = __dev_pm_qos_flags(dev, mask);
    spin_unlock_irqrestore(&mut (*dev).power.lock, irqflags);
    ret
}

pub unsafe fn __dev_pm_qos_resume_latency(dev: *mut device) -> s32 {
    lockdep_assert_held(&mut (*dev).power.lock);
    dev_pm_qos_raw_resume_latency(dev)
}

pub unsafe fn dev_pm_qos_read_value(dev: *mut device, type_: dev_pm_qos_req_type) -> s32 {
    let qos = (*dev).power.qos;
    let mut flags: ulong = 0;
    let ret: s32;
    spin_lock_irqsave(&mut (*dev).power.lock, &mut flags);
    ret = match type_ {
        DEV_PM_QOS_RESUME_LATENCY => if IS_ERR_OR_NULL(qos) { PM_QOS_RESUME_LATENCY_NO_CONSTRAINT } else { pm_qos_read_value(&mut (*qos).resume_latency) },
        DEV_PM_QOS_MIN_FREQUENCY => if IS_ERR_OR_NULL(qos) { PM_QOS_MIN_FREQUENCY_DEFAULT_VALUE } else { freq_qos_read_value(&mut (*qos).freq, FREQ_QOS_MIN) },
        DEV_PM_QOS_MAX_FREQUENCY => if IS_ERR_OR_NULL(qos) { PM_QOS_MAX_FREQUENCY_DEFAULT_VALUE } else { freq_qos_read_value(&mut (*qos).freq, FREQ_QOS_MAX) },
        _ => { WARN_ON(1); 0 },
    };
    spin_unlock_irqrestore(&mut (*dev).power.lock, flags);
    ret
}

unsafe fn apply_constraint(req: *mut dev_pm_qos_request, action: pm_qos_req_action, mut value: s32) -> c_int {
    let qos = (*(*req).dev).power.qos;
    match (*req).type_ {
        DEV_PM_QOS_RESUME_LATENCY => {
            if WARN_ON(action != PM_QOS_REMOVE_REQ && value < 0) { value = 0; }
            pm_qos_update_target(&mut (*qos).resume_latency, &mut (*req).data.pnode, action, value)
        }
        DEV_PM_QOS_LATENCY_TOLERANCE => {
            let ret = pm_qos_update_target(&mut (*qos).latency_tolerance, &mut (*req).data.pnode, action, value);
            if ret != 0 {
                value = pm_qos_read_value(&mut (*qos).latency_tolerance);
                ((*(*req).dev).power.set_latency_tolerance.unwrap())((*req).dev, value);
            }
            ret
        }
        DEV_PM_QOS_MIN_FREQUENCY | DEV_PM_QOS_MAX_FREQUENCY => freq_qos_apply(&mut (*req).data.freq, action, value),
        DEV_PM_QOS_FLAGS => pm_qos_update_flags(&mut (*qos).flags, &mut (*req).data.flr, action, value),
        _ => -EINVAL,
    }
}

unsafe fn dev_pm_qos_constraints_allocate(dev: *mut device) -> c_int {
    let qos = kzalloc_obj::<dev_pm_qos>();
    if qos.is_null() { return -ENOMEM; }
    let n = kzalloc_objs::<blocking_notifier_head>(3);
    if n.is_null() { kfree(qos as *mut c_void); return -ENOMEM; }
    let c = &mut (*qos).resume_latency;
    plist_head_init(&mut c.list); c.target_value = PM_QOS_RESUME_LATENCY_DEFAULT_VALUE; c.default_value = PM_QOS_RESUME_LATENCY_DEFAULT_VALUE; c.no_constraint_value = PM_QOS_RESUME_LATENCY_NO_CONSTRAINT; c.type_ = PM_QOS_MIN; c.notifiers = n; BLOCKING_INIT_NOTIFIER_HEAD(n);
    let c = &mut (*qos).latency_tolerance;
    plist_head_init(&mut c.list); c.target_value = PM_QOS_LATENCY_TOLERANCE_DEFAULT_VALUE; c.default_value = PM_QOS_LATENCY_TOLERANCE_DEFAULT_VALUE; c.no_constraint_value = PM_QOS_LATENCY_TOLERANCE_NO_CONSTRAINT; c.type_ = PM_QOS_MIN;
    freq_constraints_init(&mut (*qos).freq);
    INIT_LIST_HEAD(&mut (*qos).flags.list);
    spin_lock_irq(&mut (*dev).power.lock); (*dev).power.qos = qos; spin_unlock_irq(&mut (*dev).power.lock);
    0
}

unsafe fn __dev_pm_qos_hide_latency_limit(dev: *mut device);
unsafe fn __dev_pm_qos_hide_flags(dev: *mut device);

pub unsafe fn dev_pm_qos_constraints_destroy(dev: *mut device) {
    mutex_lock(&mut dev_pm_qos_sysfs_mtx); pm_qos_sysfs_remove_resume_latency(dev); pm_qos_sysfs_remove_flags(dev); mutex_lock(&mut dev_pm_qos_mtx);
    __dev_pm_qos_hide_latency_limit(dev); __dev_pm_qos_hide_flags(dev);
    let qos = (*dev).power.qos; if qos.is_null() { mutex_unlock(&mut dev_pm_qos_mtx); mutex_unlock(&mut dev_pm_qos_sysfs_mtx); return; }
    let mut req: *mut dev_pm_qos_request; let mut tmp: *mut dev_pm_qos_request;
    plist_for_each_entry_safe!(req, tmp, &mut (*qos).resume_latency.list, data.pnode) { apply_constraint(req, PM_QOS_REMOVE_REQ, PM_QOS_DEFAULT_VALUE); memset(req as *mut c_void, 0, core::mem::size_of::<dev_pm_qos_request>()); }
    plist_for_each_entry_safe!(req, tmp, &mut (*qos).latency_tolerance.list, data.pnode) { apply_constraint(req, PM_QOS_REMOVE_REQ, PM_QOS_DEFAULT_VALUE); memset(req as *mut c_void, 0, core::mem::size_of::<dev_pm_qos_request>()); }
    plist_for_each_entry_safe!(req, tmp, &mut (*qos).freq.min_freq.list, data.freq.pnode) { apply_constraint(req, PM_QOS_REMOVE_REQ, PM_QOS_MIN_FREQUENCY_DEFAULT_VALUE); memset(req as *mut c_void, 0, core::mem::size_of::<dev_pm_qos_request>()); }
    plist_for_each_entry_safe!(req, tmp, &mut (*qos).freq.max_freq.list, data.freq.pnode) { apply_constraint(req, PM_QOS_REMOVE_REQ, PM_QOS_MAX_FREQUENCY_DEFAULT_VALUE); memset(req as *mut c_void, 0, core::mem::size_of::<dev_pm_qos_request>()); }
    list_for_each_entry_safe!(req, tmp, &mut (*qos).flags.list, data.flr.node) { apply_constraint(req, PM_QOS_REMOVE_REQ, PM_QOS_DEFAULT_VALUE); memset(req as *mut c_void, 0, core::mem::size_of::<dev_pm_qos_request>()); }
    spin_lock_irq(&mut (*dev).power.lock); (*dev).power.qos = ERR_PTR(-ENODEV); spin_unlock_irq(&mut (*dev).power.lock); kfree((*qos).resume_latency.notifiers as *mut c_void); kfree(qos as *mut c_void); mutex_unlock(&mut dev_pm_qos_mtx); mutex_unlock(&mut dev_pm_qos_sysfs_mtx);
}

unsafe fn dev_pm_qos_invalid_req_type(dev: *mut device, type_: dev_pm_qos_req_type) -> bool { type_ == DEV_PM_QOS_LATENCY_TOLERANCE && (*dev).power.set_latency_tolerance.is_none() }

unsafe fn __dev_pm_qos_add_request(dev: *mut device, req: *mut dev_pm_qos_request, type_: dev_pm_qos_req_type, value: s32) -> c_int {
    if dev.is_null() || req.is_null() || dev_pm_qos_invalid_req_type(dev, type_) { return -EINVAL; }
    if WARN(dev_pm_qos_request_active(req), "%s() called for already added request\n", __func__) { return -EINVAL; }
    let mut ret = 0; if IS_ERR((*dev).power.qos) { ret = -ENODEV; } else if (*dev).power.qos.is_null() { ret = dev_pm_qos_constraints_allocate(dev); }
    trace_dev_pm_qos_add_request(dev_name(dev), type_, value); if ret != 0 { return ret; }
    (*req).dev = dev; (*req).type_ = type_;
    if type_ == DEV_PM_QOS_MIN_FREQUENCY { freq_qos_add_request(&mut (*(*dev).power.qos).freq, &mut (*req).data.freq, FREQ_QOS_MIN, value) } else if type_ == DEV_PM_QOS_MAX_FREQUENCY { freq_qos_add_request(&mut (*(*dev).power.qos).freq, &mut (*req).data.freq, FREQ_QOS_MAX, value) } else { apply_constraint(req, PM_QOS_ADD_REQ, value) }
}

pub unsafe fn dev_pm_qos_add_request(dev: *mut device, req: *mut dev_pm_qos_request, type_: dev_pm_qos_req_type, value: s32) -> c_int { mutex_lock(&mut dev_pm_qos_mtx); let ret = __dev_pm_qos_add_request(dev, req, type_, value); mutex_unlock(&mut dev_pm_qos_mtx); ret }

unsafe fn __dev_pm_qos_update_request(req: *mut dev_pm_qos_request, new_value: s32) -> c_int {
    if req.is_null() { return -EINVAL; } if WARN(!dev_pm_qos_request_active(req), "%s() called for unknown object\n", __func__) { return -EINVAL; } if IS_ERR_OR_NULL((*(*req).dev).power.qos) { return -ENODEV; }
    let curr_value = match (*req).type_ { DEV_PM_QOS_RESUME_LATENCY | DEV_PM_QOS_LATENCY_TOLERANCE => (*req).data.pnode.prio, DEV_PM_QOS_MIN_FREQUENCY | DEV_PM_QOS_MAX_FREQUENCY => (*req).data.freq.pnode.prio, DEV_PM_QOS_FLAGS => (*req).data.flr.flags, _ => return -EINVAL };
    trace_dev_pm_qos_update_request(dev_name((*req).dev), (*req).type_, new_value); if curr_value != new_value { apply_constraint(req, PM_QOS_UPDATE_REQ, new_value) } else { 0 }
}

pub unsafe fn dev_pm_qos_update_request(req: *mut dev_pm_qos_request, new_value: s32) -> c_int { mutex_lock(&mut dev_pm_qos_mtx); let ret = __dev_pm_qos_update_request(req, new_value); mutex_unlock(&mut dev_pm_qos_mtx); ret }

unsafe fn __dev_pm_qos_remove_request(req: *mut dev_pm_qos_request) -> c_int { if req.is_null() { return -EINVAL; } if WARN(!dev_pm_qos_request_active(req), "%s() called for unknown object\n", __func__) { return -EINVAL; } if IS_ERR_OR_NULL((*(*req).dev).power.qos) { return -ENODEV; } trace_dev_pm_qos_remove_request(dev_name((*req).dev), (*req).type_, PM_QOS_DEFAULT_VALUE); let ret = apply_constraint(req, PM_QOS_REMOVE_REQ, PM_QOS_DEFAULT_VALUE); memset(req as *mut c_void, 0, core::mem::size_of::<dev_pm_qos_request>()); ret }

pub unsafe fn dev_pm_qos_remove_request(req: *mut dev_pm_qos_request) -> c_int { mutex_lock(&mut dev_pm_qos_mtx); let ret = __dev_pm_qos_remove_request(req); mutex_unlock(&mut dev_pm_qos_mtx); ret }

pub unsafe fn dev_pm_qos_add_notifier(dev: *mut device, notifier: *mut notifier_block, type_: dev_pm_qos_req_type) -> c_int { mutex_lock(&mut dev_pm_qos_mtx); let mut ret = 0; if IS_ERR((*dev).power.qos) { ret = -ENODEV; } else if (*dev).power.qos.is_null() { ret = dev_pm_qos_constraints_allocate(dev); } if ret != 0 { mutex_unlock(&mut dev_pm_qos_mtx); return ret; } ret = match type_ { DEV_PM_QOS_RESUME_LATENCY => blocking_notifier_chain_register((*(*dev).power.qos).resume_latency.notifiers, notifier), DEV_PM_QOS_MIN_FREQUENCY => freq_qos_add_notifier(&mut (*(*dev).power.qos).freq, FREQ_QOS_MIN, notifier), DEV_PM_QOS_MAX_FREQUENCY => freq_qos_add_notifier(&mut (*(*dev).power.qos).freq, FREQ_QOS_MAX, notifier), _ => { WARN_ON(1); -EINVAL } }; mutex_unlock(&mut dev_pm_qos_mtx); ret }

pub unsafe fn dev_pm_qos_remove_notifier(dev: *mut device, notifier: *mut notifier_block, type_: dev_pm_qos_req_type) -> c_int { mutex_lock(&mut dev_pm_qos_mtx); if IS_ERR_OR_NULL((*dev).power.qos) { mutex_unlock(&mut dev_pm_qos_mtx); return 0; } let ret = match type_ { DEV_PM_QOS_RESUME_LATENCY => blocking_notifier_chain_unregister((*(*dev).power.qos).resume_latency.notifiers, notifier), DEV_PM_QOS_MIN_FREQUENCY => freq_qos_remove_notifier(&mut (*(*dev).power.qos).freq, FREQ_QOS_MIN, notifier), DEV_PM_QOS_MAX_FREQUENCY => freq_qos_remove_notifier(&mut (*(*dev).power.qos).freq, FREQ_QOS_MAX, notifier), _ => { WARN_ON(1); -EINVAL } }; mutex_unlock(&mut dev_pm_qos_mtx); ret }

pub unsafe fn dev_pm_qos_add_ancestor_request(dev: *mut device, req: *mut dev_pm_qos_request, type_: dev_pm_qos_req_type, value: s32) -> c_int { let mut ancestor = (*dev).parent; let mut ret = -ENODEV; match type_ { DEV_PM_QOS_RESUME_LATENCY => while !ancestor.is_null() && !(*ancestor).power.ignore_children { ancestor = (*ancestor).parent; }, DEV_PM_QOS_LATENCY_TOLERANCE => while !ancestor.is_null() && (*ancestor).power.set_latency_tolerance.is_none() { ancestor = (*ancestor).parent; }, _ => ancestor = core::ptr::null_mut() } if !ancestor.is_null() { ret = dev_pm_qos_add_request(ancestor, req, type_, value); } if ret < 0 { (*req).dev = core::ptr::null_mut(); } ret }

unsafe fn __dev_pm_qos_drop_user_request(dev: *mut device, type_: dev_pm_qos_req_type) { let req = match type_ { DEV_PM_QOS_RESUME_LATENCY => { let r = (*(*dev).power.qos).resume_latency_req; (*(*dev).power.qos).resume_latency_req = core::ptr::null_mut(); r }, DEV_PM_QOS_LATENCY_TOLERANCE => { let r = (*(*dev).power.qos).latency_tolerance_req; (*(*dev).power.qos).latency_tolerance_req = core::ptr::null_mut(); r }, DEV_PM_QOS_FLAGS => { let r = (*(*dev).power.qos).flags_req; (*(*dev).power.qos).flags_req = core::ptr::null_mut(); r }, _ => { WARN_ON(1); return } }; __dev_pm_qos_remove_request(req); kfree(req as *mut c_void); }
unsafe fn dev_pm_qos_drop_user_request(dev: *mut device, type_: dev_pm_qos_req_type) { mutex_lock(&mut dev_pm_qos_mtx); __dev_pm_qos_drop_user_request(dev, type_); mutex_unlock(&mut dev_pm_qos_mtx); }

pub unsafe fn dev_pm_qos_expose_latency_limit(dev: *mut device, value: s32) -> c_int { if !device_is_registered(dev) || value < 0 { return -EINVAL; } let req = kzalloc_obj::<dev_pm_qos_request>(); if req.is_null() { return -ENOMEM; } let mut ret = dev_pm_qos_add_request(dev, req, DEV_PM_QOS_RESUME_LATENCY, value); if ret < 0 { kfree(req as *mut c_void); return ret; } mutex_lock(&mut dev_pm_qos_sysfs_mtx); mutex_lock(&mut dev_pm_qos_mtx); if IS_ERR_OR_NULL((*dev).power.qos) { ret = -ENODEV; } else if !(*(*dev).power.qos).resume_latency_req.is_null() { ret = -EEXIST; } if ret < 0 { __dev_pm_qos_remove_request(req); kfree(req as *mut c_void); mutex_unlock(&mut dev_pm_qos_mtx); mutex_unlock(&mut dev_pm_qos_sysfs_mtx); return ret; } (*(*dev).power.qos).resume_latency_req = req; mutex_unlock(&mut dev_pm_qos_mtx); ret = pm_qos_sysfs_add_resume_latency(dev); if ret != 0 { dev_pm_qos_drop_user_request(dev, DEV_PM_QOS_RESUME_LATENCY); } mutex_unlock(&mut dev_pm_qos_sysfs_mtx); ret }
unsafe fn __dev_pm_qos_hide_latency_limit(dev: *mut device) { if !IS_ERR_OR_NULL((*dev).power.qos) && !(*(*dev).power.qos).resume_latency_req.is_null() { __dev_pm_qos_drop_user_request(dev, DEV_PM_QOS_RESUME_LATENCY); } }
pub unsafe fn dev_pm_qos_hide_latency_limit(dev: *mut device) { mutex_lock(&mut dev_pm_qos_sysfs_mtx); pm_qos_sysfs_remove_resume_latency(dev); mutex_lock(&mut dev_pm_qos_mtx); __dev_pm_qos_hide_latency_limit(dev); mutex_unlock(&mut dev_pm_qos_mtx); mutex_unlock(&mut dev_pm_qos_sysfs_mtx); }

pub unsafe fn dev_pm_qos_expose_flags(dev: *mut device, val: s32) -> c_int { if !device_is_registered(dev) { return -EINVAL; } let req = kzalloc_obj::<dev_pm_qos_request>(); if req.is_null() { return -ENOMEM; } let mut ret = dev_pm_qos_add_request(dev, req, DEV_PM_QOS_FLAGS, val); if ret < 0 { kfree(req as *mut c_void); return ret; } pm_runtime_get_sync(dev); mutex_lock(&mut dev_pm_qos_sysfs_mtx); mutex_lock(&mut dev_pm_qos_mtx); if IS_ERR_OR_NULL((*dev).power.qos) { ret = -ENODEV; } else if !(*(*dev).power.qos).flags_req.is_null() { ret = -EEXIST; } if ret < 0 { __dev_pm_qos_remove_request(req); kfree(req as *mut c_void); mutex_unlock(&mut dev_pm_qos_mtx); mutex_unlock(&mut dev_pm_qos_sysfs_mtx); pm_runtime_put(dev); return ret; } (*(*dev).power.qos).flags_req = req; mutex_unlock(&mut dev_pm_qos_mtx); ret = pm_qos_sysfs_add_flags(dev); if ret != 0 { dev_pm_qos_drop_user_request(dev, DEV_PM_QOS_FLAGS); } mutex_unlock(&mut dev_pm_qos_sysfs_mtx); pm_runtime_put(dev); ret }
unsafe fn __dev_pm_qos_hide_flags(dev: *mut device) { if !IS_ERR_OR_NULL((*dev).power.qos) && !(*(*dev).power.qos).flags_req.is_null() { __dev_pm_qos_drop_user_request(dev, DEV_PM_QOS_FLAGS); } }
pub unsafe fn dev_pm_qos_hide_flags(dev: *mut device) { pm_runtime_get_sync(dev); mutex_lock(&mut dev_pm_qos_sysfs_mtx); pm_qos_sysfs_remove_flags(dev); mutex_lock(&mut dev_pm_qos_mtx); __dev_pm_qos_hide_flags(dev); mutex_unlock(&mut dev_pm_qos_mtx); mutex_unlock(&mut dev_pm_qos_sysfs_mtx); pm_runtime_put(dev); }

pub unsafe fn dev_pm_qos_update_flags(dev: *mut device, mask: s32, set: bool) -> c_int { pm_runtime_get_sync(dev); mutex_lock(&mut dev_pm_qos_mtx); if IS_ERR_OR_NULL((*dev).power.qos) || (*(*dev).power.qos).flags_req.is_null() { mutex_unlock(&mut dev_pm_qos_mtx); pm_runtime_put(dev); return -EINVAL; } let mut value = dev_pm_qos_requested_flags(dev); if set { value |= mask; } else { value &= !mask; } let ret = __dev_pm_qos_update_request((*(*dev).power.qos).flags_req, value); mutex_unlock(&mut dev_pm_qos_mtx); pm_runtime_put(dev); ret }
pub unsafe fn dev_pm_qos_get_user_latency_tolerance(dev: *mut device) -> s32 { mutex_lock(&mut dev_pm_qos_mtx); let ret = if IS_ERR_OR_NULL((*dev).power.qos) || (*(*dev).power.qos).latency_tolerance_req.is_null() { PM_QOS_LATENCY_TOLERANCE_NO_CONSTRAINT } else { (*(*(*dev).power.qos).latency_tolerance_req).data.pnode.prio }; mutex_unlock(&mut dev_pm_qos_mtx); ret }
pub unsafe fn dev_pm_qos_update_user_latency_tolerance(dev: *mut device, val: s32) -> c_int { mutex_lock(&mut dev_pm_qos_mtx); if IS_ERR_OR_NULL((*dev).power.qos) || (*(*dev).power.qos).latency_tolerance_req.is_null() { if val < 0 { let ret = if val == PM_QOS_LATENCY_TOLERANCE_NO_CONSTRAINT { 0 } else { -EINVAL }; mutex_unlock(&mut dev_pm_qos_mtx); return ret; } let req = kzalloc_obj::<dev_pm_qos_request>(); if req.is_null() { mutex_unlock(&mut dev_pm_qos_mtx); return -ENOMEM; } let ret = __dev_pm_qos_add_request(dev, req, DEV_PM_QOS_LATENCY_TOLERANCE, val); if ret < 0 { kfree(req as *mut c_void); mutex_unlock(&mut dev_pm_qos_mtx); return ret; } (*(*dev).power.qos).latency_tolerance_req = req; mutex_unlock(&mut dev_pm_qos_mtx); ret } else { let ret = if val < 0 { __dev_pm_qos_drop_user_request(dev, DEV_PM_QOS_LATENCY_TOLERANCE); 0 } else { __dev_pm_qos_update_request((*(*dev).power.qos).latency_tolerance_req, val) }; mutex_unlock(&mut dev_pm_qos_mtx); ret } }
pub unsafe fn dev_pm_qos_expose_latency_tolerance(dev: *mut device) -> c_int { if (*dev).power.set_latency_tolerance.is_none() { return -EINVAL; } mutex_lock(&mut dev_pm_qos_sysfs_mtx); let ret = pm_qos_sysfs_add_latency_tolerance(dev); mutex_unlock(&mut dev_pm_qos_sysfs_mtx); ret }
pub unsafe fn dev_pm_qos_hide_latency_tolerance(dev: *mut device) { mutex_lock(&mut dev_pm_qos_sysfs_mtx); pm_qos_sysfs_remove_latency_tolerance(dev); mutex_unlock(&mut dev_pm_qos_sysfs_mtx); pm_runtime_get_sync(dev); dev_pm_qos_update_user_latency_tolerance(dev, PM_QOS_LATENCY_TOLERANCE_NO_CONSTRAINT); pm_runtime_put(dev); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
