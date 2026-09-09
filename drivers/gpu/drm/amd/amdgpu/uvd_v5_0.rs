/* Rust translation of uvd_v5_0.c. External kernel types, constants, and
 * register helpers are supplied by the surrounding AMDGPU translation. */

unsafe fn uvd_v5_0_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    RREG32(adev, mmUVD_RBC_RB_RPTR) as u64
}
unsafe fn uvd_v5_0_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 {
    RREG32((*ring).adev, mmUVD_RBC_RB_WPTR) as u64
}
unsafe fn uvd_v5_0_ring_set_wptr(ring: *mut amdgpu_ring) {
    WREG32((*ring).adev, mmUVD_RBC_RB_WPTR, lower_32_bits((*ring).wptr));
}

unsafe fn uvd_v5_0_early_init(ip: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip).adev; (*adev).uvd.num_uvd_inst = 1;
    uvd_v5_0_set_ring_funcs(adev); uvd_v5_0_set_irq_funcs(adev); 0
}
unsafe fn uvd_v5_0_sw_init(ip: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip).adev; let mut r;
    r = amdgpu_irq_add_id(adev, AMDGPU_IRQ_CLIENTID_LEGACY, VISLANDS30_IV_SRCID_UVD_SYSTEM_MESSAGE, &mut (*(*adev).uvd.inst).irq); if r != 0 { return r; }
    r = amdgpu_uvd_sw_init(adev); if r != 0 { return r; }
    let ring = &mut (*(*adev).uvd.inst).ring; sprintf(ring.name.as_mut_ptr(), b"uvd\0".as_ptr());
    r = amdgpu_ring_init(adev, ring, 512, &mut (*(*adev).uvd.inst).irq, 0, AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut()); if r != 0 { return r; }
    amdgpu_uvd_resume(adev)
}
unsafe fn uvd_v5_0_sw_fini(ip: *mut amdgpu_ip_block) -> i32 { let adev=(*ip).adev; let r=amdgpu_uvd_suspend(adev); if r!=0 {return r;} amdgpu_uvd_sw_fini(adev) }

unsafe fn uvd_v5_0_hw_init(ip: *mut amdgpu_ip_block) -> i32 {
    let adev=(*ip).adev; let ring=&mut (*(*adev).uvd.inst).ring; let mut r;
    amdgpu_asic_set_uvd_clocks(adev,10000,10000); uvd_v5_0_set_clockgating_state(ip,AMD_CG_STATE_UNGATE); uvd_v5_0_enable_mgcg(adev,true);
    r=amdgpu_ring_test_helper(ring); if r!=0{return r;} r=amdgpu_ring_alloc(ring,10); if r!=0{return r;}
    for (reg,val) in [(mmUVD_SEMA_WAIT_FAULT_TIMEOUT_CNTL,0xfffff),(mmUVD_SEMA_WAIT_INCOMPLETE_TIMEOUT_CNTL,0xfffff),(mmUVD_SEMA_SIGNAL_INCOMPLETE_TIMEOUT_CNTL,0xfffff)] { amdgpu_ring_write(ring,PACKET0(reg,0)); amdgpu_ring_write(ring,val); }
    amdgpu_ring_write(ring,PACKET0(mmUVD_SEMA_TIMEOUT_STATUS,0)); amdgpu_ring_write(ring,8); amdgpu_ring_write(ring,PACKET0(mmUVD_SEMA_CNTL,0)); amdgpu_ring_write(ring,3); amdgpu_ring_commit(ring); r
}
unsafe fn uvd_v5_0_hw_fini(ip:*mut amdgpu_ip_block)->i32 { let adev=(*ip).adev; cancel_delayed_work_sync(&mut (*adev).uvd.idle_work); if RREG32(adev,mmUVD_STATUS)!=0 {uvd_v5_0_stop(adev);} 0 }
unsafe fn uvd_v5_0_prepare_suspend(ip:*mut amdgpu_ip_block)->i32 {amdgpu_uvd_prepare_suspend((*ip).adev)}
unsafe fn uvd_v5_0_suspend(ip:*mut amdgpu_ip_block)->i32 {let adev=(*ip).adev; cancel_delayed_work_sync(&mut (*adev).uvd.idle_work); if (*adev).pm.dpm_enabled {amdgpu_dpm_enable_uvd(adev,false);} else {amdgpu_asic_set_uvd_clocks(adev,0,0); amdgpu_device_ip_set_powergating_state(adev,AMDGPU_IP_BLOCK_TYPE_UVD,AMD_PG_STATE_GATE); amdgpu_device_ip_set_clockgating_state(adev,AMDGPU_IP_BLOCK_TYPE_UVD,AMD_CG_STATE_GATE);} let r=uvd_v5_0_hw_fini(ip); if r!=0{return r;} amdgpu_uvd_suspend(adev)}
unsafe fn uvd_v5_0_resume(ip:*mut amdgpu_ip_block)->i32 {let r=amdgpu_uvd_resume((*ip).adev); if r!=0{return r;} uvd_v5_0_hw_init(ip)}

