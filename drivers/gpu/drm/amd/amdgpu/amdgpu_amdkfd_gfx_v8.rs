/* Direct Rust translation of amdgpu_amdkfd_gfx_v8.c. */

#[repr(C)]
pub enum hqd_dequeue_request_type { NO_ACTION = 0, DRAIN_PIPE, RESET_WAVES }

unsafe fn lock_srbm(adev: *mut amdgpu_device, mec: u32, pipe: u32, queue: u32, vmid: u32) {
    let value = PIPEID(pipe) | MEID(mec) | VMID(vmid) | QUEUEID(queue);
    mutex_lock(&mut (*adev).srbm_mutex);
    WREG32(adev, mmSRBM_GFX_CNTL, value);
}
unsafe fn unlock_srbm(adev: *mut amdgpu_device) { WREG32(adev, mmSRBM_GFX_CNTL, 0); mutex_unlock(&mut (*adev).srbm_mutex); }
unsafe fn acquire_queue(adev: *mut amdgpu_device, pipe_id: u32, queue_id: u32) {
    let mec = pipe_id / (*adev).gfx.mec.num_pipe_per_mec + 1;
    lock_srbm(adev, mec, pipe_id % (*adev).gfx.mec.num_pipe_per_mec, queue_id, 0);
}
unsafe fn release_queue(adev: *mut amdgpu_device) { unlock_srbm(adev); }

unsafe fn kgd_program_sh_mem_settings(adev: *mut amdgpu_device, vmid: u32, sh_mem_config: u32, sh_mem_ape1_base: u32, sh_mem_ape1_limit: u32, sh_mem_bases: u32, _inst: u32) {
    lock_srbm(adev, 0, 0, 0, vmid);
    WREG32(adev, mmSH_MEM_CONFIG, sh_mem_config); WREG32(adev, mmSH_MEM_APE1_BASE, sh_mem_ape1_base);
    WREG32(adev, mmSH_MEM_APE1_LIMIT, sh_mem_ape1_limit); WREG32(adev, mmSH_MEM_BASES, sh_mem_bases); unlock_srbm(adev);
}
unsafe fn kgd_set_pasid_vmid_mapping(adev: *mut amdgpu_device, pasid: u32, vmid: u32, _inst: u32) -> i32 {
    let mapping = if pasid == 0 { 0 } else { pasid | ATC_VMID0_PASID_MAPPING__VALID_MASK };
    WREG32(adev, mmATC_VMID0_PASID_MAPPING + vmid, mapping);
    while RREG32(adev, mmATC_VMID_PASID_MAPPING_UPDATE_STATUS) & (1u32 << vmid) == 0 { cpu_relax(); }
    WREG32(adev, mmATC_VMID_PASID_MAPPING_UPDATE_STATUS, 1u32 << vmid); WREG32(adev, mmIH_VMID_0_LUT + vmid, mapping); 0
}
unsafe fn kgd_init_interrupts(adev: *mut amdgpu_device, pipe_id: u32, _inst: u32) -> i32 {
    let mec = pipe_id / (*adev).gfx.mec.num_pipe_per_mec + 1; let pipe = pipe_id % (*adev).gfx.mec.num_pipe_per_mec;
    lock_srbm(adev, mec, pipe, 0, 0); WREG32(adev, mmCPC_INT_CNTL, CP_INT_CNTL_RING0__TIME_STAMP_INT_ENABLE_MASK | CP_INT_CNTL_RING0__OPCODE_ERROR_INT_ENABLE_MASK); unlock_srbm(adev); 0
}
unsafe fn get_sdma_rlc_reg_offset(m: *mut vi_sdma_mqd) -> u32 { let r = (*m).sdma_engine_id * SDMA1_REGISTER_OFFSET + (*m).sdma_queue_id * KFD_VI_SDMA_QUEUE_OFFSET; pr_debug!("RLC register offset for SDMA%d RLC%d: 0x%x\n", (*m).sdma_engine_id, (*m).sdma_queue_id, r); r }
unsafe fn get_mqd(mqd: *mut core::ffi::c_void) -> *mut vi_mqd { mqd as *mut vi_mqd }
unsafe fn get_sdma_mqd(mqd: *mut core::ffi::c_void) -> *mut vi_sdma_mqd { mqd as *mut vi_sdma_mqd }

