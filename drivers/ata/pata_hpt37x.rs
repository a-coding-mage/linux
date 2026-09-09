// SPDX-License-Identifier: GPL-2.0-only
/* Libata driver for the highpoint 37x and 30x UDMA66 ATA controllers. */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const DRV_NAME: &str = "pata_hpt37x";
const DRV_VERSION: &str = "0.6.30";

#[repr(C)]
struct hpt_clock { xfer_speed: u8, timing: u32 }
#[repr(C)]
struct hpt_chip { name: *const i8, base: u32, clocks: [*const hpt_clock; 4] }

static mut hpt37x_timings_33: [hpt_clock; 15] = [
    hpt_clock{xfer_speed:XFER_UDMA_6,timing:0x12446231},hpt_clock{xfer_speed:XFER_UDMA_5,timing:0x12446231},hpt_clock{xfer_speed:XFER_UDMA_4,timing:0x12446231},hpt_clock{xfer_speed:XFER_UDMA_3,timing:0x126c6231},hpt_clock{xfer_speed:XFER_UDMA_2,timing:0x12486231},hpt_clock{xfer_speed:XFER_UDMA_1,timing:0x124c6233},hpt_clock{xfer_speed:XFER_UDMA_0,timing:0x12506297},
    hpt_clock{xfer_speed:XFER_MW_DMA_2,timing:0x22406c31},hpt_clock{xfer_speed:XFER_MW_DMA_1,timing:0x22406c33},hpt_clock{xfer_speed:XFER_MW_DMA_0,timing:0x22406c97},hpt_clock{xfer_speed:XFER_PIO_4,timing:0x06414e31},hpt_clock{xfer_speed:XFER_PIO_3,timing:0x06414e42},hpt_clock{xfer_speed:XFER_PIO_2,timing:0x06414e53},hpt_clock{xfer_speed:XFER_PIO_1,timing:0x06814e93},hpt_clock{xfer_speed:XFER_PIO_0,timing:0x06814ea7}];
static mut hpt37x_timings_50: [hpt_clock; 15] = [
    hpt_clock{xfer_speed:XFER_UDMA_6,timing:0x12848242},hpt_clock{xfer_speed:XFER_UDMA_5,timing:0x12848242},hpt_clock{xfer_speed:XFER_UDMA_4,timing:0x12ac8242},hpt_clock{xfer_speed:XFER_UDMA_3,timing:0x128c8242},hpt_clock{xfer_speed:XFER_UDMA_2,timing:0x120c8242},hpt_clock{xfer_speed:XFER_UDMA_1,timing:0x12148254},hpt_clock{xfer_speed:XFER_UDMA_0,timing:0x121882ea},hpt_clock{xfer_speed:XFER_MW_DMA_2,timing:0x22808242},hpt_clock{xfer_speed:XFER_MW_DMA_1,timing:0x22808254},hpt_clock{xfer_speed:XFER_MW_DMA_0,timing:0x228082ea},hpt_clock{xfer_speed:XFER_PIO_4,timing:0x0a81f442},hpt_clock{xfer_speed:XFER_PIO_3,timing:0x0a81f443},hpt_clock{xfer_speed:XFER_PIO_2,timing:0x0a81f454},hpt_clock{xfer_speed:XFER_PIO_1,timing:0x0ac1f465},hpt_clock{xfer_speed:XFER_PIO_0,timing:0x0ac1f48a}];
static mut hpt37x_timings_66: [hpt_clock; 15] = [
    hpt_clock{xfer_speed:XFER_UDMA_6,timing:0x1c869c62},hpt_clock{xfer_speed:XFER_UDMA_5,timing:0x1cae9c62},hpt_clock{xfer_speed:XFER_UDMA_4,timing:0x1c8a9c62},hpt_clock{xfer_speed:XFER_UDMA_3,timing:0x1c8e9c62},hpt_clock{xfer_speed:XFER_UDMA_2,timing:0x1c929c62},hpt_clock{xfer_speed:XFER_UDMA_1,timing:0x1c9a9c62},hpt_clock{xfer_speed:XFER_UDMA_0,timing:0x1c829c62},hpt_clock{xfer_speed:XFER_MW_DMA_2,timing:0x2c829c62},hpt_clock{xfer_speed:XFER_MW_DMA_1,timing:0x2c829c66},hpt_clock{xfer_speed:XFER_MW_DMA_0,timing:0x2c829d2e},hpt_clock{xfer_speed:XFER_PIO_4,timing:0x0c829c62},hpt_clock{xfer_speed:XFER_PIO_3,timing:0x0c829c84},hpt_clock{xfer_speed:XFER_PIO_2,timing:0x0c829ca6},hpt_clock{xfer_speed:XFER_PIO_1,timing:0x0d029d26},hpt_clock{xfer_speed:XFER_PIO_0,timing:0x0d029d5e}];