unsafe fn uvd_v5_0_mc_resume(adev:*mut amdgpu_device) { let mut offset=AMDGPU_UVD_FIRMWARE_OFFSET; let mut size=AMDGPU_UVD_FIRMWARE_SIZE(adev); WREG32(adev,mmUVD_LMI_VCPU_CACHE_64BIT_BAR_LOW,lower_32_bits((*(*adev).uvd.inst).gpu_addr)); WREG32(adev,mmUVD_LMI_VCPU_CACHE_64BIT_BAR_HIGH,upper_32_bits((*(*adev).uvd.inst).gpu_addr)); WREG32(adev,mmUVD_VCPU_CACHE_OFFSET0,(offset>>3) as u32); WREG32(adev,mmUVD_VCPU_CACHE_SIZE0,size); offset+=size as u64; size=AMDGPU_UVD_HEAP_SIZE; WREG32(adev,mmUVD_VCPU_CACHE_OFFSET1,(offset>>3) as u32); WREG32(adev,mmUVD_VCPU_CACHE_SIZE1,size); offset+=size as u64; size=AMDGPU_UVD_STACK_SIZE+AMDGPU_UVD_SESSION_SIZE*(*adev).uvd.max_handles; WREG32(adev,mmUVD_VCPU_CACHE_OFFSET2,(offset>>3) as u32); WREG32(adev,mmUVD_VCPU_CACHE_SIZE2,size); WREG32(adev,mmUVD_UDEC_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config); WREG32(adev,mmUVD_UDEC_DB_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config); WREG32(adev,mmUVD_UDEC_DBW_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config); }

