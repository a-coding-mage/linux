// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata_cypress.c 	- Cypress PATA for new ATA layer
 *			  (C) 2006 Red Hat Inc
 *			  Alan Cox
 *
 * Based heavily on
 * linux/drivers/ide/pci/cy82c693.c		Version 0.40	Sep. 10, 2002
 *
 */

// Dependencies supplied by the Linux kernel bindings.

pub const DRV_NAME: &str = "pata_cypress";
pub const DRV_VERSION: &str = "0.1.5";

/* here are the offset definitions for the registers */
#[repr(u32)]
pub enum Cy82Register {
    CY82_IDE_CMDREG = 0x04,
    CY82_IDE_ADDRSETUP = 0x48,
    CY82_IDE_MASTER_IOR = 0x4C,
    CY82_IDE_MASTER_IOW = 0x4D,
    CY82_IDE_SLAVE_IOR = 0x4E,
    CY82_IDE_SLAVE_IOW = 0x4F,
    CY82_IDE_MASTER_8BIT = 0x50,
    CY82_IDE_SLAVE_8BIT = 0x51,
    CY82_INDEX_PORT = 0x22,
    CY82_DATA_PORT = 0x23,
    CY82_INDEX_CTRLREG1 = 0x01,
    CY82_INDEX_CHANNEL0 = 0x30,
    CY82_INDEX_CHANNEL1 = 0x31,
    CY82_INDEX_TIMEOUT = 0x32,
}

static mut enable_dma: bool = true;

/**
 *	cy82c693_set_piomode	-	set initial PIO mode data
 *	@ap: ATA interface
 *	@adev: ATA device
 *
 *	Called to do the PIO mode setup.
 */
unsafe fn cy82c693_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut t = ata_timing::default();
    const T: libc::c_ulong = 1000000 / 33;
    let mut time_16: i16;
    let mut time_8: i16;
    let mut addr: u32 = 0;

    if ata_timing_compute(adev, (*adev).pio_mode, &mut t, T, 1) < 0 {
        ata_dev_err(adev, concat!(DRV_NAME, ": mode computation failed.\n"));
        return;
    }

    time_16 = clamp_val(t.recover - 1, 0, 15) |
        (clamp_val(t.active - 1, 0, 15) << 4);
    time_8 = clamp_val(t.act8b - 1, 0, 15) |
        (clamp_val(t.rec8b - 1, 0, 15) << 4);

    if (*adev).devno == 0 {
        pci_read_config_dword(pdev, Cy82Register::CY82_IDE_ADDRSETUP as u32, &mut addr);
        addr &= !0x0F; /* Mask bits */
        addr |= clamp_val(t.setup - 1, 0, 15) as u32;
        pci_write_config_dword(pdev, Cy82Register::CY82_IDE_ADDRSETUP as u32, addr);
        pci_write_config_byte(pdev, Cy82Register::CY82_IDE_MASTER_IOR as u32, time_16 as u8);
        pci_write_config_byte(pdev, Cy82Register::CY82_IDE_MASTER_IOW as u32, time_16 as u8);
        pci_write_config_byte(pdev, Cy82Register::CY82_IDE_MASTER_8BIT as u32, time_8 as u8);
    } else {
        pci_read_config_dword(pdev, Cy82Register::CY82_IDE_ADDRSETUP as u32, &mut addr);
        addr &= !0xF0; /* Mask bits */
        addr |= (clamp_val(t.setup - 1, 0, 15) << 4) as u32;
        pci_write_config_dword(pdev, Cy82Register::CY82_IDE_ADDRSETUP as u32, addr);
        pci_write_config_byte(pdev, Cy82Register::CY82_IDE_SLAVE_IOR as u32, time_16 as u8);
        pci_write_config_byte(pdev, Cy82Register::CY82_IDE_SLAVE_IOW as u32, time_16 as u8);
        pci_write_config_byte(pdev, Cy82Register::CY82_IDE_SLAVE_8BIT as u32, time_8 as u8);
    }
}

/** Set initial DMA mode data. */
unsafe fn cy82c693_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let reg = Cy82Register::CY82_INDEX_CHANNEL0 as i32 + (*ap).port_no;
    /* Be afraid, be very afraid. Magic registers in low I/O space */
    outb(reg as u8, 0x22);
    outb(((*adev).dma_mode - XFER_MW_DMA_0) as u8, 0x23);
    /* 0x50 gives the best behaviour on the Alpha's using this chip */
    outb(Cy82Register::CY82_INDEX_TIMEOUT as u8, 0x22);
    outb(0x50, 0x23);
}

static cy82c693_sht: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);

static mut cy82c693_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    cable_detect: Some(ata_cable_40wire),
    set_piomode: Some(cy82c693_set_piomode),
    set_dmamode: Some(cy82c693_set_dmamode),
};

unsafe fn cy82c693_init_one(pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    static mut info: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO4,
        mwdma_mask: 0,
        port_ops: &cy82c693_port_ops,
    };
    let ppi: [*const ata_port_info; 2] = [&info, &ata_dummy_port_info];

    if enable_dma {
        info.mwdma_mask = ATA_MWDMA2;
    }

    /* Devfn 1 is the ATA primary. The secondary is magic and on devfn2.
       For the moment we don't handle the secondary. FIXME */
    if PCI_FUNC((*pdev).devfn) != 1 {
        return -ENODEV;
    }
    ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &cy82c693_sht, core::ptr::null_mut(), 0)
}

static cy82c693: [pci_device_id; 2] = [
    PCI_VDEVICE!(CONTAQ, PCI_DEVICE_ID_CONTAQ_82C693),
    pci_device_id::default(),
];

static mut cy82c693_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: cy82c693.as_ptr(),
    probe: Some(cy82c693_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)]
    suspend: Some(ata_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)]
    resume: Some(ata_pci_device_resume),
};

module_pci_driver!(cy82c693_pci_driver);

MODULE_AUTHOR!("Alan Cox");
MODULE_DESCRIPTION!("low-level driver for the CY82C693 PATA controller");
MODULE_LICENSE!("GPL");
MODULE_DEVICE_TABLE!(pci, cy82c693);
MODULE_VERSION!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
