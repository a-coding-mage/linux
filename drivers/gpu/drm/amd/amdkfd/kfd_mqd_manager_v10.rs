// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2018-2022 Advanced Micro Devices, Inc.
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

// C includes and build-time dependencies are supplied by the surrounding kernel translation.

#[inline]
unsafe fn get_mqd(mqd: *mut core::ffi::c_void) -> *mut v10_compute_mqd {
    mqd as *mut v10_compute_mqd
}

#[inline]
unsafe fn get_sdma_mqd(mqd: *mut core::ffi::c_void) -> *mut v10_sdma_mqd {
    mqd as *mut v10_sdma_mqd
}

unsafe fn update_cu_mask(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, minfo: *mut mqd_update_info) {
    let mut se_mask = [0u32; 4];
    if minfo.is_null() || (*minfo).cu_mask.ptr.is_null() { return; }
    mqd_symmetrically_map_cu_mask(mm, (*minfo).cu_mask.ptr, (*minfo).cu_mask.count, se_mask.as_mut_ptr(), 0);
    let m = get_mqd(mqd);
    (*m).compute_static_thread_mgmt_se0 = se_mask[0];
    (*m).compute_static_thread_mgmt_se1 = se_mask[1];
    (*m).compute_static_thread_mgmt_se2 = se_mask[2];
    (*m).compute_static_thread_mgmt_se3 = se_mask[3];
    pr_debug!("update cu mask to %#x %#x %#x %#x\n", (*m).compute_static_thread_mgmt_se0, (*m).compute_static_thread_mgmt_se1, (*m).compute_static_thread_mgmt_se2, (*m).compute_static_thread_mgmt_se3);
}

unsafe fn set_priority(m: *mut v10_compute_mqd, q: *mut queue_properties) {
    (*m).cp_hqd_pipe_priority = pipe_priority_map[(*q).priority as usize];
}

unsafe fn allocate_mqd(mm: *mut mqd_manager, _q: *mut queue_properties) -> *mut kfd_mem_obj {
    let mut obj: *mut kfd_mem_obj = core::ptr::null_mut();
    if kfd_gtt_sa_allocate((*mm).dev, core::mem::size_of::<v10_compute_mqd>(), &mut obj) != 0 { return core::ptr::null_mut(); }
    obj
}

unsafe fn init_mqd(mm: *mut mqd_manager, mqd: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, gart_addr: *mut u64, q: *mut queue_properties) {
    let m = (*obj).cpu_ptr as *mut v10_compute_mqd;
    let addr = (*obj).gpu_addr;
    core::ptr::write_bytes(m as *mut u8, 0, core::mem::size_of::<v10_compute_mqd>());
    (*m).header = 0xC0310800;
    (*m).compute_pipelinestat_enable = 1;
    (*m).compute_static_thread_mgmt_se0 = 0xFFFF_FFFF;
    (*m).compute_static_thread_mgmt_se1 = 0xFFFF_FFFF;
    (*m).compute_static_thread_mgmt_se2 = 0xFFFF_FFFF;
    (*m).compute_static_thread_mgmt_se3 = 0xFFFF_FFFF;
    (*m).cp_hqd_persistent_state = CP_HQD_PERSISTENT_STATE__PRELOAD_REQ_MASK | (0x53 << CP_HQD_PERSISTENT_STATE__PRELOAD_SIZE__SHIFT);
    (*m).cp_hqd_pq_control = 5 << CP_HQD_PQ_CONTROL__RPTR_BLOCK_SIZE__SHIFT;
    (*m).cp_hqd_pq_control |= CP_HQD_PQ_CONTROL__UNORD_DISPATCH_MASK;
    (*m).cp_mqd_control = 1 << CP_MQD_CONTROL__PRIV_STATE__SHIFT;
    (*m).cp_mqd_base_addr_lo = lower_32_bits(addr); (*m).cp_mqd_base_addr_hi = upper_32_bits(addr);
    (*m).cp_hqd_quantum = (1 << CP_HQD_QUANTUM__QUANTUM_EN__SHIFT) | (1 << CP_HQD_QUANTUM__QUANTUM_SCALE__SHIFT) | (1 << CP_HQD_QUANTUM__QUANTUM_DURATION__SHIFT);
    /* Set cp_hqd_hq_scheduler0 bit 14 to 1 to have the CP set up the DISPATCH_PTR. */
    (*m).cp_hqd_hq_scheduler0 = 1 << 14;
    if (*q).format == KFD_QUEUE_FORMAT_AQL { (*m).cp_hqd_aql_control = 1 << CP_HQD_AQL_CONTROL__CONTROL0__SHIFT; }
    if (*mm).dev.kfd.cwsr_enabled {
        (*m).cp_hqd_persistent_state |= 1 << CP_HQD_PERSISTENT_STATE__QSWITCH_MODE__SHIFT;
        (*m).cp_hqd_ctx_save_base_addr_lo = lower_32_bits((*q).ctx_save_restore_area_address);
        (*m).cp_hqd_ctx_save_base_addr_hi = upper_32_bits((*q).ctx_save_restore_area_address);
        (*m).cp_hqd_ctx_save_size = (*q).ctx_save_restore_area_size;
        (*m).cp_hqd_cntl_stack_size = (*q).ctl_stack_size;
        (*m).cp_hqd_cntl_stack_offset = (*q).ctl_stack_size;
        (*m).cp_hqd_wg_state_offset = (*q).ctl_stack_size;
    }
    mutex_lock(&mut (*mm).dev.kfd.profiler_lock);
    if !(*mm).dev.kfd.profiler_process.is_null() { (*m).compute_perfcount_enable = 1; }
    mutex_unlock(&mut (*mm).dev.kfd.profiler_lock);
    *mqd = m as *mut core::ffi::c_void;
    if !gart_addr.is_null() { *gart_addr = addr; }
    ((*mm).update_mqd)(mm, m as *mut core::ffi::c_void, q, core::ptr::null_mut());
}

