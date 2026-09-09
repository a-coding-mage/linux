// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

// Linux and driver-header dependencies are supplied by the surrounding translation.

const CC_MAX_POLL_ITER: usize = 10;
/* The highest descriptor count in used */
const CC_MAX_DESC_SEQ_LEN: usize = 23;

#[repr(C)]
pub struct cc_req_mgr_handle {
    /* Request manager resources */
    pub hw_queue_size: ::core::ffi::c_uint,
    pub min_free_hw_slots: ::core::ffi::c_uint,
    pub max_used_sw_slots: ::core::ffi::c_uint,
    pub req_queue: [cc_crypto_req; MAX_REQUEST_QUEUE_SIZE],
    pub req_queue_head: u32,
    pub req_queue_tail: u32,
    pub axi_completed: u32,
    pub q_free_slots: u32,
    /* This lock protects access to HW register
     * that must be single request at a time
     */
    pub hw_lock: spinlock_t,
    pub compl_desc: cc_hw_desc,
    pub dummy_comp_buff: *mut u8,
    pub dummy_comp_buff_dma: dma_addr_t,

    /* backlog queue */
    pub backlog: list_head,
    pub bl_len: ::core::ffi::c_uint,
    pub bl_lock: spinlock_t, /* protect backlog queue */

    #[cfg(COMP_IN_WQ)]
    pub workq: *mut workqueue_struct,
    #[cfg(COMP_IN_WQ)]
    pub compwork: delayed_work,
    #[cfg(not(COMP_IN_WQ))]
    pub comptask: tasklet_struct,
}

#[repr(C)]
pub struct cc_bl_item {
    pub creq: cc_crypto_req,
    pub desc: [cc_hw_desc; CC_MAX_DESC_SEQ_LEN],
    pub len: ::core::ffi::c_uint,
    pub list: list_head,
    pub notif: bool,
}

static CC_CPP_INT_MASKS: [[u32; CC_CPP_NUM_SLOTS]; CC_CPP_NUM_ALGS] = [
    [
        BIT(CC_HOST_IRR_REE_OP_ABORTED_AES_0_INT_BIT_SHIFT),
        BIT(CC_HOST_IRR_REE_OP_ABORTED_AES_1_INT_BIT_SHIFT),
        BIT(CC_HOST_IRR_REE_OP_ABORTED_AES_2_INT_BIT_SHIFT),
        BIT(CC_HOST_IRR_REE_OP_ABORTED_AES_3_INT_BIT_SHIFT),
        BIT(CC_HOST_IRR_REE_OP_ABORTED_AES_4_INT_BIT_SHIFT),
        BIT(CC_HOST_IRR_REE_OP_ABORTED_AES_5_INT_BIT_SHIFT),
        BIT(CC_HOST_IRR_REE_OP_ABORTED_AES_6_INT_BIT_SHIFT),
        BIT(CC_HOST_IRR_REE_OP_ABORTED_AES_7_INT_BIT_SHIFT),
    ],
    [
        BIT(CC_HOST_IRR_REE_OP_ABORTED_SM_0_INT_BIT_SHIFT),
        BIT(CC_HOST_IRR_REE_OP_ABORTED_SM_1_INT_BIT_SHIFT),
        BIT(CC_HOST_IRR_REE_OP_ABORTED_SM_2_INT_BIT_SHIFT),
        BIT(CC_HOST_IRR_REE_OP_ABORTED_SM_3_INT_BIT_SHIFT),
        BIT(CC_HOST_IRR_REE_OP_ABORTED_SM_4_INT_BIT_SHIFT),
        BIT(CC_HOST_IRR_REE_OP_ABORTED_SM_5_INT_BIT_SHIFT),
        BIT(CC_HOST_IRR_REE_OP_ABORTED_SM_6_INT_BIT_SHIFT),
        BIT(CC_HOST_IRR_REE_OP_ABORTED_SM_7_INT_BIT_SHIFT),
    ],
];

// Forward declarations in the C source; definitions are provided below.

#[inline]
unsafe fn cc_cpp_int_mask(mut alg: cc_cpp_alg, mut slot: i32) -> u32 {
    alg = array_index_nospec(alg, CC_CPP_NUM_ALGS);
    slot = array_index_nospec(slot, CC_CPP_NUM_SLOTS);
    CC_CPP_INT_MASKS[alg as usize][slot as usize]
}

