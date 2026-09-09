/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: the C header includes <linux/types.h> for u32.

/*
 * rx/tx dma descriptor
 */
#[repr(C)]
pub struct bcm_enet_desc {
    pub len_stat: u32,
    pub address: u32,
}

/* control */
pub const DMADESC_LENGTH_SHIFT: u32 = 16;
pub const DMADESC_LENGTH_MASK: u32 = 0xfff << DMADESC_LENGTH_SHIFT;
pub const DMADESC_OWNER_MASK: u32 = 1 << 15;
pub const DMADESC_EOP_MASK: u32 = 1 << 14;
pub const DMADESC_SOP_MASK: u32 = 1 << 13;
pub const DMADESC_ESOP_MASK: u32 = DMADESC_EOP_MASK | DMADESC_SOP_MASK;
pub const DMADESC_WRAP_MASK: u32 = 1 << 12;
pub const DMADESC_USB_NOZERO_MASK: u32 = 1 << 1;
pub const DMADESC_USB_ZERO_MASK: u32 = 1 << 0;

/* status */
pub const DMADESC_UNDER_MASK: u32 = 1 << 9;
pub const DMADESC_APPEND_CRC: u32 = 1 << 8;
pub const DMADESC_OVSIZE_MASK: u32 = 1 << 4;
pub const DMADESC_RXER_MASK: u32 = 1 << 2;
pub const DMADESC_CRC_MASK: u32 = 1 << 1;
pub const DMADESC_OV_MASK: u32 = 1 << 0;
pub const DMADESC_ERR_MASK: u32 = DMADESC_UNDER_MASK
    | DMADESC_OVSIZE_MASK
    | DMADESC_RXER_MASK
    | DMADESC_CRC_MASK
    | DMADESC_OV_MASK;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
