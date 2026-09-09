// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2019 Intel Corporation. All rights rsvd. */
// Kernel dependencies are supplied by the surrounding translated driver.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum irq_work_type { IRQ_WORK_NORMAL = 0, IRQ_WORK_PROCESS_FAULT }

#[repr(C)] pub struct idxd_resubmit { pub work: work_struct, pub desc: *mut idxd_desc }
#[repr(C)] pub struct idxd_int_handle_revoke { pub work: work_struct, pub idxd: *mut idxd_device }

unsafe fn idxd_device_reinit(work: *mut work_struct) {
    let idxd = container_of!(work, idxd_device, work);
    let dev = &(*(*idxd).pdev).dev;
    let mut rc: c_int;
    idxd_device_reset(idxd);
    rc = idxd_device_config(idxd); if rc < 0 { idxd_device_clear_state(idxd); return; }
    rc = idxd_device_enable(idxd); if rc < 0 { idxd_device_clear_state(idxd); return; }
    for i in 0..(*idxd).max_wqs {
        if test_bit(i, (*idxd).wq_enable_map) {
            let wq = *(*idxd).wqs.add(i as usize);
            rc = idxd_wq_enable(wq);
            if rc < 0 { clear_bit(i, (*idxd).wq_enable_map); dev_warn!(dev, "Unable to re-enable wq %s\n", dev_name(wq_confdev(wq))); }
        }
    }
}

/* The drain ensures all descriptors with this interrupt handle are flushed. */
unsafe fn idxd_int_handle_revoke_drain(ie: *mut idxd_irq_entry) {
    let wq = ie_to_wq(ie); let idxd = (*wq).idxd; let dev = &(*(*idxd).pdev).dev;
    let mut desc: dsa_hw_desc = core::mem::zeroed();
    desc.flags = IDXD_OP_FLAG_RCI; desc.opcode = DSA_OPCODE_DRAIN; desc.priv_ = 1;
    if (*ie).pasid != IOMMU_PASID_INVALID { desc.pasid = (*ie).pasid; }
    desc.int_handle = (*ie).int_handle;
    let portal = idxd_wq_portal_addr(wq); wmb();
    if wq_dedicated(wq) { iosubmit_cmds512(portal, &desc, 1); }
    else { let rc = idxd_enqcmds(wq, portal, &desc); if rc < 0 { dev_warn!(dev, "Failed to submit drain desc on wq %d\n", (*wq).id); } }
}

unsafe fn idxd_abort_invalid_int_handle_descs(ie: *mut idxd_irq_entry) {
    let mut flist = LIST_HEAD!(); let mut d: *mut idxd_desc; let mut t: *mut idxd_desc;
    spin_lock(&mut (*ie).list_lock); let head = llist_del_all(&mut (*ie).pending_llist);
    if !head.is_null() { llist_for_each_entry_safe!(d, t, head, llnode, { list_add_tail!(&mut (*d).list, &mut (*ie).work_list); }); }
    list_for_each_entry_safe!(d, t, &mut (*ie).work_list, list, {
        if (*(*d).completion).status == DSA_COMP_INT_HANDLE_INVAL { list_move_tail!(&mut (*d).list, &mut flist); }
    }); spin_unlock(&mut (*ie).list_lock);
    list_for_each_entry_safe!(d, t, &mut flist, list, { list_del!(&mut (*d).list); idxd_desc_complete(d, IDXD_COMPLETE_ABORT, true); });
}

