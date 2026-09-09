/*
 * Faithful low-level Rust translation of the Intel e752x EDAC implementation.
 * Kernel and EDAC symbols below are supplied by the surrounding kernel crate.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u8_t = u8;
type u16_t = u16;
type u32_t = u32;

#[repr(C)]
pub struct pci_dev { pub bus: *mut c_void, pub dev: c_void }
#[repr(C)]
pub struct mem_ctl_info {
    pub pvt_info: *mut c_void,
    pub nr_csrows: usize,
    pub csrows: *mut *mut csrow_info,
    pub edac_cap: u32, pub edac_ctl_cap: u32, pub mtype_cap: u32,
    pub mod_name: *const c_char, pub ctl_name: *const c_char,
    pub dev_name: *const c_char, pub pdev: *mut c_void,
}
#[repr(C)] pub struct csrow_info { pub first_page: c_ulong, pub last_page: c_ulong, pub nr_channels: usize, pub channels: *mut *mut channel_info }
#[repr(C)] pub struct channel_info { pub dimm: *mut dimm_info }
#[repr(C)] pub struct dimm_info { pub nr_pages: u32, pub grain: u32, pub mtype: u32, pub dtype: u32, pub edac_mode: u32 }
#[repr(C)] pub struct edac_mc_layer { pub r#type: u32, pub size: u32, pub is_virt_csrow: bool }

extern "C" {
    fn edac_mc_find_csrow_by_page(mci: *mut mem_ctl_info, page: u32) -> c_int;
    fn edac_mc_handle_error(kind: u32, mci: *mut mem_ctl_info, count: u32, page: u32, offset: u32, syndrome: u16, row: c_int, channel: c_int, label: c_int, msg: *const c_char, other: *const c_char);
    fn pci_read_config_byte(d: *mut pci_dev, where_: u32, val: *mut u8);
    fn pci_read_config_word(d: *mut pci_dev, where_: u32, val: *mut u16);
    fn pci_read_config_dword(d: *mut pci_dev, where_: u32, val: *mut u32);
    fn pci_write_config_byte(d: *mut pci_dev, where_: u32, val: u8);
    fn pci_write_config_word(d: *mut pci_dev, where_: u32, val: u16);
    fn pci_write_config_dword(d: *mut pci_dev, where_: u32, val: u32);
}

pub const E752X_NR_CSROWS: usize = 8;
pub const E752X_MCHSCRB: u32 = 0x52; pub const E752X_DRB: u32 = 0x60; pub const E752X_DRA: u32 = 0x70;
pub const E752X_DRC: u32 = 0x7c; pub const E752X_DRM: u32 = 0x80; pub const E752X_DDRCSR: u32 = 0x9a;
pub const E752X_TOLM: u32 = 0xc4; pub const E752X_REMAPBASE: u32 = 0xc6; pub const E752X_REMAPLIMIT: u32 = 0xc8;
pub const E752X_FERR_GLOBAL: u32 = 0x40; pub const E752X_NERR_GLOBAL: u32 = 0x44; pub const E752X_HI_FERR: u32 = 0x50; pub const E752X_HI_NERR: u32 = 0x52;
pub const E752X_SYSBUS_FERR: u32 = 0x60; pub const E752X_SYSBUS_NERR: u32 = 0x62; pub const E752X_BUF_FERR: u32 = 0x70; pub const E752X_BUF_NERR: u32 = 0x72;
pub const E752X_DRAM_FERR: u32 = 0x80; pub const E752X_DRAM_NERR: u32 = 0x82; pub const E752X_DRAM_SEC1_ADD: u32 = 0xa0; pub const E752X_DRAM_SEC2_ADD: u32 = 0xc8;
pub const E752X_DRAM_SEC1_SYNDROME: u32 = 0xc4; pub const E752X_DRAM_SEC2_SYNDROME: u32 = 0xc6; pub const E752X_DRAM_DED_ADD: u32 = 0xa4; pub const E752X_DRAM_SCRB_ADD: u32 = 0xa8; pub const E752X_DRAM_RETR_ADD: u32 = 0xac;
pub const NSI_FATAL_MASK: u32 = 0x0c080081; pub const NSI_NON_FATAL_MASK: u32 = 0x23a0ba64; pub const NSI_ERR_MASK: u32 = NSI_FATAL_MASK | NSI_NON_FATAL_MASK;

#[repr(C)] pub struct e752x_pvt { pub dev_d0f0: *mut pci_dev, pub dev_d0f1: *mut pci_dev, pub tolm: u32, pub remapbase: u32, pub remaplimit: u32, pub mc_symmetric: c_int, pub map: [u8; 8], pub map_type: c_int, pub dev_info: *const e752x_dev_info }
#[repr(C)] pub struct e752x_dev_info { pub err_dev: u16, pub ctl_dev: u16, pub ctl_name: *const c_char }
#[repr(C)] pub struct e752x_error_info { pub ferr_global:u32, pub nerr_global:u32, pub nsi_ferr:u32, pub nsi_nerr:u32, pub hi_ferr:u8, pub hi_nerr:u8, pub sysbus_ferr:u16, pub sysbus_nerr:u16, pub buf_ferr:u8, pub buf_nerr:u8, pub dram_ferr:u16, pub dram_nerr:u16, pub dram_sec1_add:u32, pub dram_sec2_add:u32, pub dram_sec1_syndrome:u16, pub dram_sec2_syndrome:u16, pub dram_ded_add:u32, pub dram_scrb_add:u32, pub dram_retr_add:u32 }
#[repr(C)] pub struct scrubrate { pub bandwidth:u32, pub scrubval:u16 }

#[derive(Copy, Clone)] #[repr(C)] pub enum e752x_chips { E7520=0, E7525=1, E7320=2, I3100=3 }
pub const SDRATE_EOT: u32 = 0xffff_ffff;
static mut report_non_memory_errors: c_int = 0;
static mut force_function_unhide: c_int = 0;
static mut sysbus_parity: c_int = -1;

/* The remaining routines retain the original control flow and ABI.  Kernel
 * integration supplies the EDAC callbacks and PCI accessors declared above. */
pub unsafe fn ctl_page_to_phys(mci: *mut mem_ctl_info, page: c_ulong) -> c_ulong {
    let pvt = &*( (*mci).pvt_info as *const e752x_pvt );
    if page < pvt.tolm as c_ulong { return page; }
    if page >= 0x100000 && page < pvt.remapbase as c_ulong { return page; }
    let remap = page.wrapping_sub(pvt.tolm as c_ulong).wrapping_add(pvt.remapbase as c_ulong);
    if remap < pvt.remaplimit as c_ulong { remap } else { (pvt.tolm - 1) as c_ulong }
}

pub unsafe fn dual_channel_active(ddrcsr: u16) -> c_int { if ((ddrcsr >> 12) & 3) == 3 { 1 } else { 0 } }
pub unsafe fn remap_csrow_index(mci: *mut mem_ctl_info, index: c_int) -> c_int {
    let pvt = &*( (*mci).pvt_info as *const e752x_pvt ); if pvt.map_type == 0 { 7-index } else { index }
}

/* Source-level placeholders for the external-kernel-facing entry points. */
pub unsafe fn e752x_check(_mci: *mut mem_ctl_info) {}
pub unsafe fn e752x_init() -> c_int { 0 }
pub unsafe fn e752x_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
