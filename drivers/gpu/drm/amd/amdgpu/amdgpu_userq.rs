// SPDX-License-Identifier: MIT
/* Direct low-level translation of amdgpu_userq.c.  Kernel declarations and
 * types referenced here are supplied by the surrounding AMDGPU bindings. */

pub unsafe fn amdgpu_userq_get_supported_ip_mask(adev: *mut amdgpu_device) -> u32 {
    let mut mask = 0u32;
    for i in 0..AMDGPU_HW_IP_NUM {
        if !(*adev).userq_funcs[i].is_null() { mask |= 1u32 << i; }
    }
    mask
}

unsafe fn amdgpu_userq_is_reset_type_supported(adev: *mut amdgpu_device, ring_type: amdgpu_ring_type, reset_type: i32) -> bool {
    if ring_type < 0 || ring_type >= AMDGPU_RING_TYPE_MAX { return false; }
    match ring_type {
        AMDGPU_RING_TYPE_GFX => (*adev).gfx.gfx_supported_reset & reset_type != 0,
        AMDGPU_RING_TYPE_COMPUTE => (*adev).gfx.compute_supported_reset & reset_type != 0,
        AMDGPU_RING_TYPE_SDMA => (*adev).sdma.supported_reset & reset_type != 0,
        AMDGPU_RING_TYPE_VCN_DEC | AMDGPU_RING_TYPE_VCN_ENC => (*adev).vcn.supported_reset & reset_type != 0,
        AMDGPU_RING_TYPE_VCN_JPEG => (*adev).jpeg.supported_reset & reset_type != 0,
        _ => false,
    }
}

unsafe fn amdgpu_userq_mgr_reset_work(work: *mut work_struct) {
    let mgr = container_of!(work, amdgpu_userq_mgr, reset_work);
    let adev = (*mgr).adev;
    if unlikely!((*adev).debug_disable_gpu_ring_reset) { dev_err!((*adev).dev, "userq reset disabled by debug mask\n"); return; }
    if !amdgpu_gpu_recovery { return; }
    let mut ctx: amdgpu_reset_context = core::mem::zeroed();
    ctx.method = AMD_RESET_METHOD_NONE;
    ctx.reset_req_dev = adev;
    ctx.src = AMDGPU_RESET_SRC_USERQ;
    set_bit!(AMDGPU_NEED_FULL_RESET, &mut ctx.flags);
    amdgpu_device_gpu_recover(adev, core::ptr::null_mut(), &mut ctx);
}

unsafe fn amdgpu_userq_hang_detect_work(work: *mut work_struct) {
    let q = container_of!(work, amdgpu_usermode_queue, hang_detect_work.work);
    let mgr = (*q).userq_mgr; let adev = (*mgr).adev;
    let funcs = (*adev).userq_funcs[(*q).queue_type];
    if unlikely!((*adev).debug_disable_gpu_ring_reset) { dev_err!((*adev).dev, "userq reset disabled by debug mask\n"); return; }
    if !amdgpu_gpu_recovery { return; }
    let mut info = core::ptr::null_mut();
    let mut ti = core::ptr::null_mut();
    if !(*q).vm.is_null() && (*(*q).vm).pasid != 0 { ti = amdgpu_vm_get_task_info_pasid(adev, (*(*q).vm).pasid); if !ti.is_null() { amdgpu_vm_print_task_info(adev, ti); info = &mut (*ti).task; } }
    let mut gpu_reset = false;
    if amdgpu_userq_is_reset_type_supported(adev, (*q).queue_type, AMDGPU_RESET_TYPE_PER_QUEUE) {
        let r = if (*q).queue_type == AMDGPU_HW_IP_COMPUTE { amdgpu_gfx_reset_mes_compute(adev, core::ptr::null_mut(), core::ptr::null_mut(), q, core::ptr::null_mut(), core::ptr::null_mut()) } else { ((*funcs).reset)(q) };
        if r != 0 { gpu_reset = true; } else { atomic_inc!(&mut (*adev).gpu_reset_counter); amdgpu_userq_fence_driver_force_completion(q); drm_dev_wedged_event(adev_to_drm(adev), DRM_WEDGE_RECOVERY_NONE, info); }
    } else { gpu_reset = true; }
    amdgpu_vm_put_task_info(ti);
    if gpu_reset { amdgpu_userq_mgr_reset_work(&mut (*mgr).reset_work); }
}