unsafe fn load_mqd(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, pipe_id: u32, queue_id: u32, p: *mut queue_properties, mms: *mut mm_struct) -> i32 {
    let wptr_shift = if (*p).format == KFD_QUEUE_FORMAT_AQL { 4 } else { 0 };
    ((*mm).dev.kfd2kgd.hqd_load)((*mm).dev.adev, mqd, pipe_id, queue_id, (*p).write_ptr as *mut u32, wptr_shift, 0, mms, 0)
}

unsafe fn update_mqd(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, q: *mut queue_properties, minfo: *mut mqd_update_info) {
    let m = get_mqd(mqd);
    (*m).cp_hqd_pq_control &= !CP_HQD_PQ_CONTROL__QUEUE_SIZE_MASK;
    (*m).cp_hqd_pq_control |= (ffs((*q).queue_size / core::mem::size_of::<u32>() as u64) - 1 - 1) as u32;
    (*m).cp_hqd_pq_base_lo = lower_32_bits((*q).queue_address >> 8); (*m).cp_hqd_pq_base_hi = upper_32_bits((*q).queue_address >> 8);
    (*m).cp_hqd_pq_rptr_report_addr_lo = lower_32_bits((*q).read_ptr as u64); (*m).cp_hqd_pq_rptr_report_addr_hi = upper_32_bits((*q).read_ptr as u64);
    (*m).cp_hqd_pq_wptr_poll_addr_lo = lower_32_bits((*q).write_ptr as u64); (*m).cp_hqd_pq_wptr_poll_addr_hi = upper_32_bits((*q).write_ptr as u64);
    (*m).cp_hqd_pq_doorbell_control = (*q).doorbell_off << CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_OFFSET__SHIFT;
    (*m).cp_hqd_ib_control = 3 << CP_HQD_IB_CONTROL__MIN_IB_AVAIL_SIZE__SHIFT;
    (*m).cp_hqd_eop_control = if (*q).eop_ring_buffer_size != 0 { min(0xA, ffs((*q).eop_ring_buffer_size / core::mem::size_of::<u32>() as u64) - 1 - 1) } else { 0 };
    (*m).cp_hqd_eop_base_addr_lo = lower_32_bits((*q).eop_ring_buffer_address >> 8); (*m).cp_hqd_eop_base_addr_hi = upper_32_bits((*q).eop_ring_buffer_address >> 8);
    (*m).cp_hqd_iq_timer = 0; (*m).cp_hqd_vmid = (*q).vmid;
    if (*q).format == KFD_QUEUE_FORMAT_AQL {
        (*m).cp_hqd_pq_control |= CP_HQD_PQ_CONTROL__NO_UPDATE_RPTR_MASK | (2 << CP_HQD_PQ_CONTROL__SLOT_BASED_WPTR__SHIFT) | (1 << CP_HQD_PQ_CONTROL__QUEUE_FULL_EN__SHIFT);
        (*m).cp_hqd_pq_doorbell_control |= 1 << CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_BIF_DROP__SHIFT;
    }
    if (*mm).dev.kfd.cwsr_enabled { (*m).cp_hqd_ctx_save_control = 0; }
    if !minfo.is_null() { if (*minfo).update_flag == UPDATE_FLAG_PERFCOUNT_ENABLE { (*m).compute_perfcount_enable = 1; } else if (*minfo).update_flag == UPDATE_FLAG_PERFCOUNT_DISABLE { (*m).compute_perfcount_enable = 0; } }
    update_cu_mask(mm, mqd, minfo); set_priority(m, q); (*q).is_active = QUEUE_IS_ACTIVE(*q);
}

