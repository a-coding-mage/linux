// SPDX-License-Identifier: GPL-2.0-only
/*
 * xor offload engine api
 *
 * Copyright © 2006, Intel Corporation.
 *
 *      Dan Williams <dan.j.williams@intel.com>
 *
 *      with architecture considerations by:
 *      Neil Brown <neilb@suse.de>
 *      Jeff Garzik <jeff@garzik.org>
 */

/* External kernel types, constants, and functions are supplied by the surrounding crate. */

unsafe fn do_async_xor(
    chan: *mut dma_chan,
    unmap: *mut dmaengine_unmap_data,
    submit: *mut async_submit_ctl,
) -> *mut dma_async_tx_descriptor {
    let dma = (*chan).device;
    let mut tx: *mut dma_async_tx_descriptor = core::ptr::null_mut();
    let cb_fn_orig = (*submit).cb_fn;
    let cb_param_orig = (*submit).cb_param;
    let flags_orig = (*submit).flags;
    let mut dma_flags: dma_ctrl_flags = 0;
    let mut src_cnt = (*unmap).to_cnt;
    let mut xor_src_cnt: i32;
    let dma_dest = (*unmap).addr[(*unmap).to_cnt as usize];
    let mut src_list = (*unmap).addr.as_mut_ptr();

    while src_cnt != 0 {
        let tmp: dma_addr_t;

        (*submit).flags = flags_orig;
        xor_src_cnt = core::cmp::min(src_cnt, (*dma).max_xor as i32);
        /* if we are submitting additional xors, leave the chain open
         * and clear the callback parameters
         */
        if src_cnt > xor_src_cnt {
            (*submit).flags &= !ASYNC_TX_ACK;
            (*submit).flags |= ASYNC_TX_FENCE;
            (*submit).cb_fn = None;
            (*submit).cb_param = core::ptr::null_mut();
        } else {
            (*submit).cb_fn = cb_fn_orig;
            (*submit).cb_param = cb_param_orig;
        }
        if (*submit).cb_fn.is_some() {
            dma_flags |= DMA_PREP_INTERRUPT;
        }
        if (*submit).flags & ASYNC_TX_FENCE != 0 {
            dma_flags |= DMA_PREP_FENCE;
        }

        /* Drivers force forward progress in case they can not provide a
         * descriptor
         */
        tmp = *src_list;
        if src_list != (*unmap).addr.as_mut_ptr() {
            *src_list = dma_dest;
        }
        tx = ((*dma).device_prep_dma_xor)(
            chan, dma_dest, src_list, xor_src_cnt, (*unmap).len, dma_flags,
        );

        if tx.is_null() {
            async_tx_quiesce(&mut (*submit).depend_tx);
        }

        /* spin wait for the preceding transactions to complete */
        while tx.is_null() {
            dma_async_issue_pending(chan);
            tx = ((*dma).device_prep_dma_xor)(
                chan, dma_dest, src_list, xor_src_cnt, (*unmap).len, dma_flags,
            );
        }
        *src_list = tmp;

        dma_set_unmap(tx, unmap);
        async_tx_submit(chan, tx, submit);
        (*submit).depend_tx = tx;

        if src_cnt > xor_src_cnt {
            /* drop completed sources */
            src_cnt -= xor_src_cnt;
            /* use the intermediate result a source */
            src_cnt += 1;
            src_list = src_list.add((xor_src_cnt - 1) as usize);
        } else {
            break;
        }
    }

    tx
}

unsafe fn do_sync_xor_offs(
    dest: *mut page,
    offset: u32,
    src_list: *mut *mut page,
    src_offs: *mut u32,
    src_cnt: i32,
    len: usize,
    submit: *mut async_submit_ctl,
) {
    let mut xor_src_cnt = 0;
    let srcs: *mut *mut core::ffi::c_void;

    if !(*submit).scribble.is_null() {
        srcs = (*submit).scribble as *mut *mut core::ffi::c_void;
    } else {
        srcs = src_list as *mut *mut core::ffi::c_void;
    }

    /* convert to buffer pointers */
    for i in 0..src_cnt {
        let p = *src_list.add(i as usize);
        if !p.is_null() {
            *srcs.add(xor_src_cnt as usize) = (page_address(p) as *mut u8)
                .add(if !src_offs.is_null() { *src_offs.add(i as usize) } else { offset } as usize)
                as *mut core::ffi::c_void;
            xor_src_cnt += 1;
        }
    }

    /* set destination address */
    let dest_buf = (page_address(dest) as *mut u8).add(offset as usize);
    if (*submit).flags & ASYNC_TX_XOR_ZERO_DST != 0 {
        core::ptr::write_bytes(dest_buf, 0, len);
    }
    xor_gen(dest_buf as *mut core::ffi::c_void, srcs, xor_src_cnt, len);
    async_tx_sync_epilog(submit);
}

unsafe fn dma_xor_aligned_offsets(
    device: *mut dma_device,
    offset: u32,
    src_offs: *mut u32,
    src_cnt: i32,
    len: i32,
) -> bool {
    if !is_dma_xor_aligned(device, offset, 0, len) {
        return false;
    }
    if src_offs.is_null() {
        return true;
    }
    for i in 0..src_cnt {
        if !is_dma_xor_aligned(device, *src_offs.add(i as usize), 0, len) {
            return false;
        }
    }
    true
}

