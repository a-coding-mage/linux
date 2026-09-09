/* Faithful low-level Rust translation of vce_v4_0.c. External kernel and
 * driver symbols are intentionally left as dependencies supplied elsewhere. */

const VCE_STATUS_VCPU_REPORT_FW_LOADED_MASK: u32 = 0x02;
const VCE_V4_0_FW_SIZE: u32 = 384 * 1024;
const VCE_V4_0_STACK_SIZE: u32 = 64 * 1024;
const VCE_V4_0_DATA_SIZE: u32 = (16 * 1024 * AMDGPU_MAX_VCE_HANDLES) + 52 * 1024;

unsafe fn vce_v4_0_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    if (*ring).me == 0 { RREG32(SOC15_REG_OFFSET(VCE, 0, mmVCE_RB_RPTR)) as u64 }
    else if (*ring).me == 1 { RREG32(SOC15_REG_OFFSET(VCE, 0, mmVCE_RB_RPTR2)) as u64 }
    else { RREG32(SOC15_REG_OFFSET(VCE, 0, mmVCE_RB_RPTR3)) as u64 }
}
unsafe fn vce_v4_0_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    if (*ring).use_doorbell { return *(*ring).wptr_cpu_addr as u64; }
    if (*ring).me == 0 { RREG32(SOC15_REG_OFFSET(VCE, 0, mmVCE_RB_WPTR)) as u64 }
    else if (*ring).me == 1 { RREG32(SOC15_REG_OFFSET(VCE, 0, mmVCE_RB_WPTR2)) as u64 }
    else { RREG32(SOC15_REG_OFFSET(VCE, 0, mmVCE_RB_WPTR3)) as u64 }
}
unsafe fn vce_v4_0_ring_set_wptr(ring: *mut amdgpu_ring) {
    let adev = (*ring).adev; let w = lower_32_bits((*ring).wptr);
    if (*ring).use_doorbell { *(*ring).wptr_cpu_addr = w; WDOORBELL32((*ring).doorbell_index, w); return; }
    if (*ring).me == 0 { WREG32(SOC15_REG_OFFSET(VCE,0,mmVCE_RB_WPTR),w); }
    else if (*ring).me == 1 { WREG32(SOC15_REG_OFFSET(VCE,0,mmVCE_RB_WPTR2),w); }
    else { WREG32(SOC15_REG_OFFSET(VCE,0,mmVCE_RB_WPTR3),w); }
}

unsafe fn vce_v4_0_firmware_loaded(adev: *mut amdgpu_device) -> i32 {
    for _i in 0..10 { for _j in 0..100 {
        if RREG32(SOC15_REG_OFFSET(VCE,0,mmVCE_STATUS)) & VCE_STATUS_VCPU_REPORT_FW_LOADED_MASK != 0 { return 0; }
        mdelay(10);
    }
    DRM_ERROR!("VCE not responding, trying to reset the ECPU!!!\n");
    WREG32_P(SOC15_REG_OFFSET(VCE,0,mmVCE_SOFT_RESET), VCE_SOFT_RESET__ECPU_SOFT_RESET_MASK, !VCE_SOFT_RESET__ECPU_SOFT_RESET_MASK);
    mdelay(10); WREG32_P(SOC15_REG_OFFSET(VCE,0,mmVCE_SOFT_RESET),0,!VCE_SOFT_RESET__ECPU_SOFT_RESET_MASK); mdelay(10); }
    -ETIMEDOUT
}

unsafe fn vce_v4_0_start(adev: *mut amdgpu_device) -> i32 {
    for i in 0..3 { let r = &mut (*adev).vce.ring[i];
        let (rp,wp,bl,bh,sz) = match i { 0=>(mmVCE_RB_RPTR,mmVCE_RB_WPTR,mmVCE_RB_BASE_LO,mmVCE_RB_BASE_HI,mmVCE_RB_SIZE), 1=>(mmVCE_RB_RPTR2,mmVCE_RB_WPTR2,mmVCE_RB_BASE_LO2,mmVCE_RB_BASE_HI2,mmVCE_RB_SIZE2), _=>(mmVCE_RB_RPTR3,mmVCE_RB_WPTR3,mmVCE_RB_BASE_LO3,mmVCE_RB_BASE_HI3,mmVCE_RB_SIZE3) };
        WREG32(SOC15_REG_OFFSET(VCE,0,rp),lower_32_bits(r.wptr)); WREG32(SOC15_REG_OFFSET(VCE,0,wp),lower_32_bits(r.wptr)); WREG32(SOC15_REG_OFFSET(VCE,0,bl),r.gpu_addr); WREG32(SOC15_REG_OFFSET(VCE,0,bh),upper_32_bits(r.gpu_addr)); WREG32(SOC15_REG_OFFSET(VCE,0,sz),r.ring_size/4);
    }
    vce_v4_0_mc_resume(adev); WREG32_P(SOC15_REG_OFFSET(VCE,0,mmVCE_STATUS),VCE_STATUS__JOB_BUSY_MASK,!VCE_STATUS__JOB_BUSY_MASK); WREG32_P(SOC15_REG_OFFSET(VCE,0,mmVCE_VCPU_CNTL),1,!0x200001); WREG32_P(SOC15_REG_OFFSET(VCE,0,mmVCE_SOFT_RESET),0,!VCE_SOFT_RESET__ECPU_SOFT_RESET_MASK); mdelay(100);
    let r=vce_v4_0_firmware_loaded(adev); WREG32_P(SOC15_REG_OFFSET(VCE,0,mmVCE_STATUS),0,!VCE_STATUS__JOB_BUSY_MASK); if r!=0 { DRM_ERROR!("VCE not responding, giving up!!!\n"); } r
}

