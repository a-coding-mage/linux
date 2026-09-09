/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2019 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

/* PCI CONFIGURATION SPACE */
pub const mmPCI_CONFIG_ELBI_ADDR: u32 = 0xFF0;
pub const mmPCI_CONFIG_ELBI_DATA: u32 = 0xFF4;
pub const mmPCI_CONFIG_ELBI_CTRL: u32 = 0xFF8;
pub const PCI_CONFIG_ELBI_CTRL_WRITE: u32 = 1u32 << 31;

pub const mmPCI_CONFIG_ELBI_STS: u32 = 0xFFC;
pub const PCI_CONFIG_ELBI_STS_ERR: u32 = 1u32 << 30;
pub const PCI_CONFIG_ELBI_STS_DONE: u32 = 1u32 << 31;
pub const PCI_CONFIG_ELBI_STS_MASK: u32 =
    PCI_CONFIG_ELBI_STS_ERR | PCI_CONFIG_ELBI_STS_DONE;

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hl_revision_id {
    /* PCI revision ID 0 is not legal */
    REV_ID_INVALID = 0x00,
    REV_ID_A = 0x01,
    REV_ID_B = 0x02,
    REV_ID_C = 0x03,
    REV_ID_D = 0x04,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
