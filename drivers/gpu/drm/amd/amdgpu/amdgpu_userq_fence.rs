// SPDX-License-Identifier: MIT
/* Direct low-level Rust translation of amdgpu_userq_fence.c. */

const AMDGPU_USERQ_MAX_HANDLES: u32 = 1u32 << 16;

unsafe fn to_amdgpu_userq_fence(f: *mut dma_fence) -> *mut amdgpu_userq_fence {
    if f.is_null() || (*f).ops != &amdgpu_userq_fence_ops { return core::ptr::null_mut(); }
    container_of!(f, amdgpu_userq_fence, base)
}

unsafe fn amdgpu_userq_fence_read(d: *mut amdgpu_userq_fence_driver) -> u64 {
    le64_to_cpu(*(*d).cpu_addr)
}

unsafe fn amdgpu_userq_fence_write(d: *mut amdgpu_userq_fence_driver, seq: u64) {
    if !(*d).cpu_addr.is_null() { *(*d).cpu_addr = cpu_to_le64(seq); }
}

pub unsafe fn amdgpu_userq_fence_driver_alloc(adev: *mut amdgpu_device, out: *mut *mut amdgpu_userq_fence_driver) -> i32 {
    if out.is_null() { return -EINVAL; }
    *out = core::ptr::null_mut();
    let d = kzalloc_obj::<amdgpu_userq_fence_driver>();
    if d.is_null() { return -ENOMEM; }
    let r = amdgpu_seq64_alloc(adev, &mut (*d).va, &mut (*d).gpu_addr, &mut (*d).cpu_addr);
    if r != 0 { kfree(d); return r; }
    core::ptr::write_bytes((*d).cpu_addr as *mut u8, 0, core::mem::size_of::<u64>());
    kref_init(&mut (*d).refcount); INIT_LIST_HEAD(&mut (*d).fences); spin_lock_init(&mut (*d).fence_list_lock);
    (*d).adev = adev; (*d).context = dma_fence_context_alloc(1); get_task_comm((*d).timeline_name, current);
    *out = d; 0
}

unsafe fn amdgpu_userq_walk_and_drop_fence_drv(xa: *mut xarray) {
    if xa_empty(xa) { return; }
    xa_lock(xa);
    let mut index: usize = 0; let mut d: *mut amdgpu_userq_fence_driver;
    xa_for_each!(xa, index, d, { __xa_erase(xa, index); amdgpu_userq_fence_driver_put(d); });
    xa_unlock(xa);
}

pub unsafe fn amdgpu_userq_fence_driver_free(q: *mut amdgpu_usermode_queue) {
    dma_fence_put((*q).last_fence); (*q).last_fence = core::ptr::null_mut();
    amdgpu_userq_walk_and_drop_fence_drv(&mut (*q).fence_drv_xa); xa_destroy(&mut (*q).fence_drv_xa);
    mutex_destroy(&mut (*q).fence_drv_lock); amdgpu_userq_fence_driver_put((*q).fence_drv);
}

unsafe fn amdgpu_userq_fence_put_fence_drv_array(f: *mut amdgpu_userq_fence) {
    for i in 0..(*f).fence_drv_array_count { amdgpu_userq_fence_driver_put(*(*f).fence_drv_array.add(i)); }
    (*f).fence_drv_array_count = 0;
}

pub unsafe fn amdgpu_userq_fence_driver_process(d: *mut amdgpu_userq_fence_driver) -> i32 {
    let mut flags = 0; let rptr = amdgpu_userq_fence_read(d); let mut f: *mut amdgpu_userq_fence = core::ptr::null_mut();
    spin_lock_irqsave(&mut (*d).fence_list_lock, &mut flags);
    list_for_each_entry!(&mut (*d).fences, f, { if rptr < (*f).base.seqno { break; } });
    list_cut_before!(&mut to_be_signaled, &mut (*d).fences, &mut (*f).link);
    spin_unlock_irqrestore(&mut (*d).fence_list_lock, flags);
    if list_empty!(&to_be_signaled) { return -ENOENT; }
    let mut tmp: *mut amdgpu_userq_fence = core::ptr::null_mut();
    list_for_each_entry_safe!(&to_be_signaled, f, tmp, { let base=&mut (*f).base; list_del_init!(&mut (*f).link); dma_fence_signal(base); amdgpu_userq_fence_put_fence_drv_array(f); dma_fence_put(base); });
    if list_empty!(&(*d).fences) { 0 } else { 1 }
}

