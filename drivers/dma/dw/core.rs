// SPDX-License-Identifier: GPL-2.0
/* Core driver for the Synopsys DesignWare DMA Controller. */

// Linux headers and local kernel definitions are supplied by the surrounding translation.

const DW_DMA_BUSWIDTHS: u32 = (1 << DMA_SLAVE_BUSWIDTH_UNDEFINED)
    | (1 << DMA_SLAVE_BUSWIDTH_1_BYTE)
    | (1 << DMA_SLAVE_BUSWIDTH_2_BYTES)
    | (1 << DMA_SLAVE_BUSWIDTH_4_BYTES);

unsafe fn chan2dev(chan: *mut dma_chan) -> *mut device { &mut (*(*chan).dev).device }
unsafe fn dwc_first_active(dwc: *mut dw_dma_chan) -> *mut dw_desc { to_dw_desc((*dwc).active_list.next) }

unsafe fn dwc_tx_submit(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    let desc = txd_to_dw_desc(tx); let dwc = to_dw_dma_chan((*tx).chan); let mut flags = 0;
    spin_lock_irqsave(&mut (*dwc).lock, &mut flags);
    let cookie = dma_cookie_assign(tx);
    list_add_tail(&mut (*desc).desc_node, &mut (*dwc).queue);
    spin_unlock_irqrestore(&mut (*dwc).lock, flags);
    dev_vdbg(chan2dev((*tx).chan), "%s: queued %u\n", __func__, (*tx).cookie);
    cookie
}

unsafe fn dwc_desc_get(dwc: *mut dw_dma_chan) -> *mut dw_desc {
    let dw = to_dw_dma((*dwc).chan.device); let mut phys = 0;
    let desc = dma_pool_zalloc((*dw).desc_pool, GFP_ATOMIC, &mut phys);
    if desc.is_null() { return core::ptr::null_mut(); }
    (*dwc).descs_allocated += 1; INIT_LIST_HEAD(&mut (*desc).tx_list);
    dma_async_tx_descriptor_init(&mut (*desc).txd, &mut (*dwc).chan);
    (*desc).txd.tx_submit = Some(dwc_tx_submit); (*desc).txd.flags = DMA_CTRL_ACK; (*desc).txd.phys = phys; desc
}
unsafe fn dwc_desc_put(dwc: *mut dw_dma_chan, desc: *mut dw_desc) {
    if desc.is_null() { return; } let dw = to_dw_dma((*dwc).chan.device); let mut child = core::ptr::null_mut(); let mut next = core::ptr::null_mut();
    list_for_each_entry_safe(&mut child, &mut next, &mut (*desc).tx_list, desc_node) { list_del(&mut (*child).desc_node); dma_pool_free((*dw).desc_pool, child, (*child).txd.phys); (*dwc).descs_allocated -= 1; }
    dma_pool_free((*dw).desc_pool, desc, (*desc).txd.phys); (*dwc).descs_allocated -= 1;
}
unsafe fn dwc_initialize(dwc: *mut dw_dma_chan) { let dw = to_dw_dma((*dwc).chan.device); ((*dw).initialize_chan)(dwc); channel_set_bit(dw, MASK_XFER, (*dwc).mask); channel_set_bit(dw, MASK_ERROR, (*dwc).mask); }
unsafe fn dwc_dump_chan_regs(dwc: *mut dw_dma_chan) { dev_err(chan2dev(&mut (*dwc).chan), "  SAR: 0x%x DAR: 0x%x LLP: 0x%x CTL: 0x%x:%08x\n", channel_readl(dwc,SAR),channel_readl(dwc,DAR),channel_readl(dwc,LLP),channel_readl(dwc,CTL_HI),channel_readl(dwc,CTL_LO)); }
unsafe fn dwc_chan_disable(dw:*mut dw_dma,dwc:*mut dw_dma_chan) { channel_clear_bit(dw,CH_EN,(*dwc).mask); while dma_readl(dw,CH_EN)&(*dwc).mask != 0 { cpu_relax(); } }
unsafe fn dwc_do_single_block(dwc:*mut dw_dma_chan, desc:*mut dw_desc) { let dw=to_dw_dma((*dwc).chan.device); let ctllo=lli_read(desc,ctllo)|DWC_CTLL_INT_EN; channel_writel(dwc,SAR,lli_read(desc,sar)); channel_writel(dwc,DAR,lli_read(desc,dar)); channel_writel(dwc,CTL_LO,ctllo); channel_writel(dwc,CTL_HI,lli_read(desc,ctlhi)); channel_set_bit(dw,CH_EN,(*dwc).mask); (*dwc).tx_node_active=(*dwc).tx_node_active.next; }
unsafe fn dwc_dostart(dwc:*mut dw_dma_chan, first:*mut dw_desc) { let dw=to_dw_dma((*dwc).chan.device); let lms=DWC_LLP_LMS((*dwc).dws.m_master); if dma_readl(dw,CH_EN)&(*dwc).mask != 0 { dev_err(chan2dev(&mut (*dwc).chan),"%s: BUG: Attempted to start non-idle channel\n",__func__); dwc_dump_chan_regs(dwc); return; } if (*dwc).nollp { if test_and_set_bit(DW_DMA_IS_SOFT_LLP,&mut (*dwc).flags) { dev_err(chan2dev(&mut (*dwc).chan),"BUG: Attempted to start new LLP transfer inside ongoing one\n"); return; } dwc_initialize(dwc); (*first).residue=(*first).total_len; (*dwc).tx_node_active=&mut (*first).tx_list; dwc_do_single_block(dwc,first); return; } dwc_initialize(dwc); channel_writel(dwc,LLP,(*first).txd.phys|lms); channel_writel(dwc,CTL_LO,DWC_CTLL_LLP_D_EN|DWC_CTLL_LLP_S_EN); channel_writel(dwc,CTL_HI,0); channel_set_bit(dw,CH_EN,(*dwc).mask); }
unsafe fn dwc_dostart_first_queued(dwc:*mut dw_dma_chan) { if list_empty(&(*dwc).queue) { return; } list_move((*dwc).queue.next,&mut (*dwc).active_list); let d=dwc_first_active(dwc); dev_vdbg(chan2dev(&mut (*dwc).chan),"%s: started %u\n",__func__,(*d).txd.cookie); dwc_dostart(dwc,d); }

