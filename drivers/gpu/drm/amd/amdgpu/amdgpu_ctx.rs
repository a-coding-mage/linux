/*
 * Copyright 2015 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

/* Dependencies are supplied by the surrounding kernel translation. */

#[no_mangle]
pub static amdgpu_ctx_num_entities: [c_uint; AMDGPU_HW_IP_NUM as usize] = [
    /* AMDGPU_HW_IP_GFX */ 1,       /* COMPUTE */ 4,
    /* DMA */ 2,                    /* UVD */ 1,
    /* VCE */ 1,                    /* UVD_ENC */ 1,
    /* VCN_DEC */ 1,                /* VCN_ENC */ 1,
    /* VCN_JPEG */ 1,               /* VPE */ 1,
];

pub unsafe fn amdgpu_ctx_priority_is_valid(ctx_prio: i32) -> bool {
    match ctx_prio {
        AMDGPU_CTX_PRIORITY_VERY_LOW | AMDGPU_CTX_PRIORITY_LOW |
        AMDGPU_CTX_PRIORITY_NORMAL | AMDGPU_CTX_PRIORITY_HIGH |
        AMDGPU_CTX_PRIORITY_VERY_HIGH => true,
        AMDGPU_CTX_PRIORITY_UNSET => {
            /* UNSET priority is normalized by amdgpu_ctx_ioctl(). */
            false
        }
        _ => false,
    }
}

unsafe fn amdgpu_ctx_to_drm_sched_prio(ctx_prio: i32) -> drm_sched_priority {
    match ctx_prio {
        AMDGPU_CTX_PRIORITY_UNSET => {
            pr_warn_once!("AMD-->DRM context priority value UNSET-->NORMAL");
            DRM_SCHED_PRIORITY_NORMAL
        }
        AMDGPU_CTX_PRIORITY_VERY_LOW | AMDGPU_CTX_PRIORITY_LOW => DRM_SCHED_PRIORITY_LOW,
        AMDGPU_CTX_PRIORITY_NORMAL => DRM_SCHED_PRIORITY_NORMAL,
        AMDGPU_CTX_PRIORITY_HIGH | AMDGPU_CTX_PRIORITY_VERY_HIGH => DRM_SCHED_PRIORITY_HIGH,
        _ => {
            WARN!(true, "Invalid context priority %d\n", ctx_prio);
            DRM_SCHED_PRIORITY_NORMAL
        }
    }
}

unsafe fn amdgpu_ctx_priority_permit(filp: *mut drm_file, priority: i32) -> c_int {
    if priority <= AMDGPU_CTX_PRIORITY_NORMAL || capable(CAP_SYS_NICE) || drm_is_current_master(filp) { 0 } else { -EACCES }
}

unsafe fn amdgpu_ctx_prio_to_gfx_pipe_prio(prio: i32) -> amdgpu_gfx_pipe_priority {
    match prio { AMDGPU_CTX_PRIORITY_HIGH | AMDGPU_CTX_PRIORITY_VERY_HIGH => AMDGPU_GFX_PIPE_PRIO_HIGH, _ => AMDGPU_GFX_PIPE_PRIO_NORMAL }
}

unsafe fn amdgpu_ctx_sched_prio_to_ring_prio(prio: i32) -> amdgpu_ring_priority_level {
    match prio { AMDGPU_CTX_PRIORITY_HIGH => AMDGPU_RING_PRIO_1, AMDGPU_CTX_PRIORITY_VERY_HIGH => AMDGPU_RING_PRIO_2, _ => AMDGPU_RING_PRIO_0 }
}

unsafe fn amdgpu_ctx_get_hw_prio(ctx: *mut amdgpu_ctx, mut hw_ip: u32) -> c_uint {
    let adev = (*(*ctx).mgr).adev;
    let ctx_prio = if (*ctx).override_priority == AMDGPU_CTX_PRIORITY_UNSET { (*ctx).init_priority } else { (*ctx).override_priority };
    let mut hw_prio = match hw_ip { AMDGPU_HW_IP_GFX | AMDGPU_HW_IP_COMPUTE => amdgpu_ctx_prio_to_gfx_pipe_prio(ctx_prio) as c_uint, AMDGPU_HW_IP_VCE | AMDGPU_HW_IP_VCN_ENC => amdgpu_ctx_sched_prio_to_ring_prio(ctx_prio) as c_uint, _ => AMDGPU_RING_PRIO_DEFAULT };
    hw_ip = array_index_nospec(hw_ip, AMDGPU_HW_IP_NUM);
    if (*adev).gpu_sched[hw_ip as usize][hw_prio as usize].num_scheds == 0 { hw_prio = AMDGPU_RING_PRIO_DEFAULT; }
    hw_prio
}

/* Calculate the time spent on the hardware. */
unsafe fn amdgpu_ctx_fence_time(fence: *mut dma_fence) -> ktime_t {
    if fence.is_null() { return ns_to_ktime(0); }
    let s_fence = to_drm_sched_fence(fence);
    if !test_bit(DMA_FENCE_FLAG_TIMESTAMP_BIT, &(*s_fence).scheduled.flags) { return ns_to_ktime(0); }
    if !test_bit(DMA_FENCE_FLAG_TIMESTAMP_BIT, &(*s_fence).finished.flags) { return ktime_sub(ktime_get(), (*s_fence).scheduled.timestamp); }
    ktime_sub((*s_fence).finished.timestamp, (*s_fence).scheduled.timestamp)
}