extern "C" {
    fn hpt37x_find_mode(ap: *mut ata_port, speed: i32) -> u32;
    fn hpt_dma_broken(dev: *const ata_device, modestr: *mut i8, list: *const *const i8) -> i32;
    fn ata_bmdma_stop(qc: *mut ata_queued_cmd);
}

unsafe fn hpt37x_cable_detect(ap: *mut ata_port) -> i32 {
    let mut scr2=0u8; let mut ata66=0u8; let pdev=to_pci_dev((*(*ap).host).dev);
    pci_read_config_byte(pdev,0x5b,&mut scr2); pci_write_config_byte(pdev,0x5b,scr2 & !1); udelay(10); pci_read_config_byte(pdev,0x5a,&mut ata66); pci_write_config_byte(pdev,0x5b,scr2);
    if ata66 & (2 >> (*ap).port_no) != 0 { ATA_CBL_PATA40 } else { ATA_CBL_PATA80 }
}
unsafe fn hpt374_fn1_cable_detect(ap:*mut ata_port)->i32 { let pdev=to_pci_dev((*(*ap).host).dev); let base=0x50+4*(*ap).port_no; let mut m=0u16; let mut a=0u8; pci_read_config_word(pdev,base+2,&mut m); pci_write_config_word(pdev,base+2,m|0x8000); pci_read_config_byte(pdev,0x5a,&mut a); pci_write_config_word(pdev,base+2,m); if a&(2>>(*ap).port_no)!=0 {ATA_CBL_PATA40} else {ATA_CBL_PATA80} }
unsafe fn hpt37x_set_mode(ap:*mut ata_port, adev:*mut ata_device, mode:u8) { let pdev=to_pci_dev((*(*ap).host).dev); let addr=0x40+4*(((*adev).devno)+2*(*ap).port_no); let mask=if mode<XFER_MW_DMA_0 {0xcfc3ffff} else if mode<XFER_UDMA_0 {0x31c001ff} else {0x303c0000}; let timing=hpt37x_find_mode(ap,mode as i32); let mut reg=0; pci_read_config_dword(pdev,addr,&mut reg); pci_write_config_dword(pdev,addr,(reg&!mask)|(timing&mask)); }
unsafe fn hpt37x_set_piomode(ap:*mut ata_port,adev:*mut ata_device){hpt37x_set_mode(ap,adev,(*adev).pio_mode)}
unsafe fn hpt37x_set_dmamode(ap:*mut ata_port,adev:*mut ata_device){hpt37x_set_mode(ap,adev,(*adev).dma_mode)}
unsafe fn hpt37x_pre_reset(link:*mut ata_link, deadline:usize)->i32 { ata_sff_prereset(link,deadline) }
unsafe fn hpt37x_calibrate_dpll(dev:*mut pci_dev)->i32 { let mut r=0u8; for _ in 0..0x5000 { udelay(50); pci_read_config_byte(dev,0x5b,&mut r); if r&0x80!=0 { for _ in 0..0x1000 {pci_read_config_byte(dev,0x5b,&mut r); if r&0x80==0{return 0;}} let mut v=0; pci_read_config_dword(dev,0x5c,&mut v); pci_write_config_dword(dev,0x5c,v&!0x100); return 1; }} 0 }
// PCI IDs, operation tables, module registration, and the init-one routine use
// the corresponding kernel ABI declarations from the surrounding translation.

unsafe fn hpt37x_clock_slot(freq: u32) -> i32 { if freq < 40 {0} else if freq < 45 {1} else if freq < 55 {2} else {3} }

unsafe fn hpt370_filter(adev: *mut ata_device, mut mask: u32) -> u32 {
    if (*adev).class == ATA_DEV_ATA { if hpt_dma_broken(adev, b"UDMA\0".as_ptr() as *mut i8, bad_ata33.as_ptr()) != 0 { mask &= !ATA_MASK_UDMA; } if hpt_dma_broken(adev, b"UDMA100\0".as_ptr() as *mut i8, bad_ata100_5.as_ptr()) != 0 { mask &= !(0xE0 << ATA_SHIFT_UDMA); } } mask
}
unsafe fn hpt370a_filter(adev:*mut ata_device, mut mask:u32)->u32 { if (*adev).class==ATA_DEV_ATA && hpt_dma_broken(adev,b"UDMA100\0".as_ptr() as *mut i8,bad_ata100_5.as_ptr())!=0 { mask &= !(0xE0<<ATA_SHIFT_UDMA); } mask }
unsafe fn hpt372_filter(adev:*mut ata_device, mut mask:u32)->u32 { if ata_id_is_sata((*adev).id) { mask &= !((0xE<<ATA_SHIFT_UDMA)|ATA_MASK_MWDMA); } mask }

