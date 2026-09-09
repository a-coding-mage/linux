/* Faithful low-level Rust translation of nbif_v6_3_1.c. */

// C headers provide the register constants, macros, types, and external APIs.

pub const REGGDC_S2A0_S2A_DOORBELL_ENTRY_0_CTRL_NBIF_4_10: u32 = 0x4f0aeb;
pub const REGGDC_S2A0_S2A_DOORBELL_ENTRY_0_CTRL1_NBIF_4_10: u32 = 0x4f0aec;
pub const REGGDC_S2A0_S2A_DOORBELL_ENTRY_1_CTRL_NBIF_4_10: u32 = 0x4f0aed;
pub const REGGDC_S2A0_S2A_DOORBELL_ENTRY_1_CTRL1_NBIF_4_10: u32 = 0x4f0aee;
pub const REGGDC_S2A0_S2A_DOORBELL_ENTRY_2_CTRL_NBIF_4_10: u32 = 0x4f0aef;
pub const REGGDC_S2A0_S2A_DOORBELL_ENTRY_2_CTRL1_NBIF_4_10: u32 = 0x4f0af0;
pub const REGGDC_S2A0_S2A_DOORBELL_ENTRY_3_CTRL_NBIF_4_10: u32 = 0x4f0af1;
pub const REGGDC_S2A0_S2A_DOORBELL_ENTRY_3_CTRL1_NBIF_4_10: u32 = 0x4f0af2;
pub const REGGDC_S2A0_S2A_DOORBELL_ENTRY_4_CTRL_NBIF_4_10: u32 = 0x4f0af3;
pub const REGGDC_S2A0_S2A_DOORBELL_ENTRY_4_CTRL1_NBIF_4_10: u32 = 0x4f0af4;
pub const REGGDC_S2A0_S2A_DOORBELL_ENTRY_5_CTRL_NBIF_4_10: u32 = 0x4f0af5;
pub const REGGDC_S2A0_S2A_DOORBELL_ENTRY_5_CTRL1_NBIF_4_10: u32 = 0x4f0af6;
pub const REGRCC_STRAP0_RCC_DEV0_EPF0_STRAP0_NBIF_4_10: u32 = 0x0021;
pub const REGBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_BASE_HIGH_NBIO_7_11_5: u32 = 0x8e13;
pub const REGBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_BASE_LOW_NBIO_7_11_5: u32 = 0x8e14;
pub const REGBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL_NBIO_7_11_5: u32 = 0x8e15;
pub const REGBIF_BX1_REMAP_HDP_MEM_FLUSH_CNTL_NBIO_7_11_5: u32 = 0x012d;
pub const REGBIF_BX1_REMAP_HDP_REG_FLUSH_CNTL_NBIO_7_11_5: u32 = 0x012e;
pub const REGRCC_STRAP1_RCC_DEV0_EPF0_STRAP0_NBIO_7_11_5: u32 = 0x0021;
pub const REGBIF_BX1_BIF_FB_EN_NBIO_7_11_5: u32 = 0x0100;
pub const REGBIF_BX1_INTERRUPT_CNTL_NBIO_7_11_5: u32 = 0x00f1;
pub const REGBIF_BX1_INTERRUPT_CNTL2_NBIO_7_11_5: u32 = 0x00f2;
pub const REGBIF_BX_PF1_GPU_HDP_FLUSH_REQ_NBIO_7_11_5: u32 = 0x0106;
pub const REGBIF_BX_PF1_GPU_HDP_FLUSH_DONE_NBIO_7_11_5: u32 = 0x0107;
pub const REGBIF_BX_PF1_HDP_MEM_COHERENCY_FLUSH_CNTL_NBIO_7_11_5: u32 = 0x00f7;
pub const BIF_BX1_BIF_FB_EN__FB_READ_EN__SHIFT_NBIO_7_11_5: u32 = 0;
pub const BIF_BX1_BIF_FB_EN__FB_WRITE_EN__SHIFT_NBIO_7_11_5: u32 = 1;
pub const BIF_BX1_BIF_FB_EN__FB_READ_EN_MASK_NBIO_7_11_5: u32 = 1;
pub const BIF_BX1_BIF_FB_EN__FB_WRITE_EN_MASK_NBIO_7_11_5: u32 = 2;