pub unsafe fn cc_req_mgr_fini(drvdata: *mut cc_drvdata) {
    let req_mgr_h = (*drvdata).request_mgr_handle;
    let dev = drvdata_to_dev(drvdata);
    if req_mgr_h.is_null() { return; }
    if (*req_mgr_h).dummy_comp_buff_dma != 0 {
        dma_free_coherent(dev, core::mem::size_of::<u32>(), (*req_mgr_h).dummy_comp_buff,
                          (*req_mgr_h).dummy_comp_buff_dma);
    }
    dev_dbg(dev, "max_used_hw_slots=%d\n", (*req_mgr_h).hw_queue_size - (*req_mgr_h).min_free_hw_slots);
    dev_dbg(dev, "max_used_sw_slots=%d\n", (*req_mgr_h).max_used_sw_slots);
    #[cfg(COMP_IN_WQ)]
    destroy_workqueue((*req_mgr_h).workq);
    #[cfg(not(COMP_IN_WQ))]
    tasklet_kill(&mut (*req_mgr_h).comptask);
    kfree_sensitive(req_mgr_h);
    (*drvdata).request_mgr_handle = core::ptr::null_mut();
}

pub unsafe fn cc_req_mgr_init(drvdata: *mut cc_drvdata) -> i32 {
    let mut req_mgr_h: *mut cc_req_mgr_handle = kzalloc_obj::<cc_req_mgr_handle>();
    let dev = drvdata_to_dev(drvdata);
    let mut rc: i32 = 0;
    if req_mgr_h.is_null() { rc = -ENOMEM; return rc; }
    (*drvdata).request_mgr_handle = req_mgr_h;
    spin_lock_init(&mut (*req_mgr_h).hw_lock);
    spin_lock_init(&mut (*req_mgr_h).bl_lock);
    INIT_LIST_HEAD(&mut (*req_mgr_h).backlog);
    #[cfg(COMP_IN_WQ)]
    {
        dev_dbg(dev, "Initializing completion workqueue\n");
        (*req_mgr_h).workq = create_singlethread_workqueue("ccree");
        if (*req_mgr_h).workq.is_null() { dev_err(dev, "Failed creating work queue\n"); rc = -ENOMEM; cc_req_mgr_fini(drvdata); return rc; }
        INIT_DELAYED_WORK(&mut (*req_mgr_h).compwork, comp_work_handler);
    }
    #[cfg(not(COMP_IN_WQ))]
    {
        dev_dbg(dev, "Initializing completion tasklet\n");
        tasklet_init(&mut (*req_mgr_h).comptask, comp_handler, drvdata as c_ulong);
    }
    (*req_mgr_h).hw_queue_size = cc_ioread(drvdata, CC_REG(DSCRPTR_QUEUE_SRAM_SIZE));
    dev_dbg(dev, "hw_queue_size=0x%08X\n", (*req_mgr_h).hw_queue_size);
    if (*req_mgr_h).hw_queue_size < MIN_HW_QUEUE_SIZE { dev_err(dev, "Invalid HW queue size = %u (Min. required is %u)\n", (*req_mgr_h).hw_queue_size, MIN_HW_QUEUE_SIZE); rc = -ENOMEM; cc_req_mgr_fini(drvdata); return rc; }
    (*req_mgr_h).min_free_hw_slots = (*req_mgr_h).hw_queue_size;
    (*req_mgr_h).max_used_sw_slots = 0;
    (*req_mgr_h).dummy_comp_buff = dma_alloc_coherent(dev, core::mem::size_of::<u32>(), &mut (*req_mgr_h).dummy_comp_buff_dma, GFP_KERNEL);
    if (*req_mgr_h).dummy_comp_buff.is_null() { dev_err(dev, "Not enough memory to allocate DMA (%zu) dropped buffer\n", core::mem::size_of::<u32>()); rc = -ENOMEM; cc_req_mgr_fini(drvdata); return rc; }
    hw_desc_init(&mut (*req_mgr_h).compl_desc);
    set_din_const(&mut (*req_mgr_h).compl_desc, 0, core::mem::size_of::<u32>());
    set_dout_dlli(&mut (*req_mgr_h).compl_desc, (*req_mgr_h).dummy_comp_buff_dma, core::mem::size_of::<u32>(), NS_BIT, 1);
    set_flow_mode(&mut (*req_mgr_h).compl_desc, BYPASS);
    set_queue_last_ind(drvdata, &mut (*req_mgr_h).compl_desc);
    0
}

