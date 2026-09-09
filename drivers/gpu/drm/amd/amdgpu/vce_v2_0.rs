/*
 * Copyright 2013 Advanced Micro Devices, Inc.
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to permit
 * persons to whom the Software is furnished to do so, subject to the following
 * conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 * USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: Christian König <christian.koenig@amd.com>
 */

// External kernel and AMDGPU declarations are supplied by the surrounding translation unit.

const VCE_V2_0_DATA_ENTRY_SIZE: u32 = 24 * 1024;
const VCE_V2_0_FW_SIZE: u32 = 256 * 1024;
const VCE_V2_0_STACK_SIZE: u32 = 64 * 1024;
const VCE_V2_0_DATA_SIZE: u32 = VCE_V2_0_DATA_ENTRY_SIZE * (AMDGPU_MAX_VCE_HANDLES + 1);
const VCE_STATUS_VCPU_REPORT_FW_LOADED_MASK: u32 = 0x02;

unsafe fn vce_v2_0_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    if (*ring).me == 0 { RREG32!(adev, mmVCE_RB_RPTR) as u64 } else { RREG32!(adev, mmVCE_RB_RPTR2) as u64 }
}

unsafe fn vce_v2_0_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    if (*ring).me == 0 { RREG32!(adev, mmVCE_RB_WPTR) as u64 } else { RREG32!(adev, mmVCE_RB_WPTR2) as u64 }
}

unsafe fn vce_v2_0_ring_set_wptr(ring: *mut amdgpu_ring) {
    let adev = (*ring).adev;
    if (*ring).me == 0 { WREG32!(adev, mmVCE_RB_WPTR, lower_32_bits((*ring).wptr)); }
    else { WREG32!(adev, mmVCE_RB_WPTR2, lower_32_bits((*ring).wptr)); }
}

unsafe fn vce_v2_0_lmi_clean(adev: *mut amdgpu_device) -> i32 {
    for _i in 0..10 { for _j in 0..100 {
        let status = RREG32!(adev, mmVCE_LMI_STATUS);
        if status & 0x337f != 0 { return 0; }
        mdelay(10);
    }}
    -ETIMEDOUT
}

unsafe fn vce_v2_0_firmware_loaded(adev: *mut amdgpu_device) -> i32 {
    for _i in 0..10 { for _j in 0..100 {
        let status = RREG32!(adev, mmVCE_STATUS);
        if status & VCE_STATUS_VCPU_REPORT_FW_LOADED_MASK != 0 { return 0; }
        mdelay(10);
    }
    DRM_ERROR!("VCE not responding, trying to reset the ECPU!!!\n");
    WREG32_P!(adev, mmVCE_SOFT_RESET, VCE_SOFT_RESET__ECPU_SOFT_RESET_MASK, !VCE_SOFT_RESET__ECPU_SOFT_RESET_MASK);
    mdelay(10);
    WREG32_P!(adev, mmVCE_SOFT_RESET, 0, !VCE_SOFT_RESET__ECPU_SOFT_RESET_MASK);
    mdelay(10);
    }
    -ETIMEDOUT
}

unsafe fn vce_v2_0_disable_cg(adev: *mut amdgpu_device) { WREG32!(adev, mmVCE_CGTT_CLK_OVERRIDE, 7); }

unsafe fn vce_v2_0_init_cg(adev: *mut amdgpu_device) {
    let mut tmp = RREG32!(adev, mmVCE_CLOCK_GATING_A); tmp &= !0xfff; tmp |= (0 << 0) | (4 << 4); tmp |= 0x40000; WREG32!(adev, mmVCE_CLOCK_GATING_A, tmp);
    tmp = RREG32!(adev, mmVCE_UENC_CLOCK_GATING); tmp &= !0xfff; tmp |= (0 << 0) | (4 << 4); WREG32!(adev, mmVCE_UENC_CLOCK_GATING, tmp);
    tmp = RREG32!(adev, mmVCE_CLOCK_GATING_B); tmp |= 0x10; tmp &= !0x100000; WREG32!(adev, mmVCE_CLOCK_GATING_B, tmp);
}

