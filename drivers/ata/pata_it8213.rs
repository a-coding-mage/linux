// SPDX-License-Identifier: GPL-2.0-only
/*
 *    pata_it8213.c - iTE Tech. Inc.  IT8213 PATA driver
 *
 *    The IT8213 is a very Intel ICH like device for timing purposes, having
 *    a similar register layout and the same split clock arrangement. Cable
 *    detection is different, and it does not have slave channels or all the
 *    clutter of later ICH/SATA setups.
 */

// Linux kernel dependencies supplied by other translation units.

const DRV_NAME: &str = "pata_it8213";
const DRV_VERSION: &str = "0.0.3";

unsafe fn it8213_pre_reset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    let it8213_enable_bits: [pci_bits; 1] = [pci_bits { reg: 0x41u32, width: 1u32, mask: 0x80ul, val: 0x80ul }];
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);
    if !pci_test_config_bits(pdev, &it8213_enable_bits[(*ap).port_no as usize]) {
        return -ENOENT;
    }
    ata_sff_prereset(link, deadline)
}

unsafe fn it8213_cable_detect(ap: *mut ata_port) -> c_int {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut tmp: u8 = 0;
    pci_read_config_byte(pdev, 0x42, &mut tmp);
    if tmp & 2 != 0 { // The initial docs are incorrect
        ATA_CBL_PATA40
    } else {
        ATA_CBL_PATA80
    }
}

unsafe fn it8213_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pio = (*adev).pio_mode - XFER_PIO_0;
    let dev = to_pci_dev((*(*ap).host).dev);
    let master_port: c_uint = if (*ap).port_no != 0 { 0x42 } else { 0x40 };
    let mut master_data: u16 = 0;
    let mut control: c_int = 0;
    // ISP, RTC
    let timings: [[u8; 2]; 5] = [[0, 0], [0, 0], [1, 0], [2, 1], [2, 3]];

    if pio > 1 { control |= 1; } // TIME
    if ata_pio_need_iordy(adev) { control |= 2; } // IE
    // Bit 2 is set for ATAPI on the IT8213 - reverse of ICH/PIIX
    if (*adev).class != ATA_DEV_ATA { control |= 4; } // PPE

    pci_read_config_word(dev, master_port, &mut master_data);
    if (*adev).devno == 0 {
        master_data &= 0xCCF0;
        master_data |= control as u16;
        master_data |= ((timings[pio as usize][0] as u16) << 12) |
            ((timings[pio as usize][1] as u16) << 8);
    } else {
        let mut slave_data: u8 = 0;
        master_data &= 0xFF0F;
        master_data |= (control as u16) << 4;
        pci_read_config_byte(dev, 0x44, &mut slave_data);
        slave_data &= 0xF0;
        slave_data |= (timings[pio as usize][0] << 2) | timings[pio as usize][1];
        pci_write_config_byte(dev, 0x44, slave_data);
    }
    master_data |= 0x4000; // Ensure SITRE is set
    pci_write_config_word(dev, master_port, master_data);
}

unsafe fn it8213_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let dev = to_pci_dev((*(*ap).host).dev);
    let mut master_data: u16 = 0;
    let speed: u8 = (*adev).dma_mode;
    let devid: c_int = (*adev).devno;
    let mut udma_enable: u8 = 0;
    let timings: [[u8; 2]; 5] = [[0, 0], [0, 0], [1, 0], [2, 1], [2, 3]];

    pci_read_config_word(dev, 0x40, &mut master_data);
    pci_read_config_byte(dev, 0x48, &mut udma_enable);
    if speed >= XFER_UDMA_0 {
        let udma = (*adev).dma_mode - XFER_UDMA_0;
        let u_speed = core::cmp::min(2 - (udma & 1), udma);
        let u_clock: c_int = if udma > 4 { 0x1000 } else if udma > 2 { 1 } else { 0 };
        let mut udma_timing: u16 = 0;
        let mut ideconf: u16 = 0;
        udma_enable |= 1 << devid;
        pci_read_config_word(dev, 0x4A, &mut udma_timing);
        udma_timing &= !(3 << (4 * devid));
        udma_timing |= (u_speed as u16) << (4 * devid);
        pci_write_config_word(dev, 0x4A, udma_timing);
        pci_read_config_word(dev, 0x54, &mut ideconf);
        ideconf &= !(0x1001 << devid);
        ideconf |= (u_clock as u16) << devid;
        pci_write_config_word(dev, 0x54, ideconf);
    } else {
        let mwdma = (*adev).dma_mode - XFER_MW_DMA_0;
        let needed_pio: [c_uint; 3] = [XFER_PIO_0, XFER_PIO_3, XFER_PIO_4];
        let pio = needed_pio[mwdma as usize] - XFER_PIO_0;
        let mut control: c_uint = 3; // IORDY|TIME1
        let mut slave_data: u8;
        if (*adev).pio_mode < needed_pio[mwdma as usize] { control |= 8; } // PIO cycles in PIO0
        if devid != 0 {
            master_data &= 0xFF4F;
            master_data |= (control as u16) << 4;
            slave_data = 0;
            pci_read_config_byte(dev, 0x44, &mut slave_data);
            slave_data &= 0xF0;
            slave_data |= (((timings[pio as usize][0] << 2) | timings[pio as usize][1]) as u8) << (if (*ap).port_no != 0 { 4 } else { 0 });
            pci_write_config_byte(dev, 0x44, slave_data);
        } else {
            master_data &= 0xCCF4;
            master_data |= control as u16;
            master_data |= ((timings[pio as usize][0] as u16) << 12) | ((timings[pio as usize][1] as u16) << 8);
        }
        udma_enable &= !(1 << devid);
        pci_write_config_word(dev, 0x40, master_data);
    }
    pci_write_config_byte(dev, 0x48, udma_enable);
}

static it8213_sht: scsi_host_template = scsi_host_template { /* ATA_BMDMA_SHT(DRV_NAME) */ };

static mut it8213_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    cable_detect: Some(it8213_cable_detect),
    set_piomode: Some(it8213_set_piomode),
    set_dmamode: Some(it8213_set_dmamode),
    reset: ata_port_reset_operations { prereset: Some(it8213_pre_reset) },
};

unsafe fn it8213_init_one(pdev: *mut pci_dev, _ent: *const pci_device_id) -> c_int {
    static info: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA12_ONLY,
        udma_mask: ATA_UDMA6, port_ops: &it8213_ops,
    };
    let ppi: [*const ata_port_info; 2] = [&info, &ata_dummy_port_info];
    ata_print_version_once(&mut (*pdev).dev, DRV_VERSION);
    ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &it8213_sht, core::ptr::null_mut(), 0)
}

static it8213_pci_tbl: [pci_device_id; 2] = [
    PCI_VDEVICE(ITE, PCI_DEVICE_ID_ITE_8213),
    pci_device_id::default(),
];

static mut it8213_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME, id_table: it8213_pci_tbl.as_ptr(), probe: Some(it8213_init_one),
    remove: Some(ata_pci_remove_one),
    // CONFIG_PM_SLEEP: suspend = ata_pci_device_suspend, resume = ata_pci_device_resume
};

// module_pci_driver(it8213_pci_driver);
// MODULE_AUTHOR("Alan Cox");
// MODULE_DESCRIPTION("SCSI low-level driver for the ITE 8213");
// MODULE_LICENSE("GPL");
// MODULE_DEVICE_TABLE(pci, it8213_pci_tbl);
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
