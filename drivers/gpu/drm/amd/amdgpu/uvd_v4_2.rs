/* Translated from gpu/drm/amd/amdgpu/uvd_v4_2.c. */

unsafe fn uvd_v4_2_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    RREG32!(adev, mmUVD_RBC_RB_RPTR) as u64
}
unsafe fn uvd_v4_2_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    RREG32!(adev, mmUVD_RBC_RB_WPTR) as u64
}
unsafe fn uvd_v4_2_ring_set_wptr(ring: *mut amdgpu_ring) {
    let adev = (*ring).adev;
    WREG32!(adev, mmUVD_RBC_RB_WPTR, lower_32_bits((*ring).wptr));
}

unsafe fn uvd_v4_2_early_init(ip: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip).adev;
    /* UVD doesn't work without DPM, it needs DPM to ungate it. */
    if !amdgpu_dpm { return -ENOENT; }
    (*adev).uvd.num_uvd_inst = 1;
    uvd_v4_2_set_ring_funcs(adev); uvd_v4_2_set_irq_funcs(adev); 0
}
unsafe fn uvd_v4_2_sw_init(ip: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip).adev;
    let mut r = amdgpu_irq_add_id(adev, AMDGPU_IRQ_CLIENTID_LEGACY, 124, &mut (*(*adev).uvd.inst).irq);
    if r != 0 { return r; }
    r = amdgpu_uvd_sw_init(adev); if r != 0 { return r; }
    let ring = &mut (*(*adev).uvd.inst).ring;
    sprintf!(ring.name, "uvd");
    r = amdgpu_ring_init(adev, ring, 512, &mut (*(*adev).uvd.inst).irq, 0, AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut());
    if r != 0 { return r; }
    r = amdgpu_uvd_resume(adev); if r != 0 { return r; } r
}
unsafe fn uvd_v4_2_sw_fini(ip: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip).adev; let r = amdgpu_uvd_suspend(adev); if r != 0 { return r; } amdgpu_uvd_sw_fini(adev)
}
unsafe fn uvd_v4_2_enable_mgcg(adev: *mut amdgpu_device, enable: bool) {
    let (mut orig, mut data): (u32,u32);
    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_UVD_MGCG) != 0 {
        data = RREG32_UVD_CTX!(adev, ixUVD_CGC_MEM_CTRL); data |= 0xfff; WREG32_UVD_CTX!(adev, ixUVD_CGC_MEM_CTRL, data);
        orig = RREG32!(adev, mmUVD_CGC_CTRL); data = orig | UVD_CGC_CTRL__DYN_CLOCK_MODE_MASK;
        if orig != data { WREG32!(adev, mmUVD_CGC_CTRL, data); }
    } else {
        data = RREG32_UVD_CTX!(adev, ixUVD_CGC_MEM_CTRL) & !0xfff; WREG32_UVD_CTX!(adev, ixUVD_CGC_MEM_CTRL, data);
        orig = RREG32!(adev, mmUVD_CGC_CTRL); data = orig & !UVD_CGC_CTRL__DYN_CLOCK_MODE_MASK;
        if orig != data { WREG32!(adev, mmUVD_CGC_CTRL, data); }
    }
}
unsafe fn uvd_v4_2_hw_init(ip: *mut amdgpu_ip_block) -> i32 {
    let adev=(*ip).adev; let ring=&mut (*(*adev).uvd.inst).ring; let mut r;
    uvd_v4_2_enable_mgcg(adev,true); amdgpu_asic_set_uvd_clocks(adev,10000,10000);
    r=amdgpu_ring_test_helper(ring); if r!=0{return r;} r=amdgpu_ring_alloc(ring,10); if r!=0{return r;}
    for (reg,val) in [(mmUVD_SEMA_WAIT_FAULT_TIMEOUT_CNTL,0xfffff),(mmUVD_SEMA_WAIT_INCOMPLETE_TIMEOUT_CNTL,0xfffff),(mmUVD_SEMA_SIGNAL_INCOMPLETE_TIMEOUT_CNTL,0xfffff)] { amdgpu_ring_write(ring,PACKET0!(reg,0)); amdgpu_ring_write(ring,val); }
    amdgpu_ring_write(ring,PACKET0!(mmUVD_SEMA_TIMEOUT_STATUS,0)); amdgpu_ring_write(ring,8); amdgpu_ring_write(ring,PACKET0!(mmUVD_SEMA_CNTL,0)); amdgpu_ring_write(ring,3); amdgpu_ring_commit(ring); 0
}
unsafe fn uvd_v4_2_hw_fini(ip:*mut amdgpu_ip_block)->i32 { let adev=(*ip).adev; cancel_delayed_work_sync!(&mut (*adev).uvd.idle_work); if RREG32!(adev,mmUVD_STATUS)!=0 {uvd_v4_2_stop(adev);} 0 }
unsafe fn uvd_v4_2_prepare_suspend(ip:*mut amdgpu_ip_block)->i32 { amdgpu_uvd_prepare_suspend((*ip).adev) }
unsafe fn uvd_v4_2_suspend(ip:*mut amdgpu_ip_block)->i32 { let adev=(*ip).adev; cancel_delayed_work_sync!(&mut (*adev).uvd.idle_work); if (*adev).pm.dpm_enabled {amdgpu_dpm_enable_uvd(adev,false);} else {amdgpu_asic_set_uvd_clocks(adev,0,0); amdgpu_device_ip_set_powergating_state(adev,AMDGPU_IP_BLOCK_TYPE_UVD,AMD_PG_STATE_GATE); amdgpu_device_ip_set_clockgating_state(adev,AMDGPU_IP_BLOCK_TYPE_UVD,AMD_CG_STATE_GATE);} let r=uvd_v4_2_hw_fini(ip); if r!=0{return r;} amdgpu_uvd_suspend(adev) }
unsafe fn uvd_v4_2_resume(ip:*mut amdgpu_ip_block)->i32 { let r=amdgpu_uvd_resume((*ip).adev); if r!=0{return r;} uvd_v4_2_hw_init(ip) }

