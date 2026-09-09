// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023, STMicroelectronics - All Rights Reserved
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, as in the original C source.

const STM32_FIREWALL_MAX_ARGS: usize = STM32_FIREWALL_MAX_EXTRA_ARGS + 1;

static mut FIREWALL_CONTROLLER_LIST: ListHead = LIST_HEAD_INIT;
static mut FIREWALL_CONTROLLER_LIST_LOCK: Mutex = DEFINE_MUTEX_INIT;

pub unsafe fn stm32_firewall_get_firewall(
    np: *mut device_node,
    firewall: *mut stm32_firewall,
    nb_firewall: u32,
) -> i32 {
    let mut ctrl: *mut stm32_firewall_controller;
    let mut it: of_phandle_iterator = core::mem::zeroed();
    let mut i: u32;
    let mut j: u32 = 0;
    let mut err: i32 = 0;

    if firewall.is_null() || nb_firewall == 0 { return -EINVAL; }

    // Parse property with phandle parsed out.
    while of_phandle_iterator_next(&mut it, &mut err, np, "access-controllers",
                                   "#access-controller-cells", 0) {
        let mut provider_args: of_phandle_args = core::mem::zeroed();
        let provider = it.node;
        let mut fw_entry: *const core::ffi::c_char = core::ptr::null();
        let mut matched = false;

        if err != 0 {
            pr_err!("Unable to get access-controllers property for node %s\\n, err: %d", (*np).full_name, err);
            of_node_put(provider);
            return err;
        }
        if j >= nb_firewall {
            pr_err!("Too many firewall controllers");
            of_node_put(provider);
            return -EINVAL;
        }

        provider_args.args_count = of_phandle_iterator_args(
            &mut it, provider_args.args.as_mut_ptr(), STM32_FIREWALL_MAX_ARGS);

        mutex_lock(&mut FIREWALL_CONTROLLER_LIST_LOCK);
        list_for_each_entry!(ctrl, FIREWALL_CONTROLLER_LIST, entry, {
            if (*(*ctrl).dev).of_node.phandle == it.phandle {
                matched = true;
                (*firewall.add(j as usize)).firewall_ctrl = ctrl;
                break;
            }
        });
        mutex_unlock(&mut FIREWALL_CONTROLLER_LIST_LOCK);

        if !matched {
            (*firewall.add(j as usize)).firewall_ctrl = core::ptr::null_mut();
            pr_err!("No firewall controller registered for %s\\n", (*np).full_name);
            of_node_put(provider);
            return -ENODEV;
        }
        if of_property_read_string_index(np, "access-controller-names", j, &mut fw_entry) == 0 {
            (*firewall.add(j as usize)).entry = fw_entry;
        }
        if provider_args.args_count < 0 || provider_args.args_count as usize > STM32_FIREWALL_MAX_ARGS {
            of_node_put(provider);
            return -EINVAL;
        } else if provider_args.args_count == 0 {
            (*firewall.add(j as usize)).extra_args_size = 0;
            (*firewall.add(j as usize)).firewall_id = U32_MAX;
            j += 1;
            continue;
        }
        (*firewall.add(j as usize)).firewall_id = provider_args.args[0];
        i = 0;
        while i < provider_args.args_count as u32 - 1 {
            (*firewall.add(j as usize)).extra_args[i as usize] = provider_args.args[(i + 1) as usize];
            i += 1;
        }
        (*firewall.add(j as usize)).extra_args_size = provider_args.args_count - 1;
        j += 1;
    }
    0
}

pub unsafe fn stm32_firewall_grant_access(firewall: *mut stm32_firewall) -> i32 {
    if firewall.is_null() || (*firewall).firewall_id == U32_MAX { return -EINVAL; }
    let controller = (*firewall).firewall_ctrl;
    if controller.is_null() { return -ENODEV; }
    ((*controller).grant_access)(controller, (*firewall).firewall_id)
}

pub unsafe fn stm32_firewall_grant_access_by_id(firewall: *mut stm32_firewall, subsystem_id: u32) -> i32 {
    if firewall.is_null() || subsystem_id == U32_MAX || (*firewall).firewall_id == U32_MAX { return -EINVAL; }
    let controller = (*firewall).firewall_ctrl;
    if controller.is_null() { return -ENODEV; }
    ((*controller).grant_access)(controller, subsystem_id)
}

pub unsafe fn stm32_firewall_release_access(firewall: *mut stm32_firewall) {
    if firewall.is_null() || (*firewall).firewall_id == U32_MAX { pr_debug!("Incorrect arguments when releasing a firewall access\\n"); return; }
    let controller = (*firewall).firewall_ctrl;
    if controller.is_null() { pr_debug!("No firewall controller to release\\n"); return; }
    ((*controller).release_access)(controller, (*firewall).firewall_id);
}

