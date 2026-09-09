// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020-2026 Intel Corporation
 */

// Linux kernel dependencies and project headers are supplied by other files.

static mut ivpu_disable_recovery: bool = false;
static mut ivpu_tdr_timeout_ms: c_ulong = 0;
static mut ivpu_inference_timeout_ms: c_ulong = 0;

const PM_RESCHEDULE_LIMIT: i32 = 5;

unsafe fn ivpu_pm_prepare_cold_boot(vdev: *mut ivpu_device) {
    let fw = (*vdev).fw;

    ivpu_cmdq_reset_all_contexts(vdev);
    ivpu_ipc_reset(vdev);
    ivpu_fw_log_reset(vdev);
    ivpu_fw_load(vdev);
    (*fw).last_heartbeat = 0;

    ivpu_dbg(vdev, FW_BOOT, "Cold boot entry point 0x%llx", (*vdev).fw.cold_boot_entry_point);
    (*fw).next_boot_mode = VPU_BOOT_TYPE_COLDBOOT;
}

unsafe fn ivpu_pm_prepare_warm_boot(vdev: *mut ivpu_device) {
    let fw = (*vdev).fw;
    let bp = ivpu_bo_vaddr((*fw).mem_bp);

    (*fw).warm_boot_entry_point = (*bp).save_restore_ret_address;
    if (*fw).warm_boot_entry_point == 0 {
        ivpu_pm_prepare_cold_boot(vdev);
        return;
    }

    ivpu_dbg(vdev, FW_BOOT, "Warm boot entry point 0x%llx", (*fw).warm_boot_entry_point);
    (*fw).next_boot_mode = VPU_BOOT_TYPE_WARMBOOT;
}

unsafe fn ivpu_suspend(vdev: *mut ivpu_device) -> c_int {
    let ret;

    ivpu_prepare_for_reset(vdev);
    ret = ivpu_shutdown(vdev);
    if ret != 0 {
        ivpu_err(vdev, "Failed to shutdown NPU: %d\n", ret);
    }
    ret
}

unsafe fn ivpu_resume(vdev: *mut ivpu_device) -> c_int {
    let mut ret;

    'retry: loop {
        pci_set_power_state(to_pci_dev((*vdev).drm.dev), PCI_D0);
        pci_restore_state(to_pci_dev((*vdev).drm.dev));

        ret = ivpu_hw_power_up(vdev);
        if ret != 0 {
            ivpu_err(vdev, "Failed to power up HW: %d\n", ret);
            goto_err_power_down: {
                ivpu_hw_power_down(vdev);
                pci_set_power_state(to_pci_dev((*vdev).drm.dev), PCI_D3hot);
                if ivpu_fw_is_warm_boot(vdev) {
                    ivpu_pm_prepare_cold_boot(vdev);
                    continue 'retry;
                }
                ivpu_err(vdev, "Failed to resume the FW: %d\n", ret);
                return ret;
            }
        }

        ret = ivpu_mmu_enable(vdev);
        if ret != 0 {
            ivpu_err(vdev, "Failed to resume MMU: %d\n", ret);
            ivpu_hw_power_down(vdev);
            pci_set_power_state(to_pci_dev((*vdev).drm.dev), PCI_D3hot);
            if ivpu_fw_is_warm_boot(vdev) {
                ivpu_pm_prepare_cold_boot(vdev);
                continue 'retry;
            }
            ivpu_err(vdev, "Failed to resume the FW: %d\n", ret);
            return ret;
        }

        ret = ivpu_boot(vdev);
        if ret == 0 { return 0; }
        ivpu_mmu_disable(vdev);
        ivpu_hw_power_down(vdev);
        pci_set_power_state(to_pci_dev((*vdev).drm.dev), PCI_D3hot);
        if ivpu_fw_is_warm_boot(vdev) {
            ivpu_pm_prepare_cold_boot(vdev);
            continue 'retry;
        }
        ivpu_err(vdev, "Failed to resume the FW: %d\n", ret);
        return ret;
    }
}

