// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata_atiixp.c  - ATI PATA for new ATA layer
 *                 (C) 2005 Red Hat Inc
 *                 (C) 2009-2010 Bartlomiej Zolnierkiewicz
 *
 * Based on linux/drivers/ide/pci/atiixp.c
 */

// C header dependencies are supplied by the surrounding kernel translation.

const DRV_NAME: &str = "pata_atiixp";
const DRV_VERSION: &str = "0.4.6";

const ATIIXP_IDE_PIO_TIMING: u32 = 0x40;
const ATIIXP_IDE_MWDMA_TIMING: u32 = 0x44;
const ATIIXP_IDE_PIO_CONTROL: u32 = 0x48;
const ATIIXP_IDE_PIO_MODE: u32 = 0x4a;
const ATIIXP_IDE_UDMA_CONTROL: u32 = 0x54;
const ATIIXP_IDE_UDMA_MODE: u32 = 0x56;

extern "C" {
    static attixp_cable_override_dmi_table: [dmi_system_id; 2];
    static atiixp_lock: spinlock_t;
    static ata_bmdma_port_ops: ata_port_operations;
    static ata_dummy_port_info: ata_port_info;
}

#[repr(C)]
struct dmi_system_id {
    ident: *const core::ffi::c_char,
    matches: [dmi_match; 3],
}

#[repr(C)]
struct dmi_match {
    slot: u32,
    substr: *const core::ffi::c_char,
}

#[repr(C)]
struct pci_bits {
    reg: u8,
    width: u8,
    mask: u32,
    val: u32,
}

extern "C" {
    fn to_pci_dev(dev: *mut core::ffi::c_void) -> *mut pci_dev;
    fn dmi_check_system(table: *const dmi_system_id) -> i32;
    fn pci_read_config_byte(dev: *mut pci_dev, where_: u32, val: *mut u8) -> i32;
    fn pci_read_config_word(dev: *mut pci_dev, where_: u32, val: *mut u16) -> i32;
    fn pci_write_config_word(dev: *mut pci_dev, where_: u32, val: u16) -> i32;
    fn pci_read_config_dword(dev: *mut pci_dev, where_: u32, val: *mut u32) -> i32;
    fn pci_write_config_dword(dev: *mut pci_dev, where_: u32, val: u32) -> i32;
    fn pci_test_config_bits(dev: *mut pci_dev, bits: *const pci_bits) -> bool;
    fn ata_sff_prereset(link: *mut ata_link, deadline: c_ulong) -> i32;
    fn ata_using_udma(dev: *mut ata_device) -> bool;
    fn ata_bmdma_start(qc: *mut ata_queued_cmd);
    fn ata_bmdma_stop(qc: *mut ata_queued_cmd);
    fn ata_bmdma_dumb_qc_prep(qc: *mut ata_queued_cmd);
    fn ata_pci_bmdma_init_one(pdev: *mut pci_dev, ppi: *const *const ata_port_info,
                              sht: *const scsi_host_template, private_data: *mut core::ffi::c_void,
                              flags: u32) -> i32;
    fn ata_pci_remove_one(pdev: *mut pci_dev);
    fn ata_pci_device_resume(dev: *mut pci_dev) -> i32;
    fn ata_pci_device_suspend(dev: *mut pci_dev, state: u32) -> i32;
    fn spin_lock_irqsave(lock: *const spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *const spinlock_t, flags: c_ulong);
}

type c_ulong = usize;
type c_long = isize;
type c_ulonglong = u64;
type c_int = i32;

#[repr(C)] struct pci_dev { device: u16, host: *mut core::ffi::c_void }
#[repr(C)] struct ata_port { host: *mut ata_host, port_no: u32 }
#[repr(C)] struct ata_host { dev: *mut core::ffi::c_void }
#[repr(C)] struct ata_link { ap: *mut ata_port }
#[repr(C)] struct ata_device { devno: u32, pio_mode: i32, dma_mode: i32 }
#[repr(C)] struct ata_queued_cmd { ap: *mut ata_port, dev: *mut ata_device }
#[repr(C)] struct spinlock_t { _private: [u8; 0] }
#[repr(C)] struct scsi_host_template { sg_tablesize: u32, dma_boundary: u32 }
#[repr(C)] struct ata_port_operations {
    inherits: *const ata_port_operations,
    qc_prep: Option<unsafe extern "C" fn(*mut ata_queued_cmd)>,
    bmdma_start: Option<unsafe extern "C" fn(*mut ata_queued_cmd)>,
    bmdma_stop: Option<unsafe extern "C" fn(*mut ata_queued_cmd)>,
    prereset: Option<unsafe extern "C" fn(*mut ata_link, c_ulong) -> i32>,
    cable_detect: Option<unsafe extern "C" fn(*mut ata_port) -> i32>,
    set_piomode: Option<unsafe extern "C" fn(*mut ata_port, *mut ata_device)>,
    set_dmamode: Option<unsafe extern "C" fn(*mut ata_port, *mut ata_device)>,
}
#[repr(C)] struct ata_port_info { flags: u32, pio_mask: u32, mwdma_mask: u32, udma_mask: u32, port_ops: *const ata_port_operations }
#[repr(C)] struct pci_device_id { vendor: u32, device: u32 }
#[repr(C)] struct pci_driver { name: *const core::ffi::c_char, id_table: *const pci_device_id }

