/* Translated from jpeg_v2_0.c.  External kernel types, constants, and helpers
 * are supplied by the surrounding AMDGPU bindings. */

static jpeg_reg_list_2_0: [amdgpu_hwip_reg_entry; 13] = [
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JPEG_POWER_STATUS),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JPEG_INT_STAT),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JRBC_RB_RPTR),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JRBC_RB_WPTR),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JRBC_RB_CNTL),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JRBC_RB_SIZE),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JRBC_STATUS),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmJPEG_DEC_ADDR_MODE),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmJPEG_DEC_GFX10_ADDR_CONFIG),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmJPEG_DEC_Y_GFX10_TILING_SURFACE),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmJPEG_DEC_UV_GFX10_TILING_SURFACE),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JPEG_PITCH),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JPEG_UV_PITCH),
];

unsafe fn jpeg_v2_0_early_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    (*adev).jpeg.num_jpeg_inst = 1;
    (*adev).jpeg.num_jpeg_rings = 1;
    jpeg_v2_0_set_dec_ring_funcs(adev);
    jpeg_v2_0_set_irq_funcs(adev);
    0
}

unsafe fn jpeg_v2_0_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let r = amdgpu_irq_add_id(adev, SOC15_IH_CLIENTID_VCN,
        VCN_2_0__SRCID__JPEG_DECODE, &mut (*adev).jpeg.inst.irq);
    if r != 0 { return r; }
    let r = amdgpu_jpeg_sw_init(adev); if r != 0 { return r; }
    let r = amdgpu_jpeg_resume(adev); if r != 0 { return r; }
    let ring = (*adev).jpeg.inst.ring_dec;
    (*ring).use_doorbell = true;
    (*ring).doorbell_index = ((*adev).doorbell_index.vcn.vcn_ring0_1 << 1) + 1;
    (*ring).vm_hub = AMDGPU_MMHUB0!(0);
    sprintf!((*ring).name, "jpeg_dec");
    let r = amdgpu_ring_init(adev, ring, 512, &mut (*adev).jpeg.inst.irq, 0,
        AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut()); if r != 0 { return r; }
    (*adev).jpeg.internal.jpeg_pitch[0] = mmUVD_JPEG_PITCH_INTERNAL_OFFSET;
    (*adev).jpeg.inst.external.jpeg_pitch[0] = SOC15_REG_OFFSET!(JPEG, 0, mmUVD_JPEG_PITCH);
    let r = amdgpu_jpeg_reg_dump_init(adev, jpeg_reg_list_2_0.as_ptr(), jpeg_reg_list_2_0.len());
    if r != 0 { return r; }
    (*adev).jpeg.supported_reset = amdgpu_get_soft_full_reset_mask((*adev).jpeg.inst[0].ring_dec);
    if !amdgpu_sriov_vf(adev) { (*adev).jpeg.supported_reset |= AMDGPU_RESET_TYPE_PER_QUEUE; }
    amdgpu_jpeg_sysfs_reset_mask_init(adev)
}

unsafe fn jpeg_v2_0_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let r = amdgpu_jpeg_suspend(adev); if r != 0 { return r; }
    amdgpu_jpeg_sysfs_reset_mask_fini(adev);
    amdgpu_jpeg_sw_fini(adev)
}

unsafe fn jpeg_v2_0_hw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev; let ring = (*adev).jpeg.inst.ring_dec;
    ((*adev).nbio.funcs).vcn_doorbell_range(adev, (*ring).use_doorbell,
        (*adev).doorbell_index.vcn.vcn_ring0_1 << 1, 0);
    amdgpu_ring_test_helper(ring)
}

