/* Translated from kfd_mqd_manager_v11.c. */

#[inline]
unsafe fn get_mqd(mqd: *mut core::ffi::c_void) -> *mut v11_compute_mqd { mqd as *mut v11_compute_mqd }
#[inline]
unsafe fn get_sdma_mqd(mqd: *mut core::ffi::c_void) -> *mut v11_sdma_mqd { mqd as *mut v11_sdma_mqd }

unsafe fn update_cu_mask(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, minfo: *mut mqd_update_info) {
    let mut se_mask = [0u32; KFD_MAX_NUM_SE as usize];
    let has_wa_flag = !minfo.is_null() && ((*minfo).update_flag & (UPDATE_FLAG_DBG_WA_ENABLE | UPDATE_FLAG_DBG_WA_DISABLE)) != 0;
    if minfo.is_null() || !(has_wa_flag || (*minfo).cu_mask.ptr != 0) { return; }
    let m = get_mqd(mqd);
    if has_wa_flag {
        let wa_mask = if (*minfo).update_flag & UPDATE_FLAG_DBG_WA_ENABLE != 0 { 0xffff } else { 0xffffffff };
        (*m).compute_static_thread_mgmt_se0 = wa_mask; (*m).compute_static_thread_mgmt_se1 = wa_mask;
        (*m).compute_static_thread_mgmt_se2 = wa_mask; (*m).compute_static_thread_mgmt_se3 = wa_mask;
        (*m).compute_static_thread_mgmt_se4 = wa_mask; (*m).compute_static_thread_mgmt_se5 = wa_mask;
        (*m).compute_static_thread_mgmt_se6 = wa_mask; (*m).compute_static_thread_mgmt_se7 = wa_mask;
        return;
    }
    mqd_symmetrically_map_cu_mask(mm, (*minfo).cu_mask.ptr, (*minfo).cu_mask.count, se_mask.as_mut_ptr(), 0);
    (*m).compute_static_thread_mgmt_se0 = se_mask[0]; (*m).compute_static_thread_mgmt_se1 = se_mask[1];
    (*m).compute_static_thread_mgmt_se2 = se_mask[2]; (*m).compute_static_thread_mgmt_se3 = se_mask[3];
    (*m).compute_static_thread_mgmt_se4 = se_mask[4]; (*m).compute_static_thread_mgmt_se5 = se_mask[5];
    (*m).compute_static_thread_mgmt_se6 = se_mask[6]; (*m).compute_static_thread_mgmt_se7 = se_mask[7];
    pr_debug!("update cu mask to %#x %#x %#x %#x %#x %#x %#x %#x\n", (*m).compute_static_thread_mgmt_se0, (*m).compute_static_thread_mgmt_se1, (*m).compute_static_thread_mgmt_se2, (*m).compute_static_thread_mgmt_se3, (*m).compute_static_thread_mgmt_se4, (*m).compute_static_thread_mgmt_se5, (*m).compute_static_thread_mgmt_se6, (*m).compute_static_thread_mgmt_se7);
}

unsafe fn set_priority(m: *mut v11_compute_mqd, q: *mut queue_properties) { (*m).cp_hqd_pipe_priority = pipe_priority_map[(*q).priority as usize]; }

unsafe fn allocate_mqd(mm: *mut mqd_manager, _q: *mut queue_properties) -> *mut kfd_mem_obj {
    let size = AMDGPU_MQD_SIZE_ALIGN((*mm).mqd_size); let mut obj: *mut kfd_mem_obj = core::ptr::null_mut();
    if kfd_gtt_sa_allocate((*mm).dev, size, &mut obj) != 0 { core::ptr::null_mut() } else { obj }
}

