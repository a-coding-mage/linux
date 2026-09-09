// SPDX-License-Identifier: GPL-2.0-only OR MIT
/* Copyright 2026 Arm, Ltd. */
/* Based on v3d_perfmon.c, Copyright (C) 2021 Raspberry Pi */

// Linux/DRM and driver declarations are supplied by the surrounding Rust translation.

pub unsafe fn ethosu_perfmon_get(perfmon: *mut ethosu_perfmon) {
    if !perfmon.is_null() {
        refcount_inc(&mut (*perfmon).refcnt);
    }
}

pub unsafe fn ethosu_perfmon_put(perfmon: *mut ethosu_perfmon) {
    if !perfmon.is_null() && refcount_dec_and_test(&mut (*perfmon).refcnt) {
        kfree(perfmon);
    }
}

pub unsafe fn ethosu_perfmon_start(ethosu: *mut ethosu_device, perfmon: *mut ethosu_perfmon) {
    let mut i: u32;
    let ncounters: u8;
    let mut mask: u32;

    lockdep_assert_held(&mut (*(*ethosu).perfmon_state.lock));
    if warn_on_once(perfmon.is_null() || !(*ethosu).perfmon_state.active.is_null()) {
        return;
    }

    writel_relaxed(PMCR_CNT_EN, (*ethosu).pmu_regs.add(NPU_REG_PMCR as usize));
    writel_relaxed(PMU_EV_TYPE_CYCLES, (*ethosu).pmu_regs.add(NPU_REG_PMCCNTR_CFG as usize));

    mask = 0x80000000;
    ncounters = (*perfmon).ncounters.wrapping_sub(1);
    if ncounters != 0 { mask |= genmask(ncounters as u32 - 1, 0); }
    i = 0;
    while i < ncounters as u32 {
        writel_relaxed((*perfmon).counters[i as usize], (*ethosu).pmu_regs.add(npu_reg_pmuper(i) as usize));
        i += 1;
    }
    writel_relaxed(mask, (*ethosu).pmu_regs.add(NPU_REG_PMCNTENSET as usize));
    writel_relaxed(PMCR_CNT_EN | PMCR_EVENT_CNT_RST | PMCR_CYCLE_CNT_RST,
                   (*ethosu).pmu_regs.add(NPU_REG_PMCR as usize));
    (*ethosu).perfmon_state.active = perfmon;
}

pub unsafe fn ethosu_perfmon_stop_locked(ethosu: *mut ethosu_device, perfmon: *mut ethosu_perfmon, capture: bool) {
    let mut i: u32;
    let ncounters: u8;
    let mut mask: u32;
    lockdep_assert_held(&mut (*(*ethosu).perfmon_state.lock));
    if perfmon.is_null() || perfmon != (*ethosu).perfmon_state.active { return; }
    ncounters = (*perfmon).ncounters.wrapping_sub(1);
    if !pm_runtime_get_if_active((*(*ethosu).base).dev) {
        (*ethosu).perfmon_state.active = core::ptr::null_mut(); return;
    }
    if capture {
        i = 0;
        while i < ncounters as u32 {
            (*perfmon).values[i as usize] = (*perfmon).values[i as usize].wrapping_add(readl_relaxed((*ethosu).pmu_regs.add(npu_reg_pmu_evcntr(i) as usize)) as u64);
            i += 1;
        }
        (*perfmon).values[ncounters as usize] = (*perfmon).values[ncounters as usize].wrapping_add(
            (readl_relaxed((*ethosu).pmu_regs.add(NPU_REG_PMCCNTR_LO as usize)) as u64) |
            ((readl_relaxed((*ethosu).pmu_regs.add(NPU_REG_PMCCNTR_HI as usize)) as u64) << 32));
    }
    mask = 0x80000000;
    if ncounters != 0 { mask |= genmask(ncounters as u32 - 1, 0); }
    writel_relaxed(mask, (*ethosu).pmu_regs.add(NPU_REG_PMCNTENCLR as usize));
    writel_relaxed(0, (*ethosu).pmu_regs.add(NPU_REG_PMCR as usize));
    (*ethosu).perfmon_state.active = core::ptr::null_mut();
    pm_runtime_put((*(*ethosu).base).dev);
}

pub unsafe fn ethosu_perfmon_stop(ethosu: *mut ethosu_device, perfmon: *mut ethosu_perfmon, capture: bool) {
    if perfmon.is_null() { return; }
    let _guard = mutex_guard(&mut (*(*ethosu).perfmon_state.lock));
    ethosu_perfmon_stop_locked(ethosu, perfmon, capture);
}

pub unsafe fn ethosu_perfmon_find(priv_: *mut ethosu_file_priv, id: i32) -> *mut ethosu_perfmon {
    xa_lock(&mut (*priv_).perfmons);
    let perfmon = xa_load(&mut (*priv_).perfmons, id as usize);
    ethosu_perfmon_get(perfmon);
    xa_unlock(&mut (*priv_).perfmons);
    perfmon
}

pub unsafe fn ethosu_perfmon_open_file(priv_: *mut ethosu_file_priv) {
    xa_init_flags(&mut (*priv_).perfmons, XA_FLAGS_ALLOC1);
}