unsafe fn vce_v2_0_mc_resume(adev: *mut amdgpu_device) {
    WREG32_P!(adev, mmVCE_CLOCK_GATING_A, 0, !(1 << 16)); WREG32_P!(adev, mmVCE_UENC_CLOCK_GATING, 0x1FF000, !0xFF9FF000); WREG32_P!(adev, mmVCE_UENC_REG_CLOCK_GATING, 0x3F, !0x3F); WREG32!(adev, mmVCE_CLOCK_GATING_B, 0xf7);
    WREG32!(adev, mmVCE_LMI_CTRL, 0x00398000); WREG32_P!(adev, mmVCE_LMI_CACHE_CTRL, 0, !0x1); WREG32!(adev, mmVCE_LMI_SWAP_CNTL, 0); WREG32!(adev, mmVCE_LMI_SWAP_CNTL1, 0); WREG32!(adev, mmVCE_LMI_VM_CTRL, 0);
    WREG32!(adev, mmVCE_LMI_VCPU_CACHE_40BIT_BAR, (*adev).vce.gpu_addr >> 8);
    let mut offset = AMDGPU_VCE_FIRMWARE_OFFSET; let mut size = VCE_V2_0_FW_SIZE - AMDGPU_VCE_FIRMWARE_OFFSET; WREG32!(adev, mmVCE_VCPU_CACHE_OFFSET0, offset & 0x7fffffff); WREG32!(adev, mmVCE_VCPU_CACHE_SIZE0, size);
    offset += size; size = VCE_V2_0_STACK_SIZE; WREG32!(adev, mmVCE_VCPU_CACHE_OFFSET1, offset & 0x7fffffff); WREG32!(adev, mmVCE_VCPU_CACHE_SIZE1, size);
    offset += size; size = VCE_V2_0_DATA_SIZE; WREG32!(adev, mmVCE_VCPU_CACHE_OFFSET2, offset & 0x7fffffff); WREG32!(adev, mmVCE_VCPU_CACHE_SIZE2, size);
    WREG32_P!(adev, mmVCE_LMI_CTRL2, 0, !0x100); WREG32_FIELD!(adev, VCE_SYS_INT_EN, VCE_SYS_INT_TRAP_INTERRUPT_EN, 1);
}

unsafe fn vce_v2_0_is_idle(ip_block: *mut amdgpu_ip_block) -> bool { !(RREG32!((*ip_block).adev, mmSRBM_STATUS2) & SRBM_STATUS2__VCE_BUSY_MASK != 0) }
unsafe fn vce_v2_0_wait_for_idle(ip_block: *mut amdgpu_ip_block) -> i32 { for _ in 0..(*(*ip_block).adev).usec_timeout { if vce_v2_0_is_idle(ip_block) { return 0; }} -ETIMEDOUT }

unsafe fn vce_v2_0_start(adev: *mut amdgpu_device) -> i32 {
    WREG32_P!(adev, mmVCE_STATUS, 1, !1); vce_v2_0_init_cg(adev); vce_v2_0_disable_cg(adev); vce_v2_0_mc_resume(adev);
    let ring = &mut (*adev).vce.ring[0]; WREG32!(adev, mmVCE_RB_RPTR, lower_32_bits(ring.wptr)); WREG32!(adev, mmVCE_RB_WPTR, lower_32_bits(ring.wptr)); WREG32!(adev, mmVCE_RB_BASE_LO, ring.gpu_addr); WREG32!(adev, mmVCE_RB_BASE_HI, upper_32_bits(ring.gpu_addr)); WREG32!(adev, mmVCE_RB_SIZE, ring.ring_size / 4);
    let ring = &mut (*adev).vce.ring[1]; WREG32!(adev, mmVCE_RB_RPTR2, lower_32_bits(ring.wptr)); WREG32!(adev, mmVCE_RB_WPTR2, lower_32_bits(ring.wptr)); WREG32!(adev, mmVCE_RB_BASE_LO2, ring.gpu_addr); WREG32!(adev, mmVCE_RB_BASE_HI2, upper_32_bits(ring.gpu_addr)); WREG32!(adev, mmVCE_RB_SIZE2, ring.ring_size / 4);
    WREG32_FIELD!(adev, VCE_VCPU_CNTL, CLK_EN, 1); WREG32_FIELD!(adev, VCE_SOFT_RESET, ECPU_SOFT_RESET, 1); mdelay(100); WREG32_FIELD!(adev, VCE_SOFT_RESET, ECPU_SOFT_RESET, 0);
    let r = vce_v2_0_firmware_loaded(adev); WREG32_P!(adev, mmVCE_STATUS, 0, !1); if r != 0 { DRM_ERROR!("VCE not responding, giving up!!!\n"); } r
}

