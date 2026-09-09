/* Copyright 2023 Advanced Micro Devices, Inc. */

// C translation; declarations and macros below are supplied by the surrounding driver.

static jpeg_reg_list_5_0: [amdgpu_hwip_reg_entry; 13] = [
    SOC15_REG_ENTRY_STR!(JPEG, 0, regUVD_JPEG_POWER_STATUS),
    SOC15_REG_ENTRY_STR!(JPEG, 0, regUVD_JPEG_INT_STAT),
    SOC15_REG_ENTRY_STR!(JPEG, 0, regUVD_JRBC_RB_RPTR),
    SOC15_REG_ENTRY_STR!(JPEG, 0, regUVD_JRBC_RB_WPTR),
    SOC15_REG_ENTRY_STR!(JPEG, 0, regUVD_JRBC_RB_CNTL),
    SOC15_REG_ENTRY_STR!(JPEG, 0, regUVD_JRBC_RB_SIZE),
    SOC15_REG_ENTRY_STR!(JPEG, 0, regUVD_JRBC_STATUS),
    SOC15_REG_ENTRY_STR!(JPEG, 0, regJPEG_DEC_ADDR_MODE),
    SOC15_REG_ENTRY_STR!(JPEG, 0, regJPEG_DEC_GFX10_ADDR_CONFIG),
    SOC15_REG_ENTRY_STR!(JPEG, 0, regJPEG_DEC_Y_GFX10_TILING_SURFACE),
    SOC15_REG_ENTRY_STR!(JPEG, 0, regJPEG_DEC_UV_GFX10_TILING_SURFACE),
    SOC15_REG_ENTRY_STR!(JPEG, 0, regUVD_JPEG_PITCH),
    SOC15_REG_ENTRY_STR!(JPEG, 0, regUVD_JPEG_UV_PITCH),
];

unsafe fn jpeg_v5_0_0_early_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    (*adev).jpeg.num_jpeg_inst = 1;
    (*adev).jpeg.num_jpeg_rings = 1;
    jpeg_v5_0_0_set_dec_ring_funcs(adev);
    jpeg_v5_0_0_set_irq_funcs(adev);
    0
}

unsafe fn jpeg_v5_0_0_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let mut r = amdgpu_irq_add_id(adev, SOC15_IH_CLIENTID_VCN, VCN_5_0__SRCID__JPEG_DECODE, &mut (*(*adev).jpeg.inst).irq);
    if r != 0 { return r; }
    r = amdgpu_jpeg_sw_init(adev); if r != 0 { return r; }
    r = amdgpu_jpeg_resume(adev); if r != 0 { return r; }
    let ring = (*(*adev).jpeg.inst).ring_dec;
    (*ring).use_doorbell = true;
    (*ring).doorbell_index = ((*adev).doorbell_index.vcn.vcn_ring0_1 << 1) + 1;
    (*ring).vm_hub = AMDGPU_MMHUB0!(0);
    sprintf!((*ring).name, "jpeg_dec");
    r = amdgpu_ring_init(adev, ring, 512, &mut (*(*adev).jpeg.inst).irq, 0, AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut());
    if r != 0 { return r; }
    (*adev).jpeg.internal.jpeg_pitch[0] = regUVD_JPEG_PITCH_INTERNAL_OFFSET;
    (*(*adev).jpeg.inst).external.jpeg_pitch[0] = SOC15_REG_OFFSET!(JPEG, 0, regUVD_JPEG_PITCH);
    r = amdgpu_jpeg_reg_dump_init(adev, jpeg_reg_list_5_0.as_ptr(), jpeg_reg_list_5_0.len());
    if r != 0 { return r; }
    (*adev).jpeg.supported_reset = amdgpu_get_soft_full_reset_mask(&(*(*adev).jpeg.inst[0]).ring_dec[0]);
    if !amdgpu_sriov_vf(adev) { (*adev).jpeg.supported_reset |= AMDGPU_RESET_TYPE_PER_QUEUE; }
    amdgpu_jpeg_sysfs_reset_mask_init(adev)
}

unsafe fn jpeg_v5_0_0_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let mut r = amdgpu_jpeg_suspend(adev); if r != 0 { return r; }
    amdgpu_jpeg_sysfs_reset_mask_fini(adev);
    r = amdgpu_jpeg_sw_fini(adev); r
}

unsafe fn jpeg_v5_0_0_hw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev; let ring = (*(*adev).jpeg.inst).ring_dec;
    (*adev).nbio.funcs.as_ref().unwrap().vcn_doorbell_range(adev, (*ring).use_doorbell, (*adev).doorbell_index.vcn.vcn_ring0_1 << 1, 0);
    if (*adev).pg_flags & AMD_PG_SUPPORT_JPEG_DPG != 0 { return 0; }
    amdgpu_ring_test_helper(ring)
}