unsafe fn nbif_v6_3_1_remap_hdp_registers(adev: *mut amdgpu_device) {
    if amdgpu_ip_version(adev, NBIO_HWIP, 0) == IP_VERSION(7,11,5) {
        WREG32_SOC15(NBIO,0,REGBIF_BX1_REMAP_HDP_MEM_FLUSH_CNTL_NBIO_7_11_5, (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL);
        WREG32_SOC15(NBIO,0,REGBIF_BX1_REMAP_HDP_REG_FLUSH_CNTL_NBIO_7_11_5, (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_REG_FLUSH_CNTL);
    } else {
        WREG32_SOC15(NBIO,0,regBIF_BX0_REMAP_HDP_MEM_FLUSH_CNTL, (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL);
        WREG32_SOC15(NBIO,0,regBIF_BX0_REMAP_HDP_REG_FLUSH_CNTL, (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_REG_FLUSH_CNTL);
    }
}

unsafe fn nbif_v6_3_1_get_rev_id(adev: *mut amdgpu_device) -> u32 {
    let mut tmp = if amdgpu_ip_version(adev,NBIO_HWIP,0)==IP_VERSION(7,11,4) { RREG32_SOC15(NBIO,0,REGRCC_STRAP0_RCC_DEV0_EPF0_STRAP0_NBIF_4_10) } else if amdgpu_ip_version(adev,NBIO_HWIP,0)==IP_VERSION(7,11,5) { RREG32_SOC15(NBIO,0,REGRCC_STRAP1_RCC_DEV0_EPF0_STRAP0_NBIO_7_11_5) } else { RREG32_SOC15(NBIO,0,regRCC_STRAP0_RCC_DEV0_EPF0_STRAP0) };
    tmp &= RCC_STRAP0_RCC_DEV0_EPF0_STRAP0__STRAP_ATI_REV_ID_DEV0_F0_MASK;
    tmp >>= RCC_STRAP0_RCC_DEV0_EPF0_STRAP0__STRAP_ATI_REV_ID_DEV0_F0__SHIFT;
    tmp
}

unsafe fn nbif_v6_3_1_mc_access_enable(adev:*mut amdgpu_device, enable:bool) {
    if amdgpu_ip_version(adev,NBIO_HWIP,0)==IP_VERSION(7,11,5) { WREG32_SOC15(NBIO,0,REGBIF_BX1_BIF_FB_EN_NBIO_7_11_5, if enable { BIF_BX1_BIF_FB_EN__FB_READ_EN_MASK_NBIO_7_11_5|BIF_BX1_BIF_FB_EN__FB_WRITE_EN_MASK_NBIO_7_11_5 } else { 0 }); } else { WREG32_SOC15(NBIO,0,regBIF_BX0_BIF_FB_EN, if enable { BIF_BX0_BIF_FB_EN__FB_READ_EN_MASK|BIF_BX0_BIF_FB_EN__FB_WRITE_EN_MASK } else { 0 }); }
}
unsafe fn nbif_v6_3_1_get_memsize(adev:*mut amdgpu_device)->u32 { RREG32_SOC15(NBIO,0,regRCC_DEV0_EPF0_RCC_CONFIG_MEMSIZE) }

unsafe fn nbif_v6_3_1_sdma_doorbell_range(adev:*mut amdgpu_device, instance:i32, use_doorbell:bool, doorbell_index:i32, doorbell_size:i32) {
    if instance != 0 { return; }
    let reg = if amdgpu_ip_version(adev,NBIO_HWIP,0)>=IP_VERSION(7,11,4) { regGDC_S2A0_S2A_DOORBELL_ENTRY_2_CTRL_nbif_4_10 } else { regGDC_S2A0_S2A_DOORBELL_ENTRY_2_CTRL };
    let mut v=RREG32_SOC15(NBIO,0,reg);
    if use_doorbell { v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_2_CTRL,S2A_DOORBELL_PORT2_ENABLE,1); v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_2_CTRL,S2A_DOORBELL_PORT2_AWID,0xe); v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_2_CTRL,S2A_DOORBELL_PORT2_RANGE_OFFSET,doorbell_index); v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_2_CTRL,S2A_DOORBELL_PORT2_RANGE_SIZE,doorbell_size); v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_2_CTRL,S2A_DOORBELL_PORT2_AWADDR_31_28_VALUE,3); } else { v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_2_CTRL,S2A_DOORBELL_PORT2_RANGE_SIZE,0); }
    WREG32_SOC15(NBIO,0,reg,v);
}