unsafe fn amdgpu_ctx_entity_time(ctx: *mut amdgpu_ctx, centity: *mut amdgpu_ctx_entity) -> ktime_t {
    let mut res = ns_to_ktime(0); spin_lock(&mut (*ctx).ring_lock);
    for i in 0..amdgpu_sched_jobs { res = ktime_add(res, amdgpu_ctx_fence_time((*centity).fences[i as usize])); }
    spin_unlock(&mut (*ctx).ring_lock); res
}

/* The remaining routines retain the kernel ABI and operations directly. */
pub unsafe fn amdgpu_ctx_get(fpriv: *mut amdgpu_fpriv, id: u32) -> *mut amdgpu_ctx {
    if fpriv.is_null() { return core::ptr::null_mut(); }
    let mgr = &mut (*fpriv).ctx_mgr; xa_lock(&mut mgr.ctx_handles);
    let ctx = xa_load(&mut mgr.ctx_handles, id); if !ctx.is_null() { kref_get(&mut (*ctx).refcount); }
    xa_unlock(&mut mgr.ctx_handles); ctx
}

pub unsafe fn amdgpu_ctx_add_fence(ctx: *mut amdgpu_ctx, entity: *mut drm_sched_entity, fence: *mut dma_fence) -> u64 {
    let centity = to_amdgpu_ctx_entity(entity); let seq = (*centity).sequence;
    let idx = seq & (amdgpu_sched_jobs - 1); let other = (*centity).fences[idx as usize];
    WARN_ON(!other.is_null() && !dma_fence_is_signaled(other)); dma_fence_get(fence);
    spin_lock(&mut (*ctx).ring_lock); (*centity).fences[idx as usize] = fence; (*centity).sequence += 1; spin_unlock(&mut (*ctx).ring_lock);
    atomic64_add(ktime_to_ns(amdgpu_ctx_fence_time(other)), &mut (*(*ctx).mgr).time_spend[(*centity).hw_ip as usize]); dma_fence_put(other); seq
}

pub unsafe fn amdgpu_ctx_get_fence(ctx: *mut amdgpu_ctx, entity: *mut drm_sched_entity, mut seq: u64) -> *mut dma_fence {
    let centity = to_amdgpu_ctx_entity(entity); spin_lock(&mut (*ctx).ring_lock);
    if seq == !0u64 { seq = (*centity).sequence - 1; }
    if seq >= (*centity).sequence { spin_unlock(&mut (*ctx).ring_lock); return ERR_PTR(-EINVAL); }
    if seq + amdgpu_sched_jobs < (*centity).sequence { spin_unlock(&mut (*ctx).ring_lock); return core::ptr::null_mut(); }
    let fence = dma_fence_get((*centity).fences[(seq & (amdgpu_sched_jobs - 1)) as usize]); spin_unlock(&mut (*ctx).ring_lock); fence
}

/* File-local declarations whose full bodies depend on surrounding translated kernel types. */
pub unsafe fn amdgpu_ctx_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int {
    let args = data as *mut drm_amdgpu_ctx;
    let adev = drm_to_adev(dev); let fpriv = (*filp).driver_priv;
    let mut priority = (*args).input.priority; let id = (*args).input.ctx_id;
    if !amdgpu_ctx_priority_is_valid(priority) { priority = AMDGPU_CTX_PRIORITY_NORMAL; }
    match (*args).input.op {
        AMDGPU_CTX_OP_ALLOC_CTX => { if (*args).input.flags != 0 { return -EINVAL; } let mut out_id = 0; let r = amdgpu_ctx_alloc(adev, fpriv, filp, priority, &mut out_id); (*args).output.alloc.ctx_id = out_id; r }
        AMDGPU_CTX_OP_FREE_CTX => { if (*args).input.flags != 0 { return -EINVAL; } amdgpu_ctx_free(fpriv, id) }
        AMDGPU_CTX_OP_QUERY_STATE => { if (*args).input.flags != 0 { return -EINVAL; } amdgpu_ctx_query(adev, fpriv, id, &mut (*args).output) }
        AMDGPU_CTX_OP_QUERY_STATE2 => { if (*args).input.flags != 0 { return -EINVAL; } amdgpu_ctx_query2(adev, fpriv, id, &mut (*args).output) }
        _ => -EINVAL,
    }
}

pub unsafe fn amdgpu_ctx_priority_override(ctx: *mut amdgpu_ctx, priority: i32) {
    (*ctx).override_priority = priority;
    let p = if priority == AMDGPU_CTX_PRIORITY_UNSET { (*ctx).init_priority } else { priority };
    for i in 0..AMDGPU_HW_IP_NUM as usize { for j in 0..amdgpu_ctx_num_entities[i] as usize {
        let e = (*ctx).entities[i][j]; if !e.is_null() { drm_sched_entity_set_priority(&mut (*e).entity, amdgpu_ctx_to_drm_sched_prio(p)); }
    }}
}

pub unsafe fn amdgpu_ctx_mgr_init(mgr: *mut amdgpu_ctx_mgr, adev: *mut amdgpu_device) {
    (*mgr).adev = adev; xa_init_flags(&mut (*mgr).ctx_handles, XA_FLAGS_ALLOC1);
    for i in 0..AMDGPU_HW_IP_NUM as usize { atomic64_set(&mut (*mgr).time_spend[i], 0); }
}

pub unsafe fn amdgpu_ctx_mgr_fini(mgr: *mut amdgpu_ctx_mgr) { xa_destroy(&mut (*mgr).ctx_handles); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
