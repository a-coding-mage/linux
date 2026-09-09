/* SPDX-License-Identifier: GPL-2.0 */

// C header dependency: <linux/sizes.h>

pub const AHCI_VSCAP: u32 = 0xa4;
pub const AHCI_REMAP_CAP: u32 = 0x800;

/* device class code */
pub const AHCI_REMAP_N_DCC: u32 = 0x880;

/* remap-device base relative to ahci-bar */
pub const AHCI_REMAP_N_OFFSET: u32 = 16 * 1024;
pub const AHCI_REMAP_N_SIZE: u32 = 16 * 1024;

pub const AHCI_MAX_REMAP: u32 = 3;

#[inline]
pub fn ahci_remap_dcc(i: i32) -> u32 {
    (AHCI_REMAP_N_DCC as i32 + i.wrapping_mul(0x80)) as u32
}

#[inline]
pub fn ahci_remap_base(i: i32) -> u32 {
    (AHCI_REMAP_N_OFFSET as i32
        + i.wrapping_mul(AHCI_REMAP_N_SIZE as i32)) as u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