unsafe fn idxd_int_handle_revoke(work: *mut work_struct) {
    let revoke = container_of!(work, idxd_int_handle_revoke, work); let idxd = (*revoke).idxd;
    let dev = &(*(*idxd).pdev).dev; if !(*idxd).request_int_handles { kfree(revoke); dev_warn!(dev, "Unexpected int handle refresh interrupt.\n"); return; }
    for i in 1..(*idxd).irq_cnt {
        let ie = idxd_get_ie(idxd, i); let wq = ie_to_wq(ie); if (*ie).int_handle == INVALID_INT_HANDLE { continue; }
        let mut new_handle = 0; let rc = idxd_device_request_int_handle(idxd, i, &mut new_handle, IDXD_IRQ_MSIX);
        if rc < 0 { dev_warn!(dev, "get int handle %d failed: %d\n", i, rc); (*ie).int_handle = INVALID_INT_HANDLE; idxd_wq_quiesce(wq); idxd_abort_invalid_int_handle_descs(ie); continue; }
        if (*ie).int_handle == new_handle { continue; }
        if (*wq).state != IDXD_WQ_ENABLED || (*wq).type_ != IDXD_WQT_KERNEL { (*ie).int_handle = new_handle; continue; }
        mutex_lock(&mut (*wq).wq_lock); reinit_completion(&mut (*wq).wq_resurrect); percpu_ref_kill(&mut (*wq).wq_active); wait_for_completion(&mut (*wq).wq_dead);
        (*ie).int_handle = new_handle; percpu_ref_reinit(&mut (*wq).wq_active); complete_all(&mut (*wq).wq_resurrect); mutex_unlock(&mut (*wq).wq_lock);
        if wq_dedicated(wq) { udelay(100); } idxd_int_handle_revoke_drain(ie);
    } kfree(revoke);
}

unsafe fn idxd_evl_fault_work(work: *mut work_struct) {
    let fault = container_of!(work, idxd_evl_fault, work); let wq = (*fault).wq; let idxd = (*wq).idxd; let dev = &(*(*idxd).pdev).dev;
    let evl = (*idxd).evl; let entry = (*fault).entry; let cr = (entry as *mut u8).add((*idxd).data).add((*idxd).data.evl_cr_off) as *mut c_void;
    let cr_size = (*idxd).data.compl_size; let status = (cr as *mut u8).add((*idxd).data.cr_status_off); let result = (cr as *mut u8).add((*idxd).data.cr_result_off); let mut copy_size;
    match (*fault).status {
        DSA_COMP_CRA_XLAT => { if (*entry).batch && (*entry).first_err_in_batch { (*evl).batch_fail[(*entry).batch_id as usize] = false; } copy_size = cr_size; idxd_user_counter_increment(wq, (*entry).pasid, COUNTER_FAULTS); }
        DSA_COMP_BATCH_EVL_ERR => { let bf = &mut (*evl).batch_fail[(*entry).batch_id as usize]; copy_size = if (*entry).rcr || *bf { cr_size } else { 0 }; if *bf { if *status == DSA_COMP_SUCCESS { *status = DSA_COMP_BATCH_FAIL; } *result = 1; *bf = false; } idxd_user_counter_increment(wq, (*entry).pasid, COUNTER_FAULTS); }
        DSA_COMP_DRAIN_EVL => { copy_size = cr_size; }
        _ => { copy_size = 0; dev_dbg_ratelimited!(dev, "Unrecognized error code: %#x\n", (*fault).status); }
    }
    if copy_size == 0 { return; }
    let copied = idxd_copy_cr(wq, (*entry).pasid, (*entry).fault_addr, cr, copy_size);
    if copied != copy_size { idxd_user_counter_increment(wq, (*entry).pasid, COUNTER_FAULT_FAILS); dev_dbg_ratelimited!(dev, "Failed to write completion record (%d:%d)\n", copy_size, copied); if (*fault).status == DSA_COMP_CRA_XLAT && (*entry).batch { (*evl).batch_fail[(*entry).batch_id as usize] = true; } }
    kmem_cache_free((*idxd).evl_cache, fault);
}

