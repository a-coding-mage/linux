// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IDE tuning and bus mastering support for the CS5510/CS5520 chipsets.
 *
 * The CS5510/CS5520 are slightly unusual devices. Unlike typical IDE
 * controllers they do bus mastering with the drive in PIO mode and smarter
 * silicon. We must always tune the drive for the right PIO mode and ignore
 * the drive bus mastering DMA information. We can also do DMA on PIO-only
 * drives.
 *
 * DMA on the 5510 also requires disable_hlt() during DMA on early revisions.
 *
 * *** This driver is strictly experimental ***
 *
 * (c) Copyright Red Hat Inc 2002
 *
 * Documentation: Not publicly available.
 */
// C dependencies: linux/kernel.h, linux/module.h, linux/pci.h,
// linux/blkdev.h, linux/delay.h, scsi/scsi_host.h, linux/libata.h

const DRV_NAME: &str = "pata_cs5520";
const DRV_VERSION: &str = "0.6.6";

#[repr(C)]
struct PioClocks {
    address: i32,
    assert: i32,
    recovery: i32,
}

static CS5520_PIO_CLOCKS: [PioClocks; 5] = [
    PioClocks { address: 3, assert: 6, recovery: 11 },
    PioClocks { address: 2, assert: 5, recovery: 6 },
    PioClocks { address: 1, assert: 4, recovery: 3 },
    PioClocks { address: 1, assert: 3, recovery: 2 },
    PioClocks { address: 1, assert: 2, recovery: 1 },
];

/* cs5520_set_timings - program PIO timings */
unsafe fn cs5520_set_timings(ap: *mut ata_port, adev: *mut ata_device, mut pio: i32) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let slave = (*adev).devno;

    pio -= XFER_PIO_0;
    let timing = ((CS5520_PIO_CLOCKS[pio as usize].recovery << 4)
        | CS5520_PIO_CLOCKS[pio as usize].assert) as u8;

    /* Channel command timing */
    pci_write_config_byte(pdev, 0x62 + (*ap).port_no, timing);
    /* FIXME: should these use address ? */
    /* Read command timing */
    pci_write_config_byte(pdev, 0x64 + 4 * (*ap).port_no + slave, timing);
    /* Write command timing */
    pci_write_config_byte(pdev, 0x66 + 4 * (*ap).port_no + slave, timing);
}

/* cs5520_set_piomode - program PIO timings */
unsafe fn cs5520_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    cs5520_set_timings(ap, adev, (*adev).pio_mode);
}

static mut CS5520_SHT: scsi_host_template = scsi_host_template {
    // ATA_BASE_SHT(DRV_NAME)
    sg_tablesize: LIBATA_DUMB_MAX_PRD,
    dma_boundary: ATA_DMA_BOUNDARY,
    ..unsafe { core::mem::zeroed() }
};