unsafe fn init_mqd(mm: *mut mqd_manager, mqd: *mut *mut core::ffi::c_void, obj: *mut kfd_mem_obj, gart: *mut u64, q: *mut queue_properties) {
    let m = (*obj).cpu_ptr as *mut v11_compute_mqd; let addr = (*obj).gpu_addr; let size = AMDGPU_MQD_SIZE_ALIGN((*mm).mqd_size); let wa = if (*q).is_dbg_wa { 0xffff } else { 0xffffffff };
    core::ptr::write_bytes(m as *mut u8, 0, size as usize); (*m).header=0xC0310800; (*m).compute_pipelinestat_enable=1;
    (*m).compute_static_thread_mgmt_se0=wa; (*m).compute_static_thread_mgmt_se1=wa; (*m).compute_static_thread_mgmt_se2=wa; (*m).compute_static_thread_mgmt_se3=wa; (*m).compute_static_thread_mgmt_se4=wa; (*m).compute_static_thread_mgmt_se5=wa; (*m).compute_static_thread_mgmt_se6=wa; (*m).compute_static_thread_mgmt_se7=wa;
    (*m).cp_hqd_persistent_state=CP_HQD_PERSISTENT_STATE__PRELOAD_REQ_MASK | (0x55 << CP_HQD_PERSISTENT_STATE__PRELOAD_SIZE__SHIFT); (*m).cp_hqd_pq_control=5 << CP_HQD_PQ_CONTROL__RPTR_BLOCK_SIZE__SHIFT; (*m).cp_hqd_pq_control |= CP_HQD_PQ_CONTROL__UNORD_DISPATCH_MASK; (*m).cp_mqd_control=1 << CP_MQD_CONTROL__PRIV_STATE__SHIFT;
    (*m).cp_mqd_base_addr_lo=lower_32_bits(addr); (*m).cp_mqd_base_addr_hi=upper_32_bits(addr); (*m).cp_hqd_quantum=(1<<CP_HQD_QUANTUM__QUANTUM_EN__SHIFT)|(1<<CP_HQD_QUANTUM__QUANTUM_SCALE__SHIFT)|(1<<CP_HQD_QUANTUM__QUANTUM_DURATION__SHIFT); (*m).cp_hqd_hq_status0=1<<14;
    if amdgpu_amdkfd_have_atomics_support((*(*mm).dev).adev) { (*m).cp_hqd_hq_status0 |= 1<<29; } if (*q).format==KFD_QUEUE_FORMAT_AQL { (*m).cp_hqd_aql_control=1<<CP_HQD_AQL_CONTROL__CONTROL0__SHIFT; }
    if (*(*mm).dev).kfd.cwsr_enabled { (*m).cp_hqd_persistent_state |= 1<<CP_HQD_PERSISTENT_STATE__QSWITCH_MODE__SHIFT; (*m).cp_hqd_ctx_save_base_addr_lo=lower_32_bits((*q).ctx_save_restore_area_address); (*m).cp_hqd_ctx_save_base_addr_hi=upper_32_bits((*q).ctx_save_restore_area_address); (*m).cp_hqd_ctx_save_size=(*q).ctx_save_restore_area_size; (*m).cp_hqd_cntl_stack_size=(*q).ctl_stack_size; (*m).cp_hqd_cntl_stack_offset=(*q).ctl_stack_size; (*m).cp_hqd_wg_state_offset=(*q).ctl_stack_size; }
    mutex_lock(&mut (*(*mm).dev).kfd.profiler_lock); if !(*(*mm).dev).kfd.profiler_process.is_null() { (*m).compute_perfcount_enable=1; } mutex_unlock(&mut (*(*mm).dev).kfd.profiler_lock);
    *mqd=m as *mut core::ffi::c_void; if !gart.is_null(){*gart=addr;} ((*mm).update_mqd.unwrap())(mm,m as *mut _,q,core::ptr::null_mut());
}

unsafe fn load_mqd(mm:*mut mqd_manager,mqd:*mut core::ffi::c_void,pipe:u32,queue:u32,p:*mut queue_properties,mms:*mut mm_struct)->i32 { ((*mm).dev).kfd2kgd.hqd_load.unwrap()((*mm).dev, mqd, pipe, queue, (*p).write_ptr as *mut u32, if (*p).format==KFD_QUEUE_FORMAT_AQL {4} else {0},0,mms,0) }

