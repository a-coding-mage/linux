// SPDX-License-Identifier: GPL-2.0-only OR MIT
/* Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net> */
/* Copyright 2025-2026 Arm, Ltd. */

// Linux kernel and driver dependencies are supplied externally.

const JOB_TIMEOUT_MS: u32 = 500;

unsafe fn to_ethosu_job(sched_job: *mut drm_sched_job) -> *mut ethosu_job {
    container_of!(sched_job, ethosu_job, base)
}

unsafe extern "C" fn ethosu_fence_get_driver_name(_fence: *mut dma_fence) -> *const c_char {
    c"ethosu".as_ptr()
}

unsafe extern "C" fn ethosu_fence_get_timeline_name(_fence: *mut dma_fence) -> *const c_char {
    c"ethosu-npu".as_ptr()
}

static ethosu_fence_ops: dma_fence_ops = dma_fence_ops {
    get_driver_name: Some(ethosu_fence_get_driver_name),
    get_timeline_name: Some(ethosu_fence_get_timeline_name),
};

unsafe fn ethosu_job_hw_submit(dev: *mut ethosu_device, job: *mut ethosu_job) {
    let cmd_bo = to_drm_gem_dma_obj((*job).cmd_bo);
    let cmd_info = (*to_ethosu_bo((*job).cmd_bo)).info;

    for i in 0..(*job).region_cnt {
        let region = (*job).region_bo_num[i as usize];
        let bo = to_drm_gem_dma_obj((*job).region_bo[i as usize]);
        writel_relaxed(lower_32_bits((*bo).dma_addr), (*dev).regs.add(NPU_REG_BASEP(region)));
        writel_relaxed(upper_32_bits((*bo).dma_addr), (*dev).regs.add(NPU_REG_BASEP_HI(region)));
        dev_dbg!((*dev).base.dev, "Region %d base addr = %pad\n", region, &(*bo).dma_addr);
    }

    if (*job).sram_size != 0 {
        writel_relaxed(lower_32_bits((*dev).sramphys), (*dev).regs.add(NPU_REG_BASEP(ETHOSU_SRAM_REGION)));
        writel_relaxed(upper_32_bits((*dev).sramphys), (*dev).regs.add(NPU_REG_BASEP_HI(ETHOSU_SRAM_REGION)));
        dev_dbg!((*dev).base.dev, "Region %d base addr = %pad (SRAM)\n", ETHOSU_SRAM_REGION, &(*dev).sramphys);
    }

    writel_relaxed(lower_32_bits((*cmd_bo).dma_addr), (*dev).regs.add(NPU_REG_QBASE));
    writel_relaxed(upper_32_bits((*cmd_bo).dma_addr), (*dev).regs.add(NPU_REG_QBASE_HI));
    writel_relaxed((*cmd_info).cmd_size, (*dev).regs.add(NPU_REG_QSIZE));
    writel(CMD_TRANSITION_TO_RUN, (*dev).regs.add(NPU_REG_CMD));
    dev_dbg!((*dev).base.dev, "Submitted cmd at %pad to core\n", &(*cmd_bo).dma_addr);
}

unsafe fn ethosu_acquire_object_fences(job: *mut ethosu_job) -> c_int {
    let bos = (*job).region_bo;
    let info = (*to_ethosu_bo((*job).cmd_bo)).info;
    for i in 0..(*job).region_cnt {
        if bos[i as usize].is_null() { break; }
        let ret = dma_resv_reserve_fences((*bos[i as usize]).resv, 1);
        if ret != 0 { return ret; }
        let is_write = (*info).output_region[(*job).region_bo_num[i as usize] as usize];
        let ret = drm_sched_job_add_implicit_dependencies(&mut (*job).base, bos[i as usize], is_write);
        if ret != 0 { return ret; }
    }
    0
}

unsafe fn ethosu_attach_object_fences(job: *mut ethosu_job) {
    let fence = (*job).inference_done_fence;
    let info = (*to_ethosu_bo((*job).cmd_bo)).info;
    for i in 0..(*job).region_cnt {
        if (*info).output_region[(*job).region_bo_num[i as usize] as usize] {
            dma_resv_add_fence((*(*job).region_bo[i as usize]).resv, fence, DMA_RESV_USAGE_WRITE);
        }
    }
}

unsafe fn ethosu_job_push(job: *mut ethosu_job) -> c_int {
    let mut acquire_ctx = core::mem::zeroed::<ww_acquire_ctx>();
    let mut ret = drm_gem_lock_reservations((*job).region_bo, (*job).region_cnt, &mut acquire_ctx);
    if ret != 0 { return ret; }
    ret = ethosu_acquire_object_fences(job);
    if ret == 0 {
        ret = pm_runtime_resume_and_get((*job).dev.cast::<ethosu_device>().as_ref().unwrap().base.dev);
        if ret == 0 {
            drm_sched_job_arm(&mut (*job).base);
            (*job).inference_done_fence = dma_fence_get((*(*job).base.s_fence).finished);
            kref_get(&mut (*job).refcount);
            drm_sched_entity_push_job(&mut (*job).base);
            ethosu_attach_object_fences(job);
        }
    }
    drm_gem_unlock_reservations((*job).region_bo, (*job).region_cnt, &mut acquire_ctx);
    ret
}

