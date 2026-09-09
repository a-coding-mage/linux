/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright 2020-2022 Advanced Micro Devices, Inc.
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

// External C types supplied by the surrounding kernel translation.
extern "C" {
    pub fn kfd_smi_event_open(dev: *mut kfd_node, fd: *mut u32) -> i32;
    pub fn kfd_smi_event_update_vmfault(dev: *mut kfd_node, pasid: u16);
    pub fn kfd_smi_event_update_thermal_throttling(
        dev: *mut kfd_node,
        throttle_bitmask: u64,
    );
    pub fn kfd_smi_event_update_gpu_reset(
        dev: *mut kfd_node,
        post_reset: bool,
        reset_context: *mut amdgpu_reset_context,
    );
    pub fn kfd_smi_event_page_fault_start(
        node: *mut kfd_node,
        task: *mut task_struct,
        address: usize,
        write_fault: bool,
        ts: ktime_t,
    );
    pub fn kfd_smi_event_page_fault_end(
        node: *mut kfd_node,
        task: *mut task_struct,
        address: usize,
        migration: bool,
    );
    pub fn kfd_smi_event_migration_start(
        node: *mut kfd_node,
        task: *mut task_struct,
        start: usize,
        end: usize,
        from: u32,
        to: u32,
        prefetch_loc: u32,
        preferred_loc: u32,
        trigger: u32,
    );
    pub fn kfd_smi_event_migration_end(
        node: *mut kfd_node,
        task: *mut task_struct,
        start: usize,
        end: usize,
        from: u32,
        to: u32,
        trigger: u32,
        error_code: i32,
    );
    pub fn kfd_smi_event_queue_eviction(
        node: *mut kfd_node,
        task: *mut task_struct,
        trigger: u32,
    );
    pub fn kfd_smi_event_queue_restore(node: *mut kfd_node, task: *mut task_struct);
    pub fn kfd_smi_event_queue_restore_rescheduled(mm: *mut mm_struct);
    pub fn kfd_smi_event_unmap_from_gpu(
        node: *mut kfd_node,
        task: *mut task_struct,
        address: usize,
        last: usize,
        trigger: u32,
    );
    pub fn kfd_smi_event_process(pdd: *mut kfd_process_device, start: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