const XFER_PIO_0: i32 = 0;
const XFER_UDMA_0: i32 = 0x40;
const XFER_MW_DMA_0: i32 = 0x20;
const XFER_MW_DMA_1: i32 = XFER_MW_DMA_0 + 1;
const XFER_MW_DMA_2: i32 = XFER_MW_DMA_0 + 2;
const ATA_CBL_PATA40_SHORT: i32 = 3;
const ATA_CBL_PATA80: i32 = 2;
const ATA_CBL_PATA40: i32 = 1;
const ENOENT: i32 = 2;

unsafe extern "C" fn atiixp_cable_detect(ap: *mut ata_port) -> i32 {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut udma = 0u8;
    if dmi_check_system(attixp_cable_override_dmi_table.as_ptr()) != 0 { return ATA_CBL_PATA40_SHORT; }
    pci_read_config_byte(pdev, ATIIXP_IDE_UDMA_MODE + (*ap).port_no, &mut udma);
    if (udma & 0x07) >= 0x04 || (udma & 0x70) >= 0x40 { ATA_CBL_PATA80 } else { ATA_CBL_PATA40 }
}

unsafe extern "C" fn atiixp_prereset(link: *mut ata_link, deadline: c_ulong) -> i32 {
    let bits = [pci_bits { reg: 0x48, width: 1, mask: 0x01, val: 0 }, pci_bits { reg: 0x48, width: 1, mask: 0x08, val: 0 }];
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);
    if !pci_test_config_bits(pdev, &bits[(*ap).port_no as usize]) { return -ENOENT; }
    ata_sff_prereset(link, deadline)
}

unsafe fn atiixp_set_pio_timing(ap: *mut ata_port, adev: *mut ata_device, pio: i32) {
    let timings = [0x5du32, 0x47, 0x34, 0x22, 0x20];
    let pdev = to_pci_dev((*(*ap).host).dev);
    let dn = 2 * (*ap).port_no as i32 + (*adev).devno as i32;
    let shift = 16 * (*ap).port_no + 8 * ((*adev).devno ^ 1);
    let mut mode = 0u16;
    pci_read_config_word(pdev, ATIIXP_IDE_PIO_MODE, &mut mode);
    mode &= !(0x7 << (4 * dn)); mode |= (pio as u16) << (4 * dn);
    pci_write_config_word(pdev, ATIIXP_IDE_PIO_MODE, mode);
    let mut timing = 0u32;
    pci_read_config_dword(pdev, ATIIXP_IDE_PIO_TIMING, &mut timing);
    timing &= !(0xff << shift); timing |= timings[pio as usize] << shift;
    pci_write_config_dword(pdev, ATIIXP_IDE_PIO_TIMING, timing);
}

unsafe extern "C" fn atiixp_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let mut flags = 0usize; spin_lock_irqsave(&atiixp_lock, &mut flags);
    atiixp_set_pio_timing(ap, adev, (*adev).pio_mode - XFER_PIO_0);
    spin_unlock_irqrestore(&atiixp_lock, flags);
}