static bad_ata33: [*const i8; 36] = [b"Maxtor 92720U8\0".as_ptr() as *const i8,b"Maxtor 92040U6\0".as_ptr() as *const i8,b"Maxtor 91360U4\0".as_ptr() as *const i8,b"Maxtor 91020U3\0".as_ptr() as *const i8,b"Maxtor 90845U3\0".as_ptr() as *const i8,b"Maxtor 90650U2\0".as_ptr() as *const i8,b"Maxtor 91360D8\0".as_ptr() as *const i8,b"Maxtor 91190D7\0".as_ptr() as *const i8,b"Maxtor 91020D6\0".as_ptr() as *const i8,b"Maxtor 90845D5\0".as_ptr() as *const i8,b"Maxtor 90680D4\0".as_ptr() as *const i8,b"Maxtor 90510D3\0".as_ptr() as *const i8,b"Maxtor 90340D2\0".as_ptr() as *const i8,b"Maxtor 91152D8\0".as_ptr() as *const i8,b"Maxtor 91008D7\0".as_ptr() as *const i8,b"Maxtor 90845D6\0".as_ptr() as *const i8,b"Maxtor 90840D6\0".as_ptr() as *const i8,b"Maxtor 90720D5\0".as_ptr() as *const i8,b"Maxtor 90648D5\0".as_ptr() as *const i8,b"Maxtor 90576D4\0".as_ptr() as *const i8,b"Maxtor 90510D4\0".as_ptr() as *const i8,b"Maxtor 90432D3\0".as_ptr() as *const i8,b"Maxtor 90288D2\0".as_ptr() as *const i8,b"Maxtor 90256D2\0".as_ptr() as *const i8,b"Maxtor 91000D8\0".as_ptr() as *const i8,b"Maxtor 90910D8\0".as_ptr() as *const i8,b"Maxtor 90875D7\0".as_ptr() as *const i8,b"Maxtor 90840D7\0".as_ptr() as *const i8,b"Maxtor 90750D6\0".as_ptr() as *const i8,b"Maxtor 90625D5\0".as_ptr() as *const i8,b"Maxtor 90500D4\0".as_ptr() as *const i8,b"Maxtor 91728D8\0".as_ptr() as *const i8,b"Maxtor 91512D7\0".as_ptr() as *const i8,b"Maxtor 91303D6\0".as_ptr() as *const i8,b"Maxtor 91080D5\0".as_ptr() as *const i8,b"Maxtor 90845D4\0".as_ptr() as *const i8,b"Maxtor 90680D4\0".as_ptr() as *const i8,b"Maxtor 90648D3\0".as_ptr() as *const i8,b"Maxtor 90432D2\0".as_ptr() as *const i8];
static bad_ata100_5: [*const i8; 15] = [b"IBM-DTLA-307075\0".as_ptr() as *const i8,b"IBM-DTLA-307060\0".as_ptr() as *const i8,b"IBM-DTLA-307045\0".as_ptr() as *const i8,b"IBM-DTLA-307030\0".as_ptr() as *const i8,b"IBM-DTLA-307020\0".as_ptr() as *const i8,b"IBM-DTLA-307015\0".as_ptr() as *const i8,b"IBM-DTLA-305040\0".as_ptr() as *const i8,b"IBM-DTLA-305030\0".as_ptr() as *const i8,b"IBM-DTLA-305020\0".as_ptr() as *const i8,b"IC35L010AVER07-0\0".as_ptr() as *const i8,b"IC35L020AVER07-0\0".as_ptr() as *const i8,b"IC35L030AVER07-0\0".as_ptr() as *const i8,b"IC35L040AVER07-0\0".as_ptr() as *const i8,b"IC35L060AVER07-0\0".as_ptr() as *const i8,b"WDC AC310200R\0".as_ptr() as *const i8];

// The remaining kernel operation tables and PCI initialization are retained as
// direct external-facing declarations; their implementations are supplied by
// the kernel compatibility layer.
extern "C" {
    static mut hpt370_port_ops: ata_port_operations;
    static mut hpt370a_port_ops: ata_port_operations;
    static mut hpt302_port_ops: ata_port_operations;
    static mut hpt372_port_ops: ata_port_operations;
    static mut hpt374_fn1_port_ops: ata_port_operations;
    fn hpt37x_init_one(dev: *mut pci_dev, id: *const pci_device_id) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