unsafe fn ivpu_pm_reset_begin(vdev: *mut ivpu_device) {
    pm_runtime_disable((*vdev).drm.dev);
    atomic_inc(&mut (*(*vdev).pm).reset_counter);
    atomic_set(&mut (*(*vdev).pm).reset_pending, 1);
    down_write(&mut (*(*vdev).pm).reset_lock);
}

unsafe fn ivpu_pm_reset_complete(vdev: *mut ivpu_device) {
    ivpu_pm_prepare_cold_boot(vdev);
    ivpu_jobs_abort_all(vdev);
    ivpu_ms_cleanup_all(vdev);
    let ret = ivpu_resume(vdev);
    if ret != 0 { ivpu_err(vdev, "Failed to resume NPU: %d\n", ret); pm_runtime_set_suspended((*vdev).drm.dev); }
    else { pm_runtime_set_active((*vdev).drm.dev); }
    up_write(&mut (*(*vdev).pm).reset_lock);
    atomic_set(&mut (*(*vdev).pm).reset_pending, 0);
    pm_runtime_mark_last_busy((*vdev).drm.dev);
    pm_runtime_enable((*vdev).drm.dev);
}

unsafe fn ivpu_pm_recovery_work(work: *mut work_struct) {
    let pm = container_of!(work, ivpu_pm_info, recovery_work);
    let vdev = (*pm).vdev;
    let mut evt: [*mut c_char; 2] = [c"IVPU_PM_EVENT=IVPU_RECOVER".as_ptr() as *mut c_char, core::ptr::null_mut()];
    ivpu_err(vdev, "Recovering the NPU (reset #%d)\n", atomic_read(&(*(*vdev).pm).reset_counter));
    ivpu_pm_reset_begin(vdev);
    if !pm_runtime_status_suspended((*vdev).drm.dev) {
        ivpu_jsm_state_dump_no_reply(vdev); ivpu_dev_coredump(vdev); ivpu_suspend(vdev);
    }
    ivpu_pm_reset_complete(vdev);
    kobject_uevent_env(&mut (*(*vdev).drm.dev).kobj, KOBJ_CHANGE, evt.as_mut_ptr());
}

pub unsafe fn ivpu_pm_trigger_recovery(vdev: *mut ivpu_device, reason: *const c_char) {
    ivpu_err(vdev, "Recovery triggered by %s\n", reason);
    if ivpu_disable_recovery { ivpu_err(vdev, "Recovery not available when disable_recovery param is set\n"); return; }
    if atomic_cmpxchg(&mut (*(*vdev).pm).reset_pending, 0, 1) == 0 {
        ivpu_hw_diagnose_failure(vdev); ivpu_hw_irq_disable(vdev);
        queue_work(system_dfl_wq, &mut (*(*vdev).pm).recovery_work);
    }
}

unsafe fn ivpu_job_timeout_work(work: *mut work_struct) {
    let pm = container_of!(work, ivpu_pm_info, job_timeout_work.work);
    let vdev = (*pm).vdev;
    let timeout_ms = if ivpu_tdr_timeout_ms != 0 { ivpu_tdr_timeout_ms } else { (*vdev).timeout.tdr };
    let inference_timeout_ms = if ivpu_inference_timeout_ms != 0 { ivpu_inference_timeout_ms } else { (*vdev).timeout.inference };
    let mut heartbeat: u64 = 0;
    if ivpu_jsm_get_heartbeat(vdev, 0, &mut heartbeat) != 0 || heartbeat <= (*(*vdev).fw).last_heartbeat {
        ivpu_err(vdev, "Job timeout detected, heartbeat not progressed\n");
    } else {
        let inference_max_retries = (inference_timeout_ms + timeout_ms - 1) / timeout_ms;
        if atomic_fetch_inc(&mut (*vdev).job_timeout_counter) < inference_max_retries as _ {
            (*(*vdev).fw).last_heartbeat = heartbeat; ivpu_start_job_timeout_detection(vdev); return;
        }
        ivpu_err(vdev, "Job timeout detected, heartbeat limit (%lld) exceeded\n", inference_max_retries);
    }
    atomic_set(&mut (*vdev).job_timeout_counter, 0);
    if (*(*vdev).fw).sched_mode == VPU_SCHEDULING_MODE_OS { ivpu_pm_trigger_recovery(vdev, c"Job timeout".as_ptr()); return; }
    ivpu_jsm_state_dump(vdev); ivpu_dev_coredump(vdev); queue_work(system_percpu_wq, &mut (*vdev).context_abort_work);
}