unsafe fn kgd_hqd_load(adev: *mut amdgpu_device, mqd: *mut core::ffi::c_void, pipe_id: u32, queue_id: u32, wptr: *mut u32, wptr_shift: u32, wptr_mask: u32, mm: *mut mm_struct, _inst: u32) -> i32 {
    let m = get_mqd(mqd); acquire_queue(adev, pipe_id, queue_id);
    if (*m).cp_hqd_vmid == 0 { let mec = pipe_id / (*adev).gfx.mec.num_pipe_per_mec + 1; let pipe = pipe_id % (*adev).gfx.mec.num_pipe_per_mec; pr_debug!("kfd: set HIQ, mec:%d, pipe:%d, queue:%d.\n", mec, pipe, queue_id); let mut value = RREG32(adev, mmRLC_CP_SCHEDULERS); value = REG_SET_FIELD(value, RLC_CP_SCHEDULERS, scheduler1, (mec << 5) | (pipe << 3) | queue_id | 0x80); WREG32(adev, mmRLC_CP_SCHEDULERS, value); }
    let h = &(*m).cp_mqd_base_addr_lo as *const u32; let mut reg = mmCP_MQD_BASE_ADDR; while reg <= mmCP_HQD_EOP_CONTROL { WREG32(adev, reg, *h.add((reg - mmCP_MQD_BASE_ADDR) as usize)); reg += 1; }
    if (*adev).asic_type != CHIP_TONGA { WREG32(adev, mmCP_HQD_EOP_RPTR, (*m).cp_hqd_eop_rptr); WREG32(adev, mmCP_HQD_EOP_WPTR, (*m).cp_hqd_eop_wptr); WREG32(adev, mmCP_HQD_EOP_WPTR_MEM, (*m).cp_hqd_eop_wptr_mem); }
    reg = mmCP_HQD_EOP_EVENTS; while reg <= mmCP_HQD_ERROR { WREG32(adev, reg, *h.add((reg - mmCP_MQD_BASE_ADDR) as usize)); reg += 1; }
    let mut data = REG_SET_FIELD((*m).cp_hqd_pq_doorbell_control, CP_HQD_PQ_DOORBELL_CONTROL, DOORBELL_EN, 1); WREG32(adev, mmCP_HQD_PQ_DOORBELL_CONTROL, data);
    release_queue(adev); let mut wptr_val = 0; let valid = read_user_wptr(mm, wptr, &mut wptr_val); acquire_queue(adev, pipe_id, queue_id); if valid { WREG32(adev, mmCP_HQD_PQ_WPTR, (wptr_val << wptr_shift) & wptr_mask); }
    data = REG_SET_FIELD((*m).cp_hqd_active, CP_HQD_ACTIVE, ACTIVE, 1); WREG32(adev, mmCP_HQD_ACTIVE, data); release_queue(adev); 0
}

unsafe fn kgd_hqd_dump(adev: *mut amdgpu_device, pipe_id: u32, queue_id: u32, dump: *mut *mut [[u32; 2]], n_regs: *mut u32, _inst: u32) -> i32 {
    const HQD_N_REGS: usize = 58; let mut i = 0usize; *dump = kmalloc_objs(core::ptr::null_mut(), HQD_N_REGS); if (*dump).is_null() { return -ENOMEM; } acquire_queue(adev, pipe_id, queue_id);
    let mut put = |addr: u32| { if i < HQD_N_REGS { (**dump)[i][0] = addr << 2; (**dump)[i][1] = RREG32(adev, addr); i += 1; } };
    put(mmCOMPUTE_STATIC_THREAD_MGMT_SE0); put(mmCOMPUTE_STATIC_THREAD_MGMT_SE1); put(mmCOMPUTE_STATIC_THREAD_MGMT_SE2); put(mmCOMPUTE_STATIC_THREAD_MGMT_SE3); let mut r = mmCP_MQD_BASE_ADDR; while r <= mmCP_HQD_EOP_DONES { put(r); r += 1; } release_queue(adev); WARN_ON_ONCE(i != HQD_N_REGS); *n_regs = i as u32; 0
}

