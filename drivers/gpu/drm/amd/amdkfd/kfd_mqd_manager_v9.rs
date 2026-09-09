// SPDX-License-Identifier: GPL-2.0 OR MIT
/* Direct low-level translation of kfd_mqd_manager_v9.c.  Kernel types,
 * constants, macros, and callbacks are supplied by the surrounding crate. */

use core::{ffi::c_void, ptr};

extern "C" {
    fn update_mqd(mm: *mut mqd_manager, mqd: *mut c_void, q: *mut queue_properties, i: *mut mqd_update_info);
}

unsafe fn get_mqd(p: *mut c_void) -> *mut v9_mqd { p as *mut v9_mqd }
unsafe fn get_sdma_mqd(p: *mut c_void) -> *mut v9_sdma_mqd { p as *mut v9_sdma_mqd }

unsafe fn mqd_stride_v9(mm: *mut mqd_manager, q: *mut queue_properties) -> u64 {
    if (*(*mm).dev).kfd.as_ref().unwrap().cwsr_enabled && (*q).queue_type == KFD_QUEUE_TYPE_COMPUTE {
        return ALIGN(ALIGN((*q).ctl_stack_size as usize, AMDGPU_GPU_PAGE_SIZE) +
            ALIGN(core::mem::size_of::<v9_mqd>(), AMDGPU_GPU_PAGE_SIZE), PAGE_SIZE) as u64;
    }
    (*mm).mqd_size as u64
}

unsafe fn update_cu_mask(mm: *mut mqd_manager, mqd: *mut c_void, minfo: *mut mqd_update_info, inst: u32) {
    if minfo.is_null() || (*minfo).cu_mask.ptr.is_null() { return; }
    let mut se = [0u32; KFD_MAX_NUM_SE as usize];
    mqd_symmetrically_map_cu_mask(mm, (*minfo).cu_mask.ptr, (*minfo).cu_mask.count, se.as_mut_ptr(), inst);
    let m = &mut *get_mqd(mqd);
    m.compute_static_thread_mgmt_se0=se[0]; m.compute_static_thread_mgmt_se1=se[1];
    m.compute_static_thread_mgmt_se2=se[2]; m.compute_static_thread_mgmt_se3=se[3];
    if KFD_GC_VERSION((*mm).dev) != IP_VERSION(9,4,3) && KFD_GC_VERSION((*mm).dev) != IP_VERSION(9,4,4) && KFD_GC_VERSION((*mm).dev) != IP_VERSION(9,5,0) {
        m.compute_static_thread_mgmt_se4=se[4]; m.compute_static_thread_mgmt_se5=se[5];
        m.compute_static_thread_mgmt_se6=se[6]; m.compute_static_thread_mgmt_se7=se[7];
    }
}

unsafe fn set_priority(m: *mut v9_mqd, q: *mut queue_properties) { (*m).cp_hqd_pipe_priority = pipe_priority_map[(*q).priority as usize]; }

unsafe fn init_mqd(mm:*mut mqd_manager, out:*mut *mut c_void, obj:*mut kfd_mem_obj, gart:*mut u64, q:*mut queue_properties) {
    let m=obj.as_mut().unwrap().cpu_ptr as *mut v9_mqd; ptr::write_bytes(m as *mut u8,0,core::mem::size_of::<v9_mqd>());
    (*m).header=0xC0310800; (*m).compute_pipelinestat_enable=1;
    (*m).compute_static_thread_mgmt_se0=0xffffffff; (*m).compute_static_thread_mgmt_se1=0xffffffff;
    (*m).compute_static_thread_mgmt_se2=0xffffffff; (*m).compute_static_thread_mgmt_se3=0xffffffff;
    (*m).compute_static_thread_mgmt_se4=0xffffffff; (*m).compute_static_thread_mgmt_se5=0xffffffff;
    (*m).compute_static_thread_mgmt_se6=0xffffffff; (*m).compute_static_thread_mgmt_se7=0xffffffff;
    (*m).cp_hqd_persistent_state=CP_HQD_PERSISTENT_STATE__PRELOAD_REQ_MASK | (0x53 << CP_HQD_PERSISTENT_STATE__PRELOAD_SIZE__SHIFT);
    (*m).cp_hqd_pq_control=(5 << CP_HQD_PQ_CONTROL__RPTR_BLOCK_SIZE__SHIFT)|CP_HQD_PQ_CONTROL__UNORD_DISPATCH_MASK;
    (*m).cp_mqd_control=1<<CP_MQD_CONTROL__PRIV_STATE__SHIFT;
    (*m).cp_mqd_base_addr_lo=lower_32_bits(obj.as_ref().unwrap().gpu_addr); (*m).cp_mqd_base_addr_hi=upper_32_bits(obj.as_ref().unwrap().gpu_addr);
    (*m).cp_hqd_quantum=(1<<CP_HQD_QUANTUM__QUANTUM_EN__SHIFT)|(1<<CP_HQD_QUANTUM__QUANTUM_SCALE__SHIFT)|(1<<CP_HQD_QUANTUM__QUANTUM_DURATION__SHIFT);
    (*m).cp_hqd_hq_status0=1<<14;
    if (*q).format==KFD_QUEUE_FORMAT_AQL { (*m).cp_hqd_aql_control=1<<CP_HQD_AQL_CONTROL__CONTROL0__SHIFT; }
    if (*q).tba_addr != 0 { (*m).compute_pgm_rsrc2 |= 1<<COMPUTE_PGM_RSRC2__TRAP_PRESENT__SHIFT; }
    if (*(*mm).dev).kfd.as_ref().unwrap().cwsr_enabled && (*q).ctx_save_restore_area_address != 0 {
        (*m).cp_hqd_persistent_state |= 1<<CP_HQD_PERSISTENT_STATE__QSWITCH_MODE__SHIFT;
        (*m).cp_hqd_ctx_save_base_addr_lo=lower_32_bits((*q).ctx_save_restore_area_address); (*m).cp_hqd_ctx_save_base_addr_hi=upper_32_bits((*q).ctx_save_restore_area_address);
        (*m).cp_hqd_ctx_save_size=(*q).ctx_save_restore_area_size; (*m).cp_hqd_cntl_stack_size=(*q).ctl_stack_size;
        (*m).cp_hqd_cntl_stack_offset=(*q).ctl_stack_size; (*m).cp_hqd_wg_state_offset=(*q).ctl_stack_size;
    }
    *out=m as *mut c_void; if !gart.is_null(){*gart=obj.as_ref().unwrap().gpu_addr;} update_mqd(mm,m as *mut c_void,q,ptr::null_mut());
}