unsafe fn nbif_v6_3_1_vcn_doorbell_range(adev:*mut amdgpu_device,use_doorbell:bool,doorbell_index:i32,instance:i32) {
    if instance!=0 && amdgpu_ip_version(adev,NBIO_HWIP,0)==IP_VERSION(7,11,4) { return; }
    let mut v=RREG32_SOC15(NBIO,0,if instance!=0 {regGDC_S2A0_S2A_DOORBELL_ENTRY_5_CTRL} else {regGDC_S2A0_S2A_DOORBELL_ENTRY_4_CTRL});
    if use_doorbell { v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_4_CTRL,S2A_DOORBELL_PORT4_ENABLE,1); v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_4_CTRL,S2A_DOORBELL_PORT4_AWID,if instance!=0{7}else{4}); v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_4_CTRL,S2A_DOORBELL_PORT4_RANGE_OFFSET,doorbell_index); v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_4_CTRL,S2A_DOORBELL_PORT4_RANGE_SIZE,8); v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_4_CTRL,S2A_DOORBELL_PORT4_AWADDR_31_28_VALUE,if instance!=0{7}else{4}); } else { v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_4_CTRL,S2A_DOORBELL_PORT4_RANGE_SIZE,0); }
    let reg=if amdgpu_ip_version(adev,NBIO_HWIP,0)>=IP_VERSION(7,11,4){regGDC_S2A0_S2A_DOORBELL_ENTRY_4_CTRL_nbif_4_10}else if instance!=0{regGDC_S2A0_S2A_DOORBELL_ENTRY_5_CTRL}else{regGDC_S2A0_S2A_DOORBELL_ENTRY_4_CTRL}; WREG32_SOC15(NBIO,0,reg,v);
}

unsafe fn nbif_v6_3_1_vpe_doorbell_range(adev:*mut amdgpu_device,instance:i32,use_doorbell:bool,doorbell_index:i32,doorbell_size:i32) { if instance!=0{return;} let mut v=RREG32_SOC15(NBIO,0,regGDC_S2A0_S2A_DOORBELL_ENTRY_5_CTRL); if use_doorbell {v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_5_CTRL,S2A_DOORBELL_PORT5_ENABLE,1);v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_5_CTRL,S2A_DOORBELL_PORT5_AWID,0xf);v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_5_CTRL,S2A_DOORBELL_PORT5_RANGE_OFFSET,doorbell_index);v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_5_CTRL,S2A_DOORBELL_PORT5_RANGE_SIZE,doorbell_size);v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_5_CTRL,S2A_DOORBELL_PORT5_AWADDR_31_28_VALUE,0xf);}else{v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_5_CTRL,S2A_DOORBELL_PORT5_RANGE_SIZE,0);} WREG32_SOC15(NBIO,0,if amdgpu_ip_version(adev,NBIO_HWIP,0)==IP_VERSION(7,11,4)||amdgpu_ip_version(adev,NBIO_HWIP,0)==IP_VERSION(7,11,5){regGDC_S2A0_S2A_DOORBELL_ENTRY_5_CTRL_nbif_4_10}else{regGDC_S2A0_S2A_DOORBELL_ENTRY_5_CTRL},v); }