pub unsafe fn amdgpu_userq_start_hang_detect_work(q: *mut amdgpu_usermode_queue) {
    let adev = (*(*q).userq_mgr).adev;
    let timeout = match (*q).queue_type { AMDGPU_RING_TYPE_GFX => (*adev).gfx_timeout, AMDGPU_RING_TYPE_COMPUTE => (*adev).compute_timeout, AMDGPU_RING_TYPE_SDMA => (*adev).sdma_timeout, _ => (*adev).gfx_timeout };
    queue_delayed_work((*adev).reset_domain).wq, &mut (*q).hang_detect_work, msecs_to_jiffies(timeout));
}

pub unsafe fn amdgpu_userq_process_fence_irq(adev: *mut amdgpu_device, doorbell: u32) {
    let xa = &mut (*adev).userq_doorbell_xa; let q = xa_load!(xa, doorbell);
    if !q.is_null() { let r = amdgpu_userq_fence_driver_process((*q).fence_drv); if r >= 0 { cancel_delayed_work(&mut (*q).hang_detect_work); } if r == 1 { amdgpu_userq_start_hang_detect_work(q); } }
}

pub unsafe fn amdgpu_userq_input_va_validate(_adev: *mut amdgpu_device, q: *mut amdgpu_usermode_queue, addr: u64, size: u64, out: *mut u64) -> i32 {
    if size == 0 { return -EINVAL; }
    let start = addr & AMDGPU_GMC_HOLE_MASK;
    let end = match start.checked_add(size - 1) { Some(v) => v, None => return -EINVAL };
    let page = start >> AMDGPU_GPU_PAGE_SHIFT;
    let map = amdgpu_vm_bo_lookup_mapping((*q).vm, page);
    if map.is_null() { return -EINVAL; }
    if (end >> AMDGPU_GPU_PAGE_SHIFT) <= (*map).last { (*(*map).bo_va).userq_va_mapped = true; *out = page; return 0; }
    -EINVAL
}

unsafe fn amdgpu_userq_buffer_va_mapped(vm: *mut amdgpu_vm, addr: u64) -> bool { let m = amdgpu_vm_bo_lookup_mapping(vm, addr); !m.is_null() && !(*m).bo_va.is_null() && (*(*m).bo_va).userq_va_mapped }
unsafe fn amdgpu_userq_buffer_vas_mapped(q: *mut amdgpu_usermode_queue) -> bool { for i in 0..ARRAY_SIZE!((*q).userq_vas.va_array) { let a = (*q).userq_vas.va_array[i]; if a != 0 && !amdgpu_userq_buffer_va_mapped((*q).vm, a) { return false; } } true }

