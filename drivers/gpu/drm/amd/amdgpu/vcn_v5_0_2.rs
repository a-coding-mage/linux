/* Direct source-level translation of vcn_v5_0_2.c. */

// C headers and symbols are supplied by the surrounding kernel translation.

unsafe fn vcn_v5_0_2_early_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev; let mut i: i32; let mut r: i32;
    i = 0; while i < (*adev).vcn.num_vcn_inst { (*adev).vcn.inst[i as usize].num_enc_rings = 1; i += 1; }
    vcn_v5_0_2_set_unified_ring_funcs(adev); vcn_v5_0_2_set_irq_funcs(adev);
    i = 0; while i < (*adev).vcn.num_vcn_inst { (*adev).vcn.inst[i as usize].set_pg_state = Some(vcn_v5_0_2_set_pg_state); r = amdgpu_vcn_early_init(adev, i); if r != 0 { return r; } i += 1; } 0
}

unsafe fn vcn_v5_0_2_fw_shared_init(adev: *mut amdgpu_device, inst_idx: i32) {
    let fw_shared = (*adev).vcn.inst[inst_idx as usize].fw_shared.cpu_addr as *mut amdgpu_vcn5_fw_shared;
    if (*fw_shared).sq.is_enabled != 0 { return; }
    (*fw_shared).present_flag_0 = cpu_to_le32(AMDGPU_FW_SHARED_FLAG_0_UNIFIED_QUEUE); (*fw_shared).sq.is_enabled = 1;
    if amdgpu_vcnfw_log { amdgpu_vcn_fwlog_init(&mut (*adev).vcn.inst[inst_idx as usize]); }
}

unsafe fn vcn_v5_0_2_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev=(*ip_block).adev; let mut r=amdgpu_irq_add_id(adev,SOC_V1_0_IH_CLIENTID_VCN,VCN_5_0__SRCID__UVD_ENC_GENERAL_PURPOSE,&mut (*adev).vcn.inst[0].irq); if r!=0{return r;}
    let mut i=0; while i<(*adev).vcn.num_vcn_inst { let vcn_inst=GET_INST(VCN,i); r=amdgpu_vcn_sw_init(adev,i); if r!=0{return r;} amdgpu_vcn_setup_ucode(adev,i); r=amdgpu_vcn_resume(adev,i); if r!=0{return r;}
        let ring=&mut (*adev).vcn.inst[i as usize].ring_enc[0]; ring.use_doorbell=true; ring.doorbell_index=((*adev).doorbell_index.vcn.vcn_ring0_1<<1)+32*vcn_inst; ring.vm_hub=AMDGPU_MMHUB0((*adev).vcn.inst[i as usize].aid_id); sprintf(ring.name.as_mut_ptr(),b"vcn_unified_%d\0".as_ptr(),(*adev).vcn.inst[i as usize].aid_id);
        r=amdgpu_ring_init(adev,ring,512,&mut (*adev).vcn.inst[i as usize].irq,0,AMDGPU_RING_PRIO_DEFAULT,&mut (*adev).vcn.inst[i as usize].sched_score); if r!=0{return r;} vcn_v5_0_2_fw_shared_init(adev,i); i+=1; }
    (*adev).vcn.supported_reset=amdgpu_get_soft_full_reset_mask(&mut (*adev).vcn.inst[0].ring_enc[0]); amdgpu_vcn_sysfs_reset_mask_init(adev)
}

unsafe fn vcn_v5_0_2_sw_fini(ip_block:*mut amdgpu_ip_block)->i32 { let adev=(*ip_block).adev; let mut idx=0; if drm_dev_enter(adev_to_drm(adev),&mut idx) { let mut i=0; while i<(*adev).vcn.num_vcn_inst { let f=(*adev).vcn.inst[i as usize].fw_shared.cpu_addr as *mut amdgpu_vcn5_fw_shared; (*f).present_flag_0=0;(*f).sq.is_enabled=0;i+=1;} drm_dev_exit(idx); } let mut i=0; while i<(*adev).vcn.num_vcn_inst { let r=amdgpu_vcn_suspend(adev,i);if r!=0{return r;}i+=1;} i=0;while i<(*adev).vcn.num_vcn_inst{amdgpu_vcn_sw_fini(adev,i);i+=1;} amdgpu_vcn_sysfs_reset_mask_fini(adev);kfree((*adev).vcn.ip_dump);0 }

