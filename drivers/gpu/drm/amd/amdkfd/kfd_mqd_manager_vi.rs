// SPDX-License-Identifier: GPL-2.0 OR MIT
/* Direct translation of kfd_mqd_manager_vi.c. */

const CP_MQD_CONTROL_PRIV_STATE_SHIFT: u32 = 0x8;

#[inline]
unsafe fn get_mqd(mqd: *mut core::ffi::c_void) -> *mut vi_mqd { mqd as *mut vi_mqd }

#[inline]
unsafe fn get_sdma_mqd(mqd: *mut core::ffi::c_void) -> *mut vi_sdma_mqd { mqd as *mut vi_sdma_mqd }

unsafe fn update_cu_mask(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, minfo: *mut mqd_update_info) {
    if minfo.is_null() || (*minfo).cu_mask.ptr.is_null() { return; }
    let mut se_mask = [0u32; 4];
    mqd_symmetrically_map_cu_mask(mm, (*minfo).cu_mask.ptr, (*minfo).cu_mask.count, se_mask.as_mut_ptr(), 0);
    let m = get_mqd(mqd);
    (*m).compute_static_thread_mgmt_se0 = se_mask[0];
    (*m).compute_static_thread_mgmt_se1 = se_mask[1];
    (*m).compute_static_thread_mgmt_se2 = se_mask[2];
    (*m).compute_static_thread_mgmt_se3 = se_mask[3];
    pr_debug!("Update cu mask to %#x %#x %#x %#x\n", (*m).compute_static_thread_mgmt_se0, (*m).compute_static_thread_mgmt_se1, (*m).compute_static_thread_mgmt_se2, (*m).compute_static_thread_mgmt_se3);
}

unsafe fn set_priority(m: *mut vi_mqd, q: *mut queue_properties) { (*m).cp_hqd_pipe_priority = pipe_priority_map[(*q).priority as usize]; }

unsafe fn allocate_mqd(mm: *mut mqd_manager, _q: *mut queue_properties) -> *mut kfd_mem_obj {
    let mut obj: *mut kfd_mem_obj = core::ptr::null_mut();
    if kfd_gtt_sa_allocate((*mm).dev, core::mem::size_of::<vi_mqd>(), &mut obj) != 0 { core::ptr::null_mut() } else { obj }
}

unsafe fn init_mqd(mm: *mut mqd_manager, mqd: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, gart_addr: *mut u64, q: *mut queue_properties) {
    let m = (*obj).cpu_ptr as *mut vi_mqd;
    let addr = (*obj).gpu_addr;
    core::ptr::write_bytes(m as *mut u8, 0, core::mem::size_of::<vi_mqd>());
    (*m).header = 0xC0310800; (*m).compute_pipelinestat_enable = 1;
    (*m).compute_static_thread_mgmt_se0 = u32::MAX; (*m).compute_static_thread_mgmt_se1 = u32::MAX; (*m).compute_static_thread_mgmt_se2 = u32::MAX; (*m).compute_static_thread_mgmt_se3 = u32::MAX;
    (*m).cp_hqd_persistent_state = CP_HQD_PERSISTENT_STATE_PRELOAD_REQ_MASK | (0x53 << CP_HQD_PERSISTENT_STATE_PRELOAD_SIZE_SHIFT);
    (*m).cp_mqd_control = (1 << CP_MQD_CONTROL_PRIV_STATE_SHIFT) | (MTYPE_UC << CP_MQD_CONTROL_MTYPE_SHIFT);
    (*m).cp_mqd_base_addr_lo = lower_32_bits(addr); (*m).cp_mqd_base_addr_hi = upper_32_bits(addr);
    (*m).cp_hqd_quantum = (1 << CP_HQD_QUANTUM_QUANTUM_EN_SHIFT) | (1 << CP_HQD_QUANTUM_QUANTUM_SCALE_SHIFT) | (1 << CP_HQD_QUANTUM_QUANTUM_DURATION_SHIFT);
    set_priority(m, q); (*m).cp_hqd_eop_rptr = 1 << CP_HQD_EOP_RPTR_INIT_FETCHER_SHIFT;
    if (*q).format == KFD_QUEUE_FORMAT_AQL { (*m).cp_hqd_iq_rptr = 1; }
    if (*q).tba_addr != 0 { let a = (*q).tba_addr >> 8; (*m).compute_tba_lo = lower_32_bits(a); (*m).compute_tba_hi = upper_32_bits(a); let a = (*q).tma_addr >> 8; (*m).compute_tma_lo = lower_32_bits(a); (*m).compute_tma_hi = upper_32_bits(a); (*m).compute_pgm_rsrc2 |= 1 << COMPUTE_PGM_RSRC2_TRAP_PRESENT_SHIFT; }
    if (*(*mm).dev).kfd.cwsr_enabled && (*q).ctx_save_restore_area_address != 0 { (*m).cp_hqd_persistent_state |= 1 << CP_HQD_PERSISTENT_STATE_QSWITCH_MODE_SHIFT; (*m).cp_hqd_ctx_save_base_addr_lo = lower_32_bits((*q).ctx_save_restore_area_address); (*m).cp_hqd_ctx_save_base_addr_hi = upper_32_bits((*q).ctx_save_restore_area_address); (*m).cp_hqd_ctx_save_size = (*q).ctx_save_restore_area_size; (*m).cp_hqd_cntl_stack_size = (*q).ctl_stack_size; (*m).cp_hqd_cntl_stack_offset = (*q).ctl_stack_size; (*m).cp_hqd_wg_state_offset = (*q).ctl_stack_size; }
    mutex_lock(&(*(*mm).dev).kfd.profiler_lock); if !(*(*mm).dev).kfd.profiler_process.is_null() { (*m).compute_perfcount_enable = 1; } mutex_unlock(&(*(*mm).dev).kfd.profiler_lock);
    *mqd = m as *mut core::ffi::c_void; if !gart_addr.is_null() { *gart_addr = addr; } ((*mm).update_mqd)(mm, m as *mut _, q, core::ptr::null_mut());
}

