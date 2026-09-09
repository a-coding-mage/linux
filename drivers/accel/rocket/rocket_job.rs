// SPDX-License-Identifier: GPL-2.0-only
/* Copyright 2019 Linaro, Ltd, Rob Herring <robh@kernel.org> */
/* Copyright 2019 Collabora ltd. */
/* Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net> */

// Dependencies are supplied by the surrounding DRM/Linux Rust bindings.

const JOB_TIMEOUT_MS: u32 = 500;

unsafe fn to_rocket_job(sched_job: *mut drm_sched_job) -> *mut rocket_job {
    container_of!(sched_job, rocket_job, base)
}

unsafe extern "C" fn rocket_fence_get_driver_name(_fence: *mut dma_fence) -> *const c_char { b"rocket\0".as_ptr() as *const c_char }
unsafe extern "C" fn rocket_fence_get_timeline_name(_fence: *mut dma_fence) -> *const c_char { b"rockchip-npu\0".as_ptr() as *const c_char }

static ROCKET_FENCE_OPS: dma_fence_ops = dma_fence_ops {
    get_driver_name: Some(rocket_fence_get_driver_name),
    get_timeline_name: Some(rocket_fence_get_timeline_name),
};

unsafe fn rocket_fence_create(core: *mut rocket_core) -> *mut dma_fence {
    let fence = kzalloc_obj!(dma_fence);
    if fence.is_null() { return ERR_PTR!(-ENOMEM); }
    dma_fence_init(fence, &ROCKET_FENCE_OPS, &mut (*core).fence_lock,
                   (*core).fence_context, (*core).emit_seqno.wrapping_add(1));
    (*core).emit_seqno = (*core).emit_seqno.wrapping_add(1);
    fence
}

unsafe fn rocket_copy_tasks(dev: *mut drm_device, _file_priv: *mut drm_file,
                            job: *mut drm_rocket_job, rjob: *mut rocket_job) -> c_int {
    let mut ret = 0;
    if (*job).task_struct_size < core::mem::size_of::<drm_rocket_task>() { return -EINVAL; }
    (*rjob).task_count = (*job).task_count;
    if (*rjob).task_count == 0 { return 0; }
    (*rjob).tasks = kvmalloc_objs!((*rjob).tasks, (*job).task_count);
    if (*rjob).tasks.is_null() { drm_dbg!(dev, "Failed to allocate task array\n"); return -ENOMEM; }
    for i in 0..(*rjob).task_count {
        let mut task: drm_rocket_task = core::mem::zeroed();
        if copy_from_user(&mut task, u64_to_user_ptr((*job).tasks).add(i * (*job).task_struct_size), core::mem::size_of::<drm_rocket_task>()) != 0 {
            drm_dbg!(dev, "Failed to copy incoming tasks\n"); ret = -EFAULT; break;
        }
        if task.regcmd_count == 0 { drm_dbg!(dev, "regcmd_count field in drm_rocket_task should be > 0.\n"); ret = -EINVAL; break; }
        (*rjob).tasks.add(i).write(rocket_task { regcmd: task.regcmd, regcmd_count: task.regcmd_count });
    }
    if ret != 0 { kvfree((*rjob).tasks as *mut c_void); (*rjob).tasks = core::ptr::null_mut(); }
    ret
}

unsafe fn rocket_job_hw_submit(core: *mut rocket_core, job: *mut rocket_job) {
    if atomic_read(&(*core).reset.pending) != 0 { return; }
    let task = (*job).tasks.add((*job).next_task_idx); (*job).next_task_idx += 1;
    rocket_pc_writel(core, BASE_ADDRESS, 0x1);
    let extra_bit = 0x10000000u32.wrapping_mul((*core).index);
    rocket_cna_writel(core, S_POINTER, CNA_S_POINTER_POINTER_PP_EN(1) | CNA_S_POINTER_EXECUTER_PP_EN(1) | CNA_S_POINTER_POINTER_PP_MODE(1) | extra_bit);
    rocket_core_writel(core, S_POINTER, CORE_S_POINTER_POINTER_PP_EN(1) | CORE_S_POINTER_EXECUTER_PP_EN(1) | CORE_S_POINTER_POINTER_PP_MODE(1) | extra_bit);
    rocket_pc_writel(core, BASE_ADDRESS, (*task).regcmd);
    rocket_pc_writel(core, REGISTER_AMOUNTS, PC_REGISTER_AMOUNTS_PC_DATA_AMOUNT(((*task).regcmd_count + 1) / 2 - 1));
    rocket_pc_writel(core, INTERRUPT_MASK, PC_INTERRUPT_MASK_DPU_0 | PC_INTERRUPT_MASK_DPU_1);
    rocket_pc_writel(core, INTERRUPT_CLEAR, PC_INTERRUPT_CLEAR_DPU_0 | PC_INTERRUPT_CLEAR_DPU_1);
    rocket_pc_writel(core, TASK_CON, PC_TASK_CON_RESERVED(1) | PC_TASK_CON_TASK_COUNT_CLEAR(1) | PC_TASK_CON_TASK_NUMBER(1) | PC_TASK_CON_TASK_PP_EN(1));
    rocket_pc_writel(core, TASK_DMA_BASE_ADDR, PC_TASK_DMA_BASE_ADDR_DMA_BASE_ADDR(0));
    rocket_pc_writel(core, OPERATION_ENABLE, PC_OPERATION_ENABLE_OP_EN(1));
    dev_dbg!((*core).dev, "Submitted regcmd at 0x{:x} to core {}", (*task).regcmd, (*core).index);
}