unsafe fn check_preemption_failed(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void) -> bool { kfd_check_hiq_mqd_doorbell_id((*mm).dev, (*get_mqd(mqd)).queue_doorbell_id0, 0) }

unsafe fn get_wave_state(_mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, _q: *mut queue_properties, ctl_stack: *mut core::ffi::c_void, ctl_stack_used_size: *mut u32, save_area_used_size: *mut u32) -> i32 {
    let m = get_mqd(mqd); *ctl_stack_used_size = (*m).cp_hqd_cntl_stack_size - (*m).cp_hqd_cntl_stack_offset; *save_area_used_size = (*m).cp_hqd_wg_state_offset - (*m).cp_hqd_cntl_stack_size;
    let mut header: kfd_context_save_area_header = core::mem::zeroed(); header.wave_state.control_stack_size = *ctl_stack_used_size; header.wave_state.wave_state_size = *save_area_used_size; header.wave_state.wave_state_offset = (*m).cp_hqd_wg_state_offset; header.wave_state.control_stack_offset = (*m).cp_hqd_cntl_stack_offset;
    if copy_to_user(ctl_stack, &header.wave_state as *const _, core::mem::size_of::<_>()) != 0 { return -EFAULT; } 0
}

unsafe fn checkpoint_mqd(_mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, mqd_dst: *mut core::ffi::c_void, _ctl_stack_dst: *mut core::ffi::c_void) { core::ptr::copy_nonoverlapping(get_mqd(mqd) as *const u8, mqd_dst as *mut u8, core::mem::size_of::<v10_compute_mqd>()); }

unsafe fn restore_mqd(_mm: *mut mqd_manager, mqd: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, gart_addr: *mut u64, qp: *mut queue_properties, mqd_src: *const core::ffi::c_void, _ctl_stack_src: *const core::ffi::c_void, _ctl_stack_size: u32) {
    let m = (*obj).cpu_ptr as *mut v10_compute_mqd; let addr = (*obj).gpu_addr; core::ptr::copy_nonoverlapping(mqd_src as *const u8, m as *mut u8, core::mem::size_of::<v10_compute_mqd>()); *mqd = m as *mut core::ffi::c_void; if !gart_addr.is_null() { *gart_addr = addr; } (*m).cp_hqd_pq_doorbell_control = (*qp).doorbell_off << CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_OFFSET__SHIFT; (*qp).is_active = 0;
}

unsafe fn init_mqd_hiq(mm: *mut mqd_manager, mqd: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, addr: *mut u64, q: *mut queue_properties) { init_mqd(mm, mqd, obj, addr, q); let m = get_mqd(*mqd); (*m).cp_hqd_pq_control |= 1 << CP_HQD_PQ_CONTROL__PRIV_STATE__SHIFT | 1 << CP_HQD_PQ_CONTROL__KMD_QUEUE__SHIFT; }

