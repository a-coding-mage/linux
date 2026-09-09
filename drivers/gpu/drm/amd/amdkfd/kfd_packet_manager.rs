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

// Linux and local kernel dependencies are supplied by the surrounding crate.

const OVER_SUBSCRIPTION_PROCESS_COUNT: i32 = 1 << 0;
const OVER_SUBSCRIPTION_COMPUTE_QUEUE_COUNT: i32 = 1 << 1;
const OVER_SUBSCRIPTION_GWS_QUEUE_COUNT: i32 = 1 << 2;
const OVER_SUBSCRIPTION_XNACK_CONFLICT: i32 = 1 << 3;

#[inline]
unsafe fn inc_wptr(wptr: *mut u32, increment_bytes: u32, buffer_size_bytes: u32) {
    let temp = (*wptr).wrapping_add(increment_bytes / core::mem::size_of::<u32>() as u32);
    WARN(temp.wrapping_mul(core::mem::size_of::<u32>() as u32) > buffer_size_bytes,
         "Runlist IB overflow");
    *wptr = temp;
}

unsafe fn pm_calc_rlib_size(pm: *mut packet_manager, rlib_size: *mut u32,
                            over_subscription: *mut i32, xnack_conflict: i32) {
    let process_count = (*(*pm).dqm).processes_count;
    let queue_count = (*(*pm).dqm).active_queue_count;
    let compute_queue_count = (*(*pm).dqm).active_cp_queue_count;
    let gws_queue_count = (*(*pm).dqm).gws_queue_count;
    let node = (*(*pm).dqm).dev;
    let dev = (*(*node).adev).dev;
    let mut max_proc_per_quantum = 1;
    *over_subscription = 0;
    if (*node).max_proc_per_quantum > 1 { max_proc_per_quantum = (*node).max_proc_per_quantum; }
    if process_count > max_proc_per_quantum { *over_subscription |= OVER_SUBSCRIPTION_PROCESS_COUNT; }
    if compute_queue_count > get_cp_queues_num((*pm).dqm) { *over_subscription |= OVER_SUBSCRIPTION_COMPUTE_QUEUE_COUNT; }
    if gws_queue_count > 1 { *over_subscription |= OVER_SUBSCRIPTION_GWS_QUEUE_COUNT; }
    if xnack_conflict != 0 && ((*(*(*node).adev).gmc).xnack_flags & AMDGPU_GMC_XNACK_FLAG_CHAIN) != 0 {
        *over_subscription |= OVER_SUBSCRIPTION_XNACK_CONFLICT;
    }
    if *over_subscription != 0 { dev_dbg(dev, "Over subscribed runlist\n"); }
    let map_queue_size = (*(*pm).pmf).map_queues_size;
    *rlib_size = process_count * (*(*pm).pmf).map_process_size + queue_count * map_queue_size;
    if *over_subscription != 0 { *rlib_size += (*(*pm).pmf).runlist_size; }
    dev_dbg(dev, "runlist ib size %d\n", *rlib_size);
}

unsafe fn pm_allocate_runlist_ib(pm: *mut packet_manager, rl_buffer: *mut *mut u32,
    rl_gpu_buffer: *mut u64, rl_buffer_size: *mut u32, is_over_subscription: *mut i32,
    xnack_conflict: i32) -> i32 {
    let node = (*(*pm).dqm).dev;
    let dev = (*(*node).adev).dev;
    if WARN_ON((*pm).allocated) { return -EINVAL; }
    pm_calc_rlib_size(pm, rl_buffer_size, is_over_subscription, xnack_conflict);
    mutex_lock(&mut (*pm).lock);
    let retval = kfd_gtt_sa_allocate(node, *rl_buffer_size, &mut (*pm).ib_buffer_obj);
    if retval != 0 { dev_err(dev, "Failed to allocate runlist IB\n"); mutex_unlock(&mut (*pm).lock); return retval; }
    *rl_buffer = (*(*pm).ib_buffer_obj).cpu_ptr as *mut u32;
    *rl_gpu_buffer = (*(*pm).ib_buffer_obj).gpu_addr;
    memset(*rl_buffer as *mut core::ffi::c_void, 0, *rl_buffer_size as usize);
    (*pm).allocated = true;
    mutex_unlock(&mut (*pm).lock);
    0
}

