/* SPDX-License-Identifier: GPL-2.0 */

pub const BCMA_ADDR_BASE: u32 = 0x18000000;
pub const BCMA_WRAP_BASE: u32 = 0x18100000;

pub const SCAN_ER_VALID: u32 = 0x00000001;
pub const SCAN_ER_TAGX: u32 = 0x00000006; /* we have to ignore 0x8 bit when checking tag for SCAN_ER_TAG_ADDR */
pub const SCAN_ER_TAG: u32 = 0x0000000E;
pub const SCAN_ER_TAG_CI: u32 = 0x00000000;
pub const SCAN_ER_TAG_MP: u32 = 0x00000002;
pub const SCAN_ER_TAG_ADDR: u32 = 0x00000004;
pub const SCAN_ER_TAG_END: u32 = 0x0000000E;
pub const SCAN_ER_BAD: u32 = 0xFFFFFFFF;

pub const SCAN_CIA_CLASS: u32 = 0x000000F0;
pub const SCAN_CIA_CLASS_SHIFT: u32 = 4;
pub const SCAN_CIA_ID: u32 = 0x000FFF00;
pub const SCAN_CIA_ID_SHIFT: u32 = 8;
pub const SCAN_CIA_MANUF: u32 = 0xFFF00000;
pub const SCAN_CIA_MANUF_SHIFT: u32 = 20;

pub const SCAN_CIB_NMP: u32 = 0x000001F0;
pub const SCAN_CIB_NMP_SHIFT: u32 = 4;
pub const SCAN_CIB_NSP: u32 = 0x00003E00;
pub const SCAN_CIB_NSP_SHIFT: u32 = 9;
pub const SCAN_CIB_NMW: u32 = 0x0007C000;
pub const SCAN_CIB_NMW_SHIFT: u32 = 14;
pub const SCAN_CIB_NSW: u32 = 0x00F80000;
pub const SCAN_CIB_NSW_SHIFT: u32 = 19;
pub const SCAN_CIB_REV: u32 = 0xFF000000;
pub const SCAN_CIB_REV_SHIFT: u32 = 24;

pub const SCAN_ADDR_AG32: u32 = 0x00000008;
pub const SCAN_ADDR_SZ: u32 = 0x00000030;
pub const SCAN_ADDR_SZ_SHIFT: u32 = 4;
pub const SCAN_ADDR_SZ_4K: u32 = 0x00000000;
pub const SCAN_ADDR_SZ_8K: u32 = 0x00000010;
pub const SCAN_ADDR_SZ_16K: u32 = 0x00000020;
pub const SCAN_ADDR_SZ_SZD: u32 = 0x00000030;
pub const SCAN_ADDR_TYPE: u32 = 0x000000C0;
pub const SCAN_ADDR_TYPE_SLAVE: u32 = 0x00000000;
pub const SCAN_ADDR_TYPE_BRIDGE: u32 = 0x00000040;
pub const SCAN_ADDR_TYPE_SWRAP: u32 = 0x00000080;
pub const SCAN_ADDR_TYPE_MWRAP: u32 = 0x000000C0;
pub const SCAN_ADDR_PORT: u32 = 0x00000F00;
pub const SCAN_ADDR_PORT_SHIFT: u32 = 8;
pub const SCAN_ADDR_ADDR: u32 = 0xFFFFF000;

pub const SCAN_ADDR_SZ_BASE: u32 = 0x00001000; /* 4KB */

pub const SCAN_SIZE_SZ_ALIGN: u32 = 0x00000FFF;
pub const SCAN_SIZE_SZ: u32 = 0xFFFFF000;
pub const SCAN_SIZE_SG32: u32 = 0x00000008;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