pub unsafe fn ivpu_start_job_timeout_detection(vdev: *mut ivpu_device) {
    let timeout_ms = if ivpu_tdr_timeout_ms != 0 { ivpu_tdr_timeout_ms } else { (*vdev).timeout.tdr };
    queue_delayed_work(system_percpu_wq, &mut (*(*vdev).pm).job_timeout_work, msecs_to_jiffies(timeout_ms));
}

pub unsafe fn ivpu_stop_job_timeout_detection(vdev: *mut ivpu_device) {
    cancel_delayed_work_sync(&mut (*(*vdev).pm).job_timeout_work);
    atomic_set(&mut (*vdev).job_timeout_counter, 0);
}

// The remaining callbacks retain the kernel-facing ABI and delegate to the corresponding project APIs.
pub unsafe fn ivpu_pm_suspend_cb(dev: *mut device) -> c_int { let vdev = to_ivpu_device(dev_get_drvdata(dev)); trace_pm!("suspend"); let timeout = jiffies + msecs_to_jiffies((*vdev).timeout.tdr); while !ivpu_hw_is_idle(vdev) { cond_resched(); if time_after_eq(jiffies, timeout) { ivpu_err(vdev, "Failed to enter idle on system suspend\n"); return -EBUSY; } } ivpu_jsm_pwr_d0i3_enter(vdev); ivpu_suspend(vdev); ivpu_pm_prepare_warm_boot(vdev); trace_pm!("suspend done"); 0 }
pub unsafe fn ivpu_pm_resume_cb(dev: *mut device) -> c_int { let vdev = to_ivpu_device(dev_get_drvdata(dev)); trace_pm!("resume"); let ret = ivpu_resume(vdev); if ret != 0 { ivpu_err(vdev, "Failed to resume: %d\n", ret); } trace_pm!("resume done"); ret }
pub unsafe fn ivpu_pm_runtime_suspend_cb(dev: *mut device) -> c_int { let vdev = to_ivpu_device(dev_get_drvdata(dev)); ivpu_mmu_disable(vdev); let is_idle = ivpu_hw_is_idle(vdev) || (*(*vdev).pm).dct_active_percent != 0; let ret_d0i3 = ivpu_jsm_pwr_d0i3_enter(vdev); let ret = ivpu_suspend(vdev); if !is_idle || ret_d0i3 != 0 { atomic_inc(&mut (*(*vdev).pm).reset_counter); ivpu_dev_coredump(vdev); ivpu_pm_prepare_cold_boot(vdev); } else { ivpu_pm_prepare_warm_boot(vdev); } 0 }
pub unsafe fn ivpu_pm_runtime_resume_cb(dev: *mut device) -> c_int { let vdev = to_ivpu_device(dev_get_drvdata(dev)); ivpu_resume(vdev) }
pub unsafe fn ivpu_rpm_get(vdev: *mut ivpu_device) -> c_int { let ret = pm_runtime_resume_and_get((*vdev).drm.dev); if ret < 0 { pm_runtime_set_suspended((*vdev).drm.dev); } ret }
pub unsafe fn ivpu_rpm_put(vdev: *mut ivpu_device) { pm_runtime_put_autosuspend((*vdev).drm.dev); }

pub unsafe fn ivpu_pm_reset_prepare_cb(pdev: *mut pci_dev) {
    let vdev = pci_get_drvdata(pdev);
    ivpu_dbg(vdev, PM, "Pre-reset..\n");
    ivpu_pm_reset_begin(vdev);
    if !pm_runtime_status_suspended((*vdev).drm.dev) { ivpu_prepare_for_reset(vdev); ivpu_hw_reset(vdev); }
    ivpu_dbg(vdev, PM, "Pre-reset done.\n");
}

pub unsafe fn ivpu_pm_reset_done_cb(pdev: *mut pci_dev) {
    let vdev = pci_get_drvdata(pdev);
    ivpu_dbg(vdev, PM, "Post-reset..\n");
    ivpu_pm_reset_complete(vdev);
    ivpu_dbg(vdev, PM, "Post-reset done.\n");
}

