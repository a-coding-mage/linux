// SPDX-License-Identifier: MIT
/* Translated from amdgpu_amdkfd.c. External kernel types and functions are
 * supplied by the surrounding translation unit. */

pub static mut amdgpu_amdkfd_total_mem_size: u64 = 0;
static mut kfd_initialized: bool = false;

pub unsafe fn amdgpu_amdkfd_init() -> i32 {
    let mut si: sysinfo = core::mem::zeroed();
    si_meminfo(&mut si);
    amdgpu_amdkfd_total_mem_size = (si.freeram - si.freehigh) * si.mem_unit;
    let ret = kgd2kfd_init();
    kfd_initialized = ret == 0;
    ret
}

pub unsafe fn amdgpu_amdkfd_fini() {
    if kfd_initialized {
        kgd2kfd_exit();
        kfd_initialized = false;
    }
}

pub unsafe fn amdgpu_amdkfd_device_probe(adev: *mut amdgpu_device) {
    let vf = amdgpu_sriov_vf(adev);
    if !kfd_initialized { return; }
    (*adev).kfd.dev = kgd2kfd_probe(adev, vf);
}

unsafe fn amdgpu_doorbell_get_kfd_info(adev: *mut amdgpu_device, aperture_base: *mut phys_addr_t, aperture_size: *mut usize, start_offset: *mut usize) {
    if (*adev).enable_mes {
        *aperture_base = (*adev).doorbell.base; *aperture_size = 0; *start_offset = 0;
    } else if (*adev).doorbell.size > (*adev).doorbell.num_kernel_doorbells * core::mem::size_of::<u32>() {
        *aperture_base = (*adev).doorbell.base;
        *aperture_size = (*adev).doorbell.size;
        *start_offset = (*adev).doorbell.num_kernel_doorbells * core::mem::size_of::<u32>();
    } else { *aperture_base = 0; *aperture_size = 0; *start_offset = 0; }
}

unsafe fn amdgpu_amdkfd_reset_work(work: *mut work_struct) {
    let adev = container_of!(work, amdgpu_device, kfd.reset_work);
    let mut reset_context: amdgpu_reset_context = core::mem::zeroed();
    reset_context.method = AMD_RESET_METHOD_NONE;
    reset_context.reset_req_dev = adev;
    reset_context.src = if (*adev).enable_mes { AMDGPU_RESET_SRC_MES } else { AMDGPU_RESET_SRC_HWS };
    clear_bit(AMDGPU_NEED_FULL_RESET, &mut reset_context.flags);
    amdgpu_device_gpu_recover(adev, core::ptr::null_mut(), &mut reset_context);
}

static kfd_client_funcs: drm_client_funcs = drm_client_funcs { unregister: Some(drm_client_release) };

pub unsafe fn amdgpu_amdkfd_drm_client_create(adev: *mut amdgpu_device) -> i32 {
    if !(*adev).kfd.init_complete || !(*adev).kfd.client.dev.is_null() { return 0; }
    let ret = drm_client_init(&mut (*adev).ddev, &mut (*adev).kfd.client, b"kfd\0".as_ptr() as *const i8, &kfd_client_funcs);
    if ret != 0 { dev_err((*adev).dev, b"Failed to init DRM client: %d\n\0".as_ptr(), ret); return ret; }
    drm_client_register(&mut (*adev).kfd.client); 0
}

