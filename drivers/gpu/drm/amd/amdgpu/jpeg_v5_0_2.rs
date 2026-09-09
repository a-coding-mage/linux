// SPDX-License-Identifier: GPL-2.0 OR MIT
/* Copyright 2025-2026 Advanced Micro Devices, Inc. All rights reserved. */
/* C headers and build-time dependencies are supplied by the surrounding kernel translation. */

static mut amdgpu_ih_srcid_jpeg: [i32; 10] = [
    VCN_5_0__SRCID__JPEG_DECODE, VCN_5_0__SRCID__JPEG1_DECODE,
    VCN_5_0__SRCID__JPEG2_DECODE, VCN_5_0__SRCID__JPEG3_DECODE,
    VCN_5_0__SRCID__JPEG4_DECODE, VCN_5_0__SRCID__JPEG5_DECODE,
    VCN_5_0__SRCID__JPEG6_DECODE, VCN_5_0__SRCID__JPEG7_DECODE,
    VCN_5_0__SRCID__JPEG8_DECODE, VCN_5_0__SRCID__JPEG9_DECODE,
];

unsafe fn jpeg_v5_0_2_core_reg_offset(pipe: u32) -> i32 {
    if pipe <= AMDGPU_MAX_JPEG_RINGS_4_0_3 { (0x40 * pipe) as i32 - 0xc80 } else { (0x40 * pipe) as i32 - 0x440 }
}

unsafe fn jpeg_v5_0_2_early_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    if (*(*adev).jpeg).num_jpeg_inst == 0 || (*(*adev).jpeg).num_jpeg_inst > AMDGPU_MAX_JPEG_INSTANCES { return -ENOENT; }
    (*(*adev).jpeg).num_jpeg_rings = AMDGPU_MAX_JPEG_RINGS;
    jpeg_v5_0_2_set_dec_ring_funcs(adev); jpeg_v5_0_2_set_irq_funcs(adev); 0
}

unsafe fn jpeg_v5_0_2_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev; let mut r: i32;
    for j in 0..(*(*adev).jpeg).num_jpeg_rings { r = amdgpu_irq_add_id(adev, SOC15_IH_CLIENTID_VCN, amdgpu_ih_srcid_jpeg[j as usize], &mut (*(*adev).jpeg).inst.irq); if r != 0 { return r; } }
    r = amdgpu_jpeg_sw_init(adev); if r != 0 { return r; }
    r = amdgpu_jpeg_resume(adev); if r != 0 { return r; }
    for i in 0..(*(*adev).jpeg).num_jpeg_inst { let jpeg_inst = GET_INST(JPEG, i); for j in 0..(*(*adev).jpeg).num_jpeg_rings {
        let ring = &mut (*(*adev).jpeg).inst.add(i as usize).ring_dec[j as usize]; ring.use_doorbell = false;
        ring.vm_hub = AMDGPU_MMHUB0((*(*adev).jpeg).inst.add(i as usize).aid_id);
        ring.doorbell_index = ((*adev).doorbell_index.vcn.vcn_ring0_1 << 1) + 1 + j + 11 * jpeg_inst;
        sprintf(ring.name.as_mut_ptr(), b"jpeg_dec_%d.%d\0".as_ptr(), (*(*adev).jpeg).inst.add(i as usize).aid_id, j);
        r = amdgpu_ring_init(adev, ring, 512, &mut (*(*adev).jpeg).inst.irq, 0, AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut()); if r != 0 { return r; }
        (*(*adev).jpeg).internal.jpeg_pitch[j as usize] = regUVD_JRBC0_UVD_JRBC_SCRATCH0_INTERNAL_OFFSET;
        (*(*adev).jpeg).inst.add(i as usize).external.jpeg_pitch[j as usize] = SOC15_REG_OFFSET1(JPEG, jpeg_inst, regUVD_JRBC_SCRATCH0, if j != 0 { jpeg_v5_0_2_core_reg_offset(j as u32) } else { 0 });
    }}
    r = amdgpu_jpeg_reg_dump_init(adev, jpeg_reg_list_5_0_2, ARRAY_SIZE(jpeg_reg_list_5_0_2)); if r != 0 { return r; }
    (*(*adev).jpeg).supported_reset = amdgpu_get_soft_full_reset_mask(&(*(*adev).jpeg).inst.ring_dec[0]);
    (*(*adev).jpeg).supported_reset |= AMDGPU_RESET_TYPE_PER_QUEUE; amdgpu_jpeg_sysfs_reset_mask_init(adev)
}

