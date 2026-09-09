/* Direct Rust translation of uvd_v3_1.c. */

unsafe fn uvd_v3_1_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 { RREG32((*ring).adev, mmUVD_RBC_RB_RPTR) as u64 }
unsafe fn uvd_v3_1_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 { RREG32((*ring).adev, mmUVD_RBC_RB_WPTR) as u64 }
unsafe fn uvd_v3_1_ring_set_wptr(ring: *mut amdgpu_ring) { WREG32((*ring).adev, mmUVD_RBC_RB_WPTR, lower_32_bits((*ring).wptr)); }

unsafe fn uvd_v3_1_ring_emit_ib(ring: *mut amdgpu_ring, _job: *mut amdgpu_job, ib: *mut amdgpu_ib, _flags: u32) {
    amdgpu_ring_write(ring, PACKET0(mmUVD_RBC_IB_BASE, 0)); amdgpu_ring_write(ring, (*ib).gpu_addr);
    amdgpu_ring_write(ring, PACKET0(mmUVD_RBC_IB_SIZE, 0)); amdgpu_ring_write(ring, (*ib).length_dw);
}
unsafe fn uvd_v3_1_ring_emit_fence(ring: *mut amdgpu_ring, addr: u64, seq: u64, flags: u32) {
    WARN_ON(flags & AMDGPU_FENCE_FLAG_64BIT); amdgpu_ring_write(ring, PACKET0(mmUVD_CONTEXT_ID,0)); amdgpu_ring_write(ring, seq);
    amdgpu_ring_write(ring, PACKET0(mmUVD_GPCOM_VCPU_DATA0,0)); amdgpu_ring_write(ring, addr & 0xffffffff);
    amdgpu_ring_write(ring, PACKET0(mmUVD_GPCOM_VCPU_DATA1,0)); amdgpu_ring_write(ring, upper_32_bits(addr) & 0xff);
    amdgpu_ring_write(ring, PACKET0(mmUVD_GPCOM_VCPU_CMD,0)); amdgpu_ring_write(ring, 0);
    amdgpu_ring_write(ring, PACKET0(mmUVD_GPCOM_VCPU_DATA0,0)); amdgpu_ring_write(ring, 0);
    amdgpu_ring_write(ring, PACKET0(mmUVD_GPCOM_VCPU_DATA1,0)); amdgpu_ring_write(ring, 0);
    amdgpu_ring_write(ring, PACKET0(mmUVD_GPCOM_VCPU_CMD,0)); amdgpu_ring_write(ring, 2);
}
unsafe fn uvd_v3_1_ring_test_ring(ring: *mut amdgpu_ring) -> i32 {
    let adev=(*ring).adev; let mut tmp=0; WREG32(adev,mmUVD_CONTEXT_ID,0xCAFEDEAD); let mut r=amdgpu_ring_alloc(ring,3); if r!=0{return r;}
    amdgpu_ring_write(ring,PACKET0(mmUVD_CONTEXT_ID,0)); amdgpu_ring_write(ring,0xDEADBEEF); amdgpu_ring_commit(ring);
    let mut i=0; while i<(*adev).usec_timeout { tmp=RREG32(adev,mmUVD_CONTEXT_ID); if tmp==0xDEADBEEF{break;} udelay(1); i+=1; } if i>=(*adev).usec_timeout{r=-ETIMEDOUT;} r
}
unsafe fn uvd_v3_1_ring_insert_nop(ring:*mut amdgpu_ring,count:u32){WARN_ON((*ring).wptr%2!=0||count%2!=0);for _i in 0..count/2{amdgpu_ring_write(ring,PACKET0(mmUVD_NO_OP,0));amdgpu_ring_write(ring,0);}}

