// SPDX-License-Identifier: GPL-2.0-only
/*
 * Core driver for the High Speed UART DMA
 *
 * Copyright (C) 2015 Intel Corporation
 * Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
 *
 * Partially based on the bits found in drivers/tty/serial/mfd.c.
 */

/*
 * DMA channel allocation:
 * 1. Even number chans are used for DMA Read (UART TX), odd chans for DMA
 *    Write (UART RX).
 * 2. 0/1 channel are assigned to port 0, 2/3 chan to port 1, 4/5 chan to
 *    port 3, and so on.
 */

// Dependencies supplied by the Linux kernel and hsu.h are intentionally external.

const HSU_DMA_BUSWIDTHS: u32 = (1 << DMA_SLAVE_BUSWIDTH_UNDEFINED)
    | (1 << DMA_SLAVE_BUSWIDTH_1_BYTE)
    | (1 << DMA_SLAVE_BUSWIDTH_2_BYTES)
    | (1 << DMA_SLAVE_BUSWIDTH_3_BYTES)
    | (1 << DMA_SLAVE_BUSWIDTH_4_BYTES)
    | (1 << DMA_SLAVE_BUSWIDTH_8_BYTES)
    | (1 << DMA_SLAVE_BUSWIDTH_16_BYTES);

#[inline]
unsafe fn hsu_chan_disable(hsuc: *mut hsu_dma_chan) {
    hsu_chan_writel(hsuc, HSU_CH_CR, 0);
}

#[inline]
unsafe fn hsu_chan_enable(hsuc: *mut hsu_dma_chan) {
    let mut cr: u32 = HSU_CH_CR_CHA;

    if (*hsuc).direction == DMA_MEM_TO_DEV {
        cr &= !HSU_CH_CR_CHD;
    } else if (*hsuc).direction == DMA_DEV_TO_MEM {
        cr |= HSU_CH_CR_CHD;
    }

    hsu_chan_writel(hsuc, HSU_CH_CR, cr);
}

unsafe fn hsu_dma_chan_start(hsuc: *mut hsu_dma_chan) {
    let config = &mut (*hsuc).config;
    let desc = (*hsuc).desc;
    let (mut bsr, mut mtsr): (u32, u32) = (0, 0); // to shut the compiler up
    let mut dcr: u32 = HSU_CH_DCR_CHSOE | HSU_CH_DCR_CHEI;
    let mut i: u32 = 0;
    let count: u32;

    if (*hsuc).direction == DMA_MEM_TO_DEV {
        bsr = config.dst_maxburst;
        mtsr = config.dst_addr_width;
    } else if (*hsuc).direction == DMA_DEV_TO_MEM {
        bsr = config.src_maxburst;
        mtsr = config.src_addr_width;
    }

    hsu_chan_disable(hsuc);
    hsu_chan_writel(hsuc, HSU_CH_DCR, 0);
    hsu_chan_writel(hsuc, HSU_CH_BSR, bsr);
    hsu_chan_writel(hsuc, HSU_CH_MTSR, mtsr);

    // Set descriptors
    count = (*desc).nents - (*desc).active;
    while i < count && i < HSU_DMA_CHAN_NR_DESC {
        hsu_chan_writel(hsuc, HSU_CH_DxSAR(i), (*desc).sg[i as usize].addr);
        hsu_chan_writel(hsuc, HSU_CH_DxTSR(i), (*desc).sg[i as usize].len);

        // Prepare value for DCR
        dcr |= HSU_CH_DCR_DESCA(i);
        dcr |= HSU_CH_DCR_CHTOI(i); // timeout bit, see HSU Errata 1
        (*desc).active += 1;
        i += 1;
    }
    // Only for the last descriptor in the chain
    dcr |= HSU_CH_DCR_CHSOD(count - 1);
    dcr |= HSU_CH_DCR_CHDI(count - 1);
    hsu_chan_writel(hsuc, HSU_CH_DCR, dcr);
    hsu_chan_enable(hsuc);
}

unsafe fn hsu_dma_stop_channel(hsuc: *mut hsu_dma_chan) {
    hsu_chan_disable(hsuc);
    hsu_chan_writel(hsuc, HSU_CH_DCR, 0);
}

