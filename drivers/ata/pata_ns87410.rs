// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * pata_ns87410.c - National Semiconductor 87410 PATA for new ATA layer
 *                 (C) 2006 Red Hat Inc
 *
 * Rust translation of the implementation source. Kernel declarations and
 * constants referenced below are supplied by the surrounding kernel bindings.
 */

const DRV_NAME: &str = "pata_ns87410";
const DRV_VERSION: &str = "0.4.6";

unsafe fn ns87410_pre_reset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);
    static NS87410_ENABLE_BITS: [pci_bits; 2] = [
        pci_bits { reg: 0x43, width: 1, mask: 0x08, val: 0x08 },
        pci_bits { reg: 0x47, width: 1, mask: 0x08, val: 0x08 },
    ];

    if !pci_test_config_bits(pdev, &NS87410_ENABLE_BITS[(*ap).port_no as usize]) {
        return -ENOENT;
    }

    ata_sff_prereset(link, deadline)
}

unsafe fn ns87410_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let port: c_int = 0x40 + 4 * (*ap).port_no as c_int;
    let mut idetcr: u8;
    let mut idefr: u8 = 0;
    let mut at: ata_timing = core::mem::zeroed();

    static ACTIVEBITS: [u8; 15] = [0, 1, 2, 3, 4, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7];
    static RECOVERBITS: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 6, 7, 7, 7, 7];

    pci_read_config_byte(pdev, port + 3, &mut idefr);

    if ata_pio_need_iordy(adev) {
        idefr |= 0x04; // IORDY enable
    } else {
        idefr &= !0x04;
    }

    if ata_timing_compute(adev, (*adev).pio_mode, &mut at, 30303, 1) < 0 {
        dev_err(&(*pdev).dev, "unknown mode %d\n", (*adev).pio_mode);
        return;
    }

    at.active = clamp_val(at.active, 2, 16) - 2;
    at.setup = clamp_val(at.setup, 1, 4) - 1;
    at.recover = clamp_val(at.recover, 1, 12) - 1;

    idetcr = ((at.setup << 6)
        | (RECOVERBITS[at.recover as usize] << 3)
        | ACTIVEBITS[at.active as usize]) as u8;

    pci_write_config_byte(pdev, port, idetcr);
    pci_write_config_byte(pdev, port + 3, idefr);
    // ap->private_data points to the device currently loaded for timing.
    (*ap).private_data = adev as *mut c_void;
}

unsafe fn ns87410_qc_issue(qc: *mut ata_queued_cmd) -> c_uint {
    let ap = (*qc).ap;
    let adev = (*qc).dev;

    // Load channel timing when the configured device is not already loaded.
    if (*adev).pio_mode != 0 && (adev as *mut c_void) != (*ap).private_data {
        ns87410_set_piomode(ap, adev);
    }

    ata_sff_qc_issue(qc)
}

static mut ns87410_sht: scsi_host_template = scsi_host_template {
    // ATA_PIO_SHT(DRV_NAME)
    ..core::mem::zeroed()
};

static mut ns87410_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_sff_port_ops,
    qc_issue: Some(ns87410_qc_issue),
    cable_detect: Some(ata_cable_40wire),
    set_piomode: Some(ns87410_set_piomode),
    reset: ata_port_operations_reset {
        prereset: Some(ns87410_pre_reset),
        ..core::mem::zeroed()
    },
    ..core::mem::zeroed()
};

unsafe fn ns87410_init_one(dev: *mut pci_dev, _id: *const pci_device_id) -> c_int {
    static mut INFO: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO3,
        port_ops: &ns87410_port_ops,
        ..core::mem::zeroed()
    };
    let ppi: [*const ata_port_info; 2] = [&INFO, core::ptr::null()];
    ata_pci_sff_init_one(dev, ppi.as_ptr(), &ns87410_sht, core::ptr::null_mut(), 0)
}

static mut ns87410: [pci_device_id; 2] = [
    pci_device_id { vendor: PCI_VENDOR_ID_NS, device: PCI_DEVICE_ID_NS_87410, ..core::mem::zeroed() },
    pci_device_id { ..core::mem::zeroed() },
];

static mut ns87410_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: ns87410.as_ptr(),
    probe: Some(ns87410_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)]
    suspend: Some(ata_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)]
    resume: Some(ata_pci_device_resume),
    ..core::mem::zeroed()
};

// module_pci_driver(ns87410_pci_driver);
// MODULE_AUTHOR("Alan Cox");
// MODULE_DESCRIPTION("low-level driver for Nat Semi 87410");
// MODULE_LICENSE("GPL");
// MODULE_DEVICE_TABLE(pci, ns87410);
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
