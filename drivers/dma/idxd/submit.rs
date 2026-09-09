// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2019 Intel Corporation. All rights rsvd. */

// Linux/kernel dependencies supplied by the surrounding translation unit.

unsafe fn __get_desc(wq: *mut idxd_wq, idx: i32, cpu: i32) -> *mut idxd_desc {
    let desc = (*wq).descs.add(idx as usize).read();
    let idxd = (*wq).idxd;

    core::ptr::write_bytes((*desc).hw, 0, core::mem::size_of::<dsa_hw_desc>());
    core::ptr::write_bytes((*desc).completion, 0, (*(*idxd).data).compl_size as usize);
    (*desc).cpu = cpu;

    if device_pasid_enabled(idxd) {
        (*(*desc).hw).pasid = (*idxd).pasid;
    }
    desc
}

pub unsafe fn idxd_alloc_desc(wq: *mut idxd_wq, optype: idxd_op_type) -> *mut idxd_desc {
    let idxd = (*wq).idxd;
    let mut wait = DEFINE_SBQ_WAIT!();
    let sbq = &mut (*wq).sbq;
    let mut cpu: i32 = 0;
    let mut idx = sbitmap_queue_get(sbq, &mut cpu);

    if (*idxd).state != IDXD_DEV_ENABLED {
        return ERR_PTR(-EIO);
    }

    if idx < 0 {
        if optype == IDXD_OP_NONBLOCK {
            return ERR_PTR(-EAGAIN);
        }
    } else {
        return __get_desc(wq, idx, cpu);
    }

    let ws = (*sbq).ws;
    loop {
        sbitmap_prepare_to_wait(sbq, ws, &mut wait, TASK_INTERRUPTIBLE);
        if signal_pending_state(TASK_INTERRUPTIBLE, current) {
            break;
        }
        idx = sbitmap_queue_get(sbq, &mut cpu);
        if idx >= 0 {
            break;
        }
        schedule();
    }

    sbitmap_finish_wait(sbq, ws, &mut wait);
    if idx < 0 {
        return ERR_PTR(-EAGAIN);
    }
    __get_desc(wq, idx, cpu)
}

pub unsafe fn idxd_free_desc(wq: *mut idxd_wq, desc: *mut idxd_desc) {
    let cpu = (*desc).cpu;
    (*desc).cpu = -1;
    sbitmap_queue_clear(&mut (*wq).sbq, (*desc).id, cpu);
}

unsafe fn list_abort_desc(_wq: *mut idxd_wq, ie: *mut idxd_irq_entry,
                          desc: *mut idxd_desc) -> *mut idxd_desc {
    lockdep_assert_held(&(*ie).list_lock);
    let mut d = list_first_entry(&(*ie).work_list);
    while !d.is_null() {
        let next = list_next_entry(d);
        if d == desc {
            list_del(&mut (*d).list);
            return d;
        }
        d = next;
    }
    /* The completion handler may hold the descriptor between pending and work lists. */
    core::ptr::null_mut()
}

unsafe fn llist_abort_desc(wq: *mut idxd_wq, ie: *mut idxd_irq_entry,
                           desc: *mut idxd_desc) {
    let mut flist = LIST_HEAD!();
    (*(*desc).completion).status = IDXD_COMP_DESC_ABORT;

    spin_lock(&mut (*ie).list_lock);
    let head = llist_del_all(&mut (*ie).pending_llist);
    let mut found: *mut idxd_desc = core::ptr::null_mut();
    if !head.is_null() {
        let mut d = llist_first_entry(head);
        while !d.is_null() {
            let next = llist_next_entry(d);
            if d == desc {
                found = desc;
            } else if (*(*d).completion).status != 0 {
                list_add_tail(&mut (*d).list, &mut flist);
            } else {
                list_add_tail(&mut (*d).list, &mut (*ie).work_list);
            }
            d = next;
        }
    }
    if found.is_null() {
        found = list_abort_desc(wq, ie, desc);
    }
    spin_unlock(&mut (*ie).list_lock);

    if !found.is_null() {
        idxd_dma_complete_txd(found, IDXD_COMPLETE_ABORT, false, core::ptr::null_mut(), core::ptr::null_mut());
    }
    let mut d = list_first_entry(&flist);
    while !d.is_null() {
        let next = list_next_entry(d);
        list_del_init(&mut (*d).list);
        idxd_dma_complete_txd(d, IDXD_COMPLETE_ABORT, true, core::ptr::null_mut(), core::ptr::null_mut());
        d = next;
    }
}

pub unsafe fn idxd_enqcmds(wq: *mut idxd_wq, portal: *mut core::ffi::c_void,
                           desc: *const core::ffi::c_void) -> i32 {
    let mut retries = (*wq).enqcmds_retries;
    let mut rc;
    loop {
        rc = enqcmds(portal, desc);
        if rc == 0 { break; }
        cpu_relax();
        if retries == 0 { break; }
        retries = retries.wrapping_sub(1);
    }
    rc
}

pub unsafe fn idxd_submit_desc(wq: *mut idxd_wq, desc: *mut idxd_desc) -> i32 {
    let idxd = (*wq).idxd;
    let mut ie: *mut idxd_irq_entry = core::ptr::null_mut();
    let desc_flags = (*(*desc).hw).flags;
    let portal;
    if (*idxd).state != IDXD_DEV_ENABLED { return -EIO; }
    if !percpu_ref_tryget_live(&mut (*wq).wq_active) {
        wait_for_completion(&mut (*wq).wq_resurrect);
        if !percpu_ref_tryget_live(&mut (*wq).wq_active) { return -ENXIO; }
    }
    portal = idxd_wq_portal_addr(wq);
    if desc_flags & IDXD_OP_FLAG_RCI != 0 {
        ie = &mut (*wq).ie;
        (*(*desc).hw).int_handle = (*ie).int_handle;
        llist_add(&mut (*desc).llnode, &mut (*ie).pending_llist);
    }
    wmb();
    if wq_dedicated(wq) {
        iosubmit_cmds512(portal, (*desc).hw, 1);
    } else {
        let rc = idxd_enqcmds(wq, portal, (*desc).hw as *const _ as *const core::ffi::c_void);
        if rc < 0 {
            percpu_ref_put(&mut (*wq).wq_active);
            if !ie.is_null() { llist_abort_desc(wq, ie, desc); }
            return rc;
        }
    }
    percpu_ref_put(&mut (*wq).wq_active);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