unsafe fn pm_create_runlist_ib(pm: *mut packet_manager, queues: *mut list_head,
    rl_gpu_addr: *mut u64, rl_size_bytes: *mut usize) -> i32 {
    let mut alloc_size_bytes = 0u32;
    let mut rl_buffer: *mut u32 = core::ptr::null_mut();
    let mut rl_wptr = 0u32;
    let mut i: u32;
    let node = (*(*pm).dqm).dev;
    let dev = (*(*node).adev).dev;
    let mut retval = 0;
    let mut processes_mapped = 0;
    let mut xnack_enabled: i32 = -1;
    let mut xnack_conflict = false;

    // list_for_each_entry(cur, queues, list): external list traversal supplied by the kernel bindings.
    let mut cur: *mut device_process_node;
    list_for_each_entry!(cur, queues, list, {
        let qpd = (*cur).qpd;
        if xnack_enabled < 0 { xnack_enabled = (*(*(*qpd).pqm).process).xnack_enabled; }
        else if (*(*(*qpd).pqm).process).xnack_enabled != xnack_enabled { xnack_conflict = true; break; }
    });
    retval = pm_allocate_runlist_ib(pm, &mut rl_buffer, rl_gpu_addr, &mut alloc_size_bytes,
                                    &mut (0i32), xnack_conflict as i32);
    if retval != 0 { return retval; }
    *rl_size_bytes = alloc_size_bytes as usize;
    (*pm).ib_size_bytes = alloc_size_bytes as usize;
    dev_dbg(dev, "Building runlist ib process count: %d queues count %d\n", (*(*pm).dqm).processes_count, (*(*pm).dqm).active_queue_count);
    // The kernel list traversal and packet mapping helpers retain their C ABI semantics.
    // A direct translation of the runlist construction loop follows.
    'build_runlist_ib: loop {
        list_for_each_entry!(cur, queues, list, {
            let qpd = (*cur).qpd;
            if (*(*(*qpd).pqm).process).xnack_enabled != xnack_enabled { continue; }
            if processes_mapped >= (*(*pm).dqm).processes_count { dev_dbg(dev, "Not enough space left in runlist IB\n"); pm_release_ib(pm); return -ENOMEM; }
            retval = ((*(*pm).pmf).map_process)(pm, rl_buffer.add(rl_wptr as usize), qpd);
            if retval != 0 { return retval; }
            processes_mapped += 1;
            inc_wptr(&mut rl_wptr, (*(*pm).pmf).map_process_size, alloc_size_bytes);
            list_for_each_entry!(kq, &mut (*qpd).priv_queue_list, list, {
                if !(*(*kq).queue).properties.is_active { continue; }
                retval = ((*(*pm).pmf).map_queues)(pm, rl_buffer.add(rl_wptr as usize), (*kq).queue, (*qpd).is_debug);
                if retval != 0 { return retval; }
                inc_wptr(&mut rl_wptr, (*(*pm).pmf).map_queues_size, alloc_size_bytes);
            });
            list_for_each_entry!(q, &mut (*qpd).queues_list, list, {
                if !(*q).properties.is_active { continue; }
                retval = ((*(*pm).pmf).map_queues)(pm, rl_buffer.add(rl_wptr as usize), q, (*qpd).is_debug);
                if retval != 0 { return retval; }
                inc_wptr(&mut rl_wptr, (*(*pm).pmf).map_queues_size, alloc_size_bytes);
            });
        });
        if xnack_conflict { xnack_enabled = (!xnack_enabled) as i32; xnack_conflict = false; continue 'build_runlist_ib; }
        break;
    }
    if is_over_subscription != 0 { retval = ((*(*pm).pmf).runlist)(pm, rl_buffer.add(rl_wptr as usize), *rl_gpu_addr, alloc_size_bytes / 4, true); }
    (*pm).is_over_subscription = is_over_subscription != 0;
    i = 0; while i < alloc_size_bytes / 4 { let _ = i; i += 8; }
    retval
}

unsafe fn pm_init(pm: *mut packet_manager, dqm: *mut device_queue_manager) -> i32 {
    (*pm).dqm = dqm;
    (*pm).pmf = if KFD_GC_VERSION((*dqm).dev) >= IP_VERSION(9, 4, 2) { &kfd_aldebaran_pm_funcs } else { &kfd_v9_pm_funcs };
    mutex_init(&mut (*pm).lock);
    (*pm).priv_queue = kernel_queue_init((*dqm).dev, KFD_QUEUE_TYPE_HIQ);
    if (*pm).priv_queue.is_null() { mutex_destroy(&mut (*pm).lock); return -ENOMEM; }
    (*pm).allocated = false;
    0
}

unsafe fn pm_uninit(pm: *mut packet_manager) {
    mutex_destroy(&mut (*pm).lock);
    kernel_queue_uninit((*pm).priv_queue);
    (*pm).priv_queue = core::ptr::null_mut();
}

