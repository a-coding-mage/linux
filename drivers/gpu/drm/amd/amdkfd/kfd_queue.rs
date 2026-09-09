// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2014-2022 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn print_queue_properties(q: *mut queue_properties) {
    if q.is_null() { return; }
    pr_debug!("Printing queue properties:\n");
    pr_debug!("Queue Type: %u\n", (*q).type_);
    pr_debug!("Queue Size: %llu\n", (*q).queue_size);
    pr_debug!("Queue percent: %u\n", (*q).queue_percent);
    pr_debug!("Queue Address: 0x%llX\n", (*q).queue_address);
    pr_debug!("Queue Id: %u\n", (*q).queue_id);
    pr_debug!("Queue Process Vmid: %u\n", (*q).vmid);
    pr_debug!("Queue Read Pointer: 0x%px\n", (*q).read_ptr);
    pr_debug!("Queue Write Pointer: 0x%px\n", (*q).write_ptr);
    pr_debug!("Queue Doorbell Pointer: 0x%p\n", (*q).doorbell_ptr);
    pr_debug!("Queue Doorbell Offset: %u\n", (*q).doorbell_off);
}

pub unsafe fn print_queue(q: *mut queue) {
    if q.is_null() { return; }
    pr_debug!("Printing queue:\n");
    print_queue_properties(&mut (*q).properties);
    pr_debug!("Queue MQD Address: 0x%p\n", (*q).mqd);
    pr_debug!("Queue MQD Gart: 0x%llX\n", (*q).gart_mqd_addr);
    pr_debug!("Queue Process Address: 0x%p\n", (*q).process);
    pr_debug!("Queue Device Address: 0x%p\n", (*q).device);
}

pub unsafe fn init_queue(q: *mut *mut queue, properties: *const queue_properties) -> i32 {
    let tmp_q = kzalloc_obj::<queue>();
    if tmp_q.is_null() { return -ENOMEM; }
    core::ptr::copy_nonoverlapping(properties, &mut (*tmp_q).properties, 1);
    *q = tmp_q;
    0
}

pub unsafe fn uninit_queue(q: *mut queue) { kfree(q); }

#[cfg(CONFIG_HSA_AMD_SVM)]
unsafe fn kfd_queue_buffer_svm_get(pdd: *mut kfd_process_device, mut addr: u64, mut size: u64) -> i32 {
    let p = (*pdd).process;
    let mut update_list = list_head::default();
    let mut ret = -EINVAL;
    INIT_LIST_HEAD(&mut update_list);
    addr >>= PAGE_SHIFT; size >>= PAGE_SHIFT;
    mutex_lock(&mut (*p).svms.lock);
    while size != 0 {
        let mut gpuid = 0u32; let mut gpuidx = 0u32;
        let prange = svm_range_from_addr(&mut (*p).svms, addr, core::ptr::null_mut());
        if prange.is_null() { break; }
        if kfd_process_gpuid_from_node(p, (*pdd).dev, &mut gpuid, &mut gpuidx) < 0 { break; }
        if !test_bit(gpuidx, (*prange).bitmap_mapped) ||
           (!test_bit(gpuidx, (*prange).bitmap_access) && !test_bit(gpuidx, (*prange).bitmap_aip)) ||
           ((*prange).flags & KFD_IOCTL_SVM_FLAG_GPU_ALWAYS_MAPPED) == 0 { break; }
        list_add(&mut (*prange).update_list, &mut update_list);
        let length = (*prange).last - (*prange).start + 1;
        if length >= size { size = 0; break; }
        size -= length; addr += length;
    }
    if size != 0 { pr_debug!("[0x%llx 0x%llx] not registered\n", addr, addr + size - 1); mutex_unlock(&mut (*p).svms.lock); return ret; }
    list_for_each_entry!(prange, &mut update_list, update_list, { atomic_inc(&mut (*prange).queue_refcount); });
    ret = 0; mutex_unlock(&mut (*p).svms.lock); ret
}

#[cfg(not(CONFIG_HSA_AMD_SVM))]
unsafe fn kfd_queue_buffer_svm_get(_: *mut kfd_process_device, _: u64, _: u64) -> i32 { -EINVAL }

