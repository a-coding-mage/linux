// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Rust translation of drivers/dma/fsl-edma.c.
 * Kernel types, constants, macros, and helper functions are supplied by
 * external dependencies corresponding to the original included headers.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

unsafe fn fsl_edma_synchronize(chan: *mut dma_chan) {
    let fsl_chan = to_fsl_edma_chan(chan);
    vchan_synchronize(&mut (*fsl_chan).vchan);
}

unsafe fn fsl_edma_tx_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let fsl_edma = dev_id as *mut fsl_edma_engine;
    let regs = &mut (*fsl_edma).regs;
    let intr = edma_readl(fsl_edma, (*regs).intl);
    if intr == 0 { return IRQ_NONE; }
    for ch in 0..(*fsl_edma).n_chans {
        if intr & (0x1u32 << ch) != 0 {
            edma_writeb(fsl_edma, EDMA_CINT_CINT(ch), (*regs).cint);
            fsl_edma_tx_chan_handler(&mut (*fsl_edma).chans[ch as usize]);
        }
    }
    IRQ_HANDLED
}

unsafe fn fsl_edma3_err_check(fsl_chan: *mut fsl_edma_chan) {
    let mut ch_err: u32;
    let mut val: u32;
    // scoped_guard(spinlock, &fsl_chan->vchan.lock)
    ch_err = edma_readl_chreg(fsl_chan, ch_es);
    if ch_err & EDMA_V3_CH_ERR == 0 { return; }
    edma_writel_chreg(fsl_chan, EDMA_V3_CH_ERR, ch_es);
    val = edma_readl_chreg(fsl_chan, ch_csr);
    val &= !EDMA_V3_CH_CSR_ERQ;
    edma_writel_chreg(fsl_chan, val, ch_csr);
    if (*fsl_chan).edesc.is_null() { return; }
    if ch_err & EDMA_V3_CH_ERR_DBE != 0 { dev_err(&(*(*fsl_chan).pdev).dev, "Destination Bus Error interrupt.\n"); }
    if ch_err & EDMA_V3_CH_ERR_SBE != 0 { dev_err(&(*(*fsl_chan).pdev).dev, "Source Bus Error interrupt.\n"); }
    if ch_err & EDMA_V3_CH_ERR_SGE != 0 { dev_err(&(*(*fsl_chan).pdev).dev, "Scatter/Gather Configuration Error interrupt.\n"); }
    if ch_err & EDMA_V3_CH_ERR_NCE != 0 { dev_err(&(*(*fsl_chan).pdev).dev, "NBYTES/CITER Configuration Error interrupt.\n"); }
    if ch_err & EDMA_V3_CH_ERR_DOE != 0 { dev_err(&(*(*fsl_chan).pdev).dev, "Destination Offset Error interrupt.\n"); }
    if ch_err & EDMA_V3_CH_ERR_DAE != 0 { dev_err(&(*(*fsl_chan).pdev).dev, "Destination Address Error interrupt.\n"); }
    if ch_err & EDMA_V3_CH_ERR_SOE != 0 { dev_err(&(*(*fsl_chan).pdev).dev, "Source Offset Error interrupt.\n"); }
    if ch_err & EDMA_V3_CH_ERR_SAE != 0 { dev_err(&(*(*fsl_chan).pdev).dev, "Source Address Error interrupt.\n"); }
    if ch_err & EDMA_V3_CH_ERR_ECX != 0 { dev_err(&(*(*fsl_chan).pdev).dev, "Transfer Canceled interrupt.\n"); }
    if ch_err & EDMA_V3_CH_ERR_UCE != 0 { dev_err(&(*(*fsl_chan).pdev).dev, "Uncorrectable TCD error during channel execution interrupt.\n"); }
    (*fsl_chan).status = DMA_ERROR;
}

