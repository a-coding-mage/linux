/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C dependencies: linux/pci.h, amdgpu.h, amdgpu_ih.h, OSSSYS register headers,
// soc15_common.h, and navi10_ih.h.

const MAX_REARM_RETRY: u32 = 10;
const mmIH_CHICKEN_Sienna_Cichlid: u32 = 0x018d;
const mmIH_CHICKEN_Sienna_Cichlid_BASE_IDX: u32 = 0;

unsafe fn navi10_ih_init_register_offset(adev: *mut amdgpu_device) {
    if (*adev).irq.ih.ring_size != 0 { let r = &mut (*adev).irq.ih.ih_regs;
        r.ih_rb_base = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_BASE); r.ih_rb_base_hi = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_BASE_HI); r.ih_rb_cntl = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_CNTL); r.ih_rb_wptr = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_WPTR); r.ih_rb_rptr = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_RPTR); r.ih_doorbell_rptr = SOC15_REG_OFFSET(OSSSYS,0,mmIH_DOORBELL_RPTR); r.ih_rb_wptr_addr_lo = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_WPTR_ADDR_LO); r.ih_rb_wptr_addr_hi = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_WPTR_ADDR_HI); r.psp_reg_id = PSP_REG_IH_RB_CNTL;
    }
    if (*adev).irq.ih1.ring_size != 0 { let r = &mut (*adev).irq.ih1.ih_regs;
        r.ih_rb_base = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_BASE_RING1); r.ih_rb_base_hi = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_BASE_HI_RING1); r.ih_rb_cntl = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_CNTL_RING1); r.ih_rb_wptr = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_WPTR_RING1); r.ih_rb_rptr = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_RPTR_RING1); r.ih_doorbell_rptr = SOC15_REG_OFFSET(OSSSYS,0,mmIH_DOORBELL_RPTR_RING1); r.psp_reg_id = PSP_REG_IH_RB_CNTL_RING1;
    }
    if (*adev).irq.ih2.ring_size != 0 { let r = &mut (*adev).irq.ih2.ih_regs;
        r.ih_rb_base = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_BASE_RING2); r.ih_rb_base_hi = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_BASE_HI_RING2); r.ih_rb_cntl = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_CNTL_RING2); r.ih_rb_wptr = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_WPTR_RING2); r.ih_rb_rptr = SOC15_REG_OFFSET(OSSSYS,0,mmIH_RB_RPTR_RING2); r.ih_doorbell_rptr = SOC15_REG_OFFSET(OSSSYS,0,mmIH_DOORBELL_RPTR_RING2); r.psp_reg_id = PSP_REG_IH_RB_CNTL_RING2;
    }
}

unsafe fn force_update_wptr_for_self_int(adev: *mut amdgpu_device, threshold: u32, timeout: u32, enabled: bool) {
    if amdgpu_ip_version(adev, OSSSYS_HWIP, 0) < IP_VERSION(5,0,3) { return; }
    let mut ih_cntl = RREG32_SOC15(OSSSYS,0,mmIH_CNTL2); let mut rb = RREG32_SOC15(OSSSYS,0,mmIH_RB_CNTL_RING1);
    ih_cntl = REG_SET_FIELD(ih_cntl, IH_CNTL2, SELF_IV_FORCE_WPTR_UPDATE_TIMEOUT, timeout); ih_cntl = REG_SET_FIELD(ih_cntl, IH_CNTL2, SELF_IV_FORCE_WPTR_UPDATE_ENABLE, enabled); rb = REG_SET_FIELD(rb, IH_RB_CNTL_RING1, RB_USED_INT_THRESHOLD, threshold);
    if amdgpu_sriov_vf(adev) && amdgpu_sriov_reg_indirect_ih(adev) { if psp_reg_program(&mut (*adev).psp, PSP_REG_IH_RB_CNTL_RING1, rb) != 0 { return; } } else { WREG32_SOC15(OSSSYS,0,mmIH_RB_CNTL_RING1,rb); }
    rb = RREG32_SOC15(OSSSYS,0,mmIH_RB_CNTL_RING2); rb = REG_SET_FIELD(rb, IH_RB_CNTL_RING2, RB_USED_INT_THRESHOLD, threshold);
    if amdgpu_sriov_vf(adev) && amdgpu_sriov_reg_indirect_ih(adev) { if psp_reg_program(&mut (*adev).psp, PSP_REG_IH_RB_CNTL_RING2, rb) != 0 { return; } } else { WREG32_SOC15(OSSSYS,0,mmIH_RB_CNTL_RING2,rb); }
    WREG32_SOC15(OSSSYS,0,mmIH_CNTL2,ih_cntl);
}

