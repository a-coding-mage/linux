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

/* KFD Interrupts. */

const KFD_IH_NUM_ENTRIES: usize = 16384;

pub unsafe fn kfd_interrupt_init(node: *mut kfd_node) -> i32 {
    let mut r: i32;

    r = kfifo_alloc(
        &mut (*node).ih_fifo,
        KFD_IH_NUM_ENTRIES * (*(*node).kfd).device_info.ih_ring_entry_size,
        GFP_KERNEL,
    );
    if r != 0 {
        dev_err((*node).adev.dev, "Failed to allocate IH fifo\n");
        return r;
    }

    if (*(*node).kfd).ih_wq.is_null() {
        (*(*node).kfd).ih_wq = alloc_workqueue(
            "KFD IH",
            WQ_HIGHPRI | WQ_UNBOUND,
            (*(*node).kfd).num_nodes,
        );
        if unlikely((*(*node).kfd).ih_wq.is_null()) {
            kfifo_free(&mut (*node).ih_fifo);
            dev_err((*node).adev.dev, "Failed to allocate KFD IH workqueue\n");
            return -ENOMEM;
        }
    }
    spin_lock_init(&mut (*node).interrupt_lock);

    INIT_WORK(&mut (*node).interrupt_work, interrupt_wq);
    (*node).interrupts_active = true;

    /* Ensure the interrupt running on another processor sees the writes above. */
    smp_wmb();

    0
}

pub unsafe fn kfd_interrupt_exit(node: *mut kfd_node) {
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*node).interrupt_lock, &mut flags);
    (*node).interrupts_active = false;
    spin_unlock_irqrestore(&mut (*node).interrupt_lock, flags);
    kfifo_free(&mut (*node).ih_fifo);
}

/* Assumption: single reader/writer. This function is not re-entrant. */
pub unsafe fn enqueue_ih_ring_entry(node: *mut kfd_node, ih_ring_entry: *const c_void) -> bool {
    if kfifo_is_full(&(*node).ih_fifo) {
        dev_warn_ratelimited((*node).adev.dev, "KFD node %d ih_fifo overflow\n", (*node).node_id);
        return false;
    }

    kfifo_in(
        &mut (*node).ih_fifo,
        ih_ring_entry,
        (*(*node).kfd).device_info.ih_ring_entry_size,
    );
    true
}

/* Assumption: single reader/writer. This function is not re-entrant. */
unsafe fn dequeue_ih_ring_entry(node: *mut kfd_node, ih_ring_entry: *mut *mut u32) -> bool {
    let count: i32;

    if kfifo_is_empty(&(*node).ih_fifo) {
        return false;
    }

    count = kfifo_out_linear_ptr(
        &mut (*node).ih_fifo,
        ih_ring_entry,
        (*(*node).kfd).device_info.ih_ring_entry_size,
    );
    WARN_ON(count != (*(*node).kfd).device_info.ih_ring_entry_size as i32);
    count == (*(*node).kfd).device_info.ih_ring_entry_size as i32
}

unsafe extern "C" fn interrupt_wq(work: *mut work_struct) {
    let dev: *mut kfd_node = container_of!(work, kfd_node, interrupt_work);
    let mut ih_ring_entry: *mut u32 = core::ptr::null_mut();
    let start_jiffies: u64 = jiffies;

    while dequeue_ih_ring_entry(dev, &mut ih_ring_entry) {
        ((*(*(*dev).kfd).device_info.event_interrupt_class).interrupt_wq)(dev, ih_ring_entry);
        kfifo_skip_count(
            &mut (*dev).ih_fifo,
            (*(*dev).kfd).device_info.ih_ring_entry_size,
        );

        if time_is_before_jiffies(start_jiffies.wrapping_add(HZ)) {
            /* Reschedule after a second to avoid soft-lockup warnings. */
            queue_work((*(*dev).kfd).ih_wq, &mut (*dev).interrupt_work);
            break;
        }
    }
}

pub unsafe fn interrupt_is_wanted(
    dev: *mut kfd_node,
    ih_ring_entry: *const u32,
    patched_ihre: *mut u32,
    flag: *mut bool,
) -> bool {
    /* Integer and bitwise OR so there is no boolean short-circuiting. */
    let mut wanted: u32 = 0;
    wanted |= ((*(*(*dev).kfd).device_info.event_interrupt_class).interrupt_isr)(
        dev,
        ih_ring_entry,
        patched_ihre,
        flag,
    );
    wanted != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
