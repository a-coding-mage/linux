// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata_cmd640.c 	- CMD640 PCI PATA for new ATA layer
 *			  (C) 2007 Red Hat Inc
 *
 * Based upon
 *  linux/drivers/ide/pci/cmd640.c		Version 1.02  Sep 01, 1996
 *
 *  Copyright (C) 1995-1996  Linus Torvalds & authors (see driver)
 *
 *	This drives only the PCI version of the controller. If you have a
 *	VLB one then we have enough docs to support it but you can write
 *	 your own code.
 */

// C header dependencies are supplied by the surrounding kernel bindings.

const DRV_NAME: &str = "pata_cmd640";
const DRV_VERSION: &str = "0.0.5";

#[repr(C)]
struct cmd640_reg {
    last: i32,
    reg58: [u8; ATA_MAX_DEVICES],
}

const CFR: u8 = 0x50;
const CNTRL: u8 = 0x51;
const CMDTIM: u8 = 0x52;
const ARTIM0: u8 = 0x53;
const DRWTIM0: u8 = 0x54;
const ARTIM23: u8 = 0x57;
const DRWTIM23: u8 = 0x58;
const BRST: u8 = 0x59;

/**
 * cmd640_set_piomode - set initial PIO mode data
 * @ap: ATA port
 * @adev: ATA device
 *
 * Called to do the PIO mode setup.
 */
unsafe fn cmd640_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let timing = (*ap).private_data as *mut cmd640_reg;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut t: ata_timing = core::mem::zeroed();
    const T: c_ulong = 1000000 / 33;
    let setup_data: [u8; 5] = [0x40, 0x40, 0x40, 0x40, 0x00];
    let mut reg: u8 = 0;
    let arttim: u8 = ARTIM0 + 2 * (*adev).devno;
    let pair = ata_dev_pair(adev);

    if ata_timing_compute(adev, (*adev).pio_mode, &mut t, T, 0) < 0 {
        ata_dev_err(adev, concat!(DRV_NAME, ": mode computation failed.\n"));
        return;
    }

    /* The second channel has shared timings and the setup timing is
       messy to switch to merge it for worst case */
    if (*ap).port_no != 0 && !pair.is_null() {
        let mut p: ata_timing = core::mem::zeroed();
        ata_timing_compute(pair, (*pair).pio_mode, &mut p, T, 1);
        ata_timing_merge(&p, &t, &mut t, ATA_TIMING_SETUP);
    }

    /* Make the timings fit */
    if t.recover > 16 { t.active += t.recover - 16; t.recover = 16; }
    if t.active > 16 { t.active = 16; }

    /* Now convert the clocks into values we can actually stuff into
       the chip */
    if t.recover > 1 { t.recover -= 1; /* 640B only */ }
    else { t.recover = 15; }
    if t.setup > 4 { t.setup = 0xC0; }
    else { t.setup = setup_data[t.setup as usize]; }

    if (*ap).port_no == 0 {
        t.active &= 0x0F; /* 0 = 16 */
        pci_read_config_byte(pdev, arttim, &mut reg);
        reg &= 0x3F;
        reg |= t.setup;
        pci_write_config_byte(pdev, arttim, reg);
        pci_write_config_byte(pdev, arttim + 1, (t.active << 4) | t.recover);
    } else {
        pci_read_config_byte(pdev, ARTIM23, &mut reg);
        reg &= 0x3F;
        reg |= t.setup;
        pci_write_config_byte(pdev, ARTIM23, reg);
        (*timing).reg58[(*adev).devno as usize] = (t.active << 4) | t.recover;
    }
}

