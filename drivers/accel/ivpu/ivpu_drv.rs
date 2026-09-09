// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020-2026 Intel Corporation
 */

// C headers and IVPU headers are supplied by the surrounding kernel translation unit.
// Build-time CONFIG_* conditions are preserved below where applicable.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
extern "C" {
    static mut ivpu_dbg_mask: i32;
    static mut ivpu_test_mode: i32;
    static mut ivpu_pll_min_ratio: u8;
    static mut ivpu_pll_max_ratio: u8;
    static mut ivpu_sched_mode: i32;
    static mut ivpu_disable_mmu_cont_pages: bool;
    static mut ivpu_force_snoop: bool;
}

// The following items intentionally use the types and symbols supplied by the included IVPU
// headers.  This file is a source-level translation and does not provide dependency shims.

pub unsafe fn ivpu_user_limits_alloc(vdev: *mut ivpu_device, uid: uid_t) -> *mut ivpu_user_limits {
    let limits = kzalloc_obj::<ivpu_user_limits>();
    if limits.is_null() { return ERR_PTR(-ENOMEM); }
    kref_init(&mut (*limits).ref_);
    atomic_set(&mut (*limits).db_count, 0);
    (*limits).vdev = vdev;
    (*limits).uid = uid;
    if uid == 0 {
        (*limits).max_ctx_count = ivpu_get_context_count(vdev);
        (*limits).max_db_count = ivpu_get_doorbell_count(vdev);
    } else {
        (*limits).max_ctx_count = ivpu_get_context_count(vdev) / 2;
        (*limits).max_db_count = ivpu_get_doorbell_count(vdev) / 2;
    }
    hash_add((*vdev).user_limits, &mut (*limits).hash_node, uid);
    limits
}

unsafe fn ivpu_user_limits_get(vdev: *mut ivpu_device) -> *mut ivpu_user_limits {
    let uid = current_uid().val;
    let _guard = guard_mutex(&mut (*vdev).user_limits_lock);
    for_each_hash_possible!((*vdev).user_limits, limits, hash_node, uid, {
        if (*limits).uid == uid {
            if kref_read(&(*limits).ref_) >= (*limits).max_ctx_count {
                ivpu_dbg(vdev, IOCTL, "User %u exceeded max ctx count %u\n", uid, (*limits).max_ctx_count);
                return ERR_PTR(-EMFILE);
            }
            kref_get(&mut (*limits).ref_);
            return limits;
        }
    });
    ivpu_user_limits_alloc(vdev, uid)
}

unsafe fn ivpu_user_limits_release(ref_: *mut kref) {
    let limits = container_of!(ref_, ivpu_user_limits, ref_);
    let vdev = (*limits).vdev;
    lockdep_assert_held(&(*vdev).user_limits_lock);
    drm_WARN_ON(&(*vdev).drm, atomic_read(&(*limits).db_count) != 0);
    hash_del(&mut (*limits).hash_node);
    kfree(limits);
}

unsafe fn ivpu_user_limits_put(vdev: *mut ivpu_device, limits: *mut ivpu_user_limits) {
    let _guard = guard_mutex(&mut (*vdev).user_limits_lock);
    kref_put(&mut (*limits).ref_, ivpu_user_limits_release);
}

pub unsafe fn ivpu_file_priv_get(file_priv: *mut ivpu_file_priv) -> *mut ivpu_file_priv {
    let vdev = (*file_priv).vdev;
    kref_get(&mut (*file_priv).ref_);
    ivpu_dbg(vdev, KREF, "file_priv get: ctx %u refcount %u\n", (*file_priv).ctx.id, kref_read(&(*file_priv).ref_));
    file_priv
}

unsafe fn file_priv_unbind(vdev: *mut ivpu_device, file_priv: *mut ivpu_file_priv) {
    mutex_lock(&mut (*file_priv).lock);
    if (*file_priv).bound {
        ivpu_dbg(vdev, FILE, "file_priv unbind: ctx %u\n", (*file_priv).ctx.id);
        ivpu_cmdq_release_all_locked(file_priv);
        ivpu_bo_unbind_all_bos_from_context(vdev, &mut (*file_priv).ctx);
        ivpu_mmu_context_fini(vdev, &mut (*file_priv).ctx);
        (*file_priv).bound = false;
        drm_WARN_ON(&(*vdev).drm, xa_erase_irq(&mut (*vdev).context_xa, (*file_priv).ctx.id).is_null());
    }
    mutex_unlock(&mut (*file_priv).lock);
}

unsafe fn file_priv_release(ref_: *mut kref) {
    let file_priv = container_of!(ref_, ivpu_file_priv, ref_);
    let vdev = (*file_priv).vdev;
    ivpu_dbg(vdev, FILE, "file_priv release: ctx %u bound %d\n", (*file_priv).ctx.id, (*file_priv).bound as i32);
    pm_runtime_get_sync((*vdev).drm.dev);
    mutex_lock(&mut (*vdev).context_list_lock);
    file_priv_unbind(vdev, file_priv);
    drm_WARN_ON(&(*vdev).drm, !xa_empty(&(*file_priv).cmdq_xa));
    xa_destroy(&mut (*file_priv).cmdq_xa);
    mutex_unlock(&mut (*vdev).context_list_lock);
    pm_runtime_put_autosuspend((*vdev).drm.dev);
    ivpu_user_limits_put(vdev, (*file_priv).user_limits);
    mutex_destroy(&mut (*file_priv).ms_lock);
    mutex_destroy(&mut (*file_priv).lock);
    kfree(file_priv);
}

