// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Generic PXA PATA driver
 *
 * Copyright (C) 2010 Marek Vasut <marek.vasut@gmail.com>
 */

// Linux kernel dependencies supplied by the surrounding translation.

const DRV_NAME: &str = "pata_pxa";
const DRV_VERSION: &str = "0.1";

#[repr(C)]
struct PataPxaData {
    dma_chan: *mut dma_chan,
    dma_cookie: dma_cookie_t,
    dma_done: completion,
}

/* DMA interrupt handler. */
unsafe extern "C" fn pxa_ata_dma_irq(d: *mut c_void) {
    let pd = d as *mut PataPxaData;
    let status: dma_status = dmaengine_tx_status((*pd).dma_chan, (*pd).dma_cookie, core::ptr::null_mut());

    if status == DMA_ERROR || status == DMA_COMPLETE {
        complete(&mut (*pd).dma_done);
    }
}

/* Prepare taskfile for submission. */
unsafe extern "C" fn pxa_qc_prep(qc: *mut ata_queued_cmd) -> ata_completion_errors {
    let pd = (*(*qc).ap).private_data as *mut PataPxaData;
    let mut tx: *mut dma_async_tx_descriptor;
    let dir: dma_transfer_direction;

    if (*qc).flags & ATA_QCFLAG_DMAMAP == 0 {
        return AC_ERR_OK;
    }

    dir = if (*qc).dma_dir == DMA_TO_DEVICE { DMA_MEM_TO_DEV } else { DMA_DEV_TO_MEM };
    tx = dmaengine_prep_slave_sg((*pd).dma_chan, (*qc).sg, (*qc).n_elem, dir, DMA_PREP_INTERRUPT);
    if tx.is_null() {
        ata_dev_err((*qc).dev, "prep_slave_sg() failed\n");
        return AC_ERR_OK;
    }
    (*tx).callback = Some(pxa_ata_dma_irq);
    (*tx).callback_param = pd as *mut c_void;
    (*pd).dma_cookie = dmaengine_submit(tx);

    AC_ERR_OK
}

/*
 * Configure the DMA controller, load the DMA descriptors, but don't start the
 * DMA controller yet. Only issue the ATA command.
 */
unsafe extern "C" fn pxa_bmdma_setup(qc: *mut ata_queued_cmd) {
    ((*(*qc).ap).ops).sff_exec_command.unwrap()((*qc).ap, &(*qc).tf);
}

/* Execute the DMA transfer. */
unsafe extern "C" fn pxa_bmdma_start(qc: *mut ata_queued_cmd) {
    let pd = (*(*qc).ap).private_data as *mut PataPxaData;
    init_completion(&mut (*pd).dma_done);
    dma_async_issue_pending((*pd).dma_chan);
}

/* Wait until the DMA transfer completes, then stop the DMA controller. */
unsafe extern "C" fn pxa_bmdma_stop(qc: *mut ata_queued_cmd) {
    let pd = (*(*qc).ap).private_data as *mut PataPxaData;
    let status = dmaengine_tx_status((*pd).dma_chan, (*pd).dma_cookie, core::ptr::null_mut());
    if status != DMA_ERROR && status != DMA_COMPLETE && wait_for_completion_timeout(&mut (*pd).dma_done, HZ) != 0 {
        ata_dev_err((*qc).dev, "Timeout waiting for DMA completion!");
    }
    dmaengine_terminate_all((*pd).dma_chan);
}

/* Read DMA status. */
unsafe extern "C" fn pxa_bmdma_status(ap: *mut ata_port) -> u8 {
    let pd = (*ap).private_data as *mut PataPxaData;
    let mut ret: u8 = ATA_DMA_INTR;
    let mut state: dma_tx_state = core::mem::zeroed();
    let status = dmaengine_tx_status((*pd).dma_chan, (*pd).dma_cookie, &mut state);
    if status != DMA_COMPLETE {
        ret |= ATA_DMA_ERR;
    }
    ret
}

/* No IRQ register present so we do nothing. */
unsafe extern "C" fn pxa_irq_clear(_ap: *mut ata_port) {}

/* ATAPI DMA is unsupported by this driver. */
unsafe extern "C" fn pxa_check_atapi_dma(_qc: *mut ata_queued_cmd) -> i32 {
    -EOPNOTSUPP
}

static mut pxa_ata_sht: scsi_host_template = scsi_host_template { /* ATA_BMDMA_SHT(DRV_NAME) */ };

static mut pxa_ata_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    cable_detect: Some(ata_cable_40wire),
    bmdma_setup: Some(pxa_bmdma_setup),
    bmdma_start: Some(pxa_bmdma_start),
    bmdma_stop: Some(pxa_bmdma_stop),
    bmdma_status: Some(pxa_bmdma_status),
    check_atapi_dma: Some(pxa_check_atapi_dma),
    sff_irq_clear: Some(pxa_irq_clear),
    qc_prep: Some(pxa_qc_prep),
};

