// SPDX-License-Identifier: GPL-2.0-only
/*
 * core routines for the asynchronous memory transfer/transform api
 *
 * Copyright © 2006, Intel Corporation.
 *
 *	Dan Williams <dan.j.williams@intel.com>
 *
 *	with architecture considerations by:
 *	Neil Brown <neilb@suse.de>
 *	Jeff Garzik <jeff@garzik.org>
 */

// C dependencies supplied by the kernel headers are intentionally external.

#[cfg(feature = "CONFIG_DMA_ENGINE")]
unsafe fn async_tx_init() -> i32 {
    async_dmaengine_get();
    printk(KERN_INFO, "async_tx: api initialized (async)\n");
    0
}

#[cfg(feature = "CONFIG_DMA_ENGINE")]
unsafe fn async_tx_exit() {
    async_dmaengine_put();
}

#[cfg(feature = "CONFIG_DMA_ENGINE")]
pub unsafe fn __async_tx_find_channel(
    submit: *mut async_submit_ctl,
    tx_type: dma_transaction_type,
) -> *mut dma_chan {
    let depend_tx = (*submit).depend_tx;
    if !depend_tx.is_null()
        && dma_has_cap(tx_type, (*(*depend_tx).chan).device.as_ref().unwrap().cap_mask)
    {
        return (*depend_tx).chan;
    }
    async_dma_find_channel(tx_type)
}

unsafe fn async_tx_channel_switch(
    depend_tx: *mut dma_async_tx_descriptor,
    tx: *mut dma_async_tx_descriptor,
) {
    let chan = (*depend_tx).chan;
    let device = (*chan).device;
    let mut intr_tx = !0usize as *mut dma_async_tx_descriptor;

    txd_lock(depend_tx);
    if !txd_parent(depend_tx).is_null() && (*depend_tx).chan == (*tx).chan {
        txd_chain(depend_tx, tx);
        intr_tx = core::ptr::null_mut();
    }
    txd_unlock(depend_tx);

    if intr_tx.is_null() {
        ((*device).device_issue_pending)(chan);
        return;
    }

    if dma_has_cap(DMA_INTERRUPT, (*device).cap_mask) {
        intr_tx = ((*device).device_prep_dma_interrupt)(chan, 0);
    } else {
        intr_tx = core::ptr::null_mut();
    }

    if !intr_tx.is_null() {
        (*intr_tx).callback = None;
        (*intr_tx).callback_param = core::ptr::null_mut();
        txd_chain(intr_tx, tx);

        txd_lock(depend_tx);
        if !txd_parent(depend_tx).is_null() {
            txd_chain(depend_tx, intr_tx);
            async_tx_ack(intr_tx);
            intr_tx = core::ptr::null_mut();
        }
        txd_unlock(depend_tx);

        if !intr_tx.is_null() {
            txd_clear_parent(intr_tx);
            ((*intr_tx).tx_submit)(intr_tx);
            async_tx_ack(intr_tx);
        }
        ((*device).device_issue_pending)(chan);
    } else {
        if dma_wait_for_async_tx(depend_tx) != DMA_COMPLETE {
            panic!("{}: DMA error waiting for depend_tx\n", "async_tx_channel_switch");
        }
        ((*tx).tx_submit)(tx);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
enum submit_disposition {
    ASYNC_TX_SUBMITTED,
    ASYNC_TX_CHANNEL_SWITCH,
    ASYNC_TX_DIRECT_SUBMIT,
}

pub unsafe fn async_tx_submit(
    chan: *mut dma_chan,
    tx: *mut dma_async_tx_descriptor,
    submit: *mut async_submit_ctl,
) {
    let depend_tx = (*submit).depend_tx;
    (*tx).callback = (*submit).cb_fn;
    (*tx).callback_param = (*submit).cb_param;

    if !depend_tx.is_null() {
        assert!(!async_tx_test_ack(depend_tx) && txd_next(depend_tx).is_null() && txd_parent(tx).is_null());
        txd_lock(depend_tx);
        let s = if !txd_parent(depend_tx).is_null() {
            if (*depend_tx).chan == chan {
                txd_chain(depend_tx, tx);
                submit_disposition::ASYNC_TX_SUBMITTED
            } else {
                submit_disposition::ASYNC_TX_CHANNEL_SWITCH
            }
        } else if (*depend_tx).chan == chan {
            submit_disposition::ASYNC_TX_DIRECT_SUBMIT
        } else {
            submit_disposition::ASYNC_TX_CHANNEL_SWITCH
        };
        txd_unlock(depend_tx);
        match s {
            submit_disposition::ASYNC_TX_SUBMITTED => {}
            submit_disposition::ASYNC_TX_CHANNEL_SWITCH => async_tx_channel_switch(depend_tx, tx),
            submit_disposition::ASYNC_TX_DIRECT_SUBMIT => {
                txd_clear_parent(tx);
                ((*tx).tx_submit)(tx);
            }
        }
    } else {
        txd_clear_parent(tx);
        ((*tx).tx_submit)(tx);
    }

    if ((*submit).flags & ASYNC_TX_ACK) != 0 {
        async_tx_ack(tx);
    }
    if !depend_tx.is_null() {
        async_tx_ack(depend_tx);
    }
}

pub unsafe fn async_trigger_callback(
    submit: *mut async_submit_ctl,
) -> *mut dma_async_tx_descriptor {
    let depend_tx = (*submit).depend_tx;
    let (chan, device) = if !depend_tx.is_null() {
        let chan = (*depend_tx).chan;
        let mut device = (*chan).device;
        if device.is_null() || !dma_has_cap(DMA_INTERRUPT, (*device).cap_mask) {
            device = core::ptr::null_mut();
        }
        (chan, device)
    } else {
        (core::ptr::null_mut(), core::ptr::null_mut())
    };
    let tx = if !device.is_null() {
        ((*device).device_prep_dma_interrupt)(chan, 0)
    } else {
        core::ptr::null_mut()
    };
    if !tx.is_null() {
        async_tx_submit(chan, tx, submit);
    } else {
        async_tx_quiesce(&mut (*submit).depend_tx);
        async_tx_sync_epilog(submit);
    }
    tx
}

pub unsafe fn async_tx_quiesce(tx: *mut *mut dma_async_tx_descriptor) {
    if !(*tx).is_null() {
        assert!(!async_tx_test_ack(*tx));
        if dma_wait_for_async_tx(*tx) != DMA_COMPLETE {
            panic!("{}: DMA error waiting for transaction\n", "async_tx_quiesce");
        }
        async_tx_ack(*tx);
        *tx = core::ptr::null_mut();
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
