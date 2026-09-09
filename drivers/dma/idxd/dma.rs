// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2019 Intel Corporation. All rights rsvd. */

#[inline]
unsafe fn to_idxd_wq(c: *mut dma_chan) -> *mut idxd_wq {
    let idxd_chan = container_of!(c, idxd_dma_chan, chan);
    (*idxd_chan).wq
}

unsafe fn idxd_dma_complete_txd(
    desc: *mut idxd_desc,
    comp_type: idxd_complete_type,
    free_desc: bool,
    ctx: *mut core::ffi::c_void,
    status: *mut u32,
) {
    let idxd = (*(*desc).wq).idxd;
    let mut tx: *mut dma_async_tx_descriptor;
    let mut res: dmaengine_result;
    let mut complete = 1;

    if (*(*desc).completion).status == DSA_COMP_SUCCESS {
        res.result = DMA_TRANS_NOERROR;
    } else if (*(*desc).completion).status != 0 {
        if (*idxd).request_int_handles
            && comp_type != IDXD_COMPLETE_ABORT
            && (*(*desc).completion).status == DSA_COMP_INT_HANDLE_INVAL
            && idxd_queue_int_handle_resubmit(desc)
        {
            return;
        }
        res.result = DMA_TRANS_WRITE_FAILED;
    } else if comp_type == IDXD_COMPLETE_ABORT {
        res.result = DMA_TRANS_ABORTED;
    } else {
        complete = 0;
    }

    tx = &mut (*desc).txd;
    if complete != 0 && (*tx).cookie != 0 {
        dma_cookie_complete(tx);
        dma_descriptor_unmap(tx);
        dmaengine_desc_get_callback_invoke(tx, &mut res);
        (*tx).callback = None;
        (*tx).callback_result = None;
    }

    if free_desc {
        idxd_free_desc((*desc).wq, desc);
    }
}

unsafe fn op_flag_setup(flags: usize, desc_flags: *mut u32) {
    *desc_flags = IDXD_OP_FLAG_CRAV | IDXD_OP_FLAG_RCR;
    if flags & DMA_PREP_INTERRUPT != 0 {
        *desc_flags |= IDXD_OP_FLAG_RCI;
    }
}

#[inline]
unsafe fn idxd_prep_desc_common(
    wq: *mut idxd_wq,
    hw: *mut dsa_hw_desc,
    opcode: i8,
    addr_f1: u64,
    addr_f2: u64,
    len: u64,
    compl: u64,
    flags: u32,
) {
    (*hw).flags = flags;
    (*hw).opcode = opcode;
    (*hw).src_addr = addr_f1;
    (*hw).dst_addr = addr_f2;
    (*hw).xfer_size = len;
    /*
     * For dedicated WQ, this field is ignored and HW will use the WQCFG.priv
     * field instead. This field should be set to 0 for kernel descriptors
     * since kernel DMA on VT-d supports "user" privilege only.
     */
    (*hw).priv_ = 0;
    (*hw).completion_addr = compl;
}

unsafe fn idxd_dma_prep_interrupt(c: *mut dma_chan, flags: usize) -> *mut dma_async_tx_descriptor {
    let wq = to_idxd_wq(c);
    let mut desc_flags = 0u32;
    let desc: *mut idxd_desc;

    if (*wq).state != IDXD_WQ_ENABLED {
        return core::ptr::null_mut();
    }
    op_flag_setup(flags, &mut desc_flags);
    desc = idxd_alloc_desc(wq, IDXD_OP_BLOCK);
    if is_err!(desc) {
        return core::ptr::null_mut();
    }
    idxd_prep_desc_common(wq, (*desc).hw, DSA_OPCODE_NOOP, 0, 0, 0, (*desc).compl_dma, desc_flags);
    (*desc).txd.flags = flags;
    &mut (*desc).txd
}

unsafe fn idxd_dma_submit_memcpy(
    c: *mut dma_chan,
    dma_dest: dma_addr_t,
    dma_src: dma_addr_t,
    len: usize,
    flags: usize,
) -> *mut dma_async_tx_descriptor {
    let wq = to_idxd_wq(c);
    let mut desc_flags = 0u32;
    let idxd = (*wq).idxd;
    let desc: *mut idxd_desc;

    if (*wq).state != IDXD_WQ_ENABLED || len > (*idxd).max_xfer_bytes {
        return core::ptr::null_mut();
    }
    op_flag_setup(flags, &mut desc_flags);
    desc = idxd_alloc_desc(wq, IDXD_OP_BLOCK);
    if is_err!(desc) {
        return core::ptr::null_mut();
    }
    idxd_prep_desc_common(wq, (*desc).hw, DSA_OPCODE_MEMMOVE, dma_src, dma_dest, len as u64, (*desc).compl_dma, desc_flags);
    (*desc).txd.flags = flags;
    &mut (*desc).txd
}