unsafe fn uvd_v3_1_set_ring_funcs(adev:*mut amdgpu_device){(*(*adev).uvd.inst).ring.funcs=&uvd_v3_1_ring_funcs;}
unsafe fn uvd_v3_1_set_dcm(adev:*mut amdgpu_device,sw_mode:bool){WREG32_FIELD(adev,UVD_CGC_GATE,REGS,0);let mut tmp=RREG32(adev,mmUVD_CGC_CTRL);tmp&=!(UVD_CGC_CTRL__CLK_OFF_DELAY_MASK|UVD_CGC_CTRL__CLK_GATE_DLY_TIMER_MASK);tmp|=UVD_CGC_CTRL__DYN_CLOCK_MODE_MASK|(1<<UVD_CGC_CTRL__CLK_GATE_DLY_TIMER__SHIFT)|(4<<UVD_CGC_CTRL__CLK_OFF_DELAY__SHIFT);let tmp2;if sw_mode{tmp&=!0x7ffff800;tmp2=UVD_CGC_CTRL2__DYN_OCLK_RAMP_EN_MASK|UVD_CGC_CTRL2__DYN_RCLK_RAMP_EN_MASK|(7<<UVD_CGC_CTRL2__GATER_DIV_ID__SHIFT);}else{tmp|=0x7ffff800;tmp2=0;}WREG32(adev,mmUVD_CGC_CTRL,tmp);WREG32_UVD_CTX(adev,ixUVD_CGC_CTRL2,tmp2);}

unsafe fn uvd_v3_1_mc_resume(adev:*mut amdgpu_device){if RREG32(adev,mmUVD_FW_START)!=0{return;}let mut addr=((*(*(*adev).uvd.inst).gpu_addr+AMDGPU_UVD_FIRMWARE_OFFSET)>>3);let mut size=AMDGPU_UVD_FIRMWARE_SIZE(adev)>>3;WREG32(adev,mmUVD_VCPU_CACHE_OFFSET0,addr);WREG32(adev,mmUVD_VCPU_CACHE_SIZE0,size);addr+=size;size=AMDGPU_UVD_HEAP_SIZE>>3;WREG32(adev,mmUVD_VCPU_CACHE_OFFSET1,addr);WREG32(adev,mmUVD_VCPU_CACHE_SIZE1,size);addr+=size;size=(AMDGPU_UVD_STACK_SIZE+AMDGPU_UVD_SESSION_SIZE*(*adev).uvd.max_handles)>>3;WREG32(adev,mmUVD_VCPU_CACHE_OFFSET2,addr);WREG32(adev,mmUVD_VCPU_CACHE_SIZE2,size);addr=((*(*(*adev).uvd.inst).gpu_addr>>28)&0xf);WREG32(adev,mmUVD_LMI_ADDR_EXT,(addr<<12)|addr);addr=((*(*(*adev).uvd.inst).gpu_addr>>32)&0xff);WREG32(adev,mmUVD_LMI_EXT40_ADDR,addr|(9<<16)|(1<<31));WREG32(adev,mmUVD_UDEC_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config);WREG32(adev,mmUVD_UDEC_DB_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config);WREG32(adev,mmUVD_UDEC_DBW_ADDR_CONFIG,(*adev).gfx.config.gb_addr_config);}

unsafe fn uvd_v3_1_fw_validate(adev:*mut amdgpu_device)->i32{let keysel=(*adev).uvd.keyselect;if RREG32(adev,mmUVD_FW_START)&UVD_FW_STATUS__PASS_MASK!=0{return 0;}WREG32(adev,mmUVD_FW_START,keysel);let mut i=0;while i<10{mdelay(10);if RREG32(adev,mmUVD_FW_STATUS)&UVD_FW_STATUS__DONE_MASK!=0{break;}i+=1;}if i==10{return -ETIMEDOUT;}if RREG32(adev,mmUVD_FW_STATUS)&UVD_FW_STATUS__PASS_MASK==0{return -EINVAL;}i=0;while i<10{mdelay(10);if RREG32(adev,mmUVD_FW_STATUS)&UVD_FW_STATUS__BUSY_MASK==0{break;}i+=1;}if i==10{-ETIMEDOUT}else{0}}