unsafe fn update_mqd(mm:*mut mqd_manager,mqd:*mut core::ffi::c_void,q:*mut queue_properties,minfo:*mut mqd_update_info) { let m=get_mqd(mqd); (*m).cp_hqd_pq_control &= !CP_HQD_PQ_CONTROL__QUEUE_SIZE_MASK; (*m).cp_hqd_pq_control |= ffs((*q).queue_size / core::mem::size_of::<u32>() as u32)-1-1; (*m).cp_hqd_pq_base_lo=lower_32_bits((*q).queue_address>>8); (*m).cp_hqd_pq_base_hi=upper_32_bits((*q).queue_address>>8); (*m).cp_hqd_pq_rptr_report_addr_lo=lower_32_bits((*q).read_ptr as u64); (*m).cp_hqd_pq_rptr_report_addr_hi=upper_32_bits((*q).read_ptr as u64); (*m).cp_hqd_pq_wptr_poll_addr_lo=lower_32_bits((*q).write_ptr as u64); (*m).cp_hqd_pq_wptr_poll_addr_hi=upper_32_bits((*q).write_ptr as u64); (*m).cp_hqd_pq_doorbell_control=(*q).doorbell_off<<CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_OFFSET__SHIFT; (*m).cp_hqd_ib_control=3<<CP_HQD_IB_CONTROL__MIN_IB_AVAIL_SIZE__SHIFT; (*m).cp_hqd_eop_control=if (*q).eop_ring_buffer_size!=0 { core::cmp::min(0xA,ffs((*q).eop_ring_buffer_size/core::mem::size_of::<u32>() as u32)-1-1) } else {0}; (*m).cp_hqd_eop_base_addr_lo=lower_32_bits((*q).eop_ring_buffer_address>>8); (*m).cp_hqd_eop_base_addr_hi=upper_32_bits((*q).eop_ring_buffer_address>>8); (*m).cp_hqd_iq_timer=0; (*m).cp_hqd_vmid=(*q).vmid; if (*q).format==KFD_QUEUE_FORMAT_AQL { (*m).cp_hqd_pq_control|=CP_HQD_PQ_CONTROL__NO_UPDATE_RPTR_MASK | (2<<CP_HQD_PQ_CONTROL__SLOT_BASED_WPTR__SHIFT) | (1<<CP_HQD_PQ_CONTROL__QUEUE_FULL_EN__SHIFT); (*m).cp_hqd_pq_doorbell_control|=1<<CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_BIF_DROP__SHIFT; } if (*(*mm).dev).kfd.cwsr_enabled {(*m).cp_hqd_ctx_save_control=0;} if !minfo.is_null(){if (*minfo).update_flag==UPDATE_FLAG_PERFCOUNT_ENABLE{(*m).compute_perfcount_enable=1}else if (*minfo).update_flag==UPDATE_FLAG_PERFCOUNT_DISABLE{(*m).compute_perfcount_enable=0}} update_cu_mask(mm,mqd,minfo); set_priority(m,q); (*q).is_active=QUEUE_IS_ACTIVE(*q); }

unsafe fn checkpoint_mqd(_mm:*mut mqd_manager,mqd:*mut core::ffi::c_void,dst:*mut core::ffi::c_void,_ctl:*mut core::ffi::c_void){core::ptr::copy_nonoverlapping(get_mqd(mqd) as *const u8,dst as *mut u8,core::mem::size_of::<v11_compute_mqd>());}
unsafe fn init_mqd_hiq(mm:*mut mqd_manager,mqd:*mut *mut core::ffi::c_void,obj:*mut kfd_mem_obj,g:*mut u64,q:*mut queue_properties){init_mqd(mm,mqd,obj,g,q);let m=get_mqd(*mqd);(*m).cp_hqd_pq_control|=1<<CP_HQD_PQ_CONTROL__PRIV_STATE__SHIFT|1<<CP_HQD_PQ_CONTROL__KMD_QUEUE__SHIFT;}
unsafe fn destroy_hiq_mqd(mm:*mut mqd_manager,mqd:*mut core::ffi::c_void,_t:kfd_preempt_type,_timeout:u32,_p:u32,_q:u32)->i32{let m=get_mqd(mqd);amdgpu_amdkfd_unmap_hiq((*mm).dev.adev,(*m).cp_hqd_pq_doorbell_control>>CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_OFFSET__SHIFT,0)}

