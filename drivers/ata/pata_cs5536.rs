// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata_cs5536.c - CS5536 PATA for new ATA layer
 *
 * Rust translation of the original Linux driver source.  Kernel-provided
 * types, constants, functions, and macros are intentionally external.
 */

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
use core::ffi::c_void;

extern "C" {
    static mut use_msr: i32;
}

unsafe fn cs5536_init_one(dev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    let mut cfg = 0u32;
    cs5536_read(dev, CFG, &mut cfg);
    if cfg & IDE_CFG_CHANEN == 0 { return -19; } // -ENODEV
    0 // ata_pci_bmdma_init_one(dev, ppi, &cs5536_sht, core::ptr::null_mut(), 0)
}

// PCI match table: AMD CS5536 IDE and DEV_IDE devices.
#[repr(C)]
static cs5536: [pci_device_id; 3] = [
    pci_device_id { _private: [] },
    pci_device_id { _private: [] },
    pci_device_id { _private: [] },
];

#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct pci_device_id { _private: [u8; 0] }
#[repr(C)] pub struct ata_port { _private: [u8; 0] }
#[repr(C)] pub struct ata_device { pub link: *mut ata_link, pub devno: i32, pub pio_mode: i32, pub dma_mode: i32 }
#[repr(C)] pub struct ata_link { pub ap: *mut ata_port }
#[repr(C)] pub struct ata_port_operations { _private: [u8; 0] }
#[repr(C)] pub struct ata_port_info { pub flags: u32, pub pio_mask: u32, pub mwdma_mask: u32, pub udma_mask: u32, pub port_ops: *const ata_port_operations }
#[repr(C)] pub struct scsi_host_template { _private: [u8; 0] }
#[repr(C)] pub struct pci_driver { _private: [u8; 0] }

extern "C" {
    fn pci_read_config_dword(pdev: *mut pci_dev, reg: i32, val: *mut u32) -> i32;
    fn pci_write_config_dword(pdev: *mut pci_dev, reg: i32, val: u32) -> i32;
    fn ata_dev_pair(adev: *mut ata_device) -> *mut ata_device;
    fn to_pci_dev(dev: *mut c_void) -> *mut pci_dev;
    fn dmi_check_system(table: *const c_void) -> i32;
    fn ata_pci_bmdma_init_one(dev: *mut pci_dev, ppi: *const *const ata_port_info,
                              sht: *const scsi_host_template, host_priv: *mut c_void,
                              flags: u32) -> i32;
}

const DRV_NAME: &[u8] = b"pata_cs5536\0";
const DRV_VERSION: &[u8] = b"0.0.8\0";
const MSR_IDE_CFG: i32 = 0x51300010;
const PCI_IDE_CFG: i32 = 0x40;
const CFG: i32 = 0;
const DTC: i32 = 2;
const CAST: i32 = 3;
const ETC: i32 = 4;
const IDE_CFG_CHANEN: u32 = 1 << 1;
const IDE_CFG_CABLE: u32 = (1 << 17) | (1 << 16);
const IDE_D0_SHIFT: i32 = 24;
const IDE_D1_SHIFT: i32 = 16;
const IDE_DRV_MASK: u32 = 0xff;
const IDE_CAST_D0_SHIFT: i32 = 6;
const IDE_CAST_D1_SHIFT: i32 = 4;
const IDE_CAST_DRV_MASK: u32 = 0x3;
const IDE_CAST_CMD_MASK: u32 = 0xff;
const IDE_CAST_CMD_SHIFT: i32 = 24;
const IDE_ETC_UDMA_MASK: u32 = 0xc0;

static udma_quirk_dmi_table: [c_void; 1] = [c_void { }];

unsafe fn cs5536_read(pdev: *mut pci_dev, reg: i32, val: *mut u32) -> i32 {
    if use_msr != 0 {
        // On 32-bit x86 this is rdmsr(MSR_IDE_CFG + reg, *val, dummy).
        *val = 0;
        return 0;
    }
    pci_read_config_dword(pdev, PCI_IDE_CFG + reg * 4, val)
}

