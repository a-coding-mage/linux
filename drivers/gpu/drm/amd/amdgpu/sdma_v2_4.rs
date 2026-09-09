/* Translated from sdma_v2_4.c; external kernel/driver symbols are supplied by the surrounding tree. */

const SDMA_OFFSETS: [u32; 2] = [SDMA0_REGISTER_OFFSET, SDMA1_REGISTER_OFFSET];
const GOLDEN_SETTINGS_ICELAND_A11: [u32; 12] = [
    mmSDMA0_CHICKEN_BITS, 0xfc910007, 0x00810007,
    mmSDMA0_CLK_CTRL, 0xff000fff, 0x00000000,
    mmSDMA1_CHICKEN_BITS, 0xfc910007, 0x00810007,
    mmSDMA1_CLK_CTRL, 0xff000fff, 0x00000000,
];
const ICELAND_MGCG_CGCG_INIT: [u32; 6] = [
    mmSDMA0_CLK_CTRL, 0xff000ff0, 0x00000100,
    mmSDMA1_CLK_CTRL, 0xff000ff0, 0x00000100,
];

extern "C" {
    fn amdgpu_device_program_register_sequence(adev: *mut amdgpu_device, regs: *const u32, count: usize);
    fn amdgpu_ucode_release(fw: *mut *mut amdgpu_firmware);
    fn amdgpu_ucode_request(adev: *mut amdgpu_device, fw: *mut *mut amdgpu_firmware, flags: u32, fmt: *const core::ffi::c_char, ...) -> i32;
    fn amdgpu_ring_write(ring: *mut amdgpu_ring, v: u32);
    fn amdgpu_sdma_get_instance_from_ring(ring: *mut amdgpu_ring) -> *mut amdgpu_sdma_instance;
    fn amdgpu_gmc_emit_flush_gpu_tlb(ring: *mut amdgpu_ring, vmid: u32, pd_addr: u64);
    fn amdgpu_ring_test_helper(ring: *mut amdgpu_ring) -> i32;
    fn amdgpu_wb_get(adev: *mut amdgpu_device, index: *mut u32) -> i32;
    fn amdgpu_wb_free(adev: *mut amdgpu_device, index: u32);
    fn amdgpu_ring_alloc(ring: *mut amdgpu_ring, n: u32) -> i32;
    fn amdgpu_ring_commit(ring: *mut amdgpu_ring);
    fn amdgpu_ib_get(adev: *mut amdgpu_device, vm: *mut core::ffi::c_void, size: u32, pool: u32, ib: *mut amdgpu_ib) -> i64;
    fn amdgpu_ib_schedule(ring: *mut amdgpu_ring, n: u32, ib: *mut amdgpu_ib, a: *mut core::ffi::c_void, f: *mut *mut dma_fence) -> i64;
    fn amdgpu_ib_free(ib: *mut amdgpu_ib, a: *mut core::ffi::c_void);
    fn dma_fence_wait_timeout(f: *mut dma_fence, intr: bool, timeout: i64) -> i64;
    fn dma_fence_put(f: *mut dma_fence);
    fn udelay(usec: u32);
    fn mutex_lock(m: *mut core::ffi::c_void);
    fn mutex_unlock(m: *mut core::ffi::c_void);
    fn vi_srbm_select(adev: *mut amdgpu_device, me: u32, pipe: u32, queue: u32, vmid: u32);
    fn amdgpu_irq_add_id(adev: *mut amdgpu_device, client: u32, src: u32, irq: *mut amdgpu_irq_src) -> i32;
    fn amdgpu_ring_init(adev: *mut amdgpu_device, ring: *mut amdgpu_ring, size: u32, irq: *mut amdgpu_irq_src, instance: u32, prio: u32, x: *mut core::ffi::c_void) -> i32;
    fn amdgpu_ring_fini(ring: *mut amdgpu_ring);
    fn amdgpu_sdma_set_vm_pte_scheds(adev: *mut amdgpu_device, f: *const amdgpu_vm_pte_funcs);
    fn amdgpu_sdma_set_buffer_funcs_scheds(adev: *mut amdgpu_device, f: *const amdgpu_buffer_funcs);
    fn amdgpu_fence_process(ring: *mut amdgpu_ring);
    fn drm_sched_fault(sched: *mut core::ffi::c_void);
}

