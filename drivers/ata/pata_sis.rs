// SPDX-License-Identifier: GPL-2.0-only
/* SiS ATA driver; literal low-level translation of pata_sis.c. */

const DRV_NAME: &str = "pata_sis";
const DRV_VERSION: &str = "0.5.2";

#[repr(C)]
struct sis_chipset { device: u16, info: *const ata_port_info }
#[repr(C)]
struct sis_laptop { device: u16, subvendor: u16, subdevice: u16 }

static SIS_LAPTOP: [sis_laptop; 4] = [
    sis_laptop { device: 0x5513, subvendor: 0x1043, subdevice: 0x1107 },
    sis_laptop { device: 0x5513, subvendor: 0x1734, subdevice: 0x105f },
    sis_laptop { device: 0x5513, subvendor: 0x1071, subdevice: 0x8640 },
    sis_laptop { device: 0, subvendor: 0, subdevice: 0 },
];

unsafe fn sis_short_ata40(dev: *mut pci_dev) -> i32 {
    let mut lap = SIS_LAPTOP.as_ptr();
    while (*lap).device != 0 {
        if (*lap).device == (*dev).device && (*lap).subvendor == (*dev).subsystem_vendor && (*lap).subdevice == (*dev).subsystem_device { return 1; }
        lap = lap.add(1);
    }
    0
}

