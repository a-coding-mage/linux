// SPDX-License-Identifier: MIT
/*
 * Copyright 2024 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 */

// Dependencies supplied by the surrounding kernel/amdgpu translation.
const AMDGPU_USERQ_PROC_CTX_SZ: usize = PAGE_SIZE;
const AMDGPU_USERQ_GANG_CTX_SZ: usize = PAGE_SIZE;

unsafe fn mes_userq_create_wptr_mapping(adev: *mut amdgpu_device, uq_mgr: *mut amdgpu_userq_mgr, queue: *mut amdgpu_usermode_queue, mut wptr: u64) -> i32 {
    let mut wptr_mapping: *mut amdgpu_bo_va_mapping;
    let wptr_obj = &mut (*queue).wptr_obj;
    let mut obj: *mut amdgpu_bo;
    let vm = (*queue).vm;
    let mut exec: drm_exec = core::mem::zeroed();
    let mut ret: i32;
    wptr &= AMDGPU_GMC_HOLE_MASK;
    drm_exec_init(&mut exec, DRM_EXEC_IGNORE_DUPLICATES, 2);
    drm_exec_until_all_locked!(&mut exec, {
        ret = amdgpu_vm_lock_pd(vm, &mut exec, 1);
        drm_exec_retry_on_contention!(&mut exec);
        if unlikely(ret != 0) { goto!(fail_lock); }
        wptr_mapping = amdgpu_vm_bo_lookup_mapping(vm, wptr >> PAGE_SHIFT);
        if wptr_mapping.is_null() { ret = -EINVAL; goto!(fail_lock); }
        obj = (*(*wptr_mapping).bo_va).base.bo;
        ret = drm_exec_lock_obj(&mut exec, &mut (*obj).tbo.base);
        drm_exec_retry_on_contention!(&mut exec);
        if unlikely(ret != 0) { goto!(fail_lock); }
    });
    (*wptr_obj).obj = amdgpu_bo_ref((*(*wptr_mapping).bo_va).base.bo);
    if (*(*wptr_obj).obj).tbo.base.size > PAGE_SIZE { ret = -EINVAL; goto!(fail_map); }
    // Keep WPTR BO under eviction-fence control instead of pinning.
    ret = amdgpu_evf_mgr_attach_fence(&mut uq_mgr_to_fpriv(uq_mgr).evf_mgr, (*wptr_obj).obj);
    if ret != 0 { DRM_ERROR!("Failed to attach eviction fence to wptr bo. ret %d\n", ret); goto!(fail_map); }
    ret = amdgpu_ttm_alloc_gart(&mut (*wptr_obj).obj.tbo);
    if ret != 0 { DRM_ERROR!("Failed to bind wptr bo to GART. ret %d\n", ret); goto!(fail_map); }
    (*queue).wptr_obj.gpu_addr = amdgpu_bo_gpu_offset((*wptr_obj).obj);
    drm_exec_fini(&mut exec);
    return 0;
fail_map:
    amdgpu_bo_unref(&mut (*wptr_obj).obj);
fail_lock:
    drm_exec_fini(&mut exec);
    ret
}

unsafe fn convert_to_mes_priority(priority: i32) -> i32 {
    match priority {
        AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_LOW => AMDGPU_MES_PRIORITY_LEVEL_LOW,
        AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_NORMAL_HIGH => AMDGPU_MES_PRIORITY_LEVEL_MEDIUM,
        AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_HIGH => AMDGPU_MES_PRIORITY_LEVEL_HIGH,
        _ => AMDGPU_MES_PRIORITY_LEVEL_NORMAL,
    }
}

