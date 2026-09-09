/* Translated from amdgpu_amdkfd_gc_9_4_3.c. External kernel definitions are
 * supplied by the surrounding repository. */

#[inline]
unsafe fn get_sdma_mqd(mqd: *mut core::ffi::c_void) -> *mut v9_sdma_mqd {
    mqd as *mut v9_sdma_mqd
}

unsafe fn get_sdma_rlc_reg_offset(adev: *mut amdgpu_device, engine_id: u32, queue_id: u32) -> u32 {
    let sdma_engine_reg_base = SOC15_REG_OFFSET(SDMA0, GET_INST(SDMA0, engine_id), regSDMA_RLC0_RB_CNTL) - regSDMA_RLC0_RB_CNTL;
    let retval = sdma_engine_reg_base + queue_id * (regSDMA_RLC1_RB_CNTL - regSDMA_RLC0_RB_CNTL);
    pr_debug!("RLC register offset for SDMA{} RLC{}: 0x{:x}\n", engine_id, queue_id, retval);
    retval
}

unsafe fn kgd_gfx_v9_4_3_hqd_sdma_load(adev: *mut amdgpu_device, mqd: *mut core::ffi::c_void, wptr: *mut u32, mm: *mut mm_struct) -> i32 {
    let m = get_sdma_mqd(mqd);
    let off = get_sdma_rlc_reg_offset(adev, (*m).sdma_engine_id, (*m).sdma_queue_id);
    WREG32(off + regSDMA_RLC0_RB_CNTL, (*m).sdmax_rlcx_rb_cntl & !SDMA_RLC0_RB_CNTL__RB_ENABLE_MASK);
    let end = msecs_to_jiffies(2000) + jiffies;
    loop {
        let data = RREG32(off + regSDMA_RLC0_CONTEXT_STATUS);
        if data & SDMA_RLC0_CONTEXT_STATUS__IDLE_MASK != 0 { break; }
        if time_after(jiffies, end) { pr_err!("SDMA RLC not idle in {}\n", core::module_path!()); return -ETIME; }
        usleep_range(500, 1000);
    }
    WREG32(off + regSDMA_RLC0_DOORBELL_OFFSET, (*m).sdmax_rlcx_doorbell_offset);
    let data = REG_SET_FIELD((*m).sdmax_rlcx_doorbell, SDMA_RLC0_DOORBELL, ENABLE, 1);
    WREG32(off + regSDMA_RLC0_DOORBELL, data);
    WREG32(off + regSDMA_RLC0_RB_RPTR, (*m).sdmax_rlcx_rb_rptr);
    WREG32(off + regSDMA_RLC0_RB_RPTR_HI, (*m).sdmax_rlcx_rb_rptr_hi);
    WREG32(off + regSDMA_RLC0_MINOR_PTR_UPDATE, 1);
    let mut data64 = 0u64;
    if read_user_wptr(mm, wptr as *mut u64, &mut data64) {
        WREG32(off + regSDMA_RLC0_RB_WPTR, lower_32_bits(data64)); WREG32(off + regSDMA_RLC0_RB_WPTR_HI, upper_32_bits(data64));
    } else {
        WREG32(off + regSDMA_RLC0_RB_WPTR, (*m).sdmax_rlcx_rb_rptr); WREG32(off + regSDMA_RLC0_RB_WPTR_HI, (*m).sdmax_rlcx_rb_rptr_hi);
    }
    WREG32(off + regSDMA_RLC0_MINOR_PTR_UPDATE, 0);
    WREG32(off + regSDMA_RLC0_RB_BASE, (*m).sdmax_rlcx_rb_base); WREG32(off + regSDMA_RLC0_RB_BASE_HI, (*m).sdmax_rlcx_rb_base_hi);
    WREG32(off + regSDMA_RLC0_RB_RPTR_ADDR_LO, (*m).sdmax_rlcx_rb_rptr_addr_lo); WREG32(off + regSDMA_RLC0_RB_RPTR_ADDR_HI, (*m).sdmax_rlcx_rb_rptr_addr_hi);
    let data = REG_SET_FIELD((*m).sdmax_rlcx_rb_cntl, SDMA_RLC0_RB_CNTL, RB_ENABLE, 1);
    WREG32(off + regSDMA_RLC0_RB_CNTL, data); 0
}

