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

#[no_mangle]
pub static mut pipe_priority_map: [i32; 16] = [
    KFD_PIPE_PRIORITY_CS_LOW, KFD_PIPE_PRIORITY_CS_LOW,
    KFD_PIPE_PRIORITY_CS_LOW, KFD_PIPE_PRIORITY_CS_LOW,
    KFD_PIPE_PRIORITY_CS_LOW, KFD_PIPE_PRIORITY_CS_LOW,
    KFD_PIPE_PRIORITY_CS_LOW, KFD_PIPE_PRIORITY_CS_MEDIUM,
    KFD_PIPE_PRIORITY_CS_MEDIUM, KFD_PIPE_PRIORITY_CS_MEDIUM,
    KFD_PIPE_PRIORITY_CS_MEDIUM, KFD_PIPE_PRIORITY_CS_HIGH,
    KFD_PIPE_PRIORITY_CS_HIGH, KFD_PIPE_PRIORITY_CS_HIGH,
    KFD_PIPE_PRIORITY_CS_HIGH, KFD_PIPE_PRIORITY_CS_HIGH,
];

pub unsafe fn allocate_hiq_mqd(mm: *mut mqd_manager, _q: *mut queue_properties) -> *mut kfd_mem_obj {
    let dev = (*mm).dev;
    let mqd_mem_obj = kzalloc_obj::<kfd_mem_obj>();
    if mqd_mem_obj.is_null() { return core::ptr::null_mut(); }
    (*mqd_mem_obj).mem = (*(*dev).dqm).hiq_sdma_mqd.mem;
    (*mqd_mem_obj).gpu_addr = (*(*dev).dqm).hiq_sdma_mqd.gpu_addr;
    (*mqd_mem_obj).cpu_ptr = (*(*dev).dqm).hiq_sdma_mqd.cpu_ptr;
    mqd_mem_obj
}

pub unsafe fn allocate_sdma_mqd(mm: *mut mqd_manager, q: *mut queue_properties) -> *mut kfd_mem_obj {
    let dev = (*mm).dev;
    let mqd_mem_obj = kzalloc_obj::<kfd_mem_obj>();
    if mqd_mem_obj.is_null() { return core::ptr::null_mut(); }
    let mut offset = ((*q).sdma_engine_id as u64 * (*(*dev).kfd).device_info.num_sdma_queues_per_engine as u64
        + (*q).sdma_queue_id as u64) * (*(*dev).dqm).mqd_mgrs[KFD_MQD_TYPE_SDMA].mqd_size;
    offset += (*(*dev).dqm).mqd_mgrs[KFD_MQD_TYPE_HIQ].mqd_size * NUM_XCC((*dev).xcc_mask);
    (*mqd_mem_obj).mem = ((*(*dev).dqm).hiq_sdma_mqd.mem as u64 + offset) as *mut core::ffi::c_void;
    (*mqd_mem_obj).gpu_addr = (*(*dev).dqm).hiq_sdma_mqd.gpu_addr + offset;
    (*mqd_mem_obj).cpu_ptr = ((*(*dev).dqm).hiq_sdma_mqd.cpu_ptr as u64 + offset) as *mut u32;
    mqd_mem_obj
}

pub unsafe fn free_mqd_hiq_sdma(_mm: *mut mqd_manager, _mqd: *mut core::ffi::c_void, obj: *mut kfd_mem_obj) {
    WARN_ON((*obj).mem.is_null());
    kfree(obj);
}

pub unsafe fn mqd_symmetrically_map_cu_mask(mm: *mut mqd_manager, cu_mask: *const u32, mut cu_mask_count: u32, se_mask: *mut u32, inst: u32) {
    let cu_info = &(*(*(*mm).dev).adev).gfx.cu_info;
    let gfx_info = &(*(*(*mm).dev).adev).gfx.config;
    let mut cu_per_sh = [[0u32; KFD_MAX_NUM_SH_PER_SE]; KFD_MAX_NUM_SE];
    let wgp_mode_req = KFD_GC_VERSION((*mm).dev) >= IP_VERSION(10, 0, 0);
    let en_mask = if wgp_mode_req { 3 } else { 1 };
    let cu_inc = if wgp_mode_req { 2 } else { 1 };
    let cu_active_per_node = cu_info.number / (*(*(*mm).dev).kfd).num_nodes;
    if cu_mask_count > cu_active_per_node { cu_mask_count = cu_active_per_node; }
    if gfx_info.max_shader_engines > KFD_MAX_NUM_SE { dev_err((*(*(*mm).dev).adev).dev, "Exceeded KFD_MAX_NUM_SE, chip reports %d\n", gfx_info.max_shader_engines); return; }
    if gfx_info.max_sh_per_se > KFD_MAX_NUM_SH_PER_SE { dev_err((*(*(*mm).dev).adev).dev, "Exceeded KFD_MAX_NUM_SH, chip reports %d\n", gfx_info.max_sh_per_se * gfx_info.max_shader_engines); return; }
    let cu_bitmap_sh_mul = if KFD_GC_VERSION((*mm).dev) >= IP_VERSION(11, 0, 0) && KFD_GC_VERSION((*mm).dev) < IP_VERSION(13, 0, 0) { 2 } else { 1 };
    let xcc_inst = inst + ffs((*(*mm).dev).xcc_mask) - 1;
    for se in 0..gfx_info.max_shader_engines as usize { for sh in 0..gfx_info.max_sh_per_se as usize { cu_per_sh[se][sh] = hweight32(cu_info.bitmap[xcc_inst as usize][se % 4][sh + (se / 4) * cu_bitmap_sh_mul as usize]); } }
    for i in 0..gfx_info.max_shader_engines as usize { *se_mask.add(i) = 0; }
    let mut i = inst;
    let inc = cu_inc * NUM_XCC((*mm).dev).try_into().unwrap();
    for cu in (0..16).step_by(cu_inc as usize) {
        for sh in 0..gfx_info.max_sh_per_se as usize { for se in 0..gfx_info.max_shader_engines as usize {
            if cu_per_sh[se][sh] > cu { if *cu_mask.add((i / 32) as usize) & (en_mask << (i % 32)) != 0 { *se_mask.add(se) |= en_mask << (cu + sh as u32 * 16); } i += inc; if i >= cu_mask_count { return; } }
        } }
    }
}

