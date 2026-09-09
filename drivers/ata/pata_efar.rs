// SPDX-License-Identifier: GPL-2.0-only
/*
 *    pata_efar.c - EFAR PIIX clone controller driver
 *
 *	(C) 2005 Red Hat
 *	(C) 2009-2010 Bartlomiej Zolnierkiewicz
 *
 *    Some parts based on ata_piix.c by Jeff Garzik and others.
 *
 *    The EFAR is a PIIX4 clone with UDMA66 support. Unlike the later
 *    Intel ICH controllers the EFAR widened the UDMA mode register bits
 *    and doesn't require the funky clock selection.
 */

const DRV_NAME: &str = "pata_efar";
const DRV_VERSION: &str = "0.4.5";

/* Dependencies supplied by the kernel's Rust bindings. */
use core::ptr;

unsafe fn efar_pre_reset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    static EFAR_ENABLE_BITS: [pci_bits; 2] = [
        pci_bits { reg: 0x41u8, width: 1u8, mask: 0x80ul, val: 0x80ul },
        pci_bits { reg: 0x43u8, width: 1u8, mask: 0x80ul, val: 0x80ul },
    ];
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);

    if !pci_test_config_bits(pdev, &EFAR_ENABLE_BITS[(*ap).port_no as usize]) {
        return -ENOENT;
    }

    ata_sff_prereset(link, deadline)
}

unsafe fn efar_cable_detect(ap: *mut ata_port) -> c_int {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut tmp: u8 = 0;

    pci_read_config_byte(pdev, 0x47, &mut tmp);
    if (tmp & (2u8 >> (*ap).port_no)) != 0 {
        return ATA_CBL_PATA40;
    }
    ATA_CBL_PATA80
}

static mut efar_lock: spinlock_t = DEFINE_SPINLOCK!();

unsafe fn efar_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pio = (*adev).pio_mode - XFER_PIO_0;
    let dev = to_pci_dev((*(*ap).host).dev);
    let master_port: c_uint = if (*ap).port_no != 0 { 0x42 } else { 0x40 };
    let mut flags: c_ulong = 0;
    let mut master_data: u16 = 0;
    let mut udma_enable: u8 = 0;
    let mut control: c_int = 0;
    static TIMINGS: [[u8; 2]; 5] = [[0, 0], [0, 0], [1, 0], [2, 1], [2, 3]];

    if pio > 1 { control |= 1; }
    if ata_pio_need_iordy(adev) { control |= 2; }
    if (*adev).class == ATA_DEV_ATA { control |= 4; }

    spin_lock_irqsave(&mut efar_lock, &mut flags);
    pci_read_config_word(dev, master_port, &mut master_data);

    if (*adev).devno == 0 {
        master_data &= 0xCCF0;
        master_data |= control as u16;
        master_data |= ((TIMINGS[pio as usize][0] as u16) << 12)
            | ((TIMINGS[pio as usize][1] as u16) << 8);
    } else {
        let shift = 4 * (*ap).port_no;
        let mut slave_data: u8 = 0;
        master_data &= 0xFF0F;
        master_data |= (control as u16) << 4;
        pci_read_config_byte(dev, 0x44, &mut slave_data);
        slave_data &= if (*ap).port_no != 0 { 0x0F } else { 0xF0 };
        slave_data |= (((TIMINGS[pio as usize][0] << 2) | TIMINGS[pio as usize][1]) as u16 << shift) as u8;
        pci_write_config_byte(dev, 0x44, slave_data);
    }
    master_data |= 0x4000;
    pci_write_config_word(dev, master_port, master_data);
    pci_read_config_byte(dev, 0x48, &mut udma_enable);
    udma_enable &= !(1u8 << (2 * (*ap).port_no + (*adev).devno));
    pci_write_config_byte(dev, 0x48, udma_enable);
    spin_unlock_irqrestore(&mut efar_lock, flags);
}