unsafe fn enqueue_seq(drvdata: *mut cc_drvdata, seq: *mut cc_hw_desc, seq_len: ::core::ffi::c_uint) {
    let reg = (*drvdata).cc_base.add(CC_REG(DSCRPTR_QUEUE_WORD0));
    let dev = drvdata_to_dev(drvdata);
    for i in 0..seq_len {
        for w in 0..=5 { writel_relaxed((*seq.add(i as usize)).word[w], reg); }
        if cc_dump_desc { dev_dbg(dev, "desc[%02d]: 0x%08X 0x%08X 0x%08X 0x%08X 0x%08X 0x%08X\n", i, (*seq.add(i as usize)).word[0], (*seq.add(i as usize)).word[1], (*seq.add(i as usize)).word[2], (*seq.add(i as usize)).word[3], (*seq.add(i as usize)).word[4], (*seq.add(i as usize)).word[5]); }
    }
}

unsafe fn request_mgr_complete(_dev: *mut device, dx_compl_h: *mut c_void, _dummy: i32) { complete(dx_compl_h as *mut completion); }

unsafe fn cc_queues_status(drvdata: *mut cc_drvdata, req_mgr_h: *mut cc_req_mgr_handle, total_seq_len: u32) -> i32 {
    let dev = drvdata_to_dev(drvdata);
    if (((*req_mgr_h).req_queue_head + 1) & (MAX_REQUEST_QUEUE_SIZE as u32 - 1)) == (*req_mgr_h).req_queue_tail { dev_err(dev, "SW FIFO is full. req_queue_head=%d sw_fifo_len=%d\n", (*req_mgr_h).req_queue_head, MAX_REQUEST_QUEUE_SIZE); return -ENOSPC; }
    if (*req_mgr_h).q_free_slots >= total_seq_len { return 0; }
    for _ in 0..CC_MAX_POLL_ITER { (*req_mgr_h).q_free_slots = cc_ioread(drvdata, CC_REG(DSCRPTR_QUEUE_CONTENT)); if (*req_mgr_h).q_free_slots < (*req_mgr_h).min_free_hw_slots { (*req_mgr_h).min_free_hw_slots = (*req_mgr_h).q_free_slots; } if (*req_mgr_h).q_free_slots >= total_seq_len { return 0; } dev_dbg(dev, "HW FIFO is full. q_free_slots=%d total_seq_len=%d\n", (*req_mgr_h).q_free_slots, total_seq_len); }
    dev_dbg(dev, "HW FIFO full, timeout. req_queue_head=%d sw_fifo_len=%d q_free_slots=%d total_seq_len=%d\n", (*req_mgr_h).req_queue_head, MAX_REQUEST_QUEUE_SIZE, (*req_mgr_h).q_free_slots, total_seq_len); -ENOSPC
}

unsafe fn cc_do_send_request(drvdata: *mut cc_drvdata, cc_req: *mut cc_crypto_req, desc: *mut cc_hw_desc, len: u32, add_comp: bool) {
    let h = (*drvdata).request_mgr_handle; let mut total_seq_len = len; let dev = drvdata_to_dev(drvdata);
    let used = ((*h).req_queue_head - (*h).req_queue_tail) & (MAX_REQUEST_QUEUE_SIZE as u32 - 1); if used > (*h).max_used_sw_slots { (*h).max_used_sw_slots = used; }
    (*h).req_queue[(*h).req_queue_head as usize] = *cc_req; (*h).req_queue_head = ((*h).req_queue_head + 1) & (MAX_REQUEST_QUEUE_SIZE as u32 - 1); dev_dbg(dev, "Enqueue request head=%u\n", (*h).req_queue_head); wmb(); enqueue_seq(drvdata, desc, len);
    if add_comp { enqueue_seq(drvdata, &mut (*h).compl_desc, 1); total_seq_len += 1; }
    if (*h).q_free_slots < total_seq_len { dev_err(dev, "HW free slot count mismatch."); (*h).q_free_slots = 0; } else { (*h).q_free_slots -= total_seq_len; }
}

