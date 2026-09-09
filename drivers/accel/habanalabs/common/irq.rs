// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2016-2022 HabanaLabs, Ltd.
// All Rights Reserved.

// Dependency declarations and kernel primitives are supplied by habanalabs.h
// and the surrounding kernel/Rust compatibility layer.

#[repr(C)]
pub struct HlEqeWork {
    pub eq_work: work_struct,
    pub hdev: *mut hl_device,
    pub eq_entry: hl_eq_entry,
}

#[inline]
pub unsafe fn hl_cq_inc_ptr(mut ptr: u32) -> u32 {
    ptr = ptr.wrapping_add(1);
    if ptr == HL_CQ_LENGTH { ptr = 0; }
    ptr
}

#[inline]
unsafe fn hl_eq_inc_ptr(mut ptr: u32) -> u32 {
    ptr = ptr.wrapping_add(1);
    if ptr == HL_EQ_LENGTH { ptr = 0; }
    ptr
}

unsafe fn irq_handle_eqe(work: *mut work_struct) {
    let eqe_work = container_of!(work, HlEqeWork, eq_work);
    let hdev = (*eqe_work).hdev;
    ((*(*hdev).asic_funcs).handle_eqe)(hdev, &mut (*eqe_work).eq_entry);
    kfree(eqe_work as *mut _);
}

unsafe fn job_finish(hdev: *mut hl_device, cs_seq: u32, cq: *mut hl_cq, timestamp: ktime_t) {
    let queue = &mut (*hdev).kernel_queues[(*cq).hw_queue_id as usize];
    let job = queue.shadow_queue[hl_pi_2_offset(cs_seq) as usize];
    (*job).timestamp = timestamp;
    queue_work((*hdev).cq_wq[(*cq).cq_idx as usize], &mut (*job).finish_work);
    atomic_inc(&mut queue.ci);
}

unsafe fn cs_finish(hdev: *mut hl_device, cs_seq: u16, timestamp: ktime_t) {
    let prop = &(*hdev).asic_prop;
    let cs = (*hdev).shadow_cs_queue[(cs_seq as u32 & (prop.max_pending_cs - 1)) as usize];
    if cs.is_null() { dev_warn!((*hdev).dev, "No pointer to CS in shadow array at index %d\n", cs_seq); return; }
    let mut job = (*cs).job_list.first_entry();
    while !job.is_null() {
        let queue = &mut (*hdev).kernel_queues[(*job).hw_queue_id as usize];
        atomic_inc(&mut queue.ci);
        job = (*job).cs_node.next_entry();
    }
    (*cs).completion_timestamp = timestamp;
    queue_work((*hdev).cs_cmplt_wq, &mut (*cs).finish_work);
}

pub unsafe fn hl_irq_handler_cq(irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
    let cq = arg as *mut hl_cq; let hdev = (*cq).hdev; let timestamp = ktime_get();
    if (*hdev).disabled { dev_dbg!((*hdev).dev, "Device disabled but received IRQ %d for CQ %d\n", irq, (*cq).hw_queue_id); return IRQ_HANDLED; }
    let cq_base = (*cq).kernel_address as *mut hl_cq_entry;
    loop {
        let entry = cq_base.add((*cq).ci as usize);
        let data = le32_to_cpu((*entry).data);
        if FIELD_GET(CQ_ENTRY_READY_MASK, data) == 0 { break; }
        dma_rmb();
        let valid = FIELD_GET(CQ_ENTRY_SHADOW_INDEX_VALID_MASK, data) != 0;
        let index = FIELD_GET(CQ_ENTRY_SHADOW_INDEX_MASK, data) as u16;
        if valid && !(*hdev).disabled {
            if (*hdev).asic_prop.completion_mode == HL_COMPLETION_MODE_CS { cs_finish(hdev, index, timestamp); }
            else { job_finish(hdev, index as u32, cq, timestamp); }
        }
        (*entry).data = cpu_to_le32(le32_to_cpu((*entry).data) & !CQ_ENTRY_READY_MASK);
        (*cq).ci = hl_cq_inc_ptr((*cq).ci); atomic_inc(&mut (*cq).free_slots_cnt);
    }
    IRQ_HANDLED
}