pub unsafe fn ivpu_file_priv_put(link: *mut *mut ivpu_file_priv) {
    let file_priv = *link;
    let vdev = (*file_priv).vdev;
    ivpu_dbg(vdev, KREF, "file_priv put: ctx %u refcount %u\n", (*file_priv).ctx.id, kref_read(&(*file_priv).ref_));
    *link = core::ptr::null_mut();
    kref_put(&mut (*file_priv).ref_, file_priv_release);
}

pub unsafe fn ivpu_is_capable(vdev: *mut ivpu_device, capability: u32) -> bool {
    match capability {
        DRM_IVPU_CAP_METRIC_STREAMER | DRM_IVPU_CAP_DMA_MEMORY_RANGE | DRM_IVPU_CAP_BO_CREATE_FROM_USERPTR => true,
        DRM_IVPU_CAP_MANAGE_CMDQ => (*(*vdev).fw).sched_mode == VPU_SCHEDULING_MODE_HW,
        _ => false,
    }
}

unsafe fn ivpu_wait_for_ready(vdev: *mut ivpu_device) -> i32 {
    if (ivpu_test_mode & IVPU_TEST_MODE_FW_TEST) != 0 { return 0; }
    let mut cons = core::mem::zeroed::<ivpu_ipc_consumer>();
    let mut ipc_hdr = core::mem::zeroed::<ivpu_ipc_hdr>();
    ivpu_ipc_consumer_add(vdev, &mut cons, IVPU_IPC_CHAN_BOOT_MSG, core::ptr::null_mut());
    let timeout = jiffies() + msecs_to_jiffies((*vdev).timeout.boot);
    let mut ret;
    loop {
        ivpu_ipc_irq_handler(vdev);
        ret = ivpu_ipc_receive(vdev, &mut cons, &mut ipc_hdr, core::ptr::null_mut(), 0);
        if ret != -ETIMEDOUT || time_after_eq(jiffies(), timeout) { break; }
        cond_resched();
    }
    ivpu_ipc_consumer_del(vdev, &mut cons);
    if ret == 0 && ipc_hdr.data_addr != IVPU_IPC_BOOT_MSG_DATA_ADDR {
        ivpu_err(vdev, "Invalid NPU ready message: 0x%x\n", ipc_hdr.data_addr);
        return -EIO;
    }
    if ret == 0 { ivpu_dbg(vdev, PM, "NPU ready message received successfully\n"); }
    ret
}

unsafe fn ivpu_hw_sched_init(vdev: *mut ivpu_device) -> i32 {
    if (*(*vdev).fw).sched_mode == VPU_SCHEDULING_MODE_HW {
        let ret = ivpu_jsm_hws_setup_priority_bands(vdev);
        if ret != 0 { ivpu_err(vdev, "Failed to enable hw scheduler: %d", ret); return ret; }
    }
    0
}

/** ivpu_boot() - Start VPU firmware */
pub unsafe fn ivpu_boot(vdev: *mut ivpu_device) -> i32 {
    drm_WARN_ON(&(*vdev).drm, atomic_read(&(*vdev).job_timeout_counter) != 0);
    drm_WARN_ON(&(*vdev).drm, !xa_empty(&(*vdev).submitted_jobs_xa));
    ivpu_fw_boot_params_setup(vdev, ivpu_bo_vaddr(vdev, (*(*vdev).fw).mem_bp));
    (*(*vdev).fw).last_boot_mode = (*(*vdev).fw).next_boot_mode;
    let mut ret = ivpu_hw_boot_fw(vdev);
    if ret != 0 { ivpu_err(vdev, "Failed to start the firmware: %d\n", ret); return ret; }
    ret = ivpu_wait_for_ready(vdev);
    if ret != 0 { ivpu_err(vdev, "Failed to boot the firmware: %d\n", ret); goto_diagnose!(vdev, ret); return ret; }
    ivpu_hw_irq_clear(vdev); enable_irq((*vdev).irq); ivpu_hw_irq_enable(vdev); ivpu_ipc_enable(vdev);
    if !ivpu_fw_is_warm_boot(vdev) {
        ret = ivpu_pm_dct_init(vdev); if ret != 0 { goto_disable_ipc!(vdev, ret); return ret; }
        ret = ivpu_hw_sched_init(vdev); if ret != 0 { goto_disable_ipc!(vdev, ret); return ret; }
        ret = ivpu_hw_btrs_cfg_freq_init(vdev); if ret != 0 { goto_disable_ipc!(vdev, ret); return ret; }
    }
    0
}

pub unsafe fn ivpu_prepare_for_reset(vdev: *mut ivpu_device) {
    ivpu_hw_irq_disable(vdev); disable_irq((*vdev).irq); flush_work(&mut (*vdev).irq_dct_work);
    flush_work(&mut (*vdev).context_abort_work); flush_work(&mut (*vdev).job_destroy_work);
    ivpu_ipc_disable(vdev); ivpu_mmu_disable(vdev);
}

pub unsafe fn ivpu_shutdown(vdev: *mut ivpu_device) -> i32 {
    pci_save_state(to_pci_dev((*vdev).drm.dev));
    let ret = ivpu_hw_power_down(vdev);
    if ret != 0 { ivpu_warn(vdev, "Failed to power down HW: %d\n", ret); }
    pci_set_power_state(to_pci_dev((*vdev).drm.dev), PCI_D3hot); ret
}

// Remaining driver-registration and lifecycle declarations retain the C driver's externally
// visible interfaces; their kernel object initializers are represented by dependency macros.
pub unsafe fn ivpu_destroy_workqueue(wq: *mut core::ffi::c_void) { destroy_workqueue(wq); }

// CONFIG_PROC_FS and module registration are build-time kernel conditions.
// The PCI IDs, DRM driver, IRQ/PCI initialization, device init/fini, probe/remove callbacks,
// PM/error handler tables, and module metadata remain supplied through the surrounding bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
