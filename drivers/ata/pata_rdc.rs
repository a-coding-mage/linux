// SPDX-License-Identifier: GPL-2.0-or-later
/* Driver for later RDC PATA controllers; translated from pata_rdc.c. */

const DRV_NAME: &str = "pata_rdc";
const DRV_VERSION: &str = "0.01";

#[repr(C)]
struct RdcHostPriv { saved_iocfg: u32 }

unsafe fn rdc_pata_cable_detect(ap: *mut ata_port) -> i32 {
    let hpriv = (*(*ap).host).private_data as *mut RdcHostPriv;
    let mask: u8 = 0x30u8 << (2 * (*ap).port_no);
    if ((*hpriv).saved_iocfg & mask as u32) == 0 { ATA_CBL_PATA40 } else { ATA_CBL_PATA80 }
}

unsafe fn rdc_pata_prereset(link: *mut ata_link, deadline: c_ulong) -> i32 {
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let rdc_enable_bits: [pci_bits; 2] = [
        pci_bits { reg: 0x41, width: 1, mask: 0x80, val: 0x80 },
        pci_bits { reg: 0x43, width: 1, mask: 0x80, val: 0x80 },
    ];
    if !pci_test_config_bits(pdev, &rdc_enable_bits[(*ap).port_no as usize]) { return -ENOENT; }
    ata_sff_prereset(link, deadline)
}

static mut RDC_LOCK: spinlock_t = spinlock_t::new();

unsafe fn rdc_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pio = (*adev).pio_mode - XFER_PIO_0;
    let dev = to_pci_dev((*(*ap).host).dev);
    let mut flags: c_ulong = 0;
    let is_slave = ((*adev).devno != 0) as u32;
    let master_port: u32 = if (*ap).port_no != 0 { 0x42 } else { 0x40 };
    let slave_port = 0x44;
    let mut master_data: u16 = 0;
    let mut slave_data: u8 = 0;
    let mut udma_enable: u8 = 0;
    let mut control: u16 = 0;
    let timings: [[u16; 2]; 5] = [[0,0], [0,0], [1,0], [2,1], [2,3]];
    if pio >= 2 { control |= 1; }
    if ata_pio_need_iordy(adev) { control |= 2; }
    if (*adev).class == ATA_DEV_ATA { control |= 4; }
    spin_lock_irqsave(&mut RDC_LOCK, &mut flags);
    pci_read_config_word(dev, master_port, &mut master_data);
    if is_slave != 0 {
        master_data &= 0xff0f; master_data |= 0x4000; master_data |= control << 4;
        pci_read_config_byte(dev, slave_port, &mut slave_data);
        slave_data &= if (*ap).port_no != 0 { 0x0f } else { 0xf0 };
        slave_data |= ((timings[pio as usize][0] << 2) | timings[pio as usize][1]) << if (*ap).port_no != 0 { 4 } else { 0 };
    } else {
        master_data &= 0xccf0; master_data |= control;
        master_data |= (timings[pio as usize][0] << 12) | (timings[pio as usize][1] << 8);
    }
    pci_write_config_word(dev, master_port, master_data);
    if is_slave != 0 { pci_write_config_byte(dev, slave_port, slave_data); }
    pci_read_config_byte(dev, 0x48, &mut udma_enable);
    udma_enable &= !(1u8 << (2 * (*ap).port_no + (*adev).devno));
    pci_write_config_byte(dev, 0x48, udma_enable);
    spin_unlock_irqrestore(&mut RDC_LOCK, flags);
}

