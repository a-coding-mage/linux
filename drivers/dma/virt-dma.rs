// SPDX-License-Identifier: GPL-2.0-only
/*
 * Virtual DMA channel support for DMAengine
 *
 * Copyright (C) 2012 Russell King
 */

// Dependencies are supplied by the surrounding kernel translation.

unsafe fn to_virt_desc(
    tx: *mut dma_async_tx_descriptor,
) -> *mut virt_dma_desc {
    container_of!(tx, virt_dma_desc, tx)
}

pub unsafe extern "C" fn vchan_tx_submit(
    tx: *mut dma_async_tx_descriptor,
) -> dma_cookie_t {
    let vc: *mut virt_dma_chan = to_virt_chan((*tx).chan);
    let vd: *mut virt_dma_desc = to_virt_desc(tx);
    let mut flags: c_ulong = 0;
    let cookie: dma_cookie_t;

    spin_lock_irqsave(&mut (*vc).lock, &mut flags);
    cookie = dma_cookie_assign(tx);

    list_move_tail(&mut (*vd).node, &mut (*vc).desc_submitted);
    spin_unlock_irqrestore(&mut (*vc).lock, flags);

    dev_dbg(
        (*(*vc).chan.device).dev,
        "vchan %p: txd %p[%x]: submitted\n",
        vc,
        vd,
        cookie,
    );

    cookie
}

// EXPORT_SYMBOL_GPL(vchan_tx_submit);

/**
 * vchan_tx_desc_free - free a reusable descriptor
 * @tx: the transfer
 *
 * This function frees a previously allocated reusable descriptor. The only
 * other way is to clear the DMA_CTRL_REUSE flag and submit one last time the
 * transfer.
 *
 * Returns 0 upon success
 */
pub unsafe extern "C" fn vchan_tx_desc_free(
    tx: *mut dma_async_tx_descriptor,
) -> c_int {
    let vc: *mut virt_dma_chan = to_virt_chan((*tx).chan);
    let vd: *mut virt_dma_desc = to_virt_desc(tx);
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*vc).lock, &mut flags);
    list_del(&mut (*vd).node);
    spin_unlock_irqrestore(&mut (*vc).lock, flags);

    dev_dbg(
        (*(*vc).chan.device).dev,
        "vchan %p: txd %p[%x]: freeing\n",
        vc,
        vd,
        (*vd).tx.cookie,
    );
    ((*vc).desc_free)(vd);
    0
}

// EXPORT_SYMBOL_GPL(vchan_tx_desc_free);

pub unsafe extern "C" fn vchan_find_desc(
    vc: *mut virt_dma_chan,
    cookie: dma_cookie_t,
) -> *mut virt_dma_desc {
    // list_for_each_entry(vd, &vc->desc_issued, node)
    let mut vd: *mut virt_dma_desc = list_first_entry!(&mut (*vc).desc_issued, virt_dma_desc, node);
    while !vd.is_null() {
        if (*vd).tx.cookie == cookie {
            return vd;
        }
        vd = list_next_entry!(vd, node);
    }
    core::ptr::null_mut()
}

// EXPORT_SYMBOL_GPL(vchan_find_desc);

/*
 * This tasklet handles the completion of a DMA descriptor by
 * calling its callback and freeing it.
 */
unsafe fn vchan_complete(t: *mut tasklet_struct) {
    let vc: *mut virt_dma_chan = from_tasklet!(t, task, virt_dma_chan);
    let mut vd: *mut virt_dma_desc;
    let mut _vd: *mut virt_dma_desc;
    let mut cb: dmaengine_desc_callback;
    let mut head: list_head = LIST_HEAD_INIT!();

    spin_lock_irq(&mut (*vc).lock);
    list_splice_tail_init(&mut (*vc).desc_completed, &mut head);
    vd = (*vc).cyclic;
    if !vd.is_null() {
        (*vc).cyclic = core::ptr::null_mut();
        dmaengine_desc_get_callback(&mut (*vd).tx, &mut cb);
    } else {
        core::ptr::write_bytes(&mut cb as *mut _, 0, 1);
    }
    spin_unlock_irq(&mut (*vc).lock);

    dmaengine_desc_callback_invoke(&mut cb, &mut (*vd).tx_result);

    // list_for_each_entry_safe(vd, _vd, &head, node)
    vd = list_first_entry!(&mut head, virt_dma_desc, node);
    while !vd.is_null() {
        _vd = list_next_entry!(vd, node);
        dmaengine_desc_get_callback(&mut (*vd).tx, &mut cb);
        list_del(&mut (*vd).node);
        dmaengine_desc_callback_invoke(&mut cb, &mut (*vd).tx_result);
        vchan_vdesc_fini(vd);
        vd = _vd;
    }
}

pub unsafe extern "C" fn vchan_dma_desc_free_list(
    vc: *mut virt_dma_chan,
    head: *mut list_head,
) {
    // list_for_each_entry_safe(vd, _vd, head, node)
    let mut vd: *mut virt_dma_desc = list_first_entry!(head, virt_dma_desc, node);
    while !vd.is_null() {
        let _vd: *mut virt_dma_desc = list_next_entry!(vd, node);
        list_del(&mut (*vd).node);
        vchan_vdesc_fini(vd);
        vd = _vd;
    }
    let _ = vc;
}

// EXPORT_SYMBOL_GPL(vchan_dma_desc_free_list);

pub unsafe extern "C" fn vchan_init(
    vc: *mut virt_dma_chan,
    dmadev: *mut dma_device,
) {
    dma_cookie_init(&mut (*vc).chan);

    spin_lock_init(&mut (*vc).lock);
    init_list_head(&mut (*vc).desc_allocated);
    init_list_head(&mut (*vc).desc_submitted);
    init_list_head(&mut (*vc).desc_issued);
    init_list_head(&mut (*vc).desc_completed);
    init_list_head(&mut (*vc).desc_terminated);

    tasklet_setup(&mut (*vc).task, vchan_complete);

    (*vc).chan.device = dmadev;
    list_add_tail(&mut (*vc).chan.device_node, &mut (*dmadev).channels);
}

// EXPORT_SYMBOL_GPL(vchan_init);

// MODULE_AUTHOR("Russell King");
// MODULE_DESCRIPTION("Virtual DMA channel support for DMAengine");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
