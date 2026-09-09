/* Translated from jpeg_v5_3_0.c. External kernel types, functions, and register macros
 * are supplied by the surrounding translation unit. */

unsafe extern "C" {
    fn jpeg_v5_3_0_set_dec_ring_funcs(adev: *mut amdgpu_device);
    fn jpeg_v5_3_0_set_irq_funcs(adev: *mut amdgpu_device);
    fn jpeg_v5_3_0_set_powergating_state(ip_block: *mut amdgpu_ip_block, state: amd_powergating_state) -> i32;
}

unsafe fn jpeg_v5_3_0_early_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    (*adev).jpeg.num_jpeg_inst = 1;
    (*adev).jpeg.num_jpeg_rings = 1;
    jpeg_v5_3_0_set_dec_ring_funcs(adev);
    jpeg_v5_3_0_set_irq_funcs(adev);
    0
}

unsafe fn jpeg_v5_3_0_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let ring: *mut amdgpu_ring;
    let mut r: i32;
    r = amdgpu_irq_add_id(adev, SOC15_IH_CLIENTID_VCN, VCN_5_0__SRCID__JPEG_DECODE, &mut (*(*adev).jpeg.inst).irq);
    if r != 0 { return r; }
    r = amdgpu_jpeg_sw_init(adev); if r != 0 { return r; }
    r = amdgpu_jpeg_resume(adev); if r != 0 { return r; }
    ring = (*(*adev).jpeg.inst).ring_dec;
    (*ring).use_doorbell = true;
    (*ring).doorbell_index = ((*adev).doorbell_index.vcn.vcn_ring0_1 << 1) + 1;
    (*ring).vm_hub = AMDGPU_MMHUB0(0);
    sprintf((*ring).name.as_mut_ptr(), b"jpeg_dec\0".as_ptr() as *const i8);
    r = amdgpu_ring_init(adev, ring, 512, &mut (*(*adev).jpeg.inst).irq, 0, AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut());
    if r != 0 { return r; }
    (*adev).jpeg.internal.jpeg_pitch[0] = regUVD_JPEG_PITCH_INTERNAL_OFFSET;
    (*(*adev).jpeg.inst).external.jpeg_pitch[0] = SOC15_REG_OFFSET(JPEG, 0, regUVD_JPEG_PITCH);
    (*adev).jpeg.supported_reset = amdgpu_get_soft_full_reset_mask(&mut (*(*adev).jpeg.inst).ring_dec[0]);
    if !amdgpu_sriov_vf(adev) { (*adev).jpeg.supported_reset |= AMDGPU_RESET_TYPE_PER_QUEUE; }
    r = amdgpu_jpeg_sysfs_reset_mask_init(adev); if r != 0 { return r; }
    0
}

unsafe fn jpeg_v5_3_0_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let mut r = amdgpu_jpeg_suspend(adev); if r != 0 { return r; }
    amdgpu_jpeg_sysfs_reset_mask_fini(adev);
    r = amdgpu_jpeg_sw_fini(adev); r
}

unsafe fn jpeg_v5_3_0_hw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let ring = (*(*adev).jpeg.inst).ring_dec;
    (*adev).nbio.funcs.vcn_doorbell_range(adev, (*ring).use_doorbell, (*adev).doorbell_index.vcn.vcn_ring0_1 << 1, 0);
    if (*adev).pg_flags & AMD_PG_SUPPORT_JPEG_DPG != 0 { return 0; }
    let r = amdgpu_ring_test_helper(ring); if r != 0 { return r; } 0
}

unsafe fn jpeg_v5_3_0_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    cancel_delayed_work_sync(&mut (*adev).jpeg.idle_work);
    if (*adev).jpeg.cur_state != AMD_PG_STATE_GATE && RREG32_SOC15(JPEG, 0, regUVD_JRBC0_UVD_JRBC_STATUS) != 0 { jpeg_v5_3_0_set_powergating_state(ip_block, AMD_PG_STATE_GATE); }
    0
}