unsafe fn jpeg_v5_0_2_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32 { let adev=(*ip_block).adev; let r=amdgpu_jpeg_suspend(adev); if r!=0{return r;} amdgpu_jpeg_sysfs_reset_mask_fini(adev); amdgpu_jpeg_sw_fini(adev) }
unsafe fn jpeg_v5_0_2_hw_init(ip_block:*mut amdgpu_ip_block)->i32 { let adev=(*ip_block).adev; if RREG32_SOC15(VCN,GET_INST(VCN,0),regVCN_RRMT_CNTL)&0x100!=0 {(*(*adev).jpeg).caps|=AMDGPU_JPEG_CAPS(RRMT_ENABLED);} for i in 0..(*(*adev).jpeg).num_jpeg_inst { let ji=GET_INST(JPEG,i); let r=&mut (*(*adev).jpeg).inst.add(i as usize).ring_dec[0]; let mut t=RREG32_SOC15(JPEG,ji,regUVD_JPEG_POWER_STATUS); t&=!UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK; WREG32_SOC15(JPEG,ji,regUVD_JPEG_POWER_STATUS,t); if r.use_doorbell {(*adev).nbio.funcs.vcn_doorbell_range(adev,true,((*adev).doorbell_index.vcn.vcn_ring0_1<<1)+11*ji,(*(*adev).jpeg).inst.add(i as usize).aid_id);} for j in 0..(*(*adev).jpeg).num_jpeg_rings {let ring=&mut (*(*adev).jpeg).inst.add(i as usize).ring_dec[j as usize]; if ring.use_doorbell {WREG32_SOC15_OFFSET(VCN,GET_INST(VCN,i),regVCN_JPEG_DB_CTRL,ring.pipe,ring.doorbell_index<<VCN_JPEG_DB_CTRL__OFFSET__SHIFT|VCN_JPEG_DB_CTRL__EN_MASK);} let e=amdgpu_ring_test_helper(ring); if e!=0{return e;}}} 0 }
unsafe fn jpeg_v5_0_2_hw_fini(ip_block:*mut amdgpu_ip_block)->i32 { let adev=(*ip_block).adev; cancel_delayed_work_sync(&mut (*(*adev).jpeg).idle_work); if (*(*adev).jpeg).cur_state!=AMD_PG_STATE_GATE {jpeg_v5_0_2_set_powergating_state(ip_block,AMD_PG_STATE_GATE)} else {0} }
unsafe fn jpeg_v5_0_2_suspend(ip:*mut amdgpu_ip_block)->i32 {let r=jpeg_v5_0_2_hw_fini(ip);if r!=0{return r;}amdgpu_jpeg_suspend((*ip).adev)}
unsafe fn jpeg_v5_0_2_resume(ip:*mut amdgpu_ip_block)->i32 {let r=amdgpu_jpeg_resume((*ip).adev);if r!=0{return r;}jpeg_v5_0_2_hw_init(ip)}

// The remaining register and callback tables retain the C ABI and are declared through the translated kernel bindings.
unsafe fn jpeg_v5_0_2_set_dec_ring_funcs(adev:*mut amdgpu_device) { for i in 0..(*(*adev).jpeg).num_jpeg_inst { for j in 0..(*(*adev).jpeg).num_jpeg_rings { let r=&mut (*(*adev).jpeg).inst.add(i as usize).ring_dec[j as usize]; r.funcs=&jpeg_v5_0_2_dec_ring_vm_funcs; r.me=i; r.pipe=j; } let ji=GET_INST(JPEG,i); (*(*adev).jpeg).inst.add(i as usize).aid_id=ji/(*(*adev).jpeg).num_inst_per_aid; } }
unsafe fn jpeg_v5_0_2_set_irq_funcs(adev:*mut amdgpu_device) { for _ in 0..(*(*adev).jpeg).num_jpeg_inst {(*(*adev).jpeg).inst.irq.num_types+=(*(*adev).jpeg).num_jpeg_rings;} (*(*adev).jpeg).inst.irq.funcs=&jpeg_v5_0_2_irq_funcs; }