pub unsafe fn amdgpu_amdkfd_device_init(adev: *mut amdgpu_device) {
    amdgpu_amdkfd_gpuvm_init_mem_limits();
    if !(*adev).kfd.dev.is_null() {
        let mut gpu_resources: kgd2kfd_shared_resources = core::mem::zeroed();
        gpu_resources.compute_vmid_bitmap = ((1 << AMDGPU_NUM_VMID) - 1) - ((1 << (*adev).vm_manager.first_kfd_vmid) - 1);
        gpu_resources.num_pipe_per_mec = (*adev).gfx.mec.num_pipe_per_mec;
        gpu_resources.num_queue_per_pipe = (*adev).gfx.mec.num_queue_per_pipe;
        gpu_resources.gpuvm_size = core::cmp::min((*adev).vm_manager.max_pfn << AMDGPU_GPU_PAGE_SHIFT, AMDGPU_GMC_HOLE_START);
        gpu_resources.drm_render_minor = adev_to_drm(adev).render.index;
        gpu_resources.sdma_doorbell_idx = (*adev).doorbell_index.sdma_engine;
        gpu_resources.enable_mes = (*adev).enable_mes;
        bitmap_complement(&mut gpu_resources.cp_queue_bitmap, &(*adev).gfx.mec_bitmap[0].queue_bitmap, AMDGPU_MAX_QUEUES);
        let last_valid_bit = (*adev).gfx.mec.num_pipe_per_mec * (*adev).gfx.mec.num_queue_per_pipe;
        for i in last_valid_bit..AMDGPU_MAX_QUEUES { clear_bit(i, &mut gpu_resources.cp_queue_bitmap); }
        amdgpu_doorbell_get_kfd_info(adev, &mut gpu_resources.doorbell_physical_address, &mut gpu_resources.doorbell_aperture_size, &mut gpu_resources.doorbell_start_offset);
        if (*adev).asic_type >= CHIP_VEGA10 { gpu_resources.non_cp_doorbells_start = (*adev).doorbell_index.first_non_cp; gpu_resources.non_cp_doorbells_end = (*adev).doorbell_index.last_non_cp; }
        (*adev).kfd.init_complete = kgd2kfd_device_init((*adev).kfd.dev, &mut gpu_resources);
        amdgpu_amdkfd_total_mem_size += (*adev).gmc.real_vram_size;
        INIT_WORK!(&mut (*adev).kfd.reset_work, amdgpu_amdkfd_reset_work);
    }
}

pub unsafe fn amdgpu_amdkfd_device_fini_sw(adev: *mut amdgpu_device) { if !(*adev).kfd.dev.is_null() { kgd2kfd_device_exit((*adev).kfd.dev); (*adev).kfd.dev = core::ptr::null_mut(); amdgpu_amdkfd_total_mem_size -= (*adev).gmc.real_vram_size; } }
pub unsafe fn amdgpu_amdkfd_interrupt(adev: *mut amdgpu_device, entry: *const core::ffi::c_void) { if !(*adev).kfd.dev.is_null() { kgd2kfd_interrupt((*adev).kfd.dev, entry); } }
pub unsafe fn amdgpu_amdkfd_teardown_processes(adev: *mut amdgpu_device) { kgd2kfd_teardown_processes(adev); }
pub unsafe fn amdgpu_amdkfd_suspend(adev: *mut amdgpu_device, suspend_proc: bool) { if !(*adev).kfd.dev.is_null() { if (*adev).in_s0ix { kgd2kfd_stop_sched_all_nodes((*adev).kfd.dev); } else { kgd2kfd_suspend((*adev).kfd.dev, suspend_proc); } } }
pub unsafe fn amdgpu_amdkfd_resume(adev: *mut amdgpu_device, resume_proc: bool) -> i32 { if (*adev).kfd.dev.is_null() { 0 } else if (*adev).in_s0ix { kgd2kfd_start_sched_all_nodes((*adev).kfd.dev) } else { kgd2kfd_resume((*adev).kfd.dev, resume_proc) } }
pub unsafe fn amdgpu_amdkfd_suspend_process(adev: *mut amdgpu_device) { if !(*adev).kfd.dev.is_null() { kgd2kfd_suspend_process((*adev).kfd.dev); } }
pub unsafe fn amdgpu_amdkfd_resume_process(adev: *mut amdgpu_device) -> i32 { if (*adev).kfd.dev.is_null() { 0 } else { kgd2kfd_resume_process((*adev).kfd.dev) } }
pub unsafe fn amdgpu_amdkfd_pre_reset(adev: *mut amdgpu_device, c: *mut amdgpu_reset_context) -> i32 { if (*adev).kfd.dev.is_null() { 0 } else { kgd2kfd_pre_reset((*adev).kfd.dev, c) } }
pub unsafe fn amdgpu_amdkfd_post_reset(adev: *mut amdgpu_device) -> i32 { if (*adev).kfd.dev.is_null() { 0 } else { kgd2kfd_post_reset((*adev).kfd.dev) } }
pub unsafe fn amdgpu_amdkfd_gpu_reset(adev: *mut amdgpu_device) { if amdgpu_device_should_recover_gpu(adev) { amdgpu_reset_domain_schedule((*adev).reset_domain, &mut (*adev).kfd.reset_work); } }

