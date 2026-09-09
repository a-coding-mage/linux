// SPDX-License-Identifier: GPL-2.0-only
/*
 *  pata_sch.c - Intel SCH PATA controllers
 *
 *  Copyright (c) 2008 Alek Du <alek.du@intel.com>
 */

// Supports Intel SCH (AF82US15W, AF82US15L, AF82UL11L) chipsets.

const DRV_NAME: &str = "pata_sch";
const DRV_VERSION: &str = "0.2";

const D0TIM: u32 = 0x80;
const D1TIM: u32 = 0x84;
const PM: u32 = 0x07;
const MDM: u32 = 0x03 << 8;
const UDM: u32 = 0x07 << 16;
const PPE: u32 = 1 << 30;
const USD: u32 = 1 << 31;

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_device_id {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ata_port {
    pub host: *mut ata_host,
}
#[repr(C)]
pub struct ata_host {
    pub dev: *mut device,
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ata_device {
    pub pio_mode: u32,
    pub devno: u8,
    pub class: u32,
    pub dma_mode: u32,
}
#[repr(C)]
pub struct ata_port_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct scsi_host_template {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ata_port_operations {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_driver {
    _private: [u8; 0],
}

extern "C" {
    static ata_bmdma_port_ops: ata_port_operations;
    static ata_cable_unknown: usize;
    static ata_pci_remove_one: usize;
    static ata_pci_device_suspend: usize;
    static ata_pci_device_resume: usize;

    fn pci_read_config_dword(dev: *mut pci_dev, port: u32, data: *mut u32);
    fn pci_write_config_dword(dev: *mut pci_dev, port: u32, data: u32);
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn ata_print_version_once(dev: *mut device, version: *const u8);
    fn ata_pci_bmdma_init_one(
        pdev: *mut pci_dev,
        ppi: *const *const ata_port_info,
        sht: *const scsi_host_template,
        private_data: *mut core::ffi::c_void,
        flags: u32,
    ) -> i32;
}

const XFER_PIO_0: u32 = 0;
const XFER_UDMA_0: u32 = 0x40;
const XFER_MW_DMA_0: u32 = 0x20;
const ATA_DEV_ATA: u32 = 0;

unsafe fn sch_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pio = (*adev).pio_mode.wrapping_sub(XFER_PIO_0);
    let dev = to_pci_dev((*(*ap).host).dev);
    let port = if (*adev).devno != 0 { D1TIM } else { D0TIM };
    let mut data: u32 = 0;

    pci_read_config_dword(dev, port, &mut data);
    data &= !(PM | PPE);
    data |= pio;
    if (*adev).class == ATA_DEV_ATA {
        data |= PPE;
    }
    pci_write_config_dword(dev, port, data);
}

unsafe fn sch_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let dma_mode = (*adev).dma_mode;
    let dev = to_pci_dev((*(*ap).host).dev);
    let port = if (*adev).devno != 0 { D1TIM } else { D0TIM };
    let mut data: u32 = 0;

    pci_read_config_dword(dev, port, &mut data);
    if dma_mode >= XFER_UDMA_0 {
        data |= USD;
        data &= !UDM;
        data |= dma_mode.wrapping_sub(XFER_UDMA_0) << 16;
    } else {
        data &= !(USD | MDM);
        data |= dma_mode.wrapping_sub(XFER_MW_DMA_0) << 8;
    }
    pci_write_config_dword(dev, port, data);
}

unsafe fn sch_init_one(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    let ppi: [*const ata_port_info; 2] = [core::ptr::addr_of!(SCH_PORT_INFO), core::ptr::null()];

    ata_print_version_once(pdev as *mut device, DRV_VERSION.as_ptr());
    ata_pci_bmdma_init_one(
        pdev,
        ppi.as_ptr(),
        core::ptr::addr_of!(SCH_SHT),
        core::ptr::null_mut(),
        0,
    )
}

static SCH_PCI_TBL: [pci_device_id; 2] = [
    pci_device_id { _private: [] },
    pci_device_id { _private: [] },
];
static SCH_SHT: scsi_host_template = scsi_host_template { _private: [] };
static SCH_PATA_OPS: ata_port_operations = ata_port_operations { _private: [] };
static SCH_PORT_INFO: ata_port_info = ata_port_info { _private: [] };
static SCH_PCI_DRIVER: pci_driver = pci_driver { _private: [] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