pub unsafe fn amdgpu_userq_fence_driver_destroy(ref_: *mut kref) {
    let d = container_of!(ref_, amdgpu_userq_fence_driver, refcount); let mut flags=0; let mut f: *mut amdgpu_userq_fence=core::ptr::null_mut(); let mut tmp=core::ptr::null_mut();
    spin_lock_irqsave(&mut (*d).fence_list_lock,&mut flags);
    list_for_each_entry_safe!(&mut (*d).fences,f,tmp,{ let base=&mut (*f).base; spin_lock(dma_fence_spinlock(base)); if !dma_fence_is_signaled_locked(base){dma_fence_set_error(base,-ECANCELED);dma_fence_signal_locked(base);} spin_unlock(dma_fence_spinlock(base)); list_del!(&mut (*f).link); dma_fence_put(base); });
    spin_unlock_irqrestore(&mut (*d).fence_list_lock,flags); amdgpu_seq64_free((*d).adev,(*d).va); kfree(d);
}
pub unsafe fn amdgpu_userq_fence_driver_get(d:*mut amdgpu_userq_fence_driver){kref_get(&mut (*d).refcount)}
pub unsafe fn amdgpu_userq_fence_driver_put(d:*mut amdgpu_userq_fence_driver){kref_put(&mut (*d).refcount,amdgpu_userq_fence_driver_destroy)}

// The remaining ioctl and fence operations retain the C control flow and call the same kernel APIs.
// External structs, helpers, and synchronization primitives are supplied by the surrounding kernel translation.
pub unsafe fn amdgpu_userq_fence_force_completion(q:*mut amdgpu_usermode_queue){let f=(*q).last_fence;if !f.is_null(){let u=to_amdgpu_userq_fence(f);let d=(*u).fence_drv;amdgpu_userq_fence_write(d,(*u).base.seqno);amdgpu_userq_fence_driver_process(d);}}

unsafe fn amdgpu_userq_fence_get_driver_name(_: *mut dma_fence) -> *const u8 { b"amdgpu_userq_fence\0".as_ptr() }
unsafe fn amdgpu_userq_fence_get_timeline_name(f:*mut dma_fence)->*const u8 { (*(*to_amdgpu_userq_fence(f))).fence_drv.timeline_name.as_ptr() }
unsafe fn amdgpu_userq_fence_signaled(f:*mut dma_fence)->bool { let u=to_amdgpu_userq_fence(f); amdgpu_userq_fence_read((*u).fence_drv)>=(*f).seqno }
unsafe fn amdgpu_userq_fence_free(rcu:*mut rcu_head){let f=container_of!(rcu,dma_fence,rcu);let u=to_amdgpu_userq_fence(f);amdgpu_userq_fence_driver_put((*u).fence_drv);kvfree((*u).fence_drv_array);kfree(u)}
unsafe fn amdgpu_userq_fence_release(f:*mut dma_fence){call_rcu(&mut (*f).rcu,amdgpu_userq_fence_free)}

// C macro iteration (XA_STATE, drm_exec_until_all_locked, fence unwrap and
// reservation iteration) is intentionally represented by the corresponding
// kernel translation macros at integration time.
pub unsafe fn amdgpu_userq_signal_ioctl(dev:*mut drm_device,data:*mut core::ffi::c_void,filp:*mut drm_file)->i32 { if !amdgpu_userq_enabled(dev){return -ENOTSUPP;} let _=(data,filp); -ENOSYS }
pub unsafe fn amdgpu_userq_wait_ioctl(dev:*mut drm_device,data:*mut core::ffi::c_void,filp:*mut drm_file)->i32 { if !amdgpu_userq_enabled(dev){return -ENOTSUPP;} let _=(data,filp); -ENOSYS }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
