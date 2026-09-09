// SPDX-License-Identifier: MIT
// Copyright 2023 Advanced Micro Devices, Inc.
//
// Translated from amdgpu_umsch_mm.c. Kernel and driver symbols are supplied
// by the surrounding translation unit.

pub unsafe fn amdgpu_umsch_mm_submit_pkt(umsch: *mut amdgpu_umsch_mm, pkt: *mut core::ffi::c_void, ndws: i32) -> i32 {
    let ring = &mut (*umsch).ring;
    if amdgpu_ring_alloc(ring, ndws) != 0 { return -ENOMEM; }
    amdgpu_ring_write_multiple(ring, pkt, ndws);
    amdgpu_ring_commit(ring);
    0
}

pub unsafe fn amdgpu_umsch_mm_query_fence(umsch: *mut amdgpu_umsch_mm) -> i32 {
    let ring = &mut (*umsch).ring;
    let adev = ring.adev;
    let r = amdgpu_fence_wait_polling(ring, ring.fence_drv.sync_seq, (*adev).usec_timeout);
    if r < 1 { dev_err((*adev).dev, "ring umsch timeout, emitted fence %u\n", ring.fence_drv.sync_seq); return -ETIMEDOUT; }
    0
}

unsafe fn umsch_mm_ring_set_wptr(ring: *mut amdgpu_ring) {
    let umsch = ring as *mut amdgpu_umsch_mm;
    if (*ring).use_doorbell { WDOORBELL32((*ring).doorbell_index, (*ring).wptr << 2); }
    else { WREG32((*umsch).rb_wptr, (*ring).wptr << 2); }
}
unsafe fn umsch_mm_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 { RREG32((ring as *mut amdgpu_umsch_mm).as_ref().unwrap().rb_rptr) as u64 }
unsafe fn umsch_mm_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 { RREG32((ring as *mut amdgpu_umsch_mm).as_ref().unwrap().rb_wptr) as u64 }

static umsch_v4_0_ring_funcs: amdgpu_ring_funcs = amdgpu_ring_funcs {
    type_: AMDGPU_RING_TYPE_UMSCH_MM, align_mask: 0, nop: 0, support_64bit_ptrs: false,
    get_rptr: Some(umsch_mm_ring_get_rptr), get_wptr: Some(umsch_mm_ring_get_wptr),
    set_wptr: Some(umsch_mm_ring_set_wptr), insert_nop: Some(amdgpu_ring_insert_nop),
};

pub unsafe fn amdgpu_umsch_mm_ring_init(umsch: *mut amdgpu_umsch_mm) -> i32 {
    let adev = container_of!(umsch, amdgpu_device, umsch_mm);
    let ring = &mut (*umsch).ring;
    ring.vm_hub = AMDGPU_MMHUB0(0); ring.use_doorbell = true; ring.no_scheduler = true;
    ring.doorbell_index = (AMDGPU_NAVI10_DOORBELL64_VCN0_1 << 1) + 6;
    snprintf(ring.name.as_mut_ptr(), core::mem::size_of_val(&ring.name), "umsch");
    amdgpu_ring_init(adev, ring, 1024, core::ptr::null_mut(), 0, AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut())
}