unsafe fn cc_enqueue_backlog(drvdata: *mut cc_drvdata, bli: *mut cc_bl_item) { let mgr = (*drvdata).request_mgr_handle; let dev = drvdata_to_dev(drvdata); spin_lock_bh(&mut (*mgr).bl_lock); list_add_tail(&mut (*bli).list, &mut (*mgr).backlog); (*mgr).bl_len += 1; dev_dbg(dev, "+++bl len: %d\n", (*mgr).bl_len); spin_unlock_bh(&mut (*mgr).bl_lock); tasklet_schedule(&mut (*mgr).comptask); }

unsafe fn cc_proc_backlog(drvdata: *mut cc_drvdata) {
    let mgr = (*drvdata).request_mgr_handle; let dev = drvdata_to_dev(drvdata); spin_lock(&mut (*mgr).bl_lock);
    while (*mgr).bl_len != 0 { let bli = list_first_entry(&mut (*mgr).backlog, cc_bl_item, list); dev_dbg(dev, "---bl len: %d\n", (*mgr).bl_len); spin_unlock(&mut (*mgr).bl_lock); let creq = &mut (*bli).creq; let req = creq.user_arg; if !(*bli).notif { (creq.user_cb)(dev, req, -EINPROGRESS); (*bli).notif = true; } spin_lock(&mut (*mgr).hw_lock); let rc = cc_queues_status(drvdata, mgr, (*bli).len); if rc != 0 { spin_unlock(&mut (*mgr).hw_lock); return; } cc_do_send_request(drvdata, creq, (*bli).desc.as_mut_ptr(), (*bli).len, false); spin_unlock(&mut (*mgr).hw_lock); spin_lock(&mut (*mgr).bl_lock); list_del(&mut (*bli).list); (*mgr).bl_len -= 1; kfree(bli); }
    spin_unlock(&mut (*mgr).bl_lock);
}

pub unsafe fn cc_send_request(drvdata: *mut cc_drvdata, cc_req: *mut cc_crypto_req, desc: *mut cc_hw_desc, len: u32, req: *mut crypto_async_request) -> i32 {
    let mgr = (*drvdata).request_mgr_handle; let dev = drvdata_to_dev(drvdata); let backlog_ok = (*req).flags & CRYPTO_TFM_REQ_MAY_BACKLOG != 0; let flags = cc_gfp_flags(req); let rc = cc_pm_get(dev); if rc != 0 { dev_err(dev, "cc_pm_get returned %x\n", rc); return rc; }
    spin_lock_bh(&mut (*mgr).hw_lock); let mut rc = cc_queues_status(drvdata, mgr, len);
    #[cfg(CC_DEBUG_FORCE_BACKLOG)] if backlog_ok { rc = -ENOSPC; }
    if rc == -ENOSPC && backlog_ok { spin_unlock_bh(&mut (*mgr).hw_lock); let bli = kmalloc_obj::<cc_bl_item>(flags); if bli.is_null() { cc_pm_put_suspend(dev); return -ENOMEM; } core::ptr::copy_nonoverlapping(cc_req, &mut (*bli).creq, 1); core::ptr::copy_nonoverlapping(desc, (*bli).desc.as_mut_ptr(), len as usize); (*bli).len = len; (*bli).notif = false; cc_enqueue_backlog(drvdata, bli); return -EBUSY; }
    if rc == 0 { cc_do_send_request(drvdata, cc_req, desc, len, false); rc = -EINPROGRESS; } spin_unlock_bh(&mut (*mgr).hw_lock); rc
}

pub unsafe fn cc_send_sync_request(drvdata: *mut cc_drvdata, cc_req: *mut cc_crypto_req, desc: *mut cc_hw_desc, len: u32) -> i32 { let dev = drvdata_to_dev(drvdata); let mgr = (*drvdata).request_mgr_handle; init_completion(&mut (*cc_req).seq_compl); (*cc_req).user_cb = Some(request_mgr_complete); (*cc_req).user_arg = &mut (*cc_req).seq_compl as *mut _ as *mut c_void; let rc = cc_pm_get(dev); if rc != 0 { dev_err(dev, "cc_pm_get returned %x\n", rc); return rc; } loop { spin_lock_bh(&mut (*mgr).hw_lock); let rc = cc_queues_status(drvdata, mgr, len + 1); if rc == 0 { break; } spin_unlock_bh(&mut (*mgr).hw_lock); wait_for_completion_interruptible(&mut (*drvdata).hw_queue_avail); reinit_completion(&mut (*drvdata).hw_queue_avail); } cc_do_send_request(drvdata, cc_req, desc, len, true); spin_unlock_bh(&mut (*mgr).hw_lock); wait_for_completion(&mut (*cc_req).seq_compl); 0 }

