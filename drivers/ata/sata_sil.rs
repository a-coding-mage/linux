// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  sata_sil.c - Silicon Image SATA
 *
 *  Maintained by:  Tejun Heo <tj@kernel.org>
 *                  Please ALWAYS copy linux-ide@vger.kernel.org
 *
 *  Copyright 2003-2005 Red Hat, Inc.
 *  Copyright 2003 Benjamin Herrenschmidt
 *
 *  libata documentation is available via 'make {ps|pdf}docs',
 *  as Documentation/driver-api/libata.rst
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Kernel headers and build-time configuration are supplied by other files.

pub const DRV_NAME: &str = "sata_sil";
pub const DRV_VERSION: &str = "2.4";
pub const SIL_DMA_BOUNDARY: c_ulong = 0x7fffffff;

pub const SIL_MMIO_BAR: c_uint = 5;
pub const SIL_FLAG_NO_SATA_IRQ: c_uint = 1 << 28;
pub const SIL_FLAG_RERR_ON_DMA_ACT: c_uint = 1 << 29;
pub const SIL_FLAG_MOD15WRITE: c_uint = 1 << 30;
pub const SIL_DFL_PORT_FLAGS: c_uint = ATA_FLAG_SATA;

pub const sil_3112: c_uint = 0;
pub const sil_3112_no_sata_irq: c_uint = 1;
pub const sil_3512: c_uint = 2;
pub const sil_3114: c_uint = 3;

pub const SIL_SYSCFG: c_uint = 0x48;
pub const SIL_MASK_IDE0_INT: c_uint = 1 << 22;
pub const SIL_MASK_IDE1_INT: c_uint = 1 << 23;
pub const SIL_MASK_IDE2_INT: c_uint = 1 << 24;
pub const SIL_MASK_IDE3_INT: c_uint = 1 << 25;
pub const SIL_MASK_2PORT: c_uint = SIL_MASK_IDE0_INT | SIL_MASK_IDE1_INT;
pub const SIL_MASK_4PORT: c_uint = SIL_MASK_2PORT | SIL_MASK_IDE2_INT | SIL_MASK_IDE3_INT;
pub const SIL_INTR_STEERING: c_uint = 1 << 1;
pub const SIL_DMA_ENABLE: c_uint = 1 << 0; // DMA run switch
pub const SIL_DMA_RDWR: c_uint = 1 << 3; // DMA Rd-Wr
pub const SIL_DMA_SATA_IRQ: c_uint = 1 << 4; // OR of all SATA IRQs
pub const SIL_DMA_ACTIVE: c_uint = 1 << 16; // DMA running
pub const SIL_DMA_ERROR: c_uint = 1 << 17; // PCI bus error
pub const SIL_DMA_COMPLETE: c_uint = 1 << 18; // cmd complete / IRQ pending
pub const SIL_DMA_N_SATA_IRQ: c_uint = 1 << 6; // SATA_IRQ for the next channel
pub const SIL_DMA_N_ACTIVE: c_uint = 1 << 24; // ACTIVE for the next channel
pub const SIL_DMA_N_ERROR: c_uint = 1 << 25; // ERROR for the next channel
pub const SIL_DMA_N_COMPLETE: c_uint = 1 << 26; // COMPLETE for the next channel
pub const SIL_SIEN_N: c_uint = 1 << 16; // triggered by SError.N
pub const SIL_QUIRK_MOD15WRITE: c_uint = 1 << 0;
pub const SIL_QUIRK_UDMA5MAX: c_uint = 1 << 1;

extern "C" {
    fn sil_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int;
    fn sil_pci_device_resume(pdev: *mut pci_dev) -> c_int;
    fn sil_dev_config(dev: *mut ata_device);
    fn sil_scr_read(link: *mut ata_link, sc_reg: c_uint, val: *mut u32) -> c_int;
    fn sil_scr_write(link: *mut ata_link, sc_reg: c_uint, val: u32) -> c_int;
    fn sil_set_mode(link: *mut ata_link, r_failed: *mut *mut ata_device) -> c_int;
    fn sil_qc_prep(qc: *mut ata_queued_cmd) -> c_int;
    fn sil_bmdma_setup(qc: *mut ata_queued_cmd);
    fn sil_bmdma_start(qc: *mut ata_queued_cmd);
    fn sil_bmdma_stop(qc: *mut ata_queued_cmd);
    fn sil_freeze(ap: *mut ata_port);
    fn sil_thaw(ap: *mut ata_port);
}