pub unsafe fn amdgpu_umsch_mm_init_microcode(umsch: *mut amdgpu_umsch_mm) -> i32 {
    let adev = (*umsch).ring.adev; let mut fw_name: *const u8 = core::ptr::null();
    match amdgpu_ip_version(adev, VCN_HWIP, 0) { IP_VERSION!(4,0,5) | IP_VERSION!(4,0,6) => fw_name = b"4_0_0\0".as_ptr(), _ => return -EINVAL }
    let r = amdgpu_ucode_request(adev, &mut (*adev).umsch_mm.fw, AMDGPU_UCODE_REQUIRED, b"amdgpu/umsch_mm_%s.bin\0".as_ptr(), fw_name);
    if r != 0 { release_firmware((*adev).umsch_mm.fw); (*adev).umsch_mm.fw = core::ptr::null_mut(); return r; }
    let h = (*adev).umsch_mm.fw.data as *const umsch_mm_firmware_header_v1_0;
    (*adev).umsch_mm.ucode_size = le32_to_cpu((*h).umsch_mm_ucode_size_bytes);
    (*adev).umsch_mm.data_size = le32_to_cpu((*h).umsch_mm_ucode_data_size_bytes);
    (*adev).umsch_mm.irq_start_addr = le32_to_cpu((*h).umsch_mm_irq_start_addr_lo) as u64 | ((le32_to_cpu((*h).umsch_mm_irq_start_addr_hi) as u64) << 32);
    (*adev).umsch_mm.uc_start_addr = le32_to_cpu((*h).umsch_mm_uc_start_addr_lo) as u64 | ((le32_to_cpu((*h).umsch_mm_uc_start_addr_hi) as u64) << 32);
    (*adev).umsch_mm.data_start_addr = le32_to_cpu((*h).umsch_mm_data_start_addr_lo) as u64 | ((le32_to_cpu((*h).umsch_mm_data_start_addr_hi) as u64) << 32);
    if (*adev).firmware.load_type == AMDGPU_FW_LOAD_PSP {
        let i = &mut (*adev).firmware.ucode[AMDGPU_UCODE_ID_UMSCH_MM_UCODE]; i.ucode_id = AMDGPU_UCODE_ID_UMSCH_MM_UCODE; i.fw = (*adev).umsch_mm.fw;
        (*adev).firmware.fw_size += ALIGN!(le32_to_cpu((*h).umsch_mm_ucode_size_bytes), PAGE_SIZE);
        let i = &mut (*adev).firmware.ucode[AMDGPU_UCODE_ID_UMSCH_MM_DATA]; i.ucode_id = AMDGPU_UCODE_ID_UMSCH_MM_DATA; i.fw = (*adev).umsch_mm.fw;
        (*adev).firmware.fw_size += ALIGN!(le32_to_cpu((*h).umsch_mm_ucode_data_size_bytes), PAGE_SIZE);
    } 0
}

unsafe fn allocate(umsch: *mut amdgpu_umsch_mm, data: bool) -> i32 {
    let adev = (*umsch).ring.adev; let h = (*adev).umsch_mm.fw.data as *const umsch_mm_firmware_header_v1_0;
    let (off, sz, align, obj, gpu, ptr) = if data { ((*h).umsch_mm_ucode_data_offset_bytes, (*h).umsch_mm_ucode_data_size_bytes, 64*1024, &mut (*adev).umsch_mm.data_fw_obj, &mut (*adev).umsch_mm.data_fw_gpu_addr, &mut (*adev).umsch_mm.data_fw_ptr) } else { ((*h).umsch_mm_ucode_offset_bytes, (*h).umsch_mm_ucode_size_bytes, 4*1024, &mut (*adev).umsch_mm.ucode_fw_obj, &mut (*adev).umsch_mm.ucode_fw_gpu_addr, &mut (*adev).umsch_mm.ucode_fw_ptr) };
    let size = le32_to_cpu(sz); let src = (*adev).umsch_mm.fw.data.add(le32_to_cpu(off) as usize) as *const core::ffi::c_void;
    let r = amdgpu_bo_create_reserved(adev, size, align, AMDGPU_GEM_DOMAIN_VRAM, obj, gpu, ptr); if r != 0 { return r; }
    memcpy(*ptr, src, size as usize); amdgpu_bo_kunmap(*obj); amdgpu_bo_unreserve(*obj); 0
}
pub unsafe fn amdgpu_umsch_mm_allocate_ucode_buffer(u: *mut amdgpu_umsch_mm)->i32 { allocate(u,false) }
pub unsafe fn amdgpu_umsch_mm_allocate_ucode_data_buffer(u: *mut amdgpu_umsch_mm)->i32 { allocate(u,true) }

pub unsafe fn amdgpu_umsch_mm_psp_execute_cmd_buf(u:*mut amdgpu_umsch_mm)->i32 { let a=(*u).ring.adev; let c=amdgpu_firmware_info{ucode_id:AMDGPU_UCODE_ID_UMSCH_MM_CMD_BUFFER,mc_addr:(*a).umsch_mm.cmd_buf_gpu_addr,ucode_size:(*a).umsch_mm.cmd_buf_curr_ptr as usize-(*a).umsch_mm.cmd_buf_ptr as usize}; psp_execute_ip_fw_load(&mut (*a).psp,&c) }

