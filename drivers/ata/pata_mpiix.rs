// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata_mpiix.c 	- Intel MPIIX PATA for new ATA layer
 *			  (C) 2005-2006 Red Hat Inc
 *			  Alan Cox <alan@lxorguk.ukuu.org.uk>
 *
 * The MPIIX is different enough to the PIIX4 and friends that we give it
 * a separate driver. The old ide/pci code handles this by just not tuning
 * MPIIX at all.
 *
 * The MPIIX also differs in another important way from the majority of PIIX
 * devices. The chip is a bridge (pardon the pun) between the old world of
 * ISA IDE and PCI IDE. Although the ATA timings are PCI configured the actual
 * IDE controller is not decoded in PCI space and the chip does not claim to
 * be IDE class PCI. This requires slightly non-standard probe logic compared
 * with PCI IDE and also that we do not disable the device when our driver is
 * unloaded (as it has many other functions).
 *
 * The driver consciously keeps this logic internally to avoid pushing quirky
 * PATA history into the clean libata layer.
 *
 * Thinkpad specific note: If you boot an MPIIX using a thinkpad with a PCMCIA
 * hard disk present this driver will not detect it. This is not a bug. In
 * this configuration the secondary port of the MPIIX is disabled and the
 * addresses are decoded by the PCMCIA bridge and therefore are for a generic
 * IDE driver to operate.
 */

// Linux kernel dependencies supplied externally.

const DRV_NAME: &str = "pata_mpiix";
const DRV_VERSION: &str = "0.7.7";

const IDETIM: u16 = 0x6c; // IDE control register
const IORDY: u16 = 1 << 1;
const PPE: u16 = 1 << 2;
const FTIM: u16 = 1 << 0;
const ENABLED: u16 = 1 << 15;
const SECONDARY: u16 = 1 << 14;

unsafe fn mpiix_pre_reset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mpiix_enable_bits = pci_bits { reg: 0x6d, width: 1, mask: 0x80, val: 0x80 };

    if !pci_test_config_bits(pdev, &mpiix_enable_bits) {
        return -ENOENT;
    }
    ata_sff_prereset(link, deadline)
}

unsafe fn mpiix_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let mut control: c_int = 0;
    let pio = (*adev).pio_mode - XFER_PIO_0;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut idetim: u16 = 0;
    let timings: [[u8; 2]; 5] = [[0, 0], [0, 0], [1, 0], [2, 1], [2, 3]];

    pci_read_config_word(pdev, IDETIM, &mut idetim);

    // Mask the IORDY/TIME/PPE for this device
    if (*adev).class == ATA_DEV_ATA { control |= PPE as c_int; }
    if ata_pio_need_iordy(adev) { control |= IORDY as c_int; }
    if pio > 1 { control |= FTIM as c_int; }

    idetim &= 0xCCEE;
    idetim &= !(0x07 << (4 * (*adev).devno));
    idetim |= ((control as u16) << (4 * (*adev).devno));
    idetim |= ((timings[pio as usize][0] as u16) << 12)
        | ((timings[pio as usize][1] as u16) << 8);
    pci_write_config_word(pdev, IDETIM, idetim);

    // We use ap->private_data as a pointer to the device currently loaded for timing
    (*ap).private_data = adev as *mut c_void;
}

unsafe fn mpiix_qc_issue(qc: *mut ata_queued_cmd) -> c_uint {
    let ap = (*qc).ap;
    let adev = (*qc).dev;

    // If modes have been configured and the channel data is not loaded then load it.
    if (*adev).pio_mode != 0 && (adev as *mut c_void) != (*ap).private_data {
        mpiix_set_piomode(ap, adev);
    }
    ata_sff_qc_issue(qc)
}

static mut mpiix_sht: scsi_host_template = scsi_host_template {
    // ATA_PIO_SHT(DRV_NAME)
};

static mut mpiix_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_sff_port_ops,
    qc_issue: Some(mpiix_qc_issue),
    cable_detect: Some(ata_cable_40wire),
    set_piomode: Some(mpiix_set_piomode),
    reset: ata_port_reset_operations { prereset: Some(mpiix_pre_reset) },
    sff_data_xfer: Some(ata_sff_data_xfer32),
};

unsafe fn mpiix_init_one(dev: *mut pci_dev, _id: *const pci_device_id) -> c_int {
    // Single threaded by the PCI probe logic
    let host = ata_host_alloc(&mut (*dev).dev, 1);
    if host.is_null() { return -ENOMEM; }
    let ap = (*host).ports[0];
    let mut idetim: u16 = 0;
    let (cmd, ctl, irq): (c_int, c_int, c_int);

    ata_print_version_once(&mut (*dev).dev, DRV_VERSION);
    pci_read_config_word(dev, IDETIM, &mut idetim);
    if idetim & ENABLED == 0 { return -ENODEV; }

    if idetim & SECONDARY == 0 {
        cmd = 0x1F0; ctl = 0x3F6; irq = 14;
    } else {
        cmd = 0x170; ctl = 0x376; irq = 15;
    }

    let cmd_addr = devm_ioport_map(&mut (*dev).dev, cmd, 8);
    let ctl_addr = devm_ioport_map(&mut (*dev).dev, ctl, 1);
    if cmd_addr.is_null() || ctl_addr.is_null() { return -ENOMEM; }

    ata_port_desc(ap, "cmd 0x%x ctl 0x%x", cmd, ctl);
    (*ap).ops = &mpiix_port_ops;
    (*ap).pio_mask = ATA_PIO4;
    (*ap).flags |= ATA_FLAG_SLAVE_POSS;
    (*ap).ioaddr.cmd_addr = cmd_addr;
    (*ap).ioaddr.ctl_addr = ctl_addr;
    (*ap).ioaddr.altstatus_addr = ctl_addr;
    ata_sff_std_ports(&mut (*ap).ioaddr);

    ata_host_activate(host, irq, ata_sff_interrupt, IRQF_SHARED, &mpiix_sht)
}

static mut mpiix: [pci_device_id; 2] = [
    PCI_VDEVICE(INTEL, PCI_DEVICE_ID_INTEL_82371MX),
    pci_device_id {},
];

static mut mpiix_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: mpiix.as_ptr(),
    probe: Some(mpiix_init_one),
    remove: Some(ata_pci_remove_one),
    // #ifdef CONFIG_PM_SLEEP: suspend = ata_pci_device_suspend, resume = ata_pci_device_resume
};

module_pci_driver!(mpiix_pci_driver);

module_author!("Alan Cox");
module_description!("low-level driver for Intel MPIIX");
module_license!("GPL");
module_device_table!(pci, mpiix);
module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