unsafe fn efar_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let dev = to_pci_dev((*(*ap).host).dev);
    let master_port: u8 = if (*ap).port_no != 0 { 0x42 } else { 0x40 };
    let mut master_data: u16 = 0;
    let speed = (*adev).dma_mode;
    let devid = (*adev).devno + 2 * (*ap).port_no;
    let mut flags: c_ulong = 0;
    let mut udma_enable: u8 = 0;
    static TIMINGS: [[u8; 2]; 5] = [[0, 0], [0, 0], [1, 0], [2, 1], [2, 3]];
    spin_lock_irqsave(&mut efar_lock, &mut flags);
    pci_read_config_word(dev, master_port as c_uint, &mut master_data);
    pci_read_config_byte(dev, 0x48, &mut udma_enable);
    if speed >= XFER_UDMA_0 {
        let udma = (*adev).dma_mode - XFER_UDMA_0;
        let mut udma_timing: u16 = 0;
        udma_enable |= 1u8 << devid;
        pci_read_config_word(dev, 0x4A, &mut udma_timing);
        udma_timing &= !(7 << (4 * devid));
        udma_timing |= udma << (4 * devid);
        pci_write_config_word(dev, 0x4A, udma_timing);
    } else {
        let mwdma = (*adev).dma_mode - XFER_MW_DMA_0;
        let needed_pio = [XFER_PIO_0, XFER_PIO_3, XFER_PIO_4];
        let pio = needed_pio[mwdma as usize] - XFER_PIO_0;
        let mut control = 3u16;
        if (*adev).pio_mode < needed_pio[mwdma as usize] { control |= 8; }
        if (*adev).devno != 0 {
            let mut slave_data: u8 = 0;
            master_data &= 0xFF4F;
            master_data |= control << 4;
            pci_read_config_byte(dev, 0x44, &mut slave_data);
            slave_data &= if (*ap).port_no != 0 { 0x0F } else { 0xF0 };
            slave_data |= (((TIMINGS[pio as usize][0] << 2) | TIMINGS[pio as usize][1]) as u16
                << if (*ap).port_no != 0 { 4 } else { 0 }) as u8;
            pci_write_config_byte(dev, 0x44, slave_data);
        } else {
            master_data &= 0xCCF4;
            master_data |= control;
            master_data |= ((TIMINGS[pio as usize][0] as u16) << 12)
                | ((TIMINGS[pio as usize][1] as u16) << 8);
        }
        udma_enable &= !(1u8 << devid);
        pci_write_config_word(dev, master_port as c_uint, master_data);
    }
    pci_write_config_byte(dev, 0x48, udma_enable);
    spin_unlock_irqrestore(&mut efar_lock, flags);
}

static efar_sht: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);

static mut efar_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    cable_detect: Some(efar_cable_detect),
    set_piomode: Some(efar_set_piomode),
    set_dmamode: Some(efar_set_dmamode),
    reset: ata_port_reset_operations { prereset: Some(efar_pre_reset) },
};

unsafe fn efar_init_one(pdev: *mut pci_dev, _ent: *const pci_device_id) -> c_int {
    static INFO: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO4,
        mwdma_mask: ATA_MWDMA12_ONLY,
        udma_mask: ATA_UDMA4,
        port_ops: &efar_ops,
    };
    let ppi: [*const ata_port_info; 2] = [&INFO, &INFO];
    ata_print_version_once(&mut (*pdev).dev, DRV_VERSION);
    ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &efar_sht, ptr::null_mut(), ATA_HOST_PARALLEL_SCAN)
}

static efar_pci_tbl: [pci_device_id; 2] = [
    PCI_VDEVICE!(EFAR, 0x9130),
    pci_device_id {},
];

static mut efar_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: efar_pci_tbl.as_ptr(),
    probe: Some(efar_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)]
    suspend: Some(ata_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)]
    resume: Some(ata_pci_device_resume),
};

module_pci_driver!(efar_pci_driver);
module_author!("Alan Cox");
module_description!("SCSI low-level driver for EFAR PIIX clones");
module_license!("GPL");
module_device_table!(pci, efar_pci_tbl);
module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