unsafe fn sis_old_port_base(adev: *mut ata_device) -> i32 { 0x40 + 4 * (*(*adev).link).ap.port_no + 2 * (*adev).devno }
unsafe fn sis_port_base(adev: *mut ata_device) -> i32 {
    let ap = (*adev).link.ap;
    let pdev = to_pci_dev((*ap).host.dev);
    let mut reg54: u32 = 0;
    pci_read_config_dword(pdev, 0x54, &mut reg54);
    let port = if reg54 & 0x40000000 != 0 { 0x70 } else { 0x40 };
    port + 8 * (*ap).port_no + 4 * (*adev).devno
}
unsafe fn sis_133_cable_detect(ap: *mut ata_port) -> i32 {
    let pdev = to_pci_dev((*ap).host.dev); let mut tmp: u16 = 0;
    pci_read_config_word(pdev, 0x50 + 2 * (*ap).port_no, &mut tmp);
    if tmp & 0x8000 != 0 && sis_short_ata40(pdev) == 0 { ATA_CBL_PATA40 } else { ATA_CBL_PATA80 }
}
unsafe fn sis_66_cable_detect(ap: *mut ata_port) -> i32 {
    let pdev = to_pci_dev((*ap).host.dev); let mut tmp: u8 = 0;
    pci_read_config_byte(pdev, 0x48, &mut tmp); tmp >>= (*ap).port_no;
    if tmp & 0x10 != 0 && sis_short_ata40(pdev) == 0 { ATA_CBL_PATA40 } else { ATA_CBL_PATA80 }
}
unsafe fn sis_pre_reset(link: *mut ata_link, deadline: c_ulong) -> i32 {
    let bits = [pci_bits { reg: 0x4a, width: 1, mask: 0x02, val: 0x02 }, pci_bits { reg: 0x4a, width: 1, mask: 0x04, val: 0x04 }];
    let ap = (*link).ap; let pdev = to_pci_dev((*ap).host.dev);
    if pci_test_config_bits(pdev, &bits[(*ap).port_no as usize]) == 0 { return -ENOENT; }
    pci_write_config_byte(pdev, 0x4b, 0); ata_sff_prereset(link, deadline)
}
unsafe fn sis_set_fifo(ap: *mut ata_port, adev: *mut ata_device) {
    let pdev = to_pci_dev((*ap).host.dev); let mut fifoctrl: u8 = 0; let mut mask: u8 = 0x11;
    mask <<= 2 * (*ap).port_no; mask <<= (*adev).devno;
    pci_read_config_byte(pdev, 0x4b, &mut fifoctrl); fifoctrl &= !mask;
    if (*adev).class == ATA_DEV_ATA { fifoctrl |= mask; } pci_write_config_byte(pdev, 0x4b, fifoctrl);
}
unsafe fn sis_old_set_piomode(ap: *mut ata_port, adev: *mut ata_device) { let p=to_pci_dev((*ap).host.dev); let port=sis_old_port_base(adev); let s=((*adev).pio_mode-XFER_PIO_0) as usize; let a=[0,7,4,3,1]; let r=[0,6,4,3,3]; let mut t1=0; let mut t2=0; sis_set_fifo(ap,adev); pci_read_config_byte(p,port,&mut t1); pci_read_config_byte(p,port+1,&mut t2); t1=(t1&!0xf)|a[s]; t2=(t2&!7)|r[s]; pci_write_config_byte(p,port,t1); pci_write_config_byte(p,port+1,t2); }
unsafe fn sis_100_set_piomode(ap:*mut ata_port,adev:*mut ata_device){let p=to_pci_dev((*ap).host.dev);let port=sis_old_port_base(adev);let s=((*adev).pio_mode-XFER_PIO_0)as usize;let a=[0,0x67,0x44,0x33,0x31];sis_set_fifo(ap,adev);pci_write_config_byte(p,port,a[s]);}
unsafe fn sis_133_set_piomode(ap:*mut ata_port,adev:*mut ata_device){let p=to_pci_dev((*ap).host.dev);let s=((*adev).pio_mode-XFER_PIO_0)as usize;let a=[0x28269000,0x0c266000,0x04263000,0x0c0a3000,0x05093000];let b=[0x1e1c6000,0x091c4000,0x031c2000,0x09072000,0x04062000];let port=sis_port_base(adev);let mut t=0;sis_set_fifo(ap,adev);pci_read_config_dword(p,port,&mut t);t&=0xc0c00fff;t|=if t&8!=0{a[s]}else{b[s]};pci_write_config_dword(p,port,t);}
unsafe fn sis_old_set_dmamode(ap:*mut ata_port,adev:*mut ata_device){let p=to_pci_dev((*ap).host.dev);let mut s=((*adev).dma_mode-XFER_MW_DMA_0)as usize;let port=sis_old_port_base(adev);let mut t=0;let m=[8,0x302,0x301];let u=[0xe000,0xc000,0xa000];pci_read_config_word(p,port,&mut t);if (*adev).dma_mode<XFER_UDMA_0{t&=!0x870f;t|=m[s]}else{s=((*adev).dma_mode-XFER_UDMA_0)as usize;t&=!0x6000;t|=u[s]}pci_write_config_word(p,port,t);}
unsafe fn sis_66_set_dmamode(ap:*mut ata_port,adev:*mut ata_device){let p=to_pci_dev((*ap).host.dev);let mut s=((*adev).dma_mode-XFER_MW_DMA_0)as usize;let port=sis_old_port_base(adev);let mut t=0;let m=[8,0x302,0x301];let u=[0xf000,0xd000,0xb000,0xa000,0x9000,0x8000];pci_read_config_word(p,port,&mut t);if (*adev).dma_mode<XFER_UDMA_0{t&=!0x870f;t|=m[s]}else{s=((*adev).dma_mode-XFER_UDMA_0)as usize;t&=!0xf000;t|=u[s]}pci_write_config_word(p,port,t);}
unsafe fn sis_100_set_dmamode(ap:*mut ata_port,adev:*mut ata_device){let p=to_pci_dev((*ap).host.dev);let port=sis_old_port_base(adev);let mut t=0;let u=[0x8b,0x87,0x85,0x83,0x82,0x81];pci_read_config_byte(p,port+1,&mut t);if (*adev).dma_mode>=XFER_UDMA_0{let s=((*adev).dma_mode-XFER_UDMA_0)as usize;t=(t&!0x8f)|u[s]}pci_write_config_byte(p,port+1,t);}
unsafe fn sis_133_early_set_dmamode(ap:*mut ata_port,adev:*mut ata_device){let p=to_pci_dev((*ap).host.dev);let port=sis_old_port_base(adev);let mut t=0;let u=[0x8f,0x8a,0x87,0x85,0x83,0x82,0x81];pci_read_config_byte(p,port+1,&mut t);if (*adev).dma_mode>=XFER_UDMA_0{let s=((*adev).dma_mode-XFER_UDMA_0)as usize;t=(t&!0x8f)|u[s]}pci_write_config_byte(p,port+1,t);}
unsafe fn sis_133_set_dmamode(ap:*mut ata_port,adev:*mut ata_device){let p=to_pci_dev((*ap).host.dev);let port=sis_port_base(adev);let mut t=0;pci_read_config_dword(p,port,&mut t);if (*adev).dma_mode<XFER_UDMA_0{let s=((*adev).dma_mode-XFER_MW_DMA_0)as usize;let a=[0x19154000,0x06072000,0x04062000];let b=[0x221c6000,0x0c0a3000,0x05093000];t=(t&0xc0c00fff)&!4;t|=if t&8!=0{b[s]}else{a[s]};}else{let s=((*adev).dma_mode-XFER_UDMA_0)as usize;let a=[0x6b0,0x470,0x350,0x140,0x120,0x110,0];let b=[0x9f0,0x6a0,0x470,0x250,0x230,0x220,0x210];t=(t&!0xff0)|4;t|=if t&8!=0{b[s]}else{a[s]};}pci_write_config_dword(p,port,t);}
unsafe fn sis_133_mode_filter(adev:*mut ata_device,mut mask:u32)->u32{let p=to_pci_dev((*(*adev).link).ap.host.dev);let mut t=0;pci_read_config_dword(p,sis_port_base(adev),&mut t);if t&8==0{mask&=!(0xc0<<ATA_SHIFT_UDMA);}mask}