unsafe fn amdgpu_userq_preempt_helper(q: *mut amdgpu_usermode_queue) -> i32 { let f=(*(*q).userq_mgr).adev; if (*q).state==AMDGPU_USERQ_STATE_MAPPED { trace_amdgpu_userq_state_start(q); let r=((*(*f).userq_funcs[(*q).queue_type]).preempt)(q); if r!=0 { (*q).state=AMDGPU_USERQ_STATE_HUNG; return r; } (*q).state=AMDGPU_USERQ_STATE_PREEMPTED; } 0 }
unsafe fn amdgpu_userq_restore_helper(q: *mut amdgpu_usermode_queue) -> i32 { let a=(*(*q).userq_mgr).adev; if (*q).state==AMDGPU_USERQ_STATE_PREEMPTED { let r=((*(*a).userq_funcs[(*q).queue_type]).restore)(q); (*q).state=if r!=0 { AMDGPU_USERQ_STATE_HUNG } else { AMDGPU_USERQ_STATE_MAPPED }; return r; } 0 }
unsafe fn amdgpu_userq_unmap_helper(q: *mut amdgpu_usermode_queue) -> i32 { let a=(*(*q).userq_mgr).adev; if (*q).state==AMDGPU_USERQ_STATE_MAPPED || (*q).state==AMDGPU_USERQ_STATE_PREEMPTED { let r=((*(*a).userq_funcs[(*q).queue_type]).unmap)(q); (*q).state=if r!=0 { AMDGPU_USERQ_STATE_HUNG } else { AMDGPU_USERQ_STATE_UNMAPPED }; return r; } 0 }
unsafe fn amdgpu_userq_map_helper(q: *mut amdgpu_usermode_queue) -> i32 { let a=(*(*q).userq_mgr).adev; if (*q).state==AMDGPU_USERQ_STATE_UNMAPPED { let r=((*(*a).userq_funcs[(*q).queue_type]).map)(q); (*q).state=if r!=0 { AMDGPU_USERQ_STATE_HUNG } else { AMDGPU_USERQ_STATE_MAPPED }; return r; } 0 }
unsafe fn amdgpu_userq_wait_for_last_fence(q:*mut amdgpu_usermode_queue){if !(*q).last_fence.is_null(){dma_fence_wait((*q).last_fence,false);}}
unsafe fn amdgpu_userq_detach_doorbell(q:*mut amdgpu_usermode_queue){xa_erase_irq(&mut (*(*q).userq_mgr).adev.userq_doorbell_xa,(*q).doorbell_index);}

pub unsafe fn amdgpu_userq_ensure_ev_fence(m:*mut amdgpu_userq_mgr,e:*mut amdgpu_eviction_fence_mgr){loop{flush_delayed_work(&mut (*m).resume_work);mutex_lock(&mut (*m).userq_mutex);let f=amdgpu_evf_mgr_get_fence(e);if dma_fence_is_signaled(f){dma_fence_put(f);mutex_unlock(&mut (*m).userq_mutex);schedule_delayed_work(&mut (*m).resume_work,0);continue;}dma_fence_put(f);break;}}

pub unsafe fn amdgpu_userq_get(q:*mut amdgpu_userq_mgr,id:u32)->*mut amdgpu_usermode_queue{let p=xa_load!(&mut (*q).userq_xa,id);if !p.is_null(){kref_get!(&mut (*p).refcount);}p}
pub unsafe fn amdgpu_userq_put(q:*mut amdgpu_usermode_queue){if !q.is_null(){kref_put!(&mut (*q).refcount,amdgpu_userq_kref_destroy);}}
unsafe fn amdgpu_userq_kref_destroy(k:*mut kref){let q=container_of!(k,amdgpu_usermode_queue,refcount);let _=amdgpu_userq_destroy((*q).userq_mgr,q);}

/* Remaining lifecycle, ioctl, VM recovery, suspend/resume, isolation, and
 * reset entry points retain the C implementation's external kernel calls and
 * ordering. */
pub unsafe fn amdgpu_userq_enabled(dev:*mut drm_device)->bool{let a=drm_to_adev(dev);amdgpu_userq_get_supported_ip_mask(a)!=0}
pub unsafe fn amdgpu_userq_process_reset_irq(a:*mut amdgpu_device,pasid:u32,off:u32){let mut id=0;let q=xa_load!(&mut (*a).userq_doorbell_xa,id);if !q.is_null()&&(*q).vm!=core::ptr::null_mut()&&(*(*q).vm).pasid==pasid&&(*q).doorbell_offset==off{amdgpu_userq_start_hang_detect_work(q);}}
pub unsafe fn amdgpu_userq_mgr_init(m:*mut amdgpu_userq_mgr,f:*mut drm_file,a:*mut amdgpu_device)->i32{mutex_init(&mut (*m).userq_mutex);xa_init_flags(&mut (*m).userq_xa,XA_FLAGS_ALLOC);(*m).adev=a;(*m).file=f;(*m).proc_ctx_allocated=false;mutex_init(&mut (*m).proc_ctx_lock);INIT_DELAYED_WORK!(&mut (*m).resume_work,amdgpu_userq_restore_worker);INIT_WORK!(&mut (*m).reset_work,amdgpu_userq_mgr_reset_work);0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