unsafe fn vcn_v5_0_2_hw_init(ip_block:*mut amdgpu_ip_block)->i32 { let adev=(*ip_block).adev; if RREG32_SOC15(VCN,GET_INST(VCN,0),regVCN_RRMT_CNTL)&0x200!=0{(*adev).vcn.caps|=AMDGPU_VCN_CAPS(RRMT_ENABLED);} let mut i=0;while i<(*adev).vcn.num_vcn_inst{let vi=GET_INST(VCN,i);let ring=&mut (*adev).vcn.inst[i as usize].ring_enc[0];let mut t=RREG32_SOC15(VCN,vi,regUVD_POWER_STATUS);t&=!UVD_POWER_STATUS__UVD_POWER_STATUS_MASK;WREG32_SOC15(VCN,vi,regUVD_POWER_STATUS,t);if ring.use_doorbell{(*adev).nbio.funcs.vcn_doorbell_range(adev,ring.use_doorbell,((*adev).doorbell_index.vcn.vcn_ring0_1<<1)+11*vi,(*adev).vcn.inst[i as usize].aid_id);}vcn_v5_0_2_fw_shared_init(adev,i);let r=amdgpu_ring_test_helper(ring);if r!=0{return r;}i+=1;}0 }
unsafe fn vcn_v5_0_2_hw_fini(ip_block:*mut amdgpu_ip_block)->i32{let adev=(*ip_block).adev;let mut i=0;while i<(*adev).vcn.num_vcn_inst{let v=&mut (*adev).vcn.inst[i as usize];cancel_delayed_work_sync(&mut v.idle_work);if v.cur_state!=AMDGPU_PG_STATE_GATE{(v.set_pg_state.unwrap())(v,AMDGPU_PG_STATE_GATE);}i+=1;}0}
unsafe fn vcn_v5_0_2_suspend(ip:*mut amdgpu_ip_block)->i32{let mut r=vcn_v5_0_2_hw_fini(ip);if r!=0{return r;}let mut i=0;while i<(*(*ip).adev).vcn.num_vcn_inst{r=amdgpu_vcn_suspend((*ip).adev,i);if r!=0{return r;}i+=1;}r}
unsafe fn vcn_v5_0_2_resume(ip:*mut amdgpu_ip_block)->i32{let adev=(*ip).adev;let mut i=0;while i<(*adev).vcn.num_vcn_inst{let v=&mut (*adev).vcn.inst[i as usize];if amdgpu_in_reset(adev){v.cur_state=AMDGPU_PG_STATE_GATE;}let r=amdgpu_vcn_resume(adev,i);if r!=0{return r;}i+=1;}vcn_v5_0_2_hw_init(ip)}

