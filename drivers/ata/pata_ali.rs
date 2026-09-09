/*
 * pata_ali.c - ALI 15x3 PATA for new ATA layer
 * Rust translation of the implementation source.
 */

const DRV_NAME: &str = "pata_ali";
const DRV_VERSION: &str = "0.7.8";

static mut ali_atapi_dma: i32 = 0;
static mut ali_isa_bridge: *mut pci_dev = core::ptr::null_mut();

static cable_dmi_table: [dmi_system_id; 3] = [
    dmi_system_id {
        ident: "HP Pavilion N5430",
        matches: [DMI_MATCH(DMI_BOARD_VENDOR, "Hewlett-Packard"), DMI_MATCH(DMI_BOARD_VERSION, "OmniBook N32N-736")],
    },
    dmi_system_id {
        ident: "Toshiba Satellite S1800-814",
        matches: [DMI_MATCH(DMI_SYS_VENDOR, "TOSHIBA"), DMI_MATCH(DMI_PRODUCT_NAME, "S1800-814")],
    },
    dmi_system_id::default(),
];

unsafe fn ali_cable_override(pdev: *mut pci_dev) -> i32 {
    if (*pdev).subsystem_vendor == 0x10CF && (*pdev).subsystem_device == 0x10AF { return 1; }
    if (*pdev).subsystem_vendor == 0x1071 && (*pdev).subsystem_device == 0x8317 { return 1; }
    if dmi_check_system(cable_dmi_table.as_ptr()) != 0 { return 1; }
    0
}

unsafe fn ali_c2_cable_detect(ap: *mut ata_port) -> i32 {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut ata66: u8 = 0;
    if ali_cable_override(pdev) != 0 { return ATA_CBL_PATA40_SHORT; }
    pci_read_config_byte(pdev, 0x4A, &mut ata66);
    if (ata66 & (1 << (*ap).port_no)) != 0 { ATA_CBL_PATA40 } else { ATA_CBL_PATA80 }
}

unsafe fn ali_20_filter(adev: *mut ata_device, mut mask: u32) -> u32 {
    let mut model_num = [0i8; ATA_ID_PROD_LEN + 1];
    if (*adev).class != ATA_DEV_ATA { mask &= !(ATA_MASK_MWDMA | ATA_MASK_UDMA); }
    ata_id_c_string((*adev).id, model_num.as_mut_ptr(), ATA_ID_PROD, model_num.len());
    if strstr(model_num.as_ptr(), b"WDC\0".as_ptr() as *const i8) != core::ptr::null() { mask &= !ATA_MASK_UDMA; }
    mask
}

unsafe fn ali_fifo_control(ap: *mut ata_port, adev: *mut ata_device, on: i32) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let pio_fifo = 0x54 + (*ap).port_no as i32;
    let mut fifo: u8 = 0;
    let shift = 4 * (*adev).devno;
    pci_read_config_byte(pdev, pio_fifo, &mut fifo);
    fifo &= !(0x0F << shift);
    fifo |= (on << shift) as u8;
    pci_write_config_byte(pdev, pio_fifo, fifo);
}

unsafe fn ali_program_modes(ap: *mut ata_port, adev: *mut ata_device, t: *mut ata_timing, ultra: u8) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let cas = 0x58 + 4 * (*ap).port_no as i32;
    let cbt = 0x59 + 4 * (*ap).port_no as i32;
    let drwt = 0x5A + 4 * (*ap).port_no as i32 + (*adev).devno as i32;
    let udmat = 0x56 + (*ap).port_no as i32;
    let shift = 4 * (*adev).devno;
    if !t.is_null() {
        (*t).setup = (clamp_val((*t).setup, 1, 8) & 7) as _;
        (*t).act8b = (clamp_val((*t).act8b, 1, 8) & 7) as _;
        (*t).rec8b = (clamp_val((*t).rec8b, 1, 16) & 15) as _;
        (*t).active = (clamp_val((*t).active, 1, 8) & 7) as _;
        (*t).recover = (clamp_val((*t).recover, 1, 16) & 15) as _;
        pci_write_config_byte(pdev, cas, (*t).setup);
        pci_write_config_byte(pdev, cbt, ((*t).act8b << 4) | (*t).rec8b);
        pci_write_config_byte(pdev, drwt, ((*t).active << 4) | (*t).recover);
    }
    let mut udma = 0u8;
    pci_read_config_byte(pdev, udmat, &mut udma);
    udma &= !(0x0F << shift);
    udma |= ultra << shift;
    pci_write_config_byte(pdev, udmat, udma);
}