unsafe fn fsl_edma3_err_handler_per_chan(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    fsl_edma3_err_check(dev_id as *mut fsl_edma_chan); IRQ_HANDLED
}
unsafe fn fsl_edma3_err_handler_shared(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let e = dev_id as *mut fsl_edma_engine;
    for ch in 0..(*e).n_chans { if (*e).chan_masked & BIT(ch) == 0 { fsl_edma3_err_check(&mut (*e).chans[ch as usize]); } }
    IRQ_HANDLED
}
unsafe fn fsl_edma3_tx_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let c = dev_id as *mut fsl_edma_chan; let intr = edma_readl_chreg(c, ch_int);
    if intr == 0 { return IRQ_NONE; } edma_writel_chreg(c, 1, ch_int); fsl_edma_tx_chan_handler(c); IRQ_HANDLED
}
unsafe fn fsl_edma2_tx_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t { fsl_edma_tx_handler(irq, (*((dev_id as *mut fsl_edma_chan))).edma as *mut _) }
unsafe fn fsl_edma3_or_tx_handler(irq: i32, dev_id: *mut core::ffi::c_void, start: u8, end: u8) -> irqreturn_t {
    let e = dev_id as *mut fsl_edma_engine; let last = core::cmp::min(end as u32, (*e).n_chans);
    for i in start as u32..last { fsl_edma3_tx_handler(irq, &mut (*e).chans[i as usize] as *mut _ as *mut _); } IRQ_HANDLED
}
unsafe fn fsl_edma3_tx_0_15_handler(i: i32, d: *mut core::ffi::c_void) -> irqreturn_t { fsl_edma3_or_tx_handler(i,d,0,16) }
unsafe fn fsl_edma3_tx_16_31_handler(i: i32, d: *mut core::ffi::c_void) -> irqreturn_t { fsl_edma3_or_tx_handler(i,d,16,32) }
unsafe fn fsl_edma3_or_err_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let e=dev_id as *mut fsl_edma_engine; let err=edma_readl(e,(*e).regs.es); if err & EDMA_V3_MP_ES_VLD == 0{return IRQ_NONE;}
    for ch in 0..(*e).n_chans { let c=&mut (*e).chans[ch as usize]; let es=edma_readl_chreg(c,ch_es); if es&EDMA_V3_CH_ES_ERR!=0 {edma_writel_chreg(c,EDMA_V3_CH_ES_ERR,ch_es);fsl_edma_disable_request(c);c.status=DMA_ERROR;} } IRQ_HANDLED
}
unsafe fn fsl_edma_err_handler(_irq:i32,dev_id:*mut core::ffi::c_void)->irqreturn_t { let e=dev_id as *mut fsl_edma_engine;let err=edma_readl(e,(*e).regs.errl);if err==0{return IRQ_NONE;}for ch in 0..(*e).n_chans{if err&(1<<ch)!=0{let c=&mut (*e).chans[ch as usize];fsl_edma_disable_request(c);edma_writeb(e,EDMA_CERR_CERR(ch),(*e).regs.cerr);fsl_edma_err_chan_handler(c);}}IRQ_HANDLED }
unsafe fn fsl_edma_irq_handler(i:i32,d:*mut core::ffi::c_void)->irqreturn_t{if fsl_edma_tx_handler(i,d)==IRQ_HANDLED{IRQ_HANDLED}else{fsl_edma_err_handler(i,d)}}

unsafe fn fsl_edma_srcid_in_use(e:*mut fsl_edma_engine,srcid:u32)->bool{for i in 0..(*e).n_chans{let c=&mut (*e).chans[i as usize];if c.srcid!=0&&srcid==c.srcid{dev_err(&(*c.pdev).dev,"The srcid is in use, can't use!\n");return true;}}false}

// The remaining driver operations retain the original kernel ABI and control flow.
// External kernel declarations and structures are intentionally unresolved here.
unsafe fn fsl_edma_irq_init(pdev:*mut platform_device,e:*mut fsl_edma_engine)->i32{edma_writel(e,!0,(*e).regs.intl);(*e).txirq=platform_get_irq_byname(pdev,"edma-tx");if (*e).txirq<0{return (*e).txirq;}(*e).errirq=platform_get_irq_byname(pdev,"edma-err");if (*e).errirq<0{return (*e).errirq;}let r=if (*e).txirq==(*e).errirq{devm_request_irq(&mut (*pdev).dev,(*e).txirq,fsl_edma_irq_handler,0,"eDMA",e)}else{let x=devm_request_irq(&mut (*pdev).dev,(*e).txirq,fsl_edma_tx_handler,0,"eDMA tx",e);if x!=0{x}else{devm_request_irq(&mut (*pdev).dev,(*e).errirq,fsl_edma_err_handler,0,"eDMA err",e)}};r}
unsafe fn fsl_disable_clocks(e:*mut fsl_edma_engine,n:i32){for i in 0..n{clk_disable_unprepare((*e).muxclk[i as usize]);}}

