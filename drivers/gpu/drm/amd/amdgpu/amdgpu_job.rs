/* Translated from amdgpu_job.c. External kernel and AMDGPU declarations are
 * supplied by the surrounding translation unit. */

unsafe fn amdgpu_job_do_core_dump(adev: *mut amdgpu_device, job: *mut amdgpu_job) {
    dev_info((*adev).dev, "Dumping IP State\n");
    for i in 0..(*adev).num_ip_blocks {
        let block = &mut (*adev).ip_blocks[i as usize];
        if let Some(f) = (*(*block.version).funcs).dump_ip_state {
            f(block as *mut _ as *mut core::ffi::c_void);
        }
    }
    dev_info((*adev).dev, "Dumping IP State Completed\n");
    amdgpu_coredump(adev, true, false, job);
}

unsafe fn amdgpu_job_core_dump(adev: *mut amdgpu_device, job: *mut amdgpu_job) {
    let mut device_list: list_head = core::mem::zeroed();
    let mut device_list_handle: *mut list_head = core::ptr::null_mut();
    let mut tmp_adev: *mut amdgpu_device = core::ptr::null_mut();
    let mut hive: *mut amdgpu_hive_info = core::ptr::null_mut();
    if !amdgpu_sriov_vf(adev) { hive = amdgpu_get_xgmi_hive(adev); }
    if !hive.is_null() { mutex_lock(&mut (*hive).hive_lock); }
    INIT_LIST_HEAD(&mut device_list);
    if !amdgpu_sriov_vf(adev) && (*adev).gmc.xgmi.num_physical_nodes > 1 && !hive.is_null() {
        list_for_each_entry!(tmp_adev, &mut (*hive).device_list, gmc.xgmi.head,
            list_add_tail!(&mut (*tmp_adev).reset_list, &mut device_list));
        if !list_is_first(&(*adev).reset_list, &device_list) {
            list_rotate_to_front(&mut (*adev).reset_list, &mut device_list);
        }
        device_list_handle = &mut device_list;
    } else {
        list_add_tail(&mut (*adev).reset_list, &mut device_list);
        device_list_handle = &mut device_list;
    }
    list_for_each_entry!(tmp_adev, device_list_handle, reset_list,
        amdgpu_job_do_core_dump(tmp_adev, job));
    if !hive.is_null() { mutex_unlock(&mut (*hive).hive_lock); amdgpu_put_xgmi_hive(hive); }
}

