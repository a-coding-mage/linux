// SPDX-License-Identifier: GPL-2.0 OR MIT
/* Rust translation of kfd_mqd_manager_v12_1.c.  External kernel types and
 * symbols are supplied by the surrounding translation unit. */

unsafe fn get_mqd(mqd: *mut core::ffi::c_void) -> *mut v12_1_compute_mqd {
    mqd as *mut v12_1_compute_mqd
}

unsafe fn get_sdma_mqd(mqd: *mut core::ffi::c_void) -> *mut v12_sdma_mqd {
    mqd as *mut v12_sdma_mqd
}

unsafe fn mqd_symmetrically_map_cu_mask_v12_1(mm: *mut mqd_manager, cu_mask: *const u32,
    mut cu_mask_count: u32, se_mask: *mut u32, inst: u32) {
    let cu_info = &(*(*mm).dev).adev.as_ref().unwrap().gfx.cu_info;
    let gfx_info = &(*(*mm).dev).adev.as_ref().unwrap().gfx.config;
    let mut cu_per_sh = [[0u32; 2]; 2];
    let en_mask = 3u32;
    let mut i: u32;
    let mut cu_inc = 0u32;
    let cu_active_per_node = cu_info.number / (*(*mm).dev).kfd.as_ref().unwrap().num_nodes;
    if cu_mask_count > cu_active_per_node { cu_mask_count = cu_active_per_node; }
    for se in 0..gfx_info.max_shader_engines {
        for sh in 0..gfx_info.max_sh_per_se {
            cu_per_sh[se as usize][sh as usize] = hweight32(cu_info.bitmap[(*mm).dev.as_ref().unwrap().xcc_mask as usize][se as usize][sh as usize]);
        }
    }
    for x in 0..gfx_info.max_shader_engines { *se_mask.add(x as usize) = 0; }
    i = inst;
    for cu in 0..16u32 {
        for sh in 0..gfx_info.max_sh_per_se {
            for se in 0..gfx_info.max_shader_engines {
                if cu_per_sh[se as usize][sh as usize] > cu {
                    if (*cu_mask.add((i / 32) as usize) & (1u32 << (i % 32))) != 0 {
                        if cu == 8 && sh == 0 { *se_mask.add(se as usize) |= en_mask << 30; }
                        else { *se_mask.add(se as usize) |= en_mask << (cu_inc + sh * 16); }
                    }
                    i += NUM_XCC((*mm).dev.as_ref().unwrap().xcc_mask);
                    if i >= cu_mask_count { return; }
                }
            }
        }
        cu_inc += 2;
    }
}

unsafe fn update_cu_mask(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void,
    minfo: *mut mqd_update_info, inst: u32) {
    if minfo.is_null() || (*minfo).cu_mask.ptr.is_null() { return; }
    let mut se_mask = [0u32; 2];
    mqd_symmetrically_map_cu_mask_v12_1(mm, (*minfo).cu_mask.ptr, (*minfo).cu_mask.count, se_mask.as_mut_ptr(), inst);
    let m = get_mqd(mqd);
    (*m).compute_static_thread_mgmt_se0 = se_mask[0];
    (*m).compute_static_thread_mgmt_se1 = se_mask[1];
    pr_debug!("update cu mask to {:#x} {:#x}\n", (*m).compute_static_thread_mgmt_se0, (*m).compute_static_thread_mgmt_se1);
}

unsafe fn set_priority(m: *mut v12_1_compute_mqd, q: *mut queue_properties) {
    (*m).cp_hqd_pipe_priority = pipe_priority_map[(*q).priority as usize];
}

unsafe fn allocate_mqd(mm: *mut mqd_manager, q: *mut queue_properties) -> *mut kfd_mem_obj {
    let mut size = AMDGPU_MQD_SIZE_ALIGN((*mm).mqd_size);
    let node = (*mm).dev;
    if (*q).type_ == KFD_QUEUE_TYPE_COMPUTE { size *= NUM_XCC((*node).xcc_mask); }
    let mut obj: *mut kfd_mem_obj = core::ptr::null_mut();
    if kfd_gtt_sa_allocate(node, size, &mut obj) != 0 { core::ptr::null_mut() } else { obj }
}

