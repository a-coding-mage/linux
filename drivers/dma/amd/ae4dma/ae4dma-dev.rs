// SPDX-License-Identifier: GPL-2.0
/*
 * AMD AE4DMA driver
 *
 * Copyright (c) 2024, Advanced Micro Devices, Inc.
 * All Rights Reserved.
 *
 * Author: Basavaraj Natikar <Basavaraj.Natikar@amd.com>
 */

// Dependency supplied by ae4dma.h and the surrounding kernel/Rust bindings.

static mut max_hw_q: ::core::ffi::c_uint = 1;
// module_param(max_hw_q, uint, 0444);
// MODULE_PARM_DESC(max_hw_q, "max hw queues supported by engine (any non-zero value, default: 1)");

unsafe fn ae4_pending_work(work: *mut work_struct) {
    let ae4cmd_q = container_of(work, ae4_cmd_queue, p_work.work);
    let cmd_q: *mut pt_cmd_queue = &mut (*ae4cmd_q).cmd_q;
    let mut cmd: *mut pt_cmd;
    let mut cridx: u32;

    loop {
        wait_event_interruptible(
            (*ae4cmd_q).q_w,
            atomic64_read(&(*ae4cmd_q).done_cnt) < atomic64_read(&(*ae4cmd_q).intr_cnt),
        );

        atomic64_inc(&(*ae4cmd_q).done_cnt);

        mutex_lock(&(*ae4cmd_q).cmd_lock);
        cridx = readl((*cmd_q).reg_control.add(AE4_RD_IDX_OFF as usize));
        while (*ae4cmd_q).dridx != cridx && !list_empty(&(*ae4cmd_q).cmd) {
            cmd = list_first_entry(&(*ae4cmd_q).cmd, pt_cmd, entry);
            list_del(&mut (*cmd).entry);

            ae4_check_status_error(ae4cmd_q, (*ae4cmd_q).dridx);
            ((*cmd).pt_cmd_callback)((*cmd).data, (*cmd).ret);

            (*ae4cmd_q).q_cmd_count -= 1;
            (*ae4cmd_q).dridx = ((*ae4cmd_q).dridx + 1) % CMD_Q_LEN;

            complete_all(&mut (*ae4cmd_q).cmp);
        }
        mutex_unlock(&(*ae4cmd_q).cmd_lock);
    }
}

unsafe fn ae4_core_irq_handler(_irq: i32, data: *mut ::core::ffi::c_void) -> irqreturn_t {
    let ae4cmd_q = data as *mut ae4_cmd_queue;
    let cmd_q: *mut pt_cmd_queue;
    let pt: *mut pt_device;
    let mut status: u32;

    cmd_q = &mut (*ae4cmd_q).cmd_q;
    pt = (*cmd_q).pt;

    (*pt).total_interrupts += 1;
    atomic64_inc(&(*ae4cmd_q).intr_cnt);

    status = readl((*cmd_q).reg_control.add(AE4_INTR_STS_OFF as usize));
    if status & BIT(0) != 0 {
        status &= GENMASK(31, 1);
        writel(status, (*cmd_q).reg_control.add(AE4_INTR_STS_OFF as usize));
    }

    wake_up(&mut (*ae4cmd_q).q_w);

    IRQ_HANDLED
}

pub unsafe fn ae4_destroy_work(ae4: *mut ae4_device) {
    let mut ae4cmd_q: *mut ae4_cmd_queue;
    let mut i: i32;

    i = 0;
    while i < (*ae4).cmd_q_count {
        ae4cmd_q = &mut (*ae4).ae4cmd_q[i as usize];

        if (*ae4cmd_q).pws.is_null() {
            break;
        }

        cancel_delayed_work_sync(&mut (*ae4cmd_q).p_work);
        destroy_workqueue((*ae4cmd_q).pws);
        i += 1;
    }
}

pub unsafe fn ae4_core_init(ae4: *mut ae4_device) -> i32 {
    let pt: *mut pt_device = &mut (*ae4).pt;
    let mut ae4cmd_q: *mut ae4_cmd_queue;
    let dev: *mut device = (*pt).dev;
    let mut cmd_q: *mut pt_cmd_queue;
    let mut i: i32;
    let mut ret: i32 = 0;

    writel(max_hw_q, (*pt).io_regs);

    i = 0;
    while i < max_hw_q as i32 {
        ae4cmd_q = &mut (*ae4).ae4cmd_q[i as usize];
        (*ae4cmd_q).id = (*ae4).cmd_q_count;
        (*ae4).cmd_q_count += 1;

        cmd_q = &mut (*ae4cmd_q).cmd_q;
        (*cmd_q).pt = pt;
        (*cmd_q).reg_control = (*pt).io_regs.add(((i + 1) as usize) * AE4_Q_SZ as usize);

        ret = devm_request_irq(dev, (*ae4).ae4_irq[i as usize], ae4_core_irq_handler, 0,
                               dev_name((*pt).dev), ae4cmd_q as *mut ::core::ffi::c_void);
        if ret != 0 {
            return ret;
        }

        (*cmd_q).qsize = Q_SIZE(core::mem::size_of::<ae4dma_desc>());
        (*cmd_q).qbase = dmam_alloc_coherent(dev, (*cmd_q).qsize, &mut (*cmd_q).qbase_dma,
                                             GFP_KERNEL);
        if (*cmd_q).qbase.is_null() {
            return -ENOMEM;
        }
        i += 1;
    }

    i = 0;
    while i < (*ae4).cmd_q_count {
        ae4cmd_q = &mut (*ae4).ae4cmd_q[i as usize];
        cmd_q = &mut (*ae4cmd_q).cmd_q;
        (*cmd_q).reg_control = (*pt).io_regs.add(((i + 1) as usize) * AE4_Q_SZ as usize);

        // Update the device registers with queue information.
        writel(CMD_Q_LEN, (*cmd_q).reg_control.add(AE4_MAX_IDX_OFF as usize));
        (*cmd_q).qdma_tail = (*cmd_q).qbase_dma;
        writel(lower_32_bits((*cmd_q).qdma_tail), (*cmd_q).reg_control.add(AE4_Q_BASE_L_OFF as usize));
        writel(upper_32_bits((*cmd_q).qdma_tail), (*cmd_q).reg_control.add(AE4_Q_BASE_H_OFF as usize));

        INIT_LIST_HEAD(&mut (*ae4cmd_q).cmd);
        init_waitqueue_head(&mut (*ae4cmd_q).q_w);

        (*ae4cmd_q).pws = alloc_ordered_workqueue("ae4dma_%d", WQ_MEM_RECLAIM, (*ae4cmd_q).id);
        if (*ae4cmd_q).pws.is_null() {
            ae4_destroy_work(ae4);
            return -ENOMEM;
        }
        INIT_DELAYED_WORK(&mut (*ae4cmd_q).p_work, ae4_pending_work);
        queue_delayed_work((*ae4cmd_q).pws, &mut (*ae4cmd_q).p_work, usecs_to_jiffies(100));
        init_completion(&mut (*ae4cmd_q).cmp);
        i += 1;
    }

    ret = pt_dmaengine_register(pt);
    if ret != 0 {
        ae4_destroy_work(ae4);
    } else {
        ptdma_debugfs_setup(pt);
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