/* The remaining register programming is kept in direct unsafe form to preserve C ordering. */
unsafe fn uvd_v4_2_start(adev:*mut amdgpu_device)->i32 { let ring=&mut (*(*adev).uvd.inst).ring; let mut r=-1; WREG32_P!(adev,mmUVD_STATUS,1<<2,!(1<<2)); uvd_v4_2_set_dcm(adev,true); WREG32!(adev,mmUVD_CGC_GATE,0); WREG32_P!(adev,mmSRBM_SOFT_RESET,0,!SRBM_SOFT_RESET__SOFT_RESET_UVD_MASK); mdelay(5); WREG32!(adev,mmUVD_VCPU_CNTL,1<<9); WREG32_P!(adev,mmUVD_MASTINT_EN,0,!(1<<1)); WREG32!(adev,mmUVD_LMI_SWAP_CNTL,0); WREG32!(adev,mmUVD_MP_SWAP_CNTL,0); WREG32!(adev,mmUVD_LMI_CTRL,0x203108); let tmp=RREG32!(adev,mmUVD_MPC_CNTL); WREG32!(adev,mmUVD_MPC_CNTL,tmp|0x10); WREG32!(adev,mmUVD_MPC_SET_MUXA0,0x40c2040); WREG32!(adev,mmUVD_MPC_SET_MUXA1,0); WREG32!(adev,mmUVD_MPC_SET_MUXB0,0x40c2040); WREG32!(adev,mmUVD_MPC_SET_MUXB1,0); WREG32!(adev,mmUVD_MPC_SET_ALU,0); WREG32!(adev,mmUVD_MPC_SET_MUX,0x88); uvd_v4_2_mc_resume(adev); let t=RREG32_UVD_CTX!(adev,ixUVD_LMI_CACHE_CTRL); WREG32_UVD_CTX!(adev,ixUVD_LMI_CACHE_CTRL,t&!0x10); WREG32_P!(adev,mmUVD_LMI_CTRL2,0,!(1<<8)); WREG32_P!(adev,mmUVD_SOFT_RESET,0,!UVD_SOFT_RESET__LMI_SOFT_RESET_MASK); WREG32_P!(adev,mmUVD_SOFT_RESET,0,!UVD_SOFT_RESET__LMI_UMC_SOFT_RESET_MASK); WREG32_P!(adev,mmUVD_SOFT_RESET,0,!UVD_SOFT_RESET__VCPU_SOFT_RESET_MASK); mdelay(10); for _ in 0..10 { for _ in 0..100 { if RREG32!(adev,mmUVD_STATUS)&2!=0 {r=0;break;} mdelay(10); } if r==0 {break;} WREG32_P!(adev,mmUVD_SOFT_RESET,UVD_SOFT_RESET__VCPU_SOFT_RESET_MASK,!UVD_SOFT_RESET__VCPU_SOFT_RESET_MASK); mdelay(10); WREG32_P!(adev,mmUVD_SOFT_RESET,0,!UVD_SOFT_RESET__VCPU_SOFT_RESET_MASK); mdelay(10); } if r!=0{return r;} WREG32_P!(adev,mmUVD_MASTINT_EN,3<<1,!(3<<1)); WREG32_P!(adev,mmUVD_STATUS,0,!(1<<2)); WREG32!(adev,mmUVD_RBC_RB_CNTL,0x11010101); WREG32!(adev,mmUVD_RBC_RB_WPTR_CNTL,0); WREG32!(adev,mmUVD_LMI_EXT40_ADDR,upper_32_bits((*ring).gpu_addr)|(0x7<<16)|(1<<31)); WREG32!(adev,mmUVD_RBC_RB_RPTR,0); (*ring).wptr=RREG32!(adev,mmUVD_RBC_RB_RPTR) as u64; WREG32!(adev,mmUVD_RBC_RB_WPTR,lower_32_bits((*ring).wptr)); WREG32!(adev,mmUVD_RBC_RB_BASE,(*ring).gpu_addr); let sz=(1<<8)|order_base_2((*ring).ring_size); WREG32_P!(adev,mmUVD_RBC_RB_CNTL,sz,!0x11f1f); 0 }