unsafe fn jpeg_v2_0_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    cancel_delayed_work_sync(&mut (*adev).jpeg.idle_work);
    if (*adev).jpeg.cur_state != AMD_PG_STATE_GATE && RREG32_SOC15!(JPEG, 0, mmUVD_JRBC_STATUS) != 0 {
        jpeg_v2_0_set_powergating_state(ip_block, AMD_PG_STATE_GATE);
    } 0
}
unsafe fn jpeg_v2_0_suspend(ip_block: *mut amdgpu_ip_block) -> i32 {
    let r = jpeg_v2_0_hw_fini(ip_block); if r != 0 { return r; }
    amdgpu_jpeg_suspend((*ip_block).adev)
}
unsafe fn jpeg_v2_0_resume(ip_block: *mut amdgpu_ip_block) -> i32 {
    let r = amdgpu_jpeg_resume((*ip_block).adev); if r != 0 { return r; }
    jpeg_v2_0_hw_init(ip_block)
}

unsafe fn jpeg_v2_0_disable_power_gating(adev: *mut amdgpu_device) -> i32 {
    let mut data: u32;
    if (*adev).pg_flags & AMD_PG_SUPPORT_JPEG != 0 {
        data = 1 << UVD_PGFSM_CONFIG__UVDJ_PWR_CONFIG__SHIFT;
        WREG32!(SOC15_REG_OFFSET!(JPEG, 0, mmUVD_PGFSM_CONFIG), data);
        let r = SOC15_WAIT_ON_RREG!(JPEG, 0, mmUVD_PGFSM_STATUS, UVD_PGFSM_STATUS_UVDJ_PWR_ON, UVD_PGFSM_STATUS__UVDJ_PWR_STATUS_MASK);
        if r != 0 { drm_err!(adev_to_drm!(adev), "failed to disable JPEG power gating\n"); return r; }
    }
    data = RREG32!(SOC15_REG_OFFSET!(JPEG, 0, mmUVD_JPEG_POWER_STATUS)) & !1;
    WREG32!(SOC15_REG_OFFSET!(JPEG, 0, mmUVD_JPEG_POWER_STATUS), data); 0
}
unsafe fn jpeg_v2_0_enable_power_gating(adev: *mut amdgpu_device) -> i32 {
    if (*adev).pg_flags & AMD_PG_SUPPORT_JPEG != 0 {
        let mut data = RREG32!(SOC15_REG_OFFSET!(JPEG, 0, mmUVD_JPEG_POWER_STATUS));
        data = (data & !UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK) | 1;
        WREG32!(SOC15_REG_OFFSET!(JPEG, 0, mmUVD_JPEG_POWER_STATUS), data);
        data = 2 << UVD_PGFSM_CONFIG__UVDJ_PWR_CONFIG__SHIFT;
        WREG32!(SOC15_REG_OFFSET!(JPEG, 0, mmUVD_PGFSM_CONFIG), data);
        let r = SOC15_WAIT_ON_RREG!(JPEG, 0, mmUVD_PGFSM_STATUS, 2 << UVD_PGFSM_STATUS__UVDJ_PWR_STATUS__SHIFT, UVD_PGFSM_STATUS__UVDJ_PWR_STATUS_MASK);
        if r != 0 { drm_err!(adev_to_drm!(adev), "failed to enable JPEG power gating\n"); return r; }
    } 0
}