unsafe fn kgd_gfx_v9_4_3_hqd_sdma_dump(adev: *mut amdgpu_device, engine_id: u32, queue_id: u32, dump: *mut *mut [[u32;2]], n_regs: *mut u32) -> i32 {
    let off = get_sdma_rlc_reg_offset(adev, engine_id, queue_id);
    const HQD_N_REGS: usize = 19 + 6 + 7 + 12;
    *dump = kmalloc_objs::<[u32;2]>(HQD_N_REGS);
    if (*dump).is_null() { return -ENOMEM; }
    let mut i = 0usize;
    macro_rules! dump_reg { ($a:expr) => {{ if i >= HQD_N_REGS { break; } (**dump)[i][0] = ($a) << 2; (**dump)[i][1] = RREG32($a); i += 1; }}; }
    for reg in regSDMA_RLC0_RB_CNTL..=regSDMA_RLC0_DOORBELL { dump_reg!(off + reg); }
    for reg in regSDMA_RLC0_STATUS..=regSDMA_RLC0_CSA_ADDR_HI { dump_reg!(off + reg); }
    for reg in regSDMA_RLC0_IB_SUB_REMAIN..=regSDMA_RLC0_MINOR_PTR_UPDATE { dump_reg!(off + reg); }
    for reg in regSDMA_RLC0_MIDCMD_DATA0..=regSDMA_RLC0_MIDCMD_CNTL { dump_reg!(off + reg); }
    WARN_ON_ONCE(i != HQD_N_REGS); *n_regs = i as u32; 0
}

unsafe fn kgd_gfx_v9_4_3_hqd_sdma_is_occupied(adev: *mut amdgpu_device, mqd: *mut core::ffi::c_void) -> bool {
    let m = get_sdma_mqd(mqd); let off = get_sdma_rlc_reg_offset(adev, (*m).sdma_engine_id, (*m).sdma_queue_id);
    RREG32(off + regSDMA_RLC0_RB_CNTL) & SDMA_RLC0_RB_CNTL__RB_ENABLE_MASK != 0
}

unsafe fn kgd_gfx_v9_4_3_hqd_sdma_destroy(adev: *mut amdgpu_device, mqd: *mut core::ffi::c_void, utimeout: u32) -> i32 {
    let m = get_sdma_mqd(mqd); let off = get_sdma_rlc_reg_offset(adev, (*m).sdma_engine_id, (*m).sdma_queue_id);
    let mut temp = RREG32(off + regSDMA_RLC0_RB_CNTL) & !SDMA_RLC0_RB_CNTL__RB_ENABLE_MASK; WREG32(off + regSDMA_RLC0_RB_CNTL, temp);
    let end = utimeout * HZ / 1000 + jiffies; loop { temp = RREG32(off + regSDMA_RLC0_CONTEXT_STATUS); if temp & SDMA_RLC0_CONTEXT_STATUS__IDLE_MASK != 0 { break; } if time_after(jiffies,end) { pr_err!("SDMA RLC not idle\n"); return -ETIME; } usleep_range(500,1000); }
    WREG32(off + regSDMA_RLC0_DOORBELL, 0); WREG32(off + regSDMA_RLC0_RB_CNTL, RREG32(off + regSDMA_RLC0_RB_CNTL) | SDMA_RLC0_RB_CNTL__RB_ENABLE_MASK);
    (*m).sdmax_rlcx_rb_rptr = RREG32(off + regSDMA_RLC0_RB_RPTR); (*m).sdmax_rlcx_rb_rptr_hi = RREG32(off + regSDMA_RLC0_RB_RPTR_HI); 0
}

unsafe fn kgd_gfx_v9_4_3_set_pasid_vmid_mapping(adev:*mut amdgpu_device,pasid:u32,vmid:u32,xcc_inst:u32)->i32 { let phy=GET_INST(GC,xcc_inst); let aid=phy/2; let p=if pasid==0{0}else{pasid|ATC_VMID0_PASID_MAPPING__VALID_MASK}; WREG32(SOC15_REG_OFFSET(ATHUB,0,regATC_VMID0_PASID_MAPPING)+vmid,p); let timeout=jiffies+msecs_to_jiffies(10); while RREG32(SOC15_REG_OFFSET(ATHUB,0,regATC_VMID_PASID_MAPPING_UPDATE_STATUS))&(1u32<<vmid)==0 { if time_after(jiffies,timeout){return -ETIME;} cpu_relax(); } WREG32(SOC15_REG_OFFSET(ATHUB,0,regATC_VMID_PASID_MAPPING_UPDATE_STATUS),1u32<<vmid); let old=RREG32(SOC15_REG_OFFSET(OSSSYS,0,regIH_VMID_LUT_INDEX)); WREG32(SOC15_REG_OFFSET(OSSSYS,0,regIH_VMID_LUT_INDEX),aid*4+(phy%2)+1); WREG32(SOC15_REG_OFFSET(OSSSYS,0,regIH_VMID_0_LUT)+vmid,p); WREG32(SOC15_REG_OFFSET(OSSSYS,0,regIH_VMID_LUT_INDEX),aid*4); WREG32(SOC15_REG_OFFSET(OSSSYS,0,regIH_VMID_0_LUT_MM)+vmid,p); WREG32(SOC15_REG_OFFSET(OSSSYS,0,regIH_VMID_LUT_INDEX),old); 0 }