unsafe extern "C" fn pxa_ata_probe(pdev: *mut platform_device) -> i32 {
    let mut host: *mut ata_host;
    let ap: *mut ata_port;
    let data: *mut PataPxaData;
    let dma_res: *mut resource;
    let pdata: *mut pata_pxa_pdata = dev_get_platdata(&mut (*pdev).dev);
    let mut config: dma_slave_config = core::mem::zeroed();
    let mut ret: i32 = 0;
    let irq: i32;

    if (*pdev).num_resources != 4 {
        dev_err(&mut (*pdev).dev, "invalid number of resources\n");
        return -EINVAL;
    }
    dma_res = platform_get_resource(pdev, IORESOURCE_DMA, 0);
    if dma_res.is_null() { return -EINVAL; }
    irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    host = ata_host_alloc(&mut (*pdev).dev, 1);
    if host.is_null() { return -ENOMEM; }
    ap = *(*host).ports;
    (*ap).ops = &pxa_ata_port_ops;
    (*ap).pio_mask = ATA_PIO4;
    (*ap).mwdma_mask = ATA_MWDMA2;
    (*ap).ioaddr.cmd_addr = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*ap).ioaddr.cmd_addr) { return PTR_ERR((*ap).ioaddr.cmd_addr); }
    (*ap).ioaddr.ctl_addr = devm_platform_ioremap_resource(pdev, 1);
    if IS_ERR((*ap).ioaddr.ctl_addr) { return PTR_ERR((*ap).ioaddr.ctl_addr); }
    (*ap).ioaddr.bmdma_addr = devm_ioremap(&mut (*pdev).dev, (*dma_res).start, resource_size(dma_res));
    if (*ap).ioaddr.bmdma_addr.is_null() { return -ENOMEM; }

    (*ap).ioaddr.altstatus_addr = (*ap).ioaddr.ctl_addr;
    (*ap).ioaddr.data_addr = (*ap).ioaddr.cmd_addr.add(ATA_REG_DATA << (*pdata).reg_shift);
    (*ap).ioaddr.error_addr = (*ap).ioaddr.cmd_addr.add(ATA_REG_ERR << (*pdata).reg_shift);
    (*ap).ioaddr.feature_addr = (*ap).ioaddr.cmd_addr.add(ATA_REG_FEATURE << (*pdata).reg_shift);
    (*ap).ioaddr.nsect_addr = (*ap).ioaddr.cmd_addr.add(ATA_REG_NSECT << (*pdata).reg_shift);
    (*ap).ioaddr.lbal_addr = (*ap).ioaddr.cmd_addr.add(ATA_REG_LBAL << (*pdata).reg_shift);
    (*ap).ioaddr.lbam_addr = (*ap).ioaddr.cmd_addr.add(ATA_REG_LBAM << (*pdata).reg_shift);
    (*ap).ioaddr.lbah_addr = (*ap).ioaddr.cmd_addr.add(ATA_REG_LBAH << (*pdata).reg_shift);
    (*ap).ioaddr.device_addr = (*ap).ioaddr.cmd_addr.add(ATA_REG_DEVICE << (*pdata).reg_shift);
    (*ap).ioaddr.status_addr = (*ap).ioaddr.cmd_addr.add(ATA_REG_STATUS << (*pdata).reg_shift);
    (*ap).ioaddr.command_addr = (*ap).ioaddr.cmd_addr.add(ATA_REG_CMD << (*pdata).reg_shift);

    data = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<PataPxaData>(), GFP_KERNEL) as *mut PataPxaData;
    if data.is_null() { return -ENOMEM; }
    (*ap).private_data = data as *mut c_void;
    config.src_addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    config.dst_addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    config.src_addr = (*dma_res).start;
    config.dst_addr = (*dma_res).start;
    config.src_maxburst = 32;
    config.dst_maxburst = 32;
    (*data).dma_chan = devm_dma_request_chan(&mut (*pdev).dev, "data\0".as_ptr() as *const i8);
    if IS_ERR((*data).dma_chan) { return PTR_ERR((*data).dma_chan); }
    ret = dmaengine_slave_config((*data).dma_chan, &config);
    if ret < 0 { dev_err(&mut (*pdev).dev, "dma configuration failed: %d\n", ret); return ret; }
    ata_host_activate(host, irq, Some(ata_sff_interrupt), (*pdata).irq_flags, &pxa_ata_sht)
}

static mut pxa_ata_driver: platform_driver = platform_driver {
    probe: Some(pxa_ata_probe),
    remove: Some(ata_platform_remove_one),
    driver: driver { name: DRV_NAME },
};

// Equivalent of module_platform_driver(pxa_ata_driver).
module_platform_driver!(pxa_ata_driver);
module_metadata!(AUTHOR = "Marek Vasut <marek.vasut@gmail.com>", DESCRIPTION = "DMA-capable driver for PATA on PXA CPU", LICENSE = "GPL", VERSION = DRV_VERSION, ALIAS = "platform:" DRV_NAME);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
