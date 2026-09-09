// SPDX-License-Identifier: GPL-2.0-only
/*
 * Core driver for the Intel integrated DMA 64-bit
 *
 * Copyright (C) 2015 Intel Corporation
 * Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
 */

// C dependencies: linux/bitops.h, delay.h, dmaengine.h, dma-mapping.h,
// dmapool.h, init.h, module.h, platform_device.h, slab.h, dma/idma64.h,
// and the local idma64.h provide the external types, constants, and helpers.

const IDMA64_NR_CHAN: usize = 2;

unsafe fn chan2dev(chan: *mut dma_chan) -> *mut device {
    &mut (*(*chan).dev).device
}

unsafe fn idma64_off(idma64: *mut idma64) {
    let mut count: u16 = 100;
    dma_writel(idma64, CFG, 0);
    channel_clear_bit(idma64, MASK(XFER), (*idma64).all_chan_mask);
    channel_clear_bit(idma64, MASK(BLOCK), (*idma64).all_chan_mask);
    channel_clear_bit(idma64, MASK(SRC_TRAN), (*idma64).all_chan_mask);
    channel_clear_bit(idma64, MASK(DST_TRAN), (*idma64).all_chan_mask);
    channel_clear_bit(idma64, MASK(ERROR), (*idma64).all_chan_mask);
    loop {
        cpu_relax();
        if dma_readl(idma64, CFG) & IDMA64_CFG_DMA_EN == 0 || { count -= 1; count == 0 } { break; }
    }
}

unsafe fn idma64_on(idma64: *mut idma64) { dma_writel(idma64, CFG, IDMA64_CFG_DMA_EN); }

unsafe fn idma64_chan_init(idma64: *mut idma64, idma64c: *mut idma64_chan) {
    let cfghi: u32 = IDMA64C_CFGH_SRC_PER(1) | IDMA64C_CFGH_DST_PER(0);
    let cfglo: u32 = IDMA64C_CFGL_DST_BURST_ALIGN | IDMA64C_CFGL_SRC_BURST_ALIGN;
    channel_writel(idma64c, CFG_LO, cfglo);
    channel_writel(idma64c, CFG_HI, cfghi);
    channel_set_bit(idma64, MASK(XFER), (*idma64c).mask);
    channel_set_bit(idma64, MASK(ERROR), (*idma64c).mask);
    idma64_on(idma64);
}

unsafe fn idma64_chan_stop(idma64: *mut idma64, idma64c: *mut idma64_chan) { channel_clear_bit(idma64, CH_EN, (*idma64c).mask); }

unsafe fn idma64_chan_start(idma64: *mut idma64, idma64c: *mut idma64_chan) {
    let desc = (*idma64c).desc;
    let hw = &(*desc).hw[0];
    channel_writeq(idma64c, SAR, 0); channel_writeq(idma64c, DAR, 0);
    channel_writel(idma64c, CTL_HI, IDMA64C_CTLH_BLOCK_TS(!0usize as u32));
    channel_writel(idma64c, CTL_LO, IDMA64C_CTLL_LLP_S_EN | IDMA64C_CTLL_LLP_D_EN);
    channel_writeq(idma64c, LLP, hw.llp);
    channel_set_bit(idma64, CH_EN, (*idma64c).mask);
}

unsafe fn idma64_stop_transfer(idma64c: *mut idma64_chan) {
    let idma64 = to_idma64((*idma64c).vchan.chan.device);
    idma64_chan_stop(idma64, idma64c);
}

unsafe fn idma64_start_transfer(idma64c: *mut idma64_chan) {
    let idma64 = to_idma64((*idma64c).vchan.chan.device);
    let vdesc = vchan_next_desc(&mut (*idma64c).vchan);
    if vdesc.is_null() { (*idma64c).desc = core::ptr::null_mut(); return; }
    list_del(&mut (*vdesc).node);
    (*idma64c).desc = to_idma64_desc(vdesc);
    idma64_chan_init(idma64, idma64c);
    idma64_chan_start(idma64, idma64c);
}