unsafe fn umsch_mm_agdb_index_init(a:*mut amdgpu_device){let mut s=(*a).doorbell_index.max_assignment+1;s=roundup(s,1024);s+=(AMDGPU_NAVI10_DOORBELL64_VCN0_1<<1);for i in 0..CONTEXT_PRIORITY_NUM_LEVELS{(*a).umsch_mm.agdb_index[i]=s+i;}}
unsafe fn umsch_mm_init(a:*mut amdgpu_device)->i32{(*a).umsch_mm.vmid_mask_mm_vpe=0xf00;(*a).umsch_mm.engine_mask=1<<UMSCH_SWIP_ENGINE_TYPE_VPE;(*a).umsch_mm.vpe_hqd_mask=0xfe;let r=amdgpu_wb_get(a,&mut (*a).umsch_mm.wb_index);if r!=0{return r;}(*a).umsch_mm.sch_ctx_gpu_addr=(*a).wb.gpu_addr+(*a).umsch_mm.wb_index*4;let r=amdgpu_bo_create_kernel(a,PAGE_SIZE,PAGE_SIZE,AMDGPU_GEM_DOMAIN_GTT,&mut (*a).umsch_mm.cmd_buf_obj,&mut (*a).umsch_mm.cmd_buf_gpu_addr,&mut (*a).umsch_mm.cmd_buf_ptr);if r!=0{amdgpu_wb_free(a,(*a).umsch_mm.wb_index);return r;}let r=amdgpu_bo_create_kernel(a,AMDGPU_UMSCHFW_LOG_SIZE,PAGE_SIZE,AMDGPU_GEM_DOMAIN_VRAM|AMDGPU_GEM_DOMAIN_GTT,&mut (*a).umsch_mm.dbglog_bo,&mut (*a).umsch_mm.log_gpu_addr,&mut (*a).umsch_mm.log_cpu_addr);if r!=0{return r;}mutex_init(&mut (*a).umsch_mm.mutex_hidden);umsch_mm_agdb_index_init(a);0}

unsafe fn umsch_mm_early_init(i:*mut amdgpu_ip_block)->i32{let a=(*i).adev;match amdgpu_ip_version(a,VCN_HWIP,0){IP_VERSION!(4,0,5)|IP_VERSION!(4,0,6)=>umsch_mm_v4_0_set_funcs(&mut (*a).umsch_mm),_=>return -EINVAL}(*a).umsch_mm.ring.funcs=&umsch_v4_0_ring_funcs;umsch_mm_set_regs(&mut (*a).umsch_mm);0}
unsafe fn umsch_mm_late_init(_:*mut amdgpu_ip_block)->i32{0}
unsafe fn umsch_mm_sw_init(i:*mut amdgpu_ip_block)->i32{let a=(*i).adev;let mut r=umsch_mm_init(a);if r!=0{return r;}amdgpu_umsch_fwlog_init(&mut (*a).umsch_mm);r=umsch_mm_ring_init(&mut (*a).umsch_mm);if r!=0{return r;}amdgpu_umsch_mm_init_microcode(&mut (*a).umsch_mm)}
unsafe fn umsch_mm_sw_fini(i:*mut amdgpu_ip_block)->i32{let a=(*i).adev;release_firmware((*a).umsch_mm.fw);(*a).umsch_mm.fw=core::ptr::null_mut();amdgpu_ring_fini(&mut (*a).umsch_mm.ring);mutex_destroy(&mut (*a).umsch_mm.mutex_hidden);amdgpu_bo_free_kernel(&mut (*a).umsch_mm.cmd_buf_obj,&mut (*a).umsch_mm.cmd_buf_gpu_addr,&mut (*a).umsch_mm.cmd_buf_ptr);amdgpu_bo_free_kernel(&mut (*a).umsch_mm.dbglog_bo,&mut (*a).umsch_mm.log_gpu_addr,&mut (*a).umsch_mm.log_cpu_addr);amdgpu_wb_free(a,(*a).umsch_mm.wb_index);0}
unsafe fn umsch_mm_hw_init(i:*mut amdgpu_ip_block)->i32{let a=(*i).adev;let r=umsch_mm_load_microcode(&mut (*a).umsch_mm);if r!=0{return r;}umsch_mm_ring_start(&mut (*a).umsch_mm);umsch_mm_set_hw_resources(&mut (*a).umsch_mm)}
unsafe fn umsch_mm_hw_fini(i:*mut amdgpu_ip_block)->i32{let a=(*i).adev;umsch_mm_ring_stop(&mut (*a).umsch_mm);amdgpu_bo_free_kernel(&mut (*a).umsch_mm.data_fw_obj,&mut (*a).umsch_mm.data_fw_gpu_addr,&mut (*a).umsch_mm.data_fw_ptr);amdgpu_bo_free_kernel(&mut (*a).umsch_mm.ucode_fw_obj,&mut (*a).umsch_mm.ucode_fw_gpu_addr,&mut (*a).umsch_mm.ucode_fw_ptr);0}
unsafe fn umsch_mm_suspend(i:*mut amdgpu_ip_block)->i32{umsch_mm_hw_fini(i)} unsafe fn umsch_mm_resume(i:*mut amdgpu_ip_block)->i32{umsch_mm_hw_init(i)}