unsafe fn navi10_ih_toggle_ring_interrupts(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring, enable: bool) -> i32 {
    let r = &(*ih).ih_regs; let mut tmp = RREG32(r.ih_rb_cntl); tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, RB_ENABLE, if enable {1} else {0}); tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, RB_GPU_TS_ENABLE, 1);
    if ih == &mut (*adev).irq.ih { tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, ENABLE_INTR, if enable {1} else {0}); }
    if amdgpu_sriov_vf(adev) && amdgpu_sriov_reg_indirect_ih(adev) { if psp_reg_program(&mut (*adev).psp,r.psp_reg_id,tmp) != 0 { return -ETIMEDOUT; } } else { WREG32(r.ih_rb_cntl,tmp); }
    if enable { (*ih).enabled=true; } else { WREG32(r.ih_rb_rptr,0); WREG32(r.ih_rb_wptr,0); (*ih).enabled=false; (*ih).rptr=0; } 0
}

unsafe fn navi10_ih_toggle_interrupts(adev: *mut amdgpu_device, enable: bool) -> i32 { let mut a = [&mut (*adev).irq.ih, &mut (*adev).irq.ih1, &mut (*adev).irq.ih2]; for ih in a.iter_mut() { if ih.ring_size != 0 { let r=navi10_ih_toggle_ring_interrupts(adev,*ih,enable); if r != 0{return r;} } } 0 }

unsafe fn navi10_ih_rb_cntl(ih: *mut amdgpu_ih_ring, mut v: u32) -> u32 { let sz=order_base_2((*ih).ring_size/4); v=REG_SET_FIELD(v,IH_RB_CNTL,MC_SPACE,if (*ih).use_bus_addr{1}else{4}); v=REG_SET_FIELD(v,IH_RB_CNTL,WPTR_OVERFLOW_CLEAR,1); v=REG_SET_FIELD(v,IH_RB_CNTL,WPTR_OVERFLOW_ENABLE,1); v=REG_SET_FIELD(v,IH_RB_CNTL,RB_SIZE,sz); v=REG_SET_FIELD(v,IH_RB_CNTL,WPTR_WRITEBACK_ENABLE,1); v=REG_SET_FIELD(v,IH_RB_CNTL,MC_SNOOP,1); v=REG_SET_FIELD(v,IH_RB_CNTL,MC_RO,0); REG_SET_FIELD(v,IH_RB_CNTL,MC_VMID,0) }

unsafe fn navi10_ih_doorbell_rptr(ih: *mut amdgpu_ih_ring) -> u32 { let mut v=0; if (*ih).use_doorbell { v=REG_SET_FIELD(v,IH_DOORBELL_RPTR,OFFSET,(*ih).doorbell_index); v=REG_SET_FIELD(v,IH_DOORBELL_RPTR,ENABLE,1); } else { v=REG_SET_FIELD(v,IH_DOORBELL_RPTR,ENABLE,0); } v }

unsafe fn navi10_ih_enable_ring(adev:*mut amdgpu_device,ih:*mut amdgpu_ih_ring)->i32 { let r=&(*ih).ih_regs; WREG32(r.ih_rb_base,(*ih).gpu_addr>>8); WREG32(r.ih_rb_base_hi,((*ih).gpu_addr>>40)&0xff); let mut v=navi10_ih_rb_cntl(ih,RREG32(r.ih_rb_cntl)); if ih==&mut (*adev).irq.ih {v=REG_SET_FIELD(v,IH_RB_CNTL,RPTR_REARM,if (*adev).irq.msi_enabled{1}else{0});} if ih==&mut (*adev).irq.ih1 {v=REG_SET_FIELD(v,IH_RB_CNTL,RB_FULL_DRAIN_ENABLE,1);} if amdgpu_sriov_vf(adev)&&amdgpu_sriov_reg_indirect_ih(adev){if psp_reg_program(&mut (*adev).psp,r.psp_reg_id,v)!=0{return -ETIMEDOUT;}}else{WREG32(r.ih_rb_cntl,v);} if ih==&mut (*adev).irq.ih {WREG32(r.ih_rb_wptr_addr_lo,lower_32_bits((*ih).wptr_addr));WREG32(r.ih_rb_wptr_addr_hi,upper_32_bits((*ih).wptr_addr)&0xFFFF);} WREG32(r.ih_rb_wptr,0);WREG32(r.ih_rb_rptr,0);WREG32(r.ih_doorbell_rptr,navi10_ih_doorbell_rptr(ih));0 }

