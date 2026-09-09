/* SPDX-License-Identifier: GPL-2.0 */

// The original header conditionally included Linux type definitions when
// compiled in the kernel. Rust's fixed-width integer types provide the same
// declarations locally.

macro_rules! BCMA_CORE {
    ($manuf:expr, $id:expr, $rev:expr, $class:expr) => {
        bcma_device_id {
            manuf: $manuf,
            id: $id,
            rev: $rev,
            class: $class,
        }
    };
}

pub const BCMA_ANY_MANUF: u16 = 0xFFFF;
pub const BCMA_ANY_ID: u16 = 0xFFFF;
pub const BCMA_ANY_REV: u8 = 0xFF;
pub const BCMA_ANY_CLASS: u8 = 0xFF;

/* Broadcom's specific AMBA core, see drivers/bcma/ */
#[repr(C, packed(2))]
pub struct bcma_device_id {
    pub manuf: u16,
    pub id: u16,
    pub rev: u8,
    pub class: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
