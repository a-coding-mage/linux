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
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// External kernel and amdkfd declarations are supplied by the surrounding translation.

const PM4_COUNT_ZERO: u32 = (((1u32 << 15) - 1) << 16);

unsafe fn kq_initialize(
    kq: *mut kernel_queue,
    dev: *mut kfd_node,
    queue_type: kfd_queue_type,
    queue_size: c_uint,
) -> bool {
    let mut prop: queue_properties = core::mem::zeroed();
    let mut nop: PM4_MES_TYPE_3_HEADER = core::mem::zeroed();

    pr_debug!("Initializing queue type {} size {}\n", queue_type, queue_size);
    nop.opcode = IT_NOP;
    nop.type_ = PM4_TYPE_3;
    nop.u32all |= PM4_COUNT_ZERO;

    (*kq).dev = dev;
    (*kq).nop_packet = nop.u32all;
    (*kq).mqd_mgr = (*(*dev).dqm).mqd_mgrs[KFD_MQD_TYPE_HIQ as usize];
    if (*kq).mqd_mgr.is_null() { return false; }

    prop.doorbell_ptr = kfd_get_kernel_doorbell((*dev).kfd, &mut prop.doorbell_off);
    if prop.doorbell_ptr.is_null() {
        dev_err!((*(*dev).adev).dev, "Failed to initialize doorbell");
        return false;
    }

    let mut retval = kfd_gtt_sa_allocate(dev, queue_size, &mut (*kq).pq);
    if retval != 0 {
        dev_err!((*(*dev).adev).dev, "Failed to init pq queues size {}\n", queue_size);
        kfd_release_kernel_doorbell((*dev).kfd, prop.doorbell_ptr);
        return false;
    }
    (*kq).pq_kernel_addr = (*(*kq).pq).cpu_ptr;
    (*kq).pq_gpu_addr = (*(*kq).pq).gpu_addr;

    if (*(*dev).adev).asic_type > CHIP_MULLINS {
        retval = kfd_gtt_sa_allocate(dev, PAGE_SIZE, &mut (*kq).eop_mem);
        if retval != 0 {
            kfd_gtt_sa_free(dev, (*kq).pq);
            kfd_release_kernel_doorbell((*dev).kfd, prop.doorbell_ptr);
            return false;
        }
        (*kq).eop_gpu_addr = (*(*kq).eop_mem).gpu_addr;
        (*kq).eop_kernel_addr = (*(*kq).eop_mem).cpu_ptr;
        core::ptr::write_bytes((*kq).eop_kernel_addr, 0, PAGE_SIZE);
    }

    retval = kfd_gtt_sa_allocate(dev, core::mem::size_of::<*mut u32>(), &mut (*kq).rptr_mem);
    if retval != 0 { kfd_gtt_sa_free(dev, (*kq).eop_mem); kfd_gtt_sa_free(dev, (*kq).pq); kfd_release_kernel_doorbell((*dev).kfd, prop.doorbell_ptr); return false; }
    (*kq).rptr_kernel = (*(*kq).rptr_mem).cpu_ptr as *mut u32;
    (*kq).rptr_gpu_addr = (*(*kq).rptr_mem).gpu_addr;

    retval = kfd_gtt_sa_allocate(dev, (*(*dev).kfd).device_info.doorbell_size, &mut (*kq).wptr_mem);
    if retval != 0 { kfd_gtt_sa_free(dev, (*kq).rptr_mem); kfd_gtt_sa_free(dev, (*kq).eop_mem); kfd_gtt_sa_free(dev, (*kq).pq); kfd_release_kernel_doorbell((*dev).kfd, prop.doorbell_ptr); return false; }
    (*kq).wptr_kernel = (*(*kq).wptr_mem).cpu_ptr as *mut u32;
    (*kq).wptr64_kernel = (*kq).wptr_kernel as *mut u64;
    (*kq).wptr_gpu_addr = (*(*kq).wptr_mem).gpu_addr;
    core::ptr::write_bytes((*kq).pq_kernel_addr, 0, queue_size as usize);
    core::ptr::write_bytes((*kq).rptr_kernel, 0, core::mem::size_of::<u32>());
    core::ptr::write_bytes((*kq).wptr_kernel, 0, (*(*dev).kfd).device_info.doorbell_size as usize);

    prop.queue_size = queue_size; prop.is_interop = false; prop.is_gws = false;
    prop.priority = 1; prop.queue_percent = 100; prop.type_ = queue_type; prop.vmid = 0;
    prop.queue_address = (*kq).pq_gpu_addr; prop.read_ptr = (*kq).rptr_gpu_addr as *mut u32;
    prop.write_ptr = (*kq).wptr_gpu_addr as *mut u32; prop.eop_ring_buffer_address = (*kq).eop_gpu_addr; prop.eop_ring_buffer_size = PAGE_SIZE;
    if init_queue(&mut (*kq).queue, &mut prop) != 0 { kfd_gtt_sa_free(dev, (*kq).wptr_mem); kfd_gtt_sa_free(dev, (*kq).rptr_mem); kfd_gtt_sa_free(dev, (*kq).eop_mem); kfd_gtt_sa_free(dev, (*kq).pq); kfd_release_kernel_doorbell((*dev).kfd, prop.doorbell_ptr); return false; }
    (*(*kq).queue).device = dev;
    (*(*kq).queue).mqd_mem_obj = ((*(*kq).mqd_mgr).allocate_mqd)((*kq).mqd_mgr, &(*(*kq).queue).properties);
    if (*(*kq).queue).mqd_mem_obj.is_null() { uninit_queue((*kq).queue); return false; }
    ((*(*kq).mqd_mgr).init_mqd)((*kq).mqd_mgr, &mut (*(*kq).queue).mqd, (*(*kq).queue).mqd_mem_obj, &mut (*(*kq).queue).gart_mqd_addr, &(*(*kq).queue).properties);
    if queue_type == KFD_QUEUE_TYPE_HIQ {
        pr_debug!("Assigning hiq to hqd\n");
        (*(*kq).queue).pipe = KFD_CIK_HIQ_PIPE; (*(*kq).queue).queue = KFD_CIK_HIQ_QUEUE;
        ((*(*kq).mqd_mgr).load_mqd)((*kq).mqd_mgr, (*(*kq).queue).mqd, (*(*kq).queue).pipe, (*(*kq).queue).queue, &(*(*kq).queue).properties, core::ptr::null_mut());
    }
    print_queue((*kq).queue); true
}