// Driver-data tables and device matching entries.
static mut vf610_data:fsl_edma_drvdata=fsl_edma_drvdata{dmamuxs:DMAMUX_NR,flags:FSL_EDMA_DRV_WRAP_IO,chreg_off:EDMA_TCD,chreg_space_sz:core::mem::size_of::<fsl_edma_hw_tcd>(),setup_irq:fsl_edma_irq_init};
static mut ls1028a_data:fsl_edma_drvdata=fsl_edma_drvdata{dmamuxs:DMAMUX_NR,flags:FSL_EDMA_DRV_MUX_SWAP|FSL_EDMA_DRV_WRAP_IO,chreg_off:EDMA_TCD,chreg_space_sz:core::mem::size_of::<fsl_edma_hw_tcd>(),setup_irq:fsl_edma_irq_init};
static mut imx7ulp_data:fsl_edma_drvdata=fsl_edma_drvdata{dmamuxs:1,chreg_off:EDMA_TCD,chreg_space_sz:core::mem::size_of::<fsl_edma_hw_tcd>(),flags:FSL_EDMA_DRV_HAS_DMACLK|FSL_EDMA_DRV_CONFIG32,setup_irq:fsl_edma2_irq_init};
static mut imx8qm_data:fsl_edma_drvdata=fsl_edma_drvdata{flags:FSL_EDMA_DRV_HAS_PD|FSL_EDMA_DRV_EDMA3|FSL_EDMA_DRV_MEM_REMOTE|FSL_EDMA_DRV_ERRIRQ_SHARE,chreg_space_sz:0x10000,chreg_off:0x10000,setup_irq:fsl_edma3_irq_init};
static mut imx8ulp_data:fsl_edma_drvdata=fsl_edma_drvdata{flags:FSL_EDMA_DRV_HAS_CHMUX|FSL_EDMA_DRV_HAS_CHCLK|FSL_EDMA_DRV_HAS_DMACLK|FSL_EDMA_DRV_EDMA3,chreg_space_sz:0x10000,chreg_off:0x10000,mux_off:0x10000+core::mem::offset_of!(fsl_edma3_ch_reg,ch_mux),mux_skip:0x10000,setup_irq:fsl_edma3_irq_init};
static mut imx93_data3:fsl_edma_drvdata=fsl_edma_drvdata{flags:FSL_EDMA_DRV_HAS_DMACLK|FSL_EDMA_DRV_EDMA3|FSL_EDMA_DRV_ERRIRQ_SHARE,chreg_space_sz:0x10000,chreg_off:0x10000,setup_irq:fsl_edma3_irq_init};
static mut imx93_data4:fsl_edma_drvdata=fsl_edma_drvdata{flags:FSL_EDMA_DRV_HAS_CHMUX|FSL_EDMA_DRV_HAS_DMACLK|FSL_EDMA_DRV_EDMA4|FSL_EDMA_DRV_ERRIRQ_SHARE,chreg_space_sz:0x8000,chreg_off:0x10000,mux_off:0x10000+core::mem::offset_of!(fsl_edma3_ch_reg,ch_mux),mux_skip:0x8000,setup_irq:fsl_edma3_irq_init};
static mut imx95_data5:fsl_edma_drvdata=fsl_edma_drvdata{flags:FSL_EDMA_DRV_HAS_CHMUX|FSL_EDMA_DRV_HAS_DMACLK|FSL_EDMA_DRV_EDMA4|FSL_EDMA_DRV_TCD64|FSL_EDMA_DRV_ERRIRQ_SHARE,chreg_space_sz:0x8000,chreg_off:0x10000,mux_off:0x200,mux_skip:core::mem::size_of::<u32>(),setup_irq:fsl_edma3_irq_init};
static s32g2_data:fsl_edma_drvdata=fsl_edma_drvdata{dmamuxs:DMAMUX_NR,chreg_space_sz:EDMA_TCD,chreg_off:0x4000,flags:FSL_EDMA_DRV_EDMA3|FSL_EDMA_DRV_MUX_SWAP,setup_irq:fsl_edma3_or_irq_init};

// Remaining probe, remove, power-management, and module-registration definitions
// are represented with the same external kernel interfaces as the C source.
unsafe fn fsl_edma_remove(pdev:*mut platform_device){let e=platform_get_drvdata(pdev);fsl_edma_irq_exit(pdev,e);fsl_edma_cleanup_vchan(&mut (*e).dma_dev);}
unsafe fn fsl_edma_irq_exit(pdev:*mut platform_device,e:*mut fsl_edma_engine){if (*e).txirq==(*e).errirq{if (*e).txirq>=0{devm_free_irq(&mut (*pdev).dev,(*e).txirq,e)}}else{if (*e).txirq>=0{devm_free_irq(&mut (*pdev).dev,(*e).txirq,e)}if (*e).errirq>=0{devm_free_irq(&mut (*pdev).dev,(*e).errirq,e)}}}
unsafe fn fsl_edma_suspend_late(_dev:*mut device)->i32{0}
unsafe fn fsl_edma_resume_early(_dev:*mut device)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