unsafe fn nbif_v6_3_1_gc_doorbell_init(adev:*mut amdgpu_device){let (a,b)=if amdgpu_ip_version(adev,NBIO_HWIP,0)>=IP_VERSION(7,11,4){(regGDC_S2A0_S2A_DOORBELL_ENTRY_0_CTRL_nbif_4_10,regGDC_S2A0_S2A_DOORBELL_ENTRY_3_CTRL_nbif_4_10)}else{(regGDC_S2A0_S2A_DOORBELL_ENTRY_0_CTRL,regGDC_S2A0_S2A_DOORBELL_ENTRY_3_CTRL)};WREG32_SOC15(NBIO,0,a,0x30000007);WREG32_SOC15(NBIO,0,b,0x3000000d);}
unsafe fn nbif_v6_3_1_enable_doorbell_aperture(adev:*mut amdgpu_device,enable:bool){WREG32_FIELD15_PREREG(NBIO,0,RCC_DEV0_EPF0_RCC_DOORBELL_APER_EN,BIF_DOORBELL_APER_EN,if enable{1}else{0});}
unsafe fn nbif_v6_3_1_enable_doorbell_selfring_aperture(adev:*mut amdgpu_device,enable:bool){let mut t=0;if enable{t=REG_SET_FIELD(t,BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL,DOORBELL_SELFRING_GPA_APER_EN,1)|REG_SET_FIELD(t,BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL,DOORBELL_SELFRING_GPA_APER_MODE,1)|REG_SET_FIELD(t,BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL,DOORBELL_SELFRING_GPA_APER_SIZE,0);WREG32_SOC15(NBIO,0,regBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_BASE_LOW,lower_32_bits((*adev).doorbell.base));WREG32_SOC15(NBIO,0,regBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_BASE_HIGH,upper_32_bits((*adev).doorbell.base));}WREG32_SOC15(NBIO,0,regBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL,t);}

unsafe fn nbif_v6_3_1_ih_doorbell_range(adev:*mut amdgpu_device,use_doorbell:bool,doorbell_index:i32){let r=if amdgpu_ip_version(adev,NBIO_HWIP,0)>=IP_VERSION(7,11,4){regGDC_S2A0_S2A_DOORBELL_ENTRY_1_CTRL_nbif_4_10}else{regGDC_S2A0_S2A_DOORBELL_ENTRY_1_CTRL};let mut v=RREG32_SOC15(NBIO,0,r);if use_doorbell{v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_1_CTRL,S2A_DOORBELL_PORT1_ENABLE,1);v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_1_CTRL,S2A_DOORBELL_PORT1_AWID,0);v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_1_CTRL,S2A_DOORBELL_PORT1_RANGE_OFFSET,doorbell_index);v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_1_CTRL,S2A_DOORBELL_PORT1_RANGE_SIZE,2);v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_1_CTRL,S2A_DOORBELL_PORT1_AWADDR_31_28_VALUE,0);}else{v=REG_SET_FIELD(v,GDC_S2A0_S2A_DOORBELL_ENTRY_1_CTRL,S2A_DOORBELL_PORT1_RANGE_SIZE,0);}WREG32_SOC15(NBIO,0,r,v);}

unsafe fn nbif_v6_3_1_ih_control(adev:*mut amdgpu_device){if amdgpu_ip_version(adev,NBIO_HWIP,0)==IP_VERSION(7,11,5){WREG32_SOC15(NBIO,0,REGBIF_BX1_INTERRUPT_CNTL2_NBIO_7_11_5,(*adev).dummy_page_addr>>8);}else{WREG32_SOC15(NBIO,0,regBIF_BX0_INTERRUPT_CNTL2,(*adev).dummy_page_addr>>8);}let mut v=RREG32_SOC15(NBIO,0,REGBIF_BX1_INTERRUPT_CNTL_NBIO_7_11_5);v=REG_SET_FIELD(v,BIF_BX0_INTERRUPT_CNTL,IH_DUMMY_RD_OVERRIDE,0);v=REG_SET_FIELD(v,BIF_BX0_INTERRUPT_CNTL,IH_REQ_NONSNOOP_EN,0);WREG32_SOC15(NBIO,0,if amdgpu_ip_version(adev,NBIO_HWIP,0)==IP_VERSION(7,11,5){REGBIF_BX1_INTERRUPT_CNTL_NBIO_7_11_5}else{regBIF_BX0_INTERRUPT_CNTL},v);}
unsafe fn nbif_v6_3_1_update_medium_grain_clock_gating(_adev:*mut amdgpu_device,_enable:bool){}
unsafe fn nbif_v6_3_1_update_medium_grain_light_sleep(_adev:*mut amdgpu_device,_enable:bool){}
unsafe fn nbif_v6_3_1_get_clockgating_state(_adev:*mut amdgpu_device,_flags:*mut u64){}