unsafe fn ali_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pair = ata_dev_pair(adev); let mut t = ata_timing::default(); let clock: u64 = 1000000000 / 33333;
    ata_timing_compute(adev, (*adev).pio_mode, &mut t, clock, 1);
    if !pair.is_null() { let mut p = ata_timing::default(); ata_timing_compute(pair, (*pair).pio_mode, &mut p, clock, 1); ata_timing_merge(&mut p, &mut t, &mut t, ATA_TIMING_SETUP | ATA_TIMING_8BIT); if ata_dma_enabled(pair) != 0 { ata_timing_compute(pair, (*pair).dma_mode, &mut p, clock, 1); ata_timing_merge(&mut p, &mut t, &mut t, ATA_TIMING_SETUP | ATA_TIMING_8BIT); } }
    if (*adev).class != ATA_DEV_ATA { ali_fifo_control(ap, adev, 0); }
    ali_program_modes(ap, adev, &mut t, 0);
    if (*adev).class == ATA_DEV_ATA { ali_fifo_control(ap, adev, 5); }
}

unsafe fn ali_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    static UDMA_TIMING: [u8; 7] = [0xC, 0xB, 0xA, 0x9, 0x8, 0xF, 0xD];
    let pair = ata_dev_pair(adev); let mut t = ata_timing::default(); let clock: u64 = 1000000000 / 33333; let pdev = to_pci_dev((*(*ap).host).dev);
    if (*adev).class == ATA_DEV_ATA { ali_fifo_control(ap, adev, 8); }
    if (*adev).dma_mode >= XFER_UDMA_0 { ali_program_modes(ap, adev, core::ptr::null_mut(), UDMA_TIMING[((*adev).dma_mode - XFER_UDMA_0) as usize]); if (*adev).dma_mode >= XFER_UDMA_3 { let mut r = 0u8; pci_read_config_byte(pdev, 0x4B, &mut r); pci_write_config_byte(pdev, 0x4B, r | 1); } } else { ata_timing_compute(adev, (*adev).dma_mode, &mut t, clock, 1); if !pair.is_null() { let mut p = ata_timing::default(); ata_timing_compute(pair, (*pair).pio_mode, &mut p, clock, 1); ata_timing_merge(&mut p, &mut t, &mut t, ATA_TIMING_SETUP | ATA_TIMING_8BIT); if ata_dma_enabled(pair) != 0 { ata_timing_compute(pair, (*pair).dma_mode, &mut p, clock, 1); ata_timing_merge(&mut p, &mut t, &mut t, ATA_TIMING_SETUP | ATA_TIMING_8BIT); } } ali_program_modes(ap, adev, &mut t, 0); }
}