unsafe fn kgd_hqd_sdma_load(adev: *mut amdgpu_device, mqd: *mut core::ffi::c_void, wptr: *mut u32, mm: *mut mm_struct) -> i32 {
    let m = get_sdma_mqd(mqd); let off = get_sdma_rlc_reg_offset(m); WREG32(adev, off + mmSDMA0_RLC0_RB_CNTL, (*m).sdmax_rlcx_rb_cntl & !SDMA0_RLC0_RB_CNTL__RB_ENABLE_MASK); let end = msecs_to_jiffies(2000) + jiffies();
    loop { let d = RREG32(adev, off + mmSDMA0_RLC0_CONTEXT_STATUS); if d & SDMA0_RLC0_CONTEXT_STATUS__IDLE_MASK != 0 { break; } if time_after(jiffies(), end) { pr_err!("SDMA RLC not idle in %s\n", __func__); return -ETIME; } usleep_range(500,1000); }
    let mut d = REG_SET_FIELD((*m).sdmax_rlcx_doorbell, SDMA0_RLC0_DOORBELL, ENABLE, 1); WREG32(adev, off + mmSDMA0_RLC0_DOORBELL, d); WREG32(adev, off + mmSDMA0_RLC0_RB_RPTR, (*m).sdmax_rlcx_rb_rptr); if read_user_wptr(mm,wptr,&mut d) { WREG32(adev,off+mmSDMA0_RLC0_RB_WPTR,d); } else { WREG32(adev,off+mmSDMA0_RLC0_RB_WPTR,(*m).sdmax_rlcx_rb_rptr); }
    WREG32(adev,off+mmSDMA0_RLC0_VIRTUAL_ADDR,(*m).sdmax_rlcx_virtual_addr); WREG32(adev,off+mmSDMA0_RLC0_RB_BASE,(*m).sdmax_rlcx_rb_base); WREG32(adev,off+mmSDMA0_RLC0_RB_BASE_HI,(*m).sdmax_rlcx_rb_base_hi); WREG32(adev,off+mmSDMA0_RLC0_RB_RPTR_ADDR_LO,(*m).sdmax_rlcx_rb_rptr_addr_lo); WREG32(adev,off+mmSDMA0_RLC0_RB_RPTR_ADDR_HI,(*m).sdmax_rlcx_rb_rptr_addr_hi); d=REG_SET_FIELD((*m).sdmax_rlcx_rb_cntl,SDMA0_RLC0_RB_CNTL,RB_ENABLE,1); WREG32(adev,off+mmSDMA0_RLC0_RB_CNTL,d); 0
}