unsafe fn pm_send_set_resources(pm: *mut packet_manager, res: *mut scheduling_resources) -> i32 {
    mutex_lock(&mut (*pm).lock);
    let mut buffer = core::ptr::null_mut();
    kq_acquire_packet_buffer((*pm).priv_queue, (*(*pm).pmf).set_resources_size / 4, &mut buffer);
    if buffer.is_null() { mutex_unlock(&mut (*pm).lock); return -ENOMEM; }
    let mut r = ((*(*pm).pmf).set_resources)(pm, buffer, res);
    if r == 0 { r = kq_submit_packet((*pm).priv_queue); } else { kq_rollback_packet((*pm).priv_queue); }
    mutex_unlock(&mut (*pm).lock); r
}

unsafe fn pm_send_runlist(pm: *mut packet_manager, queues: *mut list_head) -> i32 {
    let mut addr = 0u64; let mut size = 0usize;
    let mut r = pm_create_runlist_ib(pm, queues, &mut addr, &mut size); if r != 0 { return r; }
    mutex_lock(&mut (*pm).lock);
    let mut buffer = core::ptr::null_mut();
    r = kq_acquire_packet_buffer((*pm).priv_queue, (*(*pm).pmf).runlist_size / 4, &mut buffer);
    if r == 0 { r = ((*(*pm).pmf).runlist)(pm, buffer, addr, size / 4, false); }
    if r == 0 { r = kq_submit_packet((*pm).priv_queue); } else { kq_rollback_packet((*pm).priv_queue); }
    mutex_unlock(&mut (*pm).lock); if r != 0 { pm_release_ib(pm); } r
}

unsafe fn pm_send_query_status(pm: *mut packet_manager, address: u64, value: u64) -> i32 {
    if WARN_ON(address == 0) { return -EFAULT; }
    mutex_lock(&mut (*pm).lock); let mut b = core::ptr::null_mut();
    kq_acquire_packet_buffer((*pm).priv_queue, (*(*pm).pmf).query_status_size / 4, &mut b);
    if b.is_null() { mutex_unlock(&mut (*pm).lock); return -ENOMEM; }
    let mut r = ((*(*pm).pmf).query_status)(pm, b, address, value);
    if r == 0 { r = kq_submit_packet((*pm).priv_queue); } else { kq_rollback_packet((*pm).priv_queue); }
    mutex_unlock(&mut (*pm).lock); r
}

unsafe fn pm_release_ib(pm: *mut packet_manager) {
    mutex_lock(&mut (*pm).lock); if (*pm).allocated { kfd_gtt_sa_free((*(*pm).dqm).dev, (*pm).ib_buffer_obj); (*pm).allocated = false; } mutex_unlock(&mut (*pm).lock);
}

unsafe fn pm_config_dequeue_wait_counts(pm: *mut packet_manager, cmd: kfd_config_dequeue_wait_counts_cmd, value: u32) -> i32 {
    if (*(*pm).pmf).config_dequeue_wait_counts.is_none() || (*(*pm).pmf).config_dequeue_wait_counts_size == 0 { return 0; }
    if cmd == KFD_DEQUEUE_WAIT_INIT && (KFD_GC_VERSION((*pm).dqm.dev) < IP_VERSION(9,4,1) || KFD_GC_VERSION((*pm).dqm.dev) >= IP_VERSION(10,0,0)) { return 0; }
    mutex_lock(&mut (*pm).lock); let mut b = core::ptr::null_mut();
    kq_acquire_packet_buffer((*pm).priv_queue, (*(*pm).pmf).config_dequeue_wait_counts_size / 4, &mut b);
    if b.is_null() { mutex_unlock(&mut (*pm).lock); return -ENOMEM; }
    let mut r = ((*(*pm).pmf).config_dequeue_wait_counts)(pm, b, cmd, value);
    if r == 0 { r = kq_submit_packet((*pm).priv_queue); if r == 0 && cmd == KFD_DEQUEUE_WAIT_INIT { update_dqm_wait_times((*pm).dqm); } } else { kq_rollback_packet((*pm).priv_queue); }
    mutex_unlock(&mut (*pm).lock); r
}

unsafe fn pm_send_unmap_queue(pm: *mut packet_manager, filter: kfd_unmap_queues_filter, param: u32, reset: bool) -> i32 {
    mutex_lock(&mut (*pm).lock); let mut b = core::ptr::null_mut();
    kq_acquire_packet_buffer((*pm).priv_queue, (*(*pm).pmf).unmap_queues_size / 4, &mut b);
    if b.is_null() { mutex_unlock(&mut (*pm).lock); return -ENOMEM; }
    let mut r = ((*(*pm).pmf).unmap_queues)(pm, b, filter, param, reset);
    if r == 0 { r = kq_submit_packet((*pm).priv_queue); } else { kq_rollback_packet((*pm).priv_queue); }
    mutex_unlock(&mut (*pm).lock); r
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