unsafe fn jpeg_v2_0_disable_clock_gating(adev: *mut amdgpu_device) { let mut d=RREG32_SOC15!(JPEG,0,mmJPEG_CGC_CTRL); if (*adev).cg_flags&AMDGPU_CG_SUPPORT_JPEG_MGCG!=0 {d|=1<<JPEG_CGC_CTRL__DYN_CLOCK_MODE__SHIFT}else{d&=!JPEG_CGC_CTRL__DYN_CLOCK_MODE__SHIFT}; d|=1<<JPEG_CGC_CTRL__CLK_GATE_DLY_TIMER__SHIFT; d|=4<<JPEG_CGC_CTRL__CLK_OFF_DELAY__SHIFT; WREG32_SOC15!(JPEG,0,mmJPEG_CGC_CTRL,d); d=RREG32_SOC15!(JPEG,0,mmJPEG_CGC_GATE); d&=!(JPEG_CGC_GATE__JPEG_DEC_MASK|JPEG_CGC_GATE__JPEG2_DEC_MASK|JPEG_CGC_GATE__JPEG_ENC_MASK|JPEG_CGC_GATE__JMCIF_MASK|JPEG_CGC_GATE__JRBBM_MASK); WREG32_SOC15!(JPEG,0,mmJPEG_CGC_GATE,d); }
unsafe fn jpeg_v2_0_enable_clock_gating(adev: *mut amdgpu_device) { let mut d=RREG32_SOC15!(JPEG,0,mmJPEG_CGC_CTRL); if (*adev).cg_flags&AMDGPU_CG_SUPPORT_JPEG_MGCG!=0 {d|=1<<JPEG_CGC_CTRL__DYN_CLOCK_MODE__SHIFT}; d|=1<<JPEG_CGC_CTRL__CLK_GATE_DLY_TIMER__SHIFT; d|=4<<JPEG_CGC_CTRL__CLK_OFF_DELAY__SHIFT; WREG32_SOC15!(JPEG,0,mmJPEG_CGC_CTRL,d); d=RREG32_SOC15!(JPEG,0,mmJPEG_CGC_GATE); d|=JPEG_CGC_GATE__JPEG_DEC_MASK|JPEG_CGC_GATE__JPEG2_DEC_MASK|JPEG_CGC_GATE__JPEG_ENC_MASK|JPEG_CGC_GATE__JMCIF_MASK|JPEG_CGC_GATE__JRBBM_MASK; WREG32_SOC15!(JPEG,0,mmJPEG_CGC_GATE,d); }

unsafe fn jpeg_v2_0_start(adev:*mut amdgpu_device)->i32 { let ring=(*adev).jpeg.inst.ring_dec; if (*adev).pm.dpm_enabled {amdgpu_dpm_enable_jpeg(adev,true);} let r=jpeg_v2_0_disable_power_gating(adev); if r!=0{return r;} jpeg_v2_0_disable_clock_gating(adev); WREG32_SOC15!(JPEG,0,mmJPEG_DEC_GFX10_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config); WREG32_P!(SOC15_REG_OFFSET!(JPEG,0,mmUVD_JMI_CNTL),0,!UVD_JMI_CNTL__SOFT_RESET_MASK); WREG32_P!(SOC15_REG_OFFSET!(JPEG,0,mmJPEG_SYS_INT_EN),JPEG_SYS_INT_EN__DJRBC_MASK,!JPEG_SYS_INT_EN__DJRBC_MASK); WREG32_SOC15!(JPEG,0,mmUVD_LMI_JRBC_RB_VMID,0); WREG32_SOC15!(JPEG,0,mmUVD_JRBC_RB_CNTL,3); WREG32_SOC15!(JPEG,0,mmUVD_LMI_JRBC_RB_64BIT_BAR_LOW,lower_32_bits!((*ring).gpu_addr)); WREG32_SOC15!(JPEG,0,mmUVD_LMI_JRBC_RB_64BIT_BAR_HIGH,upper_32_bits!((*ring).gpu_addr)); WREG32_SOC15!(JPEG,0,mmUVD_JRBC_RB_RPTR,0); WREG32_SOC15!(JPEG,0,mmUVD_JRBC_RB_WPTR,0); WREG32_SOC15!(JPEG,0,mmUVD_JRBC_RB_CNTL,2); WREG32_SOC15!(JPEG,0,mmUVD_JRBC_RB_SIZE,(*ring).ring_size/4); (*ring).wptr=RREG32_SOC15!(JPEG,0,mmUVD_JRBC_RB_WPTR); 0 }
unsafe fn jpeg_v2_0_stop(adev:*mut amdgpu_device)->i32 { WREG32_P!(SOC15_REG_OFFSET!(JPEG,0,mmUVD_JMI_CNTL),UVD_JMI_CNTL__SOFT_RESET_MASK,!UVD_JMI_CNTL__SOFT_RESET_MASK); jpeg_v2_0_enable_clock_gating(adev); let r=jpeg_v2_0_enable_power_gating(adev); if r!=0{return r;} if (*adev).pm.dpm_enabled{amdgpu_dpm_enable_jpeg(adev,false);} 0 }