unsafe fn ethosu_perfmon_delete(priv_: *mut ethosu_file_priv, perfmon: *mut ethosu_perfmon) {
    let ethosu = (*priv_).edev;
    let _guard = mutex_guard(&mut (*(*ethosu).perfmon_state.lock));
    if (*ethosu).global_perfmon == perfmon {
        (*ethosu).global_perfmon = core::ptr::null_mut();
        ethosu_perfmon_put(perfmon);
    }
    ethosu_perfmon_stop_locked(ethosu, perfmon, false);
    drop(_guard);
    ethosu_perfmon_put(perfmon);
}

pub unsafe fn ethosu_perfmon_close_file(priv_: *mut ethosu_file_priv) {
    let mut id: usize = 0;
    let mut perfmon: *mut ethosu_perfmon;
    while xa_for_each(&mut (*priv_).perfmons, &mut id, &mut perfmon) {
        ethosu_perfmon_delete(priv_, perfmon);
    }
    xa_destroy(&mut (*priv_).perfmons);
}

pub unsafe fn ethosu_ioctl_perfmon_create(dev: *mut drm_device, data: *mut core::ffi::c_void, file_priv: *mut drm_file) -> i32 {
    let priv_ = (*file_priv).driver_priv as *mut ethosu_file_priv;
    let req = data as *mut drm_ethosu_perfmon_create;
    let ethosu = to_ethosu_device(dev);
    let event_max = if ethosu_is_u65(ethosu) { 433 } else { 671 };
    if (*req).ncounters > (*ethosu).npu_info.pmu_counters { return -EINVAL; }
    for i in 0..(*req).ncounters as usize { if (*req).counters[i] > event_max { return -EINVAL; } }
    (*req).ncounters += 1;
    let perfmon = kzalloc_flex((*req).ncounters);
    if perfmon.is_null() { return -ENOMEM; }
    for i in 0..((*req).ncounters - 1) as usize { (*perfmon).counters[i] = (*req).counters[i]; }
    (*perfmon).ncounters = (*req).ncounters;
    refcount_set(&mut (*perfmon).refcnt, 1);
    let mut id = 0u32;
    let ret = xa_alloc(&mut (*priv_).perfmons, &mut id, perfmon, xa_limit_32b, GFP_KERNEL);
    if ret < 0 { kfree(perfmon); return ret; }
    (*req).id = id;
    0
}

pub unsafe fn ethosu_ioctl_perfmon_destroy(_dev: *mut drm_device, data: *mut core::ffi::c_void, file_priv: *mut drm_file) -> i32 {
    let priv_ = (*file_priv).driver_priv as *mut ethosu_file_priv;
    let req = data as *mut drm_ethosu_perfmon_destroy;
    let perfmon = xa_erase(&mut (*priv_).perfmons, (*req).id as usize);
    if perfmon.is_null() { return -EINVAL; }
    ethosu_perfmon_delete(priv_, perfmon); 0
}

pub unsafe fn ethosu_ioctl_perfmon_get_values(dev: *mut drm_device, data: *mut core::ffi::c_void, file_priv: *mut drm_file) -> i32 {
    let ethosu = to_ethosu_device(dev);
    let priv_ = (*file_priv).driver_priv as *mut ethosu_file_priv;
    let req = data as *mut drm_ethosu_perfmon_get_values;
    if (*req).pad != 0 { return -EINVAL; }
    let perfmon = ethosu_perfmon_find(priv_, (*req).id as i32);
    if perfmon.is_null() { return -EINVAL; }
    let ret = pm_runtime_resume_and_get((*dev).dev);
    if ret != 0 { ethosu_perfmon_put(perfmon); return ret; }
    ethosu_perfmon_stop(ethosu, perfmon, true);
    pm_runtime_put_autosuspend((*dev).dev);
    let mut result = 0;
    if copy_to_user(u64_to_user_ptr((*req).values_ptr), (*perfmon).values, (*perfmon).ncounters as usize * core::mem::size_of::<u64>()) { result = -EFAULT; }
    ethosu_perfmon_put(perfmon); result
}

pub unsafe fn ethosu_ioctl_perfmon_set_global(dev: *mut drm_device, data: *mut core::ffi::c_void, file_priv: *mut drm_file) -> i32 {
    let priv_ = (*file_priv).driver_priv as *mut ethosu_file_priv;
    let req = data as *mut drm_ethosu_perfmon_set_global;
    let ethosu = to_ethosu_device(dev);
    if (*req).flags & !DRM_ETHOSU_PERFMON_CLEAR_GLOBAL != 0 { return -EINVAL; }
    let perfmon = ethosu_perfmon_find(priv_, (*req).id as i32);
    if perfmon.is_null() { return -EINVAL; }
    let _guard = mutex_guard(&mut (*(*ethosu).perfmon_state.lock));
    if (*req).flags & DRM_ETHOSU_PERFMON_CLEAR_GLOBAL != 0 {
        let old = (*ethosu).global_perfmon;
        if old.is_null() { ethosu_perfmon_put(perfmon); return -EINVAL; }
        (*ethosu).global_perfmon = core::ptr::null_mut();
        ethosu_perfmon_stop_locked(ethosu, old, true);
        drop(_guard); ethosu_perfmon_put(old); ethosu_perfmon_put(perfmon); return 0;
    }
    if !(*ethosu).perfmon_state.active.is_null() || !(*ethosu).global_perfmon.is_null() { ethosu_perfmon_put(perfmon); return -EBUSY; }
    (*ethosu).global_perfmon = perfmon; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