unsafe fn process_evl_entry(idxd: *mut idxd_device, entry: *mut __evl_entry, index: c_uint) {
    let evl = (*idxd).evl; let dev = &(*(*idxd).pdev).dev;
    if test_bit(index, (*evl).bmap) { clear_bit(index, (*evl).bmap); return; }
    let status = DSA_COMP_STATUS((*entry).error);
    if status == DSA_COMP_CRA_XLAT || status == DSA_COMP_DRAIN_EVL || status == DSA_COMP_BATCH_EVL_ERR {
        if (*entry).rci { dev_dbg!(dev, "Completion Int Req set, ignoring!\n"); }
        if !(*entry).rcr && status == DSA_COMP_DRAIN_EVL { return; }
        let fault = kmem_cache_alloc((*idxd).evl_cache, GFP_ATOMIC);
        if !fault { dev_warn!(dev, "Failed to service fault work.\n"); return; }
        (*fault).wq = *(*idxd).wqs.add((*entry).wq_idx as usize); (*fault).status = status; memcpy(&mut (*fault).entry, entry, evl_ent_size(idxd)); INIT_WORK!(&mut (*fault).work, idxd_evl_fault_work); queue_work((*(*fault).wq).wq, &mut (*fault).work);
    } else { dev_warn_ratelimited!(dev, "Device error %#x operation: %#x fault addr: %#llx\n", status, (*entry).operation, (*entry).fault_addr); }
}

unsafe fn process_evl_entries(idxd: *mut idxd_device) {
    let evl = (*idxd).evl; let mut status: evl_status_reg = core::mem::zeroed(); status.bits = 0; status.int_pending = 1; mutex_lock(&mut (*evl).lock);
    iowrite32(status.bits_upper32, (*idxd).reg_base.add(IDXD_EVLSTATUS_OFFSET as usize + core::mem::size_of::<u32>())); status.bits = ioread64((*idxd).reg_base.add(IDXD_EVLSTATUS_OFFSET as usize)); let mut t = status.tail; let mut h = status.head; let size = (*evl).size; let ent = evl_ent_size(idxd);
    while h != t { process_evl_entry(idxd, (*evl).log.add((h * ent) as usize) as *mut __evl_entry, h); h = (h + 1) % size; }
    status.head = h; iowrite32(status.bits_lower32, (*idxd).reg_base.add(IDXD_EVLSTATUS_OFFSET as usize)); mutex_unlock(&mut (*evl).lock);
}

unsafe fn idxd_device_flr(work: *mut work_struct) { let idxd = container_of!(work, idxd_device, work); let rc = pci_reset_function((*idxd).pdev); if rc != 0 { dev_err!(&(*(*idxd).pdev).dev, "FLR failed\n"); } }
unsafe fn idxd_wqs_flush_descs(idxd: *mut idxd_device) { for i in 0..(*idxd).max_wqs { idxd_wq_flush_descs(*(*idxd).wqs.add(i as usize)); } }

unsafe fn idxd_halt(idxd: *mut idxd_device) -> irqreturn_t {
    let gensts = ioread32((*idxd).reg_base.add(IDXD_GENSTATS_OFFSET as usize));
    if gensts.state == IDXD_DEVICE_STATE_HALT { (*idxd).state = IDXD_DEV_HALTED; match gensts.reset_type { IDXD_DEVICE_RESET_SOFTWARE => { INIT_WORK!(&mut (*idxd).work, idxd_device_reinit); queue_work((*idxd).wq, &mut (*idxd).work); }, IDXD_DEVICE_RESET_FLR => { idxd_mask_error_interrupts(idxd); idxd_wqs_flush_descs(idxd); INIT_WORK!(&mut (*idxd).work, idxd_device_flr); queue_work((*idxd).wq, &mut (*idxd).work); }, _ => { idxd_wqs_quiesce(idxd); idxd_wqs_unmap_portal(idxd); idxd_device_clear_state(idxd); dev_err!(&(*(*idxd).pdev).dev, "idxd halted, need system reset"); return -ENXIO; } } } IRQ_HANDLED
}

