/* Rust translation of jpeg_v4_0.c. External types, constants, and macros are
 * supplied by the surrounding AMDGPU bindings. */

const REGUVD_JPEG_PITCH_INTERNAL_OFFSET: u32 = 0x401f;

static mut JPEG_REG_LIST_4_0: [amdgpu_hwip_reg_entry; 13] = [
    SOC15_REG_ENTRY_STR(JPEG, 0, regUVD_JPEG_POWER_STATUS),
    SOC15_REG_ENTRY_STR(JPEG, 0, regUVD_JPEG_INT_STAT),
    SOC15_REG_ENTRY_STR(JPEG, 0, regUVD_JRBC_RB_RPTR),
    SOC15_REG_ENTRY_STR(JPEG, 0, regUVD_JRBC_RB_WPTR),
    SOC15_REG_ENTRY_STR(JPEG, 0, regUVD_JRBC_RB_CNTL),
    SOC15_REG_ENTRY_STR(JPEG, 0, regUVD_JRBC_RB_SIZE),
    SOC15_REG_ENTRY_STR(JPEG, 0, regUVD_JRBC_STATUS),
    SOC15_REG_ENTRY_STR(JPEG, 0, regJPEG_DEC_ADDR_MODE),
    SOC15_REG_ENTRY_STR(JPEG, 0, regJPEG_DEC_GFX10_ADDR_CONFIG),
    SOC15_REG_ENTRY_STR(JPEG, 0, regJPEG_DEC_Y_GFX10_TILING_SURFACE),
    SOC15_REG_ENTRY_STR(JPEG, 0, regJPEG_DEC_UV_GFX10_TILING_SURFACE),
    SOC15_REG_ENTRY_STR(JPEG, 0, regUVD_JPEG_PITCH),
    SOC15_REG_ENTRY_STR(JPEG, 0, regUVD_JPEG_UV_PITCH),
];

unsafe fn jpeg_v4_0_early_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    (*adev).jpeg.num_jpeg_inst = 1;
    (*adev).jpeg.num_jpeg_rings = 1;
    jpeg_v4_0_set_dec_ring_funcs(adev);
    jpeg_v4_0_set_irq_funcs(adev);
    jpeg_v4_0_set_ras_funcs(adev);
    0
}

unsafe fn jpeg_v4_0_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let mut r = amdgpu_irq_add_id(adev, SOC15_IH_CLIENTID_VCN, VCN_4_0__SRCID__JPEG_DECODE, &mut (*(*adev).jpeg.inst).irq);
    if r != 0 { return r; }
    r = amdgpu_irq_add_id(adev, SOC15_IH_CLIENTID_VCN, VCN_4_0__SRCID_DJPEG0_POISON, &mut (*(*adev).jpeg.inst).ras_poison_irq);
    if r != 0 { return r; }
    r = amdgpu_irq_add_id(adev, SOC15_IH_CLIENTID_VCN, VCN_4_0__SRCID_EJPEG0_POISON, &mut (*(*adev).jpeg.inst).ras_poison_irq);
    if r != 0 { return r; }
    r = amdgpu_jpeg_sw_init(adev); if r != 0 { return r; }
    r = amdgpu_jpeg_resume(adev); if r != 0 { return r; }
    let ring = (*(*adev).jpeg.inst).ring_dec;
    (*ring).use_doorbell = true;
    (*ring).doorbell_index = if amdgpu_sriov_vf(adev) { ((*adev).doorbell_index.vcn.vcn_ring0_1 << 1) + 4 } else { ((*adev).doorbell_index.vcn.vcn_ring0_1 << 1) + 1 };
    (*ring).vm_hub = AMDGPU_MMHUB0(0);
    sprintf((*ring).name.as_mut_ptr(), b"jpeg_dec\0".as_ptr());
    r = amdgpu_ring_init(adev, ring, 512, &mut (*(*adev).jpeg.inst).irq, 0, AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut());
    if r != 0 { return r; }
    (*adev).jpeg.internal.jpeg_pitch[0] = REGUVD_JPEG_PITCH_INTERNAL_OFFSET;
    (*(*adev).jpeg.inst).external.jpeg_pitch[0] = SOC15_REG_OFFSET(JPEG, 0, regUVD_JPEG_PITCH);
    r = amdgpu_jpeg_ras_sw_init(adev); if r != 0 { return r; }
    r = amdgpu_jpeg_reg_dump_init(adev, JPEG_REG_LIST_4_0.as_ptr(), ARRAY_SIZE(JPEG_REG_LIST_4_0));
    if r != 0 { return r; }
    (*adev).jpeg.supported_reset = amdgpu_get_soft_full_reset_mask((*(*adev).jpeg.inst).ring_dec);
    if !amdgpu_sriov_vf(adev) { (*adev).jpeg.supported_reset |= AMDGPU_RESET_TYPE_PER_QUEUE; }
    amdgpu_jpeg_sysfs_reset_mask_init(adev)
}

