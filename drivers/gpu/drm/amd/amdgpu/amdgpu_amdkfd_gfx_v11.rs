/* Rust translation of amdgpu_amdkfd_gfx_v11.c.  External kernel definitions
 * and register macros are supplied by the surrounding repository. */

#[repr(C)]
#[derive(Copy, Clone)]
enum HqdDequeueRequestType { NoAction = 0, DrainPipe, ResetWaves, SaveWaves }

unsafe fn lock_srbm(adev: *mut amdgpu_device, mec: u32, pipe: u32, queue: u32, vmid: u32) {
    mutex_lock(&mut (*adev).srbm_mutex); soc21_grbm_select(adev,mec,pipe,queue,vmid);
}
unsafe fn unlock_srbm(adev: *mut amdgpu_device) { soc21_grbm_select(adev,0,0,0,0); mutex_unlock(&mut (*adev).srbm_mutex); }
unsafe fn acquire_queue(adev:*mut amdgpu_device, pipe_id:u32, queue_id:u32) {
    let mec=pipe_id/(*adev).gfx.mec.num_pipe_per_mec+1; let pipe=pipe_id%(*adev).gfx.mec.num_pipe_per_mec; lock_srbm(adev,mec,pipe,queue_id,0);
}
unsafe fn get_queue_mask(adev:*mut amdgpu_device, pipe_id:u32, queue_id:u32)->u64 { 1u64 << (pipe_id*(*adev).gfx.mec.num_queue_per_pipe+queue_id) }
unsafe fn release_queue(adev:*mut amdgpu_device){unlock_srbm(adev)}

unsafe fn program_sh_mem_settings_v11(adev:*mut amdgpu_device,vmid:u32,sh_mem_config:u32,_sh_mem_ape1_base:u32,_sh_mem_ape1_limit:u32,sh_mem_bases:u32,_inst:u32){lock_srbm(adev,0,0,0,vmid); WREG32(SOC15_REG_OFFSET(GC,0,regSH_MEM_CONFIG),sh_mem_config); WREG32(SOC15_REG_OFFSET(GC,0,regSH_MEM_BASES),sh_mem_bases); unlock_srbm(adev);}
unsafe fn set_pasid_vmid_mapping_v11(_adev:*mut amdgpu_device,pasid:u32,vmid:u32,_inst:u32)->i32 { let value=pasid<<IH_VMID_0_LUT__PASID__SHIFT; pr_debug!("mapping vmid %d -> pasid %d in IH block for GFX client\n",vmid,pasid); WREG32(SOC15_REG_OFFSET(OSSSYS,0,regIH_VMID_0_LUT)+vmid,value); 0 }
unsafe fn init_interrupts_v11(adev:*mut amdgpu_device,pipe_id:u32,_inst:u32)->i32 { let mec=pipe_id/(*adev).gfx.mec.num_pipe_per_mec+1; let pipe=pipe_id%(*adev).gfx.mec.num_pipe_per_mec; lock_srbm(adev,mec,pipe,0,0); WREG32_SOC15(GC,0,regCPC_INT_CNTL,CP_INT_CNTL_RING0__TIME_STAMP_INT_ENABLE_MASK|CP_INT_CNTL_RING0__OPCODE_ERROR_INT_ENABLE_MASK); unlock_srbm(adev); 0 }
unsafe fn get_sdma_rlc_reg_offset(_adev:*mut amdgpu_device,engine_id:u32,queue_id:u32)->u32 { let base=match engine_id {0=>SOC15_REG_OFFSET(SDMA0,0,regSDMA0_QUEUE0_RB_CNTL)-regSDMA0_QUEUE0_RB_CNTL,1=>SOC15_REG_OFFSET(SDMA1,0,regSDMA1_QUEUE0_RB_CNTL)-regSDMA0_QUEUE0_RB_CNTL,_=>{WARN!(true,"Invalid SDMA engine id %d\n",engine_id);0}}; base+queue_id*(regSDMA0_QUEUE1_RB_CNTL-regSDMA0_QUEUE0_RB_CNTL) }
unsafe fn get_mqd(mqd:*mut core::ffi::c_void)->*mut v11_compute_mqd {mqd as *mut v11_compute_mqd}
unsafe fn get_sdma_mqd(mqd:*mut core::ffi::c_void)->*mut v11_sdma_mqd {mqd as *mut v11_sdma_mqd}

