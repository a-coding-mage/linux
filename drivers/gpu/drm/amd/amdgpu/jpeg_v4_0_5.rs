/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

/* C headers and generated register definitions are supplied by the surrounding
 * driver translation.  The following identifiers intentionally remain external.
 */

const REG_UVD_JPEG_PITCH_INTERNAL_OFFSET: u32 = 0x401f;
const REG_JPEG_DEC_GFX10_ADDR_CONFIG_INTERNAL_OFFSET: u32 = 0x4026;
const REG_JPEG_SYS_INT_EN_INTERNAL_OFFSET: u32 = 0x4141;
const REG_JPEG_CGC_CTRL_INTERNAL_OFFSET: u32 = 0x4161;
const REG_JPEG_CGC_GATE_INTERNAL_OFFSET: u32 = 0x4160;
const REG_UVD_NO_OP_INTERNAL_OFFSET: u32 = 0x0029;

static mut AMDGPU_IH_CLIENTID_JPEG: [i32; 2] = [SOC15_IH_CLIENTID_VCN, SOC15_IH_CLIENTID_VCN1];

unsafe fn jpeg_v4_0_5_early_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    match amdgpu_ip_version(adev, UVD_HWIP, 0) {
        x if x == IP_VERSION(4, 0, 5) => (*adev).jpeg.num_jpeg_inst = 1,
        x if x == IP_VERSION(4, 0, 6) => (*adev).jpeg.num_jpeg_inst = 2,
        _ => { DRM_DEV_ERROR((*adev).dev, "Failed to init vcn ip block(UVD_HWIP:0x%x)\n", amdgpu_ip_version(adev, UVD_HWIP, 0)); return -EINVAL; }
    }
    (*adev).jpeg.num_jpeg_rings = 1;
    jpeg_v4_0_5_set_dec_ring_funcs(adev);
    jpeg_v4_0_5_set_irq_funcs(adev);
    0
}

unsafe fn jpeg_v4_0_5_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let mut r: i32;
    for i in 0..(*adev).jpeg.num_jpeg_inst {
        if (*adev).jpeg.harvest_config & (1 << i) != 0 { continue; }
        r = amdgpu_irq_add_id(adev, AMDGPU_IH_CLIENTID_JPEG[i as usize], VCN_4_0__SRCID__JPEG_DECODE, &mut (*adev).jpeg.inst[i as usize].irq); if r != 0 { return r; }
        r = amdgpu_irq_add_id(adev, AMDGPU_IH_CLIENTID_JPEG[i as usize], VCN_4_0__SRCID_DJPEG0_POISON, &mut (*adev).jpeg.inst[i as usize].irq); if r != 0 { return r; }
        r = amdgpu_irq_add_id(adev, AMDGPU_IH_CLIENTID_JPEG[i as usize], VCN_4_0__SRCID_EJPEG0_POISON, &mut (*adev).jpeg.inst[i as usize].irq); if r != 0 { return r; }
    }
    r = amdgpu_jpeg_sw_init(adev); if r != 0 { return r; }
    r = amdgpu_jpeg_resume(adev); if r != 0 { return r; }
    for i in 0..(*adev).jpeg.num_jpeg_inst {
        if (*adev).jpeg.harvest_config & (1 << i) != 0 { continue; }
        let ring = (*adev).jpeg.inst[i as usize].ring_dec;
        (*ring).use_doorbell = true;
        (*ring).vm_hub = AMDGPU_MMHUB0(0);
        (*ring).doorbell_index = ((*adev).doorbell_index.vcn.vcn_ring0_1 << 1) + 1 + 8 * i;
        sprintf((*ring).name.as_mut_ptr(), "jpeg_dec_%d", i);
        r = amdgpu_ring_init(adev, ring, 512, &mut (*adev).jpeg.inst[i as usize].irq, 0, AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut()); if r != 0 { return r; }
        (*adev).jpeg.internal.jpeg_pitch[0] = REG_UVD_JPEG_PITCH_INTERNAL_OFFSET;
        (*adev).jpeg.inst[i as usize].external.jpeg_pitch[0] = SOC15_REG_OFFSET(JPEG, i, regUVD_JPEG_PITCH);
    }
    r = amdgpu_jpeg_reg_dump_init(adev, jpeg_reg_list_4_0_5.as_ptr(), jpeg_reg_list_4_0_5.len()); if r != 0 { return r; }
    (*adev).jpeg.supported_reset = amdgpu_get_soft_full_reset_mask((*adev).jpeg.inst[0].ring_dec);
    if !amdgpu_sriov_vf(adev) { (*adev).jpeg.supported_reset |= AMDGPU_RESET_TYPE_PER_QUEUE; }
    r = amdgpu_jpeg_sysfs_reset_mask_init(adev); if r != 0 { return r; } 0
}