// The remaining callbacks retain the source interfaces and delegate to the external AMDGPU helpers.
unsafe fn navi10_ih_irq_init(adev:*mut amdgpu_device)->i32 { let mut ret=navi10_ih_toggle_interrupts(adev,false); if ret!=0{return ret;} (*adev).nbio.funcs.ih_control(adev); let mut a=[&mut (*adev).irq.ih,&mut (*adev).irq.ih1,&mut (*adev).irq.ih2]; for ih in a.iter_mut(){if ih.ring_size!=0{ret=navi10_ih_enable_ring(adev,*ih);if ret!=0{return ret;}}} (*adev).nbio.funcs.ih_doorbell_range(adev,(*a[0]).use_doorbell,(*a[0]).doorbell_index); pci_set_master((*adev).pdev); ret=navi10_ih_toggle_interrupts(adev,true);if ret!=0{return ret;}force_update_wptr_for_self_int(adev,0,8,true);if (*adev).irq.ih_soft.ring_size!=0{(*adev).irq.ih_soft.enabled=true;}0 }
unsafe fn navi10_ih_irq_disable(adev:*mut amdgpu_device){force_update_wptr_for_self_int(adev,0,8,false);navi10_ih_toggle_interrupts(adev,false);mdelay(1);}
unsafe fn navi10_ih_self_irq(adev:*mut amdgpu_device,_source:*mut amdgpu_irq_src,entry:*mut amdgpu_iv_entry)->i32{match (*entry).ring_id{1=>schedule_work(&mut (*adev).irq.ih1_work),2=>schedule_work(&mut (*adev).irq.ih2_work),_=>{}}0}
unsafe fn navi10_ih_set_rptr(adev:*mut amdgpu_device,ih:*mut amdgpu_ih_ring){if ih==&mut (*adev).irq.ih_soft{return;}if (*ih).use_doorbell{*(*ih).rptr_cpu=(*ih).rptr;WDOORBELL32((*ih).doorbell_index,(*ih).rptr);if amdgpu_sriov_vf(adev){navi10_ih_irq_rearm(adev,ih);}}else{WREG32((*ih).ih_regs.ih_rb_rptr,(*ih).rptr);}}
unsafe fn navi10_ih_irq_rearm(_adev:*mut amdgpu_device,ih:*mut amdgpu_ih_ring){let mut i=0;while i<MAX_REARM_RETRY{let v=RREG32_NO_KIQ((*ih).ih_regs.ih_rb_rptr);if v<(*ih).ring_size&&v!=(*ih).rptr{WDOORBELL32((*ih).doorbell_index,(*ih).rptr);}else{break;}i+=1;}}

// Remaining lifecycle and clock-gating callbacks are direct source-level stubs/forwarders.
unsafe fn navi10_ih_set_interrupt_funcs(adev:*mut amdgpu_device){if (*adev).irq.ih_funcs.is_null(){(*adev).irq.ih_funcs=&navi10_ih_funcs;}}
const navi10_ih_funcs: amdgpu_ih_funcs = amdgpu_ih_funcs { get_wptr:navi10_ih_get_wptr, decode_iv:amdgpu_ih_decode_iv_helper, decode_iv_ts:amdgpu_ih_decode_iv_ts_helper, set_rptr:navi10_ih_set_rptr };

unsafe fn navi10_ih_get_wptr(adev:*mut amdgpu_device,ih:*mut amdgpu_ih_ring)->u32 { let mut w; if ih==&mut (*adev).irq.ih || ih==&mut (*adev).irq.ih_soft { w=le32_to_cpu(*(*ih).wptr_cpu); if !REG_GET_FIELD(w,IH_RB_WPTR,RB_OVERFLOW){return w&(*ih).ptr_mask;} } let r=(*ih).ih_regs; w=RREG32_NO_KIQ(r.ih_rb_wptr);if !REG_GET_FIELD(w,IH_RB_WPTR,RB_OVERFLOW){return w&(*ih).ptr_mask;}w=REG_SET_FIELD(w,IH_RB_WPTR,RB_OVERFLOW,0);let tmp=(w+32)&(*ih).ptr_mask;dev_warn((*adev).dev,"%s ring buffer overflow (0x%08X, 0x%08X, 0x%08X)\n",amdgpu_ih_ring_name(adev,ih),w,(*ih).rptr,tmp);(*ih).rptr=tmp;let mut c=RREG32_NO_KIQ(r.ih_rb_cntl);c=REG_SET_FIELD(c,IH_RB_CNTL,WPTR_OVERFLOW_CLEAR,1);WREG32_NO_KIQ(r.ih_rb_cntl,c);c=REG_SET_FIELD(c,IH_RB_CNTL,WPTR_OVERFLOW_CLEAR,0);WREG32_NO_KIQ(r.ih_rb_cntl,c);w&(*ih).ptr_mask }