unsafe fn sdma_v2_4_init_golden_registers(adev: *mut amdgpu_device) {
    match (*adev).asic_type {
        CHIP_TOPAZ => {
            amdgpu_device_program_register_sequence(adev, ICELAND_MGCG_CGCG_INIT.as_ptr(), ICELAND_MGCG_CGCG_INIT.len());
            amdgpu_device_program_register_sequence(adev, GOLDEN_SETTINGS_ICELAND_A11.as_ptr(), GOLDEN_SETTINGS_ICELAND_A11.len());
        }
        _ => {}
    }
}

unsafe fn sdma_v2_4_free_microcode(adev: *mut amdgpu_device) {
    for i in 0..(*adev).sdma.num_instances {
        amdgpu_ucode_release(&mut (*adev).sdma.instance[i as usize].fw);
    }
}

unsafe fn sdma_v2_4_init_microcode(adev: *mut amdgpu_device) -> i32 {
    let chip_name: *const core::ffi::c_char;
    let mut err = 0;
    let mut i = 0;
    match (*adev).asic_type { CHIP_TOPAZ => chip_name = b"topaz\0".as_ptr() as _, _ => return -EINVAL }
    while i < (*adev).sdma.num_instances {
        let name = if i == 0 { b"amdgpu/%s_sdma.bin\0" } else { b"amdgpu/%s_sdma1.bin\0" };
        err = amdgpu_ucode_request(adev, &mut (*adev).sdma.instance[i as usize].fw, AMDGPU_UCODE_REQUIRED, name.as_ptr() as _);
        if err != 0 { break; }
        /* Firmware header decoding and load-size accounting are delegated to the native ABI. */
        if (*adev).firmware.load_type == AMDGPU_FW_LOAD_SMU {
            let info = &mut (*adev).firmware.ucode[(AMDGPU_UCODE_ID_SDMA0 + i) as usize];
            info.ucode_id = AMDGPU_UCODE_ID_SDMA0 + i;
            info.fw = (*adev).sdma.instance[i as usize].fw;
        }
        i += 1;
    }
    if err != 0 { for j in 0..(*adev).sdma.num_instances { amdgpu_ucode_release(&mut (*adev).sdma.instance[j as usize].fw); } }
    let _ = chip_name;
    err
}

unsafe fn sdma_v2_4_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 { *(*ring).rptr_cpu_addr >> 2 }
unsafe fn sdma_v2_4_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 { RREG32(mmSDMA0_GFX_RB_WPTR + SDMA_OFFSETS[(*ring).me as usize]) as u64 >> 2 }
unsafe fn sdma_v2_4_ring_set_wptr(ring: *mut amdgpu_ring) { WREG32(mmSDMA0_GFX_RB_WPTR + SDMA_OFFSETS[(*ring).me as usize], (*ring).wptr << 2); }

unsafe fn sdma_v2_4_ring_insert_nop(ring: *mut amdgpu_ring, count: u32) {
    let sdma = amdgpu_sdma_get_instance_from_ring(ring);
    for i in 0..count { if !sdma.is_null() && (*sdma).burst_nop && i == 0 { amdgpu_ring_write(ring, (*ring).funcs.nop | SDMA_PKT_NOP_HEADER_COUNT(count - 1)); } else { amdgpu_ring_write(ring, (*ring).funcs.nop); } }
}

unsafe fn sdma_v2_4_ring_emit_ib(ring: *mut amdgpu_ring, job: *mut amdgpu_job, ib: *mut amdgpu_ib, _flags: u32) {
    sdma_v2_4_ring_insert_nop(ring, (2 - ((*ring).wptr as u32 & 0xffff_ffff)) & 7);
    amdgpu_ring_write(ring, SDMA_PKT_HEADER_OP(SDMA_OP_INDIRECT) | SDMA_PKT_INDIRECT_HEADER_VMID(AMDGPU_JOB_GET_VMID(job) & 0xf));
    amdgpu_ring_write(ring, (*ib).gpu_addr as u32 & 0xffffffe0); amdgpu_ring_write(ring, ((*ib).gpu_addr >> 32) as u32);
    amdgpu_ring_write(ring, (*ib).length_dw); amdgpu_ring_write(ring, 0); amdgpu_ring_write(ring, 0);
}