unsafe fn jpeg_v4_0_5_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32 { let adev=(*ip_block).adev; let mut r=amdgpu_jpeg_suspend(adev); if r!=0{return r;} amdgpu_jpeg_sysfs_reset_mask_fini(adev); r=amdgpu_jpeg_sw_fini(adev); r }
unsafe fn jpeg_v4_0_5_hw_init(ip_block:*mut amdgpu_ip_block)->i32 { let adev=(*ip_block).adev; if (*adev).pg_flags&AMD_PG_SUPPORT_JPEG_DPG!=0{return 0;} for i in 0..(*adev).jpeg.num_jpeg_inst { if (*adev).jpeg.harvest_config&(1<<i)!=0{continue;} let r=amdgpu_ring_test_helper((*adev).jpeg.inst[i as usize].ring_dec); if r!=0{return r;} } 0 }
unsafe fn jpeg_v4_0_5_hw_fini(ip_block:*mut amdgpu_ip_block)->i32 { let adev=(*ip_block).adev; cancel_delayed_work_sync(&mut (*adev).jpeg.idle_work); for i in 0..(*adev).jpeg.num_jpeg_inst { if (*adev).jpeg.harvest_config&(1<<i)!=0{continue;} if !amdgpu_sriov_vf(adev)&&(*adev).jpeg.cur_state!=AMD_PG_STATE_GATE&&RREG32_SOC15(JPEG,i,regUVD_JRBC_STATUS)!=0 { jpeg_v4_0_5_set_powergating_state(ip_block,AMD_PG_STATE_GATE); } } 0 }
unsafe fn jpeg_v4_0_5_suspend(ip_block:*mut amdgpu_ip_block)->i32 { let mut r=jpeg_v4_0_5_hw_fini(ip_block); if r!=0{return r;} r=amdgpu_jpeg_suspend((*ip_block).adev); r }
unsafe fn jpeg_v4_0_5_resume(ip_block:*mut amdgpu_ip_block)->i32 { let mut r=amdgpu_jpeg_resume((*ip_block).adev); if r!=0{return r;} r=jpeg_v4_0_5_hw_init(ip_block); r }

/* The register programming helpers below preserve the original ordering and
 * volatile register side effects. */
unsafe fn jpeg_v4_0_5_disable_clock_gating(adev:*mut amdgpu_device,inst:i32){let mut data=RREG32_SOC15(JPEG,inst,regJPEG_CGC_CTRL);if (*adev).cg_flags&AMD_CG_SUPPORT_JPEG_MGCG!=0{data|=1<<JPEG_CGC_CTRL__DYN_CLOCK_MODE__SHIFT;data&=!JPEG_CGC_CTRL__JPEG_DEC_MODE_MASK;}else{data&=!(1<<JPEG_CGC_CTRL__DYN_CLOCK_MODE__SHIFT);}data|=1<<JPEG_CGC_CTRL__CLK_GATE_DLY_TIMER__SHIFT;data|=4<<JPEG_CGC_CTRL__CLK_OFF_DELAY__SHIFT;WREG32_SOC15(JPEG,inst,regJPEG_CGC_CTRL,data);data=RREG32_SOC15(JPEG,inst,regJPEG_CGC_GATE);data&=!(JPEG_CGC_GATE__JPEG_DEC_MASK|JPEG_CGC_GATE__JPEG2_DEC_MASK|JPEG_CGC_GATE__JMCIF_MASK|JPEG_CGC_GATE__JRBBM_MASK);WREG32_SOC15(JPEG,inst,regJPEG_CGC_GATE,data);}
unsafe fn jpeg_v4_0_5_enable_clock_gating(adev:*mut amdgpu_device,inst:i32){let mut data=RREG32_SOC15(JPEG,inst,regJPEG_CGC_CTRL);if (*adev).cg_flags&AMD_CG_SUPPORT_JPEG_MGCG!=0{data|=1<<JPEG_CGC_CTRL__DYN_CLOCK_MODE__SHIFT;data|=JPEG_CGC_CTRL__JPEG_DEC_MODE_MASK;}else{data&=!(1<<JPEG_CGC_CTRL__DYN_CLOCK_MODE__SHIFT);}data|=1<<JPEG_CGC_CTRL__CLK_GATE_DLY_TIMER__SHIFT;data|=4<<JPEG_CGC_CTRL__CLK_OFF_DELAY__SHIFT;WREG32_SOC15(JPEG,inst,regJPEG_CGC_CTRL,data);data=RREG32_SOC15(JPEG,inst,regJPEG_CGC_GATE);data|=JPEG_CGC_GATE__JPEG_DEC_MASK|JPEG_CGC_GATE__JPEG2_DEC_MASK|JPEG_CGC_GATE__JMCIF_MASK|JPEG_CGC_GATE__JRBBM_MASK;WREG32_SOC15(JPEG,inst,regJPEG_CGC_GATE,data);}

/* Remaining source-level entry points retain the C driver's external helper
 * calls and data structures; generated bindings provide their exact types. */
unsafe fn jpeg_v4_0_5_set_dec_ring_funcs(adev:*mut amdgpu_device){for i in 0..(*adev).jpeg.num_jpeg_inst{if (*adev).jpeg.harvest_config&(1<<i)==0{(*adev).jpeg.inst[i as usize].ring_dec.funcs=&jpeg_v4_0_5_dec_ring_vm_funcs;(*(*adev).jpeg.inst[i as usize].ring_dec).me=i;}}}
unsafe fn jpeg_v4_0_5_set_irq_funcs(adev:*mut amdgpu_device){for i in 0..(*adev).jpeg.num_jpeg_inst{if (*adev).jpeg.harvest_config&(1<<i)==0{(*adev).jpeg.inst[i as usize].irq.num_types=1;(*adev).jpeg.inst[i as usize].irq.funcs=&jpeg_v4_0_5_irq_funcs;}}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