unsafe fn amdgpu_job_timedout(s_job: *mut drm_sched_job) -> drm_gpu_sched_stat {
    let ring = to_amdgpu_ring((*s_job).sched);
    let job = to_amdgpu_job(s_job);
    let mut info: *mut drm_wedge_task_info = core::ptr::null_mut();
    let mut ti: *mut amdgpu_task_info = core::ptr::null_mut();
    let adev = (*ring).adev;
    let mut idx = 0; let mut r;
    if !drm_dev_enter(adev_to_drm(adev), &mut idx) {
        dev_info((*adev).dev, "%s - device unplugged skipping recovery on scheduler:%s", __func__, (*(*s_job).sched).name);
        return DRM_GPU_SCHED_STAT_ENODEV;
    }
    if !amdgpu_sriov_vf(adev) { amdgpu_job_core_dump(adev, job); }
    if amdgpu_gpu_recovery && amdgpu_ring_is_reset_type_supported(ring, AMDGPU_RESET_TYPE_SOFT_RECOVERY) &&
       amdgpu_ring_soft_recovery(ring, (*job).vmid, (*(*s_job).s_fence).parent) {
        dev_err((*adev).dev, "ring %s timeout, but soft recovered\n", (*(*s_job).sched).name); goto_exit!();
    }
    dev_err((*adev).dev, "ring %s timeout, signaled seq=%u, emitted seq=%u\n", (*job).base.sched.name,
        atomic_read(&(*ring).fence_drv.last_seq), (*ring).fence_drv.sync_seq);
    ti = amdgpu_vm_get_task_info_pasid((*ring).adev, (*job).pasid);
    if !ti.is_null() { amdgpu_vm_print_task_info(adev, ti); info = &mut (*ti).task; }
    if amdgpu_gpu_recovery && amdgpu_ring_is_reset_type_supported(ring, AMDGPU_RESET_TYPE_PER_QUEUE) && (*ring).funcs.reset.is_some() {
        dev_err((*adev).dev, "Starting %s ring reset\n", (*(*s_job).sched).name);
        drm_sched_wqueue_stop(&mut (*ring).sched);
        r = amdgpu_ring_reset(ring, (*job).vmid, (*job).hw_fence);
        if r == 0 { drm_sched_wqueue_start(&mut (*ring).sched); atomic_inc(&mut (*ring).adev.gpu_reset_counter);
            dev_err((*adev).dev, "Ring %s reset succeeded\n", (*ring).sched.name);
            drm_dev_wedged_event(adev_to_drm(adev), DRM_WEDGE_RECOVERY_NONE, info); goto_exit!(); }
        dev_err((*adev).dev, "Ring %s reset failed\n", (*ring).sched.name);
    }
    if amdgpu_gpu_recovery && amdgpu_ring_is_reset_type_supported(ring, AMDGPU_RESET_TYPE_IP_BLOCK_SOFT_RESET) {
        r = amdgpu_device_ip_soft_reset(ring, (*job).hw_fence);
        if r == 0 { atomic_inc(&mut (*ring).adev.gpu_reset_counter); drm_dev_wedged_event(adev_to_drm(adev), DRM_WEDGE_RECOVERY_NONE, info); goto_exit!(); }
    }
    if dma_fence_get_status(&(*(*s_job).s_fence).finished) == 0 { dma_fence_set_error(&mut (*(*s_job).s_fence).finished, -ETIME); }
    if amdgpu_device_should_recover_gpu((*ring).adev) {
        let mut reset_context: amdgpu_reset_context = core::mem::zeroed();
        reset_context.method = AMD_RESET_METHOD_NONE; reset_context.reset_req_dev = adev; reset_context.src = AMDGPU_RESET_SRC_JOB;
        clear_bit(AMDGPU_NEED_FULL_RESET, &mut reset_context.flags); set_bit(AMDGPU_SKIP_COREDUMP, &mut reset_context.flags);
        r = amdgpu_device_gpu_recover((*ring).adev, job, &mut reset_context);
        if r != 0 { dev_err((*adev).dev, "GPU Recovery Failed: %d\n", r); }
    } else { drm_sched_suspend_timeout(&mut (*ring).sched); if amdgpu_sriov_vf(adev) { (*adev).virt.tdr_debug = true; } }
exit:
    amdgpu_vm_put_task_info(ti); drm_dev_exit(idx); DRM_GPU_SCHED_STAT_NO_HANG
}

pub unsafe fn amdgpu_job_alloc(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, entity: *mut drm_sched_entity, owner: *mut core::ffi::c_void, num_ibs: u32, drm_client_id: u64, gfp_flags: gfp_t, job: *mut *mut amdgpu_job) -> i32 {
    if num_ibs == 0 { return -EINVAL; }
    *job = kzalloc_flex::<amdgpu_job>(num_ibs, gfp_flags);
    if (*job).is_null() { return -ENOMEM; }
    (*job).hw_fence = kzalloc_obj::<amdgpu_fence>(gfp_flags); if (*job).hw_fence.is_null() { kfree(*job); *job=core::ptr::null_mut(); return -ENOMEM; }
    (*job).hw_vm_fence = kzalloc_obj::<amdgpu_fence>(gfp_flags); if (*job).hw_vm_fence.is_null() { kfree((*job).hw_fence); kfree(*job); *job=core::ptr::null_mut(); return -ENOMEM; }
    (*job).vm=vm; amdgpu_sync_create(&mut (*job).explicit_sync); (*job).generation=amdgpu_vm_generation(adev,vm); (*job).vm_pd_addr=AMDGPU_BO_INVALID_OFFSET;
    if entity.is_null() { return 0; }
    let r=drm_sched_job_init(&mut (*job).base,entity,1,owner,drm_client_id); if r==0 { return 0; }
    kfree((*job).hw_vm_fence); kfree((*job).hw_fence); kfree(*job); *job=core::ptr::null_mut(); r
}