unsafe fn uvd_v5_0_start(adev:*mut amdgpu_device)->i32 { let ring=&mut (*(*adev).uvd.inst).ring; WREG32_P(adev,mmUVD_POWER_STATUS,0,!(1<<2)); uvd_v5_0_mc_resume(adev); WREG32_P(adev,mmUVD_MASTINT_EN,0,!(1<<1)); WREG32_P(adev,mmUVD_LMI_CTRL2,1<<8,!(1<<8)); mdelay(1); WREG32(adev,mmUVD_SOFT_RESET,UVD_SOFT_RESET__LMI_SOFT_RESET_MASK|UVD_SOFT_RESET__VCPU_SOFT_RESET_MASK|UVD_SOFT_RESET__LBSI_SOFT_RESET_MASK|UVD_SOFT_RESET__RBC_SOFT_RESET_MASK|UVD_SOFT_RESET__CSM_SOFT_RESET_MASK|UVD_SOFT_RESET__CXW_SOFT_RESET_MASK|UVD_SOFT_RESET__TAP_SOFT_RESET_MASK|UVD_SOFT_RESET__LMI_UMC_SOFT_RESET_MASK); mdelay(5); WREG32_P(adev,mmSRBM_SOFT_RESET,0,!SRBM_SOFT_RESET__SOFT_RESET_UVD_MASK); mdelay(5); WREG32(adev,mmUVD_LMI_CTRL,0x40|(1<<8)|(1<<13)|(1<<21)|(1<<9)|(1<<20)); WREG32(adev,mmUVD_LMI_SWAP_CNTL,0); WREG32(adev,mmUVD_MP_SWAP_CNTL,0); WREG32(adev,mmUVD_MPC_SET_MUXA0,0x40c2040); WREG32(adev,mmUVD_MPC_SET_MUXA1,0); WREG32(adev,mmUVD_MPC_SET_MUXB0,0x40c2040); WREG32(adev,mmUVD_MPC_SET_MUXB1,0); WREG32(adev,mmUVD_MPC_SET_ALU,0); WREG32(adev,mmUVD_MPC_SET_MUX,0x88); WREG32(adev,mmUVD_SOFT_RESET,UVD_SOFT_RESET__VCPU_SOFT_RESET_MASK); mdelay(5); WREG32(adev,mmUVD_VCPU_CNTL,1<<9); WREG32_P(adev,mmUVD_LMI_CTRL2,0,!(1<<8)); WREG32(adev,mmUVD_SOFT_RESET,0); mdelay(10); let mut r=-1; for _ in 0..10 { let mut status=0; for _ in 0..100 {status=RREG32(adev,mmUVD_STATUS); if status&2!=0{break;} mdelay(10);} if status&2!=0 {r=0;break;} WREG32_P(adev,mmUVD_SOFT_RESET,UVD_SOFT_RESET__VCPU_SOFT_RESET_MASK,!UVD_SOFT_RESET__VCPU_SOFT_RESET_MASK); mdelay(10); WREG32_P(adev,mmUVD_SOFT_RESET,0,!UVD_SOFT_RESET__VCPU_SOFT_RESET_MASK); mdelay(10);} if r!=0{return r;} WREG32_P(adev,mmUVD_MASTINT_EN,3<<1,!(3<<1)); WREG32_P(adev,mmUVD_STATUS,0,!(2<<1)); let mut tmp=REG_SET_FIELD(0,UVD_RBC_RB_CNTL,RB_BUFSZ,order_base_2(ring.ring_size)); tmp=REG_SET_FIELD(tmp,UVD_RBC_RB_CNTL,RB_BLKSZ,1); tmp=REG_SET_FIELD(tmp,UVD_RBC_RB_CNTL,RB_NO_FETCH,1); tmp=REG_SET_FIELD(tmp,UVD_RBC_RB_CNTL,RB_WPTR_POLL_EN,0); tmp=REG_SET_FIELD(tmp,UVD_RBC_RB_CNTL,RB_NO_UPDATE,1); tmp=REG_SET_FIELD(tmp,UVD_RBC_RB_CNTL,RB_RPTR_WR_EN,1); WREG32(adev,mmUVD_RBC_RB_CNTL,tmp); WREG32(adev,mmUVD_RBC_RB_WPTR_CNTL,0); WREG32(adev,mmUVD_RBC_RB_RPTR_ADDR,upper_32_bits(ring.gpu_addr)>>2); WREG32(adev,mmUVD_LMI_RBC_RB_64BIT_BAR_LOW,lower_32_bits(ring.gpu_addr)); WREG32(adev,mmUVD_LMI_RBC_RB_64BIT_BAR_HIGH,upper_32_bits(ring.gpu_addr)); WREG32(adev,mmUVD_RBC_RB_RPTR,0); ring.wptr=RREG32(adev,mmUVD_RBC_RB_RPTR) as u64; WREG32(adev,mmUVD_RBC_RB_WPTR,lower_32_bits(ring.wptr)); WREG32_P(adev,mmUVD_RBC_RB_CNTL,0,!UVD_RBC_RB_CNTL__RB_NO_FETCH_MASK); 0 }
unsafe fn uvd_v5_0_stop(adev:*mut amdgpu_device){WREG32(adev,mmUVD_RBC_RB_CNTL,0x11010101); WREG32_P(adev,mmUVD_LMI_CTRL2,1<<8,!(1<<8)); mdelay(1); WREG32(adev,mmUVD_SOFT_RESET,UVD_SOFT_RESET__VCPU_SOFT_RESET_MASK); mdelay(5); WREG32(adev,mmUVD_VCPU_CNTL,0); WREG32_P(adev,mmUVD_LMI_CTRL2,0,!(1<<8)); WREG32(adev,mmUVD_STATUS,0)}

