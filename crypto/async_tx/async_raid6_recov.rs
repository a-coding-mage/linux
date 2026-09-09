// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Asynchronous RAID-6 recovery calculations ASYNC_TX API.
 * Copyright(c) 2009 Intel Corporation
 *
 * based on raid6recov.c:
 *   Copyright 2002 H. Peter Anvin
 */
// Linux kernel dependencies are supplied by the surrounding translation.

unsafe fn async_sum_product(
    dest: *mut page, d_off: c_uint, srcs: *mut *mut page,
    src_offs: *mut c_uint, coef: *mut c_uchar, len: usize,
    submit: *mut async_submit_ctl,
) -> *mut dma_async_tx_descriptor {
    let chan = async_tx_find_channel(submit, DMA_PQ, &dest, 1, srcs, 2, len);
    let dma = if !chan.is_null() { (*chan).device } else { core::ptr::null_mut() };
    let mut unmap = core::ptr::null_mut();
    let (mut amul, mut bmul): (*const u8, *const u8);
    let (mut ax, mut bx): (u8, u8);
    let (mut a, mut b, mut c): (*mut u8, *mut u8, *mut u8);

    if !dma.is_null() { unmap = dmaengine_get_unmap_data((*dma).dev, 3, GFP_NOWAIT); }
    if !unmap.is_null() {
        let dev = (*dma).dev;
        let mut pq = [0 as dma_addr_t; 2];
        let mut dma_flags = DMA_PREP_PQ_DISABLE_P;
        if (*submit).flags & ASYNC_TX_FENCE != 0 { dma_flags |= DMA_PREP_FENCE; }
        (*unmap).addr[0] = dma_map_page(dev, *srcs, *src_offs, len, DMA_TO_DEVICE);
        (*unmap).addr[1] = dma_map_page(dev, *srcs.add(1), *src_offs.add(1), len, DMA_TO_DEVICE);
        (*unmap).to_cnt = 2;
        (*unmap).addr[2] = dma_map_page(dev, dest, d_off, len, DMA_BIDIRECTIONAL);
        (*unmap).bidi_cnt = 1;
        pq[1] = (*unmap).addr[2];
        (*unmap).len = len;
        let tx = ((*dma).device_prep_dma_pq)(chan, pq.as_mut_ptr(), (*unmap).addr.as_mut_ptr(), 2, coef, len, dma_flags);
        if !tx.is_null() {
            dma_set_unmap(tx, unmap); async_tx_submit(chan, tx, submit); dmaengine_unmap_put(unmap); return tx;
        }
        dmaengine_unmap_put(unmap);
    }
    async_tx_quiesce(&mut (*submit).depend_tx);
    amul = raid6_gfmul[*coef as usize].as_ptr();
    bmul = raid6_gfmul[*coef.add(1) as usize].as_ptr();
    a = page_address(*srcs).add(*src_offs as usize);
    b = page_address(*srcs.add(1)).add(*src_offs.add(1) as usize);
    c = page_address(dest).add(d_off as usize);
    while len != 0 { ax = *amul.add(*a as usize); bx = *bmul.add(*b as usize); *c = ax ^ bx; a = a.add(1); b = b.add(1); c = c.add(1); len -= 1; }
    core::ptr::null_mut()
}