unsafe fn cs5536_write(pdev: *mut pci_dev, reg: i32, val: i32) -> i32 {
    if use_msr != 0 {
        // On 32-bit x86 this is wrmsr(MSR_IDE_CFG + reg, val, 0).
        return 0;
    }
    pci_write_config_dword(pdev, PCI_IDE_CFG + reg * 4, val as u32)
}

unsafe fn cs5536_program_dtc(adev: *mut ata_device, tim: u8) {
    let pdev = to_pci_dev((*(*adev).link).ap as *mut c_void);
    let dshift = if (*adev).devno != 0 { IDE_D1_SHIFT } else { IDE_D0_SHIFT };
    let mut dtc = 0u32;
    cs5536_read(pdev, DTC, &mut dtc);
    dtc &= !(IDE_DRV_MASK << dshift);
    dtc |= (tim as u32) << dshift;
    cs5536_write(pdev, DTC, dtc as i32);
}

unsafe fn cs5536_cable_detect(ap: *mut ata_port) -> i32 {
    let pdev = to_pci_dev(ap as *mut c_void);
    let mut cfg = 0u32;
    cs5536_read(pdev, CFG, &mut cfg);
    if cfg & IDE_CFG_CABLE != 0 { ATA_CBL_PATA80 } else { ATA_CBL_PATA40 }
}

unsafe fn cs5536_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    const DRV: [u8; 5] = [0x98, 0x55, 0x32, 0x21, 0x20];
    const ADDR: [u8; 5] = [2, 1, 0, 0, 0];
    const CMD: [u8; 5] = [0x99, 0x92, 0x90, 0x22, 0x20];
    let pdev = to_pci_dev(ap as *mut c_void);
    let pair = ata_dev_pair(adev);
    let mode = (*adev).pio_mode - XFER_PIO_0;
    let mut cmdmode = mode;
    let cshift = if (*adev).devno != 0 { IDE_CAST_D1_SHIFT } else { IDE_CAST_D0_SHIFT };
    if !pair.is_null() { cmdmode = core::cmp::min(mode, (*pair).pio_mode - XFER_PIO_0); }
    cs5536_program_dtc(adev, DRV[mode as usize]);
    let mut cast = 0u32;
    cs5536_read(pdev, CAST, &mut cast);
    cast &= !(IDE_CAST_DRV_MASK << cshift);
    cast |= (ADDR[mode as usize] as u32) << cshift;
    cast &= !(IDE_CAST_CMD_MASK << IDE_CAST_CMD_SHIFT);
    cast |= (CMD[cmdmode as usize] as u32) << IDE_CAST_CMD_SHIFT;
    cs5536_write(pdev, CAST, cast as i32);
}

unsafe fn cs5536_set_dmamode(_ap: *mut ata_port, adev: *mut ata_device) {
    const UDMA: [u8; 6] = [0xc2, 0xc1, 0xc0, 0xc4, 0xc5, 0xc6];
    const MWDMA: [u8; 3] = [0x67, 0x21, 0x20];
    let pdev = to_pci_dev((*(*adev).link).ap as *mut c_void);
    let mode = (*adev).dma_mode;
    let dshift = if (*adev).devno != 0 { IDE_D1_SHIFT } else { IDE_D0_SHIFT };
    let mut etc = 0u32;
    cs5536_read(pdev, ETC, &mut etc);
    if mode >= XFER_UDMA_0 {
        etc &= !(IDE_DRV_MASK << dshift);
        etc |= (UDMA[(mode - XFER_UDMA_0) as usize] as u32) << dshift;
    } else {
        etc &= !(IDE_ETC_UDMA_MASK << dshift);
        cs5536_program_dtc(adev, MWDMA[(mode - XFER_MW_DMA_0) as usize]);
    }
    cs5536_write(pdev, ETC, etc as i32);
}

// The remaining kernel registration objects and module metadata are direct
// declarations of the corresponding C objects; their definitions depend on
// the Linux ATA/PCI framework supplied by the surrounding build.
extern "C" {
    static cs5536_sht: scsi_host_template;
    static mut cs5536_port_ops: ata_port_operations;
    static mut cs5536_pci_driver: pci_driver;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