unsafe fn jpeg_v2_0_dec_ring_get_rptr(r:*mut amdgpu_ring)->u64 { let _adev=(*r).adev; RREG32_SOC15!(JPEG,0,mmUVD_JRBC_RB_RPTR) as u64 }
unsafe fn jpeg_v2_0_dec_ring_get_wptr(r:*mut amdgpu_ring)->u64 { let _adev=(*r).adev; if (*r).use_doorbell{*(*r).wptr_cpu_addr as u64}else{RREG32_SOC15!(JPEG,0,mmUVD_JRBC_RB_WPTR) as u64} }
unsafe fn jpeg_v2_0_dec_ring_set_wptr(r:*mut amdgpu_ring) { if (*r).use_doorbell{*(*r).wptr_cpu_addr=lower_32_bits!((*r).wptr); WDOORBELL32!((*r).doorbell_index,lower_32_bits!((*r).wptr));}else{WREG32_SOC15!(JPEG,0,mmUVD_JRBC_RB_WPTR,lower_32_bits!((*r).wptr));} }

pub unsafe fn jpeg_v2_0_dec_ring_insert_start(r:*mut amdgpu_ring){amdgpu_ring_write!(r,PACKETJ!(mmUVD_JRBC_EXTERNAL_REG_INTERNAL_OFFSET,0,0,PACKETJ_TYPE0));amdgpu_ring_write!(r,0x68e04);amdgpu_ring_write!(r,PACKETJ!(JRBC_DEC_EXTERNAL_REG_WRITE_ADDR,0,0,PACKETJ_TYPE0));amdgpu_ring_write!(r,0x80010000);}
pub unsafe fn jpeg_v2_0_dec_ring_insert_end(r:*mut amdgpu_ring){amdgpu_ring_write!(r,PACKETJ!(mmUVD_JRBC_EXTERNAL_REG_INTERNAL_OFFSET,0,0,PACKETJ_TYPE0));amdgpu_ring_write!(r,0x68e04);amdgpu_ring_write!(r,PACKETJ!(JRBC_DEC_EXTERNAL_REG_WRITE_ADDR,0,0,PACKETJ_TYPE0));amdgpu_ring_write!(r,0x00010000);}
pub unsafe fn jpeg_v2_0_dec_ring_emit_fence(r:*mut amdgpu_ring,addr:u64,seq:u64,flags:u32){WARN_ON!(flags&AMDGPU_FENCE_FLAG_64BIT!=0); for (p,v) in [(PACKETJ!(mmUVD_JPEG_GPCOM_DATA0_INTERNAL_OFFSET,0,0,PACKETJ_TYPE0),seq),(PACKETJ!(mmUVD_JPEG_GPCOM_DATA1_INTERNAL_OFFSET,0,0,PACKETJ_TYPE0),seq),(PACKETJ!(mmUVD_LMI_JRBC_RB_MEM_WR_64BIT_BAR_LOW_INTERNAL_OFFSET,0,0,PACKETJ_TYPE0),lower_32_bits!(addr)),(PACKETJ!(mmUVD_LMI_JRBC_RB_MEM_WR_64BIT_BAR_HIGH_INTERNAL_OFFSET,0,0,PACKETJ_TYPE0),upper_32_bits!(addr)),(PACKETJ!(mmUVD_JPEG_GPCOM_CMD_INTERNAL_OFFSET,0,0,PACKETJ_TYPE0),8),(PACKETJ!(mmUVD_JPEG_GPCOM_CMD_INTERNAL_OFFSET,0,PACKETJ_CONDITION_CHECK0,PACKETJ_TYPE4),0),(PACKETJ!(mmUVD_JRBC_EXTERNAL_REG_INTERNAL_OFFSET,0,0,PACKETJ_TYPE0),0x3fbc),(PACKETJ!(JRBC_DEC_EXTERNAL_REG_WRITE_ADDR,0,0,PACKETJ_TYPE0),1),(PACKETJ!(0,0,0,PACKETJ_TYPE7),0)]{amdgpu_ring_write!(r,p);amdgpu_ring_write!(r,v);}}

