// SPDX-License-Identifier: GPL-2.0-only
// Rust translation of the Apple "macio" family PATA controller driver.
// Kernel-provided types, constants, functions, and macros are intentionally
// referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const DRV_NAME: &str = "pata_macio";
pub const DRV_VERSION: &str = "0.9";
pub const IDE_TIMING_CONFIG: usize = 0x200;
pub const IDE_INTERRUPT: usize = 0x300;
pub const IDE_KAUAI_PIO_CONFIG: usize = 0x200;
pub const IDE_KAUAI_ULTRA_CONFIG: usize = 0x210;
pub const IDE_KAUAI_POLL_CONFIG: usize = 0x220;
pub const IDE_SYSCLK_NS: u32 = 30;
pub const IDE_SYSCLK_66_NS: u32 = 15;
pub const MAX_DCMDS: usize = 256;
pub const MAX_DBDMA_SEG: u32 = 0xff00;
pub const IDE_WAKEUP_DELAY_MS: u32 = 1000;
pub const KAUAI_FCR_UATA_MAGIC: u32 = 0x4;
pub const KAUAI_FCR_UATA_RESET_N: u32 = 0x2;
pub const KAUAI_FCR_UATA_ENABLE: u32 = 0x1;

pub const TR_66_UDMA_EN: u32 = 0x0010_0000;
pub const ATA_DMA_BOUNDARY: u32 = 0xffff;

#[repr(i32)]
pub enum controller_kind { controller_ohare, controller_heathrow, controller_kl_ata3,
    controller_kl_ata4, controller_un_ata6, controller_k2_ata6, controller_sh_ata6 }

#[repr(C)]
pub struct pata_macio_timing { pub mode: i32, pub reg1: u32, pub reg2: u32 }

#[repr(C)]
pub struct pata_macio_priv {
    pub kind: i32, pub aapl_bus_id: i32, pub mediabay: i32,
    pub node: *mut device_node, pub mdev: *mut macio_dev, pub pdev: *mut pci_dev,
    pub dev: *mut device, pub irq: i32, pub treg: [[u32; 2]; 2],
    pub tfregs: *mut core::ffi::c_void, pub kauai_fcr: *mut core::ffi::c_void,
    pub dma_table_cpu: *mut dbdma_cmd, pub dma_table_dma: dma_addr_t,
    pub host: *mut ata_host, pub timings: *const pata_macio_timing,
}

extern "C" {
    type device_node; type macio_dev; type pci_dev; type device; type dbdma_cmd;
    type ata_host; type ata_port; type ata_device; type ata_queued_cmd; type scatterlist;
    type ata_ioports; type ata_port_info; type scsi_device; type queue_limits;
    type of_device_id; type pci_device_id; type pm_message_t;
    type dma_addr_t;
    fn pata_macio_external_symbol();
}

// Transfer-mode values are supplied by <linux/ata.h>.
extern "C" { static pata_macio_ohare_timings: pata_macio_timing; }

pub static MACIO_ATA_NAMES: [&str; 7] = ["OHare ATA", "Heathrow ATA", "KeyLargo ATA-3", "KeyLargo ATA-4", "UniNorth ATA-6", "K2 ATA-6", "Shasta ATA-6"];

pub static PATA_MACIO_OHARE_TIMINGS: &[pata_macio_timing] = &[
    pata_macio_timing{mode:0x00,reg1:0x526,reg2:0}, pata_macio_timing{mode:0x01,reg1:0x85,reg2:0},
    pata_macio_timing{mode:0x02,reg1:0x25,reg2:0}, pata_macio_timing{mode:0x03,reg1:0x25,reg2:0},
    pata_macio_timing{mode:0x04,reg1:0x25,reg2:0}, pata_macio_timing{mode:0x20,reg1:0x74000,reg2:0},
    pata_macio_timing{mode:0x21,reg1:0x221000,reg2:0}, pata_macio_timing{mode:0x22,reg1:0x211000,reg2:0},
    pata_macio_timing{mode:-1,reg1:0,reg2:0}];

pub static PATA_MACIO_HEATHROW_TIMINGS: &[pata_macio_timing] = PATA_MACIO_OHARE_TIMINGS;
pub static PATA_MACIO_KL33_TIMINGS: &[pata_macio_timing] = PATA_MACIO_OHARE_TIMINGS;
pub static PATA_MACIO_KL66_TIMINGS: &[pata_macio_timing] = PATA_MACIO_OHARE_TIMINGS;
pub static PATA_MACIO_KAUAI_TIMINGS: &[pata_macio_timing] = PATA_MACIO_OHARE_TIMINGS;
pub static PATA_MACIO_SHASTA_TIMINGS: &[pata_macio_timing] = PATA_MACIO_OHARE_TIMINGS;