unsafe fn kq_uninitialize(kq: *mut kernel_queue) {
    if (*(*kq).queue).properties.type_ == KFD_QUEUE_TYPE_HIQ && down_read_trylock(&mut (*(*(*kq).dev).adev).reset_domain.sem) != 0 {
        ((*(*kq).mqd_mgr).destroy_mqd)((*kq).mqd_mgr, (*(*kq).queue).mqd, KFD_PREEMPT_TYPE_WAVEFRONT_RESET, KFD_UNMAP_LATENCY_MS, (*(*kq).queue).pipe, (*(*kq).queue).queue); up_read(&mut (*(*(*kq).dev).adev).reset_domain.sem);
    }
    ((*(*kq).mqd_mgr).free_mqd)((*kq).mqd_mgr, (*(*kq).queue).mqd, (*(*kq).queue).mqd_mem_obj);
    kfd_gtt_sa_free((*kq).dev, (*kq).rptr_mem); kfd_gtt_sa_free((*kq).dev, (*kq).wptr_mem); kfd_gtt_sa_free((*kq).dev, (*kq).eop_mem); kfd_gtt_sa_free((*kq).dev, (*kq).pq);
    kfd_release_kernel_doorbell((*(*kq).dev).kfd, (*(*kq).queue).properties.doorbell_ptr); uninit_queue((*kq).queue);
}

pub unsafe fn kq_acquire_packet_buffer(kq: *mut kernel_queue, packet_size_in_dwords: usize, buffer_ptr: *mut *mut c_uint) -> c_int {
    let mut rptr = *(*kq).rptr_kernel; let mut wptr = (*kq).pending_wptr; let mut wptr64 = (*kq).pending_wptr64;
    let queue_size_dwords = (*(*kq).queue).properties.queue_size / 4; let queue_address = (*kq).pq_kernel_addr as *mut c_uint;
    let available_size = (rptr + queue_size_dwords - 1 - wptr) % queue_size_dwords;
    if packet_size_in_dwords > available_size { *buffer_ptr = core::ptr::null_mut(); return -ENOMEM; }
    if wptr + packet_size_in_dwords >= queue_size_dwords {
        if packet_size_in_dwords >= rptr { *buffer_ptr = core::ptr::null_mut(); return -ENOMEM; }
        while wptr > 0 { *queue_address.add(wptr as usize) = (*kq).nop_packet; wptr = (wptr + 1) % queue_size_dwords; wptr64 += 1; }
    }
    *buffer_ptr = queue_address.add(wptr as usize); (*kq).pending_wptr = wptr + packet_size_in_dwords as u32; (*kq).pending_wptr64 = wptr64 + packet_size_in_dwords as u64; 0
}

pub unsafe fn kq_submit_packet(kq: *mut kernel_queue) -> c_int {
    if amdgpu_amdkfd_is_fed((*(*kq).dev).adev) { return -EIO; }
    mb();
    if (*(*kq).dev).kfd.device_info.doorbell_size == 8 { *(*kq).wptr64_kernel = (*kq).pending_wptr64; mb(); write_kernel_doorbell64((*(*kq).queue).properties.doorbell_ptr, (*kq).pending_wptr64); }
    else { *(*kq).wptr_kernel = (*kq).pending_wptr; mb(); write_kernel_doorbell((*(*kq).queue).properties.doorbell_ptr, (*kq).pending_wptr); } 0
}

pub unsafe fn kq_rollback_packet(kq: *mut kernel_queue) { if (*(*kq).dev).kfd.device_info.doorbell_size == 8 { (*kq).pending_wptr64 = *(*kq).wptr64_kernel; (*kq).pending_wptr = *(*kq).wptr_kernel % ((*(*kq).queue).properties.queue_size / 4); } else { (*kq).pending_wptr = *(*kq).wptr_kernel; } }

pub unsafe fn kernel_queue_init(dev: *mut kfd_node, queue_type: kfd_queue_type) -> *mut kernel_queue { let kq = kzalloc_obj::<kernel_queue>(); if kq.is_null() { return core::ptr::null_mut(); } if kq_initialize(kq, dev, queue_type, KFD_KERNEL_QUEUE_SIZE) { return kq; } dev_err!((*(*dev).adev).dev, "Failed to init kernel queue\n"); kfree(kq); core::ptr::null_mut() }

pub unsafe fn kernel_queue_uninit(kq: *mut kernel_queue) { kq_uninitialize(kq); kfree(kq); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