unsafe fn vce_v2_0_stop(adev: *mut amdgpu_device) -> i32 {
    if vce_v2_0_lmi_clean(adev) != 0 { drm_info!(adev_to_drm(adev), "VCE is not idle\n"); return 0; }
    let ip_block = amdgpu_device_ip_get_ip_block(adev, AMD_IP_BLOCK_TYPE_VCE); if ip_block.is_null() { return -EINVAL; }
    if vce_v2_0_wait_for_idle(ip_block) != 0 { drm_info!(adev_to_drm(adev), "VCE is busy, Can't set clock gating"); return 0; }
    WREG32_P!(adev, mmVCE_LMI_CTRL2, 1 << 8, !(1 << 8)); let mut status = 0; for _ in 0..100 { status = RREG32!(adev, mmVCE_LMI_STATUS); if status & 0x240 != 0 { break; } mdelay(1); }
    WREG32_P!(adev, mmVCE_VCPU_CNTL, 0, !0x80001); WREG32_P!(adev, mmVCE_SOFT_RESET, 1, !0x1); WREG32!(adev, mmVCE_STATUS, 0); 0
}

unsafe fn vce_v2_0_set_sw_cg(adev: *mut amdgpu_device, gated: bool) {
    let mut tmp;
    if gated { tmp=RREG32!(adev,mmVCE_CLOCK_GATING_B); tmp|=0xe70000; WREG32!(adev,mmVCE_CLOCK_GATING_B,tmp); tmp=RREG32!(adev,mmVCE_UENC_CLOCK_GATING); tmp|=0xff000000; WREG32!(adev,mmVCE_UENC_CLOCK_GATING,tmp); tmp=RREG32!(adev,mmVCE_UENC_REG_CLOCK_GATING); tmp&=!0x3fc; WREG32!(adev,mmVCE_UENC_REG_CLOCK_GATING,tmp); WREG32!(adev,mmVCE_CGTT_CLK_OVERRIDE,0); }
    else { tmp=RREG32!(adev,mmVCE_CLOCK_GATING_B); tmp|=0xe7; tmp&=!0xe70000; WREG32!(adev,mmVCE_CLOCK_GATING_B,tmp); tmp=RREG32!(adev,mmVCE_UENC_CLOCK_GATING); tmp|=0x1fe000; tmp&=!0xff000000; WREG32!(adev,mmVCE_UENC_CLOCK_GATING,tmp); tmp=RREG32!(adev,mmVCE_UENC_REG_CLOCK_GATING); tmp|=0x3fc; WREG32!(adev,mmVCE_UENC_REG_CLOCK_GATING,tmp); }
}

unsafe fn vce_v2_0_set_dyn_cg(adev: *mut amdgpu_device, gated: bool) { let mut tmp=RREG32!(adev,mmVCE_CLOCK_GATING_B); tmp&=!0x00060006; if gated {tmp|=0xe10000;} else {tmp|=0xe1;tmp&=!0xe10000;} WREG32!(adev,mmVCE_CLOCK_GATING_B,tmp); let orig=RREG32!(adev,mmVCE_UENC_CLOCK_GATING); tmp=orig&!0x1fe000&!0xff000000; if tmp!=orig {WREG32!(adev,mmVCE_UENC_CLOCK_GATING,tmp);} let orig=RREG32!(adev,mmVCE_UENC_REG_CLOCK_GATING); tmp=orig&!0x3fc; if tmp!=orig {WREG32!(adev,mmVCE_UENC_REG_CLOCK_GATING,tmp);} WREG32!(adev,mmVCE_UENC_REG_CLOCK_GATING,0); if gated {WREG32!(adev,mmVCE_CGTT_CLK_OVERRIDE,0);} }

