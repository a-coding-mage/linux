// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SiFive FU540 Platform DMA driver
 * Copyright (C) 2019 SiFive
 *
 * Based partially on:
 * - drivers/dma/fsl-edma.c
 * - drivers/dma/dw-edma/
 * - drivers/dma/pxa-dma.c
 *
 * See the following sources for further documentation:
 * - Chapter 12 "Platform DMA Engine (PDMA)" of
 *   SiFive FU540-C000 v1.0
 *   https://static.dev.sifive.com/FU540-C000-v1.0.pdf
 */

// Dependencies supplied by the surrounding kernel translation.

const PDMA_QUIRK_NO_STRICT_ORDERING: u32 = BIT(0);

#[inline]
unsafe fn readq(addr: *mut core::ffi::c_void) -> u64 {
    readl(addr) as u64 | ((readl(addr.add(4)) as u64) << 32)
}

#[inline]
unsafe fn writeq(v: u64, addr: *mut core::ffi::c_void) {
    writel(lower_32_bits(v), addr);
    writel(upper_32_bits(v), addr.add(4));
}

#[inline]
unsafe fn to_sf_pdma_chan(dchan: *mut dma_chan) -> *mut sf_pdma_chan {
    container_of!(dchan, sf_pdma_chan, vchan.chan)
}

#[inline]
unsafe fn to_sf_pdma_desc(vd: *mut virt_dma_desc) -> *mut sf_pdma_desc {
    container_of!(vd, sf_pdma_desc, vdesc)
}

unsafe fn sf_pdma_alloc_desc(chan: *mut sf_pdma_chan) -> *mut sf_pdma_desc {
    let desc = kzalloc_obj::<sf_pdma_desc>(GFP_NOWAIT);
    if desc.is_null() { return core::ptr::null_mut(); }
    (*desc).chan = chan;
    desc
}

unsafe fn sf_pdma_fill_desc(desc: *mut sf_pdma_desc, dst: u64, src: u64, size: u64) {
    (*desc).xfer_type = (*(*desc).chan).pdma.as_ref().unwrap().transfer_type;
    (*desc).xfer_size = size;
    (*desc).dst_addr = dst;
    (*desc).src_addr = src;
}

unsafe fn sf_pdma_disclaim_chan(chan: *mut sf_pdma_chan) {
    writel(PDMA_CLEAR_CTRL, (*chan).regs.ctrl);
}

unsafe fn sf_pdma_prep_dma_memcpy(dchan: *mut dma_chan, dest: dma_addr_t, src: dma_addr_t,
                                  len: usize, flags: c_ulong) -> *mut dma_async_tx_descriptor {
    let chan = to_sf_pdma_chan(dchan);
    if !chan.is_null() && (len == 0 || dest == 0 || src == 0) {
        dev_err((*(*chan).pdma).dma_dev.dev, "Please check dma len, dest, src!\n");
        return core::ptr::null_mut();
    }
    let desc = sf_pdma_alloc_desc(chan);
    if desc.is_null() { return core::ptr::null_mut(); }
    (*desc).dirn = DMA_MEM_TO_MEM;
    (*desc).async_tx = vchan_tx_prep(&mut (*chan).vchan, &mut (*desc).vdesc, flags);
    let mut iflags = 0;
    spin_lock_irqsave(&mut (*chan).vchan.lock, &mut iflags);
    sf_pdma_fill_desc(desc, dest, src, len as u64);
    spin_unlock_irqrestore(&mut (*chan).vchan.lock, iflags);
    (*desc).async_tx
}

unsafe fn sf_pdma_slave_config(dchan: *mut dma_chan, cfg: *const dma_slave_config) -> i32 {
    let chan = to_sf_pdma_chan(dchan);
    memcpy(&mut (*chan).cfg as *mut _, cfg, core::mem::size_of::<dma_slave_config>());
    0
}

unsafe fn sf_pdma_alloc_chan_resources(dchan: *mut dma_chan) -> i32 {
    let chan = to_sf_pdma_chan(dchan);
    dma_cookie_init(dchan);
    writel(PDMA_CLAIM_MASK, (*chan).regs.ctrl);
    0
}