pub unsafe fn send_request_init(drvdata: *mut cc_drvdata, desc: *mut cc_hw_desc, len: u32) -> i32 { let h = (*drvdata).request_mgr_handle; let rc = cc_queues_status(drvdata, h, len); if rc != 0 { return rc; } set_queue_last_ind(drvdata, desc.add((len - 1) as usize)); wmb(); enqueue_seq(drvdata, desc, len); (*h).q_free_slots = cc_ioread(drvdata, CC_REG(DSCRPTR_QUEUE_CONTENT)); 0 }

pub unsafe fn complete_request(drvdata: *mut cc_drvdata) { let h = (*drvdata).request_mgr_handle; complete(&mut (*drvdata).hw_queue_avail); #[cfg(COMP_IN_WQ)] queue_delayed_work((*h).workq, &mut (*h).compwork, 0); #[cfg(not(COMP_IN_WQ))] tasklet_schedule(&mut (*h).comptask); }

#[cfg(COMP_IN_WQ)]
unsafe fn comp_work_handler(work: *mut work_struct) { let drvdata = container_of(work, cc_drvdata, compwork.work); comp_handler(drvdata as c_ulong); }

unsafe fn proc_completions(drvdata: *mut cc_drvdata) { let h = (*drvdata).request_mgr_handle; let dev = drvdata_to_dev(drvdata); while (*h).axi_completed != 0 { (*h).axi_completed -= 1; if (*h).req_queue_head == (*h).req_queue_tail { dev_err(dev, "Request queue is empty head == tail %u\n", (*h).req_queue_head); break; } let req = &mut (*h).req_queue[(*h).req_queue_tail as usize]; let rc = if req.cpp.is_cpp { dev_dbg(dev, "CPP request completion slot: %d alg:%d\n", req.cpp.slot, req.cpp.alg); let mask = cc_cpp_int_mask(req.cpp.alg, req.cpp.slot); let rc = if (*drvdata).irq & mask != 0 { -EPERM } else { 0 }; dev_dbg(dev, "Got mask: %x irq: %x rc: %d\n", mask, (*drvdata).irq, rc); rc } else { dev_dbg(dev, "None CPP request completion\n"); 0 }; if let Some(cb) = req.user_cb { cb(dev, req.user_arg, rc); } (*h).req_queue_tail = ((*h).req_queue_tail + 1) & (MAX_REQUEST_QUEUE_SIZE as u32 - 1); dev_dbg(dev, "Dequeue request tail=%u\n", (*h).req_queue_tail); dev_dbg(dev, "Request completed. axi_completed=%d\n", (*h).axi_completed); cc_pm_put_suspend(dev); } }

#[inline]
unsafe fn cc_axi_comp_count(drvdata: *mut cc_drvdata) -> u32 { FIELD_GET(AXIM_MON_COMP_VALUE, cc_ioread(drvdata, (*drvdata).axim_mon_offset)) }

/* Deferred service handler, run as interrupt-fired tasklet */
unsafe fn comp_handler(devarg: c_ulong) { let drvdata = devarg as *mut cc_drvdata; let h = (*drvdata).request_mgr_handle; let dev = drvdata_to_dev(drvdata); dev_dbg(dev, "Completion handler called!\n"); let mut irq = (*drvdata).irq & (*drvdata).comp_mask; cc_iowrite(drvdata, CC_REG(HOST_ICR), irq); (*h).axi_completed += cc_axi_comp_count(drvdata); dev_dbg(dev, "AXI completion after updated: %d\n", (*h).axi_completed); while (*h).axi_completed != 0 { loop { (*drvdata).irq |= cc_ioread(drvdata, CC_REG(HOST_IRR)); irq = (*drvdata).irq & (*drvdata).comp_mask; proc_completions(drvdata); (*h).axi_completed += cc_axi_comp_count(drvdata); if (*h).axi_completed == 0 { break; } } cc_iowrite(drvdata, CC_REG(HOST_ICR), irq); (*h).axi_completed += cc_axi_comp_count(drvdata); } cc_iowrite(drvdata, CC_REG(HOST_IMR), cc_ioread(drvdata, CC_REG(HOST_IMR)) & !(*drvdata).comp_mask); cc_proc_backlog(drvdata); dev_dbg(dev, "Comp. handler done.\n"); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
