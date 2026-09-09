// SPDX-License-Identifier: GPL-2.0+
/*
 * VAS Fault handling.
 * Copyright 2019, IBM Corporation
 */

// Dependency declarations and build-time kernel configuration are supplied by
// the surrounding translation unit.

const VAS_FAULT_WIN_FIFO_SIZE: usize = 4 << 20;

unsafe fn dump_fifo(vinst: *mut vas_instance, entry: *mut core::ffi::c_void) {
    let end = (*vinst).fault_fifo.add((*vinst).fault_fifo_size as usize);
    let mut fifo = entry as *mut usize;
    let mut i = 0;

    pr_err!("Fault fifo size %d, Max crbs %d\n", (*vinst).fault_fifo_size,
        (*vinst).fault_fifo_size / CRB_SIZE);
    pr_err!("Fault FIFO Dump:\n");
    while i < 10 * (CRB_SIZE / 8) && fifo < end {
        pr_err!("[{:03}, {:p}]: 0x{:016x} 0x{:016x} 0x{:016x} 0x{:016x}\n",
            i, fifo, *fifo, *fifo.add(1), *fifo.add(2), *fifo.add(3));
        i += 4;
        fifo = fifo.add(4);
    }
}

pub unsafe extern "C" fn vas_fault_thread_fn(
    _irq: i32, data: *mut core::ffi::c_void,
) -> irqreturn_t {
    let vinst = data as *mut vas_instance;
    let mut buf: coprocessor_request_block = core::mem::zeroed();
    let crb = &mut buf as *mut coprocessor_request_block;
    let mut flags: unsigned_long = 0;

    loop {
        spin_lock_irqsave(&mut (*vinst).fault_lock, &mut flags);
        let fifo = (*vinst).fault_fifo.add(
            ((*vinst).fault_crbs * CRB_SIZE) as usize) as *mut coprocessor_request_block;
        let entry = fifo;

        if (*entry).stamp.nx.pswid == cpu_to_be32(FIFO_INVALID_ENTRY)
            || ((*entry).ccw & cpu_to_be32(CCW0_INVALID)) != 0 {
            (*vinst).fifo_in_progress = 0;
            spin_unlock_irqrestore(&mut (*vinst).fault_lock, flags);
            return IRQ_HANDLED;
        }
        spin_unlock_irqrestore(&mut (*vinst).fault_lock, flags);

        (*vinst).fault_crbs += 1;
        if (*vinst).fault_crbs == (*vinst).fault_fifo_size / CRB_SIZE {
            (*vinst).fault_crbs = 0;
        }
        memcpy(crb as *mut core::ffi::c_void, fifo as *const core::ffi::c_void, CRB_SIZE);
        (*entry).stamp.nx.pswid = cpu_to_be32(FIFO_INVALID_ENTRY);
        (*entry).ccw |= cpu_to_be32(CCW0_INVALID);
        vas_return_credit((*vinst).fault_win, false);
        pr_devel!("VAS[%d] fault_fifo %p, fifo %p, fault_crbs %d\n",
            (*vinst).vas_id, (*vinst).fault_fifo, fifo, (*vinst).fault_crbs);
        vas_dump_crb(crb);
        let window = vas_pswid_to_window(vinst, be32_to_cpu((*crb).stamp.nx.pswid));

        if IS_ERR(window) {
            dump_fifo(vinst, entry as *mut core::ffi::c_void);
            pr_err!("VAS[%d] fault_fifo %p, fifo %p, pswid 0x%x, fault_crbs %d bad CRB?\n",
                (*vinst).vas_id, (*vinst).fault_fifo, fifo,
                be32_to_cpu((*crb).stamp.nx.pswid), (*vinst).fault_crbs);
            WARN_ON_ONCE!(1);
        } else {
            if (*window).user_win {
                vas_update_csb(crb, &mut (*window).vas_win.task_ref);
            } else {
                WARN_ON_ONCE!(!(*window).user_win);
            }
            vas_return_credit(window, true);
        }
    }
}

pub unsafe extern "C" fn vas_fault_handler(
    _irq: i32, dev_id: *mut core::ffi::c_void,
) -> irqreturn_t {
    let vinst = dev_id as *mut vas_instance;
    let mut ret = IRQ_WAKE_THREAD;
    let mut flags: unsigned_long = 0;
    spin_lock_irqsave(&mut (*vinst).fault_lock, &mut flags);
    if (*vinst).fifo_in_progress {
        ret = IRQ_HANDLED;
    } else {
        (*vinst).fifo_in_progress = 1;
    }
    spin_unlock_irqrestore(&mut (*vinst).fault_lock, flags);
    ret
}

pub unsafe extern "C" fn vas_setup_fault_window(vinst: *mut vas_instance) -> i32 {
    let mut attr: vas_rx_win_attr = core::mem::zeroed();
    (*vinst).fault_fifo_size = VAS_FAULT_WIN_FIFO_SIZE as i32;
    (*vinst).fault_fifo = kzalloc((*vinst).fault_fifo_size, GFP_KERNEL);
    if (*vinst).fault_fifo.is_null() {
        pr_err!("Unable to alloc %d bytes for fault_fifo\n", (*vinst).fault_fifo_size);
        return -ENOMEM;
    }
    memset((*vinst).fault_fifo, FIFO_INVALID_ENTRY, (*vinst).fault_fifo_size);
    vas_init_rx_win_attr(&mut attr, VAS_COP_TYPE_FAULT);
    attr.rx_fifo_size = (*vinst).fault_fifo_size;
    attr.rx_fifo = __pa((*vinst).fault_fifo);
    attr.wcreds_max = (*vinst).fault_fifo_size / CRB_SIZE;
    attr.lnotify_lpid = 0;
    attr.lnotify_pid = mfspr(SPRN_PID);
    attr.lnotify_tid = mfspr(SPRN_PID);
    let win = vas_rx_win_open((*vinst).vas_id, VAS_COP_TYPE_FAULT, &mut attr);
    if IS_ERR(win) {
        pr_err!("VAS: Error %ld opening FaultWin\n", PTR_ERR(win));
        kfree((*vinst).fault_fifo);
        return PTR_ERR(win) as i32;
    }
    (*vinst).fault_win = container_of(win, pnv_vas_window, vas_win);
    pr_devel!("VAS: Created FaultWin %d, LPID/PID/TID [%d/%d/%d]\n",
        (*vinst).fault_win.vas_win.winid, attr.lnotify_lpid,
        attr.lnotify_pid, attr.lnotify_tid);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