unsafe fn uvd_v4_2_stop(adev:*mut amdgpu_device) { WREG32!(adev,mmUVD_RBC_RB_CNTL,0x11010101); for _ in 0..10 { for _ in 0..100 {if RREG32!(adev,mmUVD_STATUS)&2!=0{break;} mdelay(1);} if RREG32!(adev,mmUVD_STATUS)&2!=0{break;} } for _ in 0..10 {for _ in 0..100 {if RREG32!(adev,mmUVD_LMI_STATUS)&0xf!=0{break;} mdelay(1);} if RREG32!(adev,mmUVD_LMI_STATUS)&0xf!=0{break;}} WREG32_P!(adev,mmUVD_LMI_CTRL2,1<<8,!(1<<8)); for _ in 0..10 {for _ in 0..100 {if RREG32!(adev,mmUVD_LMI_STATUS)&0x240!=0{break;} mdelay(1);} if RREG32!(adev,mmUVD_LMI_STATUS)&0x240!=0{break;}} WREG32_P!(adev,0x3D49,0,!(1<<2)); WREG32_P!(adev,mmUVD_VCPU_CNTL,0,!(1<<9)); WREG32!(adev,mmUVD_SOFT_RESET,UVD_SOFT_RESET__LMI_SOFT_RESET_MASK|UVD_SOFT_RESET__VCPU_SOFT_RESET_MASK|UVD_SOFT_RESET__LMI_UMC_SOFT_RESET_MASK); WREG32!(adev,mmUVD_STATUS,0); uvd_v4_2_set_dcm(adev,false); }