unsafe fn ali_warn_atapi_dma(adev: *mut ata_device) { let ehc = &(*(*adev).link).eh_context; let print_info = ehc.i.flags & ATA_EHI_PRINTINFO; if print_info != 0 && (*adev).class == ATA_DEV_ATAPI && ali_atapi_dma == 0 { ata_dev_warn(adev, "WARNING: ATAPI DMA disabled for reliability issues.  It can be enabled\n"); ata_dev_warn(adev, "WARNING: via pata_ali.atapi_dma modparam or corresponding sysfs node.\n"); } }
unsafe fn ali_lock_sectors(adev: *mut ata_device) { (*adev).max_sectors = 255; ali_warn_atapi_dma(adev); }
unsafe fn ali_check_atapi_dma(qc: *mut ata_queued_cmd) -> i32 { if ali_atapi_dma == 0 { return -EOPNOTSUPP; } if atapi_cmd_type((*qc).cdb[0]) == ATAPI_MISC { return -EOPNOTSUPP; } 0 }
unsafe fn ali_c2_c3_postreset(link: *mut ata_link, classes: *mut u32) { let mut r=0u8; let bit=4 << (*(*link).ap).port_no; if !ali_isa_bridge.is_null() { pci_read_config_byte(ali_isa_bridge,0x58,&mut r); r &= !(bit as u8); pci_write_config_byte(ali_isa_bridge,0x58,r); pci_write_config_byte(ali_isa_bridge,0x58,r | bit as u8); } ata_sff_postreset(link,classes); }

/* Port operation tables and PCI driver registration are direct translations;
 * their field types and external callback symbols are supplied by the ATA
 * and PCI subsystems. */
static ali_early_port_ops: ata_port_operations = ata_port_operations { inherits: &ata_sff_port_ops, cable_detect: Some(ata_cable_40wire), set_piomode: Some(ali_set_piomode), sff_data_xfer: Some(ata_sff_data_xfer32) };
static ali_dma_base_ops: ata_port_operations = ata_port_operations { inherits: &ata_bmdma32_port_ops, set_piomode: Some(ali_set_piomode), set_dmamode: Some(ali_set_dmamode) };
static ali_20_port_ops: ata_port_operations = ata_port_operations { inherits: &ali_dma_base_ops, cable_detect: Some(ata_cable_40wire), mode_filter: Some(ali_20_filter), check_atapi_dma: Some(ali_check_atapi_dma), dev_config: Some(ali_lock_sectors) };
static ali_c2_port_ops: ata_port_operations = ata_port_operations { inherits: &ali_dma_base_ops, check_atapi_dma: Some(ali_check_atapi_dma), cable_detect: Some(ali_c2_cable_detect), dev_config: Some(ali_lock_sectors), reset_postreset: Some(ali_c2_c3_postreset) };
static ali_c4_port_ops: ata_port_operations = ata_port_operations { inherits: &ali_dma_base_ops, check_atapi_dma: Some(ali_check_atapi_dma), cable_detect: Some(ali_c2_cable_detect), dev_config: Some(ali_lock_sectors) };
static ali_c5_port_ops: ata_port_operations = ata_port_operations { inherits: &ali_dma_base_ops, check_atapi_dma: Some(ali_check_atapi_dma), dev_config: Some(ali_warn_atapi_dma), cable_detect: Some(ali_c2_cable_detect) };

unsafe fn ali_init_chipset(pdev: *mut pci_dev) {
    let mut tmp = 0u8;
    if (*pdev).revision <= 0x20 { pci_read_config_byte(pdev,0x53,&mut tmp); pci_write_config_byte(pdev,0x53,tmp|3); }
    else { pci_read_config_byte(pdev,0x4a,&mut tmp); pci_write_config_byte(pdev,0x4a,tmp|0x20); pci_read_config_byte(pdev,0x4B,&mut tmp); if (*pdev).revision < 0xC2 { tmp &= 0x7F; } if (*pdev).revision >= 0xC2 { tmp |= 1; } pci_write_config_byte(pdev,0x4B,tmp|8); pci_read_config_byte(pdev,0x53,&mut tmp); if (*pdev).revision >= 0xC7 { tmp |= 3; } else { tmp |= 1; } pci_write_config_byte(pdev,0x53,tmp); }
    let north = pci_get_domain_bus_and_slot(pci_domain_nr((*pdev).bus),0,PCI_DEVFN(0,0));
    if !north.is_null() && (*north).vendor == PCI_VENDOR_ID_AL && !ali_isa_bridge.is_null() { pci_read_config_byte(ali_isa_bridge,0x79,&mut tmp); if (*pdev).revision == 0xC2 { pci_write_config_byte(ali_isa_bridge,0x79,tmp|4); } else if (*pdev).revision > 0xC2 && (*pdev).revision < 0xC5 { pci_write_config_byte(ali_isa_bridge,0x79,tmp|2); } }
    pci_dev_put(north); ata_pci_bmdma_clear_simplex(pdev);
}

