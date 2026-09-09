// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2016-2019 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

// Translated from habanalabs/common/hw_queue.c.  Kernel and driver types,
// constants, helpers, and callbacks are supplied by the surrounding crate.

#[inline]
pub fn hl_hw_queue_add_ptr(mut ptr: u32, val: u16) -> u32 {
    ptr = ptr.wrapping_add(val as u32);
    ptr &= (HL_QUEUE_LENGTH << 1) - 1;
    ptr
}

#[inline]
unsafe fn queue_ci_get(ci: *mut atomic_t, queue_len: u32) -> i32 {
    atomic_read(ci) & ((queue_len << 1) - 1) as i32
}

#[inline]
unsafe fn queue_free_slots(q: *mut hl_hw_queue, queue_len: u32) -> i32 {
    let delta = (*q).pi as i32 - queue_ci_get(&mut (*q).ci, queue_len);
    if delta >= 0 { queue_len as i32 - delta } else { delta.abs() - queue_len as i32 }
}

pub unsafe fn hl_hw_queue_update_ci(cs: *mut hl_cs) {
    let hdev = (*(*cs).ctx).hdev;
    if (*hdev).disabled { return; }
    let mut q = (*hdev).kernel_queues;
    if (*hdev).asic_prop.max_queues == 0 || (*q).queue_type == QUEUE_TYPE_HW { return; }
    for i in 0..(*hdev).asic_prop.max_queues as usize {
        if !cs_needs_completion(cs) || (*q).queue_type == QUEUE_TYPE_INT {
            atomic_add((*cs).jobs_in_queue_cnt.add(i), &mut (*q).ci);
        }
        q = q.add(1);
    }
}

pub unsafe fn hl_hw_queue_submit_bd(hdev: *mut hl_device, q: *mut hl_hw_queue,
                                    ctl: u32, len: u32, ptr: u64) {
    let bd = ((*q).kernel_address as *mut hl_bd).add(hl_pi_2_offset((*q).pi) as usize);
    (*bd).ctl = cpu_to_le32(ctl); (*bd).len = cpu_to_le32(len); (*bd).ptr = cpu_to_le64(ptr);
    if (*q).dram_bd {
        for i in 0..2usize {
            let addr = (*q).pq_dram_address +
                ((hl_pi_2_offset((*q).pi) as usize * core::mem::size_of::<hl_bd>() +
                  i * core::mem::size_of::<u64>()) as u64);
            ((*hdev).asic_funcs).access_dev_mem(hdev, PCI_REGION_DRAM, addr,
                (bd as *mut u64).add(i), DEBUGFS_WRITE64);
        }
    }
    (*q).pi = hl_queue_inc_ptr((*q).pi);
    ((*hdev).asic_funcs).ring_doorbell(hdev, (*q).hw_queue_id, (*q).pi);
}

unsafe fn ext_queue_sanity_checks(hdev: *mut hl_device, q: *mut hl_hw_queue,
                                  n: i32, reserve: bool) -> i32 {
    let free = &mut (*hdev).completion_queue[(*q).cq_id as usize].free_slots_cnt;
    if queue_free_slots(q, HL_QUEUE_LENGTH) < n { dev_dbg!((*hdev).dev, "Queue doesn't have room\n"); return -EAGAIN; }
    if reserve && atomic_add_negative(n * -1, free) { atomic_add(n, free); return -EAGAIN; }
    0
}

unsafe fn int_queue_sanity_checks(hdev: *mut hl_device, q: *mut hl_hw_queue, n: i32) -> i32 {
    if n as u32 > (*q).int_queue_len { dev_err!((*hdev).dev, "Cannot populate queue\n"); return -ENOMEM; }
    if queue_free_slots(q, (*q).int_queue_len) < n { return -EAGAIN; }
    0
}

unsafe fn hw_queue_sanity_checks(_hdev: *mut hl_device, q: *mut hl_hw_queue, n: i32) -> i32 {
    if queue_free_slots(q, HL_QUEUE_LENGTH) < n { return -EAGAIN; } 0
}

pub unsafe fn hl_hw_queue_send_cb_no_cmpl(hdev: *mut hl_device, id: u32, size: u32, ptr: u64) -> i32 {
    let q = (*hdev).kernel_queues.add(id as usize); ((*hdev).asic_funcs).hw_queues_lock(hdev);
    let mut rc = 0;
    if (*hdev).disabled { rc = -EPERM; } else {
        if (*q).queue_type != QUEUE_TYPE_HW { rc = ext_queue_sanity_checks(hdev, q, 1, false); }
        if rc == 0 { hl_hw_queue_submit_bd(hdev, q, 0, size, ptr); }
    }
    ((*hdev).asic_funcs).hw_queues_unlock(hdev); rc
}

// The remaining scheduling and lifecycle routines retain the C driver's
// callback-driven implementation and are declared with their native ABI.
extern "C" {
    pub fn hl_hw_queue_schedule_cs(cs: *mut hl_cs) -> i32;
    pub fn hl_hw_queue_inc_ci_kernel(hdev: *mut hl_device, hw_queue_id: u32);
    pub fn hl_hw_queues_create(hdev: *mut hl_device) -> i32;
    pub fn hl_hw_queues_destroy(hdev: *mut hl_device);
    pub fn hl_hw_queue_reset(hdev: *mut hl_device, hard_reset: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