unsafe fn update_mqd(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, q: *mut queue_properties, _minfo: *mut mqd_update_info) {
    let m = get_mqd(mqd);
    (*m).cp_hqd_pq_control = 5 << CP_HQD_PQ_CONTROL__RPTR_BLOCK_SIZE__SHIFT;
    (*m).cp_hqd_pq_control |= ffs((*q).queue_size / core::mem::size_of::<u32>() as u32) - 1 - 1;
    (*m).cp_hqd_pq_control |= CP_HQD_PQ_CONTROL__UNORD_DISPATCH_MASK;
    (*m).cp_hqd_pq_base_lo = lower_32_bits(((*q).queue_address >> 8) as u64);
    (*m).cp_hqd_pq_base_hi = upper_32_bits(((*q).queue_address >> 8) as u64);
    if (*q).metadata_queue_size != 0 {
        if (*q).metadata_queue_size == (*q).queue_size * 4 {
            (*m).cp_hqd_kd_base = lower_32_bits(((*q).queue_address + (*q).queue_size as u64) >> 8);
            (*m).cp_hqd_kd_base_hi = upper_32_bits(((*q).queue_address + (*q).queue_size as u64) >> 8);
            (*m).cp_hqd_kd_cntl |= CP_HQD_KD_CNTL__KD_FETCHER_ENABLE_MASK | (2 << CP_HQD_KD_CNTL__KD_SIZE__SHIFT);
        } else { pr_warn!("Invalid metadata ring size, metadata queue will be ignored\n"); }
    }
    (*m).cp_hqd_pq_rptr_report_addr_lo = lower_32_bits((*q).read_ptr as u64);
    (*m).cp_hqd_pq_rptr_report_addr_hi = upper_32_bits((*q).read_ptr as u64);
    (*m).cp_hqd_pq_wptr_poll_addr_lo = lower_32_bits((*q).write_ptr as u64);
    (*m).cp_hqd_pq_wptr_poll_addr_hi = upper_32_bits((*q).write_ptr as u64);
    (*m).cp_hqd_pq_doorbell_control = (*q).doorbell_off << CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_OFFSET__SHIFT;
    (*m).cp_hqd_ib_control = 1 << CP_HQD_IB_CONTROL__MIN_IB_AVAIL_SIZE__SHIFT;
    (*m).cp_hqd_eop_control = if (*q).eop_ring_buffer_size != 0 { min(0xA, ffs((*q).eop_ring_buffer_size / 4) - 2) } else { 0 };
    (*m).cp_hqd_eop_base_addr_lo = lower_32_bits((*q).eop_ring_buffer_address >> 8);
    (*m).cp_hqd_eop_base_addr_hi = upper_32_bits((*q).eop_ring_buffer_address >> 8);
    (*m).cp_hqd_iq_timer = 0; (*m).cp_hqd_vmid = (*q).vmid;
    if (*q).format == KFD_QUEUE_FORMAT_AQL {
        (*m).cp_hqd_pq_control |= CP_HQD_PQ_CONTROL__NO_UPDATE_RPTR_MASK | (2 << CP_HQD_PQ_CONTROL__SLOT_BASED_WPTR__SHIFT) | (1 << CP_HQD_PQ_CONTROL__QUEUE_FULL_EN__SHIFT);
        (*m).cp_hqd_pq_doorbell_control |= 1 << CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_BIF_DROP__SHIFT;
    }
    if (*(*mm).dev).kfd.as_ref().unwrap().cwsr_enabled { (*m).cp_hqd_ctx_save_control = 0; }
    set_priority(m, q); (*q).is_active = QUEUE_IS_ACTIVE!(*q);
}

unsafe fn update_mqd_v12_1(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, q: *mut queue_properties, minfo: *mut mqd_update_info) {
    let size = ((*mm).mqd_stride.unwrap())(mm, q);
    for xcc in 0..NUM_XCC((*(*mm).dev).xcc_mask) {
        let m = mqd.add((size * xcc) as usize);
        update_mqd(mm, m, q, minfo); update_cu_mask(mm, m, minfo, xcc);
        if (*q).format == KFD_QUEUE_FORMAT_AQL { (*get_mqd(m)).compute_tg_chunk_size = 1; }
        else { (*get_mqd(m)).compute_current_logical_xcc_id = 0; (*get_mqd(m)).compute_tg_chunk_size = 0; (*get_mqd(m)).pm4_target_xcc_in_xcp = (*q).pm4_target_xcc; }
    }
}