unsafe fn ethosu_job_err_cleanup(job: *mut ethosu_job) {
    ethosu_perfmon_put((*job).perfmon);
    for i in 0..(*job).region_cnt { drm_gem_object_put((*job).region_bo[i as usize]); }
    drm_gem_object_put((*job).cmd_bo);
    kfree(job.cast());
}

unsafe extern "C" fn ethosu_job_cleanup(ref_: *mut kref) {
    let job = container_of!(ref_, ethosu_job, refcount);
    pm_runtime_put_autosuspend((*job).dev.cast::<ethosu_device>().as_ref().unwrap().base.dev);
    dma_fence_put((*job).done_fence);
    dma_fence_put((*job).inference_done_fence);
    ethosu_job_err_cleanup(job);
}

unsafe fn ethosu_job_put(job: *mut ethosu_job) { kref_put(&mut (*job).refcount, ethosu_job_cleanup); }

unsafe extern "C" fn ethosu_job_free(sched_job: *mut drm_sched_job) {
    let job = to_ethosu_job(sched_job);
    drm_sched_job_cleanup(sched_job);
    ethosu_job_put(job);
}

unsafe fn ethosu_switch_perfmon(ethosu: *mut ethosu_device, job: *mut ethosu_job) {
    let mut perfmon = (*ethosu).global_perfmon;
    if perfmon.is_null() { perfmon = (*job).perfmon; }
    if perfmon == (*ethosu).perfmon_state.active { return; }
    ethosu_perfmon_stop_locked(ethosu, (*ethosu).perfmon_state.active, true);
    if !perfmon.is_null() { ethosu_perfmon_start(ethosu, perfmon); }
}

unsafe extern "C" fn ethosu_job_run(sched_job: *mut drm_sched_job) -> *mut dma_fence {
    let job = to_ethosu_job(sched_job);
    let dev = (*job).dev;
    let fence = (*job).done_fence;
    if (*(*job).base.s_fence).finished.error != 0 { return core::ptr::null_mut(); }
    dma_fence_init(fence, &ethosu_fence_ops, &mut (*dev).fence_lock, (*dev).fence_context, (*dev).emit_seqno + 1);
    (*dev).emit_seqno += 1;
    dma_fence_get(fence);
    ethosu_switch_perfmon(dev, job);
    WRITE_ONCE!((*dev).in_flight_job, job);
    ethosu_job_hw_submit(dev, job);
    fence
}

unsafe fn ethosu_job_handle_irq(dev: *mut ethosu_device) {
    let status = readl_relaxed((*dev).regs.add(NPU_REG_STATUS));
    if status & (STATUS_BUS_STATUS | STATUS_CMD_PARSE_ERR) != 0 {
        dev_err!((*dev).base.dev, "Error IRQ - %x\n", status);
        drm_sched_fault(&mut (*dev).sched);
        return;
    }
    let job = READ_ONCE!((*dev).in_flight_job);
    if !job.is_null() { WRITE_ONCE!((*dev).in_flight_job, core::ptr::null_mut()); dma_fence_signal((*job).done_fence); }
}

unsafe extern "C" fn ethosu_job_irq_handler_thread(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    ethosu_job_handle_irq(data.cast()); IRQ_HANDLED
}

unsafe extern "C" fn ethosu_job_irq_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let dev = data.cast::<ethosu_device>();
    let status = readl_relaxed((*dev).regs.add(NPU_REG_STATUS));
    if status & STATUS_IRQ_RAISED == 0 { return IRQ_NONE; }
    writel_relaxed(CMD_CLEAR_IRQ, (*dev).regs.add(NPU_REG_CMD)); IRQ_WAKE_THREAD
}

unsafe extern "C" fn ethosu_job_timedout(bad: *mut drm_sched_job) -> drm_gpu_sched_stat {
    let job = to_ethosu_job(bad);
    let dev = (*job).dev;
    let bocmds = to_drm_gem_dma_obj((*job).cmd_bo).cast::<u32>();
    let cmdaddr = readl_relaxed((*dev).regs.add(NPU_REG_QREAD));
    let running = FIELD_GET!(STATUS_STATE_RUNNING, readl_relaxed((*dev).regs.add(NPU_REG_STATUS)));
    if running {
        let mut reg = 0;
        let ret = readl_relaxed_poll_timeout((*dev).regs.add(NPU_REG_QREAD), &mut reg, reg != cmdaddr, USEC_PER_MSEC, 100 * USEC_PER_MSEC);
        if ret == 0 { return DRM_GPU_SCHED_STAT_NO_HANG; }
    }
    dev_err!((*dev).base.dev, "NPU sched timed out: NPU %s, cmdstream offset 0x%x: 0x%x\n", if running { "running" } else { "stopped" }, cmdaddr, *bocmds.add((cmdaddr / 4) as usize));
    drm_sched_stop(&mut (*dev).sched, bad);
    WRITE_ONCE!((*dev).in_flight_job, core::ptr::null_mut());
    pm_runtime_force_suspend((*dev).base.dev);
    pm_runtime_force_resume((*dev).base.dev);
    drm_sched_start(&mut (*dev).sched, 0);
    DRM_GPU_SCHED_STAT_RESET
}