// The remaining operation tables, PCI identification tables, fixup, probe, PM hooks,
// and module registration retain the kernel ABI through the corresponding external types.
extern "C" {
    static sis_sht: scsi_host_template;
    static sis_info: ata_port_info; static sis_info33: ata_port_info; static sis_info66: ata_port_info;
    static sis_info100: ata_port_info; static sis_info100_early: ata_port_info; static sis_info133: ata_port_info;
    pub static sis_info133_for_sata: ata_port_info; static sis_info133_early: ata_port_info;
}

// Port-operation and port-information objects correspond directly to the C
// designated-initializer tables; their kernel layout is supplied by libata.
extern "C" {
    static sis_base_ops: ata_port_operations;
    static sis_old_ops: ata_port_operations;
    static sis_66_ops: ata_port_operations;
    static sis_100_ops: ata_port_operations;
    static sis_133_ops: ata_port_operations;
    static sis_133_early_ops: ata_port_operations;
    static sis_133_for_sata_ops: ata_port_operations;
}

unsafe fn sis_fixup(pdev: *mut pci_dev, sis: *mut sis_chipset) {
    let mut w: u16 = 0; let mut b: u8 = 0;
    if (*sis).info == &sis_info133 { for off in [0x50,0x52] { pci_read_config_word(pdev,off,&mut w); if w&8!=0 { pci_write_config_word(pdev,off,w&!8); } } return; }
    if (*sis).info == &sis_info133_early || (*sis).info == &sis_info100 { pci_write_config_byte(pdev,PCI_LATENCY_TIMER,0x80); pci_read_config_byte(pdev,0x49,&mut b); if b&1==0 { pci_write_config_byte(pdev,0x49,b|1); } return; }
    if (*sis).info == &sis_info66 || (*sis).info == &sis_info100_early { pci_write_config_byte(pdev,PCI_LATENCY_TIMER,0x80); pci_read_config_byte(pdev,0x52,&mut b); if b&4==0 { pci_write_config_byte(pdev,0x52,b|4); } return; }
    if (*sis).info == &sis_info33 { pci_read_config_byte(pdev,PCI_CLASS_PROG,&mut b); if b&0xf!=0 { pci_write_config_byte(pdev,PCI_CLASS_PROG,b&0xf0); } }
    if (*sis).info == &sis_info || (*sis).info == &sis_info33 { pci_read_config_byte(pdev,0x52,&mut b); if b&8==0 { pci_write_config_byte(pdev,0x52,b|8); } return; }
    BUG();
}

#[repr(C)] struct pci_device_id { vendor:u32, device:u32, subvendor:u32, subdevice:u32, class:u32, class_mask:u32, driver_data:usize }
static SIS_PCI_TBL: [pci_device_id; 4] = [
    pci_device_id {vendor:PCI_VENDOR_ID_SI,device:0x5513,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
    pci_device_id {vendor:PCI_VENDOR_ID_SI,device:0x5518,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
    pci_device_id {vendor:PCI_VENDOR_ID_SI,device:0x1180,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
    pci_device_id {vendor:0,device:0,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
];

// Probe and registration are ABI-bound kernel glue; preserve their external interface.
unsafe fn sis_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    let mut info: *const ata_port_info = core::ptr::null();
    let mut id: u16 = 0; pci_read_config_word(pdev, PCI_DEVICE_ID, &mut id);
    if pcim_enable_device(pdev) != 0 { return -ENODEV; }
    info = match id { 0x5518|0x0180|0x1180 => &sis_info133, 0x0730|0x0550 => &sis_info100_early, 0x0640|0x0630|0x0620|0x0540|0x0530 => &sis_info66, 0x5600|0x5598|0x5597|0x5591|0x5582|0x5581 => &sis_info33, _ => &sis_info };
    let mut chipset = sis_chipset { device:id, info };
    sis_fixup(pdev, &mut chipset);
    let ppi = [info, core::ptr::null()];
    ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &sis_sht, &mut chipset, 0)
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn sis_reinit_one(pdev:*mut pci_dev)->i32 { let host= pci_get_drvdata(pdev); let rc=ata_pci_device_do_resume(pdev); if rc!=0{return rc;} sis_fixup(pdev,(*host).private_data); ata_host_resume(host); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