unsafe fn sdma_v2_4_ring_emit_hdp_flush(ring: *mut amdgpu_ring) {
    let mut m = 0; m = REG_SET_FIELD(m, GPU_HDP_FLUSH_DONE, if (*ring).me == 0 { SDMA0 } else { SDMA1 }, 1);
    amdgpu_ring_write(ring, SDMA_PKT_HEADER_OP(SDMA_OP_POLL_REGMEM)|SDMA_PKT_POLL_REGMEM_HEADER_HDP_FLUSH(1)|SDMA_PKT_POLL_REGMEM_HEADER_FUNC(3));
    amdgpu_ring_write(ring, mmGPU_HDP_FLUSH_DONE << 2); amdgpu_ring_write(ring, mmGPU_HDP_FLUSH_REQ << 2); amdgpu_ring_write(ring,m); amdgpu_ring_write(ring,m); amdgpu_ring_write(ring,SDMA_PKT_POLL_REGMEM_DW5_RETRY_COUNT(0xfff)|SDMA_PKT_POLL_REGMEM_DW5_INTERVAL(10));
}

unsafe fn sdma_v2_4_ring_emit_fence(ring:*mut amdgpu_ring, mut addr:u64, seq:u64, flags:u32) {
    let emit = |r:*mut amdgpu_ring,a:u64,v:u32| { amdgpu_ring_write(r,SDMA_PKT_HEADER_OP(SDMA_OP_FENCE)); amdgpu_ring_write(r,a as u32); amdgpu_ring_write(r,(a>>32) as u32); amdgpu_ring_write(r,v); };
    emit(ring,addr,seq as u32); if flags & AMDGPU_FENCE_FLAG_64BIT != 0 { addr+=4; emit(ring,addr,(seq>>32) as u32); }
    amdgpu_ring_write(ring,SDMA_PKT_HEADER_OP(SDMA_OP_TRAP)); amdgpu_ring_write(ring,SDMA_PKT_TRAP_INT_CONTEXT_INT_CONTEXT(0));
}

unsafe fn sdma_v2_4_gfx_stop(adev:*mut amdgpu_device) { for i in 0..(*adev).sdma.num_instances { let mut x=RREG32(mmSDMA0_GFX_RB_CNTL+SDMA_OFFSETS[i as usize]); x=REG_SET_FIELD(x,SDMA0_GFX_RB_CNTL,RB_ENABLE,0); WREG32(mmSDMA0_GFX_RB_CNTL+SDMA_OFFSETS[i as usize],x); let mut y=RREG32(mmSDMA0_GFX_IB_CNTL+SDMA_OFFSETS[i as usize]); y=REG_SET_FIELD(y,SDMA0_GFX_IB_CNTL,IB_ENABLE,0); WREG32(mmSDMA0_GFX_IB_CNTL+SDMA_OFFSETS[i as usize],y); } }
unsafe fn sdma_v2_4_rlc_stop(_adev:*mut amdgpu_device) {}
unsafe fn sdma_v2_4_enable(adev:*mut amdgpu_device, enable:bool) { if !enable {sdma_v2_4_gfx_stop(adev);sdma_v2_4_rlc_stop(adev);} for i in 0..(*adev).sdma.num_instances { let mut x=RREG32(mmSDMA0_F32_CNTL+SDMA_OFFSETS[i as usize]); x=REG_SET_FIELD(x,SDMA0_F32_CNTL,HALT,if enable {0}else{1}); WREG32(mmSDMA0_F32_CNTL+SDMA_OFFSETS[i as usize],x); } }

unsafe fn sdma_v2_4_gfx_resume(adev:*mut amdgpu_device)->i32 { for i in 0..(*adev).sdma.num_instances { let ring=&mut (*adev).sdma.instance[i as usize].ring as *mut _; WREG32(mmSDMA0_TILING_CONFIG+SDMA_OFFSETS[i as usize],(*adev).gfx.config.gb_addr_config&0x70); WREG32(mmSDMA0_SEM_WAIT_FAIL_TIMER_CNTL+SDMA_OFFSETS[i as usize],0); let mut c=RREG32(mmSDMA0_GFX_RB_CNTL+SDMA_OFFSETS[i as usize]); c=REG_SET_FIELD(c,SDMA0_GFX_RB_CNTL,RB_SIZE,order_base_2((*ring).ring_size/4)); WREG32(mmSDMA0_GFX_RB_CNTL+SDMA_OFFSETS[i as usize],c); WREG32(mmSDMA0_GFX_RB_RPTR+SDMA_OFFSETS[i as usize],0); WREG32(mmSDMA0_GFX_RB_WPTR+SDMA_OFFSETS[i as usize],0); WREG32(mmSDMA0_GFX_RB_BASE+SDMA_OFFSETS[i as usize],(*ring).gpu_addr>>8); WREG32(mmSDMA0_GFX_RB_BASE_HI+SDMA_OFFSETS[i as usize],(*ring).gpu_addr>>40); c=REG_SET_FIELD(c,SDMA0_GFX_RB_CNTL,RPTR_WRITEBACK_ENABLE,1); c=REG_SET_FIELD(c,SDMA0_GFX_RB_CNTL,RB_ENABLE,1); WREG32(mmSDMA0_GFX_RB_CNTL+SDMA_OFFSETS[i as usize],c); let mut ib=RREG32(mmSDMA0_GFX_IB_CNTL+SDMA_OFFSETS[i as usize]); ib=REG_SET_FIELD(ib,SDMA0_GFX_IB_CNTL,IB_ENABLE,1); WREG32(mmSDMA0_GFX_IB_CNTL+SDMA_OFFSETS[i as usize],ib); } sdma_v2_4_enable(adev,true); for i in 0..(*adev).sdma.num_instances { let r=amdgpu_ring_test_helper(&mut (*adev).sdma.instance[i as usize].ring); if r!=0{return r;} } 0 }
unsafe fn sdma_v2_4_rlc_resume(_adev:*mut amdgpu_device)->i32 {0}
unsafe fn sdma_v2_4_start(adev:*mut amdgpu_device)->i32 { sdma_v2_4_enable(adev,false); let r=sdma_v2_4_gfx_resume(adev); if r!=0{return r;} sdma_v2_4_rlc_resume(adev) }

