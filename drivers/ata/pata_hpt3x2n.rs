// SPDX-License-Identifier: GPL-2.0-only
/* Libata driver for the HighPoint 371N, 372N, and 302N UDMA66 ATA controllers. */

const DRV_NAME: &[u8] = b"pata_hpt3x2n\0";
const DRV_VERSION: &[u8] = b"0.3.19\0";

enum { PCI66 = 1 << 1, USE_DPLL = 1 << 0 }

#[repr(C)]
struct hpt_clock { xfer_speed: u8, timing: u32 }

static mut hpt3x2n_clocks: [hpt_clock; 16] = [
    hpt_clock { xfer_speed: XFER_UDMA_7, timing: 0x1c869c62 },
    hpt_clock { xfer_speed: XFER_UDMA_6, timing: 0x1c869c62 },
    hpt_clock { xfer_speed: XFER_UDMA_5, timing: 0x1c8a9c62 },
    hpt_clock { xfer_speed: XFER_UDMA_4, timing: 0x1c8a9c62 },
    hpt_clock { xfer_speed: XFER_UDMA_3, timing: 0x1c8e9c62 },
    hpt_clock { xfer_speed: XFER_UDMA_2, timing: 0x1c929c62 },
    hpt_clock { xfer_speed: XFER_UDMA_1, timing: 0x1c9a9c62 },
    hpt_clock { xfer_speed: XFER_UDMA_0, timing: 0x1c829c62 },
    hpt_clock { xfer_speed: XFER_MW_DMA_2, timing: 0x2c829c62 },
    hpt_clock { xfer_speed: XFER_MW_DMA_1, timing: 0x2c829c66 },
    hpt_clock { xfer_speed: XFER_MW_DMA_0, timing: 0x2c829d2e },
    hpt_clock { xfer_speed: XFER_PIO_4, timing: 0x0c829c62 },
    hpt_clock { xfer_speed: XFER_PIO_3, timing: 0x0c829c84 },
    hpt_clock { xfer_speed: XFER_PIO_2, timing: 0x0c829ca6 },
    hpt_clock { xfer_speed: XFER_PIO_1, timing: 0x0d029d26 },
    hpt_clock { xfer_speed: XFER_PIO_0, timing: 0x0d029d5e },
];

unsafe fn hpt3x2n_find_mode(_ap: *mut ata_port, speed: i32) -> u32 {
    let mut clocks = hpt3x2n_clocks.as_mut_ptr();
    while (*clocks).xfer_speed != 0 {
        if (*clocks).xfer_speed as i32 == speed { return (*clocks).timing; }
        clocks = clocks.add(1);
    }
    BUG();
    0xffff_ffff
}

unsafe fn hpt372n_filter(adev: *mut ata_device, mut mask: u32) -> u32 {
    if ata_id_is_sata((*adev).id) { mask &= !((0xE << ATA_SHIFT_UDMA) | ATA_MASK_MWDMA); }
    mask
}

unsafe fn hpt3x2n_cable_detect(ap: *mut ata_port) -> i32 {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut scr2 = 0u8; let mut ata66 = 0u8;
    pci_read_config_byte(pdev, 0x5B, &mut scr2);
    pci_write_config_byte(pdev, 0x5B, scr2 & !0x01);
    udelay(10);
    pci_read_config_byte(pdev, 0x5A, &mut ata66);
    pci_write_config_byte(pdev, 0x5B, scr2);
    if ata66 & (2 >> (*ap).port_no) != 0 { ATA_CBL_PATA40 } else { ATA_CBL_PATA80 }
}

unsafe fn hpt3x2n_pre_reset(link: *mut ata_link, deadline: c_ulong) -> i32 {
    let ap = (*link).ap; let pdev = to_pci_dev((*(*ap).host).dev);
    let bits = [pci_bits { reg: 0x50, width: 1, mask: 0x04, val: 0x04 }, pci_bits { reg: 0x54, width: 1, mask: 0x04, val: 0x04 }];
    if pci_test_config_bits(pdev, &bits[(*ap).port_no]) == 0 { return -ENOENT; }
    pci_write_config_byte(pdev, 0x50 + 4 * (*ap).port_no, 0x37); udelay(100);
    let mut mcr2 = 0u8;
    pci_read_config_byte(pdev, 0x51 + 4 * (*ap).port_no, &mut mcr2);
    mcr2 &= !0x07; pci_write_config_byte(pdev, 0x51 + 4 * (*ap).port_no, mcr2);
    ata_sff_prereset(link, deadline)
}