unsafe fn uvd_v4_2_ring_emit_fence(ring:*mut amdgpu_ring,addr:u64,seq:u64,flags:u32){ WARN_ON!(flags&AMDGPU_FENCE_FLAG_64BIT); for (r,v) in [(mmUVD_CONTEXT_ID,seq as u32),(mmUVD_GPCOM_VCPU_DATA0,addr as u32),(mmUVD_GPCOM_VCPU_DATA1,((addr>>32)&0xff) as u32),(mmUVD_GPCOM_VCPU_CMD,0),(mmUVD_GPCOM_VCPU_DATA0,0),(mmUVD_GPCOM_VCPU_DATA1,0),(mmUVD_GPCOM_VCPU_CMD,2)] {amdgpu_ring_write(ring,PACKET0!(r,0));amdgpu_ring_write(ring,v);} }
unsafe fn uvd_v4_2_ring_test_ring(ring:*mut amdgpu_ring)->i32 { let adev=(*ring).adev; WREG32!(adev,mmUVD_CONTEXT_ID,0xCAFEDEAD); let mut r=amdgpu_ring_alloc(ring,3); if r!=0{return r;} amdgpu_ring_write(ring,PACKET0!(mmUVD_CONTEXT_ID,0));amdgpu_ring_write(ring,0xDEADBEEF);amdgpu_ring_commit(ring); for _ in 0..(*adev).usec_timeout {if RREG32!(adev,mmUVD_CONTEXT_ID)==0xDEADBEEF{return 0;} udelay(1);} r=-ETIMEDOUT; r }
unsafe fn uvd_v4_2_ring_emit_ib(ring:*mut amdgpu_ring,_job:*mut amdgpu_job,ib:*mut amdgpu_ib,_flags:u32){amdgpu_ring_write(ring,PACKET0!(mmUVD_RBC_IB_BASE,0));amdgpu_ring_write(ring,(*ib).gpu_addr);amdgpu_ring_write(ring,PACKET0!(mmUVD_RBC_IB_SIZE,0));amdgpu_ring_write(ring,(*ib).length_dw);}
unsafe fn uvd_v4_2_ring_insert_nop(ring:*mut amdgpu_ring,count:u32){WARN_ON!((*ring).wptr%2!=0||count%2!=0);for _ in 0..count/2{amdgpu_ring_write(ring,PACKET0!(mmUVD_NO_OP,0));amdgpu_ring_write(ring,0);}}
unsafe fn uvd_v4_2_mc_resume(adev:*mut amdgpu_device){let mut addr=((*adev).uvd.inst.gpu_addr+AMDGPU_UVD_FIRMWARE_OFFSET)>>3;let mut size=AMDGPU_UVD_FIRMWARE_SIZE(adev)>>3;WREG32!(adev,mmUVD_VCPU_CACHE_OFFSET0,addr);WREG32!(adev,mmUVD_VCPU_CACHE_SIZE0,size);addr+=size;size=AMDGPU_UVD_HEAP_SIZE>>3;WREG32!(adev,mmUVD_VCPU_CACHE_OFFSET1,addr);WREG32!(adev,mmUVD_VCPU_CACHE_SIZE1,size);addr+=size;size=(AMDGPU_UVD_STACK_SIZE+AMDGPU_UVD_SESSION_SIZE*(*adev).uvd.max_handles)>>3;WREG32!(adev,mmUVD_VCPU_CACHE_OFFSET2,addr);WREG32!(adev,mmUVD_VCPU_CACHE_SIZE2,size);addr=((*adev).uvd.inst.gpu_addr>>28)&0xf;WREG32!(adev,mmUVD_LMI_ADDR_EXT,(addr<<12)|addr);addr=((*adev).uvd.inst.gpu_addr>>32)&0xff;WREG32!(adev,mmUVD_LMI_EXT40_ADDR,addr|(9<<16)|(1<<31));WREG32!(adev,mmUVD_UDEC_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config);WREG32!(adev,mmUVD_UDEC_DB_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config);WREG32!(adev,mmUVD_UDEC_DBW_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config);}

