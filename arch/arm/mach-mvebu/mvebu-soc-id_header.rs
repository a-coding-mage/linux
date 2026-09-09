/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Marvell EBU SoC ID and revision definitions.
 *
 * Copyright (C) 2014 Marvell Semiconductor
 */

/* Armada XP ID */
pub const MV78230_DEV_ID: u32 = 0x7823;
pub const MV78260_DEV_ID: u32 = 0x7826;
pub const MV78460_DEV_ID: u32 = 0x7846;

/* Armada XP Revision */
pub const MV78XX0_A0_REV: u32 = 0x1;
pub const MV78XX0_B0_REV: u32 = 0x2;

/* Amada 370 ID */
pub const ARMADA_370_DEV_ID: u32 = 0x6710;

/* Amada 370 Revision */
pub const ARMADA_370_A1_REV: u32 = 0x1;

/* Armada 375 ID */
pub const ARMADA_375_DEV_ID: u32 = 0x6720;

/* Armada 375 */
pub const ARMADA_375_Z1_REV: u32 = 0x0;
pub const ARMADA_375_A0_REV: u32 = 0x3;

/* Armada 38x ID */
pub const ARMADA_380_DEV_ID: u32 = 0x6810;
pub const ARMADA_385_DEV_ID: u32 = 0x6820;
pub const ARMADA_388_DEV_ID: u32 = 0x6828;

/* Armada 38x Revision */
pub const ARMADA_38x_Z1_REV: u32 = 0x0;
pub const ARMADA_38x_A0_REV: u32 = 0x4;

/* CONFIG_ARCH_MVEBU build-time condition from the original header. */
#[cfg(feature = "CONFIG_ARCH_MVEBU")]
extern "C" {
    pub fn mvebu_get_soc_id(dev: *mut u32, rev: *mut u32) -> i32;
}

#[cfg(not(feature = "CONFIG_ARCH_MVEBU"))]
pub fn mvebu_get_soc_id(_dev: *mut u32, _rev: *mut u32) -> i32 {
    -1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