unsafe fn rocket_acquire_object_fences(bos: *mut *mut drm_gem_object, bo_count: c_int, job: *mut drm_sched_job, is_write: bool) -> c_int {
    for i in 0..bo_count as usize { let ret = dma_resv_reserve_fences((*(bos.add(i))).resv, 1); if ret != 0 { return ret; } let ret = drm_sched_job_add_implicit_dependencies(job, *bos.add(i), is_write); if ret != 0 { return ret; } }
    0
}

unsafe fn rocket_attach_object_fences(bos: *mut *mut drm_gem_object, bo_count: c_int, fence: *mut dma_fence) { for i in 0..bo_count as usize { dma_resv_add_fence((*(bos.add(i))).resv, fence, DMA_RESV_USAGE_WRITE); } }

unsafe fn rocket_job_cleanup(ref_: *mut kref) {
    let job = container_of!(ref_, rocket_job, refcount);
    rocket_iommu_domain_put((*job).domain); dma_fence_put((*job).done_fence); dma_fence_put((*job).inference_done_fence);
    if !(*job).in_bos.is_null() { for i in 0..(*job).in_bo_count as usize { drm_gem_object_put(*(*job).in_bos.add(i)); } kvfree((*job).in_bos as *mut c_void); }
    if !(*job).out_bos.is_null() { for i in 0..(*job).out_bo_count as usize { drm_gem_object_put(*(*job).out_bos.add(i)); } kvfree((*job).out_bos as *mut c_void); }
    kvfree((*job).tasks as *mut c_void); kfree(job as *mut c_void);
}

unsafe fn rocket_job_put(job: *mut rocket_job) { kref_put(&mut (*job).refcount, rocket_job_cleanup); }

unsafe extern "C" fn rocket_job_free(sched_job: *mut drm_sched_job) { let job = to_rocket_job(sched_job); drm_sched_job_cleanup(sched_job); rocket_job_put(job); }

unsafe fn sched_to_core(rdev: *mut rocket_device, sched: *mut drm_gpu_scheduler) -> *mut rocket_core {
    for i in 0..(*rdev).num_cores as usize { if &mut (*rdev).cores.add(i).as_mut().unwrap().sched == sched { return (*rdev).cores.add(i); } }
    core::ptr::null_mut()
}

unsafe fn rocket_job_run(sched_job: *mut drm_sched_job) -> *mut dma_fence {
    let job = to_rocket_job(sched_job); let core = sched_to_core((*job).rdev, (*sched_job).sched);
    if (*job).base.s_fence.finished.error != 0 || (*job).next_task_idx == (*job).task_count { return core::ptr::null_mut(); }
    let fence = rocket_fence_create(core); if IS_ERR!(fence) { return fence; }
    dma_fence_put((*job).done_fence); (*job).done_fence = dma_fence_get(fence);
    let ret = pm_runtime_resume_and_get((*core).dev); if ret < 0 { dma_fence_put((*job).done_fence); (*job).done_fence = core::ptr::null_mut(); dma_fence_put(fence); return ERR_PTR!(ret); }
    let ret = iommu_attach_group((*job).domain.domain, (*core).iommu_group); if ret < 0 { pm_runtime_put((*core).dev); dma_fence_put((*job).done_fence); (*job).done_fence = core::ptr::null_mut(); dma_fence_put(fence); return ERR_PTR!(ret); }
    (*core).in_flight_job = job; rocket_job_hw_submit(core, job); fence
}