unsafe fn jpeg_v5_3_0_suspend(ip_block: *mut amdgpu_ip_block) -> i32 { let r = jpeg_v5_3_0_hw_fini(ip_block); if r != 0 { return r; } amdgpu_jpeg_suspend((*ip_block).adev) }
unsafe fn jpeg_v5_3_0_resume(ip_block: *mut amdgpu_ip_block) -> i32 { let r = amdgpu_jpeg_resume((*ip_block).adev); if r != 0 { return r; } jpeg_v5_3_0_hw_init(ip_block) }

unsafe fn jpeg_v5_3_0_disable_clock_gating(adev: *mut amdgpu_device) { let mut data: u32 = 0; WREG32_SOC15(JPEG,0,regJPEG_CGC_GATE,data); data=RREG32_SOC15(JPEG,0,regJPEG_CGC_CTRL); data &= !(JPEG_CGC_CTRL__JPEG0_DEC_MODE_MASK|JPEG_CGC_CTRL__JPEG_ENC_MODE_MASK); WREG32_SOC15(JPEG,0,regJPEG_CGC_CTRL,data); }
unsafe fn jpeg_v5_3_0_enable_clock_gating(adev: *mut amdgpu_device) { let mut data=RREG32_SOC15(JPEG,0,regJPEG_CGC_CTRL); data |= 1 << JPEG_CGC_CTRL__JPEG0_DEC_MODE__SHIFT; WREG32_SOC15(JPEG,0,regJPEG_CGC_CTRL,data); data=RREG32_SOC15(JPEG,0,regJPEG_CGC_GATE); data |= JPEG_CGC_GATE__JPEG0_DEC_MASK|JPEG_CGC_GATE__JPEG_ENC_MASK|JPEG_CGC_GATE__JMCIF_MASK|JPEG_CGC_GATE__JRBBM_MASK; WREG32_SOC15(JPEG,0,regJPEG_CGC_GATE,data); }

unsafe fn jpeg_v5_3_0_disable_power_gating(adev: *mut amdgpu_device) -> i32 { let data=1 << UVD_IPX_DLDO_CONFIG_ONO1__ONO1_PWR_CONFIG__SHIFT; WREG32_SOC15(JPEG,0,regUVD_IPX_DLDO_CONFIG_ONO1,data); SOC15_WAIT_ON_RREG(JPEG,0,regUVD_IPX_DLDO_STATUS,0,UVD_IPX_DLDO_STATUS__ONO1_PWR_STATUS_MASK); WREG32_P(SOC15_REG_OFFSET(JPEG,0,regUVD_JPEG_POWER_STATUS),0,!UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK); 0 }
unsafe fn jpeg_v5_3_0_enable_power_gating(adev: *mut amdgpu_device) -> i32 { WREG32_P(SOC15_REG_OFFSET(JPEG,0,regUVD_JPEG_POWER_STATUS),UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK,!UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK); if (*adev).pg_flags & AMD_PG_SUPPORT_JPEG != 0 { WREG32(SOC15_REG_OFFSET(JPEG,0,regUVD_IPX_DLDO_CONFIG_ONO1),2 << UVD_IPX_DLDO_CONFIG_ONO1__ONO1_PWR_CONFIG__SHIFT); SOC15_WAIT_ON_RREG(JPEG,0,regUVD_IPX_DLDO_STATUS,1 << UVD_IPX_DLDO_STATUS__ONO1_PWR_STATUS__SHIFT,UVD_IPX_DLDO_STATUS__ONO1_PWR_STATUS_MASK); } 0 }

unsafe fn jpeg_engine_5_0_0_dpg_clock_gating_mode(adev:*mut amdgpu_device, inst_idx:i32, indirect:u8) { let mut data=if (*adev).cg_flags & AMD_CG_SUPPORT_JPEG_MGCG != 0 { 1 << JPEG_CGC_CTRL__DYN_CLOCK_MODE__SHIFT } else { 0 }; data |= 1 << JPEG_CGC_CTRL__CLK_GATE_DLY_TIMER__SHIFT; data |= 4 << JPEG_CGC_CTRL__CLK_OFF_DELAY__SHIFT; if indirect != 0 { ADD_SOC24_JPEG_TO_DPG_SRAM(inst_idx,vcnipJPEG_CGC_CTRL,data,indirect); data=0; ADD_SOC24_JPEG_TO_DPG_SRAM(inst_idx,vcnipJPEG_CGC_GATE,data,indirect); } else { WREG32_SOC24_JPEG_DPG_MODE(inst_idx,vcnipJPEG_CGC_CTRL,data,indirect); data=0; WREG32_SOC24_JPEG_DPG_MODE(inst_idx,vcnipJPEG_CGC_GATE,data,indirect); } }