/** Channel 1 has shared timings. Reprogram the clock on each drive switch. */
unsafe fn cmd640_qc_issue(qc: *mut ata_queued_cmd) -> c_uint {
    let ap = (*qc).ap;
    let adev = (*qc).dev;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let timing = (*ap).private_data as *mut cmd640_reg;
    if (*ap).port_no != 0 && (*adev).devno as i32 != (*timing).last {
        pci_write_config_byte(pdev, DRWTIM23, (*timing).reg58[(*adev).devno as usize]);
        (*timing).last = (*adev).devno as i32;
    }
    ata_sff_qc_issue(qc)
}

unsafe fn cmd640_port_start(ap: *mut ata_port) -> c_int {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let timing = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<cmd640_reg>(), GFP_KERNEL) as *mut cmd640_reg;
    if timing.is_null() { return -ENOMEM; }
    (*timing).last = -1;
    (*ap).private_data = timing as *mut core::ffi::c_void;
    0
}

unsafe fn cmd640_sff_irq_check(ap: *mut ata_port) -> bool {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let irq_reg = if (*ap).port_no != 0 { ARTIM23 } else { CFR };
    let mut irq_stat = 0u8;
    let irq_mask = if (*ap).port_no != 0 { 0x10 } else { 0x04 };
    pci_read_config_byte(pdev, irq_reg, &mut irq_stat);
    (irq_stat & irq_mask) != 0
}

static cmd640_sht: scsi_host_template = ATA_PIO_SHT!(DRV_NAME);

static mut cmd640_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_sff_port_ops,
    // In theory xfer_noirq is not needed once we kill the prefetcher
    sff_data_xfer: Some(ata_sff_data_xfer32),
    sff_irq_check: Some(cmd640_sff_irq_check),
    qc_issue: Some(cmd640_qc_issue),
    cable_detect: Some(ata_cable_40wire),
    set_piomode: Some(cmd640_set_piomode),
    port_start: Some(cmd640_port_start),
};

unsafe fn cmd640_hardware_init(pdev: *mut pci_dev) {
    let mut ctrl = 0u8;
    pci_write_config_byte(pdev, 0x5B, 0x00);
    pci_write_config_byte(pdev, CMDTIM, 0);
    pci_write_config_byte(pdev, BRST, 0x40);
    /* A reporter a long time ago had problems with the data fifo. */
    pci_read_config_byte(pdev, CNTRL, &mut ctrl);
    pci_write_config_byte(pdev, CNTRL, ctrl | 0xC0);
    pci_read_config_byte(pdev, ARTIM23, &mut ctrl);
    ctrl |= 0x0C;
    pci_write_config_byte(pdev, ARTIM23, ctrl);
}

unsafe fn cmd640_init_one(pdev: *mut pci_dev, _id: *const pci_device_id) -> c_int {
    static info: ata_port_info = ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, port_ops: &cmd640_port_ops };
    let ppi: [*const ata_port_info; 2] = [&info, core::ptr::null()];
    let rc = pcim_enable_device(pdev);
    if rc != 0 { return rc; }
    cmd640_hardware_init(pdev);
    ata_pci_sff_init_one(pdev, ppi.as_ptr(), &cmd640_sht, core::ptr::null_mut(), 0)
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn cmd640_reinit_one(pdev: *mut pci_dev) -> c_int {
    let host = pci_get_drvdata(pdev);
    let rc = ata_pci_device_do_resume(pdev);
    if rc != 0 { return rc; }
    cmd640_hardware_init(pdev);
    ata_host_resume(host);
    0
}

static mut cmd640: [pci_device_id; 2] = [PCI_VDEVICE!(CMD, 0x0640), pci_device_id {}];

static mut cmd640_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: cmd640.as_ptr(),
    probe: Some(cmd640_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)]
    suspend: Some(ata_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)]
    resume: Some(cmd640_reinit_one),
};

// module_pci_driver(cmd640_pci_driver);
// MODULE_AUTHOR("Alan Cox");
// MODULE_DESCRIPTION("low-level driver for CMD640 PATA controllers");
// MODULE_LICENSE("GPL");
// MODULE_DEVICE_TABLE(pci, cmd640);
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