unsafe fn rocket_job_handle_irq(core: *mut rocket_core) {
    pm_runtime_mark_last_busy((*core).dev); rocket_pc_writel(core, OPERATION_ENABLE, 0); rocket_pc_writel(core, INTERRUPT_CLEAR, 0x1ffff);
    let job = (*core).in_flight_job; if !job.is_null() {
        if (*job).next_task_idx < (*job).task_count { rocket_job_hw_submit(core, job); return; }
        iommu_detach_group(core::ptr::null_mut(), iommu_group_get((*core).dev)); dma_fence_signal((*job).done_fence); pm_runtime_put_autosuspend((*core).dev); (*core).in_flight_job = core::ptr::null_mut();
    }
}

unsafe fn rocket_reset(core: *mut rocket_core, bad: *mut drm_sched_job) {
    if atomic_read(&(*core).reset.pending) == 0 { return; }
    drm_sched_stop(&mut (*core).sched, bad); if !(*core).in_flight_job.is_null() { pm_runtime_put_noidle((*core).dev); }
    iommu_detach_group(core::ptr::null_mut(), (*core).iommu_group); (*core).in_flight_job = core::ptr::null_mut(); rocket_core_reset(core); atomic_set(&mut (*core).reset.pending, 0); drm_sched_start(&mut (*core).sched, 0);
}

unsafe fn rocket_job_timedout(sched_job: *mut drm_sched_job) -> drm_gpu_sched_stat { let job = to_rocket_job(sched_job); let core = sched_to_core((*job).rdev, (*sched_job).sched); dev_err!((*core).dev, "NPU job timed out"); atomic_set(&mut (*core).reset.pending, 1); rocket_reset(core, sched_job); DRM_GPU_SCHED_STAT_RESET }
unsafe fn rocket_reset_work(work: *mut work_struct) { let core = container_of!(work, rocket_core, reset.work); rocket_reset(core, core::ptr::null_mut()); }
unsafe extern "C" fn rocket_job_irq_handler_thread(_irq: c_int, data: *mut c_void) -> irqreturn_t { rocket_job_handle_irq(data as *mut rocket_core); IRQ_HANDLED }
unsafe extern "C" fn rocket_job_irq_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t { let core = data as *mut rocket_core; let raw = rocket_pc_readl(core, INTERRUPT_RAW_STATUS); WARN_ON!(raw & PC_INTERRUPT_RAW_STATUS_DMA_READ_ERROR != 0); WARN_ON!(raw & PC_INTERRUPT_RAW_STATUS_DMA_WRITE_ERROR != 0); if raw & (PC_INTERRUPT_RAW_STATUS_DPU_0 | PC_INTERRUPT_RAW_STATUS_DPU_1) == 0 { return IRQ_NONE; } rocket_pc_writel(core, INTERRUPT_MASK, 0); IRQ_WAKE_THREAD }

unsafe fn rocket_job_init(core: *mut rocket_core) -> c_int { INIT_WORK!(&mut (*core).reset.work, rocket_reset_work); spin_lock_init(&mut (*core).fence_lock); mutex_init(&mut (*core).job_lock); (*core).irq = platform_get_irq(to_platform_device((*core).dev), 0); if (*core).irq < 0 { return (*core).irq; } let ret = devm_request_threaded_irq((*core).dev, (*core).irq, rocket_job_irq_handler, rocket_job_irq_handler_thread, IRQF_SHARED, dev_name((*core).dev), core as *mut c_void); if ret != 0 { return ret; } (*core).reset.wq = alloc_ordered_workqueue!("rocket-reset-%d", (*core).index); if (*core).reset.wq.is_null() { return -ENOMEM; } (*core).fence_context = dma_fence_context_alloc(1); drm_sched_init_core(core); 0 }
unsafe fn rocket_job_fini(core: *mut rocket_core) { drm_sched_fini(&mut (*core).sched); cancel_work_sync(&mut (*core).reset.work); destroy_workqueue((*core).reset.wq); }
unsafe fn rocket_job_open(priv_: *mut rocket_file_priv) -> c_int { let rdev = (*priv_).rdev; (*priv_).sched_entity = drm_sched_entity_init_array((*rdev).cores, (*rdev).num_cores); 0 }
unsafe fn rocket_job_close(priv_: *mut rocket_file_priv) { drm_sched_entity_destroy(&mut (*priv_).sched_entity); }
unsafe fn rocket_job_is_idle(core: *mut rocket_core) -> c_int { (atomic_read(&(*core).sched.credit_count) == 0) as c_int }
unsafe fn rocket_ioctl_submit(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int { let args = data as *mut drm_rocket_submit; if (*args).job_count == 0 { return 0; } if (*args).job_struct_size < core::mem::size_of::<drm_rocket_job>() || (*args).reserved != 0 { return -EINVAL; } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
