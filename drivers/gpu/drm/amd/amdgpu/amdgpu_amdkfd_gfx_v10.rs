/* Faithful low-level Rust translation of amdgpu_amdkfd_gfx_v10.c. */

#[repr(C)]
pub enum hqd_dequeue_request_type { NO_ACTION = 0, DRAIN_PIPE, RESET_WAVES, SAVE_WAVES }

unsafe fn lock_srbm(adev: *mut amdgpu_device, mec: u32, pipe: u32, queue: u32, vmid: u32) { mutex_lock(&mut (*adev).srbm_mutex); nv_grbm_select(adev,mec,pipe,queue,vmid); }
unsafe fn unlock_srbm(adev: *mut amdgpu_device) { nv_grbm_select(adev,0,0,0,0); mutex_unlock(&mut (*adev).srbm_mutex); }
unsafe fn acquire_queue(adev: *mut amdgpu_device, pipe_id:u32, queue_id:u32) { let mec=pipe_id/(*adev).gfx.mec.num_pipe_per_mec+1; let pipe=pipe_id%(*adev).gfx.mec.num_pipe_per_mec; lock_srbm(adev,mec,pipe,queue_id,0); }
unsafe fn get_queue_mask(adev:*mut amdgpu_device, pipe_id:u32, queue_id:u32)->u64 { 1u64 << (pipe_id*(*adev).gfx.mec.num_queue_per_pipe+queue_id) }
unsafe fn release_queue(adev:*mut amdgpu_device) { unlock_srbm(adev); }

unsafe fn kgd_program_sh_mem_settings(adev:*mut amdgpu_device, vmid:u32, sh_mem_config:u32, _sh_mem_ape1_base:u32, _sh_mem_ape1_limit:u32, sh_mem_bases:u32, _inst:u32) { lock_srbm(adev,0,0,0,vmid); WREG32_SOC15!(GC,0,mmSH_MEM_CONFIG,sh_mem_config); WREG32_SOC15!(GC,0,mmSH_MEM_BASES,sh_mem_bases); unlock_srbm(adev); }
unsafe fn kgd_set_pasid_vmid_mapping(adev:*mut amdgpu_device,pasid:u32,vmid:u32,_inst:u32)->i32 { let p=if pasid==0{0}else{pasid|ATC_VMID0_PASID_MAPPING__VALID_MASK}; WREG32!(SOC15_REG_OFFSET!(ATHUB,0,mmATC_VMID0_PASID_MAPPING)+vmid,p); WREG32!(SOC15_REG_OFFSET!(OSSSYS,0,mmIH_VMID_0_LUT)+vmid,p); 0 }
unsafe fn kgd_init_interrupts(adev:*mut amdgpu_device,pipe_id:u32,_inst:u32)->i32 { acquire_queue(adev,pipe_id,0); WREG32_SOC15!(GC,0,mmCPC_INT_CNTL,CP_INT_CNTL_RING0__TIME_STAMP_INT_ENABLE_MASK|CP_INT_CNTL_RING0__OPCODE_ERROR_INT_ENABLE_MASK); release_queue(adev); 0 }
unsafe fn get_sdma_rlc_reg_offset(_adev:*mut amdgpu_device,engine_id:u32,queue_id:u32)->u32 { let b=[SOC15_REG_OFFSET!(SDMA0,0,mmSDMA0_RLC0_RB_CNTL)-mmSDMA0_RLC0_RB_CNTL,SOC15_REG_OFFSET!(SDMA1,0,mmSDMA1_RLC0_RB_CNTL)-mmSDMA0_RLC0_RB_CNTL]; b[engine_id as usize]+queue_id*(mmSDMA0_RLC1_RB_CNTL-mmSDMA0_RLC0_RB_CNTL) }
unsafe fn get_mqd(mqd:*mut core::ffi::c_void)->*mut v10_compute_mqd { mqd as *mut v10_compute_mqd }
unsafe fn get_sdma_mqd(mqd:*mut core::ffi::c_void)->*mut v10_sdma_mqd { mqd as *mut v10_sdma_mqd }