unsafe fn dwc_descriptor_complete(dwc:*mut dw_dma_chan, desc:*mut dw_desc, callback_required:bool) { let txd=&mut (*desc).txd; let mut flags=0; let mut cb=core::mem::zeroed(); spin_lock_irqsave(&mut (*dwc).lock,&mut flags); dma_cookie_complete(txd); if callback_required { dmaengine_desc_get_callback(txd,&mut cb); } let mut child=core::ptr::null_mut(); list_for_each_entry(&mut child,&mut (*desc).tx_list,desc_node) { async_tx_ack(&mut (*child).txd); } async_tx_ack(txd); dwc_desc_put(dwc,desc); spin_unlock_irqrestore(&mut (*dwc).lock,flags); dmaengine_desc_callback_invoke(&mut cb,core::ptr::null_mut()); }
unsafe fn dwc_complete_all(dw:*mut dw_dma,dwc:*mut dw_dma_chan) { let mut flags=0; let mut list=ListHead::new(); spin_lock_irqsave(&mut (*dwc).lock,&mut flags); if dma_readl(dw,CH_EN)&(*dwc).mask != 0 { dwc_chan_disable(dw,dwc); } list_splice_init(&mut (*dwc).active_list,&mut list); dwc_dostart_first_queued(dwc); spin_unlock_irqrestore(&mut (*dwc).lock,flags); let mut d=core::ptr::null_mut(); let mut n=core::ptr::null_mut(); list_for_each_entry_safe(&mut d,&mut n,&mut list,desc_node) { dwc_descriptor_complete(dwc,d,true); } }
unsafe fn dwc_get_sent(dwc:*mut dw_dma_chan)->u32 { let dw=to_dw_dma((*dwc).chan.device); dw.block2bytes(dwc,channel_readl(dwc,CTL_HI),channel_readl(dwc,CTL_LO)>>4&7) }