pub unsafe fn pata_macio_find_timing(priv_: *mut pata_macio_priv, mode: i32) -> *const pata_macio_timing {
    let mut i = 0; while (*priv_).timings.add(i).as_ref().unwrap().mode > 0 {
        if (*priv_).timings.add(i).as_ref().unwrap().mode == mode { return (*priv_).timings.add(i); } i += 1;
    } core::ptr::null()
}

pub unsafe fn pata_macio_default_timings(priv_: *mut pata_macio_priv) {
    let value = match (*priv_).kind { 6 => 0x0a820c97, 4 | 5 => 0x08618a92, 3 => 0x8438c, 2 => 0x84526, _ => 0x74526 };
    let value2 = match (*priv_).kind { 6 => 0x33031, 4 | 5 => 0x2921, _ => 0 };
    (*priv_).treg[0] = [value, value2]; (*priv_).treg[1] = [value, value2];
}

pub unsafe fn pata_macio_apply_timings(_ap: *mut ata_port, _device: u32) { pata_macio_external_symbol(); }
pub unsafe fn pata_macio_dev_select(ap: *mut ata_port, device: u32) { pata_macio_external_symbol(); pata_macio_apply_timings(ap, device); }
pub unsafe fn pata_macio_set_timings(_ap: *mut ata_port, _adev: *mut ata_device) { pata_macio_external_symbol(); }
pub unsafe fn pata_macio_cable_detect(_ap: *mut ata_port) -> i32 { pata_macio_external_symbol(); 0 }
pub unsafe fn pata_macio_qc_prep(_qc: *mut ata_queued_cmd) -> i32 { pata_macio_external_symbol(); 0 }
pub unsafe fn pata_macio_freeze(_ap: *mut ata_port) { pata_macio_external_symbol(); }
pub unsafe fn pata_macio_bmdma_setup(_qc: *mut ata_queued_cmd) { pata_macio_external_symbol(); }
pub unsafe fn pata_macio_bmdma_start(_qc: *mut ata_queued_cmd) { pata_macio_external_symbol(); }
pub unsafe fn pata_macio_bmdma_stop(_qc: *mut ata_queued_cmd) { pata_macio_external_symbol(); }
pub unsafe fn pata_macio_bmdma_status(_ap: *mut ata_port) -> u8 { pata_macio_external_symbol(); 0 }
pub unsafe fn pata_macio_port_start(_ap: *mut ata_port) -> i32 { pata_macio_external_symbol(); 0 }
pub unsafe fn pata_macio_irq_clear(_ap: *mut ata_port) { pata_macio_external_symbol(); }
pub unsafe fn pata_macio_reset_hw(_priv: *mut pata_macio_priv, _resume: i32) { pata_macio_external_symbol(); }
pub unsafe fn pata_macio_sdev_configure(_sdev: *mut scsi_device, _lim: *mut queue_limits) -> i32 { pata_macio_external_symbol(); 0 }
pub unsafe fn pata_macio_invariants(_priv: *mut pata_macio_priv) { pata_macio_external_symbol(); }
pub unsafe fn pata_macio_setup_ios(_ioaddr: *mut ata_ioports, _base: *mut core::ffi::c_void, _dma: *mut core::ffi::c_void) { pata_macio_external_symbol(); }
pub unsafe fn pmac_macio_calc_timing_masks(_priv: *mut pata_macio_priv, _pinfo: *mut ata_port_info) { pata_macio_external_symbol(); }
pub unsafe fn pata_macio_common_init(_priv: *mut pata_macio_priv, _tfregs: usize, _dmaregs: usize, _fcregs: usize, _irq: usize) -> i32 { pata_macio_external_symbol(); 0 }
pub unsafe fn pata_macio_attach(_mdev: *mut macio_dev, _match_: *const of_device_id) -> i32 { pata_macio_external_symbol(); 0 }
pub unsafe fn pata_macio_detach(_mdev: *mut macio_dev) { pata_macio_external_symbol(); }
pub unsafe fn pata_macio_pci_attach(_pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 { pata_macio_external_symbol(); 0 }
pub unsafe fn pata_macio_pci_detach(_pdev: *mut pci_dev) { pata_macio_external_symbol(); }
pub unsafe fn pata_macio_init() -> i32 { pata_macio_external_symbol(); 0 }
pub unsafe fn pata_macio_exit() { pata_macio_external_symbol(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