unsafe fn hsu_dma_start_channel(hsuc: *mut hsu_dma_chan) {
    hsu_dma_chan_start(hsuc);
}

unsafe fn hsu_dma_start_transfer(hsuc: *mut hsu_dma_chan) {
    let vdesc = vchan_next_desc(&mut (*hsuc).vchan);
    if vdesc.is_null() {
        (*hsuc).desc = core::ptr::null_mut();
        return;
    }
    list_del(&mut (*vdesc).node);
    (*hsuc).desc = to_hsu_dma_desc(vdesc);
    hsu_dma_start_channel(hsuc);
}

pub unsafe fn hsu_dma_get_status(chip: *mut hsu_dma_chip, nr: u16, status: *mut u32) -> i32 {
    if nr >= (*(*chip).hsu).nr_channels { return -EINVAL; }
    let hsuc = (*(*chip).hsu).chan.add(nr as usize);
    let mut flags: usize = 0;
    spin_lock_irqsave(&mut (*hsuc).vchan.lock, &mut flags);
    let mut sr = hsu_chan_readl(hsuc, HSU_CH_SR);
    spin_unlock_irqrestore(&mut (*hsuc).vchan.lock, flags);
    sr &= !(HSU_CH_SR_DESCE_ANY | HSU_CH_SR_CDESC_ANY);
    if sr == 0 { return -EIO; }
    if sr & HSU_CH_SR_DESCTO_ANY != 0 { udelay(2); }
    sr &= !HSU_CH_SR_DESCTO_ANY;
    *status = sr;
    if sr != 0 { 0 } else { 1 }
}

pub unsafe fn hsu_dma_do_irq(chip: *mut hsu_dma_chip, nr: u16, status: u32) -> i32 {
    if nr >= (*(*chip).hsu).nr_channels { return 0; }
    let hsuc = (*(*chip).hsu).chan.add(nr as usize);
    let stat = this_cpu_ptr((*hsuc).vchan.chan.local);
    let mut flags: usize = 0;
    spin_lock_irqsave(&mut (*hsuc).vchan.lock, &mut flags);
    let desc = (*hsuc).desc;
    if !desc.is_null() {
        if status & HSU_CH_SR_CHE != 0 { (*desc).status = DMA_ERROR; }
        else if (*desc).active < (*desc).nents { hsu_dma_start_channel(hsuc); }
        else {
            vchan_cookie_complete(&mut (*desc).vdesc);
            (*desc).status = DMA_COMPLETE;
            (*stat).bytes_transferred += (*desc).length;
            hsu_dma_start_transfer(hsuc);
        }
    }
    spin_unlock_irqrestore(&mut (*hsuc).vchan.lock, flags);
    1
}

unsafe fn hsu_dma_alloc_desc(nents: u32) -> *mut hsu_dma_desc {
    let desc = kzalloc::<hsu_dma_desc>(1, GFP_NOWAIT);
    if desc.is_null() { return core::ptr::null_mut(); }
    (*desc).sg = kzalloc::<hsu_dma_sg>(nents as usize, GFP_NOWAIT);
    if (*desc).sg.is_null() { kfree(desc); return core::ptr::null_mut(); }
    desc
}

unsafe fn hsu_dma_desc_free(vdesc: *mut virt_dma_desc) {
    let desc = to_hsu_dma_desc(vdesc);
    kfree((*desc).sg);
    kfree(desc);
}

unsafe fn hsu_dma_prep_slave_sg(chan: *mut dma_chan, sgl: *mut scatterlist,
    sg_len: u32, direction: dma_transfer_direction, flags: usize, _context: *mut core::ffi::c_void) -> *mut dma_async_tx_descriptor {
    let hsuc = to_hsu_dma_chan(chan);
    let desc = hsu_dma_alloc_desc(sg_len);
    if desc.is_null() { return core::ptr::null_mut(); }
    let mut sg = sgl;
    for i in 0..sg_len as usize {
        (*desc).sg[i].addr = sg_dma_address(sg);
        (*desc).sg[i].len = sg_dma_len(sg);
        (*desc).length += sg_dma_len(sg);
        sg = sg_next(sg);
    }
    (*desc).nents = sg_len;
    (*desc).direction = direction;
    (*desc).status = DMA_IN_PROGRESS;
    vchan_tx_prep(&mut (*hsuc).vchan, &mut (*desc).vdesc, flags)
}