unsafe fn uvd_v4_2_set_dcm(adev:*mut amdgpu_device,sw_mode:bool){WREG32_FIELD!(adev,UVD_CGC_GATE,REGS,0);let mut tmp=RREG32!(adev,mmUVD_CGC_CTRL);tmp&=!(UVD_CGC_CTRL__CLK_OFF_DELAY_MASK|UVD_CGC_CTRL__CLK_GATE_DLY_TIMER_MASK);tmp|=UVD_CGC_CTRL__DYN_CLOCK_MODE_MASK|(1<<UVD_CGC_CTRL__CLK_GATE_DLY_TIMER__SHIFT)|(4<<UVD_CGC_CTRL__CLK_OFF_DELAY__SHIFT);let tmp2;if sw_mode{tmp&=!0x7ffff800;tmp2=UVD_CGC_CTRL2__DYN_OCLK_RAMP_EN_MASK|UVD_CGC_CTRL2__DYN_RCLK_RAMP_EN_MASK|(7<<UVD_CGC_CTRL2__GATER_DIV_ID__SHIFT);}else{tmp|=0x7ffff800;tmp2=0;}WREG32!(adev,mmUVD_CGC_CTRL,tmp);WREG32_UVD_CTX!(adev,ixUVD_CGC_CTRL2,tmp2);}
unsafe fn uvd_v4_2_is_idle(ip:*mut amdgpu_ip_block)->bool{(RREG32!((*ip).adev,mmSRBM_STATUS)&SRBM_STATUS__UVD_BUSY_MASK)==0}
unsafe fn uvd_v4_2_wait_for_idle(ip:*mut amdgpu_ip_block)->i32{let adev=(*ip).adev;for _ in 0..(*adev).usec_timeout{if RREG32!(adev,mmSRBM_STATUS)&SRBM_STATUS__UVD_BUSY_MASK==0{return 0;}}-ETIMEDOUT}
unsafe fn uvd_v4_2_soft_reset(ip:*mut amdgpu_ip_block)->i32{let adev=(*ip).adev;uvd_v4_2_stop(adev);WREG32_P!(adev,mmSRBM_SOFT_RESET,SRBM_SOFT_RESET__SOFT_RESET_UVD_MASK,!SRBM_SOFT_RESET__SOFT_RESET_UVD_MASK);mdelay(5);uvd_v4_2_start(adev)}
unsafe fn uvd_v4_2_set_interrupt_state(_adev:*mut amdgpu_device,_source:*mut amdgpu_irq_src,_ty:u32,_state:amdgpu_interrupt_state)->i32{ /* TODO */ 0 }
unsafe fn uvd_v4_2_process_interrupt(adev:*mut amdgpu_device,_source:*mut amdgpu_irq_src,_entry:*mut amdgpu_iv_entry)->i32{DRM_DEBUG!("IH: UVD TRAP\n");amdgpu_fence_process(&mut (*(*adev).uvd.inst).ring);0}
unsafe fn uvd_v4_2_set_clockgating_state(_ip:*mut amdgpu_ip_block,_state:amd_clockgating_state)->i32{0}
unsafe fn uvd_v4_2_set_powergating_state(ip:*mut amdgpu_ip_block,state:amd_powergating_state)->i32{let adev=(*ip).adev;if state==AMD_PG_STATE_GATE{uvd_v4_2_stop(adev);if (*adev).pg_flags&AMD_PG_SUPPORT_UVD!=0&&!(*adev).pm.dpm_enabled&&RREG32_SMC!(adev,ixCURRENT_PG_STATUS)&CURRENT_PG_STATUS__UVD_PG_STATUS_MASK==0{WREG32!(adev,mmUVD_PGFSM_CONFIG,UVD_PGFSM_CONFIG__UVD_PGFSM_FSM_ADDR_MASK|UVD_PGFSM_CONFIG__UVD_PGFSM_POWER_DOWN_MASK|UVD_PGFSM_CONFIG__UVD_PGFSM_P1_SELECT_MASK);mdelay(20);}0}else{if (*adev).pg_flags&AMD_PG_SUPPORT_UVD!=0&&!(*adev).pm.dpm_enabled&&RREG32_SMC!(adev,ixCURRENT_PG_STATUS)&CURRENT_PG_STATUS__UVD_PG_STATUS_MASK!=0{WREG32!(adev,mmUVD_PGFSM_CONFIG,UVD_PGFSM_CONFIG__UVD_PGFSM_FSM_ADDR_MASK|UVD_PGFSM_CONFIG__UVD_PGFSM_POWER_UP_MASK|UVD_PGFSM_CONFIG__UVD_PGFSM_P1_SELECT_MASK);mdelay(30);}uvd_v4_2_start(adev)}}

