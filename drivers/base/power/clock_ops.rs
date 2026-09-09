// SPDX-License-Identifier: GPL-2.0
/* Generic clock manipulation PM callbacks.  Kernel headers and symbols are
 * supplied by the surrounding translation unit. */

#[cfg(CONFIG_PM_CLK)]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum pce_status {
    PCE_STATUS_NONE = 0,
    PCE_STATUS_ACQUIRED,
    PCE_STATUS_PREPARED,
    PCE_STATUS_ENABLED,
    PCE_STATUS_ERROR,
}

#[cfg(CONFIG_PM_CLK)]
#[repr(C)]
struct pm_clock_entry {
    node: list_head,
    con_id: *mut c_char,
    clk: *mut clk,
    status: pce_status,
    enabled_when_prepared: bool,
}

#[cfg(CONFIG_PM_CLK)]
unsafe fn pm_clk_list_lock(psd: *mut pm_subsys_data) {
    mutex_lock(&mut (*psd).clock_mutex);
    spin_lock_irq(&mut (*psd).lock);
}

#[cfg(CONFIG_PM_CLK)]
unsafe fn pm_clk_list_unlock(psd: *mut pm_subsys_data) {
    spin_unlock_irq(&mut (*psd).lock);
    mutex_unlock(&mut (*psd).clock_mutex);
}

#[cfg(CONFIG_PM_CLK)]
unsafe fn pm_clk_op_lock(psd: *mut pm_subsys_data, flags: *mut c_ulong, fn_name: *const c_char) -> c_int {
    let atomic_context = in_atomic() || irqs_disabled();
    loop {
        spin_lock_irqsave(&mut (*psd).lock, *flags);
        if (*psd).clock_op_might_sleep == 0 { return 0; }
        if atomic_context {
            pr_err!("%s: atomic context with clock_ops_might_sleep = %d", fn_name, (*psd).clock_op_might_sleep);
            spin_unlock_irqrestore(&mut (*psd).lock, *flags);
            might_sleep();
            return -EPERM;
        }
        spin_unlock_irqrestore(&mut (*psd).lock, *flags);
        mutex_lock(&mut (*psd).clock_mutex);
        if (*psd).clock_op_might_sleep != 0 { return 0; }
        mutex_unlock(&mut (*psd).clock_mutex);
    }
}

#[cfg(CONFIG_PM_CLK)]
unsafe fn pm_clk_op_unlock(psd: *mut pm_subsys_data, flags: *mut c_ulong) {
    if (*psd).clock_op_might_sleep != 0 { mutex_unlock(&mut (*psd).clock_mutex); }
    else { spin_unlock_irqrestore(&mut (*psd).lock, *flags); }
}

#[cfg(CONFIG_PM_CLK)]
unsafe fn __pm_clk_enable(dev: *mut device, ce: *mut pm_clock_entry) {
    let ret = match (*ce).status {
        pce_status::PCE_STATUS_ACQUIRED => clk_prepare_enable((*ce).clk),
        pce_status::PCE_STATUS_PREPARED => clk_enable((*ce).clk),
        _ => return,
    };
    if ret == 0 { (*ce).status = pce_status::PCE_STATUS_ENABLED; }
    else { dev_err!(dev, "__pm_clk_enable: failed to enable clk {:?}, error {}\n", (*ce).clk, ret); }
}

#[cfg(CONFIG_PM_CLK)]
unsafe fn pm_clk_acquire(dev: *mut device, ce: *mut pm_clock_entry) {
    if (*ce).clk.is_null() { (*ce).clk = clk_get(dev, (*ce).con_id); }
    if IS_ERR((*ce).clk) { (*ce).status = pce_status::PCE_STATUS_ERROR; }
    else if clk_is_enabled_when_prepared((*ce).clk) {
        (*ce).status = pce_status::PCE_STATUS_ACQUIRED;
        (*ce).enabled_when_prepared = true;
    } else if clk_prepare((*ce).clk) != 0 {
        (*ce).status = pce_status::PCE_STATUS_ERROR;
        dev_err!(dev, "clk_prepare() failed\n");
    } else { (*ce).status = pce_status::PCE_STATUS_PREPARED; }
    dev_dbg!(dev, "Clock {:?} con_id {:?} managed by runtime PM.\n", (*ce).clk, (*ce).con_id);
}

#[cfg(CONFIG_PM_CLK)]
unsafe fn __pm_clk_add(dev: *mut device, con_id: *const c_char, clk: *mut clk) -> c_int {
    let psd = dev_to_psd(dev);
    if psd.is_null() { return -EINVAL; }
    let ce = kzalloc_obj::<pm_clock_entry>();
    if ce.is_null() { return -ENOMEM; }
    if !con_id.is_null() {
        (*ce).con_id = kstrdup(con_id, GFP_KERNEL);
        if (*ce).con_id.is_null() { kfree(ce); return -ENOMEM; }
    } else if IS_ERR(clk) { kfree(ce); return -ENOENT; } else { (*ce).clk = clk; }
    pm_clk_acquire(dev, ce);
    pm_clk_list_lock(psd);
    list_add_tail(&mut (*ce).node, &mut (*psd).clock_list);
    if (*ce).enabled_when_prepared { (*psd).clock_op_might_sleep += 1; }
    pm_clk_list_unlock(psd);
    0
}