pub unsafe fn async_xor_offs(
    dest: *mut page, offset: u32, src_list: *mut *mut page, src_offs: *mut u32,
    mut src_cnt: i32, len: usize, submit: *mut async_submit_ctl,
) -> *mut dma_async_tx_descriptor {
    let chan = async_tx_find_channel(submit, DMA_XOR, &dest, 1, src_list, src_cnt, len);
    let device = if !chan.is_null() { (*chan).device } else { core::ptr::null_mut() };
    let mut unmap: *mut dmaengine_unmap_data = core::ptr::null_mut();

    BUG_ON(src_cnt <= 1);
    if !device.is_null() {
        unmap = dmaengine_get_unmap_data((*device).dev, (src_cnt + 1) as usize, GFP_NOWAIT);
    }
    if !unmap.is_null() && dma_xor_aligned_offsets(device, offset, src_offs, src_cnt, len as i32) {
        let mut j = 0;
        (*unmap).len = len;
        for i in 0..src_cnt {
            if (*src_list.add(i as usize)).is_null() { continue; }
            (*unmap).to_cnt += 1;
            (*unmap).addr[j] = dma_map_page((*device).dev, *src_list.add(i as usize),
                if !src_offs.is_null() { *src_offs.add(i as usize) } else { offset }, len, DMA_TO_DEVICE);
            j += 1;
        }
        (*unmap).addr[j] = dma_map_page((*device).dev, dest, offset, len, DMA_BIDIRECTIONAL);
        (*unmap).bidi_cnt = 1;
        let tx = do_async_xor(chan, unmap, submit);
        dmaengine_unmap_put(unmap);
        tx
    } else {
        dmaengine_unmap_put(unmap);
        if (*submit).flags & ASYNC_TX_XOR_DROP_DST != 0 {
            src_cnt -= 1; let _ = src_list.add(1); if !src_offs.is_null() { let _ = src_offs.add(1); }
        }
        async_tx_quiesce(&mut (*submit).depend_tx);
        do_sync_xor_offs(dest, offset, src_list, src_offs, src_cnt, len, submit);
        core::ptr::null_mut()
    }
}

pub unsafe fn async_xor(dest: *mut page, src_list: *mut *mut page, offset: u32,
                        src_cnt: i32, len: usize, submit: *mut async_submit_ctl)
                        -> *mut dma_async_tx_descriptor {
    async_xor_offs(dest, offset, src_list, core::ptr::null_mut(), src_cnt, len, submit)
}

unsafe fn page_is_zero(p: *mut page, offset: u32, len: usize) -> i32 {
    (!memchr_inv((page_address(p) as *mut u8).add(offset as usize), 0, len)) as i32
}

unsafe fn xor_val_chan(submit: *mut async_submit_ctl, dest: *mut page,
                       src_list: *mut *mut page, src_cnt: i32, len: usize) -> *mut dma_chan {
    #[cfg(not(CONFIG_ASYNC_TX_DISABLE_XOR_VAL_DMA))]
    { async_tx_find_channel(submit, DMA_XOR_VAL, &dest, 1, src_list, src_cnt, len) }
    #[cfg(CONFIG_ASYNC_TX_DISABLE_XOR_VAL_DMA)]
    { let _ = (submit, dest, src_list, src_cnt, len); core::ptr::null_mut() }
}

pub unsafe fn async_xor_val_offs(
    dest: *mut page, offset: u32, src_list: *mut *mut page, src_offs: *mut u32,
    src_cnt: i32, len: usize, result: *mut sum_check_flags, submit: *mut async_submit_ctl,
) -> *mut dma_async_tx_descriptor {
    let chan = xor_val_chan(submit, dest, src_list, src_cnt, len);
    let device = if !chan.is_null() { (*chan).device } else { core::ptr::null_mut() };
    let mut tx = core::ptr::null_mut();
    let mut unmap = core::ptr::null_mut();
    BUG_ON(src_cnt <= 1);
    if !device.is_null() { unmap = dmaengine_get_unmap_data((*device).dev, src_cnt as usize, GFP_NOWAIT); }
    if !unmap.is_null() && src_cnt <= (*device).max_xor && dma_xor_aligned_offsets(device, offset, src_offs, src_cnt, len as i32) {
        let mut flags = 0;
        if (*submit).cb_fn.is_some() { flags |= DMA_PREP_INTERRUPT; }
        if (*submit).flags & ASYNC_TX_FENCE != 0 { flags |= DMA_PREP_FENCE; }
        for i in 0..src_cnt {
            (*unmap).addr[i as usize] = dma_map_page((*device).dev, *src_list.add(i as usize), if !src_offs.is_null() { *src_offs.add(i as usize) } else { offset }, len, DMA_TO_DEVICE);
            (*unmap).to_cnt += 1;
        }
        (*unmap).len = len;
        tx = ((*device).device_prep_dma_xor_val)(chan, (*unmap).addr.as_mut_ptr(), src_cnt, len, result, flags);
        if tx.is_null() { async_tx_quiesce(&mut (*submit).depend_tx); while tx.is_null() { dma_async_issue_pending(chan); tx = ((*device).device_prep_dma_xor_val)(chan, (*unmap).addr.as_mut_ptr(), src_cnt, len, result, flags); } }
        dma_set_unmap(tx, unmap); async_tx_submit(chan, tx, submit);
    } else {
        let flags_orig = (*submit).flags;
        (*submit).flags |= ASYNC_TX_XOR_DROP_DST; (*submit).flags &= !ASYNC_TX_ACK;
        tx = async_xor_offs(dest, offset, src_list, src_offs, src_cnt, len, submit);
        async_tx_quiesce(&mut tx);
        *result = (page_is_zero(dest, offset, len) == 0) as _ << SUM_CHECK_P;
        async_tx_sync_epilog(submit); (*submit).flags = flags_orig;
    }
    dmaengine_unmap_put(unmap); tx
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