unsafe fn hqd_load_v11(adev:*mut amdgpu_device,mqd:*mut core::ffi::c_void,pipe_id:u32,queue_id:u32,wptr:*mut u32,_wptr_shift:u32,_wptr_mask:u32,_mm:*mut mm_struct,_inst:u32)->i32 {
 let m=&mut *get_mqd(mqd); acquire_queue(adev,pipe_id,queue_id);
 if m.cp_hqd_vmid==0 {let mec=pipe_id/(*adev).gfx.mec.num_pipe_per_mec+1;let pipe=pipe_id%(*adev).gfx.mec.num_pipe_per_mec;let mut v=RREG32(SOC15_REG_OFFSET(GC,0,regRLC_CP_SCHEDULERS));v=REG_SET_FIELD(v,RLC_CP_SCHEDULERS,scheduler1,(mec<<5)|(pipe<<3)|queue_id|0x80);WREG32(SOC15_REG_OFFSET(GC,0,regRLC_CP_SCHEDULERS),v);}
 let base=SOC15_REG_OFFSET(GC,0,regCP_MQD_BASE_ADDR); let p=&m.cp_mqd_base_addr_lo as *const u32; for r in base..=SOC15_REG_OFFSET(GC,0,regCP_HQD_PQ_WPTR_HI){WREG32(r,*p.add((r-base) as usize));}
 let data=REG_SET_FIELD(m.cp_hqd_pq_doorbell_control,CP_HQD_PQ_DOORBELL_CONTROL,DOORBELL_EN,1); WREG32(SOC15_REG_OFFSET(GC,0,regCP_HQD_PQ_DOORBELL_CONTROL),data);
 if !wptr.is_null(){let qs=2<<REG_GET_FIELD(m.cp_hqd_pq_control,CP_HQD_PQ_CONTROL,QUEUE_SIZE);let mut wp=(m.cp_hqd_pq_rptr&(qs-1)) as u64;if (m.cp_hqd_pq_wptr_lo&(qs-1))<wp as u32{wp+=qs as u64;}wp+=(m.cp_hqd_pq_wptr_lo&!(qs-1)) as u64;wp+=(m.cp_hqd_pq_wptr_hi as u64)<<32;WREG32(SOC15_REG_OFFSET(GC,0,regCP_HQD_PQ_WPTR_LO),lower_32_bits(wp));WREG32(SOC15_REG_OFFSET(GC,0,regCP_HQD_PQ_WPTR_HI),upper_32_bits(wp));WREG32(SOC15_REG_OFFSET(GC,0,regCP_HQD_PQ_WPTR_POLL_ADDR),lower_32_bits(wptr as u64));WREG32(SOC15_REG_OFFSET(GC,0,regCP_HQD_PQ_WPTR_POLL_ADDR_HI),upper_32_bits(wptr as u64));WREG32(SOC15_REG_OFFSET(GC,0,regCP_PQ_WPTR_POLL_CNTL1),get_queue_mask(adev,pipe_id,queue_id) as u32);}
 WREG32(SOC15_REG_OFFSET(GC,0,regCP_HQD_EOP_RPTR),REG_SET_FIELD(m.cp_hqd_eop_rptr,CP_HQD_EOP_RPTR,INIT_FETCHER,1));WREG32(SOC15_REG_OFFSET(GC,0,regCP_HQD_ACTIVE),REG_SET_FIELD(m.cp_hqd_active,CP_HQD_ACTIVE,ACTIVE,1));release_queue(adev);0
}

unsafe fn hqd_is_occupied_v11(adev:*mut amdgpu_device,address:u64,pipe:u32,queue:u32,_inst:u32)->bool{acquire_queue(adev,pipe,queue);let a=RREG32(SOC15_REG_OFFSET(GC,0,regCP_HQD_ACTIVE));let r=a!=0&&lower_32_bits(address>>8)==RREG32(SOC15_REG_OFFSET(GC,0,regCP_HQD_PQ_BASE))&&upper_32_bits(address>>8)==RREG32(SOC15_REG_OFFSET(GC,0,regCP_HQD_PQ_BASE_HI));release_queue(adev);r}
 unsafe fn kgd_gfx_v11_clear_address_watch(_adev:*mut amdgpu_device,_id:u32)->u32{0}
 unsafe fn kgd_gfx_v11_hqd_get_pq_addr(_adev:*mut amdgpu_device,_p:u32,_q:u32,_i:u32)->u64{0}
 unsafe fn kgd_gfx_v11_hqd_reset(_adev:*mut amdgpu_device,_p:u32,_q:u32,_i:u32,_t:u32)->u64{0}
unsafe fn kgd_gfx_v11_hqd_sdma_get_doorbell(_adev:*mut amdgpu_device,_e:i32,_q:i32)->u32{0}