unsafe fn mes_userq_map(queue: *mut amdgpu_usermode_queue) -> i32 {
    let uq_mgr = (*queue).userq_mgr; let adev = (*uq_mgr).adev;
    let ctx = &mut (*queue).fw_obj; let props = (*queue).userq_prop;
    let mut input: mes_add_queue_input = core::mem::zeroed(); let mes = &mut (*adev).mes;
    input.process_va_start = 0; input.process_va_end = (*adev).vm_manager.max_pfn - 1;
    input.process_quantum = 100000; input.gang_quantum = 10000; input.paging = false;
    input.process_context_addr = (*uq_mgr).proc_ctx_obj.gpu_addr; input.gang_context_addr = ctx.gpu_addr;
    input.inprocess_gang_priority = AMDGPU_MES_PRIORITY_LEVEL_NORMAL;
    input.gang_global_priority_level = convert_to_mes_priority((*queue).priority);
    input.process_id = (*(*queue).vm).pasid; input.queue_type = (*queue).queue_type;
    input.mqd_addr = (*queue).mqd.gpu_addr; input.wptr_addr = (*props).wptr_gpu_addr;
    input.queue_size = (*props).queue_size >> 2; input.doorbell_offset = (*props).doorbell_index;
    input.page_table_base_addr = amdgpu_gmc_pd_addr((*(*queue).vm).root.bo); input.wptr_mc_addr = (*queue).wptr_obj.gpu_addr;
    if mes.use_rs64mem {
        if !(*uq_mgr).proc_ctx_allocated { let r = amdgpu_mes_alloc_proc_ctx_index(mes, &mut (*uq_mgr).proc_ctx_array_index); if r != 0 { DRM_ERROR!("Failed to allocate userq process index err:%d\n", r); return r; } (*uq_mgr).proc_ctx_allocated = true; }
        let r = amdgpu_mes_alloc_gang_ctx_index(mes, &mut (*queue).gang_ctx_array_index); if r != 0 { DRM_ERROR!("Failed to allocate userq gang index err:%d\n", r); return r; }
        input.process_context_array_index = (*uq_mgr).proc_ctx_array_index; input.gang_context_array_index = (*queue).gang_ctx_array_index;
    }
    amdgpu_mes_lock(&mut (*adev).mes); let r = ((*adev).mes.funcs).add_hw_queue(&mut (*adev).mes, &mut input); amdgpu_mes_unlock(&mut (*adev).mes);
    if r != 0 { DRM_ERROR!("Failed to map queue in HW, err (%d)\n", r); return r; }
    DRM_DEBUG_DRIVER!("Queue (doorbell:%d) mapped successfully\n", (*props).doorbell_index); 0
}

unsafe fn mes_userq_unmap(queue: *mut amdgpu_usermode_queue) -> i32 {
    let adev = (*(*queue).userq_mgr).adev; let ctx = &mut (*queue).fw_obj; let mes = &mut (*adev).mes;
    let mut input: mes_remove_queue_input = core::mem::zeroed(); input.gang_context_array_index = (*queue).gang_ctx_array_index;
    input.doorbell_offset = (*queue).doorbell_index; input.gang_context_addr = ctx.gpu_addr; input.queue_type = (*queue).queue_type;
    amdgpu_mes_lock(&mut (*adev).mes); let r = ((*adev).mes.funcs).remove_hw_queue(&mut (*adev).mes, &mut input); amdgpu_mes_unlock(&mut (*adev).mes);
    if mes.use_rs64mem { amdgpu_mes_free_gang_ctx_index(mes, (*queue).gang_ctx_array_index); }
    if r != 0 { DRM_ERROR!("Failed to unmap queue in HW, err (%d)\n", r); } r
}

pub unsafe fn mes_userq_reset(queue: *mut amdgpu_usermode_queue) -> i32 {
    let adev = (*(*queue).userq_mgr).adev; let mut input: mes_reset_queue_input = core::mem::zeroed();
    input.doorbell_offset = (*queue).doorbell_index; input.queue_type = (*queue).queue_type;
    amdgpu_mes_lock(&mut (*adev).mes); let mut r = ((*adev).mes.funcs).reset_hw_queue(&mut (*adev).mes, &mut input); amdgpu_mes_unlock(&mut (*adev).mes);
    if r != 0 { return r; }
    // mes_userq_unmap() does not update queue->state; mark it UNMAPPED so the destroy path does not issue a second REMOVE_QUEUE.
    r = mes_userq_unmap(queue); if r == 0 { trace_amdgpu_userq_state_changed(queue, AMDGPU_USERQ_STATE_UNMAPPED); (*queue).state = AMDGPU_USERQ_STATE_UNMAPPED; } r
}