unsafe fn idma64_chan_irq(idma64: *mut idma64, c: u16, status_err: u32, status_xfer: u32) {
    let idma64c = &mut (*idma64).chan[c as usize];
    let stat = this_cpu_ptr(idma64c.vchan.chan.local);
    spin_lock(&mut idma64c.vchan.lock);
    let desc = idma64c.desc;
    if !desc.is_null() {
        if status_err & (1 << c) != 0 { dma_writel(idma64, CLEAR(ERROR), idma64c.mask); (*desc).status = DMA_ERROR; }
        else if status_xfer & (1 << c) != 0 { dma_writel(idma64, CLEAR(XFER), idma64c.mask); (*desc).status = DMA_COMPLETE; vchan_cookie_complete(&mut (*desc).vdesc); (*stat).bytes_transferred += (*desc).length; idma64_start_transfer(idma64c); }
        if idma64c.desc.is_null() || (*desc).status == DMA_ERROR { idma64_stop_transfer(idma64c); }
    }
    spin_unlock(&mut idma64c.vchan.lock);
}

unsafe fn idma64_irq(_irq: i32, dev: *mut core::ffi::c_void) -> irqreturn_t {
    let idma64 = dev as *mut idma64;
    let status = dma_readl(idma64, STATUS_INT);
    if status == GENMASK(31, 0) || status == 0 { return IRQ_NONE; }
    dev_vdbg((*idma64).dma.dev, "%s: status=%#x\n", __func__, status);
    let status_xfer = dma_readl(idma64, RAW(XFER));
    let status_err = dma_readl(idma64, RAW(ERROR));
    for i in 0..(*idma64).dma.chancnt { idma64_chan_irq(idma64, i as u16, status_err, status_xfer); }
    IRQ_HANDLED
}

unsafe fn idma64_alloc_desc(ndesc: usize) -> *mut idma64_desc {
    let desc = kzalloc_obj::<idma64_desc>(); if desc.is_null() { return core::ptr::null_mut(); }
    (*desc).hw = kzalloc_objs::<idma64_hw_desc>(ndesc);
    if (*desc).hw.is_null() { kfree(desc); return core::ptr::null_mut(); } desc
}

unsafe fn idma64_desc_free(idma64c: *mut idma64_chan, desc: *mut idma64_desc) {
    let mut i = (*desc).ndesc;
    while i != 0 { i -= 1; let hw = &mut (*desc).hw[i]; dma_pool_free((*idma64c).pool, hw.lli, hw.llp); }
    kfree((*desc).hw); kfree(desc);
}

unsafe fn idma64_vdesc_free(vdesc: *mut virt_dma_desc) { let c = to_idma64_chan((*vdesc).tx.chan); idma64_desc_free(c, to_idma64_desc(vdesc)); }

unsafe fn idma64_hw_desc_fill(hw: *mut idma64_hw_desc, config: *mut dma_slave_config, direction: dma_transfer_direction, llp: u64) {
    let lli = (*hw).lli; let mut sar; let mut dar; let ctlhi = IDMA64C_CTLH_BLOCK_TS((*hw).len); let mut ctllo = IDMA64C_CTLL_LLP_S_EN | IDMA64C_CTLL_LLP_D_EN; let src_width; let dst_width;
    if direction == DMA_MEM_TO_DEV { sar = (*hw).phys; dar = (*config).dst_addr; ctllo |= IDMA64C_CTLL_DST_FIX | IDMA64C_CTLL_SRC_INC | IDMA64C_CTLL_FC_M2P; src_width = __ffs(sar | (*hw).len | 4); dst_width = __ffs((*config).dst_addr_width); }
    else { sar = (*config).src_addr; dar = (*hw).phys; ctllo |= IDMA64C_CTLL_DST_INC | IDMA64C_CTLL_SRC_FIX | IDMA64C_CTLL_FC_P2M; src_width = __ffs((*config).src_addr_width); dst_width = __ffs(dar | (*hw).len | 4); }
    (*lli).sar = sar; (*lli).dar = dar; (*lli).ctlhi = ctlhi; (*lli).ctllo = ctllo | IDMA64C_CTLL_SRC_MSIZE((*config).src_maxburst) | IDMA64C_CTLL_DST_MSIZE((*config).dst_maxburst) | IDMA64C_CTLL_DST_WIDTH(dst_width) | IDMA64C_CTLL_SRC_WIDTH(src_width); (*lli).llp = llp;
}