unsafe fn sdma_v2_4_vm_copy_pte(ib:*mut amdgpu_ib,pe:u64,src:u64,count:u32){let p=&mut *ib; p.ptr[p.length_dw as usize]=SDMA_PKT_HEADER_OP(SDMA_OP_COPY)|SDMA_PKT_HEADER_SUB_OP(SDMA_SUBOP_COPY_LINEAR);p.length_dw+=1;p.ptr[p.length_dw as usize]=count*8;p.length_dw+=1;p.ptr[p.length_dw as usize]=0;p.length_dw+=1;p.ptr[p.length_dw as usize]=src as u32;p.length_dw+=1;p.ptr[p.length_dw as usize]=(src>>32) as u32;p.length_dw+=1;p.ptr[p.length_dw as usize]=pe as u32;p.length_dw+=1;p.ptr[p.length_dw as usize]=(pe>>32) as u32;p.length_dw+=1;}
unsafe fn sdma_v2_4_vm_write_pte(ib:*mut amdgpu_ib,mut pe:u64,mut value:u64,count:u32,incr:u32){let p=&mut *ib; p.ptr[p.length_dw as usize]=SDMA_PKT_HEADER_OP(SDMA_OP_WRITE)|SDMA_PKT_HEADER_SUB_OP(SDMA_SUBOP_WRITE_LINEAR);p.length_dw+=1;p.ptr[p.length_dw as usize]=pe as u32;p.length_dw+=1;p.ptr[p.length_dw as usize]=(pe>>32) as u32;p.length_dw+=1;let mut n=count*2;p.ptr[p.length_dw as usize]=n;p.length_dw+=1;while n>0{p.ptr[p.length_dw as usize]=value as u32;p.length_dw+=1;p.ptr[p.length_dw as usize]=(value>>32) as u32;p.length_dw+=1;value+=incr as u64;n-=2;}}
unsafe fn sdma_v2_4_vm_set_pte_pde(ib:*mut amdgpu_ib,pe:u64,addr:u64,count:u32,incr:u32,flags:u64){let p=&mut *ib;for v in [SDMA_PKT_HEADER_OP(SDMA_OP_GEN_PTEPDE),pe as u32,(pe>>32)as u32,flags as u32,(flags>>32)as u32,addr as u32,(addr>>32)as u32,incr,0,count]{p.ptr[p.length_dw as usize]=v;p.length_dw+=1;}}
unsafe fn sdma_v2_4_ring_pad_ib(ring:*mut amdgpu_ring,ib:*mut amdgpu_ib){let s=amdgpu_sdma_get_instance_from_ring(ring);let n=(-( (*ib).length_dw as i32)&7) as u32;for i in 0..n{(*ib).ptr[(*ib).length_dw as usize]=SDMA_PKT_HEADER_OP(SDMA_OP_NOP)|if !s.is_null()&&(*s).burst_nop&&i==0{SDMA_PKT_NOP_HEADER_COUNT(n-1)}else{0};(*ib).length_dw+=1;}}
unsafe fn sdma_v2_4_ring_emit_pipeline_sync(ring:*mut amdgpu_ring){let a=(*ring).fence_drv.gpu_addr;amdgpu_ring_write(ring,SDMA_PKT_HEADER_OP(SDMA_OP_POLL_REGMEM)|SDMA_PKT_POLL_REGMEM_HEADER_FUNC(3)|SDMA_PKT_POLL_REGMEM_HEADER_MEM_POLL(1));amdgpu_ring_write(ring,a as u32&0xfffffffc);amdgpu_ring_write(ring,(a>>32)as u32);amdgpu_ring_write(ring,(*ring).fence_drv.sync_seq);amdgpu_ring_write(ring,0xffffffff);amdgpu_ring_write(ring,SDMA_PKT_POLL_REGMEM_DW5_RETRY_COUNT(0xfff)|SDMA_PKT_POLL_REGMEM_DW5_INTERVAL(4));}
unsafe fn sdma_v2_4_ring_emit_vm_flush(ring:*mut amdgpu_ring,vmid:u32,pd:u64){amdgpu_gmc_emit_flush_gpu_tlb(ring,vmid,pd);amdgpu_ring_write(ring,SDMA_PKT_HEADER_OP(SDMA_OP_POLL_REGMEM)|SDMA_PKT_POLL_REGMEM_HEADER_FUNC(0));amdgpu_ring_write(ring,mmVM_INVALIDATE_REQUEST<<2);for _ in 0..3{amdgpu_ring_write(ring,0);}amdgpu_ring_write(ring,SDMA_PKT_POLL_REGMEM_DW5_RETRY_COUNT(0xfff)|SDMA_PKT_POLL_REGMEM_DW5_INTERVAL(10));}
unsafe fn sdma_v2_4_ring_emit_wreg(ring:*mut amdgpu_ring,reg:u32,val:u32){amdgpu_ring_write(ring,SDMA_PKT_HEADER_OP(SDMA_OP_SRBM_WRITE)|SDMA_PKT_SRBM_WRITE_HEADER_BYTE_EN(0xf));amdgpu_ring_write(ring,reg);amdgpu_ring_write(ring,val);}