pub unsafe fn mes_userq_reset_queue(adev: *mut amdgpu_device, guilty_uq: *mut amdgpu_usermode_queue, queue_type: i32, pipe: u32, queue: u32, db: u32) -> i32 {
    let mut uq: *mut amdgpu_usermode_queue; let mut uq_id: c_ulong; let use_mmio = (*adev).gfx.mec.use_mmio_for_reset;
    xa_for_each!(&(*adev).userq_doorbell_xa, uq_id, uq, { if (*uq).queue_type == queue_type && uq != guilty_uq && (*uq).doorbell_index == db {
        (*uq).state = AMDGPU_USERQ_STATE_HUNG;
        let mut r = if use_mmio { amdgpu_mes_reset_queue_mmio(adev, queue_type, 0, 1, pipe, queue, 0) } else { amdgpu_mes_reset_user_queue(adev, queue_type, db, 0) };
        if r != 0 { return r; } r = mes_userq_unmap(uq); if r != 0 { return r; } amdgpu_userq_fence_driver_force_completion(uq); break; }
    }); 0
}

unsafe fn mes_userq_create_ctx_space(uq_mgr: *mut amdgpu_userq_mgr, queue: *mut amdgpu_usermode_queue, _mqd_user: *mut drm_amdgpu_userq_in) -> i32 {
    let ctx = &mut (*queue).fw_obj; let r = amdgpu_bo_create_kernel((*uq_mgr).adev, AMDGPU_USERQ_GANG_CTX_SZ, 0, AMDGPU_GEM_DOMAIN_GTT, &mut ctx.obj, &mut ctx.gpu_addr, &mut ctx.cpu_ptr);
    if r != 0 { DRM_ERROR!("Failed to allocate ctx space bo for userqueue, err:%d\n", r); return r; } memset(ctx.cpu_ptr, 0, AMDGPU_USERQ_GANG_CTX_SZ); 0
}

unsafe fn mes_userq_create_proc_ctx_space(uq_mgr: *mut amdgpu_userq_mgr) -> i32 {
    let mut r = 0; mutex_lock(&mut (*uq_mgr).proc_ctx_lock);
    if (*uq_mgr).proc_ctx_obj.obj.is_null() { r = amdgpu_bo_create_kernel((*uq_mgr).adev, AMDGPU_USERQ_PROC_CTX_SZ, 0, AMDGPU_GEM_DOMAIN_GTT, &mut (*uq_mgr).proc_ctx_obj.obj, &mut (*uq_mgr).proc_ctx_obj.gpu_addr, &mut (*uq_mgr).proc_ctx_obj.cpu_ptr); if r == 0 { memset((*uq_mgr).proc_ctx_obj.cpu_ptr, 0, AMDGPU_USERQ_PROC_CTX_SZ); } }
    mutex_unlock(&mut (*uq_mgr).proc_ctx_lock); r
}

