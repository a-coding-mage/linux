// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2013-2014 Freescale Semiconductor, Inc
// Copyright (c) 2017 Sysam, Angelo Dureghello  <angelo@sysam.it>
//
// Linux kernel dependencies supplied by the surrounding tree are intentionally
// referenced but not reimplemented here.

const EDMA_CHANNELS: usize = 64;

#[inline]
fn edma_mask_ch(x: u32) -> u8 {
    (x & ((1u32 << 6) - 1)) as u8
}

unsafe fn mcf_edma_tx_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mcf_edma = dev_id as *mut fsl_edma_engine;
    let regs = &mut (*mcf_edma).regs;
    let mut intmap: u64;

    intmap = ioread32(regs.inth as *const core::ffi::c_void) as u64;
    intmap <<= 32;
    intmap |= ioread32(regs.intl as *const core::ffi::c_void) as u64;
    if intmap == 0 {
        return IRQ_NONE;
    }

    for ch in 0..(*mcf_edma).n_chans {
        if (intmap & (1u64 << ch)) != 0 {
            iowrite8(edma_mask_ch(ch as u32), regs.cint as *mut core::ffi::c_void);
            fsl_edma_tx_chan_handler(&mut (*mcf_edma).chans[ch]);
        }
    }

    IRQ_HANDLED
}

unsafe fn mcf_edma_err_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mcf_edma = dev_id as *mut fsl_edma_engine;
    let regs = &mut (*mcf_edma).regs;
    let mut err: u32;

    err = ioread32(regs.errl as *const core::ffi::c_void);
    if err == 0 {
        return IRQ_NONE;
    }

    for ch in 0..(EDMA_CHANNELS / 2) {
        if (err & (1u32 << ch)) != 0 {
            fsl_edma_disable_request(&mut (*mcf_edma).chans[ch]);
            iowrite8(EDMA_CERR_CERR(ch as u32), regs.cerr as *mut core::ffi::c_void);
            fsl_edma_err_chan_handler(&mut (*mcf_edma).chans[ch]);
        }
    }

    err = ioread32(regs.errh as *const core::ffi::c_void);
    if err == 0 {
        return IRQ_NONE;
    }

    for ch in (EDMA_CHANNELS / 2)..EDMA_CHANNELS {
        if (err & (1u32 << (ch - (EDMA_CHANNELS / 2)))) != 0 {
            fsl_edma_disable_request(&mut (*mcf_edma).chans[ch]);
            iowrite8(EDMA_CERR_CERR(ch as u32), regs.cerr as *mut core::ffi::c_void);
            (*mcf_edma).chans[ch].status = DMA_ERROR;
        }
    }

    IRQ_HANDLED
}

unsafe fn mcf_edma_irq_init(pdev: *mut platform_device, mcf_edma: *mut fsl_edma_engine) -> i32 {
    let mut ret = 0;
    let mut res = platform_get_resource_byname(pdev, IORESOURCE_IRQ, c"edma-tx-00-15".as_ptr());
    if res.is_null() { return -1; }
    let mut i = (*res).start;
    while i <= (*res).end { ret |= request_irq(i, mcf_edma_tx_handler, 0, c"eDMA".as_ptr(), mcf_edma as *mut _); i += 1; }
    if ret != 0 { return ret; }
    res = platform_get_resource_byname(pdev, IORESOURCE_IRQ, c"edma-tx-16-55".as_ptr());
    if res.is_null() { return -1; }
    i = (*res).start;
    while i <= (*res).end { ret |= request_irq(i, mcf_edma_tx_handler, 0, c"eDMA".as_ptr(), mcf_edma as *mut _); i += 1; }
    if ret != 0 { return ret; }
    ret = platform_get_irq_byname(pdev, c"edma-tx-56-63".as_ptr());
    if ret != -ENXIO { ret = request_irq(ret, mcf_edma_tx_handler, 0, c"eDMA".as_ptr(), mcf_edma as *mut _); if ret != 0 { return ret; } }
    ret = platform_get_irq_byname(pdev, c"edma-err".as_ptr());
    if ret != -ENXIO { ret = request_irq(ret, mcf_edma_err_handler, 0, c"eDMA".as_ptr(), mcf_edma as *mut _); if ret != 0 { return ret; } }
    0
}