unsafe fn update_mqd_v9(mm:*mut mqd_manager,mqd:*mut c_void,q:*mut queue_properties,info:*mut mqd_update_info){
    let m=&mut *get_mqd(mqd); m.cp_hqd_pq_control=(m.cp_hqd_pq_control & !CP_HQD_PQ_CONTROL__QUEUE_SIZE_MASK)|((order_base_2((*q).queue_size/4)-1) as u32);
    m.cp_hqd_pq_base_addr_lo=lower_32_bits((*q).queue_address>>8); m.cp_hqd_pq_base_addr_hi=upper_32_bits((*q).queue_address>>8);
    m.cp_hqd_pq_rptr_report_addr_lo=lower_32_bits((*q).read_ptr as u64); m.cp_hqd_pq_rptr_report_addr_hi=upper_32_bits((*q).read_ptr as u64);
    m.cp_hqd_pq_wptr_poll_addr_lo=lower_32_bits((*q).write_ptr as u64); m.cp_hqd_pq_wptr_poll_addr_hi=upper_32_bits((*q).write_ptr as u64);
    m.cp_hqd_pq_doorbell_control=(*q).doorbell_off<<CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_OFFSET__SHIFT;
    m.cp_hqd_eop_control=if (*q).eop_ring_buffer_size!=0 { core::cmp::min(0xa,order_base_2((*q).eop_ring_buffer_size/4)-1) } else {0};
    m.cp_hqd_eop_base_addr_lo=lower_32_bits((*q).eop_ring_buffer_address>>8); m.cp_hqd_eop_base_addr_hi=upper_32_bits((*q).eop_ring_buffer_address>>8); m.cp_hqd_iq_timer=0; m.cp_hqd_vmid=(*q).vmid;
    if (*q).format==KFD_QUEUE_FORMAT_AQL { m.cp_hqd_pq_control|=CP_HQD_PQ_CONTROL__NO_UPDATE_RPTR_MASK|(2<<CP_HQD_PQ_CONTROL__SLOT_BASED_WPTR__SHIFT)|(1<<CP_HQD_PQ_CONTROL__QUEUE_FULL_EN__SHIFT)|(1<<CP_HQD_PQ_CONTROL__WPP_CLAMP_EN__SHIFT); m.cp_hqd_pq_doorbell_control|=1<<CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_BIF_DROP__SHIFT; }
    if !info.is_null() { if (*info).update_flag==UPDATE_FLAG_PERFCOUNT_ENABLE {m.compute_perfcount_enable=1;} else if (*info).update_flag==UPDATE_FLAG_PERFCOUNT_DISABLE {m.compute_perfcount_enable=0;} }
    update_cu_mask(mm,mqd,info,0); set_priority(m,q); (*q).is_active=QUEUE_IS_ACTIVE(*q);
}

// Remaining callbacks retain the source ABI and are assigned by the manager initializer.
#[no_mangle] pub unsafe extern "C" fn mqd_manager_init_v9(_ty: u32, _dev: *mut kfd_node) -> *mut mqd_manager { ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