/* DPG/start/stop, ring accessors, IP/ring/IRQ tables, and exported block descriptor. */
unsafe fn jpeg_v5_3_0_start_dpg_mode(adev:*mut amdgpu_device, inst_idx:i32, indirect:bool)->i32 { let ring=(*(*adev).jpeg.inst.add(inst_idx as usize)).ring_dec; jpeg_v5_3_0_enable_power_gating(adev); let mut reg_data=RREG32_SOC15(JPEG,inst_idx,regUVD_JPEG_POWER_STATUS); reg_data |= UVD_JPEG_POWER_STATUS__JPEG_PG_MODE_MASK; WREG32_SOC15(JPEG,inst_idx,regUVD_JPEG_POWER_STATUS,reg_data); jpeg_engine_5_0_0_dpg_clock_gating_mode(adev,inst_idx,indirect as u8); if indirect { ADD_SOC24_JPEG_TO_DPG_SRAM(inst_idx,vcnipJPEG_DEC_GFX10_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config,indirect as u8); ADD_SOC24_JPEG_TO_DPG_SRAM(inst_idx,vcnipJPEG_SYS_INT_EN,JPEG_SYS_INT_EN__DJRBC0_MASK,indirect as u8); ADD_SOC24_JPEG_TO_DPG_SRAM(inst_idx,vcnipUVD_NO_OP,0,indirect as u8); amdgpu_jpeg_psp_update_sram(adev,inst_idx,0); } else { WREG32_SOC24_JPEG_DPG_MODE(inst_idx,vcnipJPEG_DEC_GFX10_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config,1); WREG32_SOC24_JPEG_DPG_MODE(inst_idx,vcnipJPEG_SYS_INT_EN,JPEG_SYS_INT_EN__DJRBC0_MASK,1); } WREG32_SOC15(VCN,0,regVCN_JPEG_DB_CTRL,(*ring).doorbell_index << VCN_JPEG_DB_CTRL__OFFSET__SHIFT | VCN_JPEG_DB_CTRL__EN_MASK); WREG32_SOC15(JPEG,inst_idx,regUVD_LMI_JRBC_RB_VMID,0); WREG32_SOC15(JPEG,inst_idx,regUVD_JRBC0_UVD_JRBC_RB_CNTL,0x00000003); WREG32_SOC15(JPEG,inst_idx,regUVD_LMI_JRBC_RB_64BIT_BAR_LOW,lower_32_bits((*ring).gpu_addr)); WREG32_SOC15(JPEG,inst_idx,regUVD_LMI_JRBC_RB_64BIT_BAR_HIGH,upper_32_bits((*ring).gpu_addr)); WREG32_SOC15(JPEG,inst_idx,regUVD_JRBC0_UVD_JRBC_RB_RPTR,0); WREG32_SOC15(JPEG,inst_idx,regUVD_JRBC0_UVD_JRBC_RB_WPTR,0); WREG32_SOC15(JPEG,inst_idx,regUVD_JRBC0_UVD_JRBC_RB_CNTL,2); WREG32_SOC15(JPEG,inst_idx,regUVD_JRBC0_UVD_JRBC_RB_SIZE,(*ring).ring_size/4); (*ring).wptr=RREG32_SOC15(JPEG,inst_idx,regUVD_JRBC0_UVD_JRBC_RB_WPTR); 0 }
unsafe fn jpeg_v5_3_0_stop_dpg_mode(adev:*mut amdgpu_device,inst_idx:i32){let mut d=RREG32_SOC15(JPEG,inst_idx,regUVD_JPEG_POWER_STATUS);d &= !UVD_JPEG_POWER_STATUS__JPEG_PG_MODE_MASK;WREG32_SOC15(JPEG,inst_idx,regUVD_JPEG_POWER_STATUS,d);}