unsafe fn vce_v2_0_enable_mgcg(adev:*mut amdgpu_device, enable:bool, sw_cg:bool) { if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_VCE_MGCG) != 0 {if sw_cg {vce_v2_0_set_sw_cg(adev,true)} else {vce_v2_0_set_dyn_cg(adev,true)}} else {vce_v2_0_disable_cg(adev); if sw_cg {vce_v2_0_set_sw_cg(adev,false)} else {vce_v2_0_set_dyn_cg(adev,false)}} }

// The remaining lifecycle, interrupt, and function-table definitions retain the C interfaces below.
// External declarations and table field types are provided by the AMDGPU translation environment.
unsafe fn vce_v2_0_early_init(ip:*mut amdgpu_ip_block)->i32 { let adev=(*ip).adev; let r=amdgpu_vce_early_init(adev); if r!=0{return r;} (*adev).vce.num_rings=2; vce_v2_0_set_ring_funcs(adev); vce_v2_0_set_irq_funcs(adev); 0 }
unsafe fn vce_v2_0_sw_init(ip:*mut amdgpu_ip_block)->i32 { let adev=(*ip).adev; let mut r=amdgpu_irq_add_id(adev,AMDGPU_IRQ_CLIENTID_LEGACY,167,&mut (*adev).vce.irq); if r!=0{return r;} r=amdgpu_vce_sw_init(adev,VCE_V2_0_FW_SIZE+VCE_V2_0_STACK_SIZE+VCE_V2_0_DATA_SIZE); if r!=0{return r;} r=amdgpu_vce_resume(adev); if r!=0{return r;} for i in 0..(*adev).vce.num_rings { let p=amdgpu_vce_get_ring_prio(i); let ring=&mut (*adev).vce.ring[i]; sprintf!(ring.name,"vce%d",i); r=amdgpu_ring_init(adev,ring,512,&mut (*adev).vce.irq,0,p,core::ptr::null_mut()); if r!=0{return r;} } r }
unsafe fn vce_v2_0_sw_fini(ip:*mut amdgpu_ip_block)->i32 { let adev=(*ip).adev; let r=amdgpu_vce_suspend(adev); if r!=0{return r;} amdgpu_vce_sw_fini(adev) }
unsafe fn vce_v2_0_hw_init(ip:*mut amdgpu_ip_block)->i32 { let adev=(*ip).adev; amdgpu_asic_set_vce_clocks(adev,10000,10000); vce_v2_0_enable_mgcg(adev,true,false); for i in 0..(*adev).vce.num_rings {let r=amdgpu_ring_test_helper(&mut (*adev).vce.ring[i]); if r!=0{return r;}} drm_info!(adev_to_drm(adev),"VCE initialized successfully.\n"); 0 }
unsafe fn vce_v2_0_hw_fini(ip:*mut amdgpu_ip_block)->i32 { cancel_delayed_work_sync!(&mut (*(*ip).adev).vce.idle_work); 0 }
unsafe fn vce_v2_0_suspend(ip:*mut amdgpu_ip_block)->i32 { let adev=(*ip).adev; cancel_delayed_work_sync!(&mut (*adev).vce.idle_work); if (*adev).pm.dpm_enabled {amdgpu_dpm_enable_vce(adev,false);} else {amdgpu_asic_set_vce_clocks(adev,0,0); amdgpu_device_ip_set_powergating_state(adev,AMDGPU_IP_BLOCK_TYPE_VCE,AMD_PG_STATE_GATE); amdgpu_device_ip_set_clockgating_state(adev,AMDGPU_IP_BLOCK_TYPE_VCE,AMD_CG_STATE_GATE);} let r=vce_v2_0_hw_fini(ip); if r!=0{return r;} amdgpu_vce_suspend(adev) }
unsafe fn vce_v2_0_resume(ip:*mut amdgpu_ip_block)->i32 { let r=amdgpu_vce_resume((*ip).adev); if r!=0{return r;} vce_v2_0_hw_init(ip) }
unsafe fn vce_v2_0_soft_reset(ip:*mut amdgpu_ip_block)->i32 { let adev=(*ip).adev; WREG32_FIELD!(adev,SRBM_SOFT_RESET,SOFT_RESET_VCE,1); mdelay(5); vce_v2_0_start(adev) }
unsafe fn vce_v2_0_set_interrupt_state(adev:*mut amdgpu_device,_source:*mut amdgpu_irq_src,_type:u32,state:amdgpu_interrupt_state)->i32 {let val=if state==AMDGPU_IRQ_STATE_ENABLE {VCE_SYS_INT_EN__VCE_SYS_INT_TRAP_INTERRUPT_EN_MASK} else {0}; WREG32_P!(adev,mmVCE_SYS_INT_EN,val,!VCE_SYS_INT_EN__VCE_SYS_INT_TRAP_INTERRUPT_EN_MASK); 0}
unsafe fn vce_v2_0_process_interrupt(adev:*mut amdgpu_device,_source:*mut amdgpu_irq_src,entry:*mut amdgpu_iv_entry)->i32 {DRM_DEBUG!("IH: VCE\n"); match (*entry).src_data[0] {0|1=>amdgpu_fence_process(&mut (*adev).vce.ring[(*entry).src_data[0] as usize]), _=>DRM_ERROR!("Unhandled interrupt: %d %d\n",(*entry).src_id,(*entry).src_data[0])}; 0}
unsafe fn vce_v2_0_set_clockgating_state(ip:*mut amdgpu_ip_block,state:amd_clockgating_state)->i32 {vce_v2_0_enable_mgcg((*ip).adev,state==AMD_CG_STATE_GATE,state==AMD_CG_STATE_GATE); 0}
unsafe fn vce_v2_0_set_powergating_state(ip:*mut amdgpu_ip_block,state:amd_powergating_state)->i32 {if state==AMD_PG_STATE_GATE {vce_v2_0_stop((*ip).adev)} else {vce_v2_0_start((*ip).adev)}}

