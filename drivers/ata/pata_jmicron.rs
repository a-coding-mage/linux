// SPDX-License-Identifier: GPL-2.0-only
/*
 *    pata_jmicron.c - JMicron ATA driver for non AHCI mode. This drives the
 *			PATA port of the controller. The SATA ports are
 *			driven by AHCI in the usual configuration although
 *			this driver can handle other setups if we need it.
 *
 *	(c) 2006 Red Hat
 */

// Dependencies supplied by the surrounding kernel translation.

pub const DRV_NAME: &str = "pata_jmicron";
pub const DRV_VERSION: &str = "0.1.5";

#[repr(C)]
#[derive(Copy, Clone)]
enum PortType {
    PORT_PATA0 = 0,
    PORT_PATA1 = 1,
    PORT_SATA = 2,
}

/**
 *	jmicron_pre_reset	- check for 40/80 pin
 *	@link: ATA link
 *	@deadline: deadline jiffies for the operation
 *
 *	Perform the PATA port setup we need.
 *
 *	On the Jmicron 361/363 there is a single PATA port that can be mapped
 *	either as primary or secondary (or neither). We don't do any policy
 *	and setup here. We assume that has been done by init_one and the
 *	BIOS.
 */
unsafe fn jmicron_pre_reset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut control: u32 = 0;
    let mut control5: u32 = 0;
    let port_mask: c_int = 1 << (4 * (*ap).port_no);
    let mut port = (*ap).port_no;
    let mut port_map: [PortType; 2] = [PortType::PORT_SATA; 2];

    pci_read_config_dword(pdev, 0x40, &mut control);
    if (control & port_mask as u32) == 0 {
        return -ENOENT;
    }

    if (control & (1 << 23)) != 0 {
        port_map[0] = PortType::PORT_SATA;
        port_map[1] = PortType::PORT_PATA0;
    } else {
        port_map[0] = PortType::PORT_SATA;
        port_map[1] = PortType::PORT_SATA;
    }

    pci_read_config_dword(pdev, 0x80, &mut control5);
    if (control5 & (1 << 24)) != 0 {
        port_map[0] = PortType::PORT_PATA1;
    }

    if (control & (1 << 22)) != 0 {
        port ^= 1;
    }

    match port_map[port as usize] {
        PortType::PORT_PATA0 => {
            if (control & (1 << 5)) == 0 {
                return -ENOENT;
            }
            if (control & (1 << 3)) != 0 {
                (*ap).cbl = ATA_CBL_PATA40;
            } else {
                (*ap).cbl = ATA_CBL_PATA80;
            }
        }
        PortType::PORT_PATA1 => {
            if (control5 & (1 << 21)) == 0 {
                return -ENOENT;
            }
            if (control5 & (1 << 19)) != 0 {
                (*ap).cbl = ATA_CBL_PATA40;
            } else {
                (*ap).cbl = ATA_CBL_PATA80;
            }
        }
        PortType::PORT_SATA => {
            (*ap).cbl = ATA_CBL_SATA;
        }
    }
    ata_sff_prereset(link, deadline)
}

// No PIO or DMA methods needed for this device

static mut jmicron_sht: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);

static mut jmicron_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    reset: ata_port_reset_operations {
        prereset: Some(jmicron_pre_reset),
    },
};

/**
 *	jmicron_init_one - Register Jmicron ATA PCI device with kernel services
 *	@pdev: PCI device to register
 *	@id: PCI device ID
 *
 *	Called from kernel PCI layer.
 *
 *	LOCKING:
 *	Inherited from PCI layer (may sleep).
 *
 *	RETURNS:
 *	Zero on success, or -ERRNO value.
 */
unsafe fn jmicron_init_one(
    pdev: *mut pci_dev,
    _id: *const pci_device_id,
) -> c_int {
    static mut INFO: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO4,
        mwdma_mask: ATA_MWDMA2,
        udma_mask: ATA_UDMA5,
        port_ops: &jmicron_ops,
    };
    let ppi: [*const ata_port_info; 2] = [&INFO, core::ptr::null()];
    ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &jmicron_sht, core::ptr::null_mut(), 0)
}

static mut jmicron_pci_tbl: [pci_device_id; 2] = [
    pci_device_id {
        vendor: PCI_VENDOR_ID_JMICRON,
        device: PCI_ANY_ID,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: PCI_CLASS_STORAGE_IDE << 8,
        class_mask: 0xffff00,
        driver_data: 0,
    },
    pci_device_id { ..pci_device_id::ZERO },
];

static mut jmicron_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: jmicron_pci_tbl.as_ptr(),
    probe: Some(jmicron_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)]
    suspend: Some(ata_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)]
    resume: Some(ata_pci_device_resume),
};

module_pci_driver!(jmicron_pci_driver);

module_author!("Alan Cox");
module_description!("SCSI low-level driver for Jmicron PATA ports");
module_license!("GPL");
module_device_table!(pci, jmicron_pci_tbl);
module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