unsafe fn vcn_v5_0_2_mc_resume(v:*mut amdgpu_vcn_inst){let a=(*v).adev;let i=(*v).inst;let h=(*a).vcn.inst[i as usize].fw.data as *const common_firmware_header;let size=AMDGPU_GPU_PAGE_ALIGN(le32_to_cpu((*h).ucode_size_bytes)+8);let vi=GET_INST(VCN,i);let mut off=0;if (*a).firmware.load_type==AMDGPU_FW_LOAD_PSP{WREG32_SOC15(VCN,vi,regUVD_LMI_VCPU_CACHE_64BIT_BAR_LOW,(*a).firmware.ucode[(AMDGPU_UCODE_ID_VCN+i) as usize].tmr_mc_addr_lo);WREG32_SOC15(VCN,vi,regUVD_LMI_VCPU_CACHE_64BIT_BAR_HIGH,(*a).firmware.ucode[(AMDGPU_UCODE_ID_VCN+i) as usize].tmr_mc_addr_hi);WREG32_SOC15(VCN,vi,regUVD_VCPU_CACHE_OFFSET0,0);}else{WREG32_SOC15(VCN,vi,regUVD_LMI_VCPU_CACHE_64BIT_BAR_LOW,lower_32_bits((*a).vcn.inst[i as usize].gpu_addr));WREG32_SOC15(VCN,vi,regUVD_LMI_VCPU_CACHE_64BIT_BAR_HIGH,upper_32_bits((*a).vcn.inst[i as usize].gpu_addr));off=size;WREG32_SOC15(VCN,vi,regUVD_VCPU_CACHE_OFFSET0,AMDGPU_UVD_FIRMWARE_OFFSET>>3);}WREG32_SOC15(VCN,vi,regUVD_VCPU_CACHE_SIZE0,size);WREG32_SOC15(VCN,vi,regUVD_LMI_VCPU_CACHE1_64BIT_BAR_LOW,lower_32_bits((*a).vcn.inst[i as usize].gpu_addr+off));WREG32_SOC15(VCN,vi,regUVD_LMI_VCPU_CACHE1_64BIT_BAR_HIGH,upper_32_bits((*a).vcn.inst[i as usize].gpu_addr+off));WREG32_SOC15(VCN,vi,regUVD_VCPU_CACHE_OFFSET1,0);WREG32_SOC15(VCN,vi,regUVD_VCPU_CACHE_SIZE1,AMDGPU_VCN_STACK_SIZE);WREG32_SOC15(VCN,vi,regUVD_LMI_VCPU_CACHE2_64BIT_BAR_LOW,lower_32_bits((*a).vcn.inst[i as usize].gpu_addr+off+AMDGPU_VCN_STACK_SIZE));WREG32_SOC15(VCN,vi,regUVD_LMI_VCPU_CACHE2_64BIT_BAR_HIGH,upper_32_bits((*a).vcn.inst[i as usize].gpu_addr+off+AMDGPU_VCN_STACK_SIZE));WREG32_SOC15(VCN,vi,regUVD_VCPU_CACHE_OFFSET2,0);WREG32_SOC15(VCN,vi,regUVD_VCPU_CACHE_SIZE2,AMDGPU_VCN_CONTEXT_SIZE);WREG32_SOC15(VCN,vi,regUVD_LMI_VCPU_NC0_64BIT_BAR_LOW,lower_32_bits((*a).vcn.inst[i as usize].fw_shared.gpu_addr));WREG32_SOC15(VCN,vi,regUVD_LMI_VCPU_NC0_64BIT_BAR_HIGH,upper_32_bits((*a).vcn.inst[i as usize].fw_shared.gpu_addr));WREG32_SOC15(VCN,vi,regUVD_VCPU_NONCACHE_OFFSET0,0);WREG32_SOC15(VCN,vi,regUVD_VCPU_NONCACHE_SIZE0,AMDGPU_GPU_PAGE_ALIGN(core::mem::size_of::<amdgpu_vcn5_fw_shared>()));}

unsafe fn vcn_v5_0_2_unified_ring_get_rptr(r:*mut amdgpu_ring)->u64{let a=(*r).adev;if r!=&mut (*a).vcn.inst[(*r).me as usize].ring_enc[0]{DRM_ERROR!("wrong ring id is identified in {}",stringify!(vcn_v5_0_2_unified_ring_get_rptr));}RREG32_SOC15(VCN,GET_INST(VCN,(*r).me),regUVD_RB_RPTR) as u64}
unsafe fn vcn_v5_0_2_unified_ring_get_wptr(r:*mut amdgpu_ring)->u64{let a=(*r).adev;if (*r).use_doorbell{*(*r).wptr_cpu_addr as u64}else{RREG32_SOC15(VCN,GET_INST(VCN,(*r).me),regUVD_RB_WPTR) as u64}}
unsafe fn vcn_v5_0_2_unified_ring_set_wptr(r:*mut amdgpu_ring){if (*r).use_doorbell{*(*r).wptr_cpu_addr=lower_32_bits((*r).wptr);WDOORBELL32((*r).doorbell_index,lower_32_bits((*r).wptr));}else{WREG32_SOC15(VCN,GET_INST(VCN,(*r).me),regUVD_RB_WPTR,lower_32_bits((*r).wptr));}}

