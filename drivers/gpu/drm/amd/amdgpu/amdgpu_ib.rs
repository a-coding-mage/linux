/*
 * Copyright 2008 Advanced Micro Devices, Inc.
 * Copyright 2008 Red Hat Inc.
 * Copyright 2009 Jerome Glisse.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependencies supplied by the surrounding driver translation.

const AMDGPU_IB_TEST_TIMEOUT: _ = msecs_to_jiffies(1000);
const AMDGPU_IB_TEST_GFX_XGMI_TIMEOUT: _ = msecs_to_jiffies(2000);

pub unsafe fn amdgpu_ib_get(adev: *mut amdgpu_device, vm: *mut amdgpu_vm,
    size: c_uint, pool_type: amdgpu_ib_pool_type, ib: *mut amdgpu_ib) -> c_int {
    let mut r: c_int;
    if size != 0 {
        r = amdgpu_sa_bo_new(&mut (*adev).ib_pools[pool_type as usize], &mut (*ib).sa_bo, size);
        if r != 0 { dev_err((*adev).dev, "failed to get a new IB (%d)\n", r); return r; }
        (*ib).ptr = amdgpu_sa_bo_cpu_addr((*ib).sa_bo);
        (*ib).flags = AMDGPU_IB_FLAG_EMIT_MEM_SYNC;
        if vm.is_null() { (*ib).gpu_addr = amdgpu_sa_bo_gpu_addr((*ib).sa_bo); }
    }
    0
}

pub unsafe fn amdgpu_ib_free(ib: *mut amdgpu_ib, f: *mut dma_fence) {
    amdgpu_sa_bo_free(&mut (*ib).sa_bo, f);
}

pub unsafe fn amdgpu_ib_schedule(ring: *mut amdgpu_ring, num_ibs: c_uint,
    ibs: *mut amdgpu_ib, job: *mut amdgpu_job, f: *mut *mut dma_fence) -> c_int {
    let adev = (*ring).adev;
    let mut ib = ibs;
    let mut tmp: *mut dma_fence = core::ptr::null_mut();
    let mut af: *mut amdgpu_fence;
    let vm: *mut amdgpu_vm;
    let mut fence_ctx: u64;
    let mut status: u32 = 0;
    let mut fence_flags: c_uint = 0;
    let mut shadow_va: u64 = 0; let mut csa_va: u64 = 0; let mut gds_va: u64 = 0;
    let vmid = AMDGPU_JOB_GET_VMID(job);
    let mut need_pipe_sync = false;
    let mut emit_spm_needed = false; let mut emit_gds_needed = false;
    let mut cond_exec: c_uint = 0;
    if num_ibs == 0 { return -EINVAL; }
    if !job.is_null() {
        vm = (*job).vm; fence_ctx = if !(*job).base.s_fence.is_null() { (*job).base.s_fence.as_ref().unwrap().finished.context } else { 0 };
        shadow_va = (*job).shadow_va; csa_va = (*job).csa_va; gds_va = (*job).gds_va;
        af = (*job).hw_fence; (*af).context = fence_ctx; (*job).hw_vm_fence.context = fence_ctx;
    } else {
        vm = core::ptr::null_mut(); fence_ctx = 0;
        af = kzalloc_obj::<amdgpu_fence>(GFP_ATOMIC); if af.is_null() { return -ENOMEM; }
    }
    if !(*ring).sched.ready { dev_err((*adev).dev, "couldn't schedule ib on ring <%s>\n", (*ring).name); goto_free_fence!(job, af, -EINVAL); }
    if !vm.is_null() && (*job).vmid == 0 { dev_err((*adev).dev, "VM IB without ID\n"); goto_free_fence!(job, af, -EINVAL); }
    if ((*ib).flags & AMDGPU_IB_FLAGS_SECURE) != 0 && !(*ring).funcs.secure_submission_supported { dev_err((*adev).dev, "secure submissions not supported on ring <%s>\n", (*ring).name); goto_free_fence!(job, af, -EINVAL); }
    let alloc_size = (*ring).funcs.emit_frame_size + num_ibs * (*ring).funcs.emit_ib_size;
    let r = amdgpu_ring_alloc(ring, alloc_size); if r != 0 { dev_err((*adev).dev, "scheduling IB failed (%d).\n", r); goto_free_fence!(job, af, r); }
    let need_ctx_switch = (*ring).current_ctx != fence_ctx;
    if (*ring).funcs.emit_pipeline_sync && !job.is_null() && ({ tmp = amdgpu_sync_get_fence(&mut (*job).explicit_sync); !tmp.is_null() || need_ctx_switch || amdgpu_vm_need_pipeline_sync(ring, job) }) { need_pipe_sync = true; if !tmp.is_null() { trace_amdgpu_ib_pipe_sync(job, tmp); } dma_fence_put(tmp); }
    if !job.is_null() { (*job).hw_vm_fence.ib_wptr = (*ring).wptr; amdgpu_vm_flush(ring, job, &mut need_pipe_sync, &mut emit_spm_needed, &mut emit_gds_needed); (*job).hw_vm_fence.ib_dw_size = amdgpu_ring_get_dw_distance(ring, (*job).hw_vm_fence.ib_wptr, (*ring).wptr); }
    (*af).ib_wptr = (*ring).wptr; amdgpu_ring_ib_begin(ring);
    if (*ring).funcs.insert_start { ((*ring).funcs.insert_start.unwrap())(ring); }
    if need_pipe_sync { amdgpu_ring_emit_pipeline_sync(ring); }
    if emit_spm_needed { ((*adev).gfx.rlc.funcs.update_spm_vmid.unwrap())(adev, (*ring).xcc_id, ring, (*job).vmid); }
    if emit_gds_needed { amdgpu_ring_emit_gds_switch(ring, (*job).vmid, (*job).gds_base, (*job).gds_size, (*job).gws_base, (*job).gws_size, (*job).oa_base, (*job).oa_size); }
    if ((*ib).flags & AMDGPU_IB_FLAG_EMIT_MEM_SYNC) != 0 && (*ring).funcs.emit_mem_sync { ((*ring).funcs.emit_mem_sync.unwrap())(ring); }
    if (*ring).funcs.emit_wave_limit && (*ring).hw_prio == AMDGPU_GFX_PIPE_PRIO_HIGH { ((*ring).funcs.emit_wave_limit.unwrap())(ring, true); }
    if (*ring).funcs.emit_gfx_shadow && (*adev).gfx.cp_gfx_shadow { amdgpu_ring_emit_gfx_shadow(ring, shadow_va, csa_va, gds_va, (*job).init_shadow, vmid); }
    if (*ring).funcs.init_cond_exec { cond_exec = amdgpu_ring_init_cond_exec(ring, (*ring).cond_exe_gpu_addr); }
    (*af).skip_ib_dw_start_offset = amdgpu_ring_get_dw_distance(ring, (*af).ib_wptr, (*ring).wptr); amdgpu_device_flush_hdp(adev, ring);
    if need_ctx_switch { status |= AMDGPU_HAVE_CTX_SWITCH; }
    if !job.is_null() && (*ring).funcs.emit_cntxcntl { status |= (*job).preamble_status | (*job).preemption_status; amdgpu_ring_emit_cntxcntl(ring, status); }
    let mut secure = false; if !job.is_null() && (*ring).funcs.emit_frame_cntl { secure = ((*ib).flags & AMDGPU_IB_FLAGS_SECURE) != 0; amdgpu_ring_emit_frame_cntl(ring, true, secure); }
    for i in 0..num_ibs { ib = ibs.add(i as usize); if !job.is_null() && (*ring).funcs.emit_frame_cntl && (secure != (((*ib).flags & AMDGPU_IB_FLAGS_SECURE) != 0)) { amdgpu_ring_emit_frame_cntl(ring, false, secure); secure = !secure; amdgpu_ring_emit_frame_cntl(ring, true, secure); } amdgpu_ring_emit_ib(ring, job, ib, status); status &= !AMDGPU_HAVE_CTX_SWITCH; }
    if !job.is_null() && (*ring).funcs.emit_frame_cntl { amdgpu_ring_emit_frame_cntl(ring, false, secure); }
    amdgpu_device_invalidate_hdp(adev, ring); (*af).skip_ib_dw_end_offset = amdgpu_ring_get_dw_distance(ring, (*af).ib_wptr, (*ring).wptr);
    if ((*ib).flags & AMDGPU_IB_FLAG_TC_WB_NOT_INVALIDATE) != 0 { fence_flags |= AMDGPU_FENCE_FLAG_TC_WB_ONLY; }
    if !job.is_null() && (*job).uf_addr != 0 { amdgpu_ring_emit_fence(ring, (*job).uf_addr, (*job).uf_sequence, fence_flags | AMDGPU_FENCE_FLAG_64BIT); }
    if (*ring).funcs.emit_gfx_shadow && (*ring).funcs.init_cond_exec && (*adev).gfx.cp_gfx_shadow { amdgpu_ring_emit_gfx_shadow(ring, 0, 0, 0, false, 0); amdgpu_ring_init_cond_exec(ring, (*ring).cond_exe_gpu_addr); }
    amdgpu_fence_emit(ring, af, fence_flags); *f = &mut (*af).base; if !job.is_null() { dma_fence_get(*f); }
    if (*ring).funcs.insert_end { ((*ring).funcs.insert_end.unwrap())(ring); } amdgpu_ring_patch_cond_exec(ring, cond_exec); (*ring).current_ctx = fence_ctx;
    if !job.is_null() && (*ring).funcs.emit_switch_buffer { ((*ring).funcs.emit_switch_buffer.unwrap())(ring); }
    if (*ring).funcs.emit_wave_limit && (*ring).hw_prio == AMDGPU_GFX_PIPE_PRIO_HIGH { ((*ring).funcs.emit_wave_limit.unwrap())(ring, false); }
    amdgpu_ring_ib_end(ring); (*af).ib_dw_size = amdgpu_ring_get_dw_distance(ring, (*af).ib_wptr, (*ring).wptr); amdgpu_ring_commit(ring); 0
}

pub unsafe fn amdgpu_ib_pool_init(adev: *mut amdgpu_device) -> c_int {
    let sizes = [SZ_1M, SZ_128K, SZ_512K]; let gfp_flags = [GFP_KERNEL, GFP_ATOMIC, GFP_ATOMIC];
    if (*adev).ib_pool_ready { return 0; } let mut i = 0; let mut r = 0;
    while i < AMDGPU_IB_POOL_MAX { r = amdgpu_sa_bo_manager_init(adev, &mut (*adev).ib_pools[i], sizes[i], gfp_flags[i]); if r != 0 { while i > 0 { i -= 1; amdgpu_sa_bo_manager_fini(adev, &mut (*adev).ib_pools[i]); } return r; } i += 1; }
    (*adev).ib_pool_ready = true; 0
}

pub unsafe fn amdgpu_ib_pool_fini(adev: *mut amdgpu_device) { if !(*adev).ib_pool_ready { return; } for i in 0..AMDGPU_IB_POOL_MAX { amdgpu_sa_bo_manager_fini(adev, &mut (*adev).ib_pools[i]); } (*adev).ib_pool_ready = false; }
pub unsafe fn amdgpu_ib_pool_gfp_flags(adev: *mut amdgpu_device, ty: amdgpu_ib_pool_type) -> gfp_t { (*adev).ib_pools[ty as usize].gfp_flags }

pub unsafe fn amdgpu_ib_ring_tests(adev: *mut amdgpu_device) -> c_int {
    let mut tmo_gfx = AMDGPU_IB_TEST_TIMEOUT; let mut tmo_mm = tmo_gfx; let mut ret = 0;
    if amdgpu_sriov_vf(adev) { tmo_mm = 8 * AMDGPU_IB_TEST_TIMEOUT; }
    if amdgpu_sriov_runtime(adev) { tmo_gfx = 8 * AMDGPU_IB_TEST_TIMEOUT; } else if (*adev).gmc.xgmi.hive_id != 0 { tmo_gfx = AMDGPU_IB_TEST_GFX_XGMI_TIMEOUT; }
    for i in 0..(*adev).num_rings { let ring = *(*adev).rings.add(i as usize); if !(*ring).sched.ready || !(*ring).funcs.test_ib { continue; } if (*adev).enable_mes && (*ring).funcs.r#type == AMDGPU_RING_TYPE_KIQ { continue; } let tmo = match (*ring).funcs.r#type { AMDGPU_RING_TYPE_UVD | AMDGPU_RING_TYPE_VCE | AMDGPU_RING_TYPE_UVD_ENC | AMDGPU_RING_TYPE_VCN_DEC | AMDGPU_RING_TYPE_VCN_ENC | AMDGPU_RING_TYPE_VCN_JPEG => tmo_mm, _ => tmo_gfx }; let r = amdgpu_ring_test_ib(ring, tmo); if r == 0 { DRM_DEV_DEBUG((*adev).dev, "ib test on %s succeeded\n", (*ring).name); continue; } (*ring).sched.ready = false; DRM_DEV_ERROR((*adev).dev, "IB test failed on %s (%d).\n", (*ring).name, r); if ring == &mut (*adev).gfx.gfx_ring[0] { (*adev).accel_working = false; return r; } ret = r; }
    ret
}

pub unsafe fn amdgpu_debugfs_sa_init(_adev: *mut amdgpu_device) {
    // CONFIG_DEBUG_FS-gated debugfs registration is supplied by the platform.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