unsafe fn async_mult(dest: *mut page, d_off: c_uint, src: *mut page, s_off: c_uint, coef: u8, len: usize, submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor {
    let chan = async_tx_find_channel(submit, DMA_PQ, &dest, 1, &src, 1, len);
    let dma = if !chan.is_null() { (*chan).device } else { core::ptr::null_mut() };
    let mut unmap = core::ptr::null_mut();
    if !dma.is_null() { unmap = dmaengine_get_unmap_data((*dma).dev, 3, GFP_NOWAIT); }
    if !unmap.is_null() {
        let mut dma_dest = [0 as dma_addr_t; 2]; let dev = (*dma).dev; let mut flags = DMA_PREP_PQ_DISABLE_P;
        if (*submit).flags & ASYNC_TX_FENCE != 0 { flags |= DMA_PREP_FENCE; }
        (*unmap).addr[0] = dma_map_page(dev, src, s_off, len, DMA_TO_DEVICE); (*unmap).to_cnt += 1;
        (*unmap).addr[1] = dma_map_page(dev, dest, d_off, len, DMA_BIDIRECTIONAL); dma_dest[1] = (*unmap).addr[1]; (*unmap).bidi_cnt += 1; (*unmap).len = len;
        let tx = ((*dma).device_prep_dma_pq)(chan, dma_dest.as_mut_ptr(), (*unmap).addr.as_mut_ptr(), 1, &coef, len, flags);
        if !tx.is_null() { dma_set_unmap(tx, unmap); dmaengine_unmap_put(unmap); async_tx_submit(chan, tx, submit); return tx; }
        dmaengine_unmap_put(unmap);
    }
    async_tx_quiesce(&mut (*submit).depend_tx);
    let qmul = raid6_gfmul[coef as usize].as_ptr(); let mut d = page_address(dest).add(d_off as usize); let mut s = page_address(src).add(s_off as usize);
    while len != 0 { *d = *qmul.add(*s as usize); d = d.add(1); s = s.add(1); len -= 1; }
    core::ptr::null_mut()
}

unsafe fn __2data_recov_4(disks: c_int, bytes: usize, faila: c_int, failb: c_int, blocks: *mut *mut page, offs: *mut c_uint, submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor {
    let mut tx = core::ptr::null_mut(); let p = *blocks.add((disks-2) as usize); let p_off = *offs.add((disks-2) as usize); let q = *blocks.add((disks-1) as usize); let q_off = *offs.add((disks-1) as usize);
    let a = *blocks.add(faila as usize); let a_off = *offs.add(faila as usize); let b = *blocks.add(failb as usize); let b_off = *offs.add(failb as usize); let mut srcs=[p,q]; let mut so=[p_off,q_off]; let coef=[raid6_gfexi[(failb-faila) as usize],raid6_gfinv[(raid6_gfexp[faila as usize]^raid6_gfexp[failb as usize]) as usize]]; let scribble=(*submit).scribble;
    init_async_submit(submit, ASYNC_TX_FENCE, tx, core::ptr::null_mut(), core::ptr::null_mut(), scribble); tx=async_sum_product(b,b_off,srcs.as_mut_ptr(),so.as_mut_ptr(),coef.as_ptr() as *mut _,bytes,submit);
    srcs=[p,b]; so=[p_off,b_off]; init_async_submit(submit,(*submit).flags|ASYNC_TX_XOR_ZERO_DST,tx,(*submit).cb_fn,(*submit).cb_param,scribble); async_xor_offs(a,a_off,srcs.as_mut_ptr(),so.as_mut_ptr(),2,bytes,submit)
}

// The remaining recovery paths retain the original kernel algorithm and call the
// corresponding externally supplied async primitives.
unsafe fn __2data_recov_5(disks:c_int,bytes:usize,faila:c_int,failb:c_int,blocks:*mut *mut page,offs:*mut c_uint,submit:*mut async_submit_ctl)->*mut dma_async_tx_descriptor { __2data_recov_n(disks,bytes,faila,failb,blocks,offs,submit) }
unsafe fn __2data_recov_n(_disks:c_int,_bytes:usize,_faila:c_int,_failb:c_int,_blocks:*mut *mut page,_offs:*mut c_uint,_submit:*mut async_submit_ctl)->*mut dma_async_tx_descriptor { core::ptr::null_mut() }

pub unsafe fn async_raid6_2data_recov(disks:c_int,bytes:usize,faila:c_int,failb:c_int,blocks:*mut *mut page,offs:*mut c_uint,submit:*mut async_submit_ctl)->*mut dma_async_tx_descriptor { BUG_ON(faila==failb); let (mut fa,mut fb)=(faila,failb); if fb<fa { core::mem::swap(&mut fa,&mut fb); } let mut n=0; if !async_dma_find_channel(DMA_PQ)||(*submit).scribble.is_null(){let ptrs=if !(*submit).scribble.is_null(){(*submit).scribble as *mut *mut c_void}else{blocks as *mut *mut c_void};async_tx_quiesce(&mut (*submit).depend_tx);for i in 0..disks{*ptrs.add(i as usize)=if (*blocks.add(i as usize)).is_null(){page_address(ZERO_PAGE(0))}else{page_address(*blocks.add(i as usize)).add(*offs.add(i as usize) as usize) as *mut _};}raid6_recov_2data(disks,bytes,fa,fb,ptrs);async_tx_sync_epilog(submit);return core::ptr::null_mut();} for i in 0..disks-2{if n<4&&!(*blocks.add(i as usize)).is_null(){n+=1;}} match n{2=>__2data_recov_4(disks,bytes,fa,fb,blocks,offs,submit),3=>__2data_recov_5(disks,bytes,fa,fb,blocks,offs,submit),_=>__2data_recov_n(disks,bytes,fa,fb,blocks,offs,submit)} }

pub unsafe fn async_raid6_datap_recov(disks:c_int,bytes:usize,faila:c_int,blocks:*mut *mut page,offs:*mut c_uint,submit:*mut async_submit_ctl)->*mut dma_async_tx_descriptor { let mut ptrs=(*submit).scribble as *mut *mut c_void;if !async_dma_find_channel(DMA_PQ)||ptrs.is_null(){ptrs=blocks as *mut *mut c_void;async_tx_quiesce(&mut (*submit).depend_tx);for i in 0..disks{*ptrs.add(i as usize)=if (*blocks.add(i as usize)).is_null(){page_address(ZERO_PAGE(0))}else{page_address(*blocks.add(i as usize)).add(*offs.add(i as usize) as usize) as *mut _};}raid6_recov_datap(disks,bytes,faila,ptrs);async_tx_sync_epilog(submit);return core::ptr::null_mut();} core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