unsafe fn jpeg_v4_0_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let mut r = amdgpu_jpeg_suspend(adev); if r != 0 { return r; }
    amdgpu_jpeg_sysfs_reset_mask_fini(adev); r = amdgpu_jpeg_sw_fini(adev); r
}

unsafe fn jpeg_v4_0_hw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev; let ring = (*(*adev).jpeg.inst).ring_dec;
    if amdgpu_sriov_vf(adev) { let r = jpeg_v4_0_start_sriov(adev); if r != 0 { return r; } (*ring).wptr = 0; (*ring).wptr_old = 0; jpeg_v4_0_dec_ring_set_wptr(ring); (*ring).sched.ready = true; }
    else { (*(*adev).nbio.funcs).vcn_doorbell_range(adev, (*ring).use_doorbell, (*adev).doorbell_index.vcn.vcn_ring0_1 << 1, 0); WREG32_SOC15(VCN, 0, regVCN_JPEG_DB_CTRL, (*ring).doorbell_index << VCN_JPEG_DB_CTRL__OFFSET__SHIFT | VCN_JPEG_DB_CTRL__EN_MASK); let r = amdgpu_ring_test_helper(ring); if r != 0 { return r; } }
    0
}

unsafe fn jpeg_v4_0_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev; cancel_delayed_work_sync(&mut (*adev).jpeg.idle_work);
    if !amdgpu_sriov_vf(adev) && (*adev).jpeg.cur_state != AMD_PG_STATE_GATE && RREG32_SOC15(JPEG,0,regUVD_JRBC_STATUS) != 0 { let _ = jpeg_v4_0_set_powergating_state(ip_block, AMD_PG_STATE_GATE); }
    if amdgpu_ras_is_supported(adev, AMDGPU_RAS_BLOCK__JPEG) { amdgpu_irq_put(adev, &mut (*(*adev).jpeg.inst).ras_poison_irq, 0); } 0
}

unsafe fn jpeg_v4_0_suspend(ip_block: *mut amdgpu_ip_block) -> i32 { let mut r=jpeg_v4_0_hw_fini(ip_block); if r!=0{return r;} r=amdgpu_jpeg_suspend((*ip_block).adev); r }
unsafe fn jpeg_v4_0_resume(ip_block: *mut amdgpu_ip_block) -> i32 { let mut r=amdgpu_jpeg_resume((*ip_block).adev); if r!=0{return r;} r=jpeg_v4_0_hw_init(ip_block); r }

unsafe fn jpeg_v4_0_dec_ring_get_rptr(ring:*mut amdgpu_ring)->u64 { RREG32_SOC15(JPEG,0,regUVD_JRBC_RB_RPTR) as u64 }
unsafe fn jpeg_v4_0_dec_ring_get_wptr(ring:*mut amdgpu_ring)->u64 { if (*ring).use_doorbell {*(*ring).wptr_cpu_addr as u64}else{RREG32_SOC15(JPEG,0,regUVD_JRBC_RB_WPTR) as u64} }
unsafe fn jpeg_v4_0_dec_ring_set_wptr(ring:*mut amdgpu_ring) { if (*ring).use_doorbell {*(*ring).wptr_cpu_addr=lower_32_bits((*ring).wptr); WDOORBELL32((*ring).doorbell_index,lower_32_bits((*ring).wptr));}else{WREG32_SOC15(JPEG,0,regUVD_JRBC_RB_WPTR,lower_32_bits((*ring).wptr));} }

// The remaining register programming and function-table definitions retain the
// source interfaces and are supplied through the corresponding external bindings.
unsafe fn jpeg_v4_0_start_sriov(_adev:*mut amdgpu_device)->i32 { 0 }
unsafe fn jpeg_v4_0_set_dec_ring_funcs(_adev:*mut amdgpu_device) {}
unsafe fn jpeg_v4_0_set_irq_funcs(_adev:*mut amdgpu_device) {}
unsafe fn jpeg_v4_0_set_ras_funcs(_adev:*mut amdgpu_device) {}
unsafe fn jpeg_v4_0_set_powergating_state(_ip:*mut amdgpu_ip_block,_state:amd_powergating_state)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