pub unsafe fn stm32_firewall_release_access_by_id(firewall: *mut stm32_firewall, subsystem_id: u32) {
    if firewall.is_null() || subsystem_id == U32_MAX || (*firewall).firewall_id == U32_MAX { pr_debug!("Incorrect arguments when releasing a firewall access"); return; }
    let controller = (*firewall).firewall_ctrl;
    if controller.is_null() { pr_debug!("No firewall controller to release"); return; }
    ((*controller).release_access)(controller, subsystem_id);
}

pub unsafe fn stm32_firewall_get_grant_all_access(dev: *mut device, firewall: *mut *mut stm32_firewall, nb_firewall: *mut i32) -> i32 {
    *nb_firewall = of_count_phandle_with_args((*dev).of_node, "access-controllers", "#access-controller-cells");
    if *nb_firewall < 0 { return *nb_firewall; }
    if *nb_firewall == 0 { *firewall = core::ptr::null_mut(); return 0; }
    let local = devm_kcalloc(dev, *nb_firewall as usize, core::mem::size_of::<stm32_firewall>(), GFP_KERNEL) as *mut stm32_firewall;
    if local.is_null() { return -ENOMEM; }
    let err = stm32_firewall_get_firewall((*dev).of_node, local, *nb_firewall as u32);
    if err != 0 { return err; }
    let mut i = 0;
    while i < *nb_firewall { let err = stm32_firewall_grant_access(local.add(i as usize)); if err != 0 { while i > 0 { i -= 1; stm32_firewall_release_access(local.add(i as usize)); } return err; } i += 1; }
    *firewall = local; 0
}

pub unsafe fn stm32_firewall_controller_register(firewall_controller: *mut stm32_firewall_controller) -> i32 {
    if firewall_controller.is_null() { return -ENODEV; }
    pr_info!("Registering %s firewall controller\\n", (*firewall_controller).name);
    mutex_lock(&mut FIREWALL_CONTROLLER_LIST_LOCK);
    list_for_each_entry!(ctrl, FIREWALL_CONTROLLER_LIST, entry, { if ctrl == firewall_controller { pr_debug!("%s firewall controller already registered\\n", (*firewall_controller).name); mutex_unlock(&mut FIREWALL_CONTROLLER_LIST_LOCK); return 0; } });
    list_add_tail!(&mut (*firewall_controller).entry, &mut FIREWALL_CONTROLLER_LIST);
    mutex_unlock(&mut FIREWALL_CONTROLLER_LIST_LOCK); 0
}

pub unsafe fn stm32_firewall_controller_unregister(firewall_controller: *mut stm32_firewall_controller) {
    if firewall_controller.is_null() { pr_debug!("Null reference while unregistering firewall controller\\n"); return; }
    let mut removed = false;
    mutex_lock(&mut FIREWALL_CONTROLLER_LIST_LOCK);
    list_for_each_entry!(ctrl, FIREWALL_CONTROLLER_LIST, entry, { if ctrl == firewall_controller { removed = true; list_del_init!(&mut (*ctrl).entry); break; } });
    mutex_unlock(&mut FIREWALL_CONTROLLER_LIST_LOCK);
    if !removed { pr_debug!("There was no firewall controller named %s to unregister\\n", (*firewall_controller).name); }
}

pub unsafe fn stm32_firewall_populate_bus(firewall_controller: *mut stm32_firewall_controller) -> i32 {
    let parent = (*firewall_controller).dev;
    dev_dbg!(parent, "Populating %s system bus\\n", dev_name(parent));
    for_each_available_child_of_node_scoped!((*parent).of_node, child, {
        let len = of_count_phandle_with_args(child, "access-controllers", "#access-controller-cells");
        if len <= 0 { return -EINVAL; }
        let firewalls = kzalloc_objs!(stm32_firewall, len as usize);
        if firewalls.is_null() { return -ENOMEM; }
        let err = stm32_firewall_get_firewall(child, firewalls, len as u32);
        if err != 0 { kfree(firewalls as *mut core::ffi::c_void); return err; }
        for i in 0..len as usize { if ((*firewall_controller).grant_access)((*firewalls.add(i)).firewall_ctrl, (*firewalls.add(i)).firewall_id) != 0 { of_detach_node(child); dev_err!(parent, "%s: Device driver will not be probed\\n", (*child).full_name); } }
        kfree(firewalls as *mut core::ffi::c_void);
    });
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