unsafe fn load_mqd(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, pipe_id: u32, queue_id: u32, p: *mut queue_properties, mms: *mut mm_struct) -> i32 {
    let shift = if (*p).format == KFD_QUEUE_FORMAT_AQL { 4 } else { 0 }; let mask = ((*p).queue_size / 4) - 1;
    ((*(*mm).dev).kfd2kgd.hqd_load)((*(*mm).dev).adev, mqd, pipe_id, queue_id, (*p).write_ptr as *mut u32, shift, mask, mms, 0)
}

unsafe fn __update_mqd(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, q: *mut queue_properties, minfo: *mut mqd_update_info, mtype: u32, atc_bit: u32) {
    let m = get_mqd(mqd); (*m).cp_hqd_pq_control = (5 << CP_HQD_PQ_CONTROL_RPTR_BLOCK_SIZE_SHIFT) | (atc_bit << CP_HQD_PQ_CONTROL_PQ_ATC_SHIFT) | (mtype << CP_HQD_PQ_CONTROL_MTYPE_SHIFT); (*m).cp_hqd_pq_control |= order_base_2((*q).queue_size / 4) - 1;
    (*m).cp_hqd_pq_base_lo = lower_32_bits((*q).queue_address >> 8); (*m).cp_hqd_pq_base_hi = upper_32_bits((*q).queue_address >> 8); (*m).cp_hqd_pq_rptr_report_addr_lo = lower_32_bits((*q).read_ptr as u64); (*m).cp_hqd_pq_rptr_report_addr_hi = upper_32_bits((*q).read_ptr as u64); (*m).cp_hqd_pq_wptr_poll_addr_lo = lower_32_bits((*q).write_ptr as u64); (*m).cp_hqd_pq_wptr_poll_addr_hi = upper_32_bits((*q).write_ptr as u64);
    (*m).cp_hqd_pq_doorbell_control = (*q).doorbell_off << CP_HQD_PQ_DOORBELL_CONTROL_DOORBELL_OFFSET_SHIFT; (*m).cp_hqd_eop_control = (atc_bit << CP_HQD_EOP_CONTROL_EOP_ATC_SHIFT) | (mtype << CP_HQD_EOP_CONTROL_MTYPE_SHIFT); (*m).cp_hqd_ib_control = (atc_bit << CP_HQD_IB_CONTROL_IB_ATC_SHIFT) | (3 << CP_HQD_IB_CONTROL_MIN_IB_AVAIL_SIZE_SHIFT) | (mtype << CP_HQD_IB_CONTROL_MTYPE_SHIFT);
    (*m).cp_hqd_eop_control |= if (*q).eop_ring_buffer_size != 0 { core::cmp::min(0xA, order_base_2((*q).eop_ring_buffer_size / 4) - 1) } else { 0 }; (*m).cp_hqd_eop_base_addr_lo = lower_32_bits((*q).eop_ring_buffer_address >> 8); (*m).cp_hqd_eop_base_addr_hi = upper_32_bits((*q).eop_ring_buffer_address >> 8); (*m).cp_hqd_iq_timer = (atc_bit << CP_HQD_IQ_TIMER_IQ_ATC_SHIFT) | (mtype << CP_HQD_IQ_TIMER_MTYPE_SHIFT); (*m).cp_hqd_vmid = (*q).vmid;
    if (*q).format == KFD_QUEUE_FORMAT_AQL { (*m).cp_hqd_pq_control |= CP_HQD_PQ_CONTROL_NO_UPDATE_RPTR_MASK | (2 << CP_HQD_PQ_CONTROL_SLOT_BASED_WPTR_SHIFT); }
    if (*(*mm).dev).kfd.cwsr_enabled && (*q).ctx_save_restore_area_address != 0 { (*m).cp_hqd_ctx_save_control = (atc_bit << CP_HQD_CTX_SAVE_CONTROL_ATC_SHIFT) | (mtype << CP_HQD_CTX_SAVE_CONTROL_MTYPE_SHIFT); }
    if !minfo.is_null() { if (*minfo).update_flag == UPDATE_FLAG_PERFCOUNT_ENABLE { (*m).compute_perfcount_enable = 1; } else if (*minfo).update_flag == UPDATE_FLAG_PERFCOUNT_DISABLE { (*m).compute_perfcount_enable = 0; } } update_cu_mask(mm, mqd, minfo); set_priority(m, q); (*q).is_active = QUEUE_IS_ACTIVE(*q);
}

