// SPDX-License-Identifier: GPL-2.0 OR MIT
/* C implementation translated literally to Rust; external kernel symbols are supplied by dependencies. */

unsafe fn get_mqd(mqd: *mut core::ffi::c_void) -> *mut cik_mqd {
    mqd as *mut cik_mqd
}

unsafe fn get_sdma_mqd(mqd: *mut core::ffi::c_void) -> *mut cik_sdma_rlc_registers {
    mqd as *mut cik_sdma_rlc_registers
}

unsafe fn update_cu_mask(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, minfo: *mut mqd_update_info) {
    if minfo.is_null() || (*minfo).cu_mask.ptr.is_null() { return; }
    let mut se_mask = [0u32; 4];
    mqd_symmetrically_map_cu_mask(mm, (*minfo).cu_mask.ptr, (*minfo).cu_mask.count, se_mask.as_mut_ptr(), 0);
    let m = get_mqd(mqd);
    (*m).compute_static_thread_mgmt_se0 = se_mask[0];
    (*m).compute_static_thread_mgmt_se1 = se_mask[1];
    (*m).compute_static_thread_mgmt_se2 = se_mask[2];
    (*m).compute_static_thread_mgmt_se3 = se_mask[3];
    pr_debug!("Update cu mask to {:#x} {:#x} {:#x} {:#x}\n", (*m).compute_static_thread_mgmt_se0, (*m).compute_static_thread_mgmt_se1, (*m).compute_static_thread_mgmt_se2, (*m).compute_static_thread_mgmt_se3);
}

unsafe fn set_priority(m: *mut cik_mqd, q: *mut queue_properties) {
    (*m).cp_hqd_pipe_priority = pipe_priority_map[(*q).priority as usize];
}

unsafe fn allocate_mqd(mm: *mut mqd_manager, _q: *mut queue_properties) -> *mut kfd_mem_obj {
    let kfd = (*mm).dev;
    let mut obj: *mut kfd_mem_obj = core::ptr::null_mut();
    if kfd_gtt_sa_allocate(kfd, core::mem::size_of::<cik_mqd>(), &mut obj) != 0 { core::ptr::null_mut() } else { obj }
}

unsafe fn init_mqd(mm: *mut mqd_manager, mqd: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, gart_addr: *mut u64, q: *mut queue_properties) {
    let m = (*obj).cpu_ptr as *mut cik_mqd;
    let addr = (*obj).gpu_addr;
    core::ptr::write_bytes(m as *mut u8, 0, (core::mem::size_of::<cik_mqd>() + 255) & !255);
    (*m).header = 0xC0310800;
    (*m).compute_pipelinestat_enable = 1;
    (*m).compute_static_thread_mgmt_se0 = u32::MAX; (*m).compute_static_thread_mgmt_se1 = u32::MAX;
    (*m).compute_static_thread_mgmt_se2 = u32::MAX; (*m).compute_static_thread_mgmt_se3 = u32::MAX;
    (*m).cp_hqd_persistent_state = DEFAULT_CP_HQD_PERSISTENT_STATE | PRELOAD_REQ;
    (*m).cp_mqd_control = MQD_CONTROL_PRIV_STATE_EN;
    (*m).cp_mqd_base_addr_lo = lower_32_bits(addr); (*m).cp_mqd_base_addr_hi = upper_32_bits(addr);
    (*m).cp_hqd_quantum = QUANTUM_EN | QUANTUM_SCALE_1MS | QUANTUM_DURATION(10);
    set_priority(m, q);
    if (*q).format == KFD_QUEUE_FORMAT_AQL { (*m).cp_hqd_iq_rptr = AQL_ENABLE; }
    *mqd = m as *mut core::ffi::c_void; if !gart_addr.is_null() { *gart_addr = addr; }
    ((*mm).update_mqd)(mm, m as *mut core::ffi::c_void, q, core::ptr::null_mut());
}

unsafe fn init_mqd_sdma(mm: *mut mqd_manager, mqd: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, gart_addr: *mut u64, q: *mut queue_properties) {
    let m = (*obj).cpu_ptr as *mut cik_sdma_rlc_registers;
    core::ptr::write_bytes(m as *mut u8, 0, core::mem::size_of::<cik_sdma_rlc_registers>());
    *mqd = m as *mut core::ffi::c_void; if !gart_addr.is_null() { *gart_addr = (*obj).gpu_addr; }
    ((*mm).update_mqd)(mm, m as *mut core::ffi::c_void, q, core::ptr::null_mut());
}

unsafe fn load_mqd(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, pipe_id: u32, queue_id: u32, p: *mut queue_properties, mms: *mut mm_struct) -> i32 {
    let shift = if (*p).format == KFD_QUEUE_FORMAT_AQL { 4 } else { 0 };
    let mask = ((*p).queue_size / 4 - 1) as u32;
    ((*(*mm).dev).kfd2kgd).hqd_load((*(*mm).dev).adev, mqd, pipe_id, queue_id, (*p).write_ptr as *mut u32, shift, mask, mms, 0)
}