pub unsafe fn amdgpu_amdkfd_clear_kfd_mapping(adev: *mut amdgpu_device) {
    /* CONFIG_HSA_AMD guarded in the C source. */
    let kfd = (*adev).kfd.dev as *mut kfd_dev; if kfd.is_null() { return; }
    for i in 0..(*kfd).num_nodes { let node = (*kfd).nodes[i]; kfd_dev_unmap_mapping_range(KFD_MMAP_TYPE_DOORBELL | KFD_MMAP_GPU_ID((*node).id), kfd_doorbell_process_slice(kfd)); kfd_dev_unmap_mapping_range(KFD_MMAP_TYPE_MMIO | KFD_MMAP_GPU_ID((*node).id), PAGE_SIZE); }
}

pub unsafe fn amdgpu_amdkfd_alloc_kernel_mem(adev: *mut amdgpu_device, size: usize, domain: u32, mem_obj: *mut *mut core::ffi::c_void, gpu_addr: *mut u64, cpu_ptr: *mut *mut core::ffi::c_void, cp_mqd_gfx9: bool) -> i32 {
    let mut bo: *mut amdgpu_bo = core::ptr::null_mut(); let mut bp: amdgpu_bo_param = core::mem::zeroed(); let mut cpu_ptr_tmp = core::ptr::null_mut();
    bp.size=size; bp.byte_align=PAGE_SIZE; bp.domain=domain; bp.flags=AMDGPU_GEM_CREATE_VRAM_CONTIGUOUS|AMDGPU_GEM_CREATE_CPU_GTT_USWC; bp.r#type=ttm_bo_type_kernel; bp.bo_ptr_size=core::mem::size_of::<amdgpu_bo>(); if cp_mqd_gfx9 { bp.flags|=AMDGPU_GEM_CREATE_CP_MQD_GFX9; }
    let mut r=amdgpu_bo_create(adev,&mut bp,&mut bo); if r!=0 { dev_err((*adev).dev,b"failed to allocate BO for amdkfd (%d)\n\0".as_ptr(),r); return r; }
    r=amdgpu_bo_reserve(bo,true); if r!=0 { goto_alloc_fail!(bo,r); }
    r=amdgpu_bo_pin(bo,domain); if r!=0 { goto_alloc_fail!(bo,r); }
    r=amdgpu_ttm_alloc_gart(&mut (*bo).tbo); if r!=0 { goto_alloc_fail!(bo,r); }
    r=amdgpu_bo_kmap(bo,&mut cpu_ptr_tmp); if r!=0 { goto_alloc_fail!(bo,r); }
    *mem_obj=bo as *mut core::ffi::c_void; *gpu_addr=amdgpu_bo_gpu_offset(bo); *cpu_ptr=cpu_ptr_tmp; amdgpu_bo_unreserve(bo); 0
}

