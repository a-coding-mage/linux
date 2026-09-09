// SPDX-License-Identifier: GPL-2.0
/*
 * Resource Director Technology (RDT)
 *
 * Pseudo-locking support built on top of Cache Allocation Technology (CAT)
 *
 * Copyright (C) 2018 Intel Corporation
 *
 * Author: Reinette Chatre <reinette.chatre@intel.com>
 */

// Kernel dependencies supplied by the surrounding translation unit.

static mut pseudo_lock_major: u32 = 0;
static mut pseudo_lock_minor_avail: c_ulong = GENMASK(MINORBITS, 0);

unsafe fn pseudo_lock_devnode(dev: *const device, mode: *mut umode_t) -> *mut c_char {
    let rdtgrp = dev_get_drvdata(dev);
    if !mode.is_null() { *mode = 0o600; }
    let _guard = mutex_guard(&rdtgroup_mutex);
    kasprintf(GFP_KERNEL, cstr!("pseudo_lock/%s"), rdt_kn_name((*rdtgrp).kn))
}

static pseudo_lock_class: class = class { name: cstr!("pseudo_lock"), devnode: Some(pseudo_lock_devnode) };

unsafe fn pseudo_lock_minor_get(minor: *mut u32) -> c_int {
    let first_bit = find_first_bit(&raw mut pseudo_lock_minor_avail, MINORBITS);
    if first_bit == MINORBITS { return -ENOSPC; }
    __clear_bit(first_bit, &raw mut pseudo_lock_minor_avail);
    *minor = first_bit as u32;
    0
}

unsafe fn pseudo_lock_minor_release(minor: u32) { __set_bit(minor as usize, &raw mut pseudo_lock_minor_avail); }

unsafe fn region_find_by_minor(minor: u32) -> *mut rdtgroup {
    let mut rdtgrp_match = core::ptr::null_mut();
    list_for_each_entry!(rdtgrp, rdt_all_groups, rdtgroup_list, {
        if !(*rdtgrp).plr.is_null() && (*(*rdtgrp).plr).minor == minor { rdtgrp_match = rdtgrp; break; }
    });
    rdtgrp_match
}

struct pseudo_lock_pm_req { list: list_head, req: dev_pm_qos_request }

unsafe fn pseudo_lock_cstates_relax(plr: *mut pseudo_lock_region) {
    list_for_each_entry_safe!(pm_req, next, (*plr).pm_reqs, list, {
        dev_pm_qos_remove_request(&mut (*pm_req).req);
        list_del(&mut (*pm_req).list);
        kfree(pm_req);
    });
}

unsafe fn pseudo_lock_cstates_constrain(plr: *mut pseudo_lock_region) -> c_int {
    let mut ret;
    for_each_cpu!(cpu, (*(*plr).d).hdr.cpu_mask, {
        let pm_req = kzalloc_obj::<pseudo_lock_pm_req>();
        if pm_req.is_null() {
            rdt_last_cmd_puts(cstr!("Failure to allocate memory for PM QoS\n"));
            ret = -ENOMEM; goto!(out_err);
        }
        ret = dev_pm_qos_add_request(get_cpu_device(cpu), &mut (*pm_req).req,
                                     DEV_PM_QOS_RESUME_LATENCY, 30);
        if ret < 0 {
            rdt_last_cmd_printf(cstr!("Failed to add latency req CPU%d\n"), cpu);
            kfree(pm_req); ret = -1; goto!(out_err);
        }
        list_add(&mut (*pm_req).list, &mut (*plr).pm_reqs);
    });
    return 0;
out_err:
    pseudo_lock_cstates_relax(plr); ret
}

unsafe fn pseudo_lock_region_clear(plr: *mut pseudo_lock_region) {
    (*plr).size = 0; (*plr).line_size = 0; kfree((*plr).kmem); (*plr).kmem = core::ptr::null_mut();
    (*plr).s = core::ptr::null_mut(); if !(*plr).d.is_null() { (*(*plr).d).plr = core::ptr::null_mut(); }
    (*plr).d = core::ptr::null_mut(); (*plr).cbm = 0; (*plr).debugfs_dir = core::ptr::null_mut();
}