unsafe fn nbif_v6_3_1_get_hdp_flush_req_offset(adev:*mut amdgpu_device)->u32{if amdgpu_ip_version(adev,NBIO_HWIP,0)==IP_VERSION(7,11,5){SOC15_REG_OFFSET(NBIO,0,REGBIF_BX_PF1_GPU_HDP_FLUSH_REQ_NBIO_7_11_5)}else{SOC15_REG_OFFSET(NBIO,0,regBIF_BX_PF0_GPU_HDP_FLUSH_REQ)}}
unsafe fn nbif_v6_3_1_get_hdp_flush_done_offset(adev:*mut amdgpu_device)->u32{if amdgpu_ip_version(adev,NBIO_HWIP,0)==IP_VERSION(7,11,5){SOC15_REG_OFFSET(NBIO,0,REGBIF_BX_PF1_GPU_HDP_FLUSH_DONE_NBIO_7_11_5)}else{SOC15_REG_OFFSET(NBIO,0,regBIF_BX_PF0_GPU_HDP_FLUSH_DONE)}}
unsafe fn nbif_v6_3_1_get_pcie_index_offset(adev:*mut amdgpu_device)->u32{if amdgpu_ip_version(adev,NBIO_HWIP,0)>=IP_VERSION(7,11,4){SOC15_REG_OFFSET(NBIO,0,regBIF_BX0_PCIE_INDEX)}else{SOC15_REG_OFFSET(NBIO,0,regBIF_BX_PF0_RSMU_INDEX)}}
unsafe fn nbif_v6_3_1_get_pcie_data_offset(adev:*mut amdgpu_device)->u32{if amdgpu_ip_version(adev,NBIO_HWIP,0)>=IP_VERSION(7,11,4){SOC15_REG_OFFSET(NBIO,0,regBIF_BX0_PCIE_DATA)}else{SOC15_REG_OFFSET(NBIO,0,regBIF_BX_PF0_RSMU_DATA)}}

pub const MMIO_REG_HOLE_OFFSET:u32=0x80000-PAGE_SIZE;
unsafe fn nbif_v6_3_1_init_registers(adev:*mut amdgpu_device){let mut d=RREG32_SOC15(NBIO,0,regRCC_DEV0_EPF2_STRAP2);d&=!RCC_DEV0_EPF2_STRAP2__STRAP_NO_SOFT_RESET_DEV0_F2_MASK;WREG32_SOC15(NBIO,0,regRCC_DEV0_EPF2_STRAP2,d);}
unsafe fn nbif_v6_3_1_get_rom_offset(adev:*mut amdgpu_device)->u32{let d=RREG32_SOC15(NBIO,0,regREGS_ROM_OFFSET_CTRL);REG_GET_FIELD(d,REGS_ROM_OFFSET_CTRL,ROM_OFFSET)}
unsafe fn nbif_v6_3_1_program_ltr(_adev:*mut amdgpu_device){}
unsafe fn nbif_v6_3_1_program_aspm(_adev:*mut amdgpu_device){}
unsafe fn nbif_v6_3_1_set_reg_remap(adev:*mut amdgpu_device){if !amdgpu_sriov_vf(adev)&&(PAGE_SIZE<=4096){(*adev).rmmio_remap.reg_offset=MMIO_REG_HOLE_OFFSET;(*adev).rmmio_remap.bus_addr=(*adev).rmmio_base+MMIO_REG_HOLE_OFFSET;}else{(*adev).rmmio_remap.reg_offset=SOC15_REG_OFFSET(NBIO,0,if amdgpu_ip_version(adev,NBIO_HWIP,0)==IP_VERSION(7,11,5){REGBIF_BX_PF1_HDP_MEM_COHERENCY_FLUSH_CNTL_NBIO_7_11_5}else{regBIF_BX_PF0_HDP_MEM_COHERENCY_FLUSH_CNTL})<<2;(*adev).rmmio_remap.bus_addr=0;}}