pub unsafe fn amdgpu_amdkfd_free_kernel_mem(_adev: *mut amdgpu_device, mem_obj: *mut *mut core::ffi::c_void) { let bo=mem_obj as *mut *mut amdgpu_bo; if bo.is_null()||(*bo).is_null(){return;} let _=amdgpu_bo_reserve(*bo,true); amdgpu_bo_kunmap(*bo); amdgpu_bo_unpin(*bo); amdgpu_bo_unreserve(*bo); amdgpu_bo_unref(bo); }
pub unsafe fn amdgpu_amdkfd_alloc_gws(adev:*mut amdgpu_device,size:usize,mem_obj:*mut *mut core::ffi::c_void)->i32 { let mut bp:amdgpu_bo_param=core::mem::zeroed(); let mut ubo: *mut amdgpu_bo_user=core::ptr::null_mut(); bp.size=size;bp.byte_align=1;bp.domain=AMDGPU_GEM_DOMAIN_GWS;bp.flags=AMDGPU_GEM_CREATE_NO_CPU_ACCESS;bp.r#type=ttm_bo_type_device;bp.bo_ptr_size=core::mem::size_of::<amdgpu_bo>(); let r=amdgpu_bo_create_user(adev,&mut bp,&mut ubo); if r!=0{return r;} *mem_obj=&mut (*ubo).bo as *mut amdgpu_bo as *mut core::ffi::c_void;0 }
pub unsafe fn amdgpu_amdkfd_free_gws(_adev:*mut amdgpu_device,mem_obj:*mut core::ffi::c_void){let mut bo=mem_obj as *mut amdgpu_bo;amdgpu_bo_unref(&mut bo);}

pub unsafe fn amdgpu_amdkfd_get_fw_version(adev:*mut amdgpu_device,ty:kgd_engine_type)->u32 { match ty { KGD_ENGINE_PFP=>(*adev).gfx.pfp_fw_version,KGD_ENGINE_ME=>(*adev).gfx.me_fw_version,KGD_ENGINE_CE=>(*adev).gfx.ce_fw_version,KGD_ENGINE_MEC1=>(*adev).gfx.mec_fw_version,KGD_ENGINE_MEC2=>(*adev).gfx.mec2_fw_version,KGD_ENGINE_RLC=>(*adev).gfx.rlc_fw_version,KGD_ENGINE_SDMA1=>(*adev).sdma.instance[0].fw_version,KGD_ENGINE_SDMA2=>(*adev).sdma.instance[1].fw_version,_=>0} }

pub unsafe fn amdgpu_amdkfd_get_local_mem_info(adev:*mut amdgpu_device,mem_info:*mut kfd_local_mem_info,xcp:*mut amdgpu_xcp){core::ptr::write_bytes(mem_info,0,1);if !xcp.is_null(){if (*adev).gmc.real_vram_size==(*adev).gmc.visible_vram_size{(*mem_info).local_mem_size_public=KFD_XCP_MEMORY_SIZE!(adev,(*xcp).id);}else{(*mem_info).local_mem_size_private=KFD_XCP_MEMORY_SIZE!(adev,(*xcp).id);}}else if (*adev).apu_prefer_gtt{(*mem_info).local_mem_size_public=ttm_tt_pages_limit()<<PAGE_SHIFT;}else{(*mem_info).local_mem_size_public=(*adev).gmc.visible_vram_size;(*mem_info).local_mem_size_private=(*adev).gmc.real_vram_size-(*adev).gmc.visible_vram_size;}(*mem_info).vram_width=(*adev).gmc.vram_width;if (*adev).pm.dpm_enabled{(*mem_info).mem_clk_max=if amdgpu_emu_mode==1{0}else{amdgpu_dpm_get_mclk(adev,false)/100};}else{(*mem_info).mem_clk_max=100;}}
pub unsafe fn amdgpu_amdkfd_get_gpu_clock_counter(adev:*mut amdgpu_device)->u64{if let Some(f)=(*(*adev).gfx.funcs).get_gpu_clock_counter{f(adev)}else{0}}
pub unsafe fn amdgpu_amdkfd_get_max_engine_clock_in_mhz(adev:*mut amdgpu_device)->u32{if (*adev).pm.dpm_enabled{amdgpu_dpm_get_sclk(adev,false)/100}else{100}}

pub unsafe fn amdgpu_amdkfd_get_pcie_bandwidth_mbytes(adev:*mut amdgpu_device,is_min:bool)->i32{let n=if is_min{ffs((*adev).pm.pcie_mlw_mask)}else{fls((*adev).pm.pcie_mlw_mask)}-1;let g=if is_min{ffs((*adev).pm.pcie_gen_mask&CAIL_PCIE_LINK_SPEED_SUPPORT_MASK)}else{fls((*adev).pm.pcie_gen_mask&CAIL_PCIE_LINK_SPEED_SUPPORT_MASK)}-1;let lanes=1<<n;let speed=1<<g;let nf=match lanes{CAIL_PCIE_LINK_WIDTH_SUPPORT_X1=>1,CAIL_PCIE_LINK_WIDTH_SUPPORT_X2=>2,CAIL_PCIE_LINK_WIDTH_SUPPORT_X4=>4,CAIL_PCIE_LINK_WIDTH_SUPPORT_X8=>8,CAIL_PCIE_LINK_WIDTH_SUPPORT_X12=>12,CAIL_PCIE_LINK_WIDTH_SUPPORT_X16=>16,CAIL_PCIE_LINK_WIDTH_SUPPORT_X32=>32,_=>0};let sf=match speed{CAIL_PCIE_LINK_SPEED_SUPPORT_GEN1=>2500,CAIL_PCIE_LINK_SPEED_SUPPORT_GEN2=>5000,CAIL_PCIE_LINK_SPEED_SUPPORT_GEN3=>8000,CAIL_PCIE_LINK_SPEED_SUPPORT_GEN4=>16000,CAIL_PCIE_LINK_SPEED_SUPPORT_GEN5=>32000,_=>0};nf*sf/BITS_PER_BYTE}

pub unsafe fn amdgpu_amdkfd_submit_ib(adev:*mut amdgpu_device,engine:kgd_engine_type,vmid:u32,gpu_addr:u64,ib_cmd:*mut u32,ib_len:u32)->i32{let ring=match engine{KGD_ENGINE_MEC1=>&mut (*adev).gfx.compute_ring[0],KGD_ENGINE_SDMA1=>&mut (*adev).sdma.instance[0].ring,KGD_ENGINE_SDMA2=>&mut (*adev).sdma.instance[1].ring,_=>return -EINVAL};let mut job=core::ptr::null_mut();let mut ret=amdgpu_job_alloc(adev,core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut(),1,0,GFP_KERNEL,&mut job);if ret!=0{return ret;}let ib=&mut (*job).ibs[0];core::ptr::write_bytes(ib,0,1);ib.gpu_addr=gpu_addr;ib.ptr=ib_cmd;ib.length_dw=ib_len;(*job).vmid=vmid;(*job).num_ibs=1;let mut f=core::ptr::null_mut();ret=amdgpu_ib_schedule(ring,1,ib,job,&mut f);if ret==0{ret=dma_fence_wait(f,false);dma_fence_put(f);}amdgpu_job_free(job);ret}

pub unsafe fn amdgpu_amdkfd_set_compute_idle(adev:*mut amdgpu_device,idle:bool){let state=if idle{AMD_PG_STATE_GATE}else{AMD_PG_STATE_UNGATE};let ver=IP_VERSION_MAJ(amdgpu_ip_version(adev,GC_HWIP,0));if(ver==11&&((*adev).mes.kiq_version&AMDGPU_MES_VERSION_MASK)<=64)||ver==12{amdgpu_gfx_off_ctrl(adev,idle);}else if ver==9&&((*adev).flags&AMD_IS_APU)!=0{let b=amdgpu_device_ip_get_ip_block(adev,AMD_IP_BLOCK_TYPE_GFX);if !b.is_null(){(*(*(*b).version).funcs).set_powergating_state(b as *mut core::ffi::c_void,state);}}let _=amdgpu_dpm_switch_power_profile(adev,PP_SMC_POWER_PROFILE_COMPUTE,!idle);}
pub unsafe fn amdgpu_amdkfd_is_kfd_vmid(adev:*mut amdgpu_device,vmid:u32)->bool{!(*adev).kfd.dev.is_null()&&vmid>=(*adev).vm_manager.first_kfd_vmid}
pub unsafe fn amdgpu_amdkfd_have_atomics_support(adev:*mut amdgpu_device)->bool{(*adev).have_atomics_support}
pub unsafe fn amdgpu_amdkfd_debug_mem_fence(adev:*mut amdgpu_device){amdgpu_device_flush_hdp(adev,core::ptr::null_mut());}
pub unsafe fn amdgpu_amdkfd_is_fed(adev:*mut amdgpu_device)->bool{amdgpu_ras_get_fed_status(adev)}
pub unsafe fn amdgpu_amdkfd_ras_pasid_poison_consumption_handler(adev:*mut amdgpu_device,block:amdgpu_ras_block,pasid:u16,pasid_fn:pasid_notify,data:*mut core::ffi::c_void,reset:u32){amdgpu_umc_pasid_poison_handler(adev,block,pasid,pasid_fn,data,reset);}
pub unsafe fn amdgpu_amdkfd_ras_poison_consumption_handler(adev:*mut amdgpu_device,block:amdgpu_ras_block,reset:u32){amdgpu_umc_pasid_poison_handler(adev,block,0,None,core::ptr::null_mut(),reset);}
pub unsafe fn amdgpu_amdkfd_send_close_event_drain_irq(adev:*mut amdgpu_device,payload:*mut u32)->i32{let r=amdgpu_ih_wait_on_checkpoint_process_ts(adev,&mut (*adev).irq.ih);if r!=0{return r;}amdgpu_amdkfd_interrupt(adev,payload as *const _);0}
pub unsafe fn amdgpu_amdkfd_check_and_lock_kfd(adev:*mut amdgpu_device)->i32{kgd2kfd_check_and_lock_kfd((*adev).kfd.dev)}
pub unsafe fn amdgpu_amdkfd_unlock_kfd(adev:*mut amdgpu_device){kgd2kfd_unlock_kfd((*adev).kfd.dev);}
pub unsafe fn amdgpu_amdkfd_xcp_memory_size(adev:*mut amdgpu_device,xcp_id:i32)->u64{let mem_id=KFD_XCP_MEM_ID!(adev,xcp_id);let mut tmp;if (*adev).gmc.num_mem_partitions!=0&&xcp_id>=0&&mem_id>=0{if (*adev).gmc.is_app_apu&&(*adev).gmc.num_mem_partitions==1{tmp=(ttm_tt_pages_limit()<<PAGE_SHIFT)/num_online_nodes();}else{tmp=(*adev).gmc.mem_partitions[mem_id as usize].size;}if (*adev).xcp_mgr.mem_alloc_mode==AMDGPU_PARTITION_MEM_CAPPING_EVEN{tmp/=(*adev).xcp_mgr.num_xcp_per_mem_partition;}ALIGN_DOWN!(tmp,PAGE_SIZE)}else if (*adev).apu_prefer_gtt{ttm_tt_pages_limit()<<PAGE_SHIFT}else{(*adev).gmc.real_vram_size}}

pub unsafe fn amdgpu_amdkfd_stop_sched(adev:*mut amdgpu_device,node_id:u32)->i32{if !(*adev).kfd.init_complete{0}else{kgd2kfd_stop_sched((*adev).kfd.dev,node_id)}}
pub unsafe fn amdgpu_amdkfd_start_sched(adev:*mut amdgpu_device,node_id:u32)->i32{if !(*adev).kfd.init_complete{0}else{kgd2kfd_start_sched((*adev).kfd.dev,node_id)}}
pub unsafe fn amdgpu_amdkfd_compute_active(adev:*mut amdgpu_device,node_id:u32)->bool{(*adev).kfd.init_complete&&kgd2kfd_compute_active((*adev).kfd.dev,node_id)}
pub unsafe fn amdgpu_amdkfd_config_sq_perfmon(adev:*mut amdgpu_device,xcp_id:u32,core_override_enable:bool,reg_override_enable:bool,perfmon_override_enable:bool)->i32{if !(*adev).kfd.init_complete{0}else{psp_config_sq_perfmon(&mut (*adev).psp,xcp_id,core_override_enable,reg_override_enable,perfmon_override_enable)}}
pub unsafe fn amdgpu_amdkfd_reset_mes_queue(adev:*mut amdgpu_device,node_id:u32,queue_type:i32,pipe:i32,queue:i32,db:u32)->i32{if !(*adev).kfd.init_complete{0}else{kgd2kfd_reset_mes_queue((*adev).kfd.dev,node_id,queue_type,pipe,queue,db)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