unsafe fn pseudo_lock_region_init(plr: *mut pseudo_lock_region) -> c_int {
    let scope = (*(*plr).s).res.ctrl_scope; let ci;
    if WARN_ON_ONCE(scope != RESCTRL_L2_CACHE && scope != RESCTRL_L3_CACHE) { return -ENODEV; }
    (*plr).cpu = cpumask_first(&(*(*plr).d).hdr.cpu_mask);
    if !cpu_online((*plr).cpu) { rdt_last_cmd_printf(cstr!("CPU %u associated with cache not online\n"), (*plr).cpu); goto!(out_region); }
    ci = get_cpu_cacheinfo_level((*plr).cpu, scope);
    if !ci.is_null() { (*plr).line_size = (*ci).coherency_line_size; (*plr).size = rdtgroup_cbm_to_size((*(*plr).s).res, (*plr).d, (*plr).cbm); return 0; }
    rdt_last_cmd_puts(cstr!("Unable to determine cache line size\n"));
out_region: pseudo_lock_region_clear(plr); -1
}

unsafe fn pseudo_lock_init(rdtgrp: *mut rdtgroup) -> c_int {
    let plr = kzalloc_obj::<pseudo_lock_region>(); if plr.is_null() { return -ENOMEM; }
    init_waitqueue_head(&mut (*plr).lock_thread_wq); INIT_LIST_HEAD(&mut (*plr).pm_reqs); (*rdtgrp).plr = plr; 0
}

unsafe fn pseudo_lock_region_alloc(plr: *mut pseudo_lock_region) -> c_int {
    let mut ret = pseudo_lock_region_init(plr); if ret < 0 { return ret; }
    if (*plr).size > KMALLOC_MAX_SIZE { rdt_last_cmd_puts(cstr!("Requested region exceeds maximum size\n")); ret = -E2BIG; goto!(out_region); }
    (*plr).kmem = kzalloc((*plr).size, GFP_KERNEL);
    if (*plr).kmem.is_null() { rdt_last_cmd_puts(cstr!("Unable to allocate memory\n")); ret = -ENOMEM; goto!(out_region); }
    return 0;
out_region: pseudo_lock_region_clear(plr); ret
}

unsafe fn pseudo_lock_free(rdtgrp: *mut rdtgroup) { pseudo_lock_region_clear((*rdtgrp).plr); kfree((*rdtgrp).plr); (*rdtgrp).plr = core::ptr::null_mut(); }
unsafe fn rdtgroup_monitor_in_progress(rdtgrp: *mut rdtgroup) -> c_int { (!list_empty(&(*rdtgrp).mon.crdtgrp_list)) as c_int }

unsafe fn rdtgroup_locksetup_user_restrict(r: *mut rdtgroup) -> c_int {
    let mut ret = rdtgroup_kn_mode_restrict(r, cstr!("tasks")); if ret != 0 { return ret; }
    ret = rdtgroup_kn_mode_restrict(r, cstr!("cpus")); if ret != 0 { rdtgroup_kn_mode_restore(r,cstr!("tasks"),0o777); return ret; }
    ret = rdtgroup_kn_mode_restrict(r,cstr!("cpus_list")); if ret != 0 { rdtgroup_kn_mode_restore(r,cstr!("cpus"),0o777); rdtgroup_kn_mode_restore(r,cstr!("tasks"),0o777); return ret; }
    if resctrl_arch_mon_capable() { ret = rdtgroup_kn_mode_restrict(r,cstr!("mon_groups")); if ret != 0 { rdtgroup_kn_mode_restore(r,cstr!("cpus_list"),0o777); rdtgroup_kn_mode_restore(r,cstr!("cpus"),0o777); rdtgroup_kn_mode_restore(r,cstr!("tasks"),0o777); return ret; } }
    0
}

unsafe fn rdtgroup_locksetup_user_restore(r: *mut rdtgroup) -> c_int {
    let mut ret = rdtgroup_kn_mode_restore(r,cstr!("tasks"),0o777); if ret != 0 { return ret; }
    ret = rdtgroup_kn_mode_restore(r,cstr!("cpus"),0o777); if ret != 0 { rdtgroup_kn_mode_restrict(r,cstr!("tasks")); return ret; }
    ret = rdtgroup_kn_mode_restore(r,cstr!("cpus_list"),0o777); if ret != 0 { rdtgroup_kn_mode_restrict(r,cstr!("cpus")); rdtgroup_kn_mode_restrict(r,cstr!("tasks")); return ret; }
    if resctrl_arch_mon_capable() { ret = rdtgroup_kn_mode_restore(r,cstr!("mon_groups"),0o777); if ret != 0 { rdtgroup_kn_mode_restrict(r,cstr!("cpus_list")); rdtgroup_kn_mode_restrict(r,cstr!("cpus")); rdtgroup_kn_mode_restrict(r,cstr!("tasks")); return ret; } }
    0
}

