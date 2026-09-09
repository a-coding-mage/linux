// SPDX-License-Identifier: GPL-2.0-only
/*
 * ACPI PATA driver
 *
 * (c) 2007 Red Hat
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const DRV_NAME: &str = "pata_acpi";
const DRV_VERSION: &str = "0.2.3";

#[repr(C)]
struct pata_acpi {
    gtm: ata_acpi_gtm,
    last: *mut core::ffi::c_void,
    mask: [c_ulong; 2],
}

/** pacpi_pre_reset - check for 40/80 pin */
unsafe fn pacpi_pre_reset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    let ap = (*link).ap;
    let acpi = (*ap).private_data as *mut pata_acpi;
    if ACPI_HANDLE(&mut (*ap).tdev) == core::ptr::null_mut()
        || ata_acpi_gtm(ap, &mut (*acpi).gtm) < 0
    {
        return -ENODEV;
    }
    ata_sff_prereset(link, deadline)
}

/** pacpi_cable_detect - cable type detection */
unsafe fn pacpi_cable_detect(ap: *mut ata_port) -> c_int {
    let acpi = (*ap).private_data as *mut pata_acpi;
    if ((*acpi).mask[0] | (*acpi).mask[1]) & ((0xF8 as c_ulong) << ATA_SHIFT_UDMA) != 0 {
        ATA_CBL_PATA80
    } else {
        ATA_CBL_PATA40
    }
}

/** pacpi_discover_modes - filter non ACPI modes */
unsafe fn pacpi_discover_modes(ap: *mut ata_port, adev: *mut ata_device) -> c_ulong {
    let acpi = (*ap).private_data as *mut pata_acpi;
    let mut probe = (*acpi).gtm;
    ata_acpi_gtm(ap, &mut probe);
    let xfer_mask = ata_acpi_gtm_xfermask(adev, &mut probe);
    if xfer_mask & ((0xF8 as c_uint) << ATA_SHIFT_UDMA) != 0 {
        (*ap).cbl = ATA_CBL_PATA80;
    }
    xfer_mask as c_ulong
}

/** pacpi_mode_filter - mode filter for ACPI */
unsafe fn pacpi_mode_filter(adev: *mut ata_device, mask: c_uint) -> c_uint {
    let acpi = (*(*adev).link).ap.private_data as *mut pata_acpi;
    mask & (*acpi).mask[(*adev).devno as usize] as c_uint
}

/** pacpi_set_piomode - set initial PIO mode data */
unsafe fn pacpi_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let mut unit = (*adev).devno;
    let acpi = (*ap).private_data as *mut pata_acpi;
    if (*acpi).gtm.flags & 0x10 == 0 { unit = 0; }
    let t = ata_timing_find_mode((*adev).pio_mode);
    (*acpi).gtm.drive[unit as usize].pio = (*t).cycle;
    ata_acpi_stm(ap, &mut (*acpi).gtm);
    ata_acpi_gtm(ap, &mut (*acpi).gtm);
}

/** pacpi_set_dmamode - set initial DMA mode data */
unsafe fn pacpi_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let mut unit = (*adev).devno;
    let acpi = (*ap).private_data as *mut pata_acpi;
    if (*acpi).gtm.flags & 0x10 == 0 { unit = 0; }
    let t = ata_timing_find_mode((*adev).dma_mode);
    if (*adev).dma_mode >= XFER_UDMA_0 {
        (*acpi).gtm.drive[unit as usize].dma = (*t).udma;
        (*acpi).gtm.flags |= 1 << (2 * unit);
    } else {
        (*acpi).gtm.drive[unit as usize].dma = (*t).cycle;
        (*acpi).gtm.flags &= !(1 << (2 * unit));
    }
    ata_acpi_stm(ap, &mut (*acpi).gtm);
    ata_acpi_gtm(ap, &mut (*acpi).gtm);
}

/** pacpi_qc_issue - command issue */
unsafe fn pacpi_qc_issue(qc: *mut ata_queued_cmd) -> c_uint {
    let ap = (*qc).ap;
    let adev = (*qc).dev;
    let acpi = (*ap).private_data as *mut pata_acpi;
    if (*acpi).gtm.flags & 0x10 != 0 { return ata_bmdma_qc_issue(qc); }
    if (*adev as *mut core::ffi::c_void) != (*acpi).last {
        pacpi_set_piomode(ap, adev);
        if ata_dma_enabled(adev) { pacpi_set_dmamode(ap, adev); }
        (*acpi).last = adev as *mut core::ffi::c_void;
    }
    ata_bmdma_qc_issue(qc)
}

/** pacpi_port_start - port setup */
unsafe fn pacpi_port_start(ap: *mut ata_port) -> c_int {
    let pdev = to_pci_dev((*(*ap).host).dev);
    if ACPI_HANDLE(&mut (*ap).tdev) == core::ptr::null_mut() { return -ENODEV; }
    let acpi = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<pata_acpi>(), GFP_KERNEL)
        as *mut pata_acpi;
    (*ap).private_data = acpi as *mut core::ffi::c_void;
    if (*ap).private_data == core::ptr::null_mut() { return -ENOMEM; }
    (*acpi).mask[0] = pacpi_discover_modes(ap, &mut (*ap).link.device[0]);
    (*acpi).mask[1] = pacpi_discover_modes(ap, &mut (*ap).link.device[1]);
    ata_bmdma_port_start(ap)
}

static pacpi_sht: scsi_host_template = scsi_host_template { /* ATA_BMDMA_SHT(DRV_NAME) */ };

static mut pacpi_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    qc_issue: Some(pacpi_qc_issue), cable_detect: Some(pacpi_cable_detect),
    mode_filter: Some(pacpi_mode_filter), set_piomode: Some(pacpi_set_piomode),
    set_dmamode: Some(pacpi_set_dmamode), reset: ata_reset_operations { prereset: Some(pacpi_pre_reset) },
    port_start: Some(pacpi_port_start),
};

unsafe fn pacpi_init_one(pdev: *mut pci_dev, _id: *const pci_device_id) -> c_int {
    static info: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2,
        udma_mask: ATA_UDMA6, port_ops: &pacpi_ops,
    };
    let ppi = [ &info as *const ata_port_info, core::ptr::null() ];
    if (*pdev).vendor == PCI_VENDOR_ID_ATI {
        let rc = pcim_enable_device(pdev); if rc < 0 { return rc; }
        pcim_pin_device(pdev);
    }
    ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &pacpi_sht, core::ptr::null_mut(), 0)
}

static pacpi_pci_tbl: [pci_device_id; 2] = [
    pci_device_id { /* PCI_DEVICE_CLASS(PCI_CLASS_STORAGE_IDE << 8, 0xFFFFFF00UL) */ driver_data: 1 },
    pci_device_id { driver_data: 0 },
];

static mut pacpi_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME, id_table: pacpi_pci_tbl.as_ptr(), probe: Some(pacpi_init_one),
    remove: Some(ata_pci_remove_one),
};

// module_pci_driver(pacpi_pci_driver);
// MODULE_AUTHOR("Alan Cox"); MODULE_DESCRIPTION("SCSI low-level driver for ATA in ACPI mode");
// MODULE_LICENSE("GPL"); MODULE_DEVICE_TABLE(pci, pacpi_pci_tbl); MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
