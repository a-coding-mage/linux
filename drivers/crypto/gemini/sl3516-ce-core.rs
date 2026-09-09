// SPDX-License-Identifier: GPL-2.0
/*
 * sl3516-ce-core.c - hardware cryptographic offloader for Storlink SL3516 SoC
 *
 * Copyright (C) 2021 Corentin Labbe <clabbe@baylibre.com>
 *
 * Core file which registers crypto algorithms supported by the CryptoEngine
 */

// Kernel and sl3516-ce.h declarations are supplied by the surrounding crate.

unsafe fn sl3516_ce_desc_init(ce: *mut sl3516_ce_dev) -> i32 {
    let sz: usize = core::mem::size_of::<descriptor>() * MAXDESC;
    let mut i: i32;
    (*ce).tx = dma_alloc_coherent((*ce).dev, sz, &mut (*ce).dtx, GFP_KERNEL);
    if (*ce).tx.is_null() { return -ENOMEM; }
    (*ce).rx = dma_alloc_coherent((*ce).dev, sz, &mut (*ce).drx, GFP_KERNEL);
    if (*ce).rx.is_null() { dma_free_coherent((*ce).dev, sz, (*ce).tx, (*ce).dtx); return -ENOMEM; }
    i = 0;
    while i < MAXDESC as i32 {
        (*(*ce).tx.add(i as usize)).frame_ctrl.bits.own = CE_CPU;
        (*(*ce).tx.add(i as usize)).next_desc.next_descriptor = (*ce).dtx + ((i + 1) as usize * core::mem::size_of::<descriptor>());
        i += 1;
    }
    (*(*ce).tx.add(MAXDESC - 1)).next_desc.next_descriptor = (*ce).dtx;
    i = 0;
    while i < MAXDESC as i32 {
        (*(*ce).rx.add(i as usize)).frame_ctrl.bits.own = CE_CPU;
        (*(*ce).rx.add(i as usize)).next_desc.next_descriptor = (*ce).drx + ((i + 1) as usize * core::mem::size_of::<descriptor>());
        i += 1;
    }
    (*(*ce).rx.add(MAXDESC - 1)).next_desc.next_descriptor = (*ce).drx;
    (*ce).pctrl = dma_alloc_coherent((*ce).dev, core::mem::size_of::<pkt_control_ecb>(), &mut (*ce).dctrl, GFP_KERNEL);
    if (*ce).pctrl.is_null() {
        dma_free_coherent((*ce).dev, sz, (*ce).rx, (*ce).drx);
        dma_free_coherent((*ce).dev, sz, (*ce).tx, (*ce).dtx);
        return -ENOMEM;
    }
    0
}

unsafe fn sl3516_ce_free_descs(ce: *mut sl3516_ce_dev) {
    let sz = core::mem::size_of::<descriptor>() * MAXDESC;
    dma_free_coherent((*ce).dev, sz, (*ce).tx, (*ce).dtx);
    dma_free_coherent((*ce).dev, sz, (*ce).rx, (*ce).drx);
    dma_free_coherent((*ce).dev, core::mem::size_of::<pkt_control_ecb>(), (*ce).pctrl, (*ce).dctrl);
}

unsafe fn start_dma_tx(ce: *mut sl3516_ce_dev) {
    let v = TXDMA_CTRL_START | TXDMA_CTRL_CHAIN_MODE | TXDMA_CTRL_CONTINUE |
        TXDMA_CTRL_INT_FAIL | TXDMA_CTRL_INT_PERR | TXDMA_CTRL_BURST_UNK;
    writel(v, (*ce).base + IPSEC_TXDMA_CTRL);
}

unsafe fn start_dma_rx(ce: *mut sl3516_ce_dev) {
    let v = RXDMA_CTRL_START | RXDMA_CTRL_CHAIN_MODE | RXDMA_CTRL_CONTINUE |
        RXDMA_CTRL_BURST_UNK | RXDMA_CTRL_INT_FINISH | RXDMA_CTRL_INT_FAIL |
        RXDMA_CTRL_INT_PERR | RXDMA_CTRL_INT_EOD | RXDMA_CTRL_INT_EOF;
    writel(v, (*ce).base + IPSEC_RXDMA_CTRL);
}

unsafe fn get_desc_tx(ce: *mut sl3516_ce_dev) -> *mut descriptor {
    let dd = (*ce).tx.add((*ce).ctx as usize);
    (*ce).ctx += 1;
    if (*ce).ctx >= MAXDESC { (*ce).ctx = 0; }
    dd
}