pub unsafe fn amdgpu_job_alloc_with_ib(adev:*mut amdgpu_device, entity:*mut drm_sched_entity, owner:*mut core::ffi::c_void, size:usize, pool_type:enum_amdgpu_ib_pool_type, k_job_id:u64, job:*mut *mut amdgpu_job)->i32 {
    let r=amdgpu_job_alloc(adev,core::ptr::null_mut(),entity,owner,1,k_job_id,amdgpu_ib_pool_gfp_flags(adev,pool_type),job); if r!=0{return r;}
    (*job).num_ibs=1; let r=amdgpu_ib_get(adev,core::ptr::null_mut(),size,pool_type,&mut (*job).ibs[0]); if r!=0 { if !entity.is_null(){drm_sched_job_cleanup(&mut (*job).base);} kfree((*job).hw_vm_fence); kfree((*job).hw_fence); kfree(*job); *job=core::ptr::null_mut(); } r
}

pub unsafe fn amdgpu_job_set_resources(job:*mut amdgpu_job,gds:*mut amdgpu_bo,gws:*mut amdgpu_bo,oa:*mut amdgpu_bo){if !gds.is_null(){(*job).gds_base=amdgpu_bo_gpu_offset(gds)>>PAGE_SHIFT;(*job).gds_size=amdgpu_bo_size(gds)>>PAGE_SHIFT;}if !gws.is_null(){(*job).gws_base=amdgpu_bo_gpu_offset(gws)>>PAGE_SHIFT;(*job).gws_size=amdgpu_bo_size(gws)>>PAGE_SHIFT;}if !oa.is_null(){(*job).oa_base=amdgpu_bo_gpu_offset(oa)>>PAGE_SHIFT;(*job).oa_size=amdgpu_bo_size(oa)>>PAGE_SHIFT;}}
pub unsafe fn amdgpu_job_free_resources(job:*mut amdgpu_job){let f=if !(*job).base.s_fence.is_null()&&dma_fence_was_initialized(&(*(*job).base.s_fence).finished){&mut (*(*job).base.s_fence).finished}else if dma_fence_was_initialized(&(*(*job).hw_fence).base){&mut (*(*job).hw_fence).base}else{core::ptr::null_mut()};for i in 0..(*job).num_ibs{amdgpu_ib_free(&mut (*job).ibs[i as usize],f);}}

unsafe fn amdgpu_job_free_cb(s_job:*mut drm_sched_job){let job=to_amdgpu_job(s_job);drm_sched_job_cleanup(s_job);amdgpu_sync_free(&mut (*job).explicit_sync);if dma_fence_was_initialized(&(*(*job).hw_fence).base){dma_fence_put(&mut (*(*job).hw_fence).base);}else{kfree((*job).hw_fence);}if dma_fence_was_initialized(&(*(*job).hw_vm_fence).base){dma_fence_put(&mut (*(*job).hw_vm_fence).base);}else{kfree((*job).hw_vm_fence);}kfree(job);}
pub unsafe fn amdgpu_job_set_gang_leader(job:*mut amdgpu_job,leader:*mut amdgpu_job){let fence=&mut (*(*leader).base.s_fence).scheduled;if (*job).gang_submit.is_some(){WARN_ON(true);}if job!=leader{dma_fence_get(fence);}(*job).gang_submit=fence;}
pub unsafe fn amdgpu_job_free(job:*mut amdgpu_job){if !(*job).base.entity.is_null(){drm_sched_job_cleanup(&mut (*job).base);}amdgpu_job_free_resources(job);amdgpu_sync_free(&mut (*job).explicit_sync);if (*job).gang_submit!=&mut (*(*job).base.s_fence).scheduled{dma_fence_put((*job).gang_submit);}if dma_fence_was_initialized(&(*(*job).hw_fence).base){dma_fence_put(&mut (*(*job).hw_fence).base);}else{kfree((*job).hw_fence);}if dma_fence_was_initialized(&(*(*job).hw_vm_fence).base){dma_fence_put(&mut (*(*job).hw_vm_fence).base);}else{kfree((*job).hw_vm_fence);}kfree(job);}
pub unsafe fn amdgpu_job_submit(job:*mut amdgpu_job)->*mut dma_fence{drm_sched_job_arm(&mut (*job).base);let f=dma_fence_get(&mut (*(*job).base.s_fence).finished);amdgpu_job_free_resources(job);drm_sched_entity_push_job(&mut (*job).base);f}
pub unsafe fn amdgpu_job_submit_direct(job:*mut amdgpu_job,ring:*mut amdgpu_ring,fence:*mut *mut dma_fence)->i32{(*job).base.sched=&mut (*ring).sched;let r=amdgpu_ib_schedule(ring,(*job).num_ibs,(*job).ibs,job,fence);if r!=0{return r;}amdgpu_job_free(job);0}