// The following externally visible tables and IRQ callbacks retain the C ABI
// layout and reference the dependency-provided types and constants.
pub static nbif_v6_3_1_hdp_flush_reg: nbio_hdp_flush_reg = nbio_hdp_flush_reg { ref_and_mask_cp0:BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP0_MASK, ref_and_mask_cp1:BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP1_MASK, ref_and_mask_cp2:BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP2_MASK, ref_and_mask_cp3:BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP3_MASK, ref_and_mask_cp4:BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP4_MASK, ref_and_mask_cp5:BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP5_MASK, ref_and_mask_cp6:BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP6_MASK, ref_and_mask_cp7:BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP7_MASK, ref_and_mask_cp8:BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP8_MASK, ref_and_mask_cp9:BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP9_MASK, ref_and_mask_sdma0:BIF_BX_PF0_GPU_HDP_FLUSH_DONE__SDMA0_MASK, ref_and_mask_sdma1:BIF_BX_PF0_GPU_HDP_FLUSH_DONE__SDMA1_MASK };

unsafe fn nbif_v6_3_1_set_ras_err_event_athub_irq_state(adev:*mut amdgpu_device,_src:*mut amdgpu_irq_src,_ty:u32,state:amdgpu_interrupt_state)->i32 { let mut v=RREG32_SOC15(NBIO,0,regBIF_BX0_BIF_DOORBELL_INT_CNTL);v=REG_SET_FIELD(v,BIF_BX0_BIF_DOORBELL_INT_CNTL,RAS_ATHUB_ERR_EVENT_INTERRUPT_DISABLE,if state==AMDGPU_IRQ_STATE_ENABLE{0}else{1});WREG32_SOC15(NBIO,0,regBIF_BX0_BIF_DOORBELL_INT_CNTL,v);0 }
unsafe fn nbif_v6_3_1_process_err_event_athub_irq(_adev:*mut amdgpu_device,_src:*mut amdgpu_irq_src,_entry:*mut amdgpu_iv_entry)->i32 { 0 }
unsafe fn nbif_v6_3_1_handle_ras_err_event_athub_intr_no_bifring(adev:*mut amdgpu_device){let mut v=RREG32_SOC15(NBIO,0,regBIF_BX0_BIF_DOORBELL_INT_CNTL);if REG_GET_FIELD(v,BIF_BX0_BIF_DOORBELL_INT_CNTL,RAS_ATHUB_ERR_EVENT_INTERRUPT_STATUS)!=0{v=REG_SET_FIELD(v,BIF_BX0_BIF_DOORBELL_INT_CNTL,RAS_ATHUB_ERR_EVENT_INTERRUPT_CLEAR,1);WREG32_SOC15(NBIO,0,regBIF_BX0_BIF_DOORBELL_INT_CNTL,v);amdgpu_ras_global_ras_isr(adev);}}
unsafe fn nbif_v6_3_1_init_ras_err_event_athub_interrupt(adev:*mut amdgpu_device)->i32{(*adev).nbio.ras_err_event_athub_irq.funcs=&nbif_v6_3_1_ras_err_event_athub_irq_funcs;(*adev).nbio.ras_err_event_athub_irq.num_types=1;amdgpu_irq_add_id(adev,SOC21_IH_CLIENTID_BIF,NBIF_7_4__SRCID__ERREVENT_ATHUB_INTERRUPT,&mut (*adev).nbio.ras_err_event_athub_irq)}
pub static nbif_v6_3_1_ras_err_event_athub_irq_funcs: amdgpu_irq_src_funcs=amdgpu_irq_src_funcs{set:Some(nbif_v6_3_1_set_ras_err_event_athub_irq_state),process:Some(nbif_v6_3_1_process_err_event_athub_irq)};
pub static mut nbif_v6_3_1_ras: amdgpu_nbio_ras=amdgpu_nbio_ras{handle_ras_err_event_athub_intr_no_bifring:Some(nbif_v6_3_1_handle_ras_err_event_athub_intr_no_bifring),init_ras_err_event_athub_interrupt:Some(nbif_v6_3_1_init_ras_err_event_athub_interrupt)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