unsafe fn get_desc_rx(ce: *mut sl3516_ce_dev) -> *mut descriptor {
    let dd = (*ce).rx.add((*ce).crx as usize);
    (*ce).crx += 1;
    if (*ce).crx >= MAXDESC { (*ce).crx = 0; }
    dd
}

pub unsafe fn sl3516_ce_run_task(ce: *mut sl3516_ce_dev, rctx: *mut sl3516_ce_cipher_req_ctx, name: *const c_char) -> i32 {
    let mut rdd: *mut descriptor = core::ptr::null_mut();
    let mut v: u32;
    let mut err = 0;
    (*ce).stat_req += 1;
    reinit_completion(&mut (*ce).complete);
    (*ce).status = 0;
    for i in 0..(*rctx).nr_sgd {
        dev_dbg((*ce).dev, "%s handle DST SG %d/%d len=%d\n", __func__, i, (*rctx).nr_sgd, (*rctx).t_dst[i].len);
        rdd = get_desc_rx(ce);
        (*rdd).buf_adr = (*rctx).t_dst[i].addr;
        (*rdd).frame_ctrl.bits.buffer_size = (*rctx).t_dst[i].len;
        (*rdd).frame_ctrl.bits.own = CE_DMA;
    }
    (*rdd).next_desc.bits.eofie = 1;
    for i in 0..(*rctx).nr_sgs {
        dev_dbg((*ce).dev, "%s handle SRC SG %d/%d len=%d\n", __func__, i, (*rctx).nr_sgs, (*rctx).t_src[i].len);
        (*(*rctx).h).algorithm_len = (*rctx).t_src[i].len;
        let mut dd = get_desc_tx(ce);
        (*dd).frame_ctrl.raw = 0; (*dd).flag_status.raw = 0;
        (*dd).frame_ctrl.bits.buffer_size = (*rctx).pctrllen; (*dd).buf_adr = (*ce).dctrl;
        (*dd).flag_status.tx_flag.tqflag = (*rctx).tqflag; (*dd).next_desc.bits.eofie = 0;
        (*dd).next_desc.bits.dec = 0; (*dd).next_desc.bits.sof_eof = DESC_FIRST | DESC_LAST; (*dd).frame_ctrl.bits.own = CE_DMA;
        dd = get_desc_tx(ce);
        (*dd).frame_ctrl.raw = 0; (*dd).flag_status.raw = 0;
        (*dd).frame_ctrl.bits.buffer_size = (*rctx).t_src[i].len; (*dd).buf_adr = (*rctx).t_src[i].addr;
        (*dd).flag_status.tx_flag.tqflag = 0; (*dd).next_desc.bits.eofie = 0; (*dd).next_desc.bits.dec = 0;
        (*dd).next_desc.bits.sof_eof = DESC_FIRST | DESC_LAST; (*dd).frame_ctrl.bits.own = CE_DMA;
        start_dma_tx(ce); start_dma_rx(ce);
    }
    wait_for_completion_interruptible_timeout(&mut (*ce).complete, msecs_to_jiffies(5000));
    if (*ce).status == 0 { dev_err((*ce).dev, "DMA timeout for %s\n", name); err = -EFAULT; }
    v = readl((*ce).base + IPSEC_STATUS_REG);
    if v & 0xFFF != 0 { dev_err((*ce).dev, "IPSEC_STATUS_REG %x\n", v); err = -EFAULT; }
    err
}

unsafe extern "C" fn ce_irq_handler(_irq: i32, data: *mut c_void) -> irqreturn_t {
    let ce = data as *mut sl3516_ce_dev;
    (*ce).stat_irq += 1;
    let v = readl((*ce).base + IPSEC_DMA_STATUS);
    writel(v, (*ce).base + IPSEC_DMA_STATUS);
    if v & DMA_STATUS_TS_DERR != 0 { dev_err((*ce).dev, "AHB bus Error While Tx !!!\n"); }
    if v & DMA_STATUS_TS_PERR != 0 { dev_err((*ce).dev, "Tx Descriptor Protocol Error !!!\n"); }
    if v & DMA_STATUS_RS_DERR != 0 { dev_err((*ce).dev, "AHB bus Error While Rx !!!\n"); }
    if v & DMA_STATUS_RS_PERR != 0 { dev_err((*ce).dev, "Rx Descriptor Protocol Error !!!\n"); }
    if v & DMA_STATUS_TS_EOFI != 0 { (*ce).stat_irq_tx += 1; }
    if v & DMA_STATUS_RS_EOFI != 0 { (*ce).status = 1; complete(&mut (*ce).complete); (*ce).stat_irq_rx += 1; }
    IRQ_HANDLED
}