unsafe fn init_mqd_sdma(mm:*mut mqd_manager,mqd:*mut *mut core::ffi::c_void,obj:*mut kfd_mem_obj,g:*mut u64,q:*mut queue_properties){let m=(*obj).cpu_ptr as *mut v11_sdma_mqd;let size=if (*mm).dev.kfd.shared_resources.enable_mes{PAGE_SIZE}else{core::mem::size_of::<v11_sdma_mqd>()};core::ptr::write_bytes(m as *mut u8,0,size);*mqd=m as *mut _;if !g.is_null(){*g=(*obj).gpu_addr;}((*mm).update_mqd.unwrap())(mm,m as *mut _,q,core::ptr::null_mut());}
const SDMA_RLC_DUMMY_DEFAULT:u32=0xf;
unsafe fn update_mqd_sdma(_mm:*mut mqd_manager,mqd:*mut core::ffi::c_void,q:*mut queue_properties,_i:*mut mqd_update_info){let m=get_sdma_mqd(mqd);(*m).sdmax_rlcx_rb_cntl=(ffs((*q).queue_size/core::mem::size_of::<u32>() as u32)-1)<<SDMA0_QUEUE0_RB_CNTL__RB_SIZE__SHIFT|(*q).vmid<<SDMA0_QUEUE0_RB_CNTL__RB_VMID__SHIFT|1<<SDMA0_QUEUE0_RB_CNTL__RPTR_WRITEBACK_ENABLE__SHIFT|6<<SDMA0_QUEUE0_RB_CNTL__RPTR_WRITEBACK_TIMER__SHIFT|1<<SDMA0_QUEUE0_RB_CNTL__F32_WPTR_POLL_ENABLE__SHIFT;(*m).sdmax_rlcx_rb_base=lower_32_bits((*q).queue_address>>8);(*m).sdmax_rlcx_rb_base_hi=upper_32_bits((*q).queue_address>>8);(*m).sdmax_rlcx_rb_rptr_addr_lo=lower_32_bits((*q).read_ptr as u64);(*m).sdmax_rlcx_rb_rptr_addr_hi=upper_32_bits((*q).read_ptr as u64);(*m).sdmax_rlcx_rb_wptr_poll_addr_lo=lower_32_bits((*q).write_ptr as u64);(*m).sdmax_rlcx_rb_wptr_poll_addr_hi=upper_32_bits((*q).write_ptr as u64);(*m).sdmax_rlcx_doorbell_offset=(*q).doorbell_off<<SDMA0_QUEUE0_DOORBELL_OFFSET__OFFSET__SHIFT;(*m).sdmax_rlcx_sched_cntl=(amdgpu_sdma_phase_quantum<<SDMA0_QUEUE0_SCHEDULE_CNTL__CONTEXT_QUANTUM__SHIFT)&SDMA0_QUEUE0_SCHEDULE_CNTL__CONTEXT_QUANTUM_MASK;(*m).sdma_engine_id=(*q).sdma_engine_id;(*m).sdma_queue_id=(*q).sdma_queue_id;(*m).sdmax_rlcx_dummy_reg=SDMA_RLC_DUMMY_DEFAULT;(*q).is_active=QUEUE_IS_ACTIVE(*q);}

unsafe fn restore_mqd(mm:*mut mqd_manager,mqd:*mut *mut core::ffi::c_void,obj:*mut kfd_mem_obj,g:*mut u64,qp:*mut queue_properties,src:*const core::ffi::c_void,_ctl:*const core::ffi::c_void,_size:u32){let m=(*obj).cpu_ptr as *mut v11_compute_mqd;core::ptr::copy_nonoverlapping(src as *const u8,m as *mut u8,core::mem::size_of::<v11_compute_mqd>());*mqd=m as *mut _;if !g.is_null(){*g=(*obj).gpu_addr;}(*m).cp_hqd_pq_doorbell_control=(*qp).doorbell_off<<CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_OFFSET__SHIFT;(*qp).is_active=0;}
unsafe fn checkpoint_mqd_sdma(_mm:*mut mqd_manager,mqd:*mut core::ffi::c_void,dst:*mut core::ffi::c_void,_ctl:*mut core::ffi::c_void){core::ptr::copy_nonoverlapping(get_sdma_mqd(mqd) as *const u8,dst as *mut u8,core::mem::size_of::<v11_sdma_mqd>());}
unsafe fn restore_mqd_sdma(_mm:*mut mqd_manager,mqd:*mut *mut core::ffi::c_void,obj:*mut kfd_mem_obj,g:*mut u64,qp:*mut queue_properties,src:*const core::ffi::c_void,_ctl:*const core::ffi::c_void,_size:u32){let m=(*obj).cpu_ptr as *mut v11_sdma_mqd;core::ptr::copy_nonoverlapping(src as *const u8,m as *mut u8,core::mem::size_of::<v11_sdma_mqd>());(*m).sdmax_rlcx_doorbell_offset=(*qp).doorbell_off<<SDMA0_QUEUE0_DOORBELL_OFFSET__OFFSET__SHIFT;*mqd=m as *mut _;if !g.is_null(){*g=(*obj).gpu_addr;}(*qp).is_active=0;}