unsafe fn uvd_v4_2_set_ring_funcs(adev:*mut amdgpu_device){(*(*adev).uvd.inst).ring.funcs=&uvd_v4_2_ring_funcs;}
unsafe fn uvd_v4_2_set_irq_funcs(adev:*mut amdgpu_device){(*(*adev).uvd.inst).irq.num_types=1;(*(*adev).uvd.inst).irq.funcs=&uvd_v4_2_irq_funcs;}

/* Function-table layouts and constants are supplied by the surrounding amdgpu bindings. */
static uvd_v4_2_ring_funcs: amdgpu_ring_funcs = amdgpu_ring_funcs { type_: AMDGPU_RING_TYPE_UVD, align_mask:0xf, support_64bit_ptrs:false, no_user_fence:true, get_rptr:uvd_v4_2_ring_get_rptr, get_wptr:uvd_v4_2_ring_get_wptr, set_wptr:uvd_v4_2_ring_set_wptr, parse_cs:amdgpu_uvd_ring_parse_cs, emit_frame_size:14, emit_ib_size:4, emit_ib:uvd_v4_2_ring_emit_ib, emit_fence:uvd_v4_2_ring_emit_fence, test_ring:uvd_v4_2_ring_test_ring, test_ib:amdgpu_uvd_ring_test_ib, insert_nop:uvd_v4_2_ring_insert_nop, pad_ib:amdgpu_ring_generic_pad_ib, begin_use:amdgpu_uvd_ring_begin_use, end_use:amdgpu_uvd_ring_end_use };
static uvd_v4_2_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { set:uvd_v4_2_set_interrupt_state, process:uvd_v4_2_process_interrupt };
static uvd_v4_2_ip_funcs: amd_ip_funcs = amd_ip_funcs { name:"uvd_v4_2", early_init:uvd_v4_2_early_init, sw_init:uvd_v4_2_sw_init, sw_fini:uvd_v4_2_sw_fini, hw_init:uvd_v4_2_hw_init, hw_fini:uvd_v4_2_hw_fini, prepare_suspend:uvd_v4_2_prepare_suspend, suspend:uvd_v4_2_suspend, resume:uvd_v4_2_resume, is_idle:uvd_v4_2_is_idle, wait_for_idle:uvd_v4_2_wait_for_idle, soft_reset:uvd_v4_2_soft_reset, set_clockgating_state:uvd_v4_2_set_clockgating_state, set_powergating_state:uvd_v4_2_set_powergating_state };
pub static uvd_v4_2_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_:AMDGPU_IP_BLOCK_TYPE_UVD, major:4, minor:2, rev:0, funcs:&uvd_v4_2_ip_funcs };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