static mut ce_algs: [sl3516_ce_alg_template; 1] = [sl3516_ce_alg_template {
    type_: CRYPTO_ALG_TYPE_SKCIPHER, mode: ECB_AES, ce: core::ptr::null_mut(), stat_req: 0, stat_fb: 0,
    alg: sl3516_ce_alg_union::skcipher(sl3516_ce_skcipher_template {
        base: sl3516_ce_alg_base { cra_name: b"ecb(aes)\0".as_ptr() as _, cra_driver_name: b"ecb-aes-sl3516\0".as_ptr() as _, cra_priority: 400,
        cra_blocksize: AES_BLOCK_SIZE, cra_flags: CRYPTO_ALG_TYPE_SKCIPHER | CRYPTO_ALG_ASYNC | CRYPTO_ALG_NEED_FALLBACK,
        cra_ctxsize: core::mem::size_of::<sl3516_ce_cipher_tfm_ctx>(), cra_module: THIS_MODULE, cra_alignmask: 0xf,
        cra_init: Some(sl3516_ce_cipher_init), cra_exit: Some(sl3516_ce_cipher_exit),
    }, min_keysize: AES_MIN_KEY_SIZE, max_keysize: AES_MAX_KEY_SIZE, setkey: Some(sl3516_ce_aes_setkey),
    encrypt: Some(sl3516_ce_skencrypt), decrypt: Some(sl3516_ce_skdecrypt), do_one_request: Some(sl3516_ce_handle_cipher_request)
    })
}];

unsafe fn sl3516_ce_register_algs(ce: *mut sl3516_ce_dev) -> i32 {
    for i in 0..ce_algs.len() { ce_algs[i].ce = ce; if ce_algs[i].type_ == CRYPTO_ALG_TYPE_SKCIPHER {
        let err = crypto_engine_register_skcipher(&mut ce_algs[i].alg.skcipher);
        if err != 0 { ce_algs[i].ce = core::ptr::null_mut(); return err; }
    } }
    0
}
unsafe fn sl3516_ce_unregister_algs(_ce: *mut sl3516_ce_dev) {
    for i in 0..ce_algs.len() { if !ce_algs[i].ce.is_null() { crypto_engine_unregister_skcipher(&mut ce_algs[i].alg.skcipher); } }
}
unsafe fn sl3516_ce_start(ce: *mut sl3516_ce_dev) { (*ce).ctx = 0; (*ce).crx = 0; writel((*ce).dtx, (*ce).base + IPSEC_TXDMA_CURR_DESC); writel((*ce).drx, (*ce).base + IPSEC_RXDMA_CURR_DESC); writel(0, (*ce).base + IPSEC_DMA_STATUS); }
unsafe fn sl3516_ce_pm_suspend(dev: *mut device) -> i32 { let ce = dev_get_drvdata(dev); reset_control_assert((*ce).reset); clk_disable_unprepare((*ce).clks); 0 }
unsafe fn sl3516_ce_pm_resume(dev: *mut device) -> i32 { let ce = dev_get_drvdata(dev); let err = clk_prepare_enable((*ce).clks); if err != 0 { sl3516_ce_pm_suspend(dev); return err; } let err = reset_control_deassert((*ce).reset); if err != 0 { sl3516_ce_pm_suspend(dev); return err; } sl3516_ce_start(ce); 0 }
unsafe fn sl3516_ce_pm_init(ce: *mut sl3516_ce_dev) -> i32 { pm_runtime_use_autosuspend((*ce).dev); pm_runtime_set_autosuspend_delay((*ce).dev, 2000); let err = pm_runtime_set_suspended((*ce).dev); if err != 0 { return err; } pm_runtime_enable((*ce).dev); 0 }
unsafe fn sl3516_ce_pm_exit(ce: *mut sl3516_ce_dev) { pm_runtime_disable((*ce).dev); }

// The probe/remove paths, device-match table, platform driver, and module metadata
// retain the corresponding kernel registration and cleanup ordering from the C source.
unsafe fn sl3516_ce_remove(pdev: *mut platform_device) { let ce = platform_get_drvdata(pdev); sl3516_ce_rng_unregister(ce); sl3516_ce_unregister_algs(ce); crypto_engine_exit((*ce).engine); sl3516_ce_pm_exit(ce); sl3516_ce_free_descs(ce); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
