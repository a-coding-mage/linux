/* Translated from jpeg_v2_5.c. External types, constants, macros, and
 * functions are supplied by the surrounding AMDGPU Rust bindings. */

const MMUVD_JPEG_PITCH_INTERNAL_OFFSET: u32 = 0x401f;
const JPEG25_MAX_HW_INSTANCES_ARCTURUS: i32 = 2;

/* The following tables retain the source-level linkage and are completed by
 * the surrounding generated AMDGPU type definitions. */
extern "C" {
    static jpeg_reg_list_2_5: [amdgpu_hwip_reg_entry; 13];
    static jpeg_v2_5_dec_ring_vm_funcs: amdgpu_ring_funcs;
    static jpeg_v2_6_dec_ring_vm_funcs: amdgpu_ring_funcs;
    static jpeg_v2_5_irq_funcs: amdgpu_irq_src_funcs;
    static jpeg_v2_6_ras_irq_funcs: amdgpu_irq_src_funcs;
    static mut jpeg_v2_6_ras: amdgpu_jpeg_ras;
}

static mut AMDGPU_IH_CLIENTID_JPEG: [i32; 2] = [SOC15_IH_CLIENTID_VCN, SOC15_IH_CLIENTID_VCN1];

unsafe fn jpeg_v2_5_early_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    (*adev).jpeg.num_jpeg_rings = 1;
    (*adev).jpeg.num_jpeg_inst = JPEG25_MAX_HW_INSTANCES_ARCTURUS;
    for i in 0..(*adev).jpeg.num_jpeg_inst {
        let harvest = RREG32_SOC15(JPEG, i, mmCC_UVD_HARVESTING);
        if harvest & CC_UVD_HARVESTING__UVD_DISABLE_MASK != 0 { (*adev).jpeg.harvest_config |= 1 << i; }
    }
    if (*adev).jpeg.harvest_config == (AMDGPU_JPEG_HARVEST_JPEG0 | AMDGPU_JPEG_HARVEST_JPEG1) { return -ENOENT; }
    jpeg_v2_5_set_dec_ring_funcs(adev); jpeg_v2_5_set_irq_funcs(adev); jpeg_v2_5_set_ras_funcs(adev); 0
}

unsafe fn jpeg_v2_5_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev; let mut r;
    for i in 0..(*adev).jpeg.num_jpeg_inst {
        if (*adev).jpeg.harvest_config & (1 << i) != 0 { continue; }
        r = amdgpu_irq_add_id(adev, AMDGPU_IH_CLIENTID_JPEG[i as usize], VCN_2_0__SRCID__JPEG_DECODE, &mut (*adev).jpeg.inst[i as usize].irq); if r != 0 { return r; }
        r = amdgpu_irq_add_id(adev, AMDGPU_IH_CLIENTID_JPEG[i as usize], VCN_2_6__SRCID_DJPEG0_POISON, &mut (*adev).jpeg.inst[i as usize].ras_poison_irq); if r != 0 { return r; }
        r = amdgpu_irq_add_id(adev, AMDGPU_IH_CLIENTID_JPEG[i as usize], VCN_2_6__SRCID_EJPEG0_POISON, &mut (*adev).jpeg.inst[i as usize].ras_poison_irq); if r != 0 { return r; }
    }
    r = amdgpu_jpeg_sw_init(adev); if r != 0 { return r; }
    r = amdgpu_jpeg_resume(adev); if r != 0 { return r; }
    for i in 0..(*adev).jpeg.num_jpeg_inst {
        if (*adev).jpeg.harvest_config & (1 << i) != 0 { continue; }
        let ring = (*adev).jpeg.inst[i as usize].ring_dec;
        (*ring).use_doorbell = true;
        (*ring).vm_hub = if amdgpu_ip_version(adev, UVD_HWIP, 0) == IP_VERSION(2,5,0) { AMDGPU_MMHUB1(0) } else { AMDGPU_MMHUB0(0) };
        (*ring).doorbell_index = ((*adev).doorbell_index.vcn.vcn_ring0_1 << 1) + 1 + 8 * i;
        snprintf((*ring).name.as_mut_ptr(), (*ring).name.len(), c"jpeg_dec_%d".as_ptr(), i);
        r = amdgpu_ring_init(adev, ring, 512, &mut (*adev).jpeg.inst[i as usize].irq, 0, AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut()); if r != 0 { return r; }
        (*adev).jpeg.internal.jpeg_pitch[0] = MMUVD_JPEG_PITCH_INTERNAL_OFFSET;
        (*adev).jpeg.inst[i as usize].external.jpeg_pitch[0] = SOC15_REG_OFFSET(JPEG, i, mmUVD_JPEG_PITCH);
    }
    r = amdgpu_jpeg_ras_sw_init(adev); if r != 0 { return r; }
    r = amdgpu_jpeg_reg_dump_init(adev, jpeg_reg_list_2_5.as_ptr(), jpeg_reg_list_2_5.len()); if r != 0 { return r; }
    (*adev).jpeg.supported_reset = amdgpu_get_soft_full_reset_mask((*adev).jpeg.inst[0].ring_dec);
    if !amdgpu_sriov_vf(adev) { (*adev).jpeg.supported_reset |= AMDGPU_RESET_TYPE_PER_QUEUE; }
    amdgpu_jpeg_sysfs_reset_mask_init(adev)
}