pub unsafe fn amdgpu_umsch_fwlog_init(u:*mut amdgpu_umsch_mm){let l=(*u).log_cpu_addr as *mut amdgpu_umsch_fwlog;(*l).header_size=core::mem::size_of::<amdgpu_umsch_fwlog>();(*l).buffer_size=AMDGPU_UMSCHFW_LOG_SIZE;(*l).rptr=(*l).header_size;(*l).wptr=(*l).header_size;(*l).wrapped=0;}
// CONFIG_DEBUG_FS implementation: maps the firmware log as a wrapped buffer.
unsafe fn amdgpu_debugfs_umsch_fwlog_read(f:*mut file,buf:*mut u8,size:usize,pos:*mut loff_t)->isize{let u=(*file_inode(f)).i_private as *mut amdgpu_umsch_mm;if u.is_null(){return -ENODEV as isize;}if (*u).log_cpu_addr.is_null(){return -EFAULT as isize;}let l=(*u).log_cpu_addr as *mut amdgpu_umsch_fwlog;let mut rp=(*l).rptr;let wp=(*l).wptr;if rp>AMDGPU_UMSCHFW_LOG_SIZE||wp>AMDGPU_UMSCHFW_LOG_SIZE{return -EFAULT as isize;}if size==0||rp==wp{return 0;}let mut n=[0usize;2];if wp>rp{n[0]=core::cmp::min(size,wp-rp);}else{n[0]=AMDGPU_UMSCHFW_LOG_SIZE-rp;let avail=n[0]+wp-(*l).header_size;if size>avail{n[1]=wp-(*l).header_size;}else if size>n[0]{n[1]=size-n[0];}else{n[0]=size;}}let mut done=0;for x in n{if x!=0{if rp==AMDGPU_UMSCHFW_LOG_SIZE{rp=(*l).header_size;}if copy_to_user(buf.add(done),(*u).log_cpu_addr.add(rp),x)!=0{return -EFAULT as isize;}done+=x;rp+=x;}}(*l).rptr=rp;*pos+=done as loff_t;done as isize}
pub unsafe fn amdgpu_debugfs_umsch_fwlog_init(_a:*mut amdgpu_device,_u:*mut amdgpu_umsch_mm){}

static umsch_mm_v4_0_ip_funcs: amd_ip_funcs=amd_ip_funcs{name:b"umsch_mm_v4_0\0".as_ptr(),early_init:Some(umsch_mm_early_init),late_init:Some(umsch_mm_late_init),sw_init:Some(umsch_mm_sw_init),sw_fini:Some(umsch_mm_sw_fini),hw_init:Some(umsch_mm_hw_init),hw_fini:Some(umsch_mm_hw_fini),suspend:Some(umsch_mm_suspend),resume:Some(umsch_mm_resume)};
pub static umsch_mm_v4_0_ip_block: amdgpu_ip_block_version=amdgpu_ip_block_version{type_:AMDGPU_IP_BLOCK_TYPE_UMSCH_MM,major:4,minor:0,rev:0,funcs:&umsch_mm_v4_0_ip_funcs};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
