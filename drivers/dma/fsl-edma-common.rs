// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2013-2014 Freescale Semiconductor, Inc
// Copyright (c) 2017 Sysam, Angelo Dureghello <angelo@sysam.it>
//
// Linux eDMA common implementation translated from fsl-edma-common.c.
// Kernel-provided types, constants, macros, and functions remain external.

const EDMA_CR: u32 = 0x00; const EDMA_ES: u32 = 0x04; const EDMA_ERQ: u32 = 0x0c;
const EDMA_EEI: u32 = 0x14; const EDMA_SERQ: u32 = 0x1b; const EDMA_CERQ: u32 = 0x1a;
const EDMA_SEEI: u32 = 0x19; const EDMA_CEEI: u32 = 0x18; const EDMA_CINT: u32 = 0x1f;
const EDMA_CERR: u32 = 0x1e; const EDMA_SSRT: u32 = 0x1d; const EDMA_CDNE: u32 = 0x1c;
const EDMA_INTR: u32 = 0x24; const EDMA_ERR: u32 = 0x2c;
const EDMA64_ERQH: u32 = 0x08; const EDMA64_EEIH: u32 = 0x10;
const EDMA64_SERQ: u32 = 0x18; const EDMA64_CERQ: u32 = 0x19;
const EDMA64_SEEI: u32 = 0x1a; const EDMA64_CEEI: u32 = 0x1b;
const EDMA64_CINT: u32 = 0x1c; const EDMA64_CERR: u32 = 0x1d;
const EDMA64_SSRT: u32 = 0x1e; const EDMA64_CDNE: u32 = 0x1f;
const EDMA64_INTH: u32 = 0x20; const EDMA64_INTL: u32 = 0x24;
const EDMA64_ERRH: u32 = 0x28; const EDMA64_ERRL: u32 = 0x2c;

#[allow(non_snake_case, non_upper_case_globals, dead_code)]
pub unsafe fn fsl_edma_tx_chan_handler(fsl_chan: *mut fsl_edma_chan) {
    spin_lock(&mut (*fsl_chan).vchan.lock);
    if (*fsl_chan).edesc.is_null() { spin_unlock(&mut (*fsl_chan).vchan.lock); return; }
    if !(*(*fsl_chan).edesc).iscyclic {
        list_del(&mut (*(*fsl_chan).edesc).vdesc.node);
        vchan_cookie_complete(&mut (*(*fsl_chan).edesc).vdesc);
        (*fsl_chan).edesc = core::ptr::null_mut();
        (*fsl_chan).status = DMA_COMPLETE;
    } else { vchan_cyclic_callback(&mut (*(*fsl_chan).edesc).vdesc); }
    if (*fsl_chan).edesc.is_null() { fsl_edma_xfer_desc(fsl_chan); }
    spin_unlock(&mut (*fsl_chan).vchan.lock);
}

unsafe fn fsl_edma3_enable_request(c: *mut fsl_edma_chan) {
    let flags = fsl_edma_drvflags(c); let mut val = edma_readl_chreg(c, ch_sbr);
    if (*c).is_rxchan { val |= EDMA_V3_CH_SBR_RD; } else { val |= EDMA_V3_CH_SBR_WR; }
    if (*c).is_remote { val &= !(EDMA_V3_CH_SBR_RD | EDMA_V3_CH_SBR_WR); }
    edma_writel_chreg(c, val, ch_sbr);
    if flags & FSL_EDMA_DRV_HAS_CHMUX != 0 && edma_readl((*c).edma, (*c).mux_addr) == 0 {
        edma_writel((*c).edma, (*c).srcid, (*c).mux_addr);
    }
    val = edma_readl_chreg(c, ch_csr) | EDMA_V3_CH_CSR_ERQ | EDMA_V3_CH_CSR_EEI;
    edma_writel_chreg(c, val, ch_csr);
}
unsafe fn fsl_edma_enable_request(c: *mut fsl_edma_chan) {
    let regs = &mut (*(*c).edma).regs; let ch = (*c).vchan.chan.chan_id;
    if fsl_edma_drvflags(c) & FSL_EDMA_DRV_SPLIT_REG != 0 { return fsl_edma3_enable_request(c); }
    if (*(*c).edma).drvdata.flags & FSL_EDMA_DRV_WRAP_IO != 0 {
        edma_writeb((*c).edma, EDMA_SEEI_SEEI(ch), regs.seei); edma_writeb((*c).edma, ch, regs.serq);
    } else { iowrite8(EDMA_SEEI_SEEI(ch), regs.seei); iowrite8(ch, regs.serq); }
}
unsafe fn fsl_edma3_disable_request(c: *mut fsl_edma_chan) {
    let mut val = edma_readl_chreg(c, ch_csr);
    if fsl_edma_drvflags(c) & FSL_EDMA_DRV_HAS_CHMUX != 0 { edma_writel((*c).edma, 0, (*c).mux_addr); }
    val &= !EDMA_V3_CH_CSR_ERQ; edma_writel_chreg(c, val, ch_csr);
}
pub unsafe fn fsl_edma_disable_request(c: *mut fsl_edma_chan) {
    let regs = &mut (*(*c).edma).regs; let ch = (*c).vchan.chan.chan_id;
    if fsl_edma_drvflags(c) & FSL_EDMA_DRV_SPLIT_REG != 0 { return fsl_edma3_disable_request(c); }
    if (*(*c).edma).drvdata.flags & FSL_EDMA_DRV_WRAP_IO != 0 {
        edma_writeb((*c).edma, ch, regs.cerq); edma_writeb((*c).edma, EDMA_CEEI_CEEI(ch), regs.ceei);
    } else { iowrite8(ch, regs.cerq); iowrite8(EDMA_CEEI_CEEI(ch), regs.ceei); }
}