unsafe fn idxd_dma_alloc_chan_resources(chan: *mut dma_chan) -> i32 {
    let wq = to_idxd_wq(chan);
    let dev = &mut (*(*(*wq).idxd).pdev).dev;
    idxd_wq_get(wq);
    dev_dbg!(dev, "%s: client_count: %d\n", "idxd_dma_alloc_chan_resources", idxd_wq_refcount(wq));
    0
}

unsafe fn idxd_dma_free_chan_resources(chan: *mut dma_chan) {
    let wq = to_idxd_wq(chan);
    let dev = &mut (*(*(*wq).idxd).pdev).dev;
    idxd_wq_put(wq);
    dev_dbg!(dev, "%s: client_count: %d\n", "idxd_dma_free_chan_resources", idxd_wq_refcount(wq));
}

unsafe fn idxd_dma_tx_status(_dma_chan: *mut dma_chan, _cookie: dma_cookie_t, _txstate: *mut dma_tx_state) -> dma_status {
    DMA_OUT_OF_ORDER
}

/* issue_pending() does not need to do anything since tx_submit() does the job
 * already.
 */
unsafe fn idxd_dma_issue_pending(_dma_chan: *mut dma_chan) {}

unsafe fn idxd_dma_tx_submit(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    let c = (*tx).chan;
    let wq = to_idxd_wq(c);
    let desc = container_of!(tx, idxd_desc, txd);
    let cookie = dma_cookie_assign(tx);
    let rc = idxd_submit_desc(wq, desc);
    if rc < 0 {
        idxd_free_desc(wq, desc);
        return rc as dma_cookie_t;
    }
    cookie
}

unsafe fn idxd_dma_release(device: *mut dma_device) {
    let idxd_dma = container_of!(device, idxd_dma_dev, dma);
    kfree(idxd_dma);
}

unsafe fn idxd_dma_terminate_all(c: *mut dma_chan) -> i32 {
    idxd_wq_flush_descs(to_idxd_wq(c));
    0
}

unsafe fn idxd_dma_synchronize(c: *mut dma_chan) {
    idxd_wq_drain(to_idxd_wq(c));
}

unsafe fn idxd_register_dma_device(idxd: *mut idxd_device) -> i32 {
    let dev = &mut (*(*idxd).pdev).dev;
    let idxd_dma = kzalloc_node(core::mem::size_of::<idxd_dma_dev>(), GFP_KERNEL, dev_to_node(dev));
    if idxd_dma.is_null() { return -ENOMEM; }
    let dma = &mut (*idxd_dma).dma;
    INIT_LIST_HEAD!(&mut dma.channels);
    dma.dev = dev;
    dma_cap_set!(DMA_INTERRUPT, dma.cap_mask);
    dma_cap_set!(DMA_PRIVATE, dma.cap_mask);
    dma_cap_set!(DMA_COMPLETION_NO_ORDER, dma.cap_mask);
    dma.device_release = Some(idxd_dma_release);
    dma.device_prep_dma_interrupt = Some(idxd_dma_prep_interrupt);
    if (*idxd).hw.opcap.bits[0] & IDXD_OPCAP_MEMMOVE != 0 {
        dma_cap_set!(DMA_MEMCPY, dma.cap_mask);
        dma.device_prep_dma_memcpy = Some(idxd_dma_submit_memcpy);
    }
    dma.device_tx_status = Some(idxd_dma_tx_status);
    dma.device_issue_pending = Some(idxd_dma_issue_pending);
    dma.device_alloc_chan_resources = Some(idxd_dma_alloc_chan_resources);
    dma.device_free_chan_resources = Some(idxd_dma_free_chan_resources);
    dma.device_terminate_all = Some(idxd_dma_terminate_all);
    dma.device_synchronize = Some(idxd_dma_synchronize);
    let rc = dma_async_device_register(dma);
    if rc < 0 { kfree(idxd_dma); return rc; }
    (*idxd_dma).idxd = idxd;
    (*idxd).idxd_dma = idxd_dma;
    0
}

