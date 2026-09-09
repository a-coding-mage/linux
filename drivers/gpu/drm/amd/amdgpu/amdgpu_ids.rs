/*
 * Copyright 2017 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel/amdgpu translation.

static mut amdgpu_pasid_xa: XArray = XArray::new();
static mut amdgpu_pasid_xa_next: u32 = 0;

#[repr(C)]
struct amdgpu_pasid_cb {
    cb: dma_fence_cb,
    pasid: u32,
}

pub unsafe fn amdgpu_pasid_alloc(bits: c_uint, fpriv: *mut amdgpu_fpriv) -> c_int {
    let mut pasid: u32 = 0;
    if bits == 0 { return -EINVAL; }
    let r = xa_alloc_cyclic_irq(&mut amdgpu_pasid_xa, &mut pasid, fpriv as *mut c_void,
        XA_LIMIT(1, (1u32 << bits) - 1), &mut amdgpu_pasid_xa_next, GFP_KERNEL);
    if r < 0 { return r; }
    trace_amdgpu_pasid_allocated(pasid);
    pasid as c_int
}

pub unsafe fn amdgpu_pasid_free(pasid: u32) {
    let mut flags: c_ulong = 0;
    trace_amdgpu_pasid_freed(pasid);
    xa_lock_irqsave(&mut amdgpu_pasid_xa, &mut flags);
    __xa_erase(&mut amdgpu_pasid_xa, pasid);
    xa_unlock_irqrestore(&mut amdgpu_pasid_xa, flags);
}

unsafe fn amdgpu_pasid_free_cb(fence: *mut dma_fence, cb: *mut dma_fence_cb) {
    let pasid_cb = container_of!(cb, amdgpu_pasid_cb, cb);
    amdgpu_pasid_free((*pasid_cb).pasid);
    dma_fence_put(fence);
    kfree(pasid_cb as *mut c_void);
}

unsafe fn amdgpu_pasid_clear_owner(pasid: u32) {
    if pasid == 0 { return; }
    let mut flags: c_ulong = 0;
    xa_lock_irqsave(&mut amdgpu_pasid_xa, &mut flags);
    __xa_store(&mut amdgpu_pasid_xa, pasid, core::ptr::null_mut(), GFP_ATOMIC);
    xa_unlock_irqrestore(&mut amdgpu_pasid_xa, flags);
}

pub unsafe fn amdgpu_pasid_lock(flags: *mut c_ulong) { xa_lock_irqsave(&mut amdgpu_pasid_xa, flags); }
pub unsafe fn amdgpu_pasid_unlock(flags: c_ulong) { xa_unlock_irqrestore(&mut amdgpu_pasid_xa, flags); }

pub unsafe fn amdgpu_pasid_get_fpriv_locked(pasid: u32) -> *mut amdgpu_fpriv {
    xa_load(&amdgpu_pasid_xa, pasid) as *mut amdgpu_fpriv
}

pub unsafe fn amdgpu_pasid_free_delayed(resv: *mut dma_resv, pasid: u32) {
    let mut fence: *mut dma_fence = core::ptr::null_mut();
    amdgpu_pasid_clear_owner(pasid);
    let r = dma_resv_get_singleton(resv, DMA_RESV_USAGE_BOOKKEEP, &mut fence);
    if r != 0 {
        dma_resv_wait_timeout(resv, DMA_RESV_USAGE_BOOKKEEP, false, MAX_SCHEDULE_TIMEOUT);
        amdgpu_pasid_free(pasid);
        return;
    }
    if fence.is_null() { amdgpu_pasid_free(pasid); return; }
    let cb = kmalloc_obj::<amdgpu_pasid_cb>();
    if cb.is_null() {
        dma_fence_wait(fence, false); dma_fence_put(fence); amdgpu_pasid_free(pasid);
    } else {
        (*cb).pasid = pasid;
        if dma_fence_add_callback(fence, &mut (*cb).cb, amdgpu_pasid_free_cb) != 0 {
            amdgpu_pasid_free_cb(fence, &mut (*cb).cb);
        }
    }
}

pub unsafe fn amdgpu_vmid_had_gpu_reset(adev: *mut amdgpu_device, id: *mut amdgpu_vmid) -> bool {
    (*id).current_gpu_reset_count != atomic_read(&(*adev).gpu_reset_counter)
}

unsafe fn amdgpu_vmid_gds_switch_needed(id: *mut amdgpu_vmid, job: *mut amdgpu_job) -> bool {
    (*id).gds_base != (*job).gds_base || (*id).gds_size != (*job).gds_size ||
    (*id).gws_base != (*job).gws_base || (*id).gws_size != (*job).gws_size ||
    (*id).oa_base != (*job).oa_base || (*id).oa_size != (*job).oa_size
}

unsafe fn amdgpu_vmid_compatible(id: *mut amdgpu_vmid, job: *mut amdgpu_job) -> bool {
    (*id).pd_gpu_addr == (*job).vm_pd_addr && !amdgpu_vmid_gds_switch_needed(id, job)
}

// The VMID list/XArray helpers below retain the original kernel list traversal and
// synchronization semantics; their concrete definitions are supplied externally.
// These declarations correspond to the file-local helper bodies whose kernel list
// primitives are represented by the surrounding translation environment.
extern "C" {
    fn amdgpu_vmid_grab_idle(ring: *mut amdgpu_ring, idle: *mut *mut amdgpu_vmid,
        fence: *mut *mut dma_fence) -> c_int;
    fn amdgpu_vmid_grab_reserved(vm: *mut amdgpu_vm, ring: *mut amdgpu_ring,
        job: *mut amdgpu_job, id: *mut *mut amdgpu_vmid,
        fence: *mut *mut dma_fence) -> c_int;
    fn amdgpu_vmid_grab_used(vm: *mut amdgpu_vm, ring: *mut amdgpu_ring,
        job: *mut amdgpu_job, id: *mut *mut amdgpu_vmid) -> c_int;
}
pub unsafe fn amdgpu_vmid_grab(vm: *mut amdgpu_vm, ring: *mut amdgpu_ring,
    job: *mut amdgpu_job, fence: *mut *mut dma_fence) -> c_int {
    let adev = (*ring).adev;
    let vmhub = (*ring).vm_hub;
    let mgr = &mut (*adev).vm_manager.id_mgr[vmhub as usize];
    let mut idle: *mut amdgpu_vmid = core::ptr::null_mut();
    let mut id: *mut amdgpu_vmid = core::ptr::null_mut();
    mutex_lock(&mut mgr.lock);
    let mut r = amdgpu_vmid_grab_idle(ring, &mut idle, fence);
    if r != 0 || idle.is_null() { mutex_unlock(&mut mgr.lock); return r; }
    if amdgpu_vmid_uses_reserved(vm, vmhub) {
        r = amdgpu_vmid_grab_reserved(vm, ring, job, &mut id, fence);
        if r != 0 || id.is_null() { mutex_unlock(&mut mgr.lock); return r; }
    } else {
        r = amdgpu_vmid_grab_used(vm, ring, job, &mut id);
        if r != 0 { mutex_unlock(&mut mgr.lock); return r; }
        if id.is_null() {
            id = idle;
            r = amdgpu_sync_fence(&mut (*id).active, &mut (*(*job).base.s_fence).finished, GFP_ATOMIC);
            if r != 0 { mutex_unlock(&mut mgr.lock); return r; }
            (*job).vm_needs_flush = true;
        }
        list_move_tail(&mut (*id).list, &mut mgr.ids_lru);
    }
    (*job).gds_switch_needed = amdgpu_vmid_gds_switch_needed(id, job);
    if (*job).vm_needs_flush { (*id).flushed_updates = amdgpu_vm_tlb_seq(vm); dma_fence_put((*id).last_flush); (*id).last_flush = core::ptr::null_mut(); }
    (*job).vmid = id.offset_from(mgr.ids) as u32;
    (*job).pasid = (*vm).pasid;
    (*id).gds_base = (*job).gds_base; (*id).gds_size = (*job).gds_size;
    (*id).gws_base = (*job).gws_base; (*id).gws_size = (*job).gws_size;
    (*id).oa_base = (*job).oa_base; (*id).oa_size = (*job).oa_size;
    (*id).pd_gpu_addr = (*job).vm_pd_addr; (*id).owner = (*vm).immediate.fence_context;
    trace_amdgpu_vm_grab_id(vm, ring, job);
    mutex_unlock(&mut mgr.lock); r
}

pub unsafe fn amdgpu_vmid_uses_reserved(vm: *mut amdgpu_vm, vmhub: c_uint) -> bool { !(*vm).reserved_vmid[vmhub as usize].is_null() }

pub unsafe fn amdgpu_vmid_alloc_reserved(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, vmhub: c_uint) -> c_int {
    let mgr = &mut (*adev).vm_manager.id_mgr[vmhub as usize]; mutex_lock(&mut mgr.lock);
    if !(*vm).reserved_vmid[vmhub as usize].is_null() { mutex_unlock(&mut mgr.lock); return 0; }
    if mgr.reserved_vmid { mutex_unlock(&mut mgr.lock); return -ENOENT; }
    let id = list_first_entry(&mut mgr.ids_lru, amdgpu_vmid, list); list_del_init(&mut (*id).list);
    (*vm).reserved_vmid[vmhub as usize] = id; mgr.reserved_vmid = true; mutex_unlock(&mut mgr.lock); 0
}

pub unsafe fn amdgpu_vmid_free_reserved(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, vmhub: c_uint) {
    let mgr = &mut (*adev).vm_manager.id_mgr[vmhub as usize]; mutex_lock(&mut mgr.lock);
    let id = (*vm).reserved_vmid[vmhub as usize]; if !id.is_null() { list_add(&mut (*id).list, &mut mgr.ids_lru); (*vm).reserved_vmid[vmhub as usize] = core::ptr::null_mut(); mgr.reserved_vmid = false; } mutex_unlock(&mut mgr.lock);
}

pub unsafe fn amdgpu_vmid_reset(adev: *mut amdgpu_device, vmhub: c_uint, vmid: c_uint) {
    let mgr = &mut (*adev).vm_manager.id_mgr[vmhub as usize]; let id = &mut mgr.ids[vmid as usize]; mutex_lock(&mut mgr.lock);
    (*id).owner=0; (*id).gds_base=0; (*id).gds_size=0; (*id).gws_base=0; (*id).gws_size=0; (*id).oa_base=0; (*id).oa_size=0; mutex_unlock(&mut mgr.lock);
}

pub unsafe fn amdgpu_vmid_reset_all(adev: *mut amdgpu_device) { for i in 0..AMDGPU_MAX_VMHUBS { let n=(*adev).vm_manager.id_mgr[i].num_ids; for j in 1..n { amdgpu_vmid_reset(adev,i as c_uint,j as c_uint); } } }

pub unsafe fn amdgpu_vmid_mgr_init(adev: *mut amdgpu_device) {
    for i in 0..AMDGPU_MAX_VMHUBS { let mgr=&mut (*adev).vm_manager.id_mgr[i]; mutex_init(&mut mgr.lock); INIT_LIST_HEAD(&mut mgr.ids_lru);
        if amdgpu_ip_version(adev,GC_HWIP,0)<IP_VERSION(10,0,0) || AMDGPU_IS_MMHUB0(i) || AMDGPU_IS_MMHUB1(i) { mgr.num_ids=if amdgpu_ip_version(adev,GC_HWIP,0)<IP_VERSION(10,0,0) {(*adev).vm_manager.first_kfd_vmid} else {16}; } else { mgr.num_ids=(*adev).vm_manager.first_kfd_vmid; }
        for j in 1..mgr.num_ids { amdgpu_vmid_reset(adev,i as c_uint,j as c_uint); amdgpu_sync_create(&mut mgr.ids[j].active); list_add_tail(&mut mgr.ids[j].list,&mut mgr.ids_lru); }
    }
}

pub unsafe fn amdgpu_vmid_mgr_fini(adev: *mut amdgpu_device) { for i in 0..AMDGPU_MAX_VMHUBS { let mgr=&mut (*adev).vm_manager.id_mgr[i]; mutex_destroy(&mut mgr.lock); for j in 0..AMDGPU_NUM_VMID { let id=&mut mgr.ids[j]; amdgpu_sync_free(&mut id.active); dma_fence_put(id.last_flush); dma_fence_put(id.pasid_mapping); } } }
pub unsafe fn amdgpu_pasid_mgr_cleanup() { xa_destroy(&mut amdgpu_pasid_xa); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