unsafe fn destroy_hiq_mqd(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, _type_: kfd_preempt_type, _timeout: u32, _pipe_id: u32, _queue_id: u32) -> i32 { let off = (*get_mqd(mqd)).cp_hqd_pq_doorbell_control >> CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_OFFSET__SHIFT; amdgpu_amdkfd_unmap_hiq((*mm).dev.adev, off, 0) }

unsafe fn init_mqd_sdma(mm: *mut mqd_manager, mqd: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, addr: *mut u64, q: *mut queue_properties) { let m = (*obj).cpu_ptr as *mut v10_sdma_mqd; core::ptr::write_bytes(m as *mut u8, 0, core::mem::size_of::<v10_sdma_mqd>()); *mqd = m as *mut core::ffi::c_void; if !addr.is_null() { *addr = (*obj).gpu_addr; } ((*mm).update_mqd)(mm, m as *mut core::ffi::c_void, q, core::ptr::null_mut()); }

const SDMA_RLC_DUMMY_DEFAULT: u32 = 0xf;

unsafe fn update_mqd_sdma(_mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, q: *mut queue_properties, _minfo: *mut mqd_update_info) { let m = get_sdma_mqd(mqd); (*m).sdmax_rlcx_rb_cntl = ((ffs((*q).queue_size / core::mem::size_of::<u32>() as u64)-1) << SDMA0_RLC0_RB_CNTL__RB_SIZE__SHIFT) | ((*q).vmid << SDMA0_RLC0_RB_CNTL__RB_VMID__SHIFT) | (1 << SDMA0_RLC0_RB_CNTL__RPTR_WRITEBACK_ENABLE__SHIFT) | (6 << SDMA0_RLC0_RB_CNTL__RPTR_WRITEBACK_TIMER__SHIFT); (*m).sdmax_rlcx_rb_base = lower_32_bits((*q).queue_address >> 8); (*m).sdmax_rlcx_rb_base_hi = upper_32_bits((*q).queue_address >> 8); (*m).sdmax_rlcx_rb_rptr_addr_lo = lower_32_bits((*q).read_ptr as u64); (*m).sdmax_rlcx_rb_rptr_addr_hi = upper_32_bits((*q).read_ptr as u64); (*m).sdmax_rlcx_doorbell_offset = (*q).doorbell_off << SDMA0_RLC0_DOORBELL_OFFSET__OFFSET__SHIFT; (*m).sdma_engine_id = (*q).sdma_engine_id; (*m).sdma_queue_id = (*q).sdma_queue_id; (*m).sdmax_rlcx_dummy_reg = SDMA_RLC_DUMMY_DEFAULT; (*q).is_active = QUEUE_IS_ACTIVE(*q); }

unsafe fn checkpoint_mqd_sdma(_mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, dst: *mut core::ffi::c_void, _ctl: *mut core::ffi::c_void) { core::ptr::copy_nonoverlapping(get_sdma_mqd(mqd) as *const u8, dst as *mut u8, core::mem::size_of::<v10_sdma_mqd>()); }
unsafe fn restore_mqd_sdma(_mm: *mut mqd_manager, mqd: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, addr: *mut u64, qp: *mut queue_properties, src: *const core::ffi::c_void, _ctl: *const core::ffi::c_void, _size: u32) { let m = (*obj).cpu_ptr as *mut v10_sdma_mqd; let a = (*obj).gpu_addr; core::ptr::copy_nonoverlapping(src as *const u8, m as *mut u8, core::mem::size_of::<v10_sdma_mqd>()); (*m).sdmax_rlcx_doorbell_offset = (*qp).doorbell_off << SDMA0_RLC0_DOORBELL_OFFSET__OFFSET__SHIFT; *mqd = m as *mut core::ffi::c_void; if !addr.is_null() { *addr = a; } (*qp).is_active = 0; }

// CONFIG_DEBUG_FS conditionally supplies the following debugfs callbacks.

unsafe fn debugfs_show_mqd(m: *mut seq_file, data: *mut core::ffi::c_void) -> i32 { seq_hex_dump(m, "    ", DUMP_PREFIX_OFFSET, 32, 4, data, core::mem::size_of::<v10_compute_mqd>(), false); 0 }
unsafe fn debugfs_show_mqd_sdma(m: *mut seq_file, data: *mut core::ffi::c_void) -> i32 { seq_hex_dump(m, "    ", DUMP_PREFIX_OFFSET, 32, 4, data, core::mem::size_of::<v10_sdma_mqd>(), false); 0 }