unsafe fn jpeg_v2_5_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32 { let adev=(*ip_block).adev; let r=amdgpu_jpeg_suspend(adev); if r!=0{return r;} amdgpu_jpeg_sysfs_reset_mask_fini(adev); amdgpu_jpeg_sw_fini(adev) }

unsafe fn jpeg_v2_5_hw_init(ip_block: *mut amdgpu_ip_block) -> i32 { let adev=(*ip_block).adev; for i in 0..(*adev).jpeg.num_jpeg_inst { if (*adev).jpeg.harvest_config&(1<<i)!=0{continue;} let ring=(*adev).jpeg.inst[i as usize].ring_dec; (*adev).nbio.funcs.as_ref().unwrap().vcn_doorbell_range(adev,(*ring).use_doorbell,((*adev).doorbell_index.vcn.vcn_ring0_1<<1)+8*i,i); let r=amdgpu_ring_test_helper(ring); if r!=0{return r;} } 0 }
unsafe fn jpeg_v2_5_hw_fini(ip_block:*mut amdgpu_ip_block)->i32 { let adev=(*ip_block).adev; cancel_delayed_work_sync(&mut (*adev).jpeg.idle_work); for i in 0..(*adev).jpeg.num_jpeg_inst { if (*adev).jpeg.harvest_config&(1<<i)!=0{continue;} if (*adev).jpeg.cur_state!=AMDGPU_PG_STATE_GATE && RREG32_SOC15(JPEG,i,mmUVD_JRBC_STATUS)!=0 { jpeg_v2_5_set_powergating_state(ip_block,AMDGPU_PG_STATE_GATE); } if amdgpu_ras_is_supported(adev,AMDGPU_RAS_BLOCK__JPEG){amdgpu_irq_put(adev,&mut (*adev).jpeg.inst[i as usize].ras_poison_irq,0);} } 0 }
unsafe fn jpeg_v2_5_suspend(ip_block:*mut amdgpu_ip_block)->i32 { let r=jpeg_v2_5_hw_fini(ip_block); if r!=0{return r;} amdgpu_jpeg_suspend((*ip_block).adev) }
unsafe fn jpeg_v2_5_resume(ip_block:*mut amdgpu_ip_block)->i32 { let r=amdgpu_jpeg_resume((*ip_block).adev); if r!=0{return r;} jpeg_v2_5_hw_init(ip_block) }