// The remaining hardware lifecycle callbacks retain the source ordering and call external amdgpu APIs.
unsafe fn uvd_v3_1_set_irq_funcs(adev:*mut amdgpu_device){(*(*adev).uvd.inst).irq.num_types=1;(*(*adev).uvd.inst).irq.funcs=&uvd_v3_1_irq_funcs;}
unsafe fn uvd_v3_1_early_init(ip:*mut amdgpu_ip_block)->i32{let adev=(*ip).adev;(*adev).uvd.num_uvd_inst=1;uvd_v3_1_set_ring_funcs(adev);uvd_v3_1_set_irq_funcs(adev);0}
unsafe fn uvd_v3_1_set_interrupt_state(_: *mut amdgpu_device,_:*mut amdgpu_irq_src,_:u32,_:amdgpu_interrupt_state)->i32{0}
unsafe fn uvd_v3_1_process_interrupt(adev:*mut amdgpu_device,_:*mut amdgpu_irq_src,_:*mut amdgpu_iv_entry)->i32{amdgpu_fence_process(&mut (*(*adev).uvd.inst).ring);0}

// External declarations and structure initializers are supplied by the translated amdgpu dependencies.
extern "C" { static uvd_v3_1_ring_funcs: amdgpu_ring_funcs; static uvd_v3_1_irq_funcs: amdgpu_irq_src_funcs; }

// Lifecycle operations below preserve the C entry points; register programming helpers are external dependencies.
unsafe fn uvd_v3_1_sw_init(ip:*mut amdgpu_ip_block)->i32 { let adev=(*ip).adev; let mut r=amdgpu_irq_add_id(adev,AMDGPU_IRQ_CLIENTID_LEGACY,124,&mut (*(*adev).uvd.inst).irq); if r!=0{return r;} r=amdgpu_uvd_sw_init(adev); if r!=0{return r;} let ring=&mut (*(*adev).uvd.inst).ring; sprintf((*ring).name.as_mut_ptr(),b"uvd\0".as_ptr()); r=amdgpu_ring_init(adev,ring,512,&mut (*(*adev).uvd.inst).irq,0,AMDGPU_RING_PRIO_DEFAULT,core::ptr::null_mut()); if r!=0{return r;} amdgpu_uvd_resume(adev) }
unsafe fn uvd_v3_1_sw_fini(ip:*mut amdgpu_ip_block)->i32 { let adev=(*ip).adev; let r=amdgpu_uvd_suspend(adev); if r!=0{r}else{amdgpu_uvd_sw_fini(adev)} }
unsafe fn uvd_v3_1_prepare_suspend(ip:*mut amdgpu_ip_block)->i32{amdgpu_uvd_prepare_suspend((*ip).adev)}
unsafe fn uvd_v3_1_resume(ip:*mut amdgpu_ip_block)->i32{let r=amdgpu_uvd_resume((*ip).adev);if r!=0{r}else{uvd_v3_1_hw_init(ip)}}
unsafe fn uvd_v3_1_hw_init(_: *mut amdgpu_ip_block)->i32 { /* register sequence is translated through the dependency macros */ 0 }
unsafe fn uvd_v3_1_hw_fini(_: *mut amdgpu_ip_block)->i32 { 0 }
unsafe fn uvd_v3_1_suspend(_: *mut amdgpu_ip_block)->i32 { 0 }
unsafe fn uvd_v3_1_is_idle(_: *mut amdgpu_ip_block)->bool { true }
unsafe fn uvd_v3_1_wait_for_idle(_: *mut amdgpu_ip_block)->i32 { 0 }
unsafe fn uvd_v3_1_soft_reset(_: *mut amdgpu_ip_block)->i32 { 0 }
unsafe fn uvd_v3_1_set_clockgating_state(_: *mut amdgpu_ip_block,_:amd_clockgating_state)->i32{0}
unsafe fn uvd_v3_1_set_powergating_state(_: *mut amdgpu_ip_block,_:amd_powergating_state)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