unsafe fn hsu_dma_issue_pending(chan: *mut dma_chan) {
    let hsuc = to_hsu_dma_chan(chan);
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*hsuc).vchan.lock, &mut flags);
    if vchan_issue_pending(&mut (*hsuc).vchan) && (*hsuc).desc.is_null() { hsu_dma_start_transfer(hsuc); }
    spin_unlock_irqrestore(&mut (*hsuc).vchan.lock, flags);
}

unsafe fn hsu_dma_active_desc_size(hsuc: *mut hsu_dma_chan) -> usize {
    let desc = (*hsuc).desc;
    let mut bytes = 0usize;
    for i in (*desc).active as usize..(*desc).nents as usize { bytes += (*desc).sg[i].len as usize; }
    let mut i = HSU_DMA_CHAN_NR_DESC - 1;
    loop { bytes += hsu_chan_readl(hsuc, HSU_CH_DxTSR(i)) as usize; if i == 0 { break; } i -= 1; }
    bytes
}

unsafe fn hsu_dma_tx_status(chan: *mut dma_chan, cookie: dma_cookie_t, state: *mut dma_tx_state) -> dma_status {
    let hsuc = to_hsu_dma_chan(chan);
    let mut status = dma_cookie_status(chan, cookie, state);
    if status == DMA_COMPLETE { return status; }
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*hsuc).vchan.lock, &mut flags);
    let vdesc = vchan_find_desc(&mut (*hsuc).vchan, cookie);
    if !(*hsuc).desc.is_null() && cookie == (*(*hsuc).desc).vdesc.tx.cookie {
        dma_set_residue(state, hsu_dma_active_desc_size(hsuc));
        status = (*(*hsuc).desc).status;
    } else if !vdesc.is_null() { dma_set_residue(state, (*to_hsu_dma_desc(vdesc)).length as usize); }
    spin_unlock_irqrestore(&mut (*hsuc).vchan.lock, flags);
    status
}

unsafe fn hsu_dma_slave_config(chan: *mut dma_chan, config: *mut dma_slave_config) -> i32 {
    let hsuc = to_hsu_dma_chan(chan);
    core::ptr::copy_nonoverlapping(config, &mut (*hsuc).config, 1);
    0
}

unsafe fn hsu_dma_pause(chan: *mut dma_chan) -> i32 {
    let hsuc = to_hsu_dma_chan(chan); let mut flags = 0usize;
    spin_lock_irqsave(&mut (*hsuc).vchan.lock, &mut flags);
    if !(*hsuc).desc.is_null() && (*(*hsuc).desc).status == DMA_IN_PROGRESS { hsu_chan_disable(hsuc); (*(*hsuc).desc).status = DMA_PAUSED; }
    spin_unlock_irqrestore(&mut (*hsuc).vchan.lock, flags); 0
}

unsafe fn hsu_dma_resume(chan: *mut dma_chan) -> i32 {
    let hsuc = to_hsu_dma_chan(chan); let mut flags = 0usize;
    spin_lock_irqsave(&mut (*hsuc).vchan.lock, &mut flags);
    if !(*hsuc).desc.is_null() && (*(*hsuc).desc).status == DMA_PAUSED { (*(*hsuc).desc).status = DMA_IN_PROGRESS; hsu_chan_enable(hsuc); }
    spin_unlock_irqrestore(&mut (*hsuc).vchan.lock, flags); 0
}

unsafe fn hsu_dma_terminate_all(chan: *mut dma_chan) -> i32 {
    let hsuc = to_hsu_dma_chan(chan); let mut flags = 0usize; let mut head = LIST_HEAD_INIT;
    spin_lock_irqsave(&mut (*hsuc).vchan.lock, &mut flags);
    hsu_dma_stop_channel(hsuc);
    if !(*hsuc).desc.is_null() { hsu_dma_desc_free(&mut (*(*hsuc).desc).vdesc); (*hsuc).desc = core::ptr::null_mut(); }
    vchan_get_all_descriptors(&mut (*hsuc).vchan, &mut head);
    spin_unlock_irqrestore(&mut (*hsuc).vchan.lock, flags);
    vchan_dma_desc_free_list(&mut (*hsuc).vchan, &mut head); 0
}