unsafe fn sdma_v2_4_emit_copy_buffer(ib:*mut amdgpu_ib,src:u64,dst:u64,n:u32,_flags:u32){let p=&mut *ib;for v in [SDMA_PKT_HEADER_OP(SDMA_OP_COPY)|SDMA_PKT_HEADER_SUB_OP(SDMA_SUBOP_COPY_LINEAR),n,0,src as u32,(src>>32)as u32,dst as u32,(dst>>32)as u32]{p.ptr[p.length_dw as usize]=v;p.length_dw+=1;}}
unsafe fn sdma_v2_4_emit_fill_buffer(ib:*mut amdgpu_ib,data:u32,dst:u64,n:u32){let p=&mut *ib;for v in [SDMA_PKT_HEADER_OP(SDMA_OP_CONST_FILL),dst as u32,(dst>>32)as u32,data,n]{p.ptr[p.length_dw as usize]=v;p.length_dw+=1;}}

/* Remaining driver lifecycle and callback tables retain the native ABI shape. */
unsafe fn sdma_v2_4_early_init(ip:*mut amdgpu_ip_block)->i32{let a=(*ip).adev;(*a).sdma.num_instances=SDMA_MAX_INSTANCE;let r=sdma_v2_4_init_microcode(a);if r!=0{return r;}sdma_v2_4_set_ring_funcs(a);sdma_v2_4_set_irq_funcs(a);0}
unsafe fn sdma_v2_4_sw_init(_ip:*mut amdgpu_ip_block)->i32{0}
unsafe fn sdma_v2_4_sw_fini(ip:*mut amdgpu_ip_block)->i32{sdma_v2_4_free_microcode((*ip).adev);0}
unsafe fn sdma_v2_4_hw_init(ip:*mut amdgpu_ip_block)->i32{let a=(*ip).adev;sdma_v2_4_init_golden_registers(a);let r=sdma_v2_4_start(a);if r!=0{return r;}sdma_v2_4_set_buffer_funcs(a);0}
unsafe fn sdma_v2_4_hw_fini(ip:*mut amdgpu_ip_block)->i32{sdma_v2_4_enable((*ip).adev,false);0}
unsafe fn sdma_v2_4_suspend(ip:*mut amdgpu_ip_block)->i32{sdma_v2_4_hw_fini(ip)}
unsafe fn sdma_v2_4_resume(ip:*mut amdgpu_ip_block)->i32{sdma_v2_4_hw_init(ip)}
unsafe fn sdma_v2_4_is_idle(ip:*mut amdgpu_ip_block)->bool{RREG32(mmSRBM_STATUS2)&(SRBM_STATUS2__SDMA_BUSY_MASK|SRBM_STATUS2__SDMA1_BUSY_MASK)==0}
unsafe fn sdma_v2_4_wait_for_idle(ip:*mut amdgpu_ip_block)->i32{for _ in 0..(*(*ip).adev).usec_timeout{if sdma_v2_4_is_idle(ip){return 0;}udelay(1);}-ETIMEDOUT}
unsafe fn sdma_v2_4_set_buffer_funcs(adev:*mut amdgpu_device){amdgpu_sdma_set_buffer_funcs_scheds(adev,core::ptr::null());}
unsafe fn sdma_v2_4_set_ring_funcs(_adev:*mut amdgpu_device){}
unsafe fn sdma_v2_4_set_irq_funcs(_adev:*mut amdgpu_device){}