unsafe fn mcf_edma_irq_free(pdev: *mut platform_device, mcf_edma: *mut fsl_edma_engine) {
    let mut res = platform_get_resource_byname(pdev, IORESOURCE_IRQ, c"edma-tx-00-15".as_ptr());
    if !res.is_null() { let mut irq = (*res).start; while irq <= (*res).end { free_irq(irq, mcf_edma as *mut _); irq += 1; } }
    res = platform_get_resource_byname(pdev, IORESOURCE_IRQ, c"edma-tx-16-55".as_ptr());
    if !res.is_null() { let mut irq = (*res).start; while irq <= (*res).end { free_irq(irq, mcf_edma as *mut _); irq += 1; } }
    let irq = platform_get_irq_byname(pdev, c"edma-tx-56-63".as_ptr()); if irq != -ENXIO { free_irq(irq, mcf_edma as *mut _); }
    let irq = platform_get_irq_byname(pdev, c"edma-err".as_ptr()); if irq != -ENXIO { free_irq(irq, mcf_edma as *mut _); }
}

static mut MCF_DATA: fsl_edma_drvdata = fsl_edma_drvdata { flags: FSL_EDMA_DRV_EDMA64, setup_irq: Some(mcf_edma_irq_init) };

unsafe fn mcf_edma_probe(pdev: *mut platform_device) -> i32 {
    let pdata = dev_get_platdata(&mut (*pdev).dev);
    if pdata.is_null() { dev_err(&(*pdev).dev, c"no platform data supplied\n".as_ptr()); return -EINVAL; }
    let chans = if (*pdata).dma_channels == 0 { dev_info(&(*pdev).dev, c"setting default channel number to 64".as_ptr()); 64 } else { (*pdata).dma_channels };
    let mcf_edma = devm_kzalloc(&mut (*pdev).dev, struct_size::<fsl_edma_engine>(chans), GFP_KERNEL) as *mut fsl_edma_engine;
    if mcf_edma.is_null() { return -ENOMEM; }
    (*mcf_edma).n_chans = chans; (*mcf_edma).drvdata = &mut MCF_DATA; (*mcf_edma).big_endian = 1;
    mutex_init(&mut (*mcf_edma).fsl_edma_mutex);
    (*mcf_edma).membase = devm_platform_ioremap_resource(pdev, 0); if IS_ERR((*mcf_edma).membase) { return PTR_ERR((*mcf_edma).membase); }
    fsl_edma_setup_regs(mcf_edma);
    let regs = &mut (*mcf_edma).regs;
    INIT_LIST_HEAD(&mut (*mcf_edma).dma_dev.channels);
    for i in 0..(*mcf_edma).n_chans { let chan = &mut (*mcf_edma).chans[i]; chan.edma = mcf_edma; chan.srcid = i; chan.dma_dir = DMA_NONE; chan.vchan.desc_free = Some(fsl_edma_free_desc); vchan_init(&mut chan.vchan, &mut (*mcf_edma).dma_dev); chan.tcd = (*mcf_edma).membase.add(EDMA_TCD + i * core::mem::size_of::<fsl_edma_hw_tcd>()); edma_write_tcdreg(chan, cpu_to_le32(0), csr); }
    iowrite32(!0, regs.inth as *mut _); iowrite32(!0, regs.intl as *mut _);
    let ret = ((*mcf_edma).drvdata.setup_irq.unwrap())(pdev, mcf_edma); if ret != 0 { return ret; }
    dma_cap_set(DMA_PRIVATE, &mut (*mcf_edma).dma_dev.cap_mask); dma_cap_set(DMA_SLAVE, &mut (*mcf_edma).dma_dev.cap_mask); dma_cap_set(DMA_CYCLIC, &mut (*mcf_edma).dma_dev.cap_mask);
    (*mcf_edma).dma_dev.dev = &mut (*pdev).dev;
    (*mcf_edma).dma_dev.device_alloc_chan_resources = Some(fsl_edma_alloc_chan_resources); (*mcf_edma).dma_dev.device_free_chan_resources = Some(fsl_edma_free_chan_resources); (*mcf_edma).dma_dev.device_config = Some(fsl_edma_slave_config); (*mcf_edma).dma_dev.device_prep_dma_cyclic = Some(fsl_edma_prep_dma_cyclic); (*mcf_edma).dma_dev.device_prep_slave_sg = Some(fsl_edma_prep_slave_sg); (*mcf_edma).dma_dev.device_tx_status = Some(fsl_edma_tx_status); (*mcf_edma).dma_dev.device_pause = Some(fsl_edma_pause); (*mcf_edma).dma_dev.device_resume = Some(fsl_edma_resume); (*mcf_edma).dma_dev.device_terminate_all = Some(fsl_edma_terminate_all); (*mcf_edma).dma_dev.device_issue_pending = Some(fsl_edma_issue_pending);
    (*mcf_edma).dma_dev.src_addr_widths = FSL_EDMA_BUSWIDTHS; (*mcf_edma).dma_dev.dst_addr_widths = FSL_EDMA_BUSWIDTHS; (*mcf_edma).dma_dev.directions = (1 << DMA_DEV_TO_MEM) | (1 << DMA_MEM_TO_DEV);
    (*mcf_edma).dma_dev.filter.fn = Some(mcf_edma_filter_fn); (*mcf_edma).dma_dev.filter.map = (*pdata).slave_map; (*mcf_edma).dma_dev.filter.mapcnt = (*pdata).slavecnt;
    platform_set_drvdata(pdev, mcf_edma as *mut _); let ret = dma_async_device_register(&mut (*mcf_edma).dma_dev); if ret != 0 { dev_err(&(*pdev).dev, c"Can't register Freescale eDMA engine. (%d)\n".as_ptr(), ret); return ret; }
    iowrite32(EDMA_CR_ERGA | EDMA_CR_ERCA, regs.cr as *mut _); 0
}