unsafe fn jpeg_v5_0_2_init_inst(adev:*mut amdgpu_device,i:i32){let ji=GET_INST(JPEG,i);WREG32_P(SOC15_REG_OFFSET(JPEG,ji,regUVD_JPEG_POWER_STATUS),0,!UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK);WREG32_P(SOC15_REG_OFFSET(JPEG,ji,regUVD_JPEG_POWER_STATUS),0,!UVD_JPEG_POWER_STATUS__JPEG_PG_MODE_MASK);WREG32_SOC15(JPEG,0,regJPEG_DEC_GFX10_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config);WREG32_P(SOC15_REG_OFFSET(JPEG,ji,regUVD_JMI_CNTL),0,!UVD_JMI_CNTL__SOFT_RESET_MASK)}
unsafe fn jpeg_v5_0_2_deinit_inst(adev:*mut amdgpu_device,i:i32){let ji=GET_INST(JPEG,i);WREG32_P(SOC15_REG_OFFSET(JPEG,ji,regUVD_JMI_CNTL),UVD_JMI_CNTL__SOFT_RESET_MASK,!UVD_JMI_CNTL__SOFT_RESET_MASK);WREG32_P(SOC15_REG_OFFSET(JPEG,ji,regUVD_JPEG_POWER_STATUS),UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK,!UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK)}
unsafe fn jpeg_v5_0_2_start(adev:*mut amdgpu_device)->i32{for i in 0..(*(*adev).jpeg).num_jpeg_inst{jpeg_v5_0_2_init_inst(adev,i);}0}
unsafe fn jpeg_v5_0_2_stop(adev:*mut amdgpu_device)->i32{for i in 0..(*(*adev).jpeg).num_jpeg_inst{jpeg_v5_0_2_deinit_inst(adev,i);}0}
unsafe fn jpeg_v5_0_2_is_idle(ip:*mut amdgpu_ip_block)->bool{let adev=(*ip).adev;let mut ret=false;for i in 0..(*(*adev).jpeg).num_jpeg_inst{for j in 0..(*(*adev).jpeg).num_jpeg_rings{ret&=(RREG32_SOC15_OFFSET(JPEG,GET_INST(JPEG,i),regUVD_JRBC_STATUS,if j!=0{jpeg_v5_0_2_core_reg_offset(j as u32)}else{0})&UVD_JRBC_STATUS__RB_JOB_DONE_MASK)==UVD_JRBC_STATUS__RB_JOB_DONE_MASK;}}ret}
unsafe fn jpeg_v5_0_2_set_powergating_state(ip:*mut amdgpu_ip_block,state:amd_powergating_state)->i32{let adev=(*ip).adev;if state==(*(*adev).jpeg).cur_state{return 0}let r=if state==AMD_PG_STATE_GATE{jpeg_v5_0_2_stop(adev)}else{jpeg_v5_0_2_start(adev)};if r==0{(*(*adev).jpeg).cur_state=state;}r}
unsafe fn jpeg_v5_0_2_set_clockgating_state(ip:*mut amdgpu_ip_block,state:amd_clockgating_state)->i32{if state!=AMD_CG_STATE_GATE{return 0}if !jpeg_v5_0_2_is_idle(ip){-EBUSY}else{0}}
unsafe fn jpeg_v5_0_2_set_interrupt_state(_: *mut amdgpu_device,_:*mut amdgpu_irq_src,_:u32,_:amdgpu_interrupt_state)->i32{0}

// External declarations and macro-generated register tables/functions are intentionally referenced from the surrounding translation unit.
extern "C" { static jpeg_reg_list_5_0_2: [amdgpu_hwip_reg_entry; 38]; static jpeg_v5_0_2_dec_ring_vm_funcs: amdgpu_ring_funcs; static jpeg_v5_0_2_irq_funcs: amdgpu_irq_src_funcs; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