unsafe fn get_mqd(mqd:*mut core::ffi::c_void)->*mut v9_mqd { mqd as *mut v9_mqd }

/* The remaining callbacks preserve the source interfaces and delegate to the
 * corresponding v9 helpers supplied by the surrounding kernel translation. */
unsafe fn kgd_gfx_v9_4_3_hqd_load(adev:*mut amdgpu_device,mqd:*mut core::ffi::c_void,pipe:u32,queue:u32,wptr:*mut u32,_shift:u32,_mask:u32,_mm:*mut mm_struct,inst:u32)->i32 { let m=get_mqd(mqd); kgd_gfx_v9_acquire_queue(adev,pipe,queue,inst); let base=SOC15_REG_OFFSET(GC,GET_INST(GC,inst),regCP_MQD_BASE_ADDR); let end=SOC15_REG_OFFSET(GC,GET_INST(GC,inst),regCP_HQD_AQL_DISPATCH_ID_HI); let a=&(*m).cp_mqd_base_addr_lo as *const u32; for r in base..=end { WREG32_XCC(r,*a.add((r-base) as usize),inst); } let d=REG_SET_FIELD((*m).cp_hqd_pq_doorbell_control,CP_HQD_PQ_DOORBELL_CONTROL,DOORBELL_EN,1); WREG32_SOC15_RLC(GC,GET_INST(GC,inst),regCP_HQD_PQ_DOORBELL_CONTROL,d); if !wptr.is_null(){let qs=2<<REG_GET_FIELD((*m).cp_hqd_pq_control,CP_HQD_PQ_CONTROL,QUEUE_SIZE);let mut wp=(*m).cp_hqd_pq_rptr&(qs-1);if ((*m).cp_hqd_pq_wptr_lo&(qs-1))<wp{wp+=qs;}wp+=(*m).cp_hqd_pq_wptr_lo&!(qs-1);wp+=(u64)(*m).cp_hqd_pq_wptr_hi<<32;WREG32_SOC15_RLC(GC,GET_INST(GC,inst),regCP_HQD_PQ_WPTR_LO,lower_32_bits(wp));WREG32_SOC15_RLC(GC,GET_INST(GC,inst),regCP_HQD_PQ_WPTR_HI,upper_32_bits(wp));WREG32_SOC15_RLC(GC,GET_INST(GC,inst),regCP_HQD_PQ_WPTR_POLL_ADDR,lower_32_bits(wptr as usize as u64));WREG32_SOC15_RLC(GC,GET_INST(GC,inst),regCP_HQD_PQ_WPTR_POLL_ADDR_HI,upper_32_bits(wptr as usize as u64));WREG32_SOC15_RLC(GC,GET_INST(GC,inst),regCP_PQ_WPTR_POLL_CNTL1,kgd_gfx_v9_get_queue_mask(adev,pipe,queue));} WREG32_SOC15_RLC(GC,GET_INST(GC,inst),regCP_HQD_EOP_RPTR,REG_SET_FIELD((*m).cp_hqd_eop_rptr,CP_HQD_EOP_RPTR,INIT_FETCHER,1)); WREG32_SOC15_RLC(GC,GET_INST(GC,inst),regCP_HQD_ACTIVE,REG_SET_FIELD((*m).cp_hqd_active,CP_HQD_ACTIVE,ACTIVE,1)); kgd_gfx_v9_release_queue(adev,inst); 0 }