unsafe fn jpeg_v5_3_0_set_mmhub_eco_sec_level(adev:*mut amdgpu_device)->i32 { if (*adev).firmware.load_type == AMDGPU_FW_LOAD_PSP { psp_set_mmhub_eco_sec_level(adev) } else { 0 } }
unsafe fn jpeg_v5_3_0_start(adev:*mut amdgpu_device)->i32 { let ring=(*(*adev).jpeg.inst).ring_dec; if (*adev).pm.dpm_enabled { amdgpu_dpm_enable_jpeg(adev,true); } if (*adev).pg_flags & AMD_PG_SUPPORT_JPEG_DPG != 0 { return jpeg_v5_3_0_start_dpg_mode(adev,0,(*adev).jpeg.indirect_sram); } let mut r=jpeg_v5_3_0_disable_power_gating(adev); if r!=0{return r;} r=jpeg_v5_3_0_set_mmhub_eco_sec_level(adev); if r!=0{return r;} jpeg_v5_3_0_disable_clock_gating(adev); WREG32_SOC15(JPEG,0,regJPEG_DEC_GFX10_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config); WREG32_P(SOC15_REG_OFFSET(JPEG,0,regUVD_JMI_CNTL),0,!UVD_JMI_CNTL__SOFT_RESET_MASK); WREG32_P(SOC15_REG_OFFSET(JPEG,0,regJPEG_SYS_INT_EN),JPEG_SYS_INT_EN__DJRBC0_MASK,!JPEG_SYS_INT_EN__DJRBC0_MASK); WREG32_SOC15(VCN,0,regVCN_JPEG_DB_CTRL,(*ring).doorbell_index << VCN_JPEG_DB_CTRL__OFFSET__SHIFT|VCN_JPEG_DB_CTRL__EN_MASK); WREG32_SOC15(JPEG,0,regUVD_LMI_JRBC_RB_VMID,0); WREG32_SOC15(JPEG,0,regUVD_JRBC0_UVD_JRBC_RB_CNTL,3); WREG32_SOC15(JPEG,0,regUVD_LMI_JRBC_RB_64BIT_BAR_LOW,lower_32_bits((*ring).gpu_addr)); WREG32_SOC15(JPEG,0,regUVD_LMI_JRBC_RB_64BIT_BAR_HIGH,upper_32_bits((*ring).gpu_addr)); WREG32_SOC15(JPEG,0,regUVD_JRBC0_UVD_JRBC_RB_RPTR,0); WREG32_SOC15(JPEG,0,regUVD_JRBC0_UVD_JRBC_RB_WPTR,0); WREG32_SOC15(JPEG,0,regUVD_JRBC0_UVD_JRBC_RB_CNTL,2); WREG32_SOC15(JPEG,0,regUVD_JRBC0_UVD_JRBC_RB_SIZE,(*ring).ring_size/4); (*ring).wptr=RREG32_SOC15(JPEG,0,regUVD_JRBC0_UVD_JRBC_RB_WPTR); 0 }
unsafe fn jpeg_v5_3_0_stop(adev:*mut amdgpu_device)->i32 { let mut r=0; if (*adev).pg_flags & AMD_PG_SUPPORT_JPEG_DPG != 0 {jpeg_v5_3_0_stop_dpg_mode(adev,0);} else {WREG32_P(SOC15_REG_OFFSET(JPEG,0,regUVD_JMI_CNTL),UVD_JMI_CNTL__SOFT_RESET_MASK,!UVD_JMI_CNTL__SOFT_RESET_MASK);jpeg_v5_3_0_enable_clock_gating(adev);r=jpeg_v5_3_0_enable_power_gating(adev);} if r==0 && (*adev).pm.dpm_enabled {amdgpu_dpm_enable_jpeg(adev,false);} r }
unsafe fn jpeg_v5_3_0_dec_ring_get_rptr(ring:*mut amdgpu_ring)->u64 {RREG32_SOC15(JPEG,0,regUVD_JRBC0_UVD_JRBC_RB_RPTR) as u64}
unsafe fn jpeg_v5_3_0_dec_ring_get_wptr(ring:*mut amdgpu_ring)->u64 {if (*ring).use_doorbell{*(*ring).wptr_cpu_addr as u64}else{RREG32_SOC15(JPEG,0,regUVD_JRBC0_UVD_JRBC_RB_WPTR) as u64}}
unsafe fn jpeg_v5_3_0_dec_ring_set_wptr(ring:*mut amdgpu_ring){let w=lower_32_bits((*ring).wptr);if (*ring).use_doorbell{*(*ring).wptr_cpu_addr=w;WDOORBELL32((*ring).doorbell_index,w);}else{WREG32_SOC15(JPEG,0,regUVD_JRBC0_UVD_JRBC_RB_WPTR,w);}}
unsafe fn jpeg_v5_3_0_is_idle(ip:*mut amdgpu_ip_block)->bool {let a=(*ip).adev; (RREG32_SOC15(JPEG,0,regUVD_JRBC0_UVD_JRBC_STATUS)&UVD_JRBC0_UVD_JRBC_STATUS__RB_JOB_DONE_MASK)==UVD_JRBC0_UVD_JRBC_STATUS__RB_JOB_DONE_MASK}
unsafe fn jpeg_v5_3_0_wait_for_idle(ip:*mut amdgpu_ip_block)->i32 {SOC15_WAIT_ON_RREG(JPEG,0,regUVD_JRBC0_UVD_JRBC_STATUS,UVD_JRBC0_UVD_JRBC_STATUS__RB_JOB_DONE_MASK,UVD_JRBC0_UVD_JRBC_STATUS__RB_JOB_DONE_MASK)}
unsafe fn jpeg_v5_3_0_set_clockgating_state(ip:*mut amdgpu_ip_block,state:amd_clockgating_state)->i32 {let a=(*ip).adev;if state==AMD_CG_STATE_GATE {if !jpeg_v5_3_0_is_idle(ip){return -EBUSY;}jpeg_v5_3_0_enable_clock_gating(a);}else{jpeg_v5_3_0_disable_clock_gating(a);}0}
unsafe fn jpeg_v5_3_0_set_powergating_state_impl(ip:*mut amdgpu_ip_block,state:amd_powergating_state)->i32 {let a=(*ip).adev;if state==(*a).jpeg.cur_state{return 0;}let r=if state==AMD_PG_STATE_GATE{jpeg_v5_3_0_stop(a)}else{jpeg_v5_3_0_start(a)};if r==0{(*a).jpeg.cur_state=state;}r}
unsafe fn jpeg_v5_3_0_set_interrupt_state(_: *mut amdgpu_device,_:*mut amdgpu_irq_src,_:u32,_:amdgpu_interrupt_state)->i32{0}
unsafe fn jpeg_v5_3_0_ring_reset(ring:*mut amdgpu_ring,_:u32,f:*mut amdgpu_fence)->i32{let a=(*ring).adev;let flags=(*a).pg_flags;amdgpu_ring_reset_helper_begin(ring,f);(*a).pg_flags &= !AMD_PG_SUPPORT_JPEG_DPG;let mut r=jpeg_v5_3_0_stop(a);if r==0{r=jpeg_v5_3_0_start(a);}(*a).pg_flags=flags;if r!=0{r}else{amdgpu_ring_reset_helper_end(ring,f)}}

static jpeg_v5_3_0_ip_funcs: amd_ip_funcs = amd_ip_funcs { name:b"jpeg_v5_3_0\0".as_ptr() as *const i8, early_init:Some(jpeg_v5_3_0_early_init), sw_init:Some(jpeg_v5_3_0_sw_init), sw_fini:Some(jpeg_v5_3_0_sw_fini), hw_init:Some(jpeg_v5_3_0_hw_init), hw_fini:Some(jpeg_v5_3_0_hw_fini), suspend:Some(jpeg_v5_3_0_suspend), resume:Some(jpeg_v5_3_0_resume), is_idle:Some(jpeg_v5_3_0_is_idle), wait_for_idle:Some(jpeg_v5_3_0_wait_for_idle), set_clockgating_state:Some(jpeg_v5_3_0_set_clockgating_state), set_powergating_state:Some(jpeg_v5_3_0_set_powergating_state_impl) };

#[no_mangle]
pub static mut jpeg_v5_3_0_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_: AMD_IP_BLOCK_TYPE_JPEG, major:5, minor:3, rev:0, funcs:&jpeg_v5_3_0_ip_funcs };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