unsafe fn sf_pdma_disable_request(chan: *mut sf_pdma_chan) {
    writel(readl((*chan).regs.ctrl) & !PDMA_RUN_MASK, (*chan).regs.ctrl);
}

unsafe fn sf_pdma_free_chan_resources(dchan: *mut dma_chan) {
    let chan = to_sf_pdma_chan(dchan); let mut flags = 0; let mut head = LIST_HEAD();
    spin_lock_irqsave(&mut (*chan).vchan.lock, &mut flags);
    sf_pdma_disable_request(chan); kfree((*chan).desc); (*chan).desc = core::ptr::null_mut();
    vchan_get_all_descriptors(&mut (*chan).vchan, &mut head); sf_pdma_disclaim_chan(chan);
    spin_unlock_irqrestore(&mut (*chan).vchan.lock, flags);
    vchan_dma_desc_free_list(&mut (*chan).vchan, &mut head);
}

unsafe fn sf_pdma_desc_residue(chan: *mut sf_pdma_chan, cookie: dma_cookie_t) -> usize {
    let mut vd: *mut virt_dma_desc = core::ptr::null_mut(); let regs = &(*chan).regs;
    let mut flags = 0; let mut residue: u64 = 0; let mut tx: *mut dma_async_tx_descriptor = core::ptr::null_mut();
    spin_lock_irqsave(&mut (*chan).vchan.lock, &mut flags);
    list_for_each_entry!(vd, (*chan).vchan.desc_submitted, node, { if (*vd).tx.cookie == cookie { tx = &mut (*vd).tx; } });
    if tx.is_null() { goto_out!(); }
    if cookie == (*(*tx).chan).completed_cookie { goto_out!(); }
    if cookie == (*tx).cookie { residue = readq(regs.residue); }
    else { vd = vchan_find_desc(&mut (*chan).vchan, cookie); if vd.is_null() { goto_out!(); } residue = (*to_sf_pdma_desc(vd)).xfer_size; }
    spin_unlock_irqrestore(&mut (*chan).vchan.lock, flags); residue as usize
}

unsafe fn sf_pdma_tx_status(dchan: *mut dma_chan, cookie: dma_cookie_t, txstate: *mut dma_tx_state) -> dma_status {
    let chan = to_sf_pdma_chan(dchan); let status = dma_cookie_status(dchan, cookie, txstate);
    if !txstate.is_null() && status != DMA_ERROR { dma_set_residue(txstate, sf_pdma_desc_residue(chan, cookie)); }
    status
}

unsafe fn sf_pdma_terminate_all(dchan: *mut dma_chan) -> i32 {
    let chan = to_sf_pdma_chan(dchan); let mut flags = 0; let mut head = LIST_HEAD();
    spin_lock_irqsave(&mut (*chan).vchan.lock, &mut flags); sf_pdma_disable_request(chan);
    kfree((*chan).desc); (*chan).desc = core::ptr::null_mut(); (*chan).xfer_err = false;
    vchan_get_all_descriptors(&mut (*chan).vchan, &mut head); spin_unlock_irqrestore(&mut (*chan).vchan.lock, flags);
    vchan_dma_desc_free_list(&mut (*chan).vchan, &mut head); 0
}

unsafe fn sf_pdma_enable_request(chan: *mut sf_pdma_chan) {
    let v = PDMA_CLAIM_MASK | PDMA_ENABLE_DONE_INT_MASK | PDMA_ENABLE_ERR_INT_MASK | PDMA_RUN_MASK;
    writel(v, (*chan).regs.ctrl);
}

unsafe fn sf_pdma_get_first_pending_desc(chan: *mut sf_pdma_chan) -> *mut sf_pdma_desc {
    if list_empty!((*chan).vchan.desc_issued) { return core::ptr::null_mut(); }
    let vdesc = list_first_entry!((*chan).vchan.desc_issued, virt_dma_desc, node);
    container_of!(vdesc, sf_pdma_desc, vdesc)
}