unsafe fn check_preemption_failed(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void) -> bool { kfd_check_hiq_mqd_doorbell_id((*mm).dev, (*get_mqd(mqd)).queue_doorbell_id0, 0) }
unsafe fn update_mqd(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, q: *mut queue_properties, i: *mut mqd_update_info) { __update_mqd(mm, mqd, q, i, MTYPE_UC, 0); }

unsafe fn get_wave_state(_mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, _q: *mut queue_properties, _ctl: *mut core::ffi::c_void, used: *mut u32, save: *mut u32) -> i32 { let m=get_mqd(mqd); *used=(*m).cp_hqd_cntl_stack_size-(*m).cp_hqd_cntl_stack_offset; *save=(*m).cp_hqd_wg_state_offset-(*m).cp_hqd_cntl_stack_size; 0 }
unsafe fn get_checkpoint_info(_mm:*mut mqd_manager,_mqd:*mut core::ffi::c_void,size:*mut u32)->i32{*size=0;0}
unsafe fn checkpoint_mqd(_mm:*mut mqd_manager,mqd:*mut core::ffi::c_void,dst:*mut core::ffi::c_void,_ctl:*mut core::ffi::c_void){core::ptr::copy_nonoverlapping(mqd as *const u8,dst as *mut u8,core::mem::size_of::<vi_mqd>());}
unsafe fn restore_mqd(_mm:*mut mqd_manager,mqd:*mut *mut core::ffi::c_void,obj:*mut kfd_mem_obj,gart:*mut u64,qp:*mut queue_properties,src:*const core::ffi::c_void,_ctl:*const core::ffi::c_void,_size:u32){let m=(*obj).cpu_ptr as *mut vi_mqd;core::ptr::copy_nonoverlapping(src as *const u8,m as *mut u8,core::mem::size_of::<vi_mqd>());*mqd=m as *mut _;if !gart.is_null(){*gart=(*obj).gpu_addr;}(*m).cp_hqd_pq_doorbell_control=(*qp).doorbell_off<<CP_HQD_PQ_DOORBELL_CONTROL_DOORBELL_OFFSET_SHIFT;(*qp).is_active=0;}