unsafe fn jpeg_v5_0_0_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev; cancel_delayed_work_sync!(&mut (*adev).jpeg.idle_work);
    if (*adev).jpeg.cur_state != AMD_PG_STATE_GATE && RREG32_SOC15!(JPEG, 0, regUVD_JRBC_STATUS) != 0 { jpeg_v5_0_0_set_powergating_state(ip_block, AMD_PG_STATE_GATE); } 0
}

unsafe fn jpeg_v5_0_0_suspend(ip_block: *mut amdgpu_ip_block) -> i32 { let mut r=jpeg_v5_0_0_hw_fini(ip_block); if r!=0{return r;} r=amdgpu_jpeg_suspend((*ip_block).adev); r }
unsafe fn jpeg_v5_0_0_resume(ip_block: *mut amdgpu_ip_block) -> i32 { let mut r=amdgpu_jpeg_resume((*ip_block).adev); if r!=0{return r;} r=jpeg_v5_0_0_hw_init(ip_block); r }

unsafe fn jpeg_v5_0_0_disable_clock_gating(adev:*mut amdgpu_device){let mut data=0u32;WREG32_SOC15!(JPEG,0,regJPEG_CGC_GATE,data);data=RREG32_SOC15!(JPEG,0,regJPEG_CGC_CTRL);data&=!(JPEG_CGC_CTRL__JPEG0_DEC_MODE_MASK|JPEG_CGC_CTRL__JPEG_ENC_MODE_MASK);WREG32_SOC15!(JPEG,0,regJPEG_CGC_CTRL,data);}
unsafe fn jpeg_v5_0_0_enable_clock_gating(adev:*mut amdgpu_device){let mut data=RREG32_SOC15!(JPEG,0,regJPEG_CGC_CTRL);data|=1<<JPEG_CGC_CTRL__JPEG0_DEC_MODE__SHIFT;WREG32_SOC15!(JPEG,0,regJPEG_CGC_CTRL,data);data=RREG32_SOC15!(JPEG,0,regJPEG_CGC_GATE);data|=JPEG_CGC_GATE__JPEG0_DEC_MASK|JPEG_CGC_GATE__JPEG_ENC_MASK|JPEG_CGC_GATE__JMCIF_MASK|JPEG_CGC_GATE__JRBBM_MASK;WREG32_SOC15!(JPEG,0,regJPEG_CGC_GATE,data);}