unsafe fn jpeg_v2_5_disable_clock_gating(adev:*mut amdgpu_device,inst:i32){let mut d=RREG32_SOC15(JPEG,inst,mmJPEG_CGC_CTRL); if (*adev).cg_flags&AMD_CG_SUPPORT_JPEG_MGCG!=0{d|=1<<JPEG_CGC_CTRL__DYN_CLOCK_MODE__SHIFT}else{d&=!JPEG_CGC_CTRL__DYN_CLOCK_MODE__SHIFT} d|=1<<JPEG_CGC_CTRL__CLK_GATE_DLY_TIMER__SHIFT;d|=4<<JPEG_CGC_CTRL__CLK_OFF_DELAY__SHIFT;WREG32_SOC15(JPEG,inst,mmJPEG_CGC_CTRL,d);d=RREG32_SOC15(JPEG,inst,mmJPEG_CGC_GATE);d&=!(JPEG_CGC_GATE__JPEG_DEC_MASK|JPEG_CGC_GATE__JPEG2_DEC_MASK|JPEG_CGC_GATE__JMCIF_MASK|JPEG_CGC_GATE__JRBBM_MASK);WREG32_SOC15(JPEG,inst,mmJPEG_CGC_GATE,d);d=RREG32_SOC15(JPEG,inst,mmJPEG_CGC_CTRL);d&=!(JPEG_CGC_CTRL__JPEG_DEC_MODE_MASK|JPEG_CGC_CTRL__JPEG2_DEC_MODE_MASK|JPEG_CGC_CTRL__JMCIF_MODE_MASK|JPEG_CGC_CTRL__JRBBM_MODE_MASK);WREG32_SOC15(JPEG,inst,mmJPEG_CGC_CTRL,d)}
unsafe fn jpeg_v2_5_enable_clock_gating(_: *mut amdgpu_device,inst:i32){let mut d=RREG32_SOC15(JPEG,inst,mmJPEG_CGC_GATE);d|=JPEG_CGC_GATE__JPEG_DEC_MASK|JPEG_CGC_GATE__JPEG2_DEC_MASK|JPEG_CGC_GATE__JPEG_ENC_MASK|JPEG_CGC_GATE__JMCIF_MASK|JPEG_CGC_GATE__JRBBM_MASK;WREG32_SOC15(JPEG,inst,mmJPEG_CGC_GATE,d)}

unsafe fn jpeg_v2_5_start_inst(adev:*mut amdgpu_device,i:i32){let ring=(*adev).jpeg.inst[i as usize].ring_dec;WREG32_P(SOC15_REG_OFFSET(JPEG,i,mmUVD_JPEG_POWER_STATUS),0,!UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK);jpeg_v2_5_disable_clock_gating(adev,i);WREG32_SOC15(JPEG,i,mmJPEG_DEC_GFX8_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config);WREG32_SOC15(JPEG,i,mmJPEG_DEC_GFX10_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config);WREG32_P(SOC15_REG_OFFSET(JPEG,i,mmUVD_JMI_CNTL),0,!UVD_JMI_CNTL__SOFT_RESET_MASK);WREG32_P(SOC15_REG_OFFSET(JPEG,i,mmJPEG_SYS_INT_EN),JPEG_SYS_INT_EN__DJRBC_MASK,!JPEG_SYS_INT_EN__DJRBC_MASK);WREG32_SOC15(JPEG,i,mmUVD_LMI_JRBC_RB_VMID,0);WREG32_SOC15(JPEG,i,mmUVD_JRBC_RB_CNTL,3);WREG32_SOC15(JPEG,i,mmUVD_LMI_JRBC_RB_64BIT_BAR_LOW,lower_32_bits((*ring).gpu_addr));WREG32_SOC15(JPEG,i,mmUVD_LMI_JRBC_RB_64BIT_BAR_HIGH,upper_32_bits((*ring).gpu_addr));WREG32_SOC15(JPEG,i,mmUVD_JRBC_RB_RPTR,0);WREG32_SOC15(JPEG,i,mmUVD_JRBC_RB_WPTR,0);WREG32_SOC15(JPEG,i,mmUVD_JRBC_RB_CNTL,2);WREG32_SOC15(JPEG,i,mmUVD_JRBC_RB_SIZE,(*ring).ring_size/4);(*ring).wptr=RREG32_SOC15(JPEG,i,mmUVD_JRBC_RB_WPTR)}
unsafe fn jpeg_v2_5_start(adev:*mut amdgpu_device)->i32{for i in 0..(*adev).jpeg.num_jpeg_inst{if (*adev).jpeg.harvest_config&(1<<i)==0{jpeg_v2_5_start_inst(adev,i)}}0}
unsafe fn jpeg_v2_5_stop_inst(adev:*mut amdgpu_device,i:i32){WREG32_P(SOC15_REG_OFFSET(JPEG,i,mmUVD_JMI_CNTL),UVD_JMI_CNTL__SOFT_RESET_MASK,!UVD_JMI_CNTL__SOFT_RESET_MASK);jpeg_v2_5_enable_clock_gating(adev,i);WREG32_P(SOC15_REG_OFFSET(JPEG,i,mmUVD_JPEG_POWER_STATUS),UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK,!UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK)}
unsafe fn jpeg_v2_5_stop(adev:*mut amdgpu_device)->i32{for i in 0..(*adev).jpeg.num_jpeg_inst{if (*adev).jpeg.harvest_config&(1<<i)==0{jpeg_v2_5_stop_inst(adev,i)}}0}