unsafe fn init_mqd_hiq(mm:*mut mqd_manager,mqd:*mut *mut core::ffi::c_void,obj:*mut kfd_mem_obj,gart:*mut u64,q:*mut queue_properties){init_mqd(mm,mqd,obj,gart,q);let m=get_mqd(*mqd);(*m).cp_hqd_pq_control|=(1<<CP_HQD_PQ_CONTROL_PRIV_STATE_SHIFT)|(1<<CP_HQD_PQ_CONTROL_KMD_QUEUE_SHIFT);}
unsafe fn update_mqd_hiq(mm:*mut mqd_manager,mqd:*mut core::ffi::c_void,q:*mut queue_properties,i:*mut mqd_update_info){__update_mqd(mm,mqd,q,i,MTYPE_UC,0);}

unsafe fn init_mqd_sdma(mm:*mut mqd_manager,mqd:*mut *mut core::ffi::c_void,obj:*mut kfd_mem_obj,gart:*mut u64,q:*mut queue_properties){let m=(*obj).cpu_ptr as *mut vi_sdma_mqd;core::ptr::write_bytes(m as *mut u8,0,core::mem::size_of::<vi_sdma_mqd>());*mqd=m as *mut _;if !gart.is_null(){*gart=(*obj).gpu_addr;}((*mm).update_mqd)(mm,m as *mut _,q,core::ptr::null_mut());}
unsafe fn update_mqd_sdma(_mm:*mut mqd_manager,mqd:*mut core::ffi::c_void,q:*mut queue_properties,_i:*mut mqd_update_info){let m=get_sdma_mqd(mqd);(*m).sdmax_rlcx_rb_cntl=(order_base_2((*q).queue_size/4)<<SDMA0_RLC0_RB_CNTL_RB_SIZE_SHIFT)|((*q).vmid<<SDMA0_RLC0_RB_CNTL_RB_VMID_SHIFT)|(1<<SDMA0_RLC0_RB_CNTL_RPTR_WRITEBACK_ENABLE_SHIFT)|(6<<SDMA0_RLC0_RB_CNTL_RPTR_WRITEBACK_TIMER_SHIFT);(*m).sdmax_rlcx_rb_base=lower_32_bits((*q).queue_address>>8);(*m).sdmax_rlcx_rb_base_hi=upper_32_bits((*q).queue_address>>8);(*m).sdmax_rlcx_rb_rptr_addr_lo=lower_32_bits((*q).read_ptr as u64);(*m).sdmax_rlcx_rb_rptr_addr_hi=upper_32_bits((*q).read_ptr as u64);(*m).sdmax_rlcx_doorbell=(*q).doorbell_off<<SDMA0_RLC0_DOORBELL_OFFSET_SHIFT;(*m).sdmax_rlcx_virtual_addr=(*q).sdma_vm_addr;(*m).sdma_engine_id=(*q).sdma_engine_id;(*m).sdma_queue_id=(*q).sdma_queue_id;(*q).is_active=QUEUE_IS_ACTIVE(*q);}
unsafe fn checkpoint_mqd_sdma(_mm:*mut mqd_manager,mqd:*mut core::ffi::c_void,dst:*mut core::ffi::c_void,_ctl:*mut core::ffi::c_void){core::ptr::copy_nonoverlapping(mqd as *const u8,dst as *mut u8,core::mem::size_of::<vi_sdma_mqd>());}
unsafe fn restore_mqd_sdma(_mm:*mut mqd_manager,mqd:*mut *mut core::ffi::c_void,obj:*mut kfd_mem_obj,gart:*mut u64,qp:*mut queue_properties,src:*const core::ffi::c_void,_ctl:*const core::ffi::c_void,_size:u32){let m=(*obj).cpu_ptr as *mut vi_sdma_mqd;core::ptr::copy_nonoverlapping(src as *const u8,m as *mut u8,core::mem::size_of::<vi_sdma_mqd>());(*m).sdmax_rlcx_doorbell=(*qp).doorbell_off<<SDMA0_RLC0_DOORBELL_OFFSET_SHIFT;*mqd=m as *mut _;if !gart.is_null(){*gart=(*obj).gpu_addr;}(*qp).is_active=0;}

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe fn debugfs_show_mqd(m:*mut seq_file,data:*mut core::ffi::c_void)->i32{seq_hex_dump(m,"    ",DUMP_PREFIX_OFFSET,32,4,data,core::mem::size_of::<vi_mqd>(),false);0}
#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe fn debugfs_show_mqd_sdma(m:*mut seq_file,data:*mut core::ffi::c_void)->i32{seq_hex_dump(m,"    ",DUMP_PREFIX_OFFSET,32,4,data,core::mem::size_of::<vi_sdma_mqd>(),false);0}