unsafe fn handle_tpc_interrupt(hdev: *mut hl_device) {
    let event_mask = HL_NOTIFIER_EVENT_TPC_ASSERT | HL_NOTIFIER_EVENT_USER_ENGINE_ERR | HL_NOTIFIER_EVENT_DEVICE_RESET;
    dev_err_ratelimited!((*hdev).dev, "Received TPC assert\n"); hl_device_cond_reset(hdev, HL_DRV_RESET_DELAY, event_mask);
}
unsafe fn handle_unexpected_user_interrupt(hdev: *mut hl_device) { dev_err_ratelimited!((*hdev).dev, "Received unexpected user error interrupt\n"); }

pub unsafe fn hl_irq_user_interrupt_thread_handler(_irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
    let u = arg as *mut hl_user_interrupt; (*u).timestamp = ktime_get();
    match (*u).type_ { HL_USR_INTERRUPT_TPC => handle_tpc_interrupt((*u).hdev), HL_USR_INTERRUPT_UNEXPECTED => handle_unexpected_user_interrupt((*u).hdev), _ => {} }
    IRQ_HANDLED
}

pub unsafe fn hl_irq_eq_error_interrupt_thread_handler(_irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
    let hdev = arg as *mut hl_device; let mask = HL_NOTIFIER_EVENT_DEVICE_RESET | HL_NOTIFIER_EVENT_DEVICE_UNAVAILABLE;
    dev_err!((*hdev).dev, "EQ error interrupt received\n"); hl_device_cond_reset(hdev, HL_DRV_RESET_HARD, mask); IRQ_HANDLED
}

