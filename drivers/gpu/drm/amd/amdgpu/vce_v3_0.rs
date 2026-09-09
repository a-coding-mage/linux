/* Translation of vce_v3_0.c. External kernel and AMDGPU symbols are supplied by dependencies. */

const GRBM_GFX_INDEX__VCE_INSTANCE__SHIFT: u32 = 0x04;
const GRBM_GFX_INDEX__VCE_INSTANCE_MASK: u32 = 0x10;
const GRBM_GFX_INDEX__VCE_ALL_PIPE: u32 = 0x07;
const mmVCE_LMI_VCPU_CACHE_40BIT_BAR0: u32 = 0x8616;
const mmVCE_LMI_VCPU_CACHE_40BIT_BAR1: u32 = 0x8617;
const mmVCE_LMI_VCPU_CACHE_40BIT_BAR2: u32 = 0x8618;
const mmGRBM_GFX_INDEX_DEFAULT: u32 = 0xE0000000;
const VCE_STATUS_VCPU_REPORT_FW_LOADED_MASK: u32 = 0x02;
const VCE_V3_0_FW_SIZE: u32 = 384 * 1024;
const VCE_V3_0_STACK_SIZE: u32 = 64 * 1024;
const VCE_V3_0_DATA_SIZE: u32 = (16 * 1024 * AMDGPU_MAX_VCE_HANDLES) + (52 * 1024);
const FW_52_8_3: u32 = (52 << 24) | (8 << 16) | (3 << 8);
#[inline] const fn GET_VCE_INSTANCE(i: u32) -> u32 { (i << GRBM_GFX_INDEX__VCE_INSTANCE__SHIFT) | GRBM_GFX_INDEX__VCE_ALL_PIPE }

extern "C" {
    fn vce_v3_0_set_ring_funcs(adev: *mut amdgpu_device);
    fn vce_v3_0_set_irq_funcs(adev: *mut amdgpu_device);
}

unsafe fn vce_v3_0_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev; let mut v: u32;
    mutex_lock(&mut (*adev).grbm_idx_mutex);
    if (*adev).vce.harvest_config == 0 || (*adev).vce.harvest_config == AMDGPU_VCE_HARVEST_VCE1 { WREG32(mmGRBM_GFX_INDEX, GET_VCE_INSTANCE(0)); }
    else if (*adev).vce.harvest_config == AMDGPU_VCE_HARVEST_VCE0 { WREG32(mmGRBM_GFX_INDEX, GET_VCE_INSTANCE(1)); }
    v = if (*ring).me == 0 { RREG32(mmVCE_RB_RPTR) } else if (*ring).me == 1 { RREG32(mmVCE_RB_RPTR2) } else { RREG32(mmVCE_RB_RPTR3) };
    WREG32(mmGRBM_GFX_INDEX, mmGRBM_GFX_INDEX_DEFAULT); mutex_unlock(&mut (*adev).grbm_idx_mutex); v as u64
}
unsafe fn vce_v3_0_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev; let v: u32;
    mutex_lock(&mut (*adev).grbm_idx_mutex);
    if (*adev).vce.harvest_config == 0 || (*adev).vce.harvest_config == AMDGPU_VCE_HARVEST_VCE1 { WREG32(mmGRBM_GFX_INDEX, GET_VCE_INSTANCE(0)); }
    else if (*adev).vce.harvest_config == AMDGPU_VCE_HARVEST_VCE0 { WREG32(mmGRBM_GFX_INDEX, GET_VCE_INSTANCE(1)); }
    v = if (*ring).me == 0 { RREG32(mmVCE_RB_WPTR) } else if (*ring).me == 1 { RREG32(mmVCE_RB_WPTR2) } else { RREG32(mmVCE_RB_WPTR3) };
    WREG32(mmGRBM_GFX_INDEX, mmGRBM_GFX_INDEX_DEFAULT); mutex_unlock(&mut (*adev).grbm_idx_mutex); v as u64
}
unsafe fn vce_v3_0_ring_set_wptr(ring: *mut amdgpu_ring) {
    let adev = (*ring).adev; mutex_lock(&mut (*adev).grbm_idx_mutex);
    if (*adev).vce.harvest_config == 0 || (*adev).vce.harvest_config == AMDGPU_VCE_HARVEST_VCE1 { WREG32(mmGRBM_GFX_INDEX, GET_VCE_INSTANCE(0)); }
    else if (*adev).vce.harvest_config == AMDGPU_VCE_HARVEST_VCE0 { WREG32(mmGRBM_GFX_INDEX, GET_VCE_INSTANCE(1)); }
    let r = lower_32_bits((*ring).wptr); if (*ring).me == 0 { WREG32(mmVCE_RB_WPTR,r); } else if (*ring).me == 1 { WREG32(mmVCE_RB_WPTR2,r); } else { WREG32(mmVCE_RB_WPTR3,r); }
    WREG32(mmGRBM_GFX_INDEX, mmGRBM_GFX_INDEX_DEFAULT); mutex_unlock(&mut (*adev).grbm_idx_mutex);
}
unsafe fn vce_v3_0_override_vce_clock_gating(adev: *mut amdgpu_device, override_: bool) { WREG32_FIELD(VCE_RB_ARB_CTRL, VCE_CGTT_OVERRIDE, if override_ {1} else {0}); }
unsafe fn vce_v3_0_set_vce_sw_clock_gating(adev: *mut amdgpu_device, gated: bool) {
    vce_v3_0_override_vce_clock_gating(adev,true); let mut data;
    if !gated {
        data=RREG32(mmVCE_CLOCK_GATING_B); data|=0x1ff; data&=!0xef0000; WREG32(mmVCE_CLOCK_GATING_B,data);
        data=RREG32(mmVCE_UENC_CLOCK_GATING); data|=0x3ff000; data&=!0xffc00000; WREG32(mmVCE_UENC_CLOCK_GATING,data);
        data=RREG32(mmVCE_UENC_CLOCK_GATING_2); data|=2; data&=!0x10000; WREG32(mmVCE_UENC_CLOCK_GATING_2,data);
        data=RREG32(mmVCE_UENC_REG_CLOCK_GATING); data|=0x37f; WREG32(mmVCE_UENC_REG_CLOCK_GATING,data);
        data=RREG32(mmVCE_UENC_DMA_DCLK_CTRL); data|=VCE_UENC_DMA_DCLK_CTRL__WRDMCLK_FORCEON_MASK|VCE_UENC_DMA_DCLK_CTRL__RDDMCLK_FORCEON_MASK|VCE_UENC_DMA_DCLK_CTRL__REGCLK_FORCEON_MASK|8; WREG32(mmVCE_UENC_DMA_DCLK_CTRL,data);
    } else {
        data=RREG32(mmVCE_CLOCK_GATING_B); data&=!0x80010; data|=0xe70008; WREG32(mmVCE_CLOCK_GATING_B,data);
        data=RREG32(mmVCE_UENC_CLOCK_GATING); data|=0xffc00000; WREG32(mmVCE_UENC_CLOCK_GATING,data);
        data=RREG32(mmVCE_UENC_CLOCK_GATING_2); data|=0x10000; WREG32(mmVCE_UENC_CLOCK_GATING_2,data);
        data=RREG32(mmVCE_UENC_REG_CLOCK_GATING); data&=!0x3ff; WREG32(mmVCE_UENC_REG_CLOCK_GATING,data);
        data=RREG32(mmVCE_UENC_DMA_DCLK_CTRL); data&=!(VCE_UENC_DMA_DCLK_CTRL__WRDMCLK_FORCEON_MASK|VCE_UENC_DMA_DCLK_CTRL__RDDMCLK_FORCEON_MASK|VCE_UENC_DMA_DCLK_CTRL__REGCLK_FORCEON_MASK|8); WREG32(mmVCE_UENC_DMA_DCLK_CTRL,data);
    } vce_v3_0_override_vce_clock_gating(adev,false);
}