static mut CS5520_PORT_OPS: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    qc_prep: Some(ata_bmdma_dumb_qc_prep),
    cable_detect: Some(ata_cable_40wire),
    set_piomode: Some(cs5520_set_piomode),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn cs5520_init_one(pdev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    let cmd_port: [u32; 2] = [0x1F0, 0x170];
    let ctl_port: [u32; 2] = [0x3F6, 0x376];
    let mut pi: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO4,
        port_ops: &CS5520_PORT_OPS,
        ..core::mem::zeroed()
    };
    let mut ppi: [*const ata_port_info; 2];
    let mut pcicfg: u8 = 0;
    let mut iomap: [*mut core::ffi::c_void; 5];
    let host: *mut ata_host;
    let mut ioaddr: *mut ata_ioports;
    let mut i: i32;
    let mut rc: i32;

    rc = pcim_enable_device(pdev);
    if rc != 0 { return rc; }

    pci_read_config_byte(pdev, 0x60, &mut pcicfg);
    if (pcicfg & 3) == 0 { return -ENODEV; }

    ppi = [&ata_dummy_port_info, &ata_dummy_port_info];
    if (pcicfg & 1) != 0 { ppi[0] = &pi; }
    if (pcicfg & 2) != 0 { ppi[1] = &pi; }

    if (pcicfg & 0x40) == 0 {
        dev_warn(&(*pdev).dev, "DMA mode disabled. Enabling.\n");
        pci_write_config_byte(pdev, 0x60, pcicfg | 0x40);
    }

    pi.mwdma_mask = (*id).driver_data;
    host = ata_host_alloc_pinfo(&(*pdev).dev, ppi.as_ptr(), 2);
    if host.is_null() { return -ENOMEM; }

    if dma_set_mask_and_coherent(&(*pdev).dev, DMA_BIT_MASK(32)) != 0 {
        dev_err(&(*pdev).dev, "unable to configure DMA mask.\n");
        return -ENODEV;
    }

    iomap[0] = devm_ioport_map(&(*pdev).dev, cmd_port[0], 8);
    iomap[1] = devm_ioport_map(&(*pdev).dev, ctl_port[0], 1);
    iomap[2] = devm_ioport_map(&(*pdev).dev, cmd_port[1], 8);
    iomap[3] = devm_ioport_map(&(*pdev).dev, ctl_port[1], 1);
    iomap[4] = pcim_iomap(pdev, 2, 0);
    if iomap.iter().any(|p| p.is_null()) { return -ENOMEM; }

    ioaddr = &mut (*(*host).ports[0]).ioaddr;
    (*ioaddr).cmd_addr = iomap[0]; (*ioaddr).ctl_addr = iomap[1];
    (*ioaddr).altstatus_addr = iomap[1]; (*ioaddr).bmdma_addr = iomap[4];
    ata_sff_std_ports(ioaddr);
    ata_port_desc((*host).ports[0], "cmd 0x%x ctl 0x%x", cmd_port[0], ctl_port[0]);
    ata_port_pbar_desc((*host).ports[0], 4, 0, "bmdma");

    ioaddr = &mut (*(*host).ports[1]).ioaddr;
    (*ioaddr).cmd_addr = iomap[2]; (*ioaddr).ctl_addr = iomap[3];
    (*ioaddr).altstatus_addr = iomap[3]; (*ioaddr).bmdma_addr = iomap[4].add(8);
    ata_sff_std_ports(ioaddr);
    ata_port_desc((*host).ports[1], "cmd 0x%x ctl 0x%x", cmd_port[1], ctl_port[1]);
    ata_port_pbar_desc((*host).ports[1], 4, 8, "bmdma");

    pci_set_master(pdev);
    rc = ata_host_start(host);
    if rc != 0 { return rc; }

    i = 0;
    while i < 2 {
        let irq: [i32; 2] = [14, 15];
        let ap = (*host).ports[i as usize];
        if !ata_port_is_dummy(ap) {
            rc = devm_request_irq(&(*pdev).dev, irq[(*ap).port_no as usize],
                Some(ata_bmdma_interrupt), 0, DRV_NAME, host);
            if rc != 0 { return rc; }
            ata_port_desc_misc(ap, irq[i as usize]);
        }
        i += 1;
    }
    ata_host_register(host, &CS5520_SHT)
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn cs5520_reinit_one(pdev: *mut pci_dev) -> i32 {
    let host = pci_get_drvdata(pdev);
    let mut pcicfg: u8 = 0;
    let rc = ata_pci_device_do_resume(pdev);
    if rc != 0 { return rc; }
    pci_read_config_byte(pdev, 0x60, &mut pcicfg);
    if (pcicfg & 0x40) == 0 { pci_write_config_byte(pdev, 0x60, pcicfg | 0x40); }
    ata_host_resume(host);
    0
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn cs5520_pci_device_suspend(pdev: *mut pci_dev, mesg: pm_message_t) -> i32 {
    let host = pci_get_drvdata(pdev);
    ata_host_suspend(host, mesg);
    pci_save_state(pdev);
    0
}

/* For now keep DMA off. We can set it for all but A rev CS5510 once the
 * core ATA code can handle it. */
static PATA_CS5520: [pci_device_id; 3] = [
    PCI_VDEVICE(CYRIX, PCI_DEVICE_ID_CYRIX_5510),
    PCI_VDEVICE(CYRIX, PCI_DEVICE_ID_CYRIX_5520),
    pci_device_id::default(),
];

static mut CS5520_PCI_DRIVER: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: &PATA_CS5520,
    probe: Some(cs5520_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)]
    suspend: Some(cs5520_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)]
    resume: Some(cs5520_reinit_one),
    ..unsafe { core::mem::zeroed() }
};

// module_pci_driver(CS5520_PCI_DRIVER);
// MODULE_AUTHOR("Alan Cox");
// MODULE_DESCRIPTION("low-level driver for Cyrix CS5510/5520");
// MODULE_LICENSE("GPL");
// MODULE_DEVICE_TABLE(pci, PATA_CS5520);
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