unsafe fn mqd_manager_init_vi(ty: KFD_MQD_TYPE, dev: *mut kfd_node) -> *mut mqd_manager {
    if ty as u32 >= KFD_MQD_TYPE_MAX as u32 { return core::ptr::null_mut(); }
    let mqd = kzalloc_obj::<mqd_manager>(); if mqd.is_null() { return core::ptr::null_mut(); } (*mqd).dev=dev;
    match ty {
        KFD_MQD_TYPE_CP => { (*mqd).allocate_mqd=Some(allocate_mqd);(*mqd).init_mqd=Some(init_mqd);(*mqd).free_mqd=Some(kfd_free_mqd_cp);(*mqd).load_mqd=Some(load_mqd);(*mqd).update_mqd=Some(update_mqd);(*mqd).destroy_mqd=Some(kfd_destroy_mqd_cp);(*mqd).is_occupied=Some(kfd_is_occupied_cp);(*mqd).get_wave_state=Some(get_wave_state);(*mqd).get_checkpoint_info=Some(get_checkpoint_info);(*mqd).checkpoint_mqd=Some(checkpoint_mqd);(*mqd).restore_mqd=Some(restore_mqd);(*mqd).mqd_size=core::mem::size_of::<vi_mqd>();#[cfg(feature="CONFIG_DEBUG_FS")] {(*mqd).debugfs_show_mqd=Some(debugfs_show_mqd);}}
        KFD_MQD_TYPE_HIQ | KFD_MQD_TYPE_DIQ => {(*mqd).allocate_mqd=Some(allocate_mqd);(*mqd).init_mqd=Some(init_mqd_hiq);(*mqd).free_mqd=Some(free_mqd_hiq_sdma);(*mqd).load_mqd=Some(load_mqd);(*mqd).update_mqd=Some(update_mqd_hiq);(*mqd).destroy_mqd=Some(kfd_destroy_mqd_cp);(*mqd).is_occupied=Some(kfd_is_occupied_cp);(*mqd).mqd_size=core::mem::size_of::<vi_mqd>();(*mqd).mqd_stride=kfd_mqd_stride;#[cfg(feature="CONFIG_DEBUG_FS")] {(*mqd).debugfs_show_mqd=Some(debugfs_show_mqd);} if ty==KFD_MQD_TYPE_HIQ {(*mqd).check_preemption_failed=Some(check_preemption_failed);}}
        KFD_MQD_TYPE_SDMA => {(*mqd).allocate_mqd=Some(allocate_sdma_mqd);(*mqd).init_mqd=Some(init_mqd_sdma);(*mqd).free_mqd=Some(free_mqd_hiq_sdma);(*mqd).load_mqd=Some(kfd_load_mqd_sdma);(*mqd).update_mqd=Some(update_mqd_sdma);(*mqd).destroy_mqd=Some(kfd_destroy_mqd_sdma);(*mqd).is_occupied=Some(kfd_is_occupied_sdma);(*mqd).checkpoint_mqd=Some(checkpoint_mqd_sdma);(*mqd).restore_mqd=Some(restore_mqd_sdma);(*mqd).mqd_size=core::mem::size_of::<vi_sdma_mqd>();(*mqd).mqd_stride=kfd_mqd_stride;#[cfg(feature="CONFIG_DEBUG_FS")] {(*mqd).debugfs_show_mqd=Some(debugfs_show_mqd_sdma);}}
        _ => { kfree(mqd as *mut core::ffi::c_void); return core::ptr::null_mut(); }
    } mqd
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