#[cfg(CONFIG_PM_CLK)]
pub unsafe fn pm_clk_add(dev: *mut device, con_id: *const c_char) -> c_int { __pm_clk_add(dev, con_id, core::ptr::null_mut()) }
#[cfg(CONFIG_PM_CLK)]
pub unsafe fn pm_clk_add_clk(dev: *mut device, clk: *mut clk) -> c_int { __pm_clk_add(dev, core::ptr::null(), clk) }

#[cfg(CONFIG_PM_CLK)]
pub unsafe fn of_pm_clk_add_clks(dev: *mut device) -> c_int {
    if dev.is_null() || (*dev).of_node.is_null() { return -EINVAL; }
    let count = of_clk_get_parent_count((*dev).of_node);
    if count <= 0 { return -ENODEV; }
    let clks = kzalloc_objs::<*mut clk>(count as usize);
    if clks.is_null() { return -ENOMEM; }
    let mut i = 0;
    while i < count {
        *clks.add(i as usize) = of_clk_get((*dev).of_node, i);
        if IS_ERR(*clks.add(i as usize)) { let ret = PTR_ERR(*clks.add(i as usize)); while i > 0 { i -= 1; pm_clk_remove_clk(dev, *clks.add(i as usize)); } kfree(clks); return ret; }
        let ret = pm_clk_add_clk(dev, *clks.add(i as usize));
        if ret != 0 { clk_put(*clks.add(i as usize)); while i > 0 { i -= 1; pm_clk_remove_clk(dev, *clks.add(i as usize)); } kfree(clks); return ret; }
        i += 1;
    }
    kfree(clks); i
}

#[cfg(CONFIG_PM_CLK)]
unsafe fn __pm_clk_remove(ce: *mut pm_clock_entry) {
    if ce.is_null() { return; }
    match (*ce).status {
        pce_status::PCE_STATUS_ENABLED => { clk_disable((*ce).clk); clk_unprepare((*ce).clk); clk_put((*ce).clk); }
        pce_status::PCE_STATUS_PREPARED => { clk_unprepare((*ce).clk); clk_put((*ce).clk); }
        pce_status::PCE_STATUS_ACQUIRED | pce_status::PCE_STATUS_ERROR => { if !IS_ERR((*ce).clk) { clk_put((*ce).clk); } }
        _ => {}
    }
    kfree((*ce).con_id); kfree(ce);
}

#[cfg(CONFIG_PM_CLK)]
pub unsafe fn pm_clk_remove_clk(dev: *mut device, clk: *mut clk) {
    let psd = dev_to_psd(dev); if psd.is_null() || clk.is_null() { return; }
    pm_clk_list_lock(psd);
    let mut ce = list_first_entry(&mut (*psd).clock_list, pm_clock_entry, node);
    while !ce.is_null() { if (*ce).clk == clk { list_del(&mut (*ce).node); if (*ce).enabled_when_prepared { (*psd).clock_op_might_sleep -= 1; } pm_clk_list_unlock(psd); __pm_clk_remove(ce); return; } ce = list_next_entry(ce, node); }
    pm_clk_list_unlock(psd);
}

#[cfg(CONFIG_PM_CLK)]
pub unsafe fn pm_clk_init(dev: *mut device) { let psd = dev_to_psd(dev); if !psd.is_null() { INIT_LIST_HEAD(&mut (*psd).clock_list); mutex_init(&mut (*psd).clock_mutex); (*psd).clock_op_might_sleep = 0; } }
#[cfg(CONFIG_PM_CLK)]
pub unsafe fn pm_clk_create(dev: *mut device) -> c_int { dev_pm_get_subsys_data(dev) }
#[cfg(CONFIG_PM_CLK)]
pub unsafe fn pm_clk_destroy(dev: *mut device) {
    let psd = dev_to_psd(dev); if psd.is_null() { return; }
    pm_clk_list_lock(psd); let mut ce = list_first_entry(&mut (*psd).clock_list, pm_clock_entry, node);
    while !ce.is_null() { let next = list_next_entry(ce, node); list_del(&mut (*ce).node); __pm_clk_remove(ce); ce = next; }
    (*psd).clock_op_might_sleep = 0; pm_clk_list_unlock(psd); dev_pm_put_subsys_data(dev);
}