unsafe fn mqd_manager_init_v10(type_: KFD_MQD_TYPE, dev: *mut kfd_node) -> *mut mqd_manager {
    if type_ as u32 >= KFD_MQD_TYPE_MAX { return core::ptr::null_mut(); }
    let mqd = kzalloc_obj::<mqd_manager>(); if mqd.is_null() { return core::ptr::null_mut(); } (*mqd).dev = dev;
    match type_ {
        KFD_MQD_TYPE_CP => { (*mqd).allocate_mqd=Some(allocate_mqd); (*mqd).init_mqd=Some(init_mqd); (*mqd).free_mqd=Some(kfd_free_mqd_cp); (*mqd).load_mqd=Some(load_mqd); (*mqd).update_mqd=Some(update_mqd); (*mqd).destroy_mqd=Some(kfd_destroy_mqd_cp); (*mqd).is_occupied=Some(kfd_is_occupied_cp); (*mqd).mqd_size=core::mem::size_of::<v10_compute_mqd>(); (*mqd).get_wave_state=Some(get_wave_state); (*mqd).checkpoint_mqd=Some(checkpoint_mqd); (*mqd).restore_mqd=Some(restore_mqd); (*mqd).mqd_stride=kfd_mqd_stride; (*mqd).debugfs_show_mqd=Some(debugfs_show_mqd); }
        KFD_MQD_TYPE_HIQ => { (*mqd).allocate_mqd=Some(allocate_hiq_mqd); (*mqd).init_mqd=Some(init_mqd_hiq); (*mqd).free_mqd=Some(free_mqd_hiq_sdma); (*mqd).load_mqd=Some(kfd_hiq_load_mqd_kiq); (*mqd).update_mqd=Some(update_mqd); (*mqd).destroy_mqd=Some(destroy_hiq_mqd); (*mqd).is_occupied=Some(kfd_is_occupied_cp); (*mqd).mqd_size=core::mem::size_of::<v10_compute_mqd>(); (*mqd).mqd_stride=kfd_mqd_stride; (*mqd).debugfs_show_mqd=Some(debugfs_show_mqd); (*mqd).check_preemption_failed=Some(check_preemption_failed); }
        KFD_MQD_TYPE_DIQ => { (*mqd).allocate_mqd=Some(allocate_mqd); (*mqd).init_mqd=Some(init_mqd_hiq); (*mqd).free_mqd=Some(kfd_free_mqd_cp); (*mqd).load_mqd=Some(load_mqd); (*mqd).update_mqd=Some(update_mqd); (*mqd).destroy_mqd=Some(kfd_destroy_mqd_cp); (*mqd).is_occupied=Some(kfd_is_occupied_cp); (*mqd).mqd_size=core::mem::size_of::<v10_compute_mqd>(); (*mqd).debugfs_show_mqd=Some(debugfs_show_mqd); }
        KFD_MQD_TYPE_SDMA => { (*mqd).allocate_mqd=Some(allocate_sdma_mqd); (*mqd).init_mqd=Some(init_mqd_sdma); (*mqd).free_mqd=Some(free_mqd_hiq_sdma); (*mqd).load_mqd=Some(kfd_load_mqd_sdma); (*mqd).update_mqd=Some(update_mqd_sdma); (*mqd).destroy_mqd=Some(kfd_destroy_mqd_sdma); (*mqd).is_occupied=Some(kfd_is_occupied_sdma); (*mqd).checkpoint_mqd=Some(checkpoint_mqd_sdma); (*mqd).restore_mqd=Some(restore_mqd_sdma); (*mqd).mqd_size=core::mem::size_of::<v10_sdma_mqd>(); (*mqd).mqd_stride=kfd_mqd_stride; (*mqd).debugfs_show_mqd=Some(debugfs_show_mqd_sdma); }
        _ => { kfree(mqd as *mut core::ffi::c_void); return core::ptr::null_mut(); }
    } mqd
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