unsafe fn uvd_v5_0_ring_emit_fence(r:*mut amdgpu_ring,addr:u64,seq:u64,flags:u32){WARN_ON(flags&AMDGPU_FENCE_FLAG_64BIT!=0); for (reg,val) in [(mmUVD_CONTEXT_ID,seq as u32),(mmUVD_GPCOM_VCPU_DATA0,addr as u32),(mmUVD_GPCOM_VCPU_DATA1,(addr>>32) as u32&0xff),(mmUVD_GPCOM_VCPU_CMD,0),(mmUVD_GPCOM_VCPU_DATA0,0),(mmUVD_GPCOM_VCPU_DATA1,0),(mmUVD_GPCOM_VCPU_CMD,2)] {amdgpu_ring_write(&mut *r,PACKET0(reg,0));amdgpu_ring_write(&mut *r,val);}}
unsafe fn uvd_v5_0_ring_test_ring(r:*mut amdgpu_ring)->i32{let a=(*r).adev;WREG32(a,mmUVD_CONTEXT_ID,0xCAFEDEAD);let mut e=amdgpu_ring_alloc(&mut *r,3);if e!=0{return e;}amdgpu_ring_write(&mut *r,PACKET0(mmUVD_CONTEXT_ID,0));amdgpu_ring_write(&mut *r,0xDEADBEEF);amdgpu_ring_commit(&mut *r);for _ in 0..(*a).usec_timeout{if RREG32(a,mmUVD_CONTEXT_ID)==0xDEADBEEF{return 0;}udelay(1)}-ETIMEDOUT}
unsafe fn uvd_v5_0_ring_emit_ib(r:*mut amdgpu_ring,_job:*mut amdgpu_job,ib:*mut amdgpu_ib,_flags:u32){for (reg,val) in [(mmUVD_LMI_RBC_IB_64BIT_BAR_LOW,lower_32_bits((*ib).gpu_addr)),(mmUVD_LMI_RBC_IB_64BIT_BAR_HIGH,upper_32_bits((*ib).gpu_addr)),(mmUVD_RBC_IB_SIZE,(*ib).length_dw)]{amdgpu_ring_write(&mut*r,PACKET0(reg,0));amdgpu_ring_write(&mut*r,val)}}
unsafe fn uvd_v5_0_ring_insert_nop(r:*mut amdgpu_ring,count:u32){WARN_ON((*r).wptr%2!=0||count%2!=0);for _ in 0..count/2{amdgpu_ring_write(&mut*r,PACKET0(mmUVD_NO_OP,0));amdgpu_ring_write(&mut*r,0)}}
unsafe fn uvd_v5_0_is_idle(ip:*mut amdgpu_ip_block)->bool{RREG32((*ip).adev,mmSRBM_STATUS)&SRBM_STATUS__UVD_BUSY_MASK==0}
unsafe fn uvd_v5_0_wait_for_idle(ip:*mut amdgpu_ip_block)->i32{for _ in 0..(*(*ip).adev).usec_timeout{if uvd_v5_0_is_idle(ip){return 0}}-ETIMEDOUT}
unsafe fn uvd_v5_0_soft_reset(ip:*mut amdgpu_ip_block)->i32{let a=(*ip).adev;uvd_v5_0_stop(a);WREG32_P(a,mmSRBM_SOFT_RESET,SRBM_SOFT_RESET__SOFT_RESET_UVD_MASK,!SRBM_SOFT_RESET__SOFT_RESET_UVD_MASK);mdelay(5);uvd_v5_0_start(a)}

// The remaining clock-gating and callback tables retain the C driver's external interfaces.
unsafe fn uvd_v5_0_set_interrupt_state(_: *mut amdgpu_device,_:*mut amdgpu_irq_src,_:u32,_:amdgpu_interrupt_state)->i32{0}
unsafe fn uvd_v5_0_process_interrupt(a:*mut amdgpu_device,_:*mut amdgpu_irq_src,_:*mut amdgpu_iv_entry)->i32{amdgpu_fence_process(&mut (*(*a).uvd.inst).ring);0}
unsafe fn uvd_v5_0_enable_mgcg(a:*mut amdgpu_device,enable:bool){let mut d=RREG32_UVD_CTX(a,ixUVD_CGC_MEM_CTRL);if enable&&(*a).cg_flags&AMD_CG_SUPPORT_UVD_MGCG!=0{d|=0xfff}else{d&=!0xfff}WREG32_UVD_CTX(a,ixUVD_CGC_MEM_CTRL,d);let o=RREG32(a,mmUVD_CGC_CTRL);let n=if enable{o|UVD_CGC_CTRL__DYN_CLOCK_MODE_MASK}else{o&!UVD_CGC_CTRL__DYN_CLOCK_MODE_MASK};if o!=n{WREG32(a,mmUVD_CGC_CTRL,n)}}
unsafe fn uvd_v5_0_set_clockgating_state(ip:*mut amdgpu_ip_block,state:amd_clockgating_state)->i32{if state==AMD_CG_STATE_GATE&&uvd_v5_0_wait_for_idle(ip)!=0{return -EBUSY}uvd_v5_0_enable_mgcg((*ip).adev,state!=AMD_CG_STATE_GATE);0}
unsafe fn uvd_v5_0_set_powergating_state(ip:*mut amdgpu_ip_block,state:amd_powergating_state)->i32{if state==AMD_PG_STATE_GATE{uvd_v5_0_stop((*ip).adev);0}else{uvd_v5_0_start((*ip).adev)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