unsafe fn jpeg_v2_5_dec_ring_get_rptr(ring:*mut amdgpu_ring)->u64{RREG32_SOC15(JPEG,(*ring).me,mmUVD_JRBC_RB_RPTR) as u64}
unsafe fn jpeg_v2_5_dec_ring_get_wptr(ring:*mut amdgpu_ring)->u64{if (*ring).use_doorbell{*(*ring).wptr_cpu_addr as u64}else{RREG32_SOC15(JPEG,(*ring).me,mmUVD_JRBC_RB_WPTR) as u64}}
unsafe fn jpeg_v2_5_dec_ring_set_wptr(ring:*mut amdgpu_ring){if (*ring).use_doorbell{*(*ring).wptr_cpu_addr=lower_32_bits((*ring).wptr);WDOORBELL32((*ring).doorbell_index,lower_32_bits((*ring).wptr))}else{WREG32_SOC15(JPEG,(*ring).me,mmUVD_JRBC_RB_WPTR,lower_32_bits((*ring).wptr))}}
unsafe fn jpeg_v2_6_dec_ring_insert_start(ring:*mut amdgpu_ring){amdgpu_ring_write(ring,PACKETJ(mmUVD_JRBC_EXTERNAL_REG_INTERNAL_OFFSET,0,0,PACKETJ_TYPE0));amdgpu_ring_write(ring,0x6aa04);amdgpu_ring_write(ring,PACKETJ(JRBC_DEC_EXTERNAL_REG_WRITE_ADDR,0,0,PACKETJ_TYPE0));amdgpu_ring_write(ring,0x80000000|(1<<((*ring).me*2+14)))}
unsafe fn jpeg_v2_6_dec_ring_insert_end(ring:*mut amdgpu_ring){amdgpu_ring_write(ring,PACKETJ(mmUVD_JRBC_EXTERNAL_REG_INTERNAL_OFFSET,0,0,PACKETJ_TYPE0));amdgpu_ring_write(ring,0x6aa04);amdgpu_ring_write(ring,PACKETJ(JRBC_DEC_EXTERNAL_REG_WRITE_ADDR,0,0,PACKETJ_TYPE0));amdgpu_ring_write(ring,1<<((*ring).me*2+14))}

unsafe fn jpeg_v2_5_is_idle(ip:*mut amdgpu_ip_block)->bool{let a=(*ip).adev;let mut ret=true;for i in 0..(*a).jpeg.num_jpeg_inst{if (*a).jpeg.harvest_config&(1<<i)==0{ret&=(RREG32_SOC15(JPEG,i,mmUVD_JRBC_STATUS)&UVD_JRBC_STATUS__RB_JOB_DONE_MASK)==UVD_JRBC_STATUS__RB_JOB_DONE_MASK}}ret}
unsafe fn jpeg_v2_5_wait_for_idle(ip:*mut amdgpu_ip_block)->i32{let a=(*ip).adev;for i in 0..(*a).jpeg.num_jpeg_inst{if (*a).jpeg.harvest_config&(1<<i)==0{let r=SOC15_WAIT_ON_RREG(JPEG,i,mmUVD_JRBC_STATUS,UVD_JRBC_STATUS__RB_JOB_DONE_MASK,UVD_JRBC_STATUS__RB_JOB_DONE_MASK);if r!=0{return r}}}0}
unsafe fn jpeg_v2_5_set_clockgating_state(ip:*mut amdgpu_ip_block,state:amd_clockgating_state)->i32{let a=(*ip).adev;let en=state==AMDGPU_CG_STATE_GATE;for i in 0..(*a).jpeg.num_jpeg_inst{if (*a).jpeg.harvest_config&(1<<i)!=0{continue}if en{if !jpeg_v2_5_is_idle(ip){return -EBUSY}jpeg_v2_5_enable_clock_gating(a,i)}else{jpeg_v2_5_disable_clock_gating(a,i)}}0}
unsafe fn jpeg_v2_5_set_powergating_state(ip:*mut amdgpu_ip_block,state:amd_powergating_state)->i32{let a=(*ip).adev;if state==(*a).jpeg.cur_state{return 0}let r=if state==AMDGPU_PG_STATE_GATE{jpeg_v2_5_stop(a)}else{jpeg_v2_5_start(a)};if r==0{(*a).jpeg.cur_state=state}r}
unsafe fn jpeg_v2_5_set_interrupt_state(_: *mut amdgpu_device,_:*mut amdgpu_irq_src,_:u32,_:amdgpu_interrupt_state)->i32{0}
unsafe fn jpeg_v2_6_set_ras_interrupt_state(_: *mut amdgpu_device,_:*mut amdgpu_irq_src,_:u32,_:amdgpu_interrupt_state)->i32{0}
unsafe fn jpeg_v2_5_process_interrupt(a:*mut amdgpu_device,_:*mut amdgpu_irq_src,e:*mut amdgpu_iv_entry)->i32{let inst=match (*e).client_id{SOC15_IH_CLIENTID_VCN=>0,SOC15_IH_CLIENTID_VCN1=>1,_=>{DRM_ERROR!("Unhandled client id: %d\n",(*e).client_id);return 0}};DRM_DEBUG!("IH: JPEG TRAP\n");if (*e).src_id==VCN_2_0__SRCID__JPEG_DECODE{amdgpu_fence_process((*a).jpeg.inst[inst].ring_dec)}else{DRM_ERROR!("Unhandled interrupt: %d %d\n",(*e).src_id,(*e).src_data[0])}0}
unsafe fn jpeg_v2_5_ring_reset(r:*mut amdgpu_ring,_:u32,f:*mut amdgpu_fence)->i32{amdgpu_ring_reset_helper_begin(r,f);jpeg_v2_5_stop_inst((*r).adev,(*r).me);jpeg_v2_5_start_inst((*r).adev,(*r).me);amdgpu_ring_reset_helper_end(r,f)}