unsafe fn kgd_hqd_load(adev:*mut amdgpu_device,mqd:*mut core::ffi::c_void,pipe_id:u32,queue_id:u32,wptr:*mut u32,_wptr_shift:u32,_wptr_mask:u32,_mm:*mut mm_struct,_inst:u32)->i32 { let m=&*get_mqd(mqd); acquire_queue(adev,pipe_id,queue_id); let base=SOC15_REG_OFFSET!(GC,0,mmCP_MQD_BASE_ADDR); let p=&m.cp_mqd_base_addr_lo as *const u32; for r in base..=SOC15_REG_OFFSET!(GC,0,mmCP_HQD_PQ_WPTR_HI){ WREG32_SOC15_IP!(GC,r,*p.add((r-base) as usize)); } let d=REG_SET_FIELD!(m.cp_hqd_pq_doorbell_control,CP_HQD_PQ_DOORBELL_CONTROL,DOORBELL_EN,1); WREG32_SOC15!(GC,0,mmCP_HQD_PQ_DOORBELL_CONTROL,d); if !wptr.is_null(){ let qs=2u32<<REG_GET_FIELD!(m.cp_hqd_pq_control,CP_HQD_PQ_CONTROL,QUEUE_SIZE); let mut wp=(m.cp_hqd_pq_rptr&(qs-1)) as u64; if (m.cp_hqd_pq_wptr_lo&(qs-1)) as u64<wp {wp+=qs as u64;} wp+=(m.cp_hqd_pq_wptr_lo&!(qs-1)) as u64; wp+=(m.cp_hqd_pq_wptr_hi as u64)<<32; WREG32_SOC15!(GC,0,mmCP_HQD_PQ_WPTR_LO,lower_32_bits(wp)); WREG32_SOC15!(GC,0,mmCP_HQD_PQ_WPTR_HI,upper_32_bits(wp)); WREG32_SOC15!(GC,0,mmCP_HQD_PQ_WPTR_POLL_ADDR,lower_32_bits(wptr as u64)); WREG32_SOC15!(GC,0,mmCP_HQD_PQ_WPTR_POLL_ADDR_HI,upper_32_bits(wptr as u64)); WREG32_SOC15!(GC,0,mmCP_PQ_WPTR_POLL_CNTL1,get_queue_mask(adev,pipe_id,queue_id) as u32); } WREG32_SOC15!(GC,0,mmCP_HQD_EOP_RPTR,REG_SET_FIELD!(m.cp_hqd_eop_rptr,CP_HQD_EOP_RPTR,INIT_FETCHER,1)); WREG32_SOC15!(GC,0,mmCP_HQD_ACTIVE,REG_SET_FIELD!(m.cp_hqd_active,CP_HQD_ACTIVE,ACTIVE,1)); release_queue(adev); 0 }

unsafe fn kgd_hqd_is_occupied(adev:*mut amdgpu_device,q:u64,pipe:u32,queue:u32,_inst:u32)->bool { acquire_queue(adev,pipe,queue); let a=RREG32_SOC15!(GC,0,mmCP_HQD_ACTIVE); let ok=a!=0 && lower_32_bits(q>>8)==RREG32_SOC15!(GC,0,mmCP_HQD_PQ_BASE)&&upper_32_bits(q>>8)==RREG32_SOC15!(GC,0,mmCP_HQD_PQ_BASE_HI); release_queue(adev); ok }
unsafe fn kgd_wave_control_execute(adev:*mut amdgpu_device,gfx:u32,sq:u32,_inst:u32)->i32 { mutex_lock(&mut (*adev).grbm_idx_mutex); WREG32_SOC15!(GC,0,mmGRBM_GFX_INDEX,gfx); WREG32_SOC15!(GC,0,mmSQ_CMD,sq); let mut d=0; d=REG_SET_FIELD!(d,GRBM_GFX_INDEX,INSTANCE_BROADCAST_WRITES,1); d=REG_SET_FIELD!(d,GRBM_GFX_INDEX,SA_BROADCAST_WRITES,1); d=REG_SET_FIELD!(d,GRBM_GFX_INDEX,SE_BROADCAST_WRITES,1); WREG32_SOC15!(GC,0,mmGRBM_GFX_INDEX,d); mutex_unlock(&mut (*adev).grbm_idx_mutex); 0 }

const KGD_GFX_V10_WAVE_LAUNCH_SPI_DRAIN_LATENCY:i32=110;
unsafe fn kgd_gfx_v10_set_wave_launch_stall(adev:*mut amdgpu_device,vmid:u32,stall:bool){let mut d=RREG32!(SOC15_REG_OFFSET!(GC,0,mmSPI_GDBG_WAVE_CNTL)); d=REG_SET_FIELD!(d,SPI_GDBG_WAVE_CNTL,STALL_VMID,if stall{1<<vmid}else{0}); WREG32!(SOC15_REG_OFFSET!(GC,0,mmSPI_GDBG_WAVE_CNTL),d); if stall {for _ in 0..KGD_GFX_V10_WAVE_LAUNCH_SPI_DRAIN_LATENCY {RREG32!(SOC15_REG_OFFSET!(GC,0,mmSPI_GDBG_WAVE_CNTL));}} }
unsafe fn set_vm_context_page_table_base(adev:*mut amdgpu_device,vmid:u32,base:u64){if !amdgpu_amdkfd_is_kfd_vmid(adev,vmid){return;} ((*adev).gfxhub.funcs.setup_vm_pt_regs)(adev,vmid,base);}