unsafe fn idma64_desc_fill(c: *mut idma64_chan, d: *mut idma64_desc) { let cfg=&mut (*c).config; let mut i=(*d).ndesc; let mut llp=0; while i!=0 { i-=1; let h=&mut (*d).hw[i]; idma64_hw_desc_fill(h,cfg,(*d).direction,llp); llp=h.llp; (*d).length+=h.len; } let lli=(*d).hw[(*d).ndesc-1].lli; (*lli).ctllo|=IDMA64C_CTLL_INT_EN; (*lli).ctllo&=!(IDMA64C_CTLL_LLP_S_EN|IDMA64C_CTLL_LLP_D_EN); }
unsafe fn idma64_prep_slave_sg(chan:*mut dma_chan, sgl:*mut scatterlist, n:usize, dir:dma_transfer_direction, flags:usize, _ctx:*mut core::ffi::c_void)->*mut dma_async_tx_descriptor { let c=to_idma64_chan(chan); let d=idma64_alloc_desc(n); if d.is_null(){return core::ptr::null_mut();} for i in 0..n { let h=&mut (*d).hw[i]; h.lli=dma_pool_alloc((*c).pool,GFP_NOWAIT,&mut h.llp); if h.lli.is_null(){(*d).ndesc=i;idma64_desc_free(c,d);return core::ptr::null_mut();} let sg=for_each_sg(sgl,i); h.phys=sg_dma_address(sg);h.len=sg_dma_len(sg); } (*d).ndesc=n;(*d).direction=dir;(*d).status=DMA_IN_PROGRESS;idma64_desc_fill(c,d);vchan_tx_prep(&mut (*c).vchan,&mut (*d).vdesc,flags) }
unsafe fn idma64_issue_pending(chan:*mut dma_chan){let c=to_idma64_chan(chan);let mut f=0;spin_lock_irqsave(&mut (*c).vchan.lock,&mut f);if vchan_issue_pending(&mut (*c).vchan)&&(*c).desc.is_null(){idma64_start_transfer(c)}spin_unlock_irqrestore(&mut (*c).vchan.lock,f);}
unsafe fn convert_burst(x:&mut u32){*x=if *x!=0{__fls(*x)}else{0}}
unsafe fn idma64_slave_config(chan:*mut dma_chan, cfg:*mut dma_slave_config)->i32{let c=to_idma64_chan(chan);core::ptr::copy_nonoverlapping(cfg,&mut (*c).config,1);convert_burst(&mut (*c).config.src_maxburst);convert_burst(&mut (*c).config.dst_maxburst);0}
unsafe fn idma64_pause(_c:*mut dma_chan)->i32{0} unsafe fn idma64_resume(_c:*mut dma_chan)->i32{0} unsafe fn idma64_terminate_all(_c:*mut dma_chan)->i32{0} unsafe fn idma64_synchronize(_c:*mut dma_chan){}
unsafe fn idma64_alloc_chan_resources(_c:*mut dma_chan)->i32{0} unsafe fn idma64_free_chan_resources(_c:*mut dma_chan){}
unsafe fn idma64_probe(_c:*mut idma64_chip)->i32{0} unsafe fn idma64_remove(_c:*mut idma64_chip){}
unsafe fn idma64_platform_probe(_p:*mut platform_device)->i32{0} unsafe fn idma64_platform_remove(_p:*mut platform_device){}
unsafe fn idma64_pm_suspend(_d:*mut device)->i32{0} unsafe fn idma64_pm_resume(_d:*mut device)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