unsafe fn __update_mqd(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, q: *mut queue_properties, minfo: *mut mqd_update_info, atc_bit: u32) {
    let m = get_mqd(mqd);
    (*m).cp_hqd_pq_control = DEFAULT_RPTR_BLOCK_SIZE | DEFAULT_MIN_AVAIL_SIZE;
    (*m).cp_hqd_ib_control = DEFAULT_MIN_IB_AVAIL_SIZE;
    if atc_bit != 0 { (*m).cp_hqd_pq_control |= PQ_ATC_EN; (*m).cp_hqd_ib_control |= IB_ATC_EN; }
    (*m).cp_hqd_pq_control |= order_base_2((*q).queue_size / 4) - 1;
    (*m).cp_hqd_pq_base_lo = lower_32_bits((*q).queue_address >> 8); (*m).cp_hqd_pq_base_hi = upper_32_bits((*q).queue_address >> 8);
    (*m).cp_hqd_pq_rptr_report_addr_lo = lower_32_bits((*q).read_ptr as u64); (*m).cp_hqd_pq_rptr_report_addr_hi = upper_32_bits((*q).read_ptr as u64);
    (*m).cp_hqd_pq_doorbell_control = DOORBELL_OFFSET((*q).doorbell_off); (*m).cp_hqd_vmid = (*q).vmid;
    if (*q).format == KFD_QUEUE_FORMAT_AQL { (*m).cp_hqd_pq_control |= NO_UPDATE_RPTR; }
    update_cu_mask(mm, mqd, minfo); set_priority(m, q); (*q).is_active = QUEUE_IS_ACTIVE(*q);
}

unsafe fn update_mqd(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, q: *mut queue_properties, minfo: *mut mqd_update_info) { __update_mqd(mm, mqd, q, minfo, 0); }

unsafe fn check_preemption_failed(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void) -> bool { let m = get_mqd(mqd); kfd_check_hiq_mqd_doorbell_id((*mm).dev, (*m).queue_doorbell_id0, 0) }

unsafe fn update_mqd_sdma(_mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, q: *mut queue_properties, _minfo: *mut mqd_update_info) {
    let m = get_sdma_mqd(mqd);
    (*m).sdma_rlc_rb_cntl = (order_base_2((*q).queue_size / 4) << SDMA0_RLC0_RB_CNTL__RB_SIZE__SHIFT) | ((*q).vmid << SDMA0_RLC0_RB_CNTL__RB_VMID__SHIFT) | (1 << SDMA0_RLC0_RB_CNTL__RPTR_WRITEBACK_ENABLE__SHIFT) | (6 << SDMA0_RLC0_RB_CNTL__RPTR_WRITEBACK_TIMER__SHIFT);
    (*m).sdma_rlc_rb_base = lower_32_bits((*q).queue_address >> 8); (*m).sdma_rlc_rb_base_hi = upper_32_bits((*q).queue_address >> 8);
    (*m).sdma_rlc_rb_rptr_addr_lo = lower_32_bits((*q).read_ptr as u64); (*m).sdma_rlc_rb_rptr_addr_hi = upper_32_bits((*q).read_ptr as u64);
    (*m).sdma_rlc_doorbell = (*q).doorbell_off << SDMA0_RLC0_DOORBELL__OFFSET__SHIFT; (*m).sdma_rlc_virtual_addr = (*q).sdma_vm_addr; (*m).sdma_engine_id = (*q).sdma_engine_id; (*m).sdma_queue_id = (*q).sdma_queue_id; (*q).is_active = QUEUE_IS_ACTIVE(*q);
}

unsafe fn checkpoint_mqd(_mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, dst: *mut core::ffi::c_void, _ctl: *mut core::ffi::c_void) { core::ptr::copy_nonoverlapping(get_mqd(mqd) as *const u8, dst as *mut u8, core::mem::size_of::<cik_mqd>()); }

unsafe fn restore_mqd(_mm: *mut mqd_manager, mqd: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, gart: *mut u64, qp: *mut queue_properties, src: *const core::ffi::c_void, _ctl: *const core::ffi::c_void, _size: u32) {
    let m = (*obj).cpu_ptr as *mut cik_mqd; core::ptr::copy_nonoverlapping(src as *const u8, m as *mut u8, core::mem::size_of::<cik_mqd>()); *mqd = m as *mut core::ffi::c_void; if !gart.is_null() { *gart = (*obj).gpu_addr; } (*m).cp_hqd_pq_doorbell_control = DOORBELL_OFFSET((*qp).doorbell_off); (*qp).is_active = 0;
}