unsafe fn rdc_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let dev = to_pci_dev((*(*ap).host).dev); let mut flags = 0;
    let master_port = if (*ap).port_no != 0 { 0x42 } else { 0x40 };
    let mut master_data=0u16; let speed=(*adev).dma_mode; let devid=(*adev).devno + 2*(*ap).port_no; let mut udma_enable=0u8;
    let timings=[[0u16,0],[0,0],[1,0],[2,1],[2,3]];
    spin_lock_irqsave(&mut RDC_LOCK, &mut flags); pci_read_config_word(dev,master_port,&mut master_data); pci_read_config_byte(dev,0x48,&mut udma_enable);
    if speed >= XFER_UDMA_0 {
        let udma=speed-XFER_UDMA_0; let u_speed=std::cmp::min(2-(udma&1),udma); let u_clock=if udma==5 {0x1000} else if udma>2 {1} else {0}; udma_enable |= 1<<devid;
        let mut t=0u16; pci_read_config_word(dev,0x4a,&mut t); t &= !(3 << (4*devid)); t |= u_speed << (4*devid); pci_write_config_word(dev,0x4a,t);
        let mut c=0u16; pci_read_config_word(dev,0x54,&mut c); c &= !(0x1001 << devid); c |= u_clock << devid; pci_write_config_word(dev,0x54,c);
    } else {
        let mwdma=speed-XFER_MW_DMA_0; let needed=[XFER_PIO_0,XFER_PIO_3,XFER_PIO_4]; let pio=needed[mwdma as usize]-XFER_PIO_0; let mut control=3u16;
        if (*adev).pio_mode < needed[mwdma as usize] { control |= 8; }
        if (*adev).devno != 0 { let mut s=0u8; master_data &= 0xff4f; master_data |= control<<4; pci_read_config_byte(dev,0x44,&mut s); s &= if (*ap).port_no!=0 {0x0f} else {0xf0}; s |= ((timings[pio as usize][0]<<2)|timings[pio as usize][1]) << if (*ap).port_no!=0 {4} else {0}; pci_write_config_byte(dev,0x44,s); }
        else { master_data &= 0xccf4; master_data |= control; master_data |= (timings[pio as usize][0]<<12)|(timings[pio as usize][1]<<8); }
        udma_enable &= !(1<<devid); pci_write_config_word(dev,master_port,master_data);
    }
    pci_write_config_byte(dev,0x48,udma_enable); spin_unlock_irqrestore(&mut RDC_LOCK,flags);
}

// The remaining kernel registration structures and module metadata are declarations
// whose concrete definitions are supplied by the surrounding kernel bindings.
static mut rdc_pata_ops: ata_port_operations = ata_port_operations { inherits: &ata_bmdma32_port_ops, cable_detect: Some(rdc_pata_cable_detect), set_piomode: Some(rdc_set_piomode), set_dmamode: Some(rdc_set_dmamode), reset_prereset: Some(rdc_pata_prereset) };

static rdc_port_info: ata_port_info = ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA12_ONLY, udma_mask: ATA_UDMA5, port_ops: unsafe { &rdc_pata_ops } };
static rdc_sht: scsi_host_template = scsi_host_template { /* ATA_BMDMA_SHT(DRV_NAME) */ };

unsafe fn rdc_init_one(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    let dev = &mut (*pdev).dev; let mut port_info = [rdc_port_info, rdc_port_info];
    let ppi = [&port_info[0] as *const ata_port_info, &port_info[1] as *const ata_port_info];
    let mut host: *mut ata_host = std::ptr::null_mut();
    ata_print_version_once(dev, DRV_VERSION);
    let rc = pcim_enable_device(pdev); if rc != 0 { return rc; }
    let hpriv = devm_kzalloc(dev, std::mem::size_of::<RdcHostPriv>(), GFP_KERNEL) as *mut RdcHostPriv;
    if hpriv.is_null() { return -ENOMEM; }
    pci_read_config_dword(pdev, 0x54, &mut (*hpriv).saved_iocfg);
    let rc = ata_pci_bmdma_prepare_host(pdev, ppi.as_ptr(), &mut host); if rc != 0 { return rc; }
    (*host).private_data = hpriv as *mut c_void; pcim_intx(pdev, 1); (*host).flags |= ATA_HOST_PARALLEL_SCAN; pci_set_master(pdev);
    ata_pci_sff_activate_host(host, ata_bmdma_interrupt, &rdc_sht)
}

unsafe fn rdc_remove_one(pdev: *mut pci_dev) {
    let host = pci_get_drvdata(pdev) as *mut ata_host; let hpriv = (*host).private_data as *mut RdcHostPriv;
    pci_write_config_dword(pdev, 0x54, (*hpriv).saved_iocfg); ata_pci_remove_one(pdev);
}

static rdc_pci_tbl: [pci_device_id; 3] = [
    pci_device_id::vdevice(PCI_VENDOR_ID_RDC, 0x1011),
    pci_device_id::vdevice(PCI_VENDOR_ID_RDC, 0x1012),
    pci_device_id::empty(),
];

static mut rdc_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME, id_table: rdc_pci_tbl.as_ptr(), probe: Some(rdc_init_one), remove: Some(rdc_remove_one),
    #[cfg(CONFIG_PM_SLEEP)] suspend: Some(ata_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)] resume: Some(ata_pci_device_resume),
};

// module_pci_driver(rdc_pci_driver)
// MODULE_AUTHOR("Alan Cox (based on ata_piix)")
// MODULE_DESCRIPTION("SCSI low-level driver for RDC PATA controllers")
// MODULE_LICENSE("GPL")
// MODULE_DEVICE_TABLE(pci, rdc_pci_tbl)
// MODULE_VERSION(DRV_VERSION)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