unsafe fn ali_init_one(pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    let mut ppi: [*const ata_port_info; 2] = [core::ptr::null(), core::ptr::null()]; let mut tmp=0u8;
    let mut info = ata_port_info::default(); info.flags=ATA_FLAG_SLAVE_POSS; info.pio_mask=ATA_PIO4; info.port_ops=&ali_early_port_ops;
    let mut dma = info; dma.flags=ATA_FLAG_SLAVE_POSS|ATA_FLAG_PIO_LBA48|ATA_FLAG_IGN_SIMPLEX; dma.mwdma_mask=ATA_MWDMA2; dma.port_ops=&ali_20_port_ops;
    let mut c2=dma; c2.udma_mask=ATA_UDMA4; c2.port_ops=&ali_c2_port_ops; let mut c4=c2; c4.udma_mask=ATA_UDMA5; c4.port_ops=&ali_c4_port_ops; let mut c5=c4; c5.flags=ATA_FLAG_SLAVE_POSS|ATA_FLAG_IGN_SIMPLEX; c5.udma_mask=ATA_UDMA6; c5.port_ops=&ali_c5_port_ops;
    let rc=pcim_enable_device(pdev); if rc!=0{return rc;} if (*pdev).revision<0x20 {ppi[0]=&info;} else if (*pdev).revision<0xC2 {ppi[0]=&dma;} else if (*pdev).revision<0xC4 {ppi[0]=&c2;} else if (*pdev).revision==0xC4 {ppi[0]=&c4;} else {ppi[0]=&c5;} ali_init_chipset(pdev);
    if !ali_isa_bridge.is_null() && (*pdev).revision>=0x20 && (*pdev).revision<0xC2 { pci_read_config_byte(ali_isa_bridge,0x5E,&mut tmp); if tmp&0x1E==0x12 { ppi[0]=&dma; } }
    if (*ppi[0]).mwdma_mask==0 && (*ppi[0]).udma_mask==0 { ata_pci_sff_init_one(pdev,ppi.as_ptr(),core::ptr::null(),core::ptr::null_mut(),0) } else { ata_pci_bmdma_init_one(pdev,ppi.as_ptr(),core::ptr::null(),core::ptr::null_mut(),0) }
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn ali_reinit_one(pdev:*mut pci_dev)->i32 { let host=pci_get_drvdata(pdev); let rc=ata_pci_device_do_resume(pdev); if rc!=0{return rc;} ali_init_chipset(pdev); ata_host_resume(host); 0 }

static ali: [pci_device_id; 3] = [PCI_VDEVICE(PCI_VENDOR_ID_AL, PCI_DEVICE_ID_AL_M5228), PCI_VDEVICE(PCI_VENDOR_ID_AL, PCI_DEVICE_ID_AL_M5229), pci_device_id::default()];
static mut ali_pci_driver: pci_driver = pci_driver::default();
unsafe fn ali_init()->i32 { ali_isa_bridge=pci_get_device(PCI_VENDOR_ID_AL,PCI_DEVICE_ID_AL_M1533,core::ptr::null_mut()); let ret=pci_register_driver(&mut ali_pci_driver); if ret<0 {pci_dev_put(ali_isa_bridge);} ret }
unsafe fn ali_exit(){pci_unregister_driver(&mut ali_pci_driver); pci_dev_put(ali_isa_bridge);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