#[cfg(CONFIG_HSA_AMD_SVM)]
unsafe fn kfd_queue_buffer_svm_put(pdd: *mut kfd_process_device, mut addr: u64, size: u64) {
    let p = (*pdd).process; addr >>= PAGE_SHIFT;
    let last = addr + (size >> PAGE_SHIFT) - 1;
    mutex_lock(&mut (*p).svms.lock);
    let mut node = interval_tree_iter_first(&mut (*p).svms.objects, addr, last);
    while !node.is_null() {
        let next_node = interval_tree_iter_next(node, addr, last);
        let next_start = core::cmp::min((*node).last, last) + 1;
        let prange = container_of!(node, svm_range, it_node);
        if atomic_add_unless(&mut (*prange).queue_refcount, -1, 0) {
            list_for_each_entry!(pchild, &mut (*prange).child_list, child_list, { atomic_add_unless(&mut (*pchild).queue_refcount, -1, 0); });
        }
        node = next_node; addr = next_start;
    }
    mutex_unlock(&mut (*p).svms.lock);
}
#[cfg(not(CONFIG_HSA_AMD_SVM))]
unsafe fn kfd_queue_buffer_svm_put(_: *mut kfd_process_device, _: u64, _: u64) {}

pub unsafe fn kfd_queue_buffer_get(vm: *mut amdgpu_vm, addr: *mut core::ffi::c_void, pbo: *mut *mut amdgpu_bo, expected_size: u64) -> i32 {
    let user_addr = addr as u64 >> AMDGPU_GPU_PAGE_SHIFT;
    let size = expected_size >> AMDGPU_GPU_PAGE_SHIFT;
    let mapping = amdgpu_vm_bo_lookup_mapping(vm, user_addr);
    if mapping.is_null() || user_addr != (*mapping).start || (size != 0 && user_addr + size - 1 != (*mapping).last) {
        *pbo = core::ptr::null_mut(); return -EINVAL;
    }
    *pbo = amdgpu_bo_ref((*(*mapping).bo_va).base.bo); (*(*mapping).bo_va).queue_refcount += 1; 0
}

// FIXME: remove this function, just call amdgpu_bo_unref directly
pub unsafe fn kfd_queue_buffer_put(bo: *mut *mut amdgpu_bo) { amdgpu_bo_unref(bo); }