unsafe fn mes_userq_mqd_create(queue: *mut amdgpu_usermode_queue, args_in: *mut drm_amdgpu_userq_in) -> i32 {
    let uq_mgr = (*queue).userq_mgr; let adev = (*uq_mgr).adev; let default_mqd = &mut (*adev).mqds[(*queue).queue_type as usize];
    let user = args_in; let props = kzalloc_obj::<amdgpu_mqd_prop>(); if props.is_null() { DRM_ERROR!("Failed to allocate memory for userq_props\n"); return -ENOMEM; }
    let mut r = amdgpu_bo_create_kernel(adev, AMDGPU_MQD_SIZE_ALIGN((*default_mqd).mqd_size), 0, AMDGPU_GEM_DOMAIN_GTT, &mut (*queue).mqd.obj, &mut (*queue).mqd.gpu_addr, &mut (*queue).mqd.cpu_ptr);
    if r != 0 { DRM_ERROR!("Failed to create MQD object for userqueue\n"); goto!(free_props); }
    memset((*queue).mqd.cpu_ptr, 0, AMDGPU_MQD_SIZE_ALIGN((*default_mqd).mqd_size));
    (*props).wptr_gpu_addr = (*user).wptr_va; (*props).rptr_gpu_addr = (*user).rptr_va; (*props).queue_size = (*user).queue_size;
    (*props).hqd_base_gpu_addr = (*user).queue_va; (*props).mqd_gpu_addr = (*queue).mqd.gpu_addr; (*props).use_doorbell = true;
    (*props).doorbell_index = (*queue).doorbell_index; (*props).fence_address = (*(*queue).fence_drv).gpu_addr;
    if (*queue).queue_type == AMDGPU_HW_IP_COMPUTE {
        if (*user).mqd_size != core::mem::size_of::<drm_amdgpu_userq_mqd_compute_gfx11>() { DRM_ERROR!("Invalid compute IP MQD size\n"); r = -EINVAL; goto!(free_mqd); }
        let p = memdup_user(u64_to_user_ptr((*user).mqd), (*user).mqd_size); if IS_ERR!(p) { DRM_ERROR!("Failed to read user MQD\n"); r = -ENOMEM; goto!(free_mqd); }
        r = amdgpu_bo_reserve((*(*queue).vm).root.bo, false); if r != 0 { kfree(p); goto!(free_mqd); }
        r = amdgpu_userq_input_va_validate(adev, queue, (*p).eop_va, 2048, &mut (*queue).userq_vas.va.eop); amdgpu_bo_unreserve((*(*queue).vm).root.bo);
        if r != 0 { kfree(p); goto!(free_mqd); } (*props).eop_gpu_addr = (*p).eop_va; (*props).hqd_pipe_priority = AMDGPU_GFX_PIPE_PRIO_NORMAL; (*props).hqd_queue_priority = AMDGPU_GFX_QUEUE_PRIORITY_MINIMUM; (*props).hqd_active = false; (*props).tmz_queue = (*user).flags & AMDGPU_USERQ_CREATE_FLAGS_QUEUE_SECURE; kfree(p);
    } else if (*queue).queue_type == AMDGPU_HW_IP_GFX {
        let mut shadow: amdgpu_gfx_shadow_info = core::mem::zeroed(); if (*adev).gfx.funcs.get_gfx_shadow_info.is_some() { ((*adev).gfx.funcs).get_gfx_shadow_info(adev, &mut shadow, true); } else { r = -EINVAL; goto!(free_mqd); }
        if (*user).mqd_size != core::mem::size_of::<drm_amdgpu_userq_mqd_gfx11>() || (*user).mqd == 0 { DRM_ERROR!("Invalid GFX MQD\n"); r = -EINVAL; goto!(free_mqd); }
        let p = memdup_user(u64_to_user_ptr((*user).mqd), (*user).mqd_size); if IS_ERR!(p) { DRM_ERROR!("Failed to read user MQD\n"); r = -ENOMEM; goto!(free_mqd); }
        (*props).shadow_addr = (*p).shadow_va; (*props).csa_addr = (*p).csa_va; (*props).tmz_queue = (*user).flags & AMDGPU_USERQ_CREATE_FLAGS_QUEUE_SECURE;
        r = amdgpu_bo_reserve((*(*queue).vm).root.bo, false); if r != 0 { kfree(p); goto!(free_mqd); }
        r = amdgpu_userq_input_va_validate(adev, queue, (*p).shadow_va, shadow.shadow_size, &mut (*queue).userq_vas.va.shadow); if r != 0 { amdgpu_bo_unreserve((*(*queue).vm).root.bo); kfree(p); goto!(free_mqd); }
        r = amdgpu_userq_input_va_validate(adev, queue, (*p).csa_va, shadow.csa_size, &mut (*queue).userq_vas.va.csa); amdgpu_bo_unreserve((*(*queue).vm).root.bo); if r != 0 { kfree(p); goto!(free_mqd); } kfree(p);
    } else if (*queue).queue_type == AMDGPU_HW_IP_DMA {
        if (*user).mqd_size != core::mem::size_of::<drm_amdgpu_userq_mqd_sdma_gfx11>() || (*user).mqd == 0 { DRM_ERROR!("Invalid SDMA MQD\n"); r = -EINVAL; goto!(free_mqd); }
        let p = memdup_user(u64_to_user_ptr((*user).mqd), (*user).mqd_size); if IS_ERR!(p) { DRM_ERROR!("Failed to read sdma user MQD\n"); r = -ENOMEM; goto!(free_mqd); }
        r = amdgpu_bo_reserve((*(*queue).vm).root.bo, false); if r != 0 { kfree(p); goto!(free_mqd); } r = amdgpu_userq_input_va_validate(adev, queue, (*p).csa_va, 32, &mut (*queue).userq_vas.va.csa); amdgpu_bo_unreserve((*(*queue).vm).root.bo); if r != 0 { kfree(p); goto!(free_mqd); } (*props).csa_addr = (*p).csa_va; kfree(p);
    }
    (*queue).userq_prop = props; r = ((*default_mqd).init_mqd)(adev, (*queue).mqd.cpu_ptr as *mut core::ffi::c_void, props); if r != 0 { DRM_ERROR!("Failed to initialize MQD for userqueue\n"); goto!(free_mqd); }
    r = mes_userq_create_proc_ctx_space(uq_mgr); if r != 0 { DRM_ERROR!("Failed to allocate MES process context space bo, error: %d\n", r); goto!(free_mqd); }
    r = mes_userq_create_ctx_space(uq_mgr, queue, user); if r != 0 { DRM_ERROR!("Failed to allocate BO for userqueue (%d)", r); goto!(free_mqd); }
    r = mes_userq_create_wptr_mapping(adev, uq_mgr, queue, (*props).wptr_gpu_addr); if r != 0 { DRM_ERROR!("Failed to create WPTR mapping\n"); goto!(free_ctx); } return 0;
free_ctx: amdgpu_bo_free_kernel(&mut (*queue).fw_obj.obj, &mut (*queue).fw_obj.gpu_addr, &mut (*queue).fw_obj.cpu_ptr);
free_mqd: amdgpu_bo_free_kernel(&mut (*queue).mqd.obj, &mut (*queue).mqd.gpu_addr, &mut (*queue).mqd.cpu_ptr);
free_props: kfree(props); r
}