unsafe fn navi10_ih_early_init(ip:*mut amdgpu_ip_block)->i32{navi10_ih_set_interrupt_funcs((*ip).adev);navi10_ih_set_self_irq_funcs((*ip).adev);0}
unsafe fn navi10_ih_set_self_irq_funcs(adev:*mut amdgpu_device){(*adev).irq.self_irq.num_types=0;(*adev).irq.self_irq.funcs=&navi10_ih_self_irq_funcs;}
const navi10_ih_self_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs{process:navi10_ih_self_irq};
unsafe fn navi10_ih_sw_init(ip:*mut amdgpu_ip_block)->i32{let adev=(*ip).adev;let mut r=amdgpu_irq_add_id(adev,SOC15_IH_CLIENTID_IH,0,&mut (*adev).irq.self_irq);if r!=0{return r;}let bus= !((*adev).flags&AMD_IS_APU!=0||(*adev).firmware.load_type==AMDGPU_FW_LOAD_PSP);r=amdgpu_ih_ring_init(adev,&mut (*adev).irq.ih,IH_RING_SIZE,bus);if r!=0{return r;}(*adev).irq.ih.use_doorbell=true;(*adev).irq.ih.doorbell_index=(*adev).doorbell_index.ih<<1;(*adev).irq.ih1.ring_size=0;(*adev).irq.ih2.ring_size=0;navi10_ih_init_register_offset(adev);r=amdgpu_ih_ring_init(adev,&mut (*adev).irq.ih_soft,IH_SW_RING_SIZE,true);if r!=0{return r;}amdgpu_irq_init(adev)}
unsafe fn navi10_ih_sw_fini(ip:*mut amdgpu_ip_block)->i32{amdgpu_irq_fini_sw((*ip).adev);0}
unsafe fn navi10_ih_hw_init(ip:*mut amdgpu_ip_block)->i32{navi10_ih_irq_init((*ip).adev)}
unsafe fn navi10_ih_hw_fini(ip:*mut amdgpu_ip_block)->i32{navi10_ih_irq_disable((*ip).adev);0}
unsafe fn navi10_ih_suspend(ip:*mut amdgpu_ip_block)->i32{navi10_ih_hw_fini(ip)}
unsafe fn navi10_ih_resume(ip:*mut amdgpu_ip_block)->i32{navi10_ih_hw_init(ip)}
unsafe fn navi10_ih_is_idle(_ip:*mut amdgpu_ip_block)->bool{true}
unsafe fn navi10_ih_wait_for_idle(_ip:*mut amdgpu_ip_block)->i32{-ETIMEDOUT}
unsafe fn navi10_ih_soft_reset(_ip:*mut amdgpu_ip_block)->i32{0}
unsafe fn navi10_ih_update_clockgating_state(adev:*mut amdgpu_device,enable:bool){if (*adev).cg_flags&AMD_CG_SUPPORT_IH_CG!=0{let def=RREG32_SOC15(OSSSYS,0,mmIH_CLK_CTRL);let f=if enable{0}else{1};let mut d=def;d=REG_SET_FIELD(d,IH_CLK_CTRL,DBUS_MUX_CLK_SOFT_OVERRIDE,f);d=REG_SET_FIELD(d,IH_CLK_CTRL,OSSSYS_SHARE_CLK_SOFT_OVERRIDE,f);d=REG_SET_FIELD(d,IH_CLK_CTRL,LIMIT_SMN_CLK_SOFT_OVERRIDE,f);d=REG_SET_FIELD(d,IH_CLK_CTRL,DYN_CLK_SOFT_OVERRIDE,f);d=REG_SET_FIELD(d,IH_CLK_CTRL,REG_CLK_SOFT_OVERRIDE,f);if def!=d{WREG32_SOC15(OSSSYS,0,mmIH_CLK_CTRL,d);}}}
unsafe fn navi10_ih_set_clockgating_state(ip:*mut amdgpu_ip_block,state:amd_clockgating_state)->i32{navi10_ih_update_clockgating_state((*ip).adev,state==AMD_CG_STATE_GATE);0}
unsafe fn navi10_ih_set_powergating_state(_ip:*mut amdgpu_ip_block,_state:amd_powergating_state)->i32{0}
unsafe fn navi10_ih_get_clockgating_state(ip:*mut amdgpu_ip_block,flags:*mut u64){if RREG32_SOC15(OSSSYS,0,mmIH_CLK_CTRL)==0{*flags|=AMD_CG_SUPPORT_IH_CG;}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