pub unsafe fn jpeg_v2_0_dec_ring_emit_ib(r:*mut amdgpu_ring,job:*mut amdgpu_job,ib:*mut amdgpu_ib,_flags:u32){let vmid=AMDGPU_JOB_GET_VMID!(job); let vals=[(mmUVD_JPEG_IH_CTRL_INTERNAL_OFFSET,vmid<<JPEG_IH_CTRL__IH_VMID__SHIFT),(mmUVD_LMI_JRBC_IB_VMID_INTERNAL_OFFSET,if (*r).funcs.parse_cs{0}else{vmid|(vmid<<4)|(vmid<<8)}),(mmUVD_LMI_JPEG_VMID_INTERNAL_OFFSET,vmid|(vmid<<4)|(vmid<<8)),(mmUVD_LMI_JRBC_IB_64BIT_BAR_LOW_INTERNAL_OFFSET,lower_32_bits!((*ib).gpu_addr)),(mmUVD_LMI_JRBC_IB_64BIT_BAR_HIGH_INTERNAL_OFFSET,upper_32_bits!((*ib).gpu_addr)),(mmUVD_JRBC_IB_SIZE_INTERNAL_OFFSET,(*ib).length_dw),(mmUVD_LMI_JRBC_RB_MEM_RD_64BIT_BAR_LOW_INTERNAL_OFFSET,lower_32_bits!((*r).gpu_addr)),(mmUVD_LMI_JRBC_RB_MEM_RD_64BIT_BAR_HIGH_INTERNAL_OFFSET,upper_32_bits!((*r).gpu_addr)),(0,PACKETJ!(0,0,PACKETJ_CONDITION_CHECK0,PACKETJ_TYPE2)),(mmUVD_JRBC_RB_COND_RD_TIMER_INTERNAL_OFFSET,0x01400200),(mmUVD_JRBC_RB_REF_DATA_INTERNAL_OFFSET,2),(mmUVD_JRBC_STATUS_INTERNAL_OFFSET,2)]; for (reg,val) in vals{amdgpu_ring_write!(r,PACKETJ!(reg,0,0,PACKETJ_TYPE0));amdgpu_ring_write!(r,val);}}