pub unsafe fn kfd_queue_acquire_buffers(pdd: *mut kfd_process_device, properties: *mut queue_properties) -> i32 {
    let topo_dev = kfd_topology_device_by_id((*(*pdd).dev).id);
    if topo_dev.is_null() { return -EINVAL; }
    let expected_queue_size = if (*properties).type_ == KFD_QUEUE_TYPE_COMPUTE && (*properties).format == KFD_QUEUE_FORMAT_AQL && (*topo_dev).node_props.gfx_target_version >= 70000 && (*topo_dev).node_props.gfx_target_version < 90000 {
        PAGE_ALIGN((*properties).queue_size / 2)
    } else { PAGE_ALIGN((*properties).queue_size + (*properties).metadata_queue_size) };
    let vm = drm_priv_to_vm((*pdd).drm_priv);
    let mut err = amdgpu_bo_reserve((*vm).root.bo, false);
    if err != 0 { return err; }
    err = kfd_queue_buffer_get(vm, (*properties).write_ptr, &mut (*properties).wptr_bo, PAGE_SIZE);
    if err != 0 { amdgpu_bo_unreserve((*vm).root.bo); return err; }
    err = kfd_queue_buffer_get(vm, (*properties).read_ptr, &mut (*properties).rptr_bo, PAGE_SIZE);
    if err != 0 { amdgpu_bo_unreserve((*vm).root.bo); kfd_queue_unref_bo_vas(pdd, properties); kfd_queue_release_buffers(pdd, properties); return err; }
    err = kfd_queue_buffer_get(vm, (*properties).queue_address as *mut _, &mut (*properties).ring_bo, expected_queue_size);
    if err != 0 { amdgpu_bo_unreserve((*vm).root.bo); kfd_queue_unref_bo_vas(pdd, properties); kfd_queue_release_buffers(pdd, properties); return err; }
    if (*properties).type_ != KFD_QUEUE_TYPE_COMPUTE { amdgpu_bo_unreserve((*vm).root.bo); return 0; }
    if (*properties).eop_ring_buffer_address != 0 {
        if (*properties).eop_ring_buffer_size < (*topo_dev).node_props.eop_buffer_size { amdgpu_bo_unreserve((*vm).root.bo); kfd_queue_unref_bo_vas(pdd, properties); kfd_queue_release_buffers(pdd, properties); return -EINVAL; }
        err = kfd_queue_buffer_get(vm, (*properties).eop_ring_buffer_address as *mut _, &mut (*properties).eop_buf_bo, ALIGN((*properties).eop_ring_buffer_size as u64, PAGE_SIZE));
        if err != 0 { amdgpu_bo_unreserve((*vm).root.bo); kfd_queue_unref_bo_vas(pdd, properties); kfd_queue_release_buffers(pdd, properties); return err; }
    }
    if (*properties).ctl_stack_size != (*topo_dev).node_props.ctl_stack_size || (*properties).ctx_save_restore_area_size < (*topo_dev).node_props.cwsr_size { amdgpu_bo_unreserve((*vm).root.bo); kfd_queue_unref_bo_vas(pdd, properties); kfd_queue_release_buffers(pdd, properties); return -EINVAL; }
    let mut total = (*properties).ctx_save_restore_area_size as u64 + (*topo_dev).node_props.debug_memory_size as u64;
    if check_mul_overflow(total, NUM_XCC((*pdd).dev).xcc_mask, &mut total) { amdgpu_bo_unreserve((*vm).root.bo); kfd_queue_unref_bo_vas(pdd, properties); kfd_queue_release_buffers(pdd, properties); return -EINVAL; }
    total = ALIGN(total, PAGE_SIZE);
    err = kfd_queue_buffer_get(vm, (*properties).ctx_save_restore_area_address as *mut _, &mut (*properties).cwsr_bo, total);
    if err == 0 { amdgpu_bo_unreserve((*vm).root.bo); return 0; }
    amdgpu_bo_unreserve((*vm).root.bo);
    err = kfd_queue_buffer_svm_get(pdd, (*properties).ctx_save_restore_area_address, total);
    if err != 0 { kfd_queue_unref_bo_vas(pdd, properties); kfd_queue_release_buffers(pdd, properties); }
    err
}

pub unsafe fn kfd_queue_release_buffers(pdd: *mut kfd_process_device, properties: *mut queue_properties) -> i32 {
    kfd_queue_buffer_put(&mut (*properties).wptr_bo); kfd_queue_buffer_put(&mut (*properties).rptr_bo); kfd_queue_buffer_put(&mut (*properties).ring_bo); kfd_queue_buffer_put(&mut (*properties).eop_buf_bo); kfd_queue_buffer_put(&mut (*properties).cwsr_bo);
    let topo = kfd_topology_device_by_id((*(*pdd).dev).id); if topo.is_null() { return -EINVAL; }
    let mut total = (*properties).ctx_save_restore_area_size as u64 + (*topo).node_props.debug_memory_size as u64;
    if check_mul_overflow(total, NUM_XCC((*pdd).dev).xcc_mask, &mut total) { return -EINVAL; }
    kfd_queue_buffer_svm_put(pdd, (*properties).ctx_save_restore_area_address, ALIGN(total, PAGE_SIZE)); 0
}

pub unsafe fn kfd_queue_unref_bo_va(vm: *mut amdgpu_vm, bo: *mut *mut amdgpu_bo) {
    if !(*bo).is_null() { let bo_va = amdgpu_vm_bo_find(vm, *bo); if !bo_va.is_null() && (*bo_va).queue_refcount != 0 { (*bo_va).queue_refcount -= 1; } }
}