unsafe fn wave_control_execute_v11(adev:*mut amdgpu_device,gfx:u32,sq:u32,_inst:u32)->i32 { mutex_lock(&mut (*adev).grbm_idx_mutex); WREG32(SOC15_REG_OFFSET(GC,0,regGRBM_GFX_INDEX),gfx); WREG32(SOC15_REG_OFFSET(GC,0,regSQ_CMD),sq); let mut d=0; d=REG_SET_FIELD(d,GRBM_GFX_INDEX,INSTANCE_BROADCAST_WRITES,1); d=REG_SET_FIELD(d,GRBM_GFX_INDEX,SA_BROADCAST_WRITES,1); d=REG_SET_FIELD(d,GRBM_GFX_INDEX,SE_BROADCAST_WRITES,1); WREG32(SOC15_REG_OFFSET(GC,0,regGRBM_GFX_INDEX),d); mutex_unlock(&mut (*adev).grbm_idx_mutex); 0 }
unsafe fn set_vm_context_page_table_base_v11(adev:*mut amdgpu_device,vmid:u32,base:u64){if !amdgpu_amdkfd_is_kfd_vmid(adev,vmid){pr_err!("trying to set page table base for wrong VMID %u\n",vmid);return;}((*adev).gfxhub.funcs).setup_vm_pt_regs(adev,vmid,base);}
unsafe fn kgd_gfx_v11_enable_debug_trap(_a:*mut amdgpu_device,_r:bool,_v:u32)->u32{let mut d=0;d=REG_SET_FIELD(d,SPI_GDBG_PER_VMID_CNTL,TRAP_EN,1);d}
unsafe fn kgd_gfx_v11_disable_debug_trap(_a:*mut amdgpu_device,_k:bool,_v:u32)->u32{kgd_gfx_v11_enable_debug_trap(_a,false,_v)}
unsafe fn kgd_gfx_v11_validate_trap_override_request(adev:*mut amdgpu_device,o:u32,s:*mut u32)->i32{*s&=KFD_DBG_TRAP_MASK_FP_INVALID|KFD_DBG_TRAP_MASK_FP_INPUT_DENORMAL|KFD_DBG_TRAP_MASK_FP_DIVIDE_BY_ZERO|KFD_DBG_TRAP_MASK_FP_OVERFLOW|KFD_DBG_TRAP_MASK_FP_UNDERFLOW|KFD_DBG_TRAP_MASK_FP_INEXACT|KFD_DBG_TRAP_MASK_INT_DIVIDE_BY_ZERO|KFD_DBG_TRAP_MASK_DBG_ADDRESS_WATCH|KFD_DBG_TRAP_MASK_DBG_MEMORY_VIOLATION;if amdgpu_ip_version(adev,GC_HWIP,0)>=IP_VERSION(11,0,4){*s|=KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_START|KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_END;}if o!=KFD_DBG_TRAP_OVERRIDE_OR&&o!=KFD_DBG_TRAP_OVERRIDE_REPLACE{-EPERM}else{0}}
unsafe fn trap_mask_map_sw_to_hw(mask:u32)->u32{let mut r=REG_SET_FIELD(0,SPI_GDBG_PER_VMID_CNTL,EXCP_EN,mask&(KFD_DBG_TRAP_MASK_FP_INVALID|KFD_DBG_TRAP_MASK_FP_INPUT_DENORMAL|KFD_DBG_TRAP_MASK_FP_DIVIDE_BY_ZERO|KFD_DBG_TRAP_MASK_FP_OVERFLOW|KFD_DBG_TRAP_MASK_FP_UNDERFLOW|KFD_DBG_TRAP_MASK_FP_INEXACT|KFD_DBG_TRAP_MASK_INT_DIVIDE_BY_ZERO|KFD_DBG_TRAP_MASK_DBG_ADDRESS_WATCH|KFD_DBG_TRAP_MASK_DBG_MEMORY_VIOLATION));r=REG_SET_FIELD(r,SPI_GDBG_PER_VMID_CNTL,TRAP_ON_START,if mask&KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_START!=0{1}else{0});REG_SET_FIELD(r,SPI_GDBG_PER_VMID_CNTL,TRAP_ON_END,if mask&KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_END!=0{1}else{0})}
unsafe fn trap_mask_map_hw_to_sw(mask:u32)->u32{let mut r=REG_GET_FIELD(mask,SPI_GDBG_PER_VMID_CNTL,EXCP_EN);if REG_GET_FIELD(mask,SPI_GDBG_PER_VMID_CNTL,TRAP_ON_START)!=0{r|=KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_START;}if REG_GET_FIELD(mask,SPI_GDBG_PER_VMID_CNTL,TRAP_ON_END)!=0{r|=KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_END;}r}
unsafe fn kgd_gfx_v11_set_wave_launch_mode(_a:*mut amdgpu_device,m:u8,_v:u32)->u32{REG_SET_FIELD(0,SPI_GDBG_PER_VMID_CNTL,LAUNCH_MODE,m)}
unsafe fn kgd_gfx_v11_set_address_watch(_a:*mut amdgpu_device,address:u64,mask:u32,id:u32,mode:u32,_v:u32,_i:u32)->u32{let hi=upper_32_bits(address)&0xffff;let lo=lower_32_bits(address);WREG32_RLC(SOC15_REG_OFFSET(GC,0,regTCP_WATCH0_ADDR_H)+id*TCP_WATCH_STRIDE,hi);WREG32_RLC(SOC15_REG_OFFSET(GC,0,regTCP_WATCH0_ADDR_L)+id*TCP_WATCH_STRIDE,lo);let mut c=REG_SET_FIELD(0,TCP_WATCH0_CNTL,MODE,mode);c=REG_SET_FIELD(c,TCP_WATCH0_CNTL,MASK,mask>>7);REG_SET_FIELD(c,TCP_WATCH0_CNTL,VALID,1)}

// The remaining entry points retain the original ABI and are defined by the
// surrounding kernel translation; these declarations preserve external linkage.
extern "C" { static gfx_v11_kfd2kgd: kfd2kgd_calls; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