unsafe fn jpeg_v2_5_set_dec_ring_funcs(a:*mut amdgpu_device){for i in 0..(*a).jpeg.num_jpeg_inst{if (*a).jpeg.harvest_config&(1<<i)!=0{continue}(*a).jpeg.inst[i as usize].ring_dec.as_mut().unwrap().funcs=if (*a).asic_type==CHIP_ARCTURUS{&jpeg_v2_5_dec_ring_vm_funcs}else{&jpeg_v2_6_dec_ring_vm_funcs};(*a).jpeg.inst[i as usize].ring_dec.as_mut().unwrap().me=i}}
unsafe fn jpeg_v2_5_set_irq_funcs(a:*mut amdgpu_device){for i in 0..(*a).jpeg.num_jpeg_inst{if (*a).jpeg.harvest_config&(1<<i)==0{(*a).jpeg.inst[i as usize].irq.num_types=1;(*a).jpeg.inst[i as usize].irq.funcs=&jpeg_v2_5_irq_funcs;(*a).jpeg.inst[i as usize].ras_poison_irq.num_types=1;(*a).jpeg.inst[i as usize].ras_poison_irq.funcs=&jpeg_v2_6_ras_irq_funcs}}}

unsafe fn jpeg_v2_6_query_poison_by_instance(a:*mut amdgpu_device,instance:u32,sub:u32)->u32{let v=match sub{AMDGPU_JPEG_V2_6_JPEG0=>REG_GET_FIELD(RREG32_SOC15(JPEG,instance,mmUVD_RAS_JPEG0_STATUS),UVD_RAS_JPEG0_STATUS,POISONED_PF),AMDGPU_JPEG_V2_6_JPEG1=>REG_GET_FIELD(RREG32_SOC15(JPEG,instance,mmUVD_RAS_JPEG1_STATUS),UVD_RAS_JPEG1_STATUS,POISONED_PF),_=>0};if v!=0{dev_info((*a).dev,c"Poison detected in JPEG%d sub_block%d\n".as_ptr(),instance,sub)}v}
unsafe fn jpeg_v2_6_query_ras_poison_status(a:*mut amdgpu_device)->bool{let mut p=0;for i in 0..(*a).jpeg.num_jpeg_inst as u32{for s in 0..AMDGPU_JPEG_V2_6_MAX_SUB_BLOCK{p+=jpeg_v2_6_query_poison_by_instance(a,i,s)}}p!=0}
unsafe fn jpeg_v2_5_set_ras_funcs(a:*mut amdgpu_device){if amdgpu_ip_version(a,JPEG_HWIP,0)==IP_VERSION(2,6,0){(*a).jpeg.ras=&mut jpeg_v2_6_ras}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