// The remaining routines preserve the original interface and register-level behavior.
unsafe fn kgd_hqd_sdma_dump(adev:*mut amdgpu_device, engine:u32, queue:u32, dump:*mut *mut [[u32;2]], n:*mut u32)->i32 { let off=engine*SDMA1_REGISTER_OFFSET+queue*KFD_VI_SDMA_QUEUE_OFFSET; let mut i=0; *dump=kmalloc_objs(core::ptr::null_mut(),35); if (*dump).is_null(){return -ENOMEM;} let mut put=|r:u32|{(**dump)[i][0]=r<<2;(**dump)[i][1]=RREG32(adev,r);i+=1;}; let mut r=mmSDMA0_RLC0_RB_CNTL;while r<=mmSDMA0_RLC0_DOORBELL{put(off+r);r+=1;}r=mmSDMA0_RLC0_VIRTUAL_ADDR;while r<=mmSDMA0_RLC0_WATERMARK{put(off+r);r+=1;}r=mmSDMA0_RLC0_CSA_ADDR_LO;while r<=mmSDMA0_RLC0_CSA_ADDR_HI{put(off+r);r+=1;}r=mmSDMA0_RLC0_IB_SUB_REMAIN;while r<=mmSDMA0_RLC0_DUMMY_REG{put(off+r);r+=1;}r=mmSDMA0_RLC0_MIDCMD_DATA0;while r<=mmSDMA0_RLC0_MIDCMD_CNTL{put(off+r);r+=1;}*n=i as u32;0 }
unsafe fn kgd_hqd_is_occupied(adev:*mut amdgpu_device, addr:u64, pipe:u32, queue:u32, _:u32)->bool { acquire_queue(adev,pipe,queue); let a=RREG32(adev,mmCP_HQD_ACTIVE)!=0; let r=a&&lower_32_bits(addr>>8)==RREG32(adev,mmCP_HQD_PQ_BASE)&&upper_32_bits(addr>>8)==RREG32(adev,mmCP_HQD_PQ_BASE_HI);release_queue(adev);r }
unsafe fn kgd_hqd_sdma_is_occupied(adev:*mut amdgpu_device, mqd:*mut core::ffi::c_void)->bool { let m=get_sdma_mqd(mqd);RREG32(adev,get_sdma_rlc_reg_offset(m)+mmSDMA0_RLC0_RB_CNTL)&SDMA0_RLC0_RB_CNTL__RB_ENABLE_MASK!=0 }
unsafe fn kgd_hqd_destroy(adev:*mut amdgpu_device, mqd:*mut core::ffi::c_void, reset:kfd_preempt_type, timeout:u32, pipe:u32, queue:u32, _:u32)->i32 { if amdgpu_in_reset(adev){return -EIO;} acquire_queue(adev,pipe,queue);let m=get_mqd(mqd);if (*m).cp_hqd_vmid==0{WREG32_FIELD(adev,RLC_CP_SCHEDULERS,scheduler1,0);}let typ=match reset{KFD_PREEMPT_TYPE_WAVEFRONT_RESET=>RESET_WAVES,_=>DRAIN_PIPE};WREG32(adev,mmCP_HQD_DEQUEUE_REQUEST,typ as u32);let end=(timeout*HZ/1000)+jiffies();loop{if RREG32(adev,mmCP_HQD_ACTIVE)&CP_HQD_ACTIVE__ACTIVE_MASK==0{break;}if time_after(jiffies(),end){release_queue(adev);return -ETIME;}usleep_range(500,1000);}release_queue(adev);0 }
unsafe fn kgd_hqd_sdma_destroy(adev:*mut amdgpu_device, mqd:*mut core::ffi::c_void, timeout:u32)->i32 {let m=get_sdma_mqd(mqd);let off=get_sdma_rlc_reg_offset(m);let end=timeout*HZ/1000+jiffies();let d=RREG32(adev,off+mmSDMA0_RLC0_RB_CNTL)&!SDMA0_RLC0_RB_CNTL__RB_ENABLE_MASK;WREG32(adev,off+mmSDMA0_RLC0_RB_CNTL,d);loop{if RREG32(adev,off+mmSDMA0_RLC0_CONTEXT_STATUS)&SDMA0_RLC0_CONTEXT_STATUS__IDLE_MASK!=0{break;}if time_after(jiffies(),end){return -ETIME;}usleep_range(500,1000);}WREG32(adev,off+mmSDMA0_RLC0_DOORBELL,0);WREG32(adev,off+mmSDMA0_RLC0_RB_CNTL,RREG32(adev,off+mmSDMA0_RLC0_RB_CNTL)|SDMA0_RLC0_RB_CNTL__RB_ENABLE_MASK);(*m).sdmax_rlcx_rb_rptr=RREG32(adev,off+mmSDMA0_RLC0_RB_RPTR);0}
unsafe fn get_atc_vmid_pasid_mapping_info(adev:*mut amdgpu_device, vmid:u8, p:*mut u16)->bool { let v=RREG32(adev,mmATC_VMID0_PASID_MAPPING+vmid as u32); *p=(v&ATC_VMID0_PASID_MAPPING__PASID_MASK) as u16; v&ATC_VMID0_PASID_MAPPING__VALID_MASK!=0 }
unsafe fn kgd_wave_control_execute(adev:*mut amdgpu_device, gfx:u32, sq:u32, _:u32)->i32 { mutex_lock(&mut (*adev).grbm_idx_mutex); WREG32(adev,mmGRBM_GFX_INDEX,gfx); WREG32(adev,mmSQ_CMD,sq); let mut d=0; d=REG_SET_FIELD(d,GRBM_GFX_INDEX,INSTANCE_BROADCAST_WRITES,1); d=REG_SET_FIELD(d,GRBM_GFX_INDEX,SH_BROADCAST_WRITES,1); d=REG_SET_FIELD(d,GRBM_GFX_INDEX,SE_BROADCAST_WRITES,1); WREG32(adev,mmGRBM_GFX_INDEX,d); mutex_unlock(&mut (*adev).grbm_idx_mutex); 0 }
unsafe fn set_scratch_backing_va(adev:*mut amdgpu_device, va:u64, vmid:u32){lock_srbm(adev,0,0,0,vmid);WREG32(adev,mmSH_HIDDEN_PRIVATE_BASE_VMID,va);unlock_srbm(adev)}
unsafe fn set_vm_context_page_table_base(adev:*mut amdgpu_device, vmid:u32, base:u64){if !amdgpu_amdkfd_is_kfd_vmid(adev,vmid){pr_err!("trying to set page table base for wrong VMID\n");return} WREG32(adev,mmVM_CONTEXT8_PAGE_TABLE_BASE_ADDR+vmid-8,lower_32_bits(base));}
unsafe fn kgd_hqd_sdma_get_doorbell(_: *mut amdgpu_device, _: i32, _: i32)->u32 {0}

pub static gfx_v8_kfd2kgd: kfd2kgd_calls = kfd2kgd_calls { program_sh_mem_settings:kgd_program_sh_mem_settings, set_pasid_vmid_mapping:kgd_set_pasid_vmid_mapping, init_interrupts:kgd_init_interrupts, hqd_load:kgd_hqd_load, hqd_sdma_load:kgd_hqd_sdma_load, hqd_dump:kgd_hqd_dump, hqd_sdma_dump:kgd_hqd_sdma_dump, hqd_is_occupied:kgd_hqd_is_occupied, hqd_sdma_is_occupied:kgd_hqd_sdma_is_occupied, hqd_destroy:kgd_hqd_destroy, hqd_sdma_destroy:kgd_hqd_sdma_destroy, wave_control_execute:kgd_wave_control_execute, get_atc_vmid_pasid_mapping_info:get_atc_vmid_pasid_mapping_info, set_scratch_backing_va:set_scratch_backing_va, set_vm_context_page_table_base:set_vm_context_page_table_base, hqd_sdma_get_doorbell:kgd_hqd_sdma_get_doorbell };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