unsafe fn get_wave_state(_mm:*mut mqd_manager,mqd:*mut core::ffi::c_void,_q:*mut queue_properties,ctl:*mut core::ffi::c_void,cu:*mut u32,su:*mut u32)->i32{let m=get_mqd(mqd);*cu=(*m).cp_hqd_cntl_stack_size-(*m).cp_hqd_cntl_stack_offset;*su=(*m).cp_hqd_wg_state_offset-(*m).cp_hqd_cntl_stack_size;let mut h: kfd_context_save_area_header=core::mem::zeroed();h.wave_state.control_stack_size=*cu;h.wave_state.wave_state_size=*su;h.wave_state.wave_state_offset=(*m).cp_hqd_wg_state_offset;h.wave_state.control_stack_offset=(*m).cp_hqd_cntl_stack_offset;if copy_to_user(ctl,&h,core::mem::size_of_val(&h.wave_state))!=0{-EFAULT}else{0}}
unsafe fn check_preemption_failed(mm:*mut mqd_manager,mqd:*mut core::ffi::c_void)->bool{let m=get_mqd(mqd);kfd_check_hiq_mqd_doorbell_id((*mm).dev,(*m).queue_doorbell_id0,0)}

unsafe fn mqd_manager_init_v11(ty: KFD_MQD_TYPE, dev: *mut kfd_node) -> *mut mqd_manager {
    if ty >= KFD_MQD_TYPE_MAX { return core::ptr::null_mut(); }
    let m = kzalloc_obj::<mqd_manager>(); if m.is_null() { return m; } (*m).dev=dev;
    match ty {
        KFD_MQD_TYPE_CP => { (*m).allocate_mqd=Some(allocate_mqd);(*m).init_mqd=Some(init_mqd);(*m).free_mqd=Some(kfd_free_mqd_cp);(*m).load_mqd=Some(load_mqd);(*m).update_mqd=Some(update_mqd);(*m).destroy_mqd=Some(kfd_destroy_mqd_cp);(*m).is_occupied=Some(kfd_is_occupied_cp);(*m).mqd_size=core::mem::size_of::<v11_compute_mqd>();(*m).get_wave_state=Some(get_wave_state);(*m).mqd_stride=kfd_mqd_stride;(*m).checkpoint_mqd=Some(checkpoint_mqd);(*m).restore_mqd=Some(restore_mqd); }
        KFD_MQD_TYPE_HIQ => {(*m).allocate_mqd=Some(allocate_hiq_mqd);(*m).init_mqd=Some(init_mqd_hiq);(*m).free_mqd=Some(free_mqd_hiq_sdma);(*m).load_mqd=Some(kfd_hiq_load_mqd_kiq);(*m).update_mqd=Some(update_mqd);(*m).destroy_mqd=Some(destroy_hiq_mqd);(*m).is_occupied=Some(kfd_is_occupied_cp);(*m).mqd_size=core::mem::size_of::<v11_compute_mqd>();(*m).mqd_stride=kfd_mqd_stride;(*m).check_preemption_failed=Some(check_preemption_failed);}
        KFD_MQD_TYPE_DIQ => {(*m).allocate_mqd=Some(allocate_mqd);(*m).init_mqd=Some(init_mqd_hiq);(*m).free_mqd=Some(kfd_free_mqd_cp);(*m).load_mqd=Some(load_mqd);(*m).update_mqd=Some(update_mqd);(*m).destroy_mqd=Some(kfd_destroy_mqd_cp);(*m).is_occupied=Some(kfd_is_occupied_cp);(*m).mqd_size=core::mem::size_of::<v11_compute_mqd>();}
        KFD_MQD_TYPE_SDMA => {(*m).allocate_mqd=Some(allocate_sdma_mqd);(*m).init_mqd=Some(init_mqd_sdma);(*m).free_mqd=Some(free_mqd_hiq_sdma);(*m).load_mqd=Some(kfd_load_mqd_sdma);(*m).update_mqd=Some(update_mqd_sdma);(*m).destroy_mqd=Some(kfd_destroy_mqd_sdma);(*m).is_occupied=Some(kfd_is_occupied_sdma);(*m).checkpoint_mqd=Some(checkpoint_mqd_sdma);(*m).restore_mqd=Some(restore_mqd_sdma);(*m).mqd_size=core::mem::size_of::<v11_sdma_mqd>();(*m).mqd_stride=kfd_mqd_stride;if (*dev).kfd.shared_resources.enable_mes{(*m).allocate_mqd=Some(allocate_mqd);(*m).free_mqd=Some(kfd_free_mqd_cp);}}
        _ => { kfree(m as *mut _); return core::ptr::null_mut(); }
    } m
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