// Remaining public entry points retain the C control flow and call the kernel
// interfaces supplied by the surrounding translation unit.
unsafe fn rdtgroup_locksetup_enter(rdtgrp: *mut rdtgroup) -> c_int {
    if rdtgrp == &raw mut rdtgroup_default { rdt_last_cmd_puts(cstr!("Cannot pseudo-lock default group\n")); return -EINVAL; }
    if resctrl_arch_get_cdp_enabled(RDT_RESOURCE_L3) || resctrl_arch_get_cdp_enabled(RDT_RESOURCE_L2) { rdt_last_cmd_puts(cstr!("CDP enabled\n")); return -EINVAL; }
    if resctrl_arch_get_prefetch_disable_bits() == 0 { rdt_last_cmd_puts(cstr!("Pseudo-locking not supported\n")); return -EINVAL; }
    if rdtgroup_monitor_in_progress(rdtgrp) != 0 { rdt_last_cmd_puts(cstr!("Monitoring in progress\n")); return -EINVAL; }
    if rdtgroup_tasks_assigned(rdtgrp) { rdt_last_cmd_puts(cstr!("Tasks assigned to resource group\n")); return -EINVAL; }
    if !cpumask_empty(&(*rdtgrp).cpu_mask) { rdt_last_cmd_puts(cstr!("CPUs assigned to resource group\n")); return -EINVAL; }
    if rdtgroup_locksetup_user_restrict(rdtgrp) != 0 { rdt_last_cmd_puts(cstr!("Unable to modify resctrl permissions\n")); return -EIO; }
    let ret = pseudo_lock_init(rdtgrp); if ret != 0 { rdt_last_cmd_puts(cstr!("Unable to init pseudo-lock region\n")); rdtgroup_locksetup_user_restore(rdtgrp); return ret; }
    free_rmid((*rdtgrp).closid, (*rdtgrp).mon.rmid); 0
}

unsafe fn rdtgroup_locksetup_exit(r: *mut rdtgroup) -> c_int {
    if resctrl_arch_mon_capable() { let ret = alloc_rmid((*r).closid); if ret < 0 { rdt_last_cmd_puts(cstr!("Out of RMIDs\n")); return ret; } (*r).mon.rmid = ret; }
    let ret = rdtgroup_locksetup_user_restore(r); if ret != 0 { free_rmid((*r).closid, (*r).mon.rmid); return ret; }
    pseudo_lock_free(r); 0
}

unsafe fn rdtgroup_cbm_overlaps_pseudo_locked(d: *mut rdt_ctrl_domain, cbm: c_ulong) -> bool {
    if !(*d).plr.is_null() { let len = (*(*(*d).plr).s).res.cache.cbm_len; let b = (*(*d).plr).cbm; if bitmap_intersects(&cbm,&b,len) { return true; } } false
}

unsafe fn rdtgroup_pseudo_locked_in_hierarchy(d: *mut rdt_ctrl_domain) -> bool {
    lockdep_assert_cpus_held(); let cpu_with_psl = zalloc_cpumask_var(GFP_KERNEL); if cpu_with_psl.is_null() { return true; }
    for_each_alloc_capable_rdt_resource!(r, { list_for_each_entry_rcu!(d_i, (*r).ctrl_domains, hdr.list, lockdep_is_cpus_held(), { if !(*d_i).plr.is_null() { cpumask_or(cpu_with_psl,cpu_with_psl,&(*d_i).hdr.cpu_mask); } }); });
    let ret = cpumask_intersects(&(*d).hdr.cpu_mask,cpu_with_psl); free_cpumask_var(cpu_with_psl); ret
}

// File-operation and device-registration definitions are direct declarations;
// their kernel ABI fields and helper implementations are external.
unsafe fn rdt_pseudo_lock_init() -> c_int { let ret = register_chrdev(0,cstr!("pseudo_lock"),&pseudo_lock_dev_fops); if ret < 0 { return ret; } pseudo_lock_major=ret as u32; let ret=class_register(&pseudo_lock_class); if ret != 0 { unregister_chrdev(pseudo_lock_major as c_int,cstr!("pseudo_lock")); return ret; } 0 }
unsafe fn rdt_pseudo_lock_release() { class_unregister(&pseudo_lock_class); unregister_chrdev(pseudo_lock_major as c_int,cstr!("pseudo_lock")); pseudo_lock_major=0; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