pub unsafe fn ivpu_pm_init(vdev: *mut ivpu_device) {
    let dev = (*vdev).drm.dev;
    let pm = (*vdev).pm;
    (*pm).vdev = vdev;
    init_rwsem(&mut (*pm).reset_lock);
    atomic_set(&mut (*pm).reset_pending, 0);
    atomic_set(&mut (*pm).reset_counter, 0);
    atomic_set(&mut (*pm).engine_reset_counter, 0);
    INIT_WORK!(&mut (*pm).recovery_work, ivpu_pm_recovery_work);
    INIT_DELAYED_WORK!(&mut (*pm).job_timeout_work, ivpu_job_timeout_work);
    let delay = if ivpu_disable_recovery { -1 } else { (*vdev).timeout.autosuspend };
    pm_runtime_use_autosuspend(dev);
    pm_runtime_set_autosuspend_delay(dev, delay);
    pm_runtime_set_active(dev);
    ivpu_dbg(vdev, PM, "Autosuspend delay = %d\n", delay);
}

pub unsafe fn ivpu_pm_disable_recovery(vdev: *mut ivpu_device) {
    drm_WARN_ON!(&(*vdev).drm, delayed_work_pending(&(*(*vdev).pm).job_timeout_work));
    disable_work_sync(&mut (*(*vdev).pm).recovery_work);
}

pub unsafe fn ivpu_pm_enable(vdev: *mut ivpu_device) {
    let dev = (*vdev).drm.dev;
    pm_runtime_allow(dev);
    pm_runtime_put_autosuspend(dev);
}

pub unsafe fn ivpu_pm_disable(vdev: *mut ivpu_device) {
    pm_runtime_get_noresume((*vdev).drm.dev);
    pm_runtime_forbid((*vdev).drm.dev);
}

pub unsafe fn ivpu_pm_dct_init(vdev: *mut ivpu_device) -> c_int {
    if (*(*vdev).pm).dct_active_percent != 0 { return ivpu_pm_dct_enable(vdev, (*(*vdev).pm).dct_active_percent); }
    0
}

pub unsafe fn ivpu_pm_dct_enable(vdev: *mut ivpu_device, active_percent: u8) -> c_int {
    if active_percent == 0 || active_percent > 100 { return -EINVAL; }
    let active_us = (DCT_PERIOD_US * active_percent as u32) / 100;
    let inactive_us = DCT_PERIOD_US - active_us;
    (*(*vdev).pm).dct_active_percent = active_percent;
    ivpu_dbg(vdev, PM, "DCT requested %u%% (D0: %uus, D0i2: %uus)\n", active_percent, active_us, inactive_us);
    let ret = ivpu_jsm_dct_enable(vdev, active_us, inactive_us);
    if ret != 0 { ivpu_err_ratelimited(vdev, "Failed to enable DCT: %d\n", ret); return ret; }
    0
}

pub unsafe fn ivpu_pm_dct_disable(vdev: *mut ivpu_device) -> c_int {
    (*(*vdev).pm).dct_active_percent = 0;
    ivpu_dbg(vdev, PM, "DCT requested to be disabled\n");
    let ret = ivpu_jsm_dct_disable(vdev);
    if ret != 0 { ivpu_err_ratelimited(vdev, "Failed to disable DCT: %d\n", ret); return ret; }
    0
}

pub unsafe fn ivpu_pm_irq_dct_work_fn(work: *mut work_struct) {
    let vdev = container_of!(work, ivpu_device, irq_dct_work);
    let mut enable = false;
    if ivpu_hw_btrs_dct_get_request(vdev, &mut enable) != 0 { return; }
    let ret = if enable { ivpu_pm_dct_enable(vdev, DCT_DEFAULT_ACTIVE_PERCENT) } else { ivpu_pm_dct_disable(vdev) };
    if ret == 0 {
        // Convert percent to U1.7 format
        let val = (((*(*vdev).pm).dct_active_percent as u32 * 128) + 50) / 100;
        ivpu_hw_btrs_dct_set_status(vdev, enable, val as u8);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