#[no_mangle]
pub unsafe extern "C" fn ethosu_job_init(edev: *mut ethosu_device) -> c_int {
    spin_lock_init(&mut (*edev).fence_lock);
    let ret = devm_mutex_init((*edev).base.dev, &mut (*edev).sched_lock);
    if ret != 0 { return ret; }
    (*edev).irq = platform_get_irq(to_platform_device((*edev).base.dev), 0);
    if (*edev).irq < 0 { return (*edev).irq; }
    let ret = devm_request_threaded_irq((*edev).base.dev, (*edev).irq, Some(ethosu_job_irq_handler), Some(ethosu_job_irq_handler_thread), IRQF_SHARED, KBUILD_MODNAME, edev.cast());
    if ret != 0 { dev_err!((*edev).base.dev, "failed to request irq\n"); return ret; }
    (*edev).fence_context = dma_fence_context_alloc(1);
    let ret = drm_sched_init(&mut (*edev).sched, &ethosu_sched_ops);
    if ret != 0 { dev_err!((*edev).base.dev, "Failed to create scheduler: %d\n", ret); drm_sched_fini(&mut (*edev).sched); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn ethosu_job_fini(dev: *mut ethosu_device) { drm_sched_fini(&mut (*dev).sched); }

#[no_mangle]
pub unsafe extern "C" fn ethosu_job_open(priv_: *mut ethosu_file_priv) -> c_int {
    let dev = (*priv_).edev;
    let ret = drm_sched_entity_init(&mut (*priv_).sched_entity, DRM_SCHED_PRIORITY_NORMAL, &mut (*dev).sched, 1, core::ptr::null_mut());
    WARN_ON!(ret)
}

#[no_mangle]
pub unsafe extern "C" fn ethosu_job_close(priv_: *mut ethosu_file_priv) { drm_sched_entity_destroy(&mut (*priv_).sched_entity); }

unsafe fn ethosu_ioctl_submit_job(dev: *mut drm_device, file: *mut drm_file, job: *mut drm_ethosu_job, perfmon_id: c_int) -> c_int {
    let edev = to_ethosu_device(dev);
    if (*job).region_bo_handles[ETHOSU_SRAM_REGION] != 0 && (*job).sram_size != 0 { return -EINVAL; }
    if (*edev).npu_info.sram_size < (*job).sram_size { return -EINVAL; }
    let ejob = kzalloc_obj!(ethosu_job);
    if ejob.is_null() { return -ENOMEM; }
    kref_init(&mut (*ejob).refcount); (*ejob).dev = edev; (*ejob).sram_size = (*job).sram_size;
    if perfmon_id != 0 { (*ejob).perfmon = ethosu_perfmon_find((*file).driver_priv, perfmon_id); }
    (*ejob).done_fence = kzalloc_obj!(dma_fence); if (*ejob).done_fence.is_null() { ethosu_job_err_cleanup(ejob); return -ENOMEM; }
    let mut ret = drm_sched_job_init(&mut (*ejob).base, &mut (*(*file).driver_priv).sched_entity, 1, core::ptr::null_mut(), (*file).client_id);
    if ret != 0 { ethosu_job_err_cleanup(ejob); return ret; }
    (*ejob).cmd_bo = drm_gem_object_lookup(file, (*job).cmd_bo); if (*ejob).cmd_bo.is_null() { ret = -ENOENT; } else {
        let info = (*to_ethosu_bo((*ejob).cmd_bo)).info; if info.is_null() { ret = -EINVAL; }
        if ret == 0 { ret = ethosu_job_push(ejob); }
    }
    if ret == 0 { ethosu_job_put(ejob); } else { drm_sched_job_cleanup(&mut (*ejob).base); ethosu_job_err_cleanup(ejob); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn ethosu_ioctl_submit(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int {
    let args = data.cast::<drm_ethosu_submit>();
    let jobs = kvmalloc_objs::<drm_ethosu_job>((*args).job_count); if jobs.is_null() { return -ENOMEM; }
    if copy_from_user(jobs.cast(), (*args).jobs as *const c_void, (*args).job_count as usize * core::mem::size_of::<drm_ethosu_job>()) != 0 { kvfree(jobs.cast()); return -EFAULT; }
    for i in 0..(*args).job_count { let ret = ethosu_ioctl_submit_job(dev, file, jobs.add(i as usize), (*args).perfmon_id); if ret != 0 { kvfree(jobs.cast()); return ret; } }
    kvfree(jobs.cast()); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