unsafe fn amdgpu_job_prepare_job(sched_job:*mut drm_sched_job,s_entity:*mut drm_sched_entity)->*mut dma_fence{let ring=to_amdgpu_ring((*(*s_entity).rq).sched);let job=to_amdgpu_job(sched_job);let mut fence;let r=drm_sched_entity_error(s_entity);if r!=0{dma_fence_set_error(&mut (*(*job).base.s_fence).finished,r);return core::ptr::null_mut();}if !(*job).gang_submit.is_null(){fence=amdgpu_device_switch_gang((*ring).adev,(*job).gang_submit);if !fence.is_null(){return fence;}}fence=amdgpu_device_enforce_isolation((*ring).adev,ring,job);if !fence.is_null(){return fence;}if !(*job).vm.is_null()&&(*job).vmid==0{let r=amdgpu_vmid_grab((*job).vm,ring,job,&mut fence);if r!=0{dev_err((*(*ring).adev).dev,"Error getting VM ID (%d)\n",r);dma_fence_set_error(&mut (*(*job).base.s_fence).finished,r);return core::ptr::null_mut();}return fence;}core::ptr::null_mut()}
unsafe fn amdgpu_job_run(sched_job:*mut drm_sched_job)->*mut dma_fence{let ring=to_amdgpu_ring((*sched_job).sched);let adev=(*ring).adev;let mut fence=core::ptr::null_mut();let job=to_amdgpu_job(sched_job);let finished=&mut (*(*job).base.s_fence).finished;trace_amdgpu_sched_run_job(job);if (*job).generation!=amdgpu_vm_generation(adev,(*job).vm)||((*job).job_run_counter!=0&&!(*job).gang_submit.is_null()){dma_fence_set_error(finished,-ECANCELED);}let mut r=0;if (*finished).error<0{dev_dbg((*adev).dev,"Skip scheduling IBs in ring(%s)",(*ring).name);}else{r=amdgpu_ib_schedule(ring,(*job).num_ibs,(*job).ibs,job,&mut fence);if r!=0{dev_err((*adev).dev,"Error scheduling IBs (%d) in ring(%s)",r,(*ring).name);}}(*job).job_run_counter+=1;amdgpu_job_free_resources(job);if r!=0{ERR_PTR(r)}else{fence}}

unsafe fn drm_sched_entity_queue_pop(entity:*mut drm_sched_entity)->*mut drm_sched_job{let node=spsc_queue_pop(&mut (*entity).job_queue);if node.is_null(){core::ptr::null_mut()}else{container_of!(node,drm_sched_job,queue_node)}}

pub unsafe fn amdgpu_job_stop_all_jobs_on_sched(sched:*mut drm_gpu_scheduler){for i in DRM_SCHED_PRIORITY_KERNEL..(*sched).num_rqs{let rq=(*sched).sched_rq[i as usize];spin_lock(&mut (*rq).lock);list_for_each_entry!(s_entity,&mut (*rq).entities,list,{while let Some(s_job)=drm_sched_entity_queue_pop(s_entity){let f=(*s_job).s_fence;dma_fence_signal(&mut (*f).scheduled);dma_fence_set_error(&mut (*f).finished,-EHWPOISON);dma_fence_signal(&mut (*f).finished);}});spin_unlock(&mut (*rq).lock);}list_for_each_entry!(s_job,&mut (*sched).pending_list,list,{let f=(*s_job).s_fence;dma_fence_set_error(&mut (*f).finished,-EHWPOISON);dma_fence_signal(&mut (*f).finished);});}

pub static amdgpu_sched_ops: drm_sched_backend_ops = drm_sched_backend_ops { prepare_job:Some(amdgpu_job_prepare_job), run_job:Some(amdgpu_job_run), timedout_job:Some(amdgpu_job_timedout), free_job:Some(amdgpu_job_free_cb) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