unsafe fn sf_pdma_xfer_desc(chan: *mut sf_pdma_chan) {
    let desc = (*chan).desc; if desc.is_null() { dev_err((*(*chan).pdma).dma_dev.dev, "NULL desc.\n"); return; }
    writel((*desc).xfer_type, (*chan).regs.xfer_type); writeq((*desc).xfer_size, (*chan).regs.xfer_size);
    writeq((*desc).dst_addr, (*chan).regs.dst_addr); writeq((*desc).src_addr, (*chan).regs.src_addr);
    (*chan).desc = desc; (*chan).status = DMA_IN_PROGRESS; sf_pdma_enable_request(chan);
}

unsafe fn sf_pdma_issue_pending(dchan: *mut dma_chan) {
    let chan = to_sf_pdma_chan(dchan); let mut flags = 0; spin_lock_irqsave(&mut (*chan).vchan.lock, &mut flags);
    if (*chan).desc.is_null() && vchan_issue_pending(&mut (*chan).vchan) { (*chan).desc = sf_pdma_get_first_pending_desc(chan); sf_pdma_xfer_desc(chan); }
    spin_unlock_irqrestore(&mut (*chan).vchan.lock, flags);
}

unsafe fn sf_pdma_free_desc(vdesc: *mut virt_dma_desc) { kfree(to_sf_pdma_desc(vdesc)); }

unsafe fn sf_pdma_donebh_tasklet(t: *mut tasklet_struct) {
    let chan = from_tasklet!(t, sf_pdma_chan, done_tasklet); let mut flags = 0;
    spin_lock_irqsave(&mut (*chan).lock, &mut flags); if (*chan).xfer_err { (*chan).retries = MAX_RETRY; (*chan).status = DMA_COMPLETE; (*chan).xfer_err = false; } spin_unlock_irqrestore(&mut (*chan).lock, flags);
    spin_lock_irqsave(&mut (*chan).vchan.lock, &mut flags); list_del!((*chan).desc, vdesc.node); vchan_cookie_complete(&mut (*(*chan).desc).vdesc); (*chan).desc = sf_pdma_get_first_pending_desc(chan); if !(*chan).desc.is_null() { sf_pdma_xfer_desc(chan); } spin_unlock_irqrestore(&mut (*chan).vchan.lock, flags);
}

unsafe fn sf_pdma_errbh_tasklet(t: *mut tasklet_struct) {
    let chan = from_tasklet!(t, sf_pdma_chan, err_tasklet); let desc = (*chan).desc; let mut flags = 0;
    spin_lock_irqsave(&mut (*chan).lock, &mut flags);
    if (*chan).retries <= 0 { spin_unlock_irqrestore(&mut (*chan).lock, flags); dmaengine_desc_get_callback_invoke((*desc).async_tx, core::ptr::null_mut()); }
    else { (*chan).retries -= 1; (*chan).xfer_err = true; (*chan).status = DMA_ERROR; sf_pdma_enable_request(chan); spin_unlock_irqrestore(&mut (*chan).lock, flags); }
}

unsafe fn sf_pdma_done_isr(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let chan = dev_id as *mut sf_pdma_chan; let regs = &(*chan).regs; spin_lock(&mut (*chan).vchan.lock);
    writel(readl(regs.ctrl) & !PDMA_DONE_STATUS_MASK, regs.ctrl); let residue = readq(regs.residue);
    if residue == 0 { tasklet_hi_schedule(&mut (*chan).done_tasklet); } else { let desc = (*chan).desc; (*desc).src_addr += (*desc).xfer_size - residue; (*desc).dst_addr += (*desc).xfer_size - residue; (*desc).xfer_size = residue; sf_pdma_xfer_desc(chan); }
    spin_unlock(&mut (*chan).vchan.lock); IRQ_HANDLED
}

unsafe fn sf_pdma_err_isr(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let chan = dev_id as *mut sf_pdma_chan; let regs = &(*chan).regs; spin_lock(&mut (*chan).lock); writel(readl(regs.ctrl) & !PDMA_ERR_STATUS_MASK, regs.ctrl); spin_unlock(&mut (*chan).lock); tasklet_schedule(&mut (*chan).err_tasklet); IRQ_HANDLED
}