unsafe fn kgd_gfx_v9_4_3_disable_debug_trap(_adev:*mut amdgpu_device,_keep:bool,_vmid:u32)->u32 { let mut d=0; d=REG_SET_FIELD(d,SPI_GDBG_PER_VMID_CNTL,TRAP_EN,1); d=REG_SET_FIELD(d,SPI_GDBG_PER_VMID_CNTL,EXCP_EN,0); REG_SET_FIELD(d,SPI_GDBG_PER_VMID_CNTL,EXCP_REPLACE,0) }
unsafe fn kgd_gfx_v9_4_3_clear_address_watch(_adev:*mut amdgpu_device,_id:u32)->u32 { 0 }
unsafe fn kgd_gfx_v9_4_3_validate_trap_override_request(_adev:*mut amdgpu_device,override_:u32,supported:*mut u32)->i32 { *supported &= KFD_DBG_TRAP_MASK_FP_INVALID|KFD_DBG_TRAP_MASK_FP_INPUT_DENORMAL|KFD_DBG_TRAP_MASK_FP_DIVIDE_BY_ZERO|KFD_DBG_TRAP_MASK_FP_OVERFLOW|KFD_DBG_TRAP_MASK_FP_UNDERFLOW|KFD_DBG_TRAP_MASK_FP_INEXACT|KFD_DBG_TRAP_MASK_INT_DIVIDE_BY_ZERO|KFD_DBG_TRAP_MASK_DBG_ADDRESS_WATCH|KFD_DBG_TRAP_MASK_DBG_MEMORY_VIOLATION|KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_START|KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_END; if override_!=KFD_DBG_TRAP_OVERRIDE_OR&&override_!=KFD_DBG_TRAP_OVERRIDE_REPLACE{-EPERM}else{0} }
unsafe fn kgd_gfx_v9_4_3_set_wave_launch_trap_override(_adev:*mut amdgpu_device,_vmid:u32,override_:u32,bits:u32,request:u32,prev:*mut u32,old:u32)->u32 { *prev=REG_GET_FIELD(old,SPI_GDBG_PER_VMID_CNTL,EXCP_EN); let mut d=REG_SET_FIELD(0,SPI_GDBG_PER_VMID_CNTL,EXCP_EN,(bits&request)|(*prev&!request)); d=REG_SET_FIELD(d,SPI_GDBG_PER_VMID_CNTL,TRAP_EN,1); REG_SET_FIELD(d,SPI_GDBG_PER_VMID_CNTL,EXCP_REPLACE,override_) }
unsafe fn kgd_gfx_v9_4_3_set_address_watch(adev:*mut amdgpu_device,address:u64,mask:u32,id:u32,mode:u32,_vmid:u32,inst:u32)->u32 { let mut c=0; c=REG_SET_FIELD(c,TCP_WATCH0_CNTL,MODE,mode); c=REG_SET_FIELD(c,TCP_WATCH0_CNTL,MASK,mask>>7); c=REG_SET_FIELD(c,TCP_WATCH0_CNTL,VALID,1); let stride=regTCP_WATCH1_ADDR_H-regTCP_WATCH0_ADDR_H; WREG32_XCC(SOC15_REG_OFFSET(GC,GET_INST(GC,inst),regTCP_WATCH0_ADDR_H)+id*stride,upper_32_bits(address)&0xffff,inst); WREG32_XCC(SOC15_REG_OFFSET(GC,GET_INST(GC,inst),regTCP_WATCH0_ADDR_L)+id*stride,lower_32_bits(address),inst); c }

unsafe fn kgd_gfx_v9_4_3_hqd_sdma_get_doorbell(adev:*mut amdgpu_device,engine:i32,queue:i32)->u32 { let o=get_sdma_rlc_reg_offset(adev,engine as u32,queue as u32); let s=RREG32(regSDMA_RLC0_CONTEXT_STATUS+o); let d=RREG32(regSDMA_RLC0_DOORBELL_OFFSET+o); if REG_GET_FIELD(s,SDMA_RLC0_CONTEXT_STATUS,SELECTED)!=0 {d>>2} else {0} }

unsafe fn kgd_v9_4_3_ptl_ctrl(adev:*mut amdgpu_device,cmd:u32,state:*mut u32,f1:*mut amdgpu_ptl_fmt,f2:*mut amdgpu_ptl_fmt)->u32 { amdgpu_ptl_perf_monitor_ctrl(adev,cmd,state,f1,f2) }