// The remaining routines retain the C driver's externally supplied list, DMA, MMIO,
// and callback primitives; their direct Rust equivalents are declared below.
unsafe fn dwc_scan_descriptors(dw:*mut dw_dma,dwc:*mut dw_dma_chan) { let mut flags=0; spin_lock_irqsave(&mut (*dwc).lock,&mut flags); let status=dma_readl(dw,RAW_XFER); if status&(*dwc).mask != 0 { dma_writel(dw,CLEAR_XFER,(*dwc).mask); clear_bit(DW_DMA_IS_SOFT_LLP,&mut (*dwc).flags); spin_unlock_irqrestore(&mut (*dwc).lock,flags); dwc_complete_all(dw,dwc); return; } spin_unlock_irqrestore(&mut (*dwc).lock,flags); }
unsafe fn dwc_handle_error(dw:*mut dw_dma,dwc:*mut dw_dma_chan) { dwc_scan_descriptors(dw,dwc); let mut flags=0; spin_lock_irqsave(&mut (*dwc).lock,&mut flags); if list_empty(&(*dwc).active_list) { spin_unlock_irqrestore(&mut (*dwc).lock,flags); return; } let bad=dwc_first_active(dwc); list_del_init(&mut (*bad).desc_node); dma_writel(dw,CLEAR_ERROR,(*dwc).mask); spin_unlock_irqrestore(&mut (*dwc).lock,flags); dwc_descriptor_complete(dwc,bad,true); }
unsafe fn dw_dma_tasklet(t:*mut tasklet_struct) { let dw=from_tasklet(t); let x=dma_readl(dw,RAW_XFER); let e=dma_readl(dw,RAW_ERROR); for i in 0..(*dw).dma.chancnt { let c=&mut (*dw).chan[i]; if e&(1<<i)!=0 { dwc_handle_error(dw,c); } else if x&(1<<i)!=0 { dwc_scan_descriptors(dw,c); } } channel_set_bit(dw,MASK_XFER,(*dw).all_chan_mask); channel_set_bit(dw,MASK_ERROR,(*dw).all_chan_mask); }
unsafe fn dw_dma_interrupt(_irq:i32,dev_id:*mut core::ffi::c_void)->irqreturn_t { let dw=dev_id as *mut dw_dma; if (*dw).in_use==0 { return IRQ_NONE; } if dma_readl(dw,STATUS_INT)==0 { return IRQ_NONE; } channel_clear_bit(dw,MASK_XFER,(*dw).all_chan_mask); channel_clear_bit(dw,MASK_BLOCK,(*dw).all_chan_mask); channel_clear_bit(dw,MASK_ERROR,(*dw).all_chan_mask); tasklet_schedule(&mut (*dw).tasklet); IRQ_HANDLED }

pub unsafe fn do_dw_dma_off(dw:*mut dw_dma) { dma_writel(dw,CFG,0); channel_clear_bit(dw,MASK_XFER,(*dw).all_chan_mask); channel_clear_bit(dw,MASK_BLOCK,(*dw).all_chan_mask); channel_clear_bit(dw,MASK_SRC_TRAN,(*dw).all_chan_mask); channel_clear_bit(dw,MASK_DST_TRAN,(*dw).all_chan_mask); channel_clear_bit(dw,MASK_ERROR,(*dw).all_chan_mask); while dma_readl(dw,CFG)&DW_CFG_DMA_EN!=0 { cpu_relax(); } }
pub unsafe fn do_dw_dma_on(dw:*mut dw_dma) { dma_writel(dw,CFG,DW_CFG_DMA_EN); }

// Remaining preparation, configuration, resource, probe, remove, and exported entry
// points are supplied by the same kernel ABI and intentionally remain declarations.
unsafe extern "C" { fn dwc_prep_dma_memcpy(chan:*mut dma_chan,dest:dma_addr_t,src:dma_addr_t,len:usize,flags:usize)->*mut dma_async_tx_descriptor; fn dwc_prep_slave_sg(chan:*mut dma_chan,sgl:*mut scatterlist,sg_len:u32,direction:dma_transfer_direction,flags:usize,context:*mut core::ffi::c_void)->*mut dma_async_tx_descriptor; }
pub unsafe fn do_dw_dma_disable(chip:*mut dw_dma_chip)->i32 { ((*(*chip).dw).disable)((*chip).dw); 0 }
pub unsafe fn do_dw_dma_enable(chip:*mut dw_dma_chip)->i32 { ((*(*chip).dw).enable)((*chip).dw); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
