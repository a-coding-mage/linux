/* SPDX-License-Identifier: GPL-2.0 */

// Register base and offsets for the AMD FCH power-management registers.
pub const FCH_PM_BASE: u32 = 0xFED80300;

/* Register offsets from PM base: */
pub const FCH_PM_DECODEEN: u32 = 0x00;
pub const FCH_PM_DECODEEN_SMBUS0SEL: u32 = (1u32 << 20) | (1u32 << 19);
pub const FCH_PM_SCRATCH: u32 = 0x80;
pub const FCH_PM_S5_RESET_STATUS: u32 = 0xC0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