unsafe fn idxd_unregister_dma_device(idxd: *mut idxd_device) {
    dma_async_device_unregister(&mut (*(*idxd).idxd_dma).dma);
}

static mut dev_types: [idxd_dev_type; 2] = [IDXD_DEV_WQ, IDXD_DEV_NONE];

static mut idxd_dmaengine_drv: idxd_device_driver = idxd_device_driver {
    probe: Some(idxd_dmaengine_drv_probe),
    remove: Some(idxd_dmaengine_drv_remove),
    desc_complete: Some(idxd_dma_complete_txd),
    name: "dmaengine",
    type_: dev_types.as_ptr(),
};

unsafe fn idxd_register_dma_channel(wq: *mut idxd_wq) -> i32 {
    let idxd = (*wq).idxd;
    let dma = &mut (*(*idxd).idxd_dma).dma;
    let dev = &mut (*(*idxd).pdev).dev;
    let idxd_chan = kzalloc_node(core::mem::size_of::<idxd_dma_chan>(), GFP_KERNEL, dev_to_node(dev));
    if idxd_chan.is_null() { return -ENOMEM; }
    let chan = &mut (*idxd_chan).chan;
    chan.device = dma;
    list_add_tail!(&mut chan.device_node, &mut dma.channels);
    for i in 0..(*wq).num_descs {
        let desc = *(*wq).descs.add(i);
        dma_async_tx_descriptor_init(&mut (*desc).txd, chan);
        (*desc).txd.tx_submit = Some(idxd_dma_tx_submit);
    }
    let rc = dma_async_device_channel_register(dma, chan, core::ptr::null_mut());
    if rc < 0 { kfree(idxd_chan); return rc; }
    (*wq).idxd_chan = idxd_chan;
    (*idxd_chan).wq = wq;
    get_device(wq_confdev(wq));
    0
}

unsafe fn idxd_unregister_dma_channel(wq: *mut idxd_wq) {
    let idxd_chan = (*wq).idxd_chan;
    let chan = &mut (*idxd_chan).chan;
    let idxd_dma = (*(*wq).idxd).idxd_dma;
    dma_async_device_channel_unregister(&mut (*idxd_dma).dma, chan);
    list_del!(&mut chan.device_node);
    kfree((*wq).idxd_chan);
    (*wq).idxd_chan = core::ptr::null_mut();
    put_device(wq_confdev(wq));
}

unsafe fn idxd_dmaengine_drv_probe(idxd_dev: *mut idxd_dev) -> i32 {
    let dev = &mut (*idxd_dev).conf_dev;
    let wq = idxd_dev_to_wq(idxd_dev);
    let idxd = (*wq).idxd;
    if (*idxd).state != IDXD_DEV_ENABLED { return -ENXIO; }
    mutex_lock(&mut (*wq).wq_lock);
    if !idxd_wq_driver_name_match(wq, dev) {
        (*idxd).cmd_status = IDXD_SCMD_WQ_NO_DRV_NAME;
        mutex_unlock(&mut (*wq).wq_lock);
        return -ENODEV;
    }
    (*wq).type_ = IDXD_WQT_KERNEL;
    let mut rc = idxd_drv_enable_wq(wq);
    if rc < 0 {
        dev_dbg!(dev, "Enable wq %d failed: %d\n", (*wq).id, rc);
        (*wq).type_ = IDXD_WQT_NONE;
        mutex_unlock(&mut (*wq).wq_lock);
        return -ENXIO;
    }
    rc = idxd_register_dma_channel(wq);
    if rc < 0 {
        (*idxd).cmd_status = IDXD_SCMD_DMA_CHAN_ERR;
        dev_dbg!(dev, "Failed to register dma channel\n");
        idxd_drv_disable_wq(wq);
        (*wq).type_ = IDXD_WQT_NONE;
        mutex_unlock(&mut (*wq).wq_lock);
        return rc;
    }
    (*idxd).cmd_status = 0;
    mutex_unlock(&mut (*wq).wq_lock);
    0
}

unsafe fn idxd_dmaengine_drv_remove(idxd_dev: *mut idxd_dev) {
    let wq = idxd_dev_to_wq(idxd_dev);
    mutex_lock(&mut (*wq).wq_lock);
    __idxd_wq_quiesce(wq);
    idxd_unregister_dma_channel(wq);
    idxd_drv_disable_wq(wq);
    mutex_unlock(&mut (*wq).wq_lock);
}

// EXPORT_SYMBOL_GPL(idxd_dmaengine_drv);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