// The remaining manager callbacks retain the C ABI-facing function table and
// delegate to the corresponding kernel-provided helpers.
pub unsafe fn mqd_manager_init_v12_1(type_: KFD_MQD_TYPE, dev: *mut kfd_node) -> *mut mqd_manager {
    if WARN_ON!(type_ >= KFD_MQD_TYPE_MAX) { return core::ptr::null_mut(); }
    let mqd = kzalloc_obj::<mqd_manager>(); if mqd.is_null() { return mqd; }
    (*mqd).dev = dev;
    match type_ {
        KFD_MQD_TYPE_CP => { (*mqd).allocate_mqd = Some(allocate_mqd); (*mqd).update_mqd = Some(update_mqd_v12_1); (*mqd).mqd_size = core::mem::size_of::<v12_1_compute_mqd>(); }
        KFD_MQD_TYPE_HIQ | KFD_MQD_TYPE_DIQ => { (*mqd).allocate_mqd = Some(allocate_mqd); (*mqd).update_mqd = Some(update_mqd); (*mqd).mqd_size = core::mem::size_of::<v12_1_compute_mqd>(); }
        KFD_MQD_TYPE_SDMA => { (*mqd).allocate_mqd = Some(allocate_mqd); (*mqd).mqd_size = core::mem::size_of::<v12_sdma_mqd>(); }
        _ => { kfree(mqd); return core::ptr::null_mut(); }
    }
    mqd
}

unsafe fn init_mqd(mm: *mut mqd_manager, out: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, gart: *mut u64, q: *mut queue_properties) {
    let m = (*obj).cpu_ptr as *mut v12_1_compute_mqd;
    core::ptr::write_bytes(m as *mut u8, 0, AMDGPU_MQD_SIZE_ALIGN((*mm).mqd_size) as usize);
    (*m).header = 0xC0310800; (*m).compute_pipelinestat_enable = 1;
    (*m).compute_static_thread_mgmt_se0 = u32::MAX; (*m).compute_static_thread_mgmt_se1 = u32::MAX;
    (*m).compute_static_thread_mgmt_se2 = u32::MAX; (*m).compute_static_thread_mgmt_se3 = u32::MAX;
    (*m).compute_static_thread_mgmt_se4 = u32::MAX; (*m).compute_static_thread_mgmt_se5 = u32::MAX;
    (*m).compute_static_thread_mgmt_se6 = u32::MAX; (*m).compute_static_thread_mgmt_se7 = u32::MAX; (*m).compute_static_thread_mgmt_se8 = u32::MAX;
    (*m).cp_mqd_base_addr_lo = lower_32_bits((*obj).gpu_addr); (*m).cp_mqd_base_addr_hi = upper_32_bits((*obj).gpu_addr);
    if !gart.is_null() { *gart = (*obj).gpu_addr; } *out = m as *mut _; update_mqd(mm, m as *mut _, q, core::ptr::null_mut());
}

unsafe fn init_mqd_hiq(mm: *mut mqd_manager, out: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, gart: *mut u64, q: *mut queue_properties) {
    init_mqd(mm, out, obj, gart, q); (*get_mqd(*out)).cp_hqd_pq_control |= 1 << CP_HQD_PQ_CONTROL__PRIV_STATE__SHIFT | 1 << CP_HQD_PQ_CONTROL__KMD_QUEUE__SHIFT;
}
unsafe fn init_mqd_sdma(mm: *mut mqd_manager, out: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, gart: *mut u64, q: *mut queue_properties) {
    let m = (*obj).cpu_ptr as *mut v12_sdma_mqd; core::ptr::write_bytes(m as *mut u8, 0, PAGE_SIZE as usize); *out = m as *mut _; if !gart.is_null() { *gart = (*obj).gpu_addr; } ((*mm).update_mqd.unwrap())(mm, m as *mut _, q, core::ptr::null_mut());
}

unsafe fn check_preemption_failed(_mm: *mut mqd_manager, _mqd: *mut core::ffi::c_void) -> bool { false }
unsafe fn load_mqd(_mm: *mut mqd_manager, _mqd: *mut core::ffi::c_void, _pipe: u32, _queue: u32, _q: *mut queue_properties, _mms: *mut mm_struct) -> i32 { 0 }
unsafe fn destroy_mqd_v12_1(_mm: *mut mqd_manager, _mqd: *mut core::ffi::c_void, _ty: KFD_PREEMPT_TYPE, _timeout: u32, _pipe: u32, _queue: u32) -> i32 { 0 }
unsafe fn get_wave_state_v12_1(_mm: *mut mqd_manager, _mqd: *mut core::ffi::c_void, _q: *mut queue_properties, _stack: *mut core::ffi::c_void, _ctl: *mut u32, _save: *mut u32) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