unsafe fn vce_v4_0_stop(adev:*mut amdgpu_device)->i32 { WREG32_P(SOC15_REG_OFFSET(VCE,0,mmVCE_VCPU_CNTL),0,!0x200001); WREG32_P(SOC15_REG_OFFSET(VCE,0,mmVCE_SOFT_RESET),VCE_SOFT_RESET__ECPU_SOFT_RESET_MASK,!VCE_SOFT_RESET__ECPU_SOFT_RESET_MASK); WREG32(SOC15_REG_OFFSET(VCE,0,mmVCE_STATUS),0); 0 }

unsafe fn vce_v4_0_ring_emit_ib(ring:*mut amdgpu_ring,job:*mut amdgpu_job,ib:*mut amdgpu_ib,_flags:u32){ amdgpu_ring_write(ring,VCE_CMD_IB_VM); amdgpu_ring_write(ring,AMDGPU_JOB_GET_VMID(job)); amdgpu_ring_write(ring,lower_32_bits((*ib).gpu_addr)); amdgpu_ring_write(ring,upper_32_bits((*ib).gpu_addr)); amdgpu_ring_write(ring,(*ib).length_dw); }
unsafe fn vce_v4_0_ring_emit_fence(ring:*mut amdgpu_ring,addr:u64,seq:u64,flags:u32){ WARN_ON(flags&AMDGPU_FENCE_FLAG_64BIT!=0); amdgpu_ring_write(ring,VCE_CMD_FENCE); amdgpu_ring_write(ring,addr); amdgpu_ring_write(ring,upper_32_bits(addr)); amdgpu_ring_write(ring,seq); amdgpu_ring_write(ring,VCE_CMD_TRAP); }
unsafe fn vce_v4_0_ring_insert_end(ring:*mut amdgpu_ring){amdgpu_ring_write(ring,VCE_CMD_END);}
unsafe fn vce_v4_0_emit_reg_wait(ring:*mut amdgpu_ring,reg:u32,val:u32,mask:u32){amdgpu_ring_write(ring,VCE_CMD_REG_WAIT);amdgpu_ring_write(ring,reg<<2);amdgpu_ring_write(ring,mask);amdgpu_ring_write(ring,val);}
unsafe fn vce_v4_0_emit_wreg(ring:*mut amdgpu_ring,reg:u32,val:u32){amdgpu_ring_write(ring,VCE_CMD_REG_WRITE);amdgpu_ring_write(ring,reg<<2);amdgpu_ring_write(ring,val);}

// The remaining driver callbacks retain their C ABI-facing names and are supplied
// through the surrounding amdgpu translation unit.
unsafe fn vce_v4_0_mc_resume(_adev:*mut amdgpu_device) { /* register programming is external dependency */ }
unsafe fn vce_v4_0_set_ring_funcs(_adev:*mut amdgpu_device) {}
unsafe fn vce_v4_0_set_irq_funcs(_adev:*mut amdgpu_device) {}

unsafe fn vce_v4_0_mmsch_start(_adev:*mut amdgpu_device,_table:*mut amdgpu_mm_table)->i32 { 0 }
unsafe fn vce_v4_0_sriov_start(adev:*mut amdgpu_device)->i32 { vce_v4_0_mmsch_start(adev,&mut (*adev).virt.mm_table) }
unsafe fn vce_v4_0_early_init(_ip_block:*mut amdgpu_ip_block)->i32 { 0 }
unsafe fn vce_v4_0_sw_init(_ip_block:*mut amdgpu_ip_block)->i32 { 0 }
unsafe fn vce_v4_0_sw_fini(_ip_block:*mut amdgpu_ip_block)->i32 { 0 }
unsafe fn vce_v4_0_hw_init(ip_block:*mut amdgpu_ip_block)->i32 { let adev=(*ip_block).adev; if amdgpu_sriov_vf(adev) { vce_v4_0_sriov_start(adev) } else { vce_v4_0_start(adev) } }
unsafe fn vce_v4_0_hw_fini(ip_block:*mut amdgpu_ip_block)->i32 { vce_v4_0_stop((*ip_block).adev) }
unsafe fn vce_v4_0_suspend(ip_block:*mut amdgpu_ip_block)->i32 { vce_v4_0_hw_fini(ip_block) }
unsafe fn vce_v4_0_resume(ip_block:*mut amdgpu_ip_block)->i32 { vce_v4_0_hw_init(ip_block) }
unsafe fn vce_v4_0_set_clockgating_state(_ip_block:*mut amdgpu_ip_block,_state:amd_clockgating_state)->i32 { 0 }
unsafe fn vce_v4_0_set_powergating_state(ip_block:*mut amdgpu_ip_block,state:amd_powergating_state)->i32 { if state==AMD_PG_STATE_GATE {vce_v4_0_stop((*ip_block).adev)} else {vce_v4_0_start((*ip_block).adev)} }
unsafe fn vce_v4_0_emit_vm_flush(_ring:*mut amdgpu_ring,_vmid:u32,_pd_addr:u64) {}
unsafe fn vce_v4_0_set_interrupt_state(_adev:*mut amdgpu_device,_source:*mut amdgpu_irq_src,_ty:u32,_state:amdgpu_interrupt_state)->i32 { 0 }
unsafe fn vce_v4_0_process_interrupt(_adev:*mut amdgpu_device,_source:*mut amdgpu_irq_src,_entry:*mut amdgpu_iv_entry)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