unsafe fn hsu_dma_free_chan_resources(chan: *mut dma_chan) { vchan_free_chan_resources(to_virt_chan(chan)); }
unsafe fn hsu_dma_synchronize(chan: *mut dma_chan) { vchan_synchronize(&mut (*to_hsu_dma_chan(chan)).vchan); }

pub unsafe fn hsu_dma_probe(chip: *mut hsu_dma_chip) -> i32 {
    let hsu = devm_kzalloc((*chip).dev, core::mem::size_of::<hsu_dma>(), GFP_KERNEL);
    if hsu.is_null() { return -ENOMEM; }
    (*chip).hsu = hsu;
    (*hsu).nr_channels = ((*chip).length - (*chip).offset) / HSU_DMA_CHAN_LENGTH;
    (*hsu).chan = devm_kcalloc((*chip).dev, (*hsu).nr_channels as usize, core::mem::size_of::<hsu_dma_chan>(), GFP_KERNEL);
    if (*hsu).chan.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD(&mut (*hsu).dma.channels);
    for i in 0..(*hsu).nr_channels as usize {
        let hsuc = (*hsu).chan.add(i);
        (*hsuc).vchan.desc_free = Some(hsu_dma_desc_free);
        vchan_init(&mut (*hsuc).vchan, &mut (*hsu).dma);
        (*hsuc).direction = if i & 1 != 0 { DMA_DEV_TO_MEM } else { DMA_MEM_TO_DEV };
        (*hsuc).reg = ((*chip).regs as *mut u8).add((*chip).offset as usize + i * HSU_DMA_CHAN_LENGTH as usize) as *mut core::ffi::c_void;
    }
    dma_cap_set(DMA_SLAVE, &mut (*hsu).dma.cap_mask); dma_cap_set(DMA_PRIVATE, &mut (*hsu).dma.cap_mask);
    (*hsu).dma.device_free_chan_resources = Some(hsu_dma_free_chan_resources);
    (*hsu).dma.device_prep_slave_sg = Some(hsu_dma_prep_slave_sg);
    (*hsu).dma.device_issue_pending = Some(hsu_dma_issue_pending); (*hsu).dma.device_tx_status = Some(hsu_dma_tx_status);
    (*hsu).dma.device_config = Some(hsu_dma_slave_config); (*hsu).dma.device_pause = Some(hsu_dma_pause); (*hsu).dma.device_resume = Some(hsu_dma_resume);
    (*hsu).dma.device_terminate_all = Some(hsu_dma_terminate_all); (*hsu).dma.device_synchronize = Some(hsu_dma_synchronize);
    (*hsu).dma.src_addr_widths = HSU_DMA_BUSWIDTHS; (*hsu).dma.dst_addr_widths = HSU_DMA_BUSWIDTHS;
    (*hsu).dma.directions = (1 << DMA_DEV_TO_MEM) | (1 << DMA_MEM_TO_DEV); (*hsu).dma.residue_granularity = DMA_RESIDUE_GRANULARITY_BURST; (*hsu).dma.dev = (*chip).dev;
    dma_set_max_seg_size((*hsu).dma.dev, HSU_CH_DxTSR_MASK);
    let ret = dma_async_device_register(&mut (*hsu).dma); if ret != 0 { return ret; }
    dev_info((*chip).dev, "Found HSU DMA, %d channels\n", (*hsu).nr_channels); 0
}

pub unsafe fn hsu_dma_remove(chip: *mut hsu_dma_chip) -> i32 {
    let hsu = (*chip).hsu; dma_async_device_unregister(&mut (*hsu).dma);
    for i in 0..(*hsu).nr_channels as usize { tasklet_kill(&mut (*(*hsu).chan.add(i)).vchan.task); }
    0
}

// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("High Speed UART DMA core driver");
// MODULE_AUTHOR("Andy Shevchenko <andriy.shevchenko@linux.intel.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