unsafe fn sf_pdma_irq_init(pdev: *mut platform_device, pdma: *mut sf_pdma) -> i32 {
    for i in 0..(*pdma).n_chans { let chan = &mut (*pdma).chans.add(i);
        let irq = platform_get_irq(pdev, i * 2); if irq < 0 { return -EINVAL; }
        let r = devm_request_irq(&mut (*pdev).dev, irq, sf_pdma_done_isr, 0, dev_name(&(*pdev).dev), chan as *mut _ as *mut _); if r != 0 { dev_err(&(*pdev).dev, "Fail to attach done ISR: %d\n", r); return -EINVAL; } (*chan).txirq = irq;
        let irq = platform_get_irq(pdev, i * 2 + 1); if irq < 0 { return -EINVAL; }
        let r = devm_request_irq(&mut (*pdev).dev, irq, sf_pdma_err_isr, 0, dev_name(&(*pdev).dev), chan as *mut _ as *mut _); if r != 0 { dev_err(&(*pdev).dev, "Fail to attach err ISR: %d\n", r); return -EINVAL; } (*chan).errirq = irq;
    } 0
}

unsafe fn sf_pdma_setup_chans(pdma: *mut sf_pdma) {
    INIT_LIST_HEAD!(&mut (*pdma).dma_dev.channels);
    for i in 0..(*pdma).n_chans { let chan = &mut (*pdma).chans.add(i); let base = SF_PDMA_REG_BASE(i);
        (*chan).regs.ctrl = base + PDMA_CTRL; (*chan).regs.xfer_type = base + PDMA_XFER_TYPE; (*chan).regs.xfer_size = base + PDMA_XFER_SIZE; (*chan).regs.dst_addr = base + PDMA_DST_ADDR; (*chan).regs.src_addr = base + PDMA_SRC_ADDR; (*chan).regs.act_type = base + PDMA_ACT_TYPE; (*chan).regs.residue = base + PDMA_REMAINING_BYTE; (*chan).regs.cur_dst_addr = base + PDMA_CUR_DST_ADDR; (*chan).regs.cur_src_addr = base + PDMA_CUR_SRC_ADDR;
        (*chan).pdma = pdma; (*chan).pm_state = RUNNING; (*chan).slave_id = i; (*chan).xfer_err = false; spin_lock_init(&mut (*chan).lock); (*chan).vchan.desc_free = Some(sf_pdma_free_desc); vchan_init(&mut (*chan).vchan, &mut (*pdma).dma_dev); writel(PDMA_CLEAR_CTRL, (*chan).regs.ctrl); tasklet_setup(&mut (*chan).done_tasklet, sf_pdma_donebh_tasklet); tasklet_setup(&mut (*chan).err_tasklet, sf_pdma_errbh_tasklet);
    }
}