pub unsafe fn kfd_queue_unref_bo_vas(pdd: *mut kfd_process_device, properties: *mut queue_properties) -> i32 {
    let vm = drm_priv_to_vm((*pdd).drm_priv); let err = amdgpu_bo_reserve((*vm).root.bo, false); if err != 0 { return err; }
    kfd_queue_unref_bo_va(vm, &mut (*properties).wptr_bo); kfd_queue_unref_bo_va(vm, &mut (*properties).rptr_bo); kfd_queue_unref_bo_va(vm, &mut (*properties).ring_bo); kfd_queue_unref_bo_va(vm, &mut (*properties).eop_buf_bo); kfd_queue_unref_bo_va(vm, &mut (*properties).cwsr_bo);
    amdgpu_bo_unreserve((*vm).root.bo); 0
}

const DEBUGGER_BYTES_ALIGN: u32 = 64;
const DEBUGGER_BYTES_PER_WAVE: u32 = 32;
const SIZEOF_HSA_USER_CONTEXT_SAVE_AREA_HEADER: u32 = 40;

unsafe fn kfd_get_sgpr_size_per_cu(gfxv: u32) -> u32 { if gfxv == 120500 || gfxv == 120501 { 0x8000 } else { 0x4000 } }
unsafe fn kfd_get_vgpr_size_per_cu(gfxv: u32) -> u32 {
    if [90402,90010,90008,90500,120500,120501].contains(&gfxv) { 0x80000 }
    else if [110000,110001,110501,120000,120001].contains(&gfxv) { 0x60000 } else { 0x40000 }
}
unsafe fn kfd_get_hwreg_size_per_cu(gfxv: u32) -> u32 { if gfxv == 120500 || gfxv == 120501 { 0x8000 } else { 0x1000 } }
unsafe fn kfd_get_lds_size_per_cu(gfxv: u32, props: *mut kfd_node_properties) -> u32 { if gfxv == 90500 || gfxv == 120500 || gfxv == 120501 { (*props).lds_size_in_kb << 10 } else { 0x10000 } }
unsafe fn get_num_waves(props: *mut kfd_node_properties, gfxv: u32, cu_num: u32) -> u32 {
    let n = if gfxv < 100100 { core::cmp::min(cu_num * 40, (*props).array_count / (*props).simd_arrays_per_engine * 512) } else if gfxv < 120500 { cu_num * 32 } else if gfxv <= 120501 { cu_num * 64 } else { 0 };
    WARN_ON!(n == 0); n
}

pub unsafe fn kfd_queue_ctx_save_restore_size(dev: *mut kfd_topology_device) {
    let props = &mut (*dev).node_props; let gfxv = props.gfx_target_version;
    if gfxv < 80001 { return; }
    let cu_num = props.simd_count / props.simd_per_cu / NUM_XCC((*dev).gpu).xcc_mask;
    let wave_num = get_num_waves(props, gfxv, cu_num);
    let wg_data_size = ALIGN(cu_num * (kfd_get_vgpr_size_per_cu(gfxv) + kfd_get_sgpr_size_per_cu(gfxv) + kfd_get_lds_size_per_cu(gfxv, props) + kfd_get_hwreg_size_per_cu(gfxv)), AMDGPU_GPU_PAGE_SIZE);
    let mut ctl_stack_size = wave_num * if gfxv >= 100100 { 12 } else { 8 } + 8;
    ctl_stack_size = ALIGN(SIZEOF_HSA_USER_CONTEXT_SAVE_AREA_HEADER + ctl_stack_size, AMDGPU_GPU_PAGE_SIZE);
    if gfxv / 10000 * 10000 == 100000 { ctl_stack_size = core::cmp::min(ctl_stack_size, 0x7000); }
    props.ctl_stack_size = ctl_stack_size;
    props.debug_memory_size = ALIGN(wave_num * DEBUGGER_BYTES_PER_WAVE, DEBUGGER_BYTES_ALIGN);
    props.cwsr_size = ALIGN(ctl_stack_size + wg_data_size, PAGE_SIZE);
    if gfxv == 80002 { props.eop_buffer_size = 0x8000; } else if gfxv == 90402 || gfxv >= 80000 { props.eop_buffer_size = 4096; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