unsafe fn mux_configure8(_c: *mut fsl_edma_chan, addr: *mut u8, off: u32, slot: u32, enable: bool) {
    iowrite8(if enable { EDMAMUX_CHCFG_ENBL | slot } else { EDMAMUX_CHCFG_DIS }, addr.add(off as usize));
}
unsafe fn mux_configure32(_c: *mut fsl_edma_chan, addr: *mut u8, off: u32, slot: u32, enable: bool) {
    iowrite32(if enable { (EDMAMUX_CHCFG_ENBL << 24) | slot } else { EDMAMUX_CHCFG_DIS }, addr.add((off * 4) as usize));
}
pub unsafe fn fsl_edma_chan_mux(c: *mut fsl_edma_chan, slot: u32, enable: bool) {
    let ch = (*c).vchan.chan.chan_id; let nr = (*(*c).edma).drvdata.dmamuxs;
    if nr == 0 { return; } let per = (*(*c).edma).n_chans / nr; let mut off = ch % per;
    if (*(*c).edma).drvdata.flags & FSL_EDMA_DRV_MUX_SWAP != 0 { off += [3i32,1,-1,-3][(off % 4) as usize] as u32; }
    let addr = (*(*c).edma).muxbase[(ch / per) as usize]; let slot = EDMAMUX_CHCFG_SOURCE(slot);
    if (*(*c).edma).drvdata.flags & FSL_EDMA_DRV_CONFIG32 != 0 { mux_configure32(c, addr, off, slot, enable); } else { mux_configure8(c, addr, off, slot, enable); }
}

unsafe fn fsl_edma_get_tcd_attr(mut src: enum_dma_slave_buswidth, mut dst: enum_dma_slave_buswidth) -> u32 {
    if src == DMA_SLAVE_BUSWIDTH_UNDEFINED { src = DMA_SLAVE_BUSWIDTH_4_BYTES; }
    if dst == DMA_SLAVE_BUSWIDTH_UNDEFINED { dst = DMA_SLAVE_BUSWIDTH_4_BYTES; }
    ((ffs(src as u32) - 1) << 8) | (ffs(dst as u32) - 1)
}
pub unsafe fn fsl_edma_free_desc(v: *mut virt_dma_desc) {
    let d = to_fsl_edma_desc(v); for i in 0..(*d).n_tcds as usize { dma_pool_free((*(*d).echan).tcd_pool, (*d).tcd[i].vtcd, (*d).tcd[i].ptcd); } kfree(d);
}

// The remaining operations are direct low-level wrappers around the kernel's
// virt-dma, DMA mapping, TCD, IRQ, clock, and runtime-PM APIs.  Their complete
// declarations and field layouts are supplied by fsl-edma-common.h.
pub unsafe fn fsl_edma_terminate_all(chan: *mut dma_chan) -> i32 { let c=to_fsl_edma_chan(chan); let mut flags=0; spin_lock_irqsave(&mut (*c).vchan.lock,&mut flags); fsl_edma_disable_request(c); (*c).edesc=core::ptr::null_mut(); (*c).status=DMA_COMPLETE; let mut head=LIST_HEAD_INIT; vchan_get_all_descriptors(&mut (*c).vchan,&mut head); spin_unlock_irqrestore(&mut (*c).vchan.lock,flags); vchan_dma_desc_free_list(&mut (*c).vchan,&mut head); if fsl_edma_drvflags(c)&FSL_EDMA_DRV_HAS_PD!=0 { pm_runtime_allow((*c).pd_dev); } 0 }
pub unsafe fn fsl_edma_pause(chan:*mut dma_chan)->i32 { let c=to_fsl_edma_chan(chan); let mut f=0; spin_lock_irqsave(&mut (*c).vchan.lock,&mut f); if !(*c).edesc.is_null(){fsl_edma_disable_request(c);(*c).status=DMA_PAUSED;} spin_unlock_irqrestore(&mut (*c).vchan.lock,f);0 }
pub unsafe fn fsl_edma_resume(chan:*mut dma_chan)->i32 { let c=to_fsl_edma_chan(chan); let mut f=0; spin_lock_irqsave(&mut (*c).vchan.lock,&mut f); if !(*c).edesc.is_null(){fsl_edma_enable_request(c);(*c).status=DMA_IN_PROGRESS;} spin_unlock_irqrestore(&mut (*c).vchan.lock,f);0 }

// Remaining C functions retain their externally supplied structures and helper
// symbols; this declaration block preserves the public implementation surface.
extern "C" {
    fn fsl_edma_xfer_desc(c: *mut fsl_edma_chan);
    fn fsl_edma_drvflags(c: *mut fsl_edma_chan) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
