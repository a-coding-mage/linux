/* SPDX-License-Identifier: GPL-2.0 OR MIT */
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel translation.
pub struct kfd_node;
pub struct mqd_manager;
pub struct queue;
pub struct kfd_mem_obj;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

/**
 * kq_acquire_packet_buffer: Returns a pointer to the location in the kernel
 * queue ring buffer where the calling function can write its packet. It is
 * Guaranteed that there is enough space for that packet. It also updates the
 * pending write pointer to that location so subsequent calls to
 * acquire_packet_buffer will get a correct write pointer
 *
 * kq_submit_packet: Update the write pointer and doorbell of a kernel queue.
 *
 * kq_rollback_packet: This routine is called if we failed to build an acquired
 * packet for some reason. It just overwrites the pending wptr with the current
 * one
 */

unsafe extern "C" {
    pub fn kq_acquire_packet_buffer(
        kq: *mut kernel_queue,
        packet_size_in_dwords: usize,
        buffer_ptr: *mut *mut u32,
    ) -> i32;
    pub fn kq_submit_packet(kq: *mut kernel_queue) -> i32;
    pub fn kq_rollback_packet(kq: *mut kernel_queue);
}

#[repr(C)]
pub union kernel_queue_wptr {
    pub wptr64_kernel: *mut u64,
    pub wptr_kernel: *mut u32,
}

#[repr(C)]
pub struct kernel_queue {
    /* data */
    pub dev: *mut kfd_node,
    pub mqd_mgr: *mut mqd_manager,
    pub queue: *mut queue,
    pub pending_wptr64: u64,
    pub pending_wptr: u32,
    pub nop_packet: u32,

    pub rptr_mem: *mut kfd_mem_obj,
    pub rptr_kernel: *mut u32,
    pub rptr_gpu_addr: u64,
    pub wptr_mem: *mut kfd_mem_obj,
    pub wptr: kernel_queue_wptr,
    pub wptr_gpu_addr: u64,
    pub pq: *mut kfd_mem_obj,
    pub pq_gpu_addr: u64,
    pub pq_kernel_addr: *mut u32,
    pub eop_mem: *mut kfd_mem_obj,
    pub eop_gpu_addr: u64,
    pub eop_kernel_addr: *mut u32,

    pub fence_mem_obj: *mut kfd_mem_obj,
    pub fence_gpu_addr: u64,
    pub fence_kernel_address: *mut core::ffi::c_void,

    pub list: list_head,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