#[repr(C)]
pub struct pci_device_id { pub driver_data: c_ulong, pub _rest: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct ata_device { _private: [u8; 0] }
#[repr(C)] pub struct ata_link { _private: [u8; 0] }
#[repr(C)] pub struct ata_queued_cmd { _private: [u8; 0] }
#[repr(C)] pub struct ata_port { _private: [u8; 0] }
#[repr(C)] pub struct ata_port_operations { pub inherits: *const c_void, pub dev_config: Option<unsafe extern "C" fn(*mut ata_device)> }
#[repr(C)] pub struct ata_port_info { pub flags: c_uint, pub pio_mask: c_uint, pub mwdma_mask: c_uint, pub udma_mask: c_uint, pub port_ops: *const ata_port_operations }
#[repr(C)] pub struct scsi_host_template { pub dma_boundary: c_ulong, pub sg_tablesize: c_uint }
#[repr(C)] pub struct pci_driver { pub name: *const c_char, pub id_table: *const pci_device_id, pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut pci_dev)> }

#[repr(C)]
pub struct sil_drivelist { pub product: *const c_char, pub quirk: c_uint }

pub static sil_pci_tbl: [pci_device_id; 8] = [
    pci_device_id { driver_data: sil_3112 as c_ulong, _rest: [] },
    pci_device_id { driver_data: sil_3112 as c_ulong, _rest: [] },
    pci_device_id { driver_data: sil_3512 as c_ulong, _rest: [] },
    pci_device_id { driver_data: sil_3114 as c_ulong, _rest: [] },
    pci_device_id { driver_data: sil_3112 as c_ulong, _rest: [] },
    pci_device_id { driver_data: sil_3112_no_sata_irq as c_ulong, _rest: [] },
    pci_device_id { driver_data: sil_3112_no_sata_irq as c_ulong, _rest: [] },
    pci_device_id { driver_data: 0, _rest: [] },
];

// TODO firmware versions should be added - eric
pub static sil_quirks: [sil_drivelist; 13] = [
    sil_drivelist { product: b"ST320012AS\0".as_ptr() as *const c_char, quirk: SIL_QUIRK_MOD15WRITE },
    sil_drivelist { product: b"ST330013AS\0".as_ptr() as *const c_char, quirk: SIL_QUIRK_MOD15WRITE },
    sil_drivelist { product: b"ST340017AS\0".as_ptr() as *const c_char, quirk: SIL_QUIRK_MOD15WRITE },
    sil_drivelist { product: b"ST360015AS\0".as_ptr() as *const c_char, quirk: SIL_QUIRK_MOD15WRITE },
    sil_drivelist { product: b"ST380023AS\0".as_ptr() as *const c_char, quirk: SIL_QUIRK_MOD15WRITE },
    sil_drivelist { product: b"ST3120023AS\0".as_ptr() as *const c_char, quirk: SIL_QUIRK_MOD15WRITE },
    sil_drivelist { product: b"ST340014ASL\0".as_ptr() as *const c_char, quirk: SIL_QUIRK_MOD15WRITE },
    sil_drivelist { product: b"ST360014ASL\0".as_ptr() as *const c_char, quirk: SIL_QUIRK_MOD15WRITE },
    sil_drivelist { product: b"ST380011ASL\0".as_ptr() as *const c_char, quirk: SIL_QUIRK_MOD15WRITE },
    sil_drivelist { product: b"ST3120022ASL\0".as_ptr() as *const c_char, quirk: SIL_QUIRK_MOD15WRITE },
    sil_drivelist { product: b"ST3160021ASL\0".as_ptr() as *const c_char, quirk: SIL_QUIRK_MOD15WRITE },
    sil_drivelist { product: b"TOSHIBA MK2561GSYN\0".as_ptr() as *const c_char, quirk: SIL_QUIRK_MOD15WRITE },
    sil_drivelist { product: b"Maxtor 4D060H3\0".as_ptr() as *const c_char, quirk: SIL_QUIRK_UDMA5MAX },
];

#[repr(C)] pub struct sil_port_regs { pub tf: c_ulong, pub ctl: c_ulong, pub bmdma: c_ulong, pub bmdma2: c_ulong, pub fifo_cfg: c_ulong, pub scr: c_ulong, pub sien: c_ulong, pub xfer_mode: c_ulong, pub sfis_cfg: c_ulong }
pub static sil_port: [sil_port_regs; 4] = [
    sil_port_regs { tf: 0x80, ctl: 0x8A, bmdma: 0x0, bmdma2: 0x10, fifo_cfg: 0x40, scr: 0x100, sien: 0x148, xfer_mode: 0xb4, sfis_cfg: 0x14c },
    sil_port_regs { tf: 0xC0, ctl: 0xCA, bmdma: 0x8, bmdma2: 0x18, fifo_cfg: 0x44, scr: 0x180, sien: 0x1c8, xfer_mode: 0xf4, sfis_cfg: 0x1cc },
    sil_port_regs { tf: 0x280, ctl: 0x28A, bmdma: 0x200, bmdma2: 0x210, fifo_cfg: 0x240, scr: 0x300, sien: 0x348, xfer_mode: 0x2b4, sfis_cfg: 0x34c },
    sil_port_regs { tf: 0x2C0, ctl: 0x2CA, bmdma: 0x208, bmdma2: 0x218, fifo_cfg: 0x244, scr: 0x380, sien: 0x3c8, xfer_mode: 0x2f4, sfis_cfg: 0x3cc },
];

// MODULE_AUTHOR("Jeff Garzik");
// MODULE_DESCRIPTION("low-level driver for Silicon Image SATA controller");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