unsafe fn sf_pdma_probe(pdev: *mut platform_device) -> i32 {
    let mut n_chans = 0; let ret = of_property_read_u32((*pdev).dev.of_node, "dma-channels", &mut n_chans);
    if ret != 0 { dev_dbg(&(*pdev).dev, "set number of channels to default value: 4\n"); n_chans = PDMA_MAX_NR_CH; } else if n_chans > PDMA_MAX_NR_CH { dev_err(&(*pdev).dev, "the number of channels exceeds the maximum\n"); return -EINVAL; }
    let pdma = devm_kzalloc_struct::<sf_pdma>(&mut (*pdev).dev, n_chans, GFP_KERNEL); if pdma.is_null() { return -ENOMEM; } (*pdma).n_chans = n_chans; (*pdma).transfer_type = PDMA_FULL_SPEED | PDMA_STRICT_ORDERING;
    let ddata = device_get_match_data(&(*pdev).dev); if !ddata.is_null() && ((*ddata).quirks & PDMA_QUIRK_NO_STRICT_ORDERING) != 0 { (*pdma).transfer_type &= !PDMA_STRICT_ORDERING; }
    (*pdma).membase = devm_platform_ioremap_resource(pdev, 0); if IS_ERR!((*pdma).membase) { return PTR_ERR!((*pdma).membase); }
    let ret = sf_pdma_irq_init(pdev, pdma); if ret != 0 { return ret; } sf_pdma_setup_chans(pdma); (*pdma).dma_dev.dev = &mut (*pdev).dev;
    dma_cap_set!(DMA_MEMCPY, (*pdma).dma_dev.cap_mask); (*pdma).dma_dev.copy_align = 2; (*pdma).dma_dev.src_addr_widths = DMA_SLAVE_BUSWIDTH_ALL; (*pdma).dma_dev.dst_addr_widths = DMA_SLAVE_BUSWIDTH_ALL; (*pdma).dma_dev.directions = BIT(DMA_MEM_TO_MEM); (*pdma).dma_dev.residue_granularity = DMA_RESIDUE_GRANULARITY_DESCRIPTOR; (*pdma).dma_dev.descriptor_reuse = true;
    (*pdma).dma_dev.device_alloc_chan_resources = Some(sf_pdma_alloc_chan_resources); (*pdma).dma_dev.device_free_chan_resources = Some(sf_pdma_free_chan_resources); (*pdma).dma_dev.device_tx_status = Some(sf_pdma_tx_status); (*pdma).dma_dev.device_prep_dma_memcpy = Some(sf_pdma_prep_dma_memcpy); (*pdma).dma_dev.device_config = Some(sf_pdma_slave_config); (*pdma).dma_dev.device_terminate_all = Some(sf_pdma_terminate_all); (*pdma).dma_dev.device_issue_pending = Some(sf_pdma_issue_pending); platform_set_drvdata(pdev, pdma);
    let ret = dma_set_mask_and_coherent(&mut (*pdev).dev, DMA_BIT_MASK(64)); if ret != 0 { dev_warn(&(*pdev).dev, "Failed to set DMA mask. Fall back to default.\n"); }
    let ret = dma_async_device_register(&mut (*pdma).dma_dev); if ret != 0 { dev_err(&(*pdev).dev, "Can't register SiFive Platform DMA. (%d)\n", ret); return ret; }
    let ret = of_dma_controller_register((*pdev).dev.of_node, of_dma_xlate_by_chan_id, pdma); if ret < 0 { dev_err(&(*pdev).dev, "Can't register SiFive Platform OF_DMA. (%d)\n", ret); dma_async_device_unregister(&mut (*pdma).dma_dev); return ret; } 0
}

unsafe fn sf_pdma_remove(pdev: *mut platform_device) { let pdma = platform_get_drvdata(pdev); for i in 0..(*pdma).n_chans { let ch = &mut (*pdma).chans.add(i); devm_free_irq(&mut (*pdev).dev, (*ch).txirq, ch as *mut _ as *mut _); devm_free_irq(&mut (*pdev).dev, (*ch).errirq, ch as *mut _ as *mut _); list_del!((*ch).vchan.chan.device_node); tasklet_kill(&mut (*ch).vchan.task); tasklet_kill(&mut (*ch).done_tasklet); tasklet_kill(&mut (*ch).err_tasklet); } if !(*pdev).dev.of_node.is_null() { of_dma_controller_free((*pdev).dev.of_node); } dma_async_device_unregister(&mut (*pdma).dma_dev); }

static MPFS_PDMA: sf_pdma_driver_platdata = sf_pdma_driver_platdata { quirks: PDMA_QUIRK_NO_STRICT_ORDERING };
static SF_PDMA_DT_IDS: &[of_device_id] = &[of_device_id { compatible: "sifive,fu540-c000-pdma", data: core::ptr::null() }, of_device_id { compatible: "sifive,pdma0", data: core::ptr::null() }, of_device_id { compatible: "microchip,mpfs-pdma", data: &MPFS_PDMA as *const _ }, of_device_id::default()];
static mut SF_PDMA_DRIVER: platform_driver = platform_driver { probe: Some(sf_pdma_probe), remove: Some(sf_pdma_remove), driver: driver { name: "sf-pdma", of_match_table: SF_PDMA_DT_IDS } };

unsafe fn sf_pdma_init() -> i32 { platform_driver_register(&mut SF_PDMA_DRIVER) }
unsafe fn sf_pdma_exit() { platform_driver_unregister(&mut SF_PDMA_DRIVER); }

// do early init
subsys_initcall!(sf_pdma_init);
module_exit!(sf_pdma_exit);
module_license!("GPL v2");
module_description!("SiFive Platform DMA driver");
module_author!("Green Wan <green.wan@sifive.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