unsafe fn mcf_edma_remove(pdev: *mut platform_device) { let mcf_edma = platform_get_drvdata(pdev) as *mut fsl_edma_engine; mcf_edma_irq_free(pdev, mcf_edma); fsl_edma_cleanup_vchan(&mut (*mcf_edma).dma_dev); dma_async_device_unregister(&mut (*mcf_edma).dma_dev); }

unsafe fn mcf_edma_filter_fn(chan: *mut dma_chan, param: *mut core::ffi::c_void) -> bool { if (*(*chan).device).dev.driver == &mut MCF_EDMA_DRIVER.driver { let mcf_chan = to_fsl_edma_chan(chan); return (*mcf_chan).srcid == param as usize; } false }

static mut MCF_EDMA_DRIVER: platform_driver = platform_driver { driver: driver { name: c"mcf-edma".as_ptr() }, probe: Some(mcf_edma_probe), remove: Some(mcf_edma_remove) };

unsafe fn mcf_edma_init() -> i32 { platform_driver_register(&mut MCF_EDMA_DRIVER) }
unsafe fn mcf_edma_exit() { platform_driver_unregister(&mut MCF_EDMA_DRIVER); }

// EXPORT_SYMBOL(mcf_edma_filter_fn);
// subsys_initcall(mcf_edma_init);
// module_exit(mcf_edma_exit);
// MODULE_ALIAS("platform:mcf-edma");
// MODULE_DESCRIPTION("Freescale eDMA engine driver, ColdFire family");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