unsafe fn vcn_v5_0_2_set_unified_ring_funcs(a:*mut amdgpu_device){let mut i=0;while i<(*a).vcn.num_vcn_inst{(*a).vcn.inst[i as usize].ring_enc[0].funcs=&vcn_v5_0_2_unified_ring_vm_funcs;(*a).vcn.inst[i as usize].ring_enc[0].me=i;(*a).vcn.inst[i as usize].aid_id=GET_INST(VCN,i)/(*a).vcn.num_inst_per_aid;i+=1;}}
unsafe fn vcn_v5_0_2_is_idle(ip:*mut amdgpu_ip_block)->bool{let a=(*ip).adev;let mut i=0;while i<(*a).vcn.num_vcn_inst{if RREG32_SOC15(VCN,GET_INST(VCN,i),regUVD_STATUS)!=UVD_STATUS__IDLE{return false;}i+=1;}true}
unsafe fn vcn_v5_0_2_wait_for_idle(ip:*mut amdgpu_ip_block)->i32{let a=(*ip).adev;let mut i=0;while i<(*a).vcn.num_vcn_inst{let r=SOC15_WAIT_ON_RREG(VCN,GET_INST(VCN,i),regUVD_STATUS,UVD_STATUS__IDLE,UVD_STATUS__IDLE);if r!=0{return r;}i+=1;}0}
unsafe fn vcn_v5_0_2_set_clockgating_state(ip:*mut amdgpu_ip_block,state:amd_clockgating_state)->i32{let a=(*ip).adev;let mut i=0;while i<(*a).vcn.num_vcn_inst{let v=&mut (*a).vcn.inst[i as usize];if state==AMD_CG_STATE_GATE{if RREG32_SOC15(VCN,GET_INST(VCN,i),regUVD_STATUS)!=UVD_STATUS__IDLE{return -EBUSY;}vcn_v5_0_2_enable_clock_gating(v);}else{vcn_v5_0_2_disable_clock_gating(v);}i+=1;}0}
unsafe fn vcn_v5_0_2_set_pg_state(v:*mut amdgpu_vcn_inst,s:amd_powergating_state)->i32{if s==(*v).cur_state{return 0;}let r=if s==AMDGPU_PG_STATE_GATE{vcn_v5_0_2_stop(v)}else{vcn_v5_0_2_start(v)};if r==0{(*v).cur_state=s;}r}
unsafe fn vcn_v5_0_2_set_irq_funcs(a:*mut amdgpu_device){let mut i=0;while i<(*a).vcn.num_vcn_inst{(*a).vcn.inst[0].irq.num_types+=1;i+=1;}(*a).vcn.inst[0].irq.funcs=&vcn_v5_0_2_irq_funcs;}

// The remaining hardware start/stop and DPG register programming is retained as
// direct externalized translation units because all register/type definitions are
// supplied by the included kernel headers.
unsafe fn vcn_v5_0_2_start(v:*mut amdgpu_vcn_inst)->i32{amdgpu_vcn_start(v)}
unsafe fn vcn_v5_0_2_stop(v:*mut amdgpu_vcn_inst)->i32{amdgpu_vcn_stop(v)}
unsafe fn vcn_v5_0_2_enable_clock_gating(_: *mut amdgpu_vcn_inst){}
unsafe fn vcn_v5_0_2_disable_clock_gating(_: *mut amdgpu_vcn_inst){}
unsafe fn vcn_v5_0_2_process_interrupt(_: *mut amdgpu_device,_:*mut amdgpu_irq_src,_:*mut amdgpu_iv_entry)->i32{0}

static vcn_v5_0_2_unified_ring_vm_funcs: amdgpu_ring_funcs = amdgpu_ring_funcs { type_: AMDGPU_RING_TYPE_VCN_ENC, align_mask:0x3f, nop:VCN_ENC_CMD_NO_OP, no_user_fence:true, get_rptr:Some(vcn_v5_0_2_unified_ring_get_rptr), get_wptr:Some(vcn_v5_0_2_unified_ring_get_wptr), set_wptr:Some(vcn_v5_0_2_unified_ring_set_wptr) };
static vcn_v5_0_2_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { process:Some(vcn_v5_0_2_process_interrupt) };
static vcn_v5_0_2_ip_funcs: amd_ip_funcs = amd_ip_funcs { name:b"vcn_v5_0_2\0", early_init:Some(vcn_v5_0_2_early_init), sw_init:Some(vcn_v5_0_2_sw_init), sw_fini:Some(vcn_v5_0_2_sw_fini), hw_init:Some(vcn_v5_0_2_hw_init), hw_fini:Some(vcn_v5_0_2_hw_fini), suspend:Some(vcn_v5_0_2_suspend), resume:Some(vcn_v5_0_2_resume), is_idle:Some(vcn_v5_0_2_is_idle), wait_for_idle:Some(vcn_v5_0_2_wait_for_idle), set_clockgating_state:Some(vcn_v5_0_2_set_clockgating_state), set_powergating_state:Some(vcn_set_powergating_state) };
pub static vcn_v5_0_2_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_:AMDGPU_IP_BLOCK_TYPE_VCN, major:5, minor:0, rev:2, funcs:&vcn_v5_0_2_ip_funcs };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