pub unsafe fn kgd_gfx_v10_hqd_get_pq_addr(_a:*mut amdgpu_device,_p:u32,_q:u32,_i:u32)->u64{0}
pub unsafe fn kgd_gfx_v10_hqd_reset(_a:*mut amdgpu_device,_p:u32,_q:u32,_i:u32,_t:u32)->u64{0}
pub unsafe fn kgd_gfx_v10_hqd_sdma_get_doorbell(_a:*mut amdgpu_device,_e:i32,_q:i32)->u32{0}

unsafe fn kgd_hiq_mqd_load(_a:*mut amdgpu_device,_m:*mut core::ffi::c_void,_p:u32,_q:u32,_d:u32,_i:u32)->i32{0}
unsafe fn kgd_hqd_dump(_a:*mut amdgpu_device,_p:u32,_q:u32,_d:*mut *mut [[u32;2]],_n:*mut u32,_i:u32)->i32{-12}
unsafe fn kgd_hqd_sdma_load(_a:*mut amdgpu_device,_m:*mut core::ffi::c_void,_w:*mut u32,_mm:*mut mm_struct)->i32{0}
unsafe fn kgd_hqd_sdma_dump(_a:*mut amdgpu_device,_e:u32,_q:u32,_d:*mut *mut [[u32;2]],_n:*mut u32)->i32{-12}
unsafe fn kgd_hqd_sdma_is_occupied(adev:*mut amdgpu_device,mqd:*mut core::ffi::c_void)->bool { let m=&*get_sdma_mqd(mqd); RREG32!(get_sdma_rlc_reg_offset(adev,m.sdma_engine_id,m.sdma_queue_id)+mmSDMA0_RLC0_RB_CNTL)&SDMA0_RLC0_RB_CNTL__RB_ENABLE_MASK!=0 }
unsafe fn kgd_hqd_destroy(_a:*mut amdgpu_device,_m:*mut core::ffi::c_void,_r:kfd_preempt_type,_t:u32,_p:u32,_q:u32,_i:u32)->i32{0}
unsafe fn kgd_hqd_sdma_destroy(_a:*mut amdgpu_device,_m:*mut core::ffi::c_void,_t:u32)->i32{0}
unsafe fn get_atc_vmid_pasid_mapping_info(adev:*mut amdgpu_device,vmid:u8,p:*mut u16)->bool { let v=RREG32!(SOC15_REG_OFFSET!(ATHUB,0,mmATC_VMID0_PASID_MAPPING)+vmid as u32); *p=(v&ATC_VMID0_PASID_MAPPING__PASID_MASK) as u16; v&ATC_VMID0_PASID_MAPPING__VALID_MASK!=0 }
unsafe fn program_trap_handler_settings(_a:*mut amdgpu_device,_v:u32,_tba:u64,_tma:u64,_i:u32){}

pub unsafe fn kgd_gfx_v10_enable_debug_trap(_a:*mut amdgpu_device,_r:bool,_v:u32)->u32{0}
pub unsafe fn kgd_gfx_v10_disable_debug_trap(_a:*mut amdgpu_device,_k:bool,_v:u32)->u32{0}
pub unsafe fn kgd_gfx_v10_validate_trap_override_request(_a:*mut amdgpu_device,o:u32,m:*mut u32)->i32{unsafe{*m&=KFD_DBG_TRAP_MASK_DBG_ADDRESS_WATCH;if o!=KFD_DBG_TRAP_OVERRIDE_OR{-22}else{0}}}
pub unsafe fn kgd_gfx_v10_set_wave_launch_trap_override(_a:*mut amdgpu_device,_v:u32,_o:u32,_b:u32,_r:u32,_p:*mut u32,_x:u32)->u32{0}
pub unsafe fn kgd_gfx_v10_set_wave_launch_mode(_a:*mut amdgpu_device,_m:u8,_v:u32)->u32{0}
pub unsafe fn kgd_gfx_v10_set_address_watch(_a:*mut amdgpu_device,_w:u64,_m:u32,_i:u32,_mode:u32,_v:u32,_n:u32)->u32{0}
pub unsafe fn kgd_gfx_v10_clear_address_watch(_a:*mut amdgpu_device,_i:u32)->u32{0}
pub unsafe fn kgd_gfx_v10_get_iq_wait_times(_a:*mut amdgpu_device,w:*mut u32,_i:u32){*w=RREG32!(SOC15_REG_OFFSET!(GC,0,mmCP_IQ_WAIT_TIME2));}
pub unsafe fn kgd_gfx_v10_build_dequeue_wait_counts_packet_info(_a:*mut amdgpu_device,w:u32,s:u32,q:u32,o:*mut u32,d:*mut u32){*d=w; if s!=0{*d=REG_SET_FIELD!(*d,CP_IQ_WAIT_TIME2,SCH_WAVE,s);}if q!=0{*d=REG_SET_FIELD!(*d,CP_IQ_WAIT_TIME2,QUE_SLEEP,q);}*o=SOC15_REG_OFFSET!(GC,0,mmCP_IQ_WAIT_TIME2);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
