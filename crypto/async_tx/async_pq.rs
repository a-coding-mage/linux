// SPDX-License-Identifier: GPL-2.0-or-later
/* Translated from async_pq.c. Kernel types and functions are supplied externally. */

use core::ffi::c_void;

const MAX_DISKS: usize = 255;
static mut PQ_SCRIBBLE_PAGE: *mut page = core::ptr::null_mut();

#[inline]
unsafe fn p(b: *mut *mut page, d: i32) -> *mut page { *b.add((d - 2) as usize) }
#[inline]
unsafe fn q(b: *mut *mut page, d: i32) -> *mut page { *b.add((d - 1) as usize) }
#[inline]
unsafe fn set_p(b: *mut *mut page, d: i32, v: *mut page) { *b.add((d - 2) as usize) = v; }
#[inline]
unsafe fn set_q(b: *mut *mut page, d: i32, v: *mut page) { *b.add((d - 1) as usize) = v; }

unsafe fn do_async_gen_syndrome(chan: *mut dma_chan, scfs: *const u8, disks: i32,
    unmap: *mut dmaengine_unmap_data, dma_flags: dma_ctrl_flags,
    submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor {
    let mut tx: *mut dma_async_tx_descriptor = core::ptr::null_mut();
    let dma = (*chan).device;
    let flags_orig = (*submit).flags;
    let cb_fn_orig = (*submit).cb_fn;
    let cb_param_orig = (*submit).cb_param;
    let mut src_cnt = disks - 2;
    let mut src_off = 0usize;
    while src_cnt > 0 {
        (*submit).flags = flags_orig;
        let pq_src_cnt = core::cmp::min(src_cnt as u16, dma_maxpq(dma, dma_flags));
        if src_cnt > pq_src_cnt as i32 {
            (*submit).flags &= !ASYNC_TX_ACK;
            (*submit).flags |= ASYNC_TX_FENCE;
            (*submit).cb_fn = None;
            (*submit).cb_param = core::ptr::null_mut();
        } else {
            (*submit).cb_fn = cb_fn_orig;
            (*submit).cb_param = cb_param_orig;
            if cb_fn_orig.is_some() { dma_flags |= DMA_PREP_INTERRUPT; }
        }
        if (*submit).flags & ASYNC_TX_FENCE != 0 { dma_flags |= DMA_PREP_FENCE; }
        loop {
            let mut dest = [(*unmap).addr[(disks - 2) as usize], (*unmap).addr[(disks - 1) as usize]];
            tx = ((*dma).device_prep_dma_pq.unwrap())(chan, dest.as_mut_ptr(),
                (*unmap).addr.add(src_off), pq_src_cnt, scfs.add(src_off), (*unmap).len, dma_flags);
            if !tx.is_null() { break; }
            async_tx_quiesce(&mut (*submit).depend_tx);
            dma_async_issue_pending(chan);
        }
        dma_set_unmap(tx, unmap);
        async_tx_submit(chan, tx, submit);
        (*submit).depend_tx = tx;
        src_cnt -= pq_src_cnt as i32;
        src_off += pq_src_cnt as usize;
        dma_flags |= DMA_PREP_CONTINUE;
    }
    tx
}

unsafe fn do_sync_gen_syndrome(blocks: *mut *mut page, offsets: *mut u32, disks: i32,
    len: usize, submit: *mut async_submit_ctl) {
    let srcs = if !(*submit).scribble.is_null() { (*submit).scribble as *mut *mut c_void } else { blocks as *mut *mut c_void };
    let mut start = -1i32;
    let mut stop = disks - 3;
    for i in 0..disks {
        let b = *blocks.add(i as usize);
        if b.is_null() {
            BUG_ON(i > disks - 3);
            *srcs.add(i as usize) = page_address(ZERO_PAGE(0));
        } else {
            *srcs.add(i as usize) = (page_address(b) as *mut u8).add(*offsets.add(i as usize) as usize) as *mut c_void;
            if i < disks - 2 { stop = i; if start == -1 { start = i; } }
        }
    }
    if (*submit).flags & ASYNC_TX_PQ_XOR_DST != 0 {
        BUG_ON(!raid6_can_xor_syndrome());
        if start >= 0 { raid6_xor_syndrome(disks, start, stop, len, srcs); }
    } else { raid6_gen_syndrome(disks, len, srcs); }
    async_tx_sync_epilog(submit);
}

unsafe fn is_dma_pq_aligned_offs(dev: *mut dma_device, offs: *mut u32, src_cnt: i32, len: usize) -> bool {
    for i in 0..src_cnt { if !is_dma_pq_aligned(dev, *offs.add(i as usize), 0, len) { return false; } }
    true
}

#[no_mangle]
pub unsafe extern "C" fn async_gen_syndrome(blocks: *mut *mut page, offsets: *mut u32, disks: i32,
    len: usize, submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor {
    let src_cnt = disks - 2;
    let chan = async_tx_find_channel(submit, DMA_PQ, blocks.add((disks - 2) as usize), 2, blocks, src_cnt, len);
    let device = if chan.is_null() { core::ptr::null_mut() } else { (*chan).device };
    let mut unmap = if !device.is_null() { dmaengine_get_unmap_data((*device).dev, disks, GFP_NOWAIT) } else { core::ptr::null_mut() };
    BUG_ON(disks > MAX_DISKS as i32 || (p(blocks, disks).is_null() && q(blocks, disks).is_null()));
    if !unmap.is_null() && (*submit).flags & ASYNC_TX_PQ_XOR_DST == 0 &&
       (src_cnt <= dma_maxpq(device, 0) as i32 || dma_maxpq(device, DMA_PREP_CONTINUE) > 0) &&
       is_dma_pq_aligned_offs(device, offsets, disks, len) {
        let mut coefs = [0u8; MAX_DISKS]; let mut j = 0usize;
        (*unmap).len = len;
        for i in 0..src_cnt { if !(*blocks.add(i as usize)).is_null() { (*unmap).addr[j] = dma_map_page((*device).dev, *blocks.add(i as usize), *offsets.add(i as usize), len, DMA_TO_DEVICE); coefs[j] = raid6_gfexp[i as usize]; (*unmap).to_cnt += 1; j += 1; } }
        (*unmap).bidi_cnt += 1; if !p(blocks, disks).is_null() { (*unmap).addr[j] = dma_map_page((*device).dev, p(blocks, disks), *offsets.add((disks-2) as usize), len, DMA_BIDIRECTIONAL); } else { (*unmap).addr[j] = 0; } j += 1;
        (*unmap).bidi_cnt += 1; if !q(blocks, disks).is_null() { (*unmap).addr[j] = dma_map_page((*device).dev, q(blocks, disks), *offsets.add((disks-1) as usize), len, DMA_BIDIRECTIONAL); } else { (*unmap).addr[j] = 0; } j += 1;
        let tx = do_async_gen_syndrome(chan, coefs.as_ptr(), j as i32, unmap, 0, submit); dmaengine_unmap_put(unmap); return tx;
    }
    dmaengine_unmap_put(unmap);
    async_tx_quiesce(&mut (*submit).depend_tx);
    if p(blocks, disks).is_null() { set_p(blocks, disks, PQ_SCRIBBLE_PAGE); *offsets.add((disks-2) as usize) = 0; }
    if q(blocks, disks).is_null() { set_q(blocks, disks, PQ_SCRIBBLE_PAGE); *offsets.add((disks-1) as usize) = 0; }
    do_sync_gen_syndrome(blocks, offsets, disks, len, submit); core::ptr::null_mut()
}

unsafe fn pq_val_chan(submit: *mut async_submit_ctl, blocks: *mut *mut page, disks: i32, len: usize) -> *mut dma_chan {
    // CONFIG_ASYNC_TX_DISABLE_PQ_VAL_DMA makes this return NULL.
    async_tx_find_channel(submit, DMA_PQ_VAL, core::ptr::null_mut(), 0, blocks, disks, len)
}

#[no_mangle]
pub unsafe extern "C" fn async_syndrome_val(blocks: *mut *mut page, offsets: *mut u32, disks: i32,
    len: usize, pqres: *mut sum_check_flags, spare: *mut page, s_off: u32,
    submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor {
    let chan = pq_val_chan(submit, blocks, disks, len);
    let device = if chan.is_null() { core::ptr::null_mut() } else { (*chan).device };
    let mut tx: *mut dma_async_tx_descriptor = core::ptr::null_mut();
    let mut unmap = if !device.is_null() { dmaengine_get_unmap_data((*device).dev, disks, GFP_NOWAIT) } else { core::ptr::null_mut() };
    BUG_ON(disks < 4 || disks > MAX_DISKS as i32);
    if !unmap.is_null() && disks <= dma_maxpq(device, 0) as i32 && is_dma_pq_aligned_offs(device, offsets, disks, len) {
        let mut coefs = [0u8; MAX_DISKS]; let mut pq = [0 as dma_addr_t; 2]; let mut j = 0usize; let mut src_cnt = 0i32;
        (*unmap).len = len;
        for i in 0..disks-2 { if !(*blocks.add(i as usize)).is_null() { (*unmap).addr[j] = dma_map_page((*device).dev, *blocks.add(i as usize), *offsets.add(i as usize), len, DMA_TO_DEVICE); coefs[j] = raid6_gfexp[i as usize]; (*unmap).to_cnt += 1; src_cnt += 1; j += 1; } }
        let mut flags = if (*submit).cb_fn.is_some() { DMA_PREP_INTERRUPT } else { 0 };
        if !p(blocks, disks).is_null() { pq[0] = dma_map_page((*device).dev, p(blocks, disks), *offsets.add((disks-2) as usize), len, DMA_TO_DEVICE); (*unmap).addr[j] = pq[0]; (*unmap).to_cnt += 1; j += 1; } else { flags |= DMA_PREP_PQ_DISABLE_P; }
        if !q(blocks, disks).is_null() { pq[1] = dma_map_page((*device).dev, q(blocks, disks), *offsets.add((disks-1) as usize), len, DMA_TO_DEVICE); (*unmap).addr[j] = pq[1]; (*unmap).to_cnt += 1; j += 1; } else { flags |= DMA_PREP_PQ_DISABLE_Q; }
        if (*submit).flags & ASYNC_TX_FENCE != 0 { flags |= DMA_PREP_FENCE; }
        loop { tx = ((*device).device_prep_dma_pq_val.unwrap())(chan, pq.as_mut_ptr(), (*unmap).addr, src_cnt, coefs.as_ptr(), len, pqres, flags); if !tx.is_null() { break; } async_tx_quiesce(&mut (*submit).depend_tx); dma_async_issue_pending(chan); }
        dma_set_unmap(tx, unmap); async_tx_submit(chan, tx, submit);
    } else {
        let p_src = p(blocks, disks); let p_off = *offsets.add((disks-2) as usize); let q_src = q(blocks, disks); let q_off = *offsets.add((disks-1) as usize);
        let flags_orig = (*submit).flags; let cb_fn_orig = (*submit).cb_fn; let cb_param_orig = (*submit).cb_param; let scribble = (*submit).scribble;
        BUG_ON(spare.is_null() || scribble.is_null()); async_tx_quiesce(&mut (*submit).depend_tx); *pqres = 0;
        if !p_src.is_null() { init_async_submit(submit, ASYNC_TX_XOR_ZERO_DST, None, core::ptr::null_mut(), core::ptr::null_mut(), scribble); tx = async_xor_offs(spare, s_off, blocks, offsets, disks-2, len, submit); async_tx_quiesce(&mut tx); *pqres |= (memcmp((page_address(p_src) as *const u8).add(p_off as usize), (page_address(spare) as *const u8).add(s_off as usize), len) != 0) as i32 << SUM_CHECK_P; }
        if !q_src.is_null() { set_p(blocks, disks, core::ptr::null_mut()); set_q(blocks, disks, spare); *offsets.add((disks-1) as usize) = s_off; init_async_submit(submit, 0, None, core::ptr::null_mut(), core::ptr::null_mut(), scribble); tx = async_gen_syndrome(blocks, offsets, disks, len, submit); async_tx_quiesce(&mut tx); *pqres |= (memcmp((page_address(q_src) as *const u8).add(q_off as usize), (page_address(spare) as *const u8).add(s_off as usize), len) != 0) as i32 << SUM_CHECK_Q; }
        set_p(blocks, disks, p_src); *offsets.add((disks-2) as usize) = p_off; set_q(blocks, disks, q_src); *offsets.add((disks-1) as usize) = q_off; (*submit).cb_fn = cb_fn_orig; (*submit).cb_param = cb_param_orig; (*submit).flags = flags_orig; async_tx_sync_epilog(submit); tx = core::ptr::null_mut();
    }
    dmaengine_unmap_put(unmap); tx
}

unsafe fn async_pq_init() -> i32 { PQ_SCRIBBLE_PAGE = alloc_page(GFP_KERNEL); if !PQ_SCRIBBLE_PAGE.is_null() { 0 } else { -12 } }
unsafe fn async_pq_exit() { __free_page(PQ_SCRIBBLE_PAGE); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
