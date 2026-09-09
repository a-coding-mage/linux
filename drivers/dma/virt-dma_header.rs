/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Virtual DMA channel support for DMAengine
 *
 * Copyright (C) 2012 Russell King
 */

// Dependencies supplied by the Linux DMAengine implementation.

#[repr(C)]
pub struct virt_dma_desc {
    pub tx: dma_async_tx_descriptor,
    pub tx_result: dmaengine_result,
    /* protected by vc.lock */
    pub node: list_head,
}

#[repr(C)]
pub struct virt_dma_chan {
    pub chan: dma_chan,
    pub task: tasklet_struct,
    pub desc_free: Option<unsafe extern "C" fn(*mut virt_dma_desc)>,

    pub lock: spinlock_t,

    /* protected by vc.lock */
    pub desc_allocated: list_head,
    pub desc_submitted: list_head,
    pub desc_issued: list_head,
    pub desc_completed: list_head,
    pub desc_terminated: list_head,

    pub cyclic: *mut virt_dma_desc,
}

#[inline]
pub unsafe fn to_virt_chan(chan: *mut dma_chan) -> *mut virt_dma_chan {
    container_of!(chan, virt_dma_chan, chan)
}

unsafe extern "C" {
    pub fn vchan_dma_desc_free_list(vc: *mut virt_dma_chan, head: *mut list_head);
    pub fn vchan_init(vc: *mut virt_dma_chan, dmadev: *mut dma_device);
    pub fn vchan_find_desc(vc: *mut virt_dma_chan, cookie: dma_cookie_t) -> *mut virt_dma_desc;
    pub fn vchan_tx_submit(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t;
    pub fn vchan_tx_desc_free(tx: *mut dma_async_tx_descriptor) -> i32;
}

/** vchan_tx_prep - prepare a descriptor */
#[inline]
pub unsafe fn vchan_tx_prep(
    vc: *mut virt_dma_chan,
    vd: *mut virt_dma_desc,
    tx_flags: c_ulong,
) -> *mut dma_async_tx_descriptor {
    let mut flags: c_ulong = 0;

    dma_async_tx_descriptor_init(&mut (*vd).tx, &mut (*vc).chan);
    (*vd).tx.flags = tx_flags;
    (*vd).tx.tx_submit = Some(vchan_tx_submit);
    (*vd).tx.desc_free = Some(vchan_tx_desc_free);

    (*vd).tx_result.result = DMA_TRANS_NOERROR;
    (*vd).tx_result.residue = 0;

    spin_lock_irqsave(&mut (*vc).lock, &mut flags);
    list_add_tail(&mut (*vd).node, &mut (*vc).desc_allocated);
    spin_unlock_irqrestore(&mut (*vc).lock, flags);

    &mut (*vd).tx
}

/** vchan_issue_pending - move submitted descriptors to issued list */
#[inline]
pub unsafe fn vchan_issue_pending(vc: *mut virt_dma_chan) -> bool {
    lockdep_assert_held(&(*vc).lock);

    list_splice_tail_init(&mut (*vc).desc_submitted, &mut (*vc).desc_issued);
    !list_empty(&(*vc).desc_issued)
}

/** vchan_cookie_complete - report completion of a descriptor */
#[inline]
pub unsafe fn vchan_cookie_complete(vd: *mut virt_dma_desc) {
    let vc = to_virt_chan((*vd).tx.chan);
    let cookie: dma_cookie_t;

    lockdep_assert_held(&(*vc).lock);

    cookie = (*vd).tx.cookie;
    dma_cookie_complete(&mut (*vd).tx);
    dev_vdbg((*vc).chan.device.dev, "txd %p[%x]: marked complete\n", vd, cookie);
    list_add_tail(&mut (*vd).node, &mut (*vc).desc_completed);

    tasklet_schedule(&mut (*vc).task);
}

/** vchan_vdesc_fini - Free or reuse a descriptor */
#[inline]
pub unsafe fn vchan_vdesc_fini(vd: *mut virt_dma_desc) {
    let vc = to_virt_chan((*vd).tx.chan);

    if dmaengine_desc_test_reuse(&(*vd).tx) {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*vc).lock, &mut flags);
        list_add(&mut (*vd).node, &mut (*vc).desc_allocated);
        spin_unlock_irqrestore(&mut (*vc).lock, flags);
    } else if let Some(desc_free) = (*vc).desc_free {
        desc_free(vd);
    }
}

/** vchan_cyclic_callback - report the completion of a period */
#[inline]
pub unsafe fn vchan_cyclic_callback(vd: *mut virt_dma_desc) {
    let vc = to_virt_chan((*vd).tx.chan);

    (*vc).cyclic = vd;
    tasklet_schedule(&mut (*vc).task);
}

/** vchan_terminate_vdesc - Disable pending cyclic callback */
#[inline]
pub unsafe fn vchan_terminate_vdesc(vd: *mut virt_dma_desc) {
    let vc = to_virt_chan((*vd).tx.chan);

    lockdep_assert_held(&(*vc).lock);

    list_add_tail(&mut (*vd).node, &mut (*vc).desc_terminated);

    if (*vc).cyclic == vd {
        (*vc).cyclic = core::ptr::null_mut();
    }
}

/** vchan_next_desc - peek at the next descriptor to be processed */
#[inline]
pub unsafe fn vchan_next_desc(vc: *mut virt_dma_chan) -> *mut virt_dma_desc {
    lockdep_assert_held(&(*vc).lock);

    list_first_entry_or_null!(&mut (*vc).desc_issued, virt_dma_desc, node)
}

/** vchan_get_all_descriptors - obtain all submitted and issued descriptors */
#[inline]
pub unsafe fn vchan_get_all_descriptors(vc: *mut virt_dma_chan, head: *mut list_head) {
    lockdep_assert_held(&(*vc).lock);

    list_splice_tail_init(&mut (*vc).desc_allocated, head);
    list_splice_tail_init(&mut (*vc).desc_submitted, head);
    list_splice_tail_init(&mut (*vc).desc_issued, head);
    list_splice_tail_init(&mut (*vc).desc_completed, head);
    list_splice_tail_init(&mut (*vc).desc_terminated, head);
}

#[inline]
pub unsafe fn vchan_free_chan_resources(vc: *mut virt_dma_chan) {
    let mut vd: *mut virt_dma_desc;
    let mut flags: c_ulong = 0;
    let mut head: list_head = LIST_HEAD_INIT;

    spin_lock_irqsave(&mut (*vc).lock, &mut flags);
    vchan_get_all_descriptors(vc, &mut head);
    list_for_each_entry!(vd, &mut head, node, {
        dmaengine_desc_clear_reuse(&mut (*vd).tx);
    });
    spin_unlock_irqrestore(&mut (*vc).lock, flags);

    vchan_dma_desc_free_list(vc, &mut head);
}

/** vchan_synchronize - synchronize callback execution to the current context */
#[inline]
pub unsafe fn vchan_synchronize(vc: *mut virt_dma_chan) {
    let mut head: list_head = LIST_HEAD_INIT;
    let mut flags: c_ulong = 0;

    tasklet_kill(&mut (*vc).task);

    spin_lock_irqsave(&mut (*vc).lock, &mut flags);
    list_splice_tail_init(&mut (*vc).desc_terminated, &mut head);
    spin_unlock_irqrestore(&mut (*vc).lock, flags);

    vchan_dma_desc_free_list(vc, &mut head);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
