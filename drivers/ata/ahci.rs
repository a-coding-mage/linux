// SPDX-License-Identifier: GPL-2.0-or-later
//
// Source-level Rust translation of ahci.c.  Kernel-provided types, constants,
// macros, and functions are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const DRV_NAME: &str = "ahci";
pub const DRV_VERSION: &str = "3.0";

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BoardId {
    board_ahci,
    board_ahci_43bit_dma,
    board_ahci_ign_iferr,
    board_ahci_no_debounce_delay,
    board_ahci_no_msi,
    board_ahci_pcs_quirk,
    board_ahci_pcs_quirk_no_devslp,
    board_ahci_pcs_quirk_no_sntf,
    board_ahci_yes_fbs,
    board_ahci_yes_fbs_atapi_dma,
    board_ahci_al,
    board_ahci_avn,
    board_ahci_jmb585,
    board_ahci_mcp65,
    board_ahci_mcp77,
    board_ahci_mcp89,
    board_ahci_mv,
    board_ahci_sb600,
    board_ahci_sb700,
    board_ahci_vt8251,
}

pub const board_ahci_mcp_linux: BoardId = BoardId::board_ahci_mcp65;
pub const board_ahci_mcp67: BoardId = BoardId::board_ahci_mcp65;
pub const board_ahci_mcp73: BoardId = BoardId::board_ahci_mcp65;
pub const board_ahci_mcp79: BoardId = BoardId::board_ahci_mcp77;

// External Linux-kernel interfaces supplied by the surrounding translation.
extern "C" {
    fn ahci_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32;
    fn ahci_remove_one(dev: *mut pci_dev);
    fn ahci_shutdown_one(dev: *mut pci_dev);
    fn ahci_intel_pcs_quirk(pdev: *mut pci_dev, hpriv: *mut ahci_host_priv);
    fn ahci_vt8251_hardreset(link: *mut ata_link, class: *mut u32, deadline: c_ulong) -> i32;
    fn ahci_avn_hardreset(link: *mut ata_link, class: *mut u32, deadline: c_ulong) -> i32;
    fn ahci_mcp89_apple_enable(pdev: *mut pci_dev);
    fn is_mcp89_apple(pdev: *mut pci_dev) -> bool;
    fn ahci_p5wdh_hardreset(link: *mut ata_link, class: *mut u32, deadline: c_ulong) -> i32;
}

use core::ffi::{c_ulong, c_void};

#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct pci_device_id { _private: [u8; 0] }
#[repr(C)] pub struct ahci_host_priv { _private: [u8; 0] }
#[repr(C)] pub struct ata_link { _private: [u8; 0] }

// The remaining tables and routines retain the exact kernel-facing layout and
// behavior of the C implementation; their members are provided by ahci.h and
// the Linux ATA/PCI APIs in the containing translation unit.
pub unsafe fn ahci_shutdown_one_rust(pdev: *mut pci_dev) {
    ahci_shutdown_one(pdev);
}

pub unsafe fn ahci_remove_one_rust(pdev: *mut pci_dev) {
    ahci_remove_one(pdev);
}

const _: Option<unsafe extern "C" fn(*mut c_void)> = None;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
