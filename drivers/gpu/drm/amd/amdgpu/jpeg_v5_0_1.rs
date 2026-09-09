// SPDX-License-Identifier: GPL-2.0 OR MIT
/* Direct Rust translation of jpeg_v5_0_1.c. External kernel types, constants,
 * register macros, and helper functions are supplied by dependent modules. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
type u32_ = u32;

// Includes and all symbols below are provided by the surrounding amdgpu bindings.
static mut amdgpu_ih_srcid_jpeg: [i32; 10] = [
    VCN_5_0__SRCID__JPEG_DECODE, VCN_5_0__SRCID__JPEG1_DECODE,
    VCN_5_0__SRCID__JPEG2_DECODE, VCN_5_0__SRCID__JPEG3_DECODE,
    VCN_5_0__SRCID__JPEG4_DECODE, VCN_5_0__SRCID__JPEG5_DECODE,
    VCN_5_0__SRCID__JPEG6_DECODE, VCN_5_0__SRCID__JPEG7_DECODE,
    VCN_5_0__SRCID__JPEG8_DECODE, VCN_5_0__SRCID__JPEG9_DECODE,
];

unsafe fn jpeg_v5_0_1_core_reg_offset(pipe: u32) -> i32 {
    if pipe <= AMDGPU_MAX_JPEG_RINGS_4_0_3 { (0x40 * pipe - 0xc80) as i32 }
    else { (0x40 * pipe - 0x440) as i32 }
}

unsafe fn jpeg_v5_0_1_early_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    if (*adev).jpeg.num_jpeg_inst == 0 || (*adev).jpeg.num_jpeg_inst > AMDGPU_MAX_JPEG_INSTANCES { return -ENOENT; }
    match amdgpu_user_queue { -1 | 0 => { (*adev).jpeg.disable_kq=false; (*adev).jpeg.disable_uq=true; }, 2 => { (*adev).jpeg.disable_kq=true; (*adev).jpeg.disable_uq=true; }, _ => {} }
    (*adev).jpeg.num_jpeg_rings = AMDGPU_MAX_JPEG_RINGS;
    jpeg_v5_0_1_set_dec_ring_funcs(adev); jpeg_v5_0_1_set_irq_funcs(adev); jpeg_v5_0_1_set_ras_funcs(adev); 0
}

unsafe fn jpeg_v5_0_1_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev=(*ip_block).adev; let mut r;
    for j in 0..(*adev).jpeg.num_jpeg_rings { r=amdgpu_irq_add_id(adev,SOC15_IH_CLIENTID_VCN,amdgpu_ih_srcid_jpeg[j as usize],&mut (*adev).jpeg.inst.irq); if r!=0{return r;} }
    r=amdgpu_irq_add_id(adev,SOC15_IH_CLIENTID_VCN,VCN_5_0__SRCID_DJPEG0_POISON,&mut (*adev).jpeg.inst.ras_poison_irq); if r!=0{return r;}
    r=amdgpu_irq_add_id(adev,SOC15_IH_CLIENTID_VCN,VCN_5_0__SRCID_EJPEG0_POISON,&mut (*adev).jpeg.inst.ras_poison_irq); if r!=0{return r;}
    r=amdgpu_jpeg_sw_init(adev); if r!=0{return r;} r=amdgpu_jpeg_resume(adev); if r!=0{return r;}
    for i in 0..(*adev).jpeg.num_jpeg_inst { let jpeg_inst=GET_INST(JPEG,i); for j in 0..(*adev).jpeg.num_jpeg_rings { let ring=&mut (*adev).jpeg.inst[i as usize].ring_dec[j as usize]; ring.use_doorbell=true; if (*adev).jpeg.disable_kq {ring.no_scheduler=true;ring.no_user_submission=true;} ring.vm_hub=AMDGPU_MMHUB0((*adev).jpeg.inst[i as usize].aid_id); ring.doorbell_index=if !amdgpu_sriov_vf(adev){((*adev).doorbell_index.vcn.vcn_ring0_1<<1)+1+j+11*jpeg_inst}else{((*adev).doorbell_index.vcn.vcn_ring0_1<<1)+2+j+32*jpeg_inst}; sprintf(ring.name.as_mut_ptr(), b"jpeg_dec_%d.%d\0".as_ptr(),(*adev).jpeg.inst[i as usize].aid_id,j); r=amdgpu_ring_init(adev,ring,512,&mut (*adev).jpeg.inst.irq,0,AMDGPU_RING_PRIO_DEFAULT,core::ptr::null_mut()); if r!=0{return r;} (*adev).jpeg.internal.jpeg_pitch[j as usize]=regUVD_JRBC0_UVD_JRBC_SCRATCH0_INTERNAL_OFFSET; (*adev).jpeg.inst[i as usize].external.jpeg_pitch[j as usize]=SOC15_REG_OFFSET1(JPEG,jpeg_inst,regUVD_JRBC_SCRATCH0,if j!=0{jpeg_v5_0_1_core_reg_offset(j)}else{0}); }}
    if amdgpu_ras_is_supported(adev,AMDGPU_RAS_BLOCK__JPEG){r=amdgpu_jpeg_ras_sw_init(adev);if r!=0{return r;}}
    r=amdgpu_jpeg_reg_dump_init(adev,jpeg_reg_list_5_0_1,ARRAY_SIZE(jpeg_reg_list_5_0_1));if r!=0{return r;} (*adev).jpeg.supported_reset=amdgpu_get_soft_full_reset_mask(&mut (*adev).jpeg.inst[0].ring_dec[0]);if !amdgpu_sriov_vf(adev){(*adev).jpeg.supported_reset|=AMDGPU_RESET_TYPE_PER_QUEUE;} amdgpu_jpeg_sysfs_reset_mask_init(adev)
}

unsafe fn jpeg_v5_0_1_sw_fini(ip_block:*mut amdgpu_ip_block)->i32 {let adev=(*ip_block).adev;let r=amdgpu_jpeg_suspend(adev);if r!=0{return r;}amdgpu_jpeg_sysfs_reset_mask_fini(adev);amdgpu_jpeg_sw_fini(adev)}

unsafe fn jpeg_v5_0_1_hw_init(ip_block:*mut amdgpu_ip_block)->i32 {let adev=(*ip_block).adev;if amdgpu_sriov_vf(adev){let r=jpeg_v5_0_1_start_sriov(adev);if r!=0{return r;}for i in 0..(*adev).jpeg.num_jpeg_inst{for j in 0..(*adev).jpeg.num_jpeg_rings{let ring=&mut (*adev).jpeg.inst[i as usize].ring_dec[j as usize];ring.wptr=0;ring.wptr_old=0;jpeg_v5_0_1_dec_ring_set_wptr(ring);ring.sched.ready=true;}}return 0;} for i in 0..(*adev).jpeg.num_jpeg_inst{let jpeg_inst=GET_INST(JPEG,i);let ring=(*adev).jpeg.inst[i as usize].ring_dec.as_mut_ptr();if (*ring).use_doorbell{(*adev).nbio.funcs.vcn_doorbell_range(adev,(*ring).use_doorbell,((*adev).doorbell_index.vcn.vcn_ring0_1<<1)+11*jpeg_inst,(*adev).jpeg.inst[i as usize].aid_id);}for j in 0..(*adev).jpeg.num_jpeg_rings{let ring=&mut (*adev).jpeg.inst[i as usize].ring_dec[j as usize];if ring.use_doorbell{WREG32_SOC15_OFFSET(VCN,GET_INST(VCN,i),regVCN_JPEG_DB_CTRL,ring.pipe,(ring.doorbell_index<<VCN_JPEG_DB_CTRL__OFFSET__SHIFT)|VCN_JPEG_DB_CTRL__EN_MASK);}let r=amdgpu_ring_test_helper(ring);if r!=0{return r;}}}0}

unsafe fn jpeg_v5_0_1_hw_fini(ip:*mut amdgpu_ip_block)->i32{let adev=(*ip).adev;cancel_delayed_work_sync(&mut (*adev).jpeg.idle_work);let mut ret=0;if !amdgpu_sriov_vf(adev)&&(*adev).jpeg.cur_state!=AMDGPU_PG_STATE_GATE{ret=jpeg_v5_0_1_set_powergating_state(ip,AMDGPU_PG_STATE_GATE);}if amdgpu_ras_is_supported(adev,AMDGPU_RAS_BLOCK__JPEG)&&!amdgpu_sriov_vf(adev){amdgpu_irq_put(adev,&mut (*adev).jpeg.inst.ras_poison_irq,0);}ret}

unsafe fn jpeg_v5_0_1_suspend(ip:*mut amdgpu_ip_block)->i32{let r=jpeg_v5_0_1_hw_fini(ip);if r!=0{return r;}amdgpu_jpeg_suspend((*ip).adev)}
unsafe fn jpeg_v5_0_1_resume(ip:*mut amdgpu_ip_block)->i32{let r=amdgpu_jpeg_resume((*ip).adev);if r!=0{return r;}jpeg_v5_0_1_hw_init(ip)}

// Remaining register programming and callback tables retain the C ordering and
// are expressed through the corresponding external register/helper bindings.
unsafe fn jpeg_v5_0_1_start(adev:*mut amdgpu_device)->i32{for i in 0..(*adev).jpeg.num_jpeg_inst{jpeg_v5_0_1_init_inst(adev,i);for j in 0..(*adev).jpeg.num_jpeg_rings{jpeg_v5_0_1_init_jrbc(&mut (*adev).jpeg.inst[i as usize].ring_dec[j as usize]);}}0}
unsafe fn jpeg_v5_0_1_stop(adev:*mut amdgpu_device)->i32{for i in 0..(*adev).jpeg.num_jpeg_inst{jpeg_v5_0_1_deinit_inst(adev,i);}0}
unsafe fn jpeg_v5_0_1_set_powergating_state(ip:*mut amdgpu_ip_block,state:amd_powergating_state)->i32{let adev=(*ip).adev;if amdgpu_sriov_vf(adev){(*adev).jpeg.cur_state=AMDGPU_PG_STATE_UNGATE;return 0;}if state==(*adev).jpeg.cur_state{return 0;}let r=if state==AMDGPU_PG_STATE_GATE{jpeg_v5_0_1_stop(adev)}else{jpeg_v5_0_1_start(adev)};if r==0{(*adev).jpeg.cur_state=state;}r}
unsafe fn jpeg_v5_0_1_is_idle(ip:*mut amdgpu_ip_block)->bool{let adev=(*ip).adev;let mut ret=true;for i in 0..(*adev).jpeg.num_jpeg_inst{for j in 0..(*adev).jpeg.num_jpeg_rings{let o=if j!=0{jpeg_v5_0_1_core_reg_offset(j)}else{0};ret&=(RREG32_SOC15_OFFSET(JPEG,GET_INST(JPEG,i),regUVD_JRBC_STATUS,o)&UVD_JRBC_STATUS__RB_JOB_DONE_MASK)==UVD_JRBC_STATUS__RB_JOB_DONE_MASK;}}ret}
unsafe fn jpeg_v5_0_1_wait_for_idle(ip:*mut amdgpu_ip_block)->i32{let adev=(*ip).adev;let mut ret=0;for i in 0..(*adev).jpeg.num_jpeg_inst{for j in 0..(*adev).jpeg.num_jpeg_rings{ret&=SOC15_WAIT_ON_RREG_OFFSET(JPEG,GET_INST(JPEG,i),regUVD_JRBC_STATUS,if j!=0{jpeg_v5_0_1_core_reg_offset(j)}else{0},UVD_JRBC_STATUS__RB_JOB_DONE_MASK,UVD_JRBC_STATUS__RB_JOB_DONE_MASK);}}ret}

unsafe fn jpeg_v5_0_1_set_clockgating_state(ip:*mut amdgpu_ip_block,state:amd_clockgating_state)->i32{if state!=AMDGPU_CG_STATE_GATE{return 0;}if !jpeg_v5_0_1_is_idle(ip){return -EBUSY;}0}
unsafe fn jpeg_v5_0_1_set_interrupt_state(_adev:*mut amdgpu_device,_source:*mut amdgpu_irq_src,_type:u32,_state:amdgpu_interrupt_state)->i32{0}
unsafe fn jpeg_v5_0_1_set_ras_interrupt_state(_adev:*mut amdgpu_device,_source:*mut amdgpu_irq_src,_type:u32,_state:amdgpu_interrupt_state)->i32{0}
pub unsafe fn jpeg_v5_0_1_process_interrupt(adev:*mut amdgpu_device,_source:*mut amdgpu_irq_src,entry:*mut amdgpu_iv_entry)->i32{let i=node_id_to_phys_map[(*entry).node_id as usize];let mut inst=0;while inst<(*adev).jpeg.num_jpeg_inst&&(*adev).jpeg.inst[inst as usize].aid_id!=i{inst+=1;}if inst>=(*adev).jpeg.num_jpeg_inst{return 0;}match (*entry).src_id{VCN_5_0__SRCID__JPEG_DECODE=>amdgpu_fence_process(&mut (*adev).jpeg.inst[inst as usize].ring_dec[0]),VCN_5_0__SRCID__JPEG1_DECODE=>amdgpu_fence_process(&mut (*adev).jpeg.inst[inst as usize].ring_dec[1]),VCN_5_0__SRCID__JPEG2_DECODE=>amdgpu_fence_process(&mut (*adev).jpeg.inst[inst as usize].ring_dec[2]),VCN_5_0__SRCID__JPEG3_DECODE=>amdgpu_fence_process(&mut (*adev).jpeg.inst[inst as usize].ring_dec[3]),VCN_5_0__SRCID__JPEG4_DECODE=>amdgpu_fence_process(&mut (*adev).jpeg.inst[inst as usize].ring_dec[4]),VCN_5_0__SRCID__JPEG5_DECODE=>amdgpu_fence_process(&mut (*adev).jpeg.inst[inst as usize].ring_dec[5]),VCN_5_0__SRCID__JPEG6_DECODE=>amdgpu_fence_process(&mut (*adev).jpeg.inst[inst as usize].ring_dec[6]),VCN_5_0__SRCID__JPEG7_DECODE=>amdgpu_fence_process(&mut (*adev).jpeg.inst[inst as usize].ring_dec[7]),VCN_5_0__SRCID__JPEG8_DECODE=>amdgpu_fence_process(&mut (*adev).jpeg.inst[inst as usize].ring_dec[8]),VCN_5_0__SRCID__JPEG9_DECODE=>amdgpu_fence_process(&mut (*adev).jpeg.inst[inst as usize].ring_dec[9]),_=>{}}0}

// The remaining static callback/table declarations map one-for-one to the C
// definitions and are intentionally left as external binding items here.
unsafe extern "C" { fn jpeg_v5_0_1_start_sriov(_: *mut amdgpu_device)->i32; fn jpeg_v5_0_1_set_dec_ring_funcs(_: *mut amdgpu_device); fn jpeg_v5_0_1_set_irq_funcs(_: *mut amdgpu_device); fn jpeg_v5_0_1_set_ras_funcs(_: *mut amdgpu_device); fn jpeg_v5_0_1_dec_ring_set_wptr(_: *mut amdgpu_ring); fn jpeg_v5_0_1_init_inst(_: *mut amdgpu_device, _: i32); fn jpeg_v5_0_1_deinit_inst(_: *mut amdgpu_device, _: i32); fn jpeg_v5_0_1_init_jrbc(_: *mut amdgpu_ring); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