unsafe fn hpt3x2n_set_mode(ap: *mut ata_port, adev: *mut ata_device, mode: u8) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let addr = 0x40 + 4 * ((*adev).devno + 2 * (*ap).port_no); let mask = if mode < XFER_MW_DMA_0 { 0xcfc3ffff } else if mode < XFER_UDMA_0 { 0x31c001ff } else { 0x303c0000 };
    let timing = hpt3x2n_find_mode(ap, mode as i32); let mut reg = 0u32;
    pci_read_config_dword(pdev, addr, &mut reg); reg = (reg & !mask) | (timing & mask); pci_write_config_dword(pdev, addr, reg);
}
unsafe fn hpt3x2n_set_piomode(ap: *mut ata_port, adev: *mut ata_device) { hpt3x2n_set_mode(ap, adev, (*adev).pio_mode); }
unsafe fn hpt3x2n_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) { hpt3x2n_set_mode(ap, adev, (*adev).dma_mode); }

unsafe fn hpt3x2n_bmdma_stop(qc: *mut ata_queued_cmd) {
    let ap = (*qc).ap; let pdev = to_pci_dev((*(*ap).host).dev); let mscreg = 0x50 + 4 * (*ap).port_no; let mut bwsr = 0u8; let mut msc = 0u8;
    pci_read_config_byte(pdev, 0x6A, &mut bwsr); pci_read_config_byte(pdev, mscreg, &mut msc);
    if bwsr & (1 << (*ap).port_no) != 0 { pci_write_config_byte(pdev, mscreg, msc | 0x30); } ata_bmdma_stop(qc);
}

unsafe fn hpt3x2n_set_clock(ap: *mut ata_port, source: i32) {
    let bmdma = (*ap).ioaddr.bmdma_addr.sub((*ap).port_no * 8);
    iowrite8(0x80, bmdma.add(0x73)); iowrite8(0x80, bmdma.add(0x77)); iowrite8(source as u8, bmdma.add(0x7B)); iowrite8(0xC0, bmdma.add(0x79));
    iowrite8(ioread8(bmdma.add(0x70)) | 0x32, bmdma.add(0x70)); iowrite8(ioread8(bmdma.add(0x74)) | 0x32, bmdma.add(0x74)); iowrite8(0, bmdma.add(0x79)); iowrite8(0, bmdma.add(0x73)); iowrite8(0, bmdma.add(0x77));
}
unsafe fn hpt3x2n_use_dpll(ap: *mut ata_port, writing: i32) -> i32 { let flags = (*(*ap).host).private_data as isize as i32; if writing != 0 || flags & PCI66 != 0 { USE_DPLL } else { 0 } }
unsafe fn hpt3x2n_qc_defer(qc: *mut ata_queued_cmd) -> i32 { let ap=(*qc).ap; let alt=(*(*ap).host).ports[(*ap).port_no ^ 1]; let flags=(*(*ap).host).private_data as isize as i32; let dpll=hpt3x2n_use_dpll(ap, ((*qc).tf.flags & ATA_TFLAG_WRITE) as i32); let rc=ata_std_qc_defer(qc); if rc != 0 { return rc; } if flags & USE_DPLL != dpll && (*alt).qc_active != 0 { return ATA_DEFER_PORT; } 0 }
unsafe fn hpt3x2n_qc_issue(qc: *mut ata_queued_cmd) -> u32 { let ap=(*qc).ap; let mut flags=(*(*ap).host).private_data as isize as i32; let dpll=hpt3x2n_use_dpll(ap, ((*qc).tf.flags & ATA_TFLAG_WRITE) as i32); if flags & USE_DPLL != dpll { flags=(flags & !USE_DPLL)|dpll; (*(*ap).host).private_data=flags as isize as *mut c_void; hpt3x2n_set_clock(ap, if dpll != 0 {0x21} else {0x23}); } ata_bmdma_qc_issue(qc) }