pub unsafe fn jpeg_v2_0_dec_ring_emit_reg_wait(r:*mut amdgpu_ring,reg:u32,val:u32,mask:u32){let ro=reg<<2;amdgpu_ring_write!(r,PACKETJ!(mmUVD_JRBC_RB_COND_RD_TIMER_INTERNAL_OFFSET,0,0,PACKETJ_TYPE0));amdgpu_ring_write!(r,0x01400200);amdgpu_ring_write!(r,PACKETJ!(mmUVD_JRBC_RB_REF_DATA_INTERNAL_OFFSET,0,0,PACKETJ_TYPE0));amdgpu_ring_write!(r,val);amdgpu_ring_write!(r,PACKETJ!(mmUVD_JRBC_EXTERNAL_REG_INTERNAL_OFFSET,0,0,PACKETJ_TYPE0));if ro>=0x10000&&ro<=0x105ff{amdgpu_ring_write!(r,0);amdgpu_ring_write!(r,PACKETJ!(ro>>2,0,0,PACKETJ_TYPE3));}else{amdgpu_ring_write!(r,ro);amdgpu_ring_write!(r,PACKETJ!(JRBC_DEC_EXTERNAL_REG_WRITE_ADDR,0,0,PACKETJ_TYPE3));}amdgpu_ring_write!(r,mask);}
pub unsafe fn jpeg_v2_0_dec_ring_emit_vm_flush(r:*mut amdgpu_ring,vmid:u32,mut pd:u64){let hub=&(*r).adev.vmhub[(*r).vm_hub];pd=amdgpu_gmc_emit_flush_gpu_tlb(r,vmid,pd);jpeg_v2_0_dec_ring_emit_reg_wait(r,hub.ctx0_ptb_addr_lo32+vmid*hub.ctx_addr_distance,lower_32_bits!(pd),0xffffffff);}
pub unsafe fn jpeg_v2_0_dec_ring_emit_wreg(r:*mut amdgpu_ring,reg:u32,val:u32){let ro=reg<<2;amdgpu_ring_write!(r,PACKETJ!(mmUVD_JRBC_EXTERNAL_REG_INTERNAL_OFFSET,0,0,PACKETJ_TYPE0));if ro>=0x10000&&ro<=0x105ff{amdgpu_ring_write!(r,0);amdgpu_ring_write!(r,PACKETJ!(ro>>2,0,0,PACKETJ_TYPE0));}else{amdgpu_ring_write!(r,ro);amdgpu_ring_write!(r,PACKETJ!(JRBC_DEC_EXTERNAL_REG_WRITE_ADDR,0,0,PACKETJ_TYPE0));}amdgpu_ring_write!(r,val);}
pub unsafe fn jpeg_v2_0_dec_ring_nop(r:*mut amdgpu_ring,count:u32){WARN_ON!((*r).wptr%2!=0||count%2!=0);for _ in 0..count/2{amdgpu_ring_write!(r,PACKETJ!(0,0,0,PACKETJ_TYPE6));amdgpu_ring_write!(r,0);}}

unsafe fn jpeg_v2_0_is_idle(ip:*mut amdgpu_ip_block)->bool{let _adev=(*ip).adev;(RREG32_SOC15!(JPEG,0,mmUVD_JRBC_STATUS)&UVD_JRBC_STATUS__RB_JOB_DONE_MASK)==UVD_JRBC_STATUS__RB_JOB_DONE_MASK}
unsafe fn jpeg_v2_0_wait_for_idle(_ip:*mut amdgpu_ip_block)->i32{SOC15_WAIT_ON_RREG!(JPEG,0,mmUVD_JRBC_STATUS,UVD_JRBC_STATUS__RB_JOB_DONE_MASK,UVD_JRBC_STATUS__RB_JOB_DONE_MASK)}
unsafe fn jpeg_v2_0_set_clockgating_state(ip:*mut amdgpu_ip_block,state:amd_clockgating_state)->i32{let enable=state==AMD_CG_STATE_GATE;if enable{if !jpeg_v2_0_is_idle(ip){return -EBUSY;}jpeg_v2_0_enable_clock_gating((*ip).adev);}else{jpeg_v2_0_disable_clock_gating((*ip).adev);}0}
unsafe fn jpeg_v2_0_set_powergating_state(ip:*mut amdgpu_ip_block,state:amd_powergating_state)->i32{let adev=(*ip).adev;if state==(*adev).jpeg.cur_state{return 0;}let r=if state==AMD_PG_STATE_GATE{jpeg_v2_0_stop(adev)}else{jpeg_v2_0_start(adev)};if r==0{(*adev).jpeg.cur_state=state;}r}
unsafe fn jpeg_v2_0_set_interrupt_state(_a:*mut amdgpu_device,_s:*mut amdgpu_irq_src,_t:u32,_state:amdgpu_interrupt_state)->i32{0}
pub unsafe fn jpeg_v2_0_process_interrupt(adev:*mut amdgpu_device,_s:*mut amdgpu_irq_src,e:*mut amdgpu_iv_entry)->i32{DRM_DEBUG!("IH: JPEG TRAP\n");match (*e).src_id{VCN_2_0__SRCID__JPEG_DECODE=>amdgpu_fence_process((*adev).jpeg.inst.ring_dec),_=>DRM_ERROR!("Unhandled interrupt: %d %d\n",(*e).src_id,(*e).src_data[0])};0}
unsafe fn jpeg_v2_0_ring_reset(r:*mut amdgpu_ring,_vmid:u32,f:*mut amdgpu_fence)->i32{amdgpu_ring_reset_helper_begin(r,f);let x=jpeg_v2_0_stop((*r).adev);if x!=0{return x;}let x=jpeg_v2_0_start((*r).adev);if x!=0{return x;}amdgpu_ring_reset_helper_end(r,f)}