#[cfg(CONFIG_PM_CLK)]
pub unsafe fn pm_clk_suspend(dev: *mut device) -> c_int {
    let psd = dev_to_psd(dev); if psd.is_null() { return 0; } let mut flags = 0; let ret = pm_clk_op_lock(psd, &mut flags, c"pm_clk_suspend".as_ptr()); if ret != 0 { return ret; }
    let mut ce = list_last_entry(&mut (*psd).clock_list, pm_clock_entry, node); while !ce.is_null() { if (*ce).status == pce_status::PCE_STATUS_ENABLED { if (*ce).enabled_when_prepared { clk_disable_unprepare((*ce).clk); (*ce).status = pce_status::PCE_STATUS_ACQUIRED; } else { clk_disable((*ce).clk); (*ce).status = pce_status::PCE_STATUS_PREPARED; } } ce = list_prev_entry(ce, node); } pm_clk_op_unlock(psd, &mut flags); 0
}

#[cfg(CONFIG_PM_CLK)]
pub unsafe fn pm_clk_resume(dev: *mut device) -> c_int { let psd = dev_to_psd(dev); if psd.is_null() { return 0; } let mut flags = 0; let ret = pm_clk_op_lock(psd, &mut flags, c"pm_clk_resume".as_ptr()); if ret != 0 { return ret; } let mut ce = list_first_entry(&mut (*psd).clock_list, pm_clock_entry, node); while !ce.is_null() { __pm_clk_enable(dev, ce); ce = list_next_entry(ce, node); } pm_clk_op_unlock(psd, &mut flags); 0 }

#[cfg(not(CONFIG_PM_CLK))]
unsafe fn enable_clock(dev: *mut device, con_id: *const c_char) { let clk = clk_get(dev, con_id); if !IS_ERR(clk) { clk_prepare_enable(clk); clk_put(clk); dev_info!(dev, "Runtime PM disabled, clock forced on.\n"); } }
#[cfg(not(CONFIG_PM_CLK))]
unsafe fn disable_clock(dev: *mut device, con_id: *const c_char) { let clk = clk_get(dev, con_id); if !IS_ERR(clk) { clk_disable_unprepare(clk); clk_put(clk); dev_info!(dev, "Runtime PM disabled, clock forced off.\n"); } }

pub unsafe fn pm_clk_runtime_suspend(dev: *mut device) -> c_int { let ret = pm_generic_runtime_suspend(dev); if ret != 0 { return ret; } let ret = pm_clk_suspend(dev); if ret != 0 { pm_generic_runtime_resume(dev); return ret; } 0 }
pub unsafe fn pm_clk_runtime_resume(dev: *mut device) -> c_int { let ret = pm_clk_resume(dev); if ret != 0 { return ret; } pm_generic_runtime_resume(dev) }

// The notifier implementations retain the two CONFIG_PM_CLK branches from
// the C source; notifier_block and pm_clk_notifier_block are external kernel
// types supplied by the surrounding translation.
unsafe fn pm_clk_notify(nb: *mut notifier_block, action: c_ulong, data: *mut c_void) -> c_int {
    let dev = data as *mut device;
    let clknb = container_of!(nb, pm_clk_notifier_block, nb);
    #[cfg(CONFIG_PM_CLK)] {
        match action {
            BUS_NOTIFY_ADD_DEVICE => {
                if !(*dev).pm_domain.is_null() { return 0; }
                if pm_clk_create(dev) == 0 {
                    dev_pm_domain_set(dev, (*clknb).pm_domain);
                    if !(*clknb).con_ids.is_null() && !(*(*clknb).con_ids).is_null() {
                        let mut p = (*clknb).con_ids;
                        while !(*p).is_null() { pm_clk_add(dev, *p); p = p.add(1); }
                    } else { pm_clk_add(dev, core::ptr::null()); }
                }
            }
            BUS_NOTIFY_DEL_DEVICE => {
                if (*dev).pm_domain == (*clknb).pm_domain { dev_pm_domain_set(dev, core::ptr::null_mut()); pm_clk_destroy(dev); }
            }
            _ => {}
        }
    }
    #[cfg(not(CONFIG_PM_CLK))] {
        match action {
            BUS_NOTIFY_BIND_DRIVER => {
                if !(*clknb).con_ids.is_null() && !(*(*clknb).con_ids).is_null() { let mut p = (*clknb).con_ids; while !(*p).is_null() { enable_clock(dev, *p); p = p.add(1); } } else { enable_clock(dev, core::ptr::null()); }
            }
            BUS_NOTIFY_DRIVER_NOT_BOUND | BUS_NOTIFY_UNBOUND_DRIVER => {
                if !(*clknb).con_ids.is_null() && !(*(*clknb).con_ids).is_null() { let mut p = (*clknb).con_ids; while !(*p).is_null() { disable_clock(dev, *p); p = p.add(1); } } else { disable_clock(dev, core::ptr::null()); }
            }
            _ => {}
        }
    }
    0
}

pub unsafe fn pm_clk_add_notifier(bus: *const bus_type, clknb: *mut pm_clk_notifier_block) {
    if bus.is_null() || clknb.is_null() { return; }
    (*clknb).nb.notifier_call = Some(pm_clk_notify);
    bus_register_notifier(bus, &mut (*clknb).nb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