pub unsafe fn kfd_hiq_load_mqd_kiq(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, pipe_id: u32, queue_id: u32, p: *mut queue_properties, _mms: *mut mm_struct) -> i32 { ((*(*mm).dev).kfd2kgd).hiq_mqd_load((*mm).dev, mqd, pipe_id, queue_id, (*p).doorbell_off, 0) }
pub unsafe fn kfd_destroy_mqd_cp(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, ty: kfd_preempt_type, timeout: u32, pipe_id: u32, queue_id: u32) -> i32 { ((*(*mm).dev).kfd2kgd).hqd_destroy((*mm).dev, mqd, ty, timeout, pipe_id, queue_id, 0) }
pub unsafe fn kfd_free_mqd_cp(mm: *mut mqd_manager, _mqd: *mut core::ffi::c_void, obj: *mut kfd_mem_obj) { if !(*obj).mem.is_null() { amdgpu_amdkfd_free_kernel_mem((*mm).dev, &mut (*obj).mem); kfree(obj); } else { kfd_gtt_sa_free(mm, obj); } }
pub unsafe fn kfd_is_occupied_cp(mm: *mut mqd_manager, _mqd: *mut core::ffi::c_void, addr: u64, pipe_id: u32, queue_id: u32) -> bool { ((*(*mm).dev).kfd2kgd).hqd_is_occupied((*mm).dev, addr, pipe_id, queue_id, 0) }
pub unsafe fn kfd_load_mqd_sdma(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, _pipe_id: u32, _queue_id: u32, p: *mut queue_properties, mms: *mut mm_struct) -> i32 { ((*(*mm).dev).kfd2kgd).hqd_sdma_load((*mm).dev, mqd, (*p).write_ptr as *mut u32, mms) }
pub unsafe fn kfd_destroy_mqd_sdma(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, _ty: kfd_preempt_type, timeout: u32, _pipe_id: u32, _queue_id: u32) -> i32 { ((*(*mm).dev).kfd2kgd).hqd_sdma_destroy((*mm).dev, mqd, timeout) }
pub unsafe fn kfd_is_occupied_sdma(mm: *mut mqd_manager, mqd: *mut core::ffi::c_void, _addr: u64, _pipe_id: u32, _queue_id: u32) -> bool { ((*(*mm).dev).kfd2kgd).hqd_sdma_is_occupied((*mm).dev, mqd) }

pub unsafe fn kfd_hiq_mqd_stride(dev: *mut kfd_node) -> u64 { (*(*dev).dqm).mqd_mgrs[KFD_MQD_TYPE_HIQ].mqd_size }
pub unsafe fn kfd_get_hiq_xcc_mqd(dev: *mut kfd_node, obj: *mut kfd_mem_obj, virtual_xcc_id: u32) { let offset = kfd_hiq_mqd_stride(dev) * virtual_xcc_id as u64; (*obj).mem = if virtual_xcc_id == 0 { (*(*dev).dqm).hiq_sdma_mqd.mem } else { core::ptr::null_mut() }; (*obj).gpu_addr = (*(*dev).dqm).hiq_sdma_mqd.gpu_addr + offset; (*obj).cpu_ptr = ((*(*dev).dqm).hiq_sdma_mqd.cpu_ptr as u64 + offset) as *mut u32; }
pub unsafe fn kfd_mqd_stride(mm: *mut mqd_manager, _q: *mut queue_properties) -> u64 { if KFD_GC_VERSION((*mm).dev) >= IP_VERSION(11, 0, 0) { AMDGPU_MQD_SIZE_ALIGN((*mm).mqd_size) } else { (*mm).mqd_size } }
pub unsafe fn kfd_check_hiq_mqd_doorbell_id(node: *mut kfd_node, doorbell_id: u32, inst: u32) -> bool { if doorbell_id != 0 { let dev = (*(*node).adev).dev; if !(*node).adev.xcp_mgr.is_null() && (*(*node).adev).xcp_mgr.num_xcps > 0 { dev_err(dev, "XCC %d: Queue preemption failed for queue with doorbell_id: %x\n", inst, doorbell_id); } else { dev_err(dev, "Queue preemption failed for queue with doorbell_id: %x\n", doorbell_id); } true } else { false } }
pub unsafe fn mqd_on_vram(adev: *mut amdgpu_device) -> bool { if (*adev).apu_prefer_gtt { return false; } match amdgpu_ip_version(adev, GC_HWIP, 0) { IP_VERSION(9,4,2) | IP_VERSION(9,4,3) | IP_VERSION(9,4,4) | IP_VERSION(9,5,0) => true, _ => false } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