unsafe fn jpeg_v2_0_set_dec_ring_funcs(a:*mut amdgpu_device){(*a).jpeg.inst.ring_dec.funcs=&jpeg_v2_0_dec_ring_vm_funcs;}
unsafe fn jpeg_v2_0_set_irq_funcs(a:*mut amdgpu_device){(*a).jpeg.inst.irq.num_types=1;(*a).jpeg.inst.irq.funcs=&jpeg_v2_0_irq_funcs;}

static jpeg_v2_0_ip_funcs: amd_ip_funcs = amd_ip_funcs { name:"jpeg_v2_0", early_init:Some(jpeg_v2_0_early_init), sw_init:Some(jpeg_v2_0_sw_init), sw_fini:Some(jpeg_v2_0_sw_fini), hw_init:Some(jpeg_v2_0_hw_init), hw_fini:Some(jpeg_v2_0_hw_fini), suspend:Some(jpeg_v2_0_suspend), resume:Some(jpeg_v2_0_resume), is_idle:Some(jpeg_v2_0_is_idle), wait_for_idle:Some(jpeg_v2_0_wait_for_idle), set_clockgating_state:Some(jpeg_v2_0_set_clockgating_state), set_powergating_state:Some(jpeg_v2_0_set_powergating_state), dump_ip_state:Some(amdgpu_jpeg_dump_ip_state), print_ip_state:Some(amdgpu_jpeg_print_ip_state) };
static jpeg_v2_0_dec_ring_vm_funcs: amdgpu_ring_funcs = amdgpu_ring_funcs { type_:AMDGPU_RING_TYPE_VCN_JPEG, align_mask:0xf, no_user_fence:true, get_rptr:Some(jpeg_v2_0_dec_ring_get_rptr), get_wptr:Some(jpeg_v2_0_dec_ring_get_wptr), set_wptr:Some(jpeg_v2_0_dec_ring_set_wptr), emit_ib:Some(jpeg_v2_0_dec_ring_emit_ib), emit_fence:Some(jpeg_v2_0_dec_ring_emit_fence), emit_vm_flush:Some(jpeg_v2_0_dec_ring_emit_vm_flush), insert_nop:Some(jpeg_v2_0_dec_ring_nop), insert_start:Some(jpeg_v2_0_dec_ring_insert_start), insert_end:Some(jpeg_v2_0_dec_ring_insert_end), emit_wreg:Some(jpeg_v2_0_dec_ring_emit_wreg), emit_reg_wait:Some(jpeg_v2_0_dec_ring_emit_reg_wait), reset:Some(jpeg_v2_0_ring_reset) };
static jpeg_v2_0_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { set:Some(jpeg_v2_0_set_interrupt_state), process:Some(jpeg_v2_0_process_interrupt) };
pub static jpeg_v2_0_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_:AMDGPU_IP_BLOCK_TYPE_JPEG, major:2, minor:0, rev:0, funcs:&jpeg_v2_0_ip_funcs };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