pub unsafe fn idxd_misc_thread(_vec: c_int, data: *mut c_void) -> irqreturn_t {
    let ie = data as *mut idxd_irq_entry; let idxd = ie_to_idxd(ie); let cause = ioread32((*idxd).reg_base.add(IDXD_INTCAUSE_OFFSET as usize)); if cause == 0 { return IRQ_NONE; } iowrite32(cause, (*idxd).reg_base.add(IDXD_INTCAUSE_OFFSET as usize)); if cause & IDXD_INTC_HALT_STATE != 0 { return idxd_halt(idxd); }
    if cause & IDXD_INTC_ERR != 0 { spin_lock(&mut (*idxd).dev_lock); for i in 0..4 { (*idxd).sw_err.bits[i] = ioread64((*idxd).reg_base.add(IDXD_SWERR_OFFSET as usize + i * 8)); } iowrite64((*idxd).sw_err.bits[0] & IDXD_SWERR_ACK, (*idxd).reg_base.add(IDXD_SWERR_OFFSET as usize)); spin_unlock(&mut (*idxd).dev_lock); }
    if cause & IDXD_INTC_INT_HANDLE_REVOKED != 0 { let r = kzalloc::<idxd_int_handle_revoke>(GFP_ATOMIC); if !r.is_null() { (*r).idxd = idxd; INIT_WORK!(&mut (*r).work, idxd_int_handle_revoke); queue_work((*idxd).wq, &mut (*r).work); } else { idxd_wqs_quiesce(idxd); } }
    if cause & IDXD_INTC_CMD != 0 { complete((*idxd).cmd_done); } if cause & IDXD_INTC_PERFMON_OVFL != 0 { perfmon_counter_overflow(idxd); } if cause & IDXD_INTC_EVL != 0 { process_evl_entries(idxd); } IRQ_HANDLED
}

unsafe fn idxd_int_handle_resubmit_work(work: *mut work_struct) { let irw = container_of!(work, idxd_resubmit, work); let d = (*irw).desc; let wq = (*d).wq; (*(*d).completion).status = 0; let rc = idxd_submit_desc(wq, d); if rc < 0 { if rc != -EAGAIN { (*(*d).completion).status = IDXD_COMP_DESC_ABORT; idxd_desc_complete(d, IDXD_COMPLETE_ABORT, false); } idxd_free_desc(wq, d); } kfree(irw); }
pub unsafe fn idxd_queue_int_handle_resubmit(desc: *mut idxd_desc) -> bool { let irw = kzalloc::<idxd_resubmit>(GFP_KERNEL); if irw.is_null() { return false; } (*irw).desc = desc; INIT_WORK!(&mut (*irw).work, idxd_int_handle_resubmit_work); queue_work((*(*(*desc).wq).idxd).wq, &mut (*irw).work); true }

unsafe fn irq_process_pending_llist(ie: *mut idxd_irq_entry) { let head = llist_del_all(&mut (*ie).pending_llist); if head.is_null() { return; } let mut d: *mut idxd_desc; let mut t: *mut idxd_desc; llist_for_each_entry_safe!(d,t,head,llnode,{ let s = (*(*d).completion).status & DSA_COMP_STATUS_MASK; if s != 0 { idxd_desc_complete(d, if (*(*d).completion).status == IDXD_COMP_DESC_ABORT { IDXD_COMPLETE_ABORT } else { IDXD_COMPLETE_NORMAL }, true); } else { spin_lock(&mut (*ie).list_lock); list_add_tail!(&mut (*d).list,&mut (*ie).work_list); spin_unlock(&mut (*ie).list_lock); } }); }
unsafe fn irq_process_work_list(ie: *mut idxd_irq_entry) { let mut flist = LIST_HEAD!(); let mut d: *mut idxd_desc; let mut n: *mut idxd_desc; spin_lock(&mut (*ie).list_lock); list_for_each_entry_safe!(d,n,&mut (*ie).work_list,list,{ if (*(*d).completion).status != 0 { list_move_tail!(&mut (*d).list,&mut flist); } }); spin_unlock(&mut (*ie).list_lock); list_for_each_entry_safe!(d,n,&mut flist,list,{ list_del!(&mut (*d).list); idxd_desc_complete(d,if (*(*d).completion).status == IDXD_COMP_DESC_ABORT { IDXD_COMPLETE_ABORT } else { IDXD_COMPLETE_NORMAL },true); }); }
pub unsafe fn idxd_wq_thread(_irq: c_int, data: *mut c_void) -> irqreturn_t { let ie = data as *mut idxd_irq_entry; irq_process_work_list(ie); irq_process_pending_llist(ie); IRQ_HANDLED }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