unsafe fn hpt3xn_calibrate_dpll(dev: *mut pci_dev) -> i32 {
    let mut reg5b=0u8; let mut reg5c=0u32; let mut tries=0;
    while tries < 0x5000 { udelay(50); pci_read_config_byte(dev,0x5b,&mut reg5b); if reg5b & 0x80 != 0 { tries=0; while tries < 0x1000 { pci_read_config_byte(dev,0x5b,&mut reg5b); if reg5b & 0x80 == 0 { return 0; } tries+=1; } pci_read_config_dword(dev,0x5c,&mut reg5c); pci_write_config_dword(dev,0x5c,reg5c & !0x100); return 1; } tries+=1; } 0
}
unsafe fn hpt3x2n_pci_clock(pdev:*mut pci_dev, base:u32)->u32 { let mut fcnt=inl(pci_resource_start(pdev)+0x90); if fcnt>>12 != 0xABCDE { let mut total=0u32; let mut sr=0u16; for _ in 0..128 { pci_read_config_word(pdev,0x78,&mut sr); total+= (sr&0x1ff) as u32; udelay(15); } fcnt=total/128; } fcnt&=0x1ff; let freq=(fcnt*base)/192; if freq<40 {33} else if freq<45 {40} else if freq<55 {50} else {66} }

#[repr(C)] struct pci_device_id { _private: [u8; 0] }
#[repr(C)] struct pci_driver { name:*const u8, id_table:*const pci_device_id, probe:unsafe extern "C" fn(*mut pci_dev,*const pci_device_id)->i32, remove:unsafe extern "C" fn(*mut pci_dev) }

unsafe extern "C" fn hpt3x2n_init_one(dev:*mut pci_dev, _id:*const pci_device_id)->i32 {
    let mut rc=pcim_enable_device(dev); if rc!=0{return rc;}
    let rev=(*dev).revision; let mut ppi=[&info_hpt3xxn as *const ata_port_info, core::ptr::null()];
    if (*dev).device==PCI_DEVICE_ID_TTI_HPT366 {if rev<6{return -ENODEV;} ppi[0]=&info_hpt372n;} else if (*dev).device==PCI_DEVICE_ID_TTI_HPT371 {if rev<2{return -ENODEV;}} else if (*dev).device==PCI_DEVICE_ID_TTI_HPT372 {if rev<2{return -ENODEV;} ppi[0]=&info_hpt372n;} else if (*dev).device==PCI_DEVICE_ID_TTI_HPT302 {if rev<2{return -ENODEV;}} else if (*dev).device==PCI_DEVICE_ID_TTI_HPT372N {ppi[0]=&info_hpt372n;} else{return -ENODEV;}
    pci_write_config_byte(dev,PCI_CACHE_LINE_SIZE,(L1_CACHE_BYTES/4) as u8); pci_write_config_byte(dev,PCI_LATENCY_TIMER,0x78); pci_write_config_byte(dev,PCI_MIN_GNT,8); pci_write_config_byte(dev,PCI_MAX_LAT,8);
    let mut irqmask=0; pci_read_config_byte(dev,0x5A,&mut irqmask); pci_write_config_byte(dev,0x5a,irqmask&!0x10);
    if (*dev).device==PCI_DEVICE_ID_TTI_HPT371 {let mut mcr1=0; pci_read_config_byte(dev,0x50,&mut mcr1); pci_write_config_byte(dev,0x50,mcr1&!4);}
    let pci_mhz=hpt3x2n_pci_clock(dev,77); let f_low=(pci_mhz*48)/66; let f_high=f_low+2; pci_write_config_dword(dev,0x5C,(f_high<<16)|f_low|0x100); pci_write_config_byte(dev,0x5B,0x21);
    let mut adjust=0; while adjust<8 {if hpt3xn_calibrate_dpll(dev)!=0{break;} pci_write_config_dword(dev,0x5C,(f_high<<16)|f_low); adjust+=1;} if adjust==8{return -ENODEV;}
    let hpriv=if pci_mhz>60 {(PCI66|USE_DPLL) as *mut c_void} else {USE_DPLL as *mut c_void}; if (*dev).device==PCI_DEVICE_ID_TTI_HPT371 {let iobase=pci_resource_start(dev,4); outb(inb(iobase+0x9c)|4,iobase+0x9c);} ata_pci_bmdma_init_one(dev,ppi.as_ptr(),&hpt3x2n_sht,hpriv,0)
}

static mut hpt3x2n_sht: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);
static mut hpt3x2n: [pci_device_id; 6] = [pci_device_id{_private:[]};6];
static mut hpt3x2n_pci_driver: pci_driver = pci_driver {name:DRV_NAME.as_ptr(),id_table:hpt3x2n.as_ptr(),probe:hpt3x2n_init_one,remove:ata_pci_remove_one};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