unsafe fn vce_v2_0_set_ring_funcs(adev:*mut amdgpu_device) {for i in 0..(*adev).vce.num_rings {(*adev).vce.ring[i].funcs=&vce_v2_0_ring_funcs;(*adev).vce.ring[i].me=i;}}
unsafe fn vce_v2_0_set_irq_funcs(adev:*mut amdgpu_device) {(*adev).vce.irq.num_types=1;(*adev).vce.irq.funcs=&vce_v2_0_irq_funcs;}

static vce_v2_0_ip_funcs: amd_ip_funcs = amd_ip_funcs { name:"vce_v2_0", early_init:vce_v2_0_early_init, sw_init:vce_v2_0_sw_init, sw_fini:vce_v2_0_sw_fini, hw_init:vce_v2_0_hw_init, hw_fini:vce_v2_0_hw_fini, suspend:vce_v2_0_suspend, resume:vce_v2_0_resume, is_idle:vce_v2_0_is_idle, wait_for_idle:vce_v2_0_wait_for_idle, soft_reset:vce_v2_0_soft_reset, set_clockgating_state:vce_v2_0_set_clockgating_state, set_powergating_state:vce_v2_0_set_powergating_state };
static vce_v2_0_ring_funcs: amdgpu_ring_funcs = amdgpu_ring_funcs { type:AMDGPU_RING_TYPE_VCE, align_mask:0xf, nop:VCE_CMD_NO_OP, support_64bit_ptrs:false, no_user_fence:true, get_rptr:vce_v2_0_ring_get_rptr, get_wptr:vce_v2_0_ring_get_wptr, set_wptr:vce_v2_0_ring_set_wptr, parse_cs:amdgpu_vce_ring_parse_cs, emit_frame_size:6, emit_ib_size:4, emit_ib:amdgpu_vce_ring_emit_ib, emit_fence:amdgpu_vce_ring_emit_fence, test_ring:amdgpu_vce_ring_test_ring, test_ib:amdgpu_vce_ring_test_ib, insert_nop:amdgpu_ring_insert_nop, pad_ib:amdgpu_ring_generic_pad_ib, begin_use:amdgpu_vce_ring_begin_use, end_use:amdgpu_vce_ring_end_use };
static vce_v2_0_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { set:vce_v2_0_set_interrupt_state, process:vce_v2_0_process_interrupt };
static vce_v2_0_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type:AMDGPU_IP_BLOCK_TYPE_VCE, major:2, minor:0, rev:0, funcs:&vce_v2_0_ip_funcs };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