unsafe fn kgd_gfx_v9_4_3_hqd_sdma_get_counter(adev:*mut amdgpu_device,mqd:*mut core::ffi::c_void,n:u32,val:*mut u64)->i32 { let m=get_sdma_mqd(mqd); if m.is_null(){return -EINVAL;} let ver=amdgpu_ip_version(adev,GC_HWIP,0); if ((ver==IP_VERSION(9,4,3)||ver==IP_VERSION(9,4,4))&&(*adev).gfx.mec_fw_version<194)||(ver==IP_VERSION(9,5,0)&&(*adev).gfx.mec_fw_version<44){pr_warn_once!("MEC FW doesn't support SDMA counter!\n");return -EOPNOTSUPP;} let mut off=0; let mut found=false; for e in 0..(*adev).sdma.num_instances { for q in 0..n { off=get_sdma_rlc_reg_offset(adev,e,q); if (*m).sdmax_rlcx_rb_base==RREG32(off+regSDMA_RLC0_RB_BASE)&&(*m).sdmax_rlcx_rb_base_hi==RREG32(off+regSDMA_RLC0_RB_BASE_HI){found=true;break;} } if found{break;} } let c=RREG32(off+regSDMA_RLC0_RB_CNTL); if c&SDMA_RLC0_RB_CNTL__RB_ENABLE_MASK!=0 {*val=((RREG32(off+regSDMA_RLC0_UTILIZATION_HI) as u64)<<32)|RREG32(off+regSDMA_RLC0_UTILIZATION_LO) as u64;}else{*val=(((*m).sdmax_rlcx_utilization_hi as u64)<<32)|(*m).sdmax_rlcx_utilization_lo as u64;} 0 }

/* The callback table mirrors gc_9_4_3_kfd2kgd exactly; its field and function
 * types are provided by the translated kernel headers. */
#[no_mangle]
pub static gc_9_4_3_kfd2kgd: kfd2kgd_calls = kfd2kgd_calls {
    program_sh_mem_settings: kgd_gfx_v9_program_sh_mem_settings,
    set_pasid_vmid_mapping: kgd_gfx_v9_4_3_set_pasid_vmid_mapping,
    init_interrupts: kgd_gfx_v9_init_interrupts,
    hqd_load: kgd_gfx_v9_4_3_hqd_load,
    hiq_mqd_load: kgd_gfx_v9_hiq_mqd_load,
    hqd_sdma_load: kgd_gfx_v9_4_3_hqd_sdma_load,
    hqd_dump: kgd_gfx_v9_hqd_dump,
    hqd_sdma_dump: kgd_gfx_v9_4_3_hqd_sdma_dump,
    hqd_is_occupied: kgd_gfx_v9_hqd_is_occupied,
    hqd_sdma_is_occupied: kgd_gfx_v9_4_3_hqd_sdma_is_occupied,
    hqd_destroy: kgd_gfx_v9_hqd_destroy,
    hqd_sdma_destroy: kgd_gfx_v9_4_3_hqd_sdma_destroy,
    wave_control_execute: kgd_gfx_v9_wave_control_execute,
    get_atc_vmid_pasid_mapping_info: kgd_gfx_v9_get_atc_vmid_pasid_mapping_info,
    set_vm_context_page_table_base: kgd_gfx_v9_set_vm_context_page_table_base,
    get_cu_occupancy: kgd_gfx_v9_get_cu_occupancy,
    program_trap_handler_settings: kgd_gfx_v9_program_trap_handler_settings,
    build_dequeue_wait_counts_packet_info: kgd_gfx_v9_build_dequeue_wait_counts_packet_info,
    get_iq_wait_times: kgd_gfx_v9_get_iq_wait_times,
    enable_debug_trap: kgd_aldebaran_enable_debug_trap,
    disable_debug_trap: kgd_gfx_v9_4_3_disable_debug_trap,
    validate_trap_override_request: kgd_gfx_v9_4_3_validate_trap_override_request,
    set_wave_launch_trap_override: kgd_gfx_v9_4_3_set_wave_launch_trap_override,
    set_wave_launch_mode: kgd_aldebaran_set_wave_launch_mode,
    set_address_watch: kgd_gfx_v9_4_3_set_address_watch,
    clear_address_watch: kgd_gfx_v9_4_3_clear_address_watch,
    hqd_get_pq_addr: kgd_gfx_v9_hqd_get_pq_addr,
    hqd_reset: kgd_gfx_v9_hqd_reset,
    hqd_sdma_get_doorbell: kgd_gfx_v9_4_3_hqd_sdma_get_doorbell,
    ptl_ctrl: kgd_v9_4_3_ptl_ctrl,
    hqd_sdma_get_counter: kgd_gfx_v9_4_3_hqd_sdma_get_counter,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