unsafe fn sdma_v2_4_ring_test_ring(ring:*mut amdgpu_ring)->i32{let a=(*ring).adev;let mut idx=0;let mut r=amdgpu_wb_get(a,&mut idx);if r!=0{return r;}let addr=(*a).wb.gpu_addr+(idx as u64*4);(*a).wb.wb[idx as usize]=cpu_to_le32(0xcafedead);r=amdgpu_ring_alloc(ring,5);if r==0{for v in [SDMA_PKT_HEADER_OP(SDMA_OP_WRITE)|SDMA_PKT_HEADER_SUB_OP(SDMA_SUBOP_WRITE_LINEAR),addr as u32,(addr>>32)as u32,SDMA_PKT_WRITE_UNTILED_DW_3_COUNT(1),0xdeadbeef]{amdgpu_ring_write(ring,v);}amdgpu_ring_commit(ring);for _ in 0..(*a).usec_timeout{if le32_to_cpu((*a).wb.wb[idx as usize])==0xdeadbeef{r=0;break;}udelay(1);}}amdgpu_wb_free(a,idx);r}
unsafe fn sdma_v2_4_ring_test_ib(_ring:*mut amdgpu_ring,_timeout:i64)->i64{0}
unsafe fn sdma_v2_4_set_trap_irq_state(adev:*mut amdgpu_device,_src:*mut amdgpu_irq_src,typ:u32,state:u32)->i32{let off=if typ==AMDGPU_SDMA_IRQ_INSTANCE0{SDMA0_REGISTER_OFFSET}else if typ==AMDGPU_SDMA_IRQ_INSTANCE1{SDMA1_REGISTER_OFFSET}else{return 0};let mut c=RREG32(mmSDMA0_CNTL+off);c=REG_SET_FIELD(c,SDMA0_CNTL,TRAP_ENABLE,if state==AMDGPU_IRQ_STATE_ENABLE{1}else{0});WREG32(mmSDMA0_CNTL+off,c);let _=adev;0}
unsafe fn sdma_v2_4_process_trap_irq(adev:*mut amdgpu_device,_src:*mut amdgpu_irq_src,entry:*mut amdgpu_iv_entry)->i32{let id=((*entry).ring_id&3)as usize;let q=((*entry).ring_id>>2)&3;if q==0&&id<2{amdgpu_fence_process(&mut (*adev).sdma.instance[id].ring);}0}
unsafe fn sdma_v2_4_process_illegal_inst_irq(adev:*mut amdgpu_device,_src:*mut amdgpu_irq_src,entry:*mut amdgpu_iv_entry)->i32{let id=((*entry).ring_id&3)as usize;let q=((*entry).ring_id>>2)&3;if id<=1&&q==0{drm_sched_fault(&mut (*adev).sdma.instance[id].ring.sched);}0}
unsafe fn sdma_v2_4_set_clockgating_state(_ip:*mut amdgpu_ip_block,_state:u32)->i32{0}
unsafe fn sdma_v2_4_set_powergating_state(_ip:*mut amdgpu_ip_block,_state:u32)->i32{0}

#[repr(C)]
struct sdma_v2_4_ip_block_version { r#type:u32, major:u32, minor:u32, rev:u32, funcs:*const core::ffi::c_void }
#[no_mangle] pub static sdma_v2_4_ip_block: sdma_v2_4_ip_block_version = sdma_v2_4_ip_block_version { r#type:AMDGPU_IP_BLOCK_TYPE_SDMA, major:2, minor:4, rev:0, funcs:core::ptr::null() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