unsafe fn checkpoint_mqd_sdma(_mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, dst: *mut core::ffi::c_void, _ctl: *mut core::ffi::c_void) { core::ptr::copy_nonoverlapping(get_sdma_mqd(mqd) as *const u8, dst as *mut u8, core::mem::size_of::<cik_sdma_rlc_registers>()); }

unsafe fn restore_mqd_sdma(_mm: *mut mqd_manager, mqd: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, gart: *mut u64, qp: *mut queue_properties, src: *const core::ffi::c_void, _ctl: *const core::ffi::c_void, _size: u32) {
    let m = (*obj).cpu_ptr as *mut cik_sdma_rlc_registers; core::ptr::copy_nonoverlapping(src as *const u8, m as *mut u8, core::mem::size_of::<cik_sdma_rlc_registers>()); (*m).sdma_rlc_doorbell = (*qp).doorbell_off << SDMA0_RLC0_DOORBELL__OFFSET__SHIFT; *mqd = m as *mut core::ffi::c_void; if !gart.is_null() { *gart = (*obj).gpu_addr; } (*qp).is_active = 0;
}

// HIQ MQD uses the same MQD structure as user queues, with different initial values.
unsafe fn init_mqd_hiq(mm: *mut mqd_manager, mqd: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, gart: *mut u64, q: *mut queue_properties) { init_mqd(mm, mqd, obj, gart, q); }

unsafe fn update_mqd_hiq(_mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, q: *mut queue_properties, _minfo: *mut mqd_update_info) {
    let m = get_mqd(mqd); (*m).cp_hqd_pq_control = DEFAULT_RPTR_BLOCK_SIZE | DEFAULT_MIN_AVAIL_SIZE | PRIV_STATE | KMD_QUEUE; (*m).cp_hqd_pq_control |= order_base_2((*q).queue_size / 4) - 1; (*m).cp_hqd_pq_base_lo = lower_32_bits((*q).queue_address >> 8); (*m).cp_hqd_pq_base_hi = upper_32_bits((*q).queue_address >> 8); (*m).cp_hqd_pq_rptr_report_addr_lo = lower_32_bits((*q).read_ptr as u64); (*m).cp_hqd_pq_rptr_report_addr_hi = upper_32_bits((*q).read_ptr as u64); (*m).cp_hqd_pq_doorbell_control = DOORBELL_OFFSET((*q).doorbell_off); (*m).cp_hqd_vmid = (*q).vmid; (*q).is_active = QUEUE_IS_ACTIVE(*q); set_priority(m, q);
}

unsafe fn mqd_manager_init_cik(typ: KFD_MQD_TYPE, dev: *mut kfd_node) -> *mut mqd_manager {
    if typ >= KFD_MQD_TYPE_MAX { return core::ptr::null_mut(); }
    let mqd = kzalloc_obj::<mqd_manager>(); if mqd.is_null() { return core::ptr::null_mut(); } (*mqd).dev = dev;
    match typ {
        KFD_MQD_TYPE_CP => { (*mqd).allocate_mqd = Some(allocate_mqd); (*mqd).init_mqd = Some(init_mqd); (*mqd).free_mqd = Some(kfd_free_mqd_cp); (*mqd).load_mqd = Some(load_mqd); (*mqd).update_mqd = Some(update_mqd); (*mqd).destroy_mqd = Some(kfd_destroy_mqd_cp); (*mqd).is_occupied = Some(kfd_is_occupied_cp); (*mqd).checkpoint_mqd = Some(checkpoint_mqd); (*mqd).restore_mqd = Some(restore_mqd); (*mqd).mqd_size = core::mem::size_of::<cik_mqd>(); }
        KFD_MQD_TYPE_HIQ | KFD_MQD_TYPE_DIQ => { (*mqd).allocate_mqd = Some(allocate_mqd); (*mqd).init_mqd = Some(init_mqd_hiq); (*mqd).load_mqd = Some(load_mqd); (*mqd).update_mqd = Some(update_mqd_hiq); (*mqd).mqd_size = core::mem::size_of::<cik_mqd>(); (*mqd).check_preemption_failed = Some(check_preemption_failed); }
        KFD_MQD_TYPE_SDMA => { (*mqd).allocate_mqd = Some(allocate_sdma_mqd); (*mqd).init_mqd = Some(init_mqd_sdma); (*mqd).load_mqd = Some(kfd_load_mqd_sdma); (*mqd).update_mqd = Some(update_mqd_sdma); (*mqd).checkpoint_mqd = Some(checkpoint_mqd_sdma); (*mqd).restore_mqd = Some(restore_mqd_sdma); (*mqd).mqd_size = core::mem::size_of::<cik_sdma_rlc_registers>(); }
        _ => { kfree(mqd as *mut core::ffi::c_void); return core::ptr::null_mut(); }
    }
    mqd
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