unsafe fn mes_userq_mqd_destroy(queue: *mut amdgpu_usermode_queue) { amdgpu_bo_free_kernel(&mut (*queue).fw_obj.obj, &mut (*queue).fw_obj.gpu_addr, &mut (*queue).fw_obj.cpu_ptr); kfree((*queue).userq_prop); amdgpu_bo_free_kernel(&mut (*queue).mqd.obj, &mut (*queue).mqd.gpu_addr, &mut (*queue).mqd.cpu_ptr); amdgpu_bo_unref(&mut (*queue).wptr_obj.obj); }

unsafe fn mes_userq_preempt(queue: *mut amdgpu_usermode_queue) -> i32 { let adev = (*(*queue).userq_mgr).adev; if (*queue).state != AMDGPU_USERQ_STATE_MAPPED { return 0; } let mut off=0; let mut r=amdgpu_wb_get(adev,&mut off); if r!=0{return r;} let ptr=(&mut (*adev).wb.wb[off as usize]) as *mut _ as *mut u64; *ptr=0; let mut input:mes_suspend_gang_input=core::mem::zeroed(); input.gang_context_addr=(*queue).fw_obj.gpu_addr; input.suspend_fence_addr=(*adev).wb.gpu_addr+off*4; input.suspend_fence_value=1; amdgpu_mes_lock(&mut (*adev).mes); r=((*adev).mes.funcs).suspend_gang(&mut (*adev).mes,&mut input); amdgpu_mes_unlock(&mut (*adev).mes); if r!=0{DRM_ERROR!("Failed to suspend gang: %d\n",r);amdgpu_wb_free(adev,off);return r;} for _ in 0..2100000 {if *ptr==1{amdgpu_wb_free(adev,off);return 0;} udelay(1);} amdgpu_wb_free(adev,off); -ETIMEDOUT }

unsafe fn mes_userq_restore(queue: *mut amdgpu_usermode_queue) -> i32 { let adev=(*(*queue).userq_mgr).adev; if (*queue).state==AMDGPU_USERQ_STATE_HUNG{return -EINVAL;} if (*queue).state!=AMDGPU_USERQ_STATE_PREEMPTED{return 0;} let mut input:mes_resume_gang_input=core::mem::zeroed();input.gang_context_addr=(*queue).fw_obj.gpu_addr;amdgpu_mes_lock(&mut (*adev).mes);let r=((*adev).mes.funcs).resume_gang(&mut (*adev).mes,&mut input);amdgpu_mes_unlock(&mut (*adev).mes);if r!=0{dev_err((*adev).dev,"Failed to resume queue, err (%d)\n",r);}r }

pub static userq_mes_funcs: amdgpu_userq_funcs = amdgpu_userq_funcs { mqd_create: mes_userq_mqd_create, mqd_destroy: mes_userq_mqd_destroy, unmap: mes_userq_unmap, map: mes_userq_map, preempt: mes_userq_preempt, restore: mes_userq_restore, reset: mes_userq_reset };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