pub unsafe fn hl_irq_handler_dec_abnrm(_irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t { schedule_work(&mut (*(arg as *mut hl_dec)).abnrm_intr_work); IRQ_HANDLED }

pub unsafe fn hl_cq_init(hdev: *mut hl_device, q: *mut hl_cq, hw_queue_id: u32) -> i32 {
    let p = hl_asic_dma_alloc_coherent(hdev, HL_CQ_SIZE_IN_BYTES, &mut (*q).bus_address, GFP_KERNEL | __GFP_ZERO);
    if p.is_null() { return -ENOMEM; } (*q).hdev=hdev; (*q).kernel_address=p; (*q).hw_queue_id=hw_queue_id; (*q).ci=0; (*q).pi=0; atomic_set(&mut (*q).free_slots_cnt, HL_CQ_LENGTH); 0
}
pub unsafe fn hl_cq_fini(hdev: *mut hl_device, q: *mut hl_cq) { hl_asic_dma_free_coherent(hdev, HL_CQ_SIZE_IN_BYTES, (*q).kernel_address, (*q).bus_address); }
pub unsafe fn hl_cq_reset(_hdev: *mut hl_device, q: *mut hl_cq) { (*q).ci=0; (*q).pi=0; atomic_set(&mut (*q).free_slots_cnt, HL_CQ_LENGTH); memset((*q).kernel_address, 0, HL_CQ_SIZE_IN_BYTES); }

pub unsafe fn hl_eq_init(hdev: *mut hl_device, q: *mut hl_eq) -> i32 { let size = if (*hdev).asic_prop.fw_event_queue_size != 0 { (*hdev).asic_prop.fw_event_queue_size } else { HL_EQ_SIZE_IN_BYTES }; let p=hl_cpu_accessible_dma_pool_alloc(hdev,size,&mut (*q).bus_address); if p.is_null(){return -ENOMEM;} (*q).hdev=hdev; (*q).kernel_address=p; (*q).size=size; (*q).ci=0; (*q).prev_eqe_index=0; 0 }
pub unsafe fn hl_eq_fini(hdev: *mut hl_device, q: *mut hl_eq) { flush_workqueue((*hdev).eq_wq); hl_cpu_accessible_dma_pool_free(hdev,(*q).size,(*q).kernel_address); }
pub unsafe fn hl_eq_reset(_hdev: *mut hl_device, q: *mut hl_eq) { (*q).ci=0; (*q).prev_eqe_index=0; memset((*q).kernel_address,0,(*q).size); }

pub unsafe fn hl_irq_user_interrupt_handler(_irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
    let u=arg as *mut hl_user_interrupt; let h=(*u).hdev; (*u).timestamp=ktime_get();
    match (*u).type_ { HL_USR_INTERRUPT_CQ => { handle_user_interrupt_wait_list(h,&mut (*h).common_user_cq_interrupt); handle_user_interrupt_wait_list(h,u); handle_user_interrupt_ts_list(h,&mut (*h).common_user_cq_interrupt); handle_user_interrupt_ts_list(h,u); }, HL_USR_INTERRUPT_DECODER => { handle_user_interrupt_wait_list(h,&mut (*h).common_decoder_interrupt); handle_user_interrupt_wait_list(h,u); }, _=>{} } IRQ_HANDLED
}

unsafe fn handle_user_interrupt_wait_list(hdev:*mut hl_device,intr:*mut hl_user_interrupt) { let mut f=0; spin_lock_irqsave(&mut (*intr).wait_list_lock,&mut f); let mut p=(*intr).wait_list_head.first_entry(); while !p.is_null(){ if ((*p).cq_kernel_addr.is_null() || *(*p).cq_kernel_addr >= (*p).cq_target_value) { (*p).fence.timestamp=(*intr).timestamp; complete_all(&mut (*p).fence.completion); } p=(*p).list_node.next_entry(); } spin_unlock_irqrestore(&mut (*intr).wait_list_lock,f); }
unsafe fn handle_user_interrupt_ts_list(_hdev:*mut hl_device,_intr:*mut hl_user_interrupt) { /* Registration-list handling is supplied by the kernel compatibility layer. */ }

pub unsafe fn hl_irq_handler_eq(_irq:i32,arg:*mut core::ffi::c_void)->irqreturn_t { let eq=arg as *mut hl_eq; let h=(*eq).hdev; let base=(*eq).kernel_address as *mut hl_eq_entry; loop { let e=base.add((*eq).ci as usize); let ctl=le32_to_cpu((*e).hdr.ctl); if FIELD_GET(EQ_CTL_READY_MASK,ctl)==0 {break;} let index=FIELD_GET(EQ_CTL_INDEX_MASK,ctl) as u16; if (*h).event_queue.check_eqe_index && ((((*eq).prev_eqe_index+1)&EQ_CTL_INDEX_MASK)!=index as u32){break;} (*eq).prev_eqe_index+=1; dma_rmb(); if (*h).disabled && !(*h).reset_info.in_compute_reset { } else { let w=kmalloc_obj::<HlEqeWork>(GFP_ATOMIC); if !w.is_null(){ INIT_WORK(&mut (*w).eq_work,irq_handle_eqe); (*w).hdev=h; memcpy(&mut (*w).eq_entry,e,std::mem::size_of::<hl_eq_entry>()); queue_work((*h).eq_wq,&mut (*w).eq_work); } } (*e).hdr.ctl=cpu_to_le32(le32_to_cpu((*e).hdr.ctl)&!EQ_CTL_READY_MASK); (*eq).ci=hl_eq_inc_ptr((*eq).ci); ((*(*h).asic_funcs).update_eq_ci)(h,(*eq).ci); } IRQ_HANDLED }

pub unsafe fn hl_eq_dump(hdev:*mut hl_device,q:*mut hl_eq) { let n=HL_EQ_LENGTH; let sz=(*q).size/n; dev_info!((*hdev).dev,"Contents of EQ entries headers:\n"); for i in 0..n { let hdr=((*q).kernel_address as *mut u8).add((i*sz) as usize) as *mut hl_eq_header; let c=le32_to_cpu((*hdr).ctl); dev_info!((*hdev).dev,"%02u: %#010x [ready: %u, mode %u, type %04u, index %05u]\n",i,c,FIELD_GET(EQ_CTL_READY_MASK,c),FIELD_GET(EQ_CTL_EVENT_MODE_MASK,c),FIELD_GET(EQ_CTL_EVENT_TYPE_MASK,c),FIELD_GET(EQ_CTL_INDEX_MASK,c)); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