unsafe fn jpeg_v5_0_0_disable_power_gating(adev:*mut amdgpu_device)->i32 { WREG32_SOC15!(JPEG,0,regUVD_IPX_DLDO_CONFIG,1<<UVD_IPX_DLDO_CONFIG__ONO1_PWR_CONFIG__SHIFT); SOC15_WAIT_ON_RREG!(JPEG,0,regUVD_IPX_DLDO_STATUS,0,UVD_IPX_DLDO_STATUS__ONO1_PWR_STATUS_MASK); WREG32_P!(SOC15_REG_OFFSET!(JPEG,0,regUVD_JPEG_POWER_STATUS),0,!UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK); 0 }
unsafe fn jpeg_v5_0_0_enable_power_gating(adev:*mut amdgpu_device)->i32 { WREG32_P!(SOC15_REG_OFFSET!(JPEG,0,regUVD_JPEG_POWER_STATUS),UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK,!UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK); if (*adev).pg_flags&AMD_PG_SUPPORT_JPEG!=0 {WREG32!(SOC15_REG_OFFSET!(JPEG,0,regUVD_IPX_DLDO_CONFIG),2<<UVD_IPX_DLDO_CONFIG__ONO1_PWR_CONFIG__SHIFT);SOC15_WAIT_ON_RREG!(JPEG,0,regUVD_IPX_DLDO_STATUS,1<<UVD_IPX_DLDO_STATUS__ONO1_PWR_STATUS__SHIFT,UVD_IPX_DLDO_STATUS__ONO1_PWR_STATUS_MASK);} 0 }
unsafe fn jpeg_v5_0_0_stop_dpg_mode(adev:*mut amdgpu_device, i:i32){let mut d=RREG32_SOC15!(JPEG,i,regUVD_JPEG_POWER_STATUS);d&=!UVD_JPEG_POWER_STATUS__JPEG_PG_MODE_MASK;WREG32_SOC15!(JPEG,i,regUVD_JPEG_POWER_STATUS,d);}
unsafe fn jpeg_v5_0_0_start(adev:*mut amdgpu_device)->i32 { if (*adev).pm.dpm_enabled {amdgpu_dpm_enable_jpeg(adev,true);} jpeg_v5_0_0_disable_power_gating(adev); jpeg_v5_0_0_disable_clock_gating(adev); WREG32_SOC15!(JPEG,0,regJPEG_DEC_GFX10_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config); WREG32_P!(SOC15_REG_OFFSET!(JPEG,0,regUVD_JMI_CNTL),0,!UVD_JMI_CNTL__SOFT_RESET_MASK); WREG32_P!(SOC15_REG_OFFSET!(JPEG,0,regJPEG_SYS_INT_EN),JPEG_SYS_INT_EN__DJRBC0_MASK,!JPEG_SYS_INT_EN__DJRBC0_MASK); 0 }
unsafe fn jpeg_v5_0_0_stop(adev:*mut amdgpu_device)->i32 { if (*adev).pg_flags&AMD_PG_SUPPORT_JPEG_DPG!=0 {jpeg_v5_0_0_stop_dpg_mode(adev,0);} else {WREG32_P!(SOC15_REG_OFFSET!(JPEG,0,regUVD_JMI_CNTL),UVD_JMI_CNTL__SOFT_RESET_MASK,!UVD_JMI_CNTL__SOFT_RESET_MASK);jpeg_v5_0_0_enable_clock_gating(adev);jpeg_v5_0_0_enable_power_gating(adev);} if (*adev).pm.dpm_enabled {amdgpu_dpm_enable_jpeg(adev,false);} 0 }
unsafe fn jpeg_v5_0_0_dec_ring_get_rptr(r:*mut amdgpu_ring)->u64{RREG32_SOC15!(JPEG,0,regUVD_JRBC_RB_RPTR) as u64}
unsafe fn jpeg_v5_0_0_dec_ring_get_wptr(r:*mut amdgpu_ring)->u64{if (*r).use_doorbell{*(*r).wptr_cpu_addr as u64}else{RREG32_SOC15!(JPEG,0,regUVD_JRBC_RB_WPTR) as u64}}
unsafe fn jpeg_v5_0_0_dec_ring_set_wptr(r:*mut amdgpu_ring){if (*r).use_doorbell{*(*r).wptr_cpu_addr=lower_32_bits!((*r).wptr);WDOORBELL32!((*r).doorbell_index,lower_32_bits!((*r).wptr));}else{WREG32_SOC15!(JPEG,0,regUVD_JRBC_RB_WPTR,lower_32_bits!((*r).wptr));}}
unsafe fn jpeg_v5_0_0_is_idle(b:*mut amdgpu_ip_block)->bool{((*RREG32_SOC15!(JPEG,0,regUVD_JRBC_STATUS as u32 as *const u32))&UVD_JRBC_STATUS__RB_JOB_DONE_MASK)==UVD_JRBC_STATUS__RB_JOB_DONE_MASK}
unsafe fn jpeg_v5_0_0_wait_for_idle(b:*mut amdgpu_ip_block)->i32{SOC15_WAIT_ON_RREG!(JPEG,0,regUVD_JRBC_STATUS,UVD_JRBC_STATUS__RB_JOB_DONE_MASK,UVD_JRBC_STATUS__RB_JOB_DONE_MASK)}
unsafe fn jpeg_v5_0_0_set_interrupt_state(a:*mut amdgpu_device,s:*mut amdgpu_irq_src,t:u32,state:amdgpu_interrupt_state)->i32{0}
pub unsafe fn jpeg_v5_0_0_process_interrupt(a:*mut amdgpu_device,s:*mut amdgpu_irq_src,e:*mut amdgpu_iv_entry)->i32{if (*e).src_id==VCN_5_0__SRCID__JPEG_DECODE{amdgpu_fence_process((*(*a).jpeg.inst).ring_dec);}else{DRM_DEV_ERROR!((*a).dev,"Unhandled interrupt: %d %d\n",(*e).src_id,(*e).src_data[0]);}0}
unsafe fn jpeg_v5_0_0_set_dec_ring_funcs(adev:*mut amdgpu_device){(*(*adev).jpeg.inst).ring_dec.funcs=&jpeg_v5_0_0_dec_ring_vm_funcs;}
unsafe fn jpeg_v5_0_0_set_irq_funcs(adev:*mut amdgpu_device){(*(*adev).jpeg.inst).irq.num_types=1;(*(*adev).jpeg.inst).irq.funcs=&jpeg_v5_0_0_irq_funcs;}

static jpeg_v5_0_0_ip_funcs: amd_ip_funcs = amd_ip_funcs { name: "jpeg_v5_0_0", early_init: Some(jpeg_v5_0_0_early_init), sw_init: Some(jpeg_v5_0_0_sw_init), sw_fini: Some(jpeg_v5_0_0_sw_fini), hw_init: Some(jpeg_v5_0_0_hw_init), hw_fini: Some(jpeg_v5_0_0_hw_fini), suspend: Some(jpeg_v5_0_0_suspend), resume: Some(jpeg_v5_0_0_resume), ..amd_ip_funcs::ZERO };
static jpeg_v5_0_0_dec_ring_vm_funcs: amdgpu_ring_funcs = amdgpu_ring_funcs { r#type: AMDGPU_RING_TYPE_VCN_JPEG, align_mask: 0xf, no_user_fence: true, ..amdgpu_ring_funcs::ZERO };
static jpeg_v5_0_0_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { set: Some(jpeg_v5_0_0_set_interrupt_state), process: Some(jpeg_v5_0_0_process_interrupt) };
pub static jpeg_v5_0_0_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { r#type: AMD_IP_BLOCK_TYPE_JPEG, major: 5, minor: 0, rev: 0, funcs: &jpeg_v5_0_0_ip_funcs };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