unsafe extern "C" fn atiixp_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let timings = [0x77u32, 0x21, 0x20]; let pdev = to_pci_dev((*(*ap).host).dev);
    let mut dma = (*adev).dma_mode; let dn = 2 * (*ap).port_no as i32 + (*adev).devno as i32;
    let mut flags = 0usize; spin_lock_irqsave(&atiixp_lock, &mut flags);
    if (*adev).dma_mode >= XFER_UDMA_0 { dma -= XFER_UDMA_0; let mut v=0u16; pci_read_config_word(pdev, ATIIXP_IDE_UDMA_MODE, &mut v); v &= !(0x7 << (4*dn)); v |= (dma as u16) << (4*dn); pci_write_config_word(pdev, ATIIXP_IDE_UDMA_MODE, v); }
    else { let shift=16*(*ap).port_no+8*((*adev).devno^1); dma-=XFER_MW_DMA_0; let mut v=0u32; pci_read_config_dword(pdev, ATIIXP_IDE_MWDMA_TIMING, &mut v); v &= !(0xff<<shift); v |= timings[dma as usize]<<shift; pci_write_config_dword(pdev, ATIIXP_IDE_MWDMA_TIMING,v); }
    let wanted = if (*adev).dma_mode >= XFER_MW_DMA_2 {4} else if (*adev).dma_mode == XFER_MW_DMA_1 {3} else if (*adev).dma_mode == XFER_MW_DMA_0 {0} else { panic!("BUG") };
    if (*adev).pio_mode != wanted { atiixp_set_pio_timing(ap, adev, wanted); }
    spin_unlock_irqrestore(&atiixp_lock, flags);
}

unsafe extern "C" fn atiixp_bmdma_start(qc: *mut ata_queued_cmd) { let ap=(*qc).ap; let adev=(*qc).dev; let pdev=to_pci_dev((*(*ap).host).dev); let dn=2*(*ap).port_no+(*adev).devno; let mut v=0u16; pci_read_config_word(pdev,ATIIXP_IDE_UDMA_CONTROL,&mut v); if ata_using_udma(adev){v|=1<<dn}else{v&=!(1<<dn)} pci_write_config_word(pdev,ATIIXP_IDE_UDMA_CONTROL,v); ata_bmdma_start(qc); }
unsafe extern "C" fn atiixp_bmdma_stop(qc: *mut ata_queued_cmd) { let ap=(*qc).ap; let pdev=to_pci_dev((*(*ap).host).dev); let dn=2*(*ap).port_no+(*(*qc).dev).devno; let mut v=0u16; pci_read_config_word(pdev,ATIIXP_IDE_UDMA_CONTROL,&mut v); v&=!(1<<dn); pci_write_config_word(pdev,ATIIXP_IDE_UDMA_CONTROL,v); ata_bmdma_stop(qc); }

// The remaining static driver registration mirrors the C aggregate definitions.
static ATI_IIXP_SHT: scsi_host_template = scsi_host_template { sg_tablesize: 0, dma_boundary: 0 };

static ATI_IIXP_PORT_OPS: ata_port_operations = ata_port_operations {
    inherits: unsafe { &ata_bmdma_port_ops }, qc_prep: Some(ata_bmdma_dumb_qc_prep),
    bmdma_start: Some(atiixp_bmdma_start), bmdma_stop: Some(atiixp_bmdma_stop),
    prereset: Some(atiixp_prereset), cable_detect: Some(atiixp_cable_detect),
    set_piomode: Some(atiixp_set_piomode), set_dmamode: Some(atiixp_set_dmamode),
};

unsafe extern "C" fn atiixp_init_one(pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    static INFO: ata_port_info = ata_port_info { flags: 1, pio_mask: 4, mwdma_mask: 3, udma_mask: 5, port_ops: &ATI_IIXP_PORT_OPS };
    let mut ppi = [&INFO as *const ata_port_info, &INFO as *const ata_port_info];
    if (*pdev).device == 0x438c { ppi[1] = &ata_dummy_port_info; }
    ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &ATI_IIXP_SHT, core::ptr::null_mut(), 1)
}

static ATI_IIXP: [pci_device_id; 7] = [
    pci_device_id { vendor: 0x1002, device: 0x4376 }, pci_device_id { vendor: 0x1002, device: 0x4370 },
    pci_device_id { vendor: 0x1002, device: 0x4379 }, pci_device_id { vendor: 0x1002, device: 0x438c },
    pci_device_id { vendor: 0x1002, device: 0x439c }, pci_device_id { vendor: 0x1022, device: 0x7800 },
    pci_device_id { vendor: 0, device: 0 },
];

static ATI_IIXP_PCI_DRIVER: pci_driver = pci_driver { name: b"pata_atiixp\0".as_ptr() as *const _, id_table: ATI_IIXP.as_ptr() };

// module_pci_driver(atiixp_pci_driver)
// MODULE_AUTHOR("Alan Cox");
// MODULE_DESCRIPTION("low-level driver for ATI IXP200/300/400");
// MODULE_LICENSE("GPL");
// MODULE_DEVICE_TABLE(pci, atiixp);
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