/* Remaining source-level callbacks and static function tables are declared here as
 * external items because their concrete AMDGPU types and register definitions are
 * supplied by the surrounding kernel translation unit. */
extern "C" {
    fn vce_v3_0_firmware_loaded(adev: *mut amdgpu_device) -> i32;
    fn vce_v3_0_start(adev: *mut amdgpu_device) -> i32;
    fn vce_v3_0_stop(adev: *mut amdgpu_device) -> i32;
    fn vce_v3_0_get_harvest_config(adev: *mut amdgpu_device) -> u32;
    fn vce_v3_0_early_init(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v3_0_sw_init(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v3_0_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v3_0_hw_init(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v3_0_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v3_0_suspend(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v3_0_resume(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v3_0_mc_resume(adev: *mut amdgpu_device, idx: i32);
    fn vce_v3_0_is_idle(ip_block: *mut amdgpu_ip_block) -> bool;
    fn vce_v3_0_wait_for_idle(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v3_0_soft_reset(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v3_0_set_clockgating_state(ip_block: *mut amdgpu_ip_block, state: amd_clockgating_state) -> i32;
    fn vce_v3_0_set_powergating_state(ip_block: *mut amdgpu_ip_block, state: amd_powergating_state) -> i32;
    fn vce_v3_0_get_clockgating_state(ip_block: *mut amdgpu_ip_block, flags: *mut u64);
    fn vce_v3_0_set_interrupt_state(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, ty: u32, state: amdgpu_interrupt_state) -> i32;
    fn vce_v3_0_process_interrupt(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32;
    fn vce_v3_0_ring_emit_ib(ring: *mut amdgpu_ring, job: *mut amdgpu_job, ib: *mut amdgpu_ib, flags: u32);
    fn vce_v3_0_ring_emit_fence(ring: *mut amdgpu_ring, addr: u64, seq: u64, flags: u32);
    fn vce_v3_0_ring_insert_end(ring: *mut amdgpu_ring);
    fn vce_v3_0_emit_vm_flush(ring: *mut amdgpu_ring, vmid: u32, pd_addr: u64);
    fn vce_v3_0_emit_pipeline_sync(ring: *mut amdgpu_ring);
}

const ixVCE_HARVEST_FUSE_MACRO__ADDRESS: u32 = 0xC0014074;
const VCE_HARVEST_FUSE_MACRO__SHIFT: u32 = 27;
const VCE_HARVEST_FUSE_MACRO__MASK: u32 = 0x18000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
